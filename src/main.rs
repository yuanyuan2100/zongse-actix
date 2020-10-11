#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;

use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::NormalizePath;
use actix_web::{middleware, App, HttpServer};
use actix_web_middleware_redirect_https::RedirectHTTPS;

use time::Duration;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use rand::Rng;
use std::env;
use tera::Tera;

use crate::utils::markdown::markdown_filter;

pub mod admin;
pub mod model;
pub mod router;
pub mod schema;
pub mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let auth_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        let mut tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
        tera.register_filter("markdown_filter", markdown_filter);

        App::new()
            .data(tera)
            // .wrap(RedirectHTTPS::with_replacements(&[(
            //     ":8000".to_owned(),
            //     ":8443".to_owned(),
            // )]))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&auth_key)
                    .name("auth")
                    .max_age_time(Duration::days(7))
                    .same_site(SameSite::Strict)
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            // .wrap(NormalizePath {})
            .configure(router::routes)
            .configure(admin::routes)
            .service(Files::new("/", "statics"))
            // .service(web::resource("/").route(web::post().to(upload_img)))
    })
    .bind_openssl("0.0.0.0:8443", builder)?
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
