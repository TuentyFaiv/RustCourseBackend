#[macro_use]
use dotenvy::dotenv;
use std::env;

extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub mod schema;
pub mod models;
// pub mod services;

use self::models::Post;
use self::schema::posts::dsl::*;
// use self::schema::posts;

// use crate::{services::{post::{create_posts, create_post, update_post, delete_post}}, models::UpdatePost};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[get("/")]
async fn index(pool: web::Data<DBPool>) -> impl Responder {

	let mut conn = pool.get().expect("Error to get database");
	match web::block(move || {posts.load::<Post>(&mut conn)}).await {
		Ok(data) => {

			HttpResponse::Ok().body(web::Json(data))
		},
		Err(data) => HttpResponse::Ok().body("Error to found data"),
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// use self::models::{Post, NewPost, PostSimple, UpdatePost};
	// use self::schema::posts;
	// use self::schema::posts::dsl::*;

	dotenv().ok();
	let db_url = env::var("DATABASE_URL").expect("DB url variable not found");

	let connection = ConnectionManager::<PgConnection>::new(db_url);
	let pool = Pool::builder().build(connection).expect("Can't build Pool");

	HttpServer::new(move || {
		App::new().service(index).app_data(web::Data::new(pool.clone()))
	})
		.bind(("0.0.0.0", 5000))
		.unwrap()
		.run()
		.await
}
