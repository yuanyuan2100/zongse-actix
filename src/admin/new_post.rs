use actix_web::{get, post, web, http, HttpResponse};
use actix_identity::Identity;
use tera::Context;
use serde_derive::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use nanoid::nanoid;

use crate::model::model_post::*;
use crate::utils::{connections::*, time::*};
use crate::admin::auth::*;
use crate::schema::posts;

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub id_url: &'a str,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub body: &'a str,
    pub published: bool,
    pub create_time: NaiveDateTime,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostForm {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub tags: String,
}

#[get("admin/new_post")]
pub fn get_new_post_inputpage(tmpl: web::Data<tera::Tera>, id: Identity) -> HttpResponse {
    
    match id.identity() {
        Some(_) => {
            let s = tmpl.render("admin/new_post.html", &Context::new()).unwrap();
    
            HttpResponse::Ok().content_type("text/html").body(s)
        }
        None => {
            HttpResponse::Found().header(http::header::LOCATION, "/").finish()
        }
    }
}

#[post("/create")]
pub fn create_new_post(
    form: web::Form<CreatePostForm>,
    id: Identity, 
    db: DB
) -> HttpResponse {

    match id.identity() {
        Some(_) => {

            let new_post = NewPost {
                id_url: &nanoid!(),
                title: &form.title,
                subtitle: &form.subtitle,
                body: &form.body,
                published: true,
                create_time: get_now(),
                tags: if form.tags.is_empty() {
                    vec![]
                } else {
                    form.tags.split("/").map(String::from).collect()
                },
            };

            diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result::<Post>(&*db)
                .expect("Error saving new post");

            HttpResponse::Found().header(http::header::LOCATION, "/").finish()
        }
        None => {
            HttpResponse::Found().header(http::header::LOCATION, "/").finish()
        }
    }
}
