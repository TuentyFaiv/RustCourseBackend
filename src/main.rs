#[macro_use]

extern crate diesel;

use::diesel::prelude::*;
use::diesel::pg::PgConnection;

use dotenvy::dotenv;
use std::env;

pub mod schema;
pub mod models;

fn main() {
	use self::models::{Post, NewPost, PostSimple};
	use self::schema::posts;
	use self::schema::posts::dsl::*;

	dotenv().ok();

	let db_url = env::var("DATABASE_URL").expect("DB url variable not found");

	let mut connection = PgConnection::establish(&db_url).expect("Can't connect to database");

	// let new_post = NewPost {
	// 	title: "Mi segundo blogpost",
	// 	body: "this is a body",
	// 	slug: "segundo-post"
	// };

	// let post: Post = diesel::insert_into(posts::table).values(&new_post).get_result(&mut connection).expect("Failed insert");

	let post_update: Post = diesel::update(posts.filter(id.eq(2))).set(body.eq("The post body has been edited")).get_result(&mut connection).expect("Error to update post");

	// Query without limit
	let all_posts = posts.load::<Post>(&mut connection).expect("Error to run query");

	// // Query with limit
	// let all_posts_limit = posts.limit(1).load::<Post>(&mut connection).expect("Error to run query");

	// // Query with specific columns
	// let all_posts_specific = posts.select((title, body)).limit(1).load::<PostSimple>(&mut connection).expect("Error to run query");

	// // Query with where
	// let post = posts.filter(id.eq(2)).limit(1).load::<Post>(&mut connection).expect("Error to run query");

	// println!("Post 2: {:?}", post);

	println!("All posts");
	for post in all_posts {
		println!("Post: {:?}", post);
	}
	// println!("All posts limited");
	// for post in all_posts_limit {
	// 	println!("Post: {:?}", post);
	// }
	// println!("Specific posts");
	// for post in all_posts_specific {
	// 	println!("Post: {:?}", post);
	// }
}
