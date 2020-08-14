use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use tera::Context;

use crate::utils::{connections::*};
use crate::model::{model_post::*};

#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>, db: DB) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let mut context = Context::new();

    let results: Vec<_> = posts
                    .filter(published.eq(true))
                    .limit(5)
                    .order(id.desc())
                    .load::<Post>(&*db)
                    .expect("Error loading posts");
    
    context.insert("posts", &results);
    
    let s = tmpl.render("index.html", &context).unwrap();
   
   HttpResponse::Ok().content_type("text/html").body(s)
}

#[get("/list")]
async fn post_list(tmpl: web::Data<tera::Tera>, db: DB) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let mut context = Context::new();

    let results: Vec<_> = posts
                    .filter(published.eq(true))
                    .order(id.desc())
                    .load::<Post>(&*db)
                    .expect("Error loading posts");
    
    context.insert("posts", &results);
    
    let s = tmpl.render("list.html", &context).unwrap();
   
   HttpResponse::Ok().content_type("text/html").body(s)
}

#[get("/{tag}")]
async fn list_by_tag(tmpl: web::Data<tera::Tera>, tag: web::Path<Vec<String>>, db: DB) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let mut context = Context::new();

    let results: Vec<_> = posts
                    .filter(published.eq(true))
                    .filter(tags.contains(&*tag))
                    .order(id.desc())
                    .load::<Post>(&*db)
                    .expect("Error loading posts");
    
    context.insert("posts", &results);
    
    let s = tmpl.render("list.html", &context).unwrap();
   
   HttpResponse::Ok().content_type("text/html").body(s)
}

#[get("/about")]
async fn about(tmpl: web::Data<tera::Tera>) -> HttpResponse {

    let context = Context::new();
    
    let s = tmpl.render("about.html", &context).unwrap();
   
   HttpResponse::Ok().content_type("text/html").body(s)
}