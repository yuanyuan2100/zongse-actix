use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use actix_web::{dev, Error, HttpRequest, FromRequest, Result};
use actix_web::error::ErrorBadRequest;
use dotenv::dotenv;
use futures::future::{ok, err, Ready};

use std::{env, ops::Deref};

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        match DB_POOL.get() {
            Ok(db) => ok(DB(db)),
            Err(_e) => err(ErrorBadRequest("no luck.")),
        }
    }
}

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}