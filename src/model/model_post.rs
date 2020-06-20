use serde_derive::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error, Insertable, Queryable};

use crate::schema::*;
use crate::schema::posts::columns::view;
use crate::utils::connections::*;

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub id_url: String,
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub published: bool,
    pub create_time: NaiveDateTime,
    pub view: i32,
    pub tags: Vec<String>,
}

impl Post {
    pub fn find_by_url(url: &str, db: &DB) -> Result<Self, Error> {
        posts::table
            .filter(posts::published.eq(true))
            .filter(posts::id_url.eq(url.to_string()))
            .first::<Post>(&**db)
    }

    pub fn find_unpublished_by_url(url: &str, db: &DB) -> Result<Self, Error> {
        posts::table
            .filter(posts::published.eq(false))
            .filter(posts::id_url.eq(url.to_string()))
            .first::<Post>(db.conn())
    }

    pub fn view_counter(&self, db: &DB) {
        let _view = diesel::update(self)
                .set(view.eq(&self.view + 1))
                .get_result::<Post>(&**db)
                .expect("View counter error.");
    }
}