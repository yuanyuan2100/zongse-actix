#[macro_use] extern crate diesel;
// #[macro_use] extern crate actix_web;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use actix_web::{get, post, web, http, middleware::NormalizePath, HttpMessage, HttpResponse, HttpRequest};
use actix_http::cookie::{Cookie, SameSite};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_identity::Identity;

use tera::{Tera, Context};
use serde_derive::{Serialize, Deserialize};
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

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Guest {
    pub guestname: String,
}

#[post("post/{post_url}/guest_signin")]
pub fn guest_login(
    post_url: web::Path<String>,
    guest: web::Json<Guest>,
) -> HttpResponse {

    let guest_name = &guest.guestname;

    let guest_cookie = Cookie::build("guest", guest_name.to_string())
                            .path("/")
                            .secure(false)
                            .finish();

    HttpResponse::Found().header(http::header::LOCATION, format!("/post/{}", &post_url))
                            .cookie(guest_cookie)            
                            .finish()
}

#[get("post/{post_url}")]
async fn load_post(
    tmpl: web::Data<tera::Tera>, 
    post_url: web::Path<String>, 
    rep: HttpRequest,
    id: Identity,
    db: DB
) -> HttpResponse {

    let mut context = Context::new();

    context.extend(posts_and_comments(&post_url, &db));

    if id.identity().is_some() {
        println!("Administrator");
        context.insert("display_signin", &"block");  
        context.insert("display_delete_post", &"block");
    } else {
        println!("Anonymous");
        context.insert("display_signin", &"block"); 
        context.insert("display_delete_post", &"none");
    }

    let guest_cookie = rep.cookie("guest");

    println!("{:?}", &guest_cookie);
    match guest_cookie {
        Some(_) => {
            context.insert("display_signin", &"none"); 
            context.insert("display_comment", &"block");
        }
        None => {
            context.insert("display_signin", &"block");
            context.insert("display_comment", &"none");
        }
    };

    let s = tmpl.render("post.html", &context).unwrap();
   
    HttpResponse::Ok().content_type("text/html").body(s)
}

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
            .service(load_post)
            .service(guest_login)
            .service(Files::new("/", "statics/"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}