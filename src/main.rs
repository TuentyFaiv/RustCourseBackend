use dotenvy::dotenv;
use std::env;

extern crate diesel;
extern crate tera;

use tera::{Tera, Context};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, put, delete, web, App};
use actix_web::{HttpResponse, HttpServer, Responder, Result};

pub mod schema;
pub mod models;

use self::models::{Post, NewPostHandler};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DBPool>, html: web::Data<Tera>) -> Result<impl Responder> {
	let mut conn = pool.get().expect("Error to get database");

	let mut ctx = Context::new();

	match web::block(move || {Post::all(&mut conn)}).await {
		Ok(data) => {
			let blogs = data.unwrap();
			ctx.insert("blogs", &blogs);

			let page = html.render("index.html", &mut ctx).unwrap();
			Ok(HttpResponse::Ok().content_type("text/html").body(page))
		},
		Err(data) => {
			let error = format!("Error: {:?}", data);
			Ok(HttpResponse::Ok().body(error))
		},
	}
}

#[get("/create")]
async fn create_blog(html: web::Data<Tera>) -> Result<impl Responder> {
	let mut ctx = Context::new();

	let page = html.render("create.html", &mut ctx).unwrap();
	Ok(HttpResponse::Ok().content_type("text/html").body(page))
}

#[get("/update/{slug}")]
async fn update_blog(
	pool: web::Data<DBPool>,
	html: web::Data<Tera>,
	slug: web::Path<String>
) -> Result<impl Responder> {
	let mut conn = pool.get().expect("Error to get database");

	let mut ctx = Context::new();

	match web::block(move || {Post::get(&mut conn, &slug.to_string())}).await {
		Ok(data) => {
			let blogs = data.unwrap();

			if blogs.len() == 0 {
				return Ok(HttpResponse::NotFound().finish());
			}

			let blog = blogs.get(0).unwrap();

			ctx.insert("blog", blog);

			let page = html.render("update.html", &mut ctx).unwrap();
			Ok(HttpResponse::Ok().content_type("text/html").body(page))
		},
		Err(data) => {
			let error = format!("Error: {:?}", data);
			Ok(HttpResponse::Ok().body(error))
		},
	}
}

#[get("/{slug}")]
async fn show_blog(
	pool: web::Data<DBPool>,
	html: web::Data<Tera>,
	slug_param: web::Path<String>
) -> impl Responder {
	let mut conn = pool.get().expect("Error to get database");

	let mut ctx = Context::new();

	match web::block(move || {Post::get(&mut conn, &slug_param.to_string())}).await {
		Ok(data) => {
			let blogs = data.unwrap();

			if blogs.len() == 0 {
				return HttpResponse::NotFound().finish();
			}

			let blog = blogs.get(0).unwrap();

			ctx.insert("blog", blog);

			let page = html.render("blog.html", &mut ctx).unwrap();
			HttpResponse::Ok().content_type("text/html").body(page)
		},
		Err(data) => {
			let error = format!("Error: {:?}", data);
			HttpResponse::Ok().body(error)
		},
	}
}

#[post("/blog/create")]
async fn new_post(pool: web::Data<DBPool>, item: web::Json<NewPostHandler>) -> impl Responder {
	let mut conn = pool.get().expect("Error to get database");

	match web::block(move || {Post::create(&mut conn, &item)}).await {
		Ok(data) => {
			let data_ok = format!("{:?}", data);
			HttpResponse::Ok().body(data_ok)
		},
		Err(data) => {
			let error = format!("Error: {:?}", data);
			HttpResponse::Ok().body(error)
		},
	}
}

#[put("/blog/update/{id}")]
async fn update_post(
	pool: web::Data<DBPool>,
	id: web::Path<String>,
	item: web::Json<NewPostHandler>
) -> impl Responder {
	let mut conn = pool.get().expect("Error to get database");

	match web::block(move || {Post::update(&mut conn, &id.to_string(), &item)}).await {
		Ok(data) => {
			let data_ok = format!("{:?}", data);
			HttpResponse::Ok().body(data_ok)
		},
		Err(data) => {
			let error = format!("Error: {:?}", data);
			HttpResponse::Ok().body(error)
		},
	}
}

#[delete("/blog/delete/{id}")]
async fn delete_post(pool: web::Data<DBPool>, id_param: web::Path<String>) -> impl Responder {
	let mut conn = pool.get().expect("Error to get database");

	match web::block(move || {Post::delete(&mut conn, &id_param.to_string())}).await {
		Ok(data) => {
			let data_ok = format!("{}", data.unwrap());
			HttpResponse::Ok().body(data_ok)
		},
		Err(data) => {
			let error = format!("Error: {:?}", data);
			HttpResponse::Ok().body(error)
		},
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	dotenv().ok();
	let db_url = env::var("DATABASE_URL").expect("DB url variable not found");

	let connection = ConnectionManager::<PgConnection>::new(db_url);
	let pool = Pool::builder().build(connection).expect("Can't build Pool");

	HttpServer::new(move || {
		let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

		App::new()
			.service(index)
			.service(create_blog)
			.service(update_blog)
			.service(show_blog)
			.service(new_post)
			.service(update_post)
			.service(delete_post)
			.app_data(web::Data::new(pool.clone()))
			.app_data(web::Data::new(tera))
	})
		.bind(("0.0.0.0", 5000))
		.unwrap()
		.run()
		.await
}
