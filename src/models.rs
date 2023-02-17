use diesel::{Queryable, Insertable};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Serialize, Deserialize};

use super::schema::posts;

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct PostSimple {
  pub title: String,
  pub body: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub slug: String,
  pub body: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
  pub title: String,
  pub body: String
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str,
  pub slug: &'a str,
}

impl Post {
  pub fn slugify(title: &String) -> String {
    title
      .replace(" ", "-")
      .to_lowercase()
  }

  pub fn all<'a>(conn: &mut Conn) -> Result<Vec<Post>, Error> {
    use super::schema::posts::dsl::*;

    posts.load::<Post>(conn)
  }

  pub fn get<'a>(conn: &mut Conn, slug_post: &String) -> Result<Vec<Post>, Error> {
    use super::schema::posts::dsl::*;

    posts
      .filter(slug.eq(slug_post))
      .load::<Post>(conn)
  }

  pub fn create<'a>(
    conn: &mut Conn,
    post: &NewPostHandler
  ) -> Result<Post, Error> {
    let slug = Post::slugify(&post.title.clone());

    let new_post = NewPost {
      title: &post.title.as_str(),
      body: &post.body.as_str(),
      slug: &slug.as_str()
    };

    diesel::insert_into(posts::table)
      .values(&new_post)
      .get_result(conn)
  }

  pub fn update<'a>(
    conn: &mut Conn,
    id_post: &String,
    post: &NewPostHandler
  ) -> Result<Post, Error> {
    use crate::schema::posts::dsl::*;

    let slug_post = Post::slugify(&post.title.clone());

    let id_num = id_post.parse::<i32>().unwrap();

    diesel::update(posts.filter(id.eq(id_num)))
      .set((
        title.eq(&post.title.as_str()),
        body.eq(&post.body.as_str()),
        slug.eq(&slug_post.as_str()),
      ))
      .get_result(conn)
  }

  pub fn delete<'a>(conn: &mut Conn, id_post: &String) -> Result<usize, Error> {
    use crate::schema::posts::dsl::*;

    let id_num = id_post.parse::<i32>().unwrap();

    diesel::delete(posts.filter(id.eq(id_num)))
      .execute(conn)
  }
}