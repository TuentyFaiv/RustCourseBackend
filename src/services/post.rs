use::diesel::pg::PgConnection;
use::diesel::prelude::*;

use crate::models::{NewPost, Post, UpdatePost};
use crate::schema::posts;

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str, slug: &str) -> Post {

  let new_post = NewPost {
    title,
    body,
    slug
  };

  let post: Post = diesel::insert_into(posts::table)
    .values(&new_post)
    .get_result(conn)
    .expect("Failed to insert post");

  return post;
}

pub fn create_posts(conn: &mut PgConnection, new_blogs: &Vec<NewPost>) -> Vec<Post> {

  let blogs = diesel::insert_into(posts::table)
    .values(new_blogs)
    .get_results::<Post>(conn)
    .expect("Error to create more than one post");

  return blogs;
}

pub fn update_post(conn: &mut PgConnection, id_post: i32, data: &UpdatePost) -> Post {
  use crate::schema::posts::dsl::*;

  let new_title = data.title.unwrap();
  let new_body = data.body.unwrap();
  let new_slug = data.slug.unwrap();

  let post_update: Post = diesel::update(posts.filter(id.eq(id_post)))
		.set((
      title.eq(new_title),
      body.eq(new_body),
      slug.eq(new_slug),
    ))
		.get_result(conn)
		.expect(format!("Error to update post {id_post}").as_str());

  return post_update;
}

pub fn delete_post(conn: &mut PgConnection, id_post: i32) -> i32 {
  use crate::schema::posts::dsl::*;

  // Delete by slug
  // slug.like("%-post%")
  diesel::delete(posts.filter(id.eq(id_post)))
    .execute(conn)
    .expect(format!("Error to delete post {id_post}").as_str());

  return id_post;
}