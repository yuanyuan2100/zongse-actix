#[macro_use] extern crate diesel;
// #[macro_use] extern crate actix_web;
#[macro_use] extern crate lazy_static;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

use actix_web::{get, web, HttpResponse};

use diesel::prelude::*;
use tera::{Tera, Context};

use std::env;

use crate::utils::connections::*;
use crate::model::model_post::*;

// pub mod router;
pub mod utils;
// pub mod admin;
pub mod model;
pub mod schema;

#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>, conn: DB) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let mut context = Context::new();

    let results: Vec<_> = posts
                    .filter(published.eq(true))
                    .limit(5)
                    .order(id.desc())
                    .load::<Post>(&*conn)
                    .expect("Error loading posts");
    
    context.insert("posts", &results);
    
    let s = tmpl.render("index.html", &context).unwrap();
   
   HttpResponse::Ok().content_type("text/html").body(s)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
            
        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .service(index)
            .service(Files::new("/", "statics/"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}