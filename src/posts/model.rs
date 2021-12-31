use crate::db;
use crate::error_handler::CustomError;
use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Posts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let posts = posts::table.load::<Posts>(&conn)?;
        Ok(posts)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let post = posts::table.filter(posts::id.eq(id)).first(&conn)?;
        Ok(post)
    }

    pub fn create(post: Post) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let post = Post::from(post);
        let post = diesel::insert_into(posts::table)
            .values(post)
            .get_result(&conn)?;
        Ok(post)
    }

    pub fn update(id: i32, post: Post) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let post = diesel::update(posts::table)
            .filter(posts::id.eq(id))
            .set(post)
            .get_result(&conn)?;
        Ok(post)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(posts::table.filter(posts::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Post {
    fn from(post: Post) -> Post {
        Post {
            title: post.title,
            body: post.body,
        }
    }
}
