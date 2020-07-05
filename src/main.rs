#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use actix_web::middleware::NormalizePath;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};

use tera::Tera;
use rand::Rng;
use chrono::Duration;

use std::env;

pub mod router;
pub mod utils;
pub mod admin;
pub mod model;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let auth_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
            
        App::new()
            .data(tera)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&auth_key)
                    .name("auth")
                    .max_age_time(Duration::days(7))
                    .same_site(SameSite::Strict)
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .wrap(NormalizePath {})
            .configure(router::routes)
            .configure(admin::routes)
            .service(Files::new("/", "statics/"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}