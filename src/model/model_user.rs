use chrono::NaiveDateTime;
use crypto::{digest::Digest, sha3::Sha3};
use diesel::{prelude::*, result::Error, AsChangeset, Insertable, Queryable};
use serde_derive::Serialize;

use crate::schema::*;
use crate::utils::connections::*;

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub create_at: NaiveDateTime,
    pub last_login_at: NaiveDateTime,
}

impl User {
    pub fn authenticated(&self, password: &str) -> bool {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        let result = hasher.result_str();
        self.password.eq(&result)
    }

    pub fn password_generate(password: &str) -> String {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        hasher.result_str()
    }

    pub fn find_by_username(username: &str, db: &DB) -> Result<Self, Error> {
        users::table
            .filter(users::username.eq(username.to_string()))
            .first::<User>(&**db)
    }

    pub fn find_by_user_id(id: i32, db: &DB) -> Result<Self, Error> {
        users::table
            .filter(users::id.eq(id))
            .first::<User>(&**db)
    }
}

