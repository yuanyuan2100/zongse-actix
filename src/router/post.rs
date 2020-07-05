use actix_web::{get, post, web, http, HttpMessage, HttpResponse, HttpRequest};
use actix_http::cookie::Cookie;
use actix_identity::Identity;

use tera::Context;
use serde_derive::{Serialize, Deserialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::utils::{connections::*, email::notification, url_converter::url_converter, time::*};
use crate::model::{model_post::*, model_comment::*};
use crate::schema::*;

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

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name="comments"]
pub struct NewComment<'a> {
    pub body: &'a str,
    pub create_time: NaiveDateTime,
    pub comment_by: &'a str,
    pub comment_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CommentInput {
    pub body: String,
}

#[post("/{post_url}/comment")]
pub fn create_comment(
    post_url: web::Path<String>, 
    comment: web::Form<CommentInput>, 
    rep: HttpRequest,
    db: DB
) -> HttpResponse {

    let post = Post::find_by_url(&post_url, &db).unwrap();
    let guestname = &rep.cookie("guest").unwrap().value().to_string();

    let new_comment = NewComment {
        body: &comment.body,
        create_time: get_now(),
        comment_by: &guestname,
        comment_id: post.id,
    };

    let email_subject = format!("{} commented on {}", &guestname, &post.title);
    let _comment_notification = notification(&email_subject, &comment.body);

    diesel::insert_into(comments::table)
        .values(&new_comment)
        .get_result::<Comment>(&*db)
        .expect("Error saving new comment");
    
    HttpResponse::Found().header(http::header::LOCATION, format!("/post/{}", &post_url)).finish()
}