use serde_derive::Serialize;
use chrono::NaiveDateTime;
use diesel::{prelude::*, pg::PgConnection, Insertable, Queryable};           

use crate::schema::*;

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset, Clone)]
#[table_name="comments"]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub create_time: NaiveDateTime,
    pub comment_by: String,
    pub comment_id: i32,
}

impl Comment {
    pub fn load_by_post_id(id: &i32, conn: &PgConnection) -> Vec<Comment> {
        comments::table
            .filter(comments::comment_id.eq(&id))
            .load::<Comment>(conn)
            .expect("No comment loaded.")
    }
}