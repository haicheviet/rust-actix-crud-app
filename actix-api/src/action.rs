use diesel::pg::PgConnection;

use diesel::prelude::*;

use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::{posts, published};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_post(db: &mut PgConnection) -> Result<Vec<Post>, DbError> {
    let posts_item = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(db)
        .expect("Error loading posts");

    Ok(posts_item)
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Result<Post, DbError> {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post");
    Ok(post)
}