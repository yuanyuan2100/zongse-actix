use actix_identity::Identity;
use actix_web::{get, http, post, web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use nanoid::nanoid;
use serde_derive::{Deserialize, Serialize};
use tera::Context;

use crate::model::model_post::*;
use crate::schema::posts;
use crate::utils::{connections::*, time::*};

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "posts"]
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

#[derive(Insertable, Deserialize, Serialize, Debug, AsChangeset)]
#[table_name = "posts"]
pub struct EditedPost<'a> {
    pub id_url: &'a str,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub body: &'a str,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EditPostForm {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub tags: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct DeletePostForm {
    pub id: i32,
    pub published: bool,
}

#[post("/create")]
pub fn create_new_post(form: web::Form<CreatePostForm>, id: Identity, db: DB) -> HttpResponse {
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

            HttpResponse::Found()
                .header(http::header::LOCATION, "/")
                .finish()
        }
        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish(),
    }
}

#[get("/{post_url}/admin/edit_post")]
pub fn get_edit_post_page(
    tmpl: web::Data<tera::Tera>,
    id: Identity,
    post_url: web::Path<String>,
    db: DB,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {

            let mut context = Context::new();

            let post = Post::find_by_url(&post_url, &db).unwrap();

            context.insert("post", &post);
            context.insert("post_url", &post.id_url);

            let s = tmpl.render("admin/edit_post.html", &context).unwrap();

            HttpResponse::Ok().content_type("text/html").body(s)
        }
        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish(),
    }
}

#[post("/{post_url}/edit_post")]
pub fn edit_post(
    post_url: web::Path<String>,
    form: web::Form<EditPostForm>,
    id: Identity,
    db: DB,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let edited_post = EditedPost {
                id_url: &post_url,
                title: &form.title,
                subtitle: &form.subtitle,
                body: &form.body,
                tags: if form.tags.is_empty() {
                    vec![]
                } else {
                    form.tags.split("/").map(String::from).collect()
                },
            };

            let post = Post::find_by_url(&post_url, &db).unwrap();

            diesel::update(&post)
                .set(&edited_post)
                .get_result::<Post>(&*db)
                .expect("Error edit post");

            let url = &post_url.to_string();

            HttpResponse::Found()
                .header(http::header::LOCATION, format!("/post/{}", &url))
                .finish()
        }

        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish(),
    }
}

#[post("/{post_url}/delete_post")]
pub fn delete_post(post_url: web::Path<String>, id: Identity, db: DB) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let post = Post::find_by_url(&post_url, &db).unwrap();
            let delete_post = DeletePostForm {
                id: post.id,
                published: false,
            };

            diesel::update(posts::table.find(post.id))
                .set(&delete_post)
                .get_result::<Post>(&*db)
                .expect("Error deleting this post.");

            HttpResponse::Found()
                .header(http::header::LOCATION, "/")
                .finish()
        }
        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish(),
    }
}
