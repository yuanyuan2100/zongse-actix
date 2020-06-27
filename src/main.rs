#[macro_use] extern crate diesel;
// #[macro_use] extern crate actix_web;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use actix_web::{get, web, middleware::NormalizePath, HttpResponse};
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_identity::Identity;
use tera::{Tera, Context};
use rand::Rng;
use chrono::Duration;

use std::env;

use crate::utils::{connections::*, url_converter::url_converter};
use crate::model::{model_post::*, model_comment::*};

pub mod router;
pub mod utils;
pub mod admin;
pub mod model;
pub mod schema;

pub fn posts_and_comments(post_url: &str, db: &DB) -> Context {

    let mut context = Context::new();

    let post = Post::find_by_url(&post_url, &*db).unwrap();
    let post_text = url_converter(&post.body);

    let _view = post.view_counter(&*db);

    let comments = Comment::load_by_post_id(&post.id, &*db);

    context.insert("post", &post);
    context.insert("post_text", &post_text);
    context.insert("comments", &comments);
    context.insert("post_url", &post_url);
    context    
}

#[get("post/{post_url}")]
async fn load_post(
    tmpl: web::Data<tera::Tera>, 
    post_url: web::Path<String>, 
    id: Identity, 
    db: DB
) -> HttpResponse {

    let mut context = Context::new();

    context.extend(posts_and_comments(&post_url, &db));

    if id.identity().is_some() {
        println!("Administrator");
        context.insert("display_signin", &"block");   
        context.insert("display_comment", &"block");
        context.insert("display_delete_post", &"block");
    } else {
        println!("Anonymous");
        context.insert("display_signin", &"block"); 
        context.insert("display_comment", &"none");
        context.insert("display_delete_post", &"none");
    }

    let s = tmpl.render("post.html", &context).unwrap();
   
    HttpResponse::Ok().content_type("text/html").body(s)

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
            
        App::new()
            .data(tera)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("auth")
                    .max_age_time(Duration::days(7))
                    .same_site(SameSite::Strict)
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .wrap(NormalizePath {})
            .configure(router::routes)
            .configure(admin::routes)
            .service(load_post)
            .service(Files::new("/", "statics/"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}