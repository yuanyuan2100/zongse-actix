#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate actix_web;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{get, error, web, Error, HttpResponse, Result};

use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use diesel::prelude::*;
use tera::{Tera, Context};
use dotenv::dotenv;
use std::env;

use crate::utils::connections::*;
use crate::model::model_post::*;

// pub mod router;
pub mod utils;
// pub mod admin;
pub mod model;
pub mod schema;


#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgConnection::establish(&database_url).unwrap();

    let mut context = Context::new();

    let results: Vec<_> = posts
                    .filter(published.eq(true))
                    .limit(5)
                    .order(id.desc())
                    .load::<Post>(&pool)
                    .expect("Error loading posts");

    let p = &results[0].title;
    
    context.insert("posts", &p);
    println!("{:?}", &context);
    
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