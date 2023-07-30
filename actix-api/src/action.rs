use diesel::pg::PgConnection;

use diesel::prelude::*;

use crate::models::{NewPost, Post};
use crate::schema::posts::dsl;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_post(connection: &mut PgConnection, post_id: i32) -> Result<Post, DbError> {
    let post_item = dsl::posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)?;

    Ok(post_item)
}

pub fn get_all_published_post(connection: &mut PgConnection) -> Result<Vec<Post>, DbError> {
    let posts_item = dsl::posts
        .filter(dsl::published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    Ok(posts_item)
}

pub fn create_post(
    connection: &mut PgConnection,
    title: &str,
    body: &str,
    published: bool,
) -> Result<Post, DbError> {
    use crate::schema::posts;

    let new_post = NewPost {
        title,
        body,
        published,
    };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new post");
    Ok(post)
}

pub fn delete_post(connection: &mut PgConnection, post_id: i32) -> Result<bool, DbError> {
    diesel::delete(dsl::posts.filter(dsl::id.eq(post_id))).execute(connection)?;
    Ok(true)
}

pub fn publish_post(connection: &mut PgConnection, post_id: i32) -> Result<Post, DbError> {
    let post = diesel::update(dsl::posts.find(post_id))
        .set(dsl::published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();
    Ok(post)
}
