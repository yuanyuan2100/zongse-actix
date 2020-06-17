#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

// pub mod router;
pub mod utils;
// pub mod admin;
pub mod model;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "./statics/").show_files_listing())
            // .service(Files::new("/", "./templates/").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}