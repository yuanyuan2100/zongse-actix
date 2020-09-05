use actix_identity::Identity;
use actix_web::{get, http, post, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::model::model_user::*;
use crate::schema::*;
use crate::utils::connections::*;

#[derive(Serialize, Deserialize)]
pub struct AdminLoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordForm {
    pub username: String,
    pub old_password: String,
    pub new_password_1: String,
    pub new_password_2: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserForm<'a> {
    pub id: i32,
    pub username: &'a str,
    pub password: &'a str,
}

#[get("admin/admin_login")]
pub async fn get_admin_login_page(tmpl: web::Data<tera::Tera>, id: Identity) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let s = tmpl
                .render("admin/admin_panel.html", &Context::new())
                .unwrap();

            HttpResponse::Ok().content_type("text/html").body(s)
        }
        None => {
            let s = tmpl
                .render("admin/admin_login.html", &Context::new())
                .unwrap();

            HttpResponse::Ok().content_type("text/html").body(s)
        }
    }
}

#[get("admin/{admin_url}")]
pub async fn get_admin_pages(
    tmpl: web::Data<tera::Tera>,
    id: Identity,
    admin_url: web::Path<String>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let url = "admin/".to_owned() + &*admin_url + ".html";

            let s = tmpl.render(&url, &Context::new()).unwrap();

            HttpResponse::Ok().content_type("text/html").body(s)
        }
        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/admin/admin_login")
            .finish(),
    }
}

#[post("admin/login")]
pub async fn admin_login(form: web::Form<AdminLoginForm>, id: Identity, db: DB) -> HttpResponse {
    let fetched_user = User::find_by_username(&form.username, &db);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&form.password) {
                let user_id = &login_user.id;

                id.remember(user_id.to_owned().to_string());

                HttpResponse::Found()
                    .header(http::header::LOCATION, "admin_panel")
                    .finish()
            } else {
                Err(HttpResponse::Found()
                    .header(http::header::LOCATION, "admin/admin_login")
                    .finish())
                .unwrap()
            }
        }
        Err(_) => Err(HttpResponse::Found()
            .header(http::header::LOCATION, "admin/admin_login")
            .finish())
        .unwrap(),
    }
}

#[post("/submit")]
pub fn change_password(form: web::Form<ChangePasswordForm>, db: DB) -> HttpResponse {
    let fetched_users = User::find_by_username(&form.username, &db);

    match fetched_users {
        Ok(login_user) => {
            if login_user.authenticated(&form.old_password) {
                if form.new_password_1 == form.new_password_2 {
                    let hashed_password = User::password_generate(&form.new_password_1);

                    let updated_user = UpdateUserForm {
                        id: login_user.id,
                        username: &form.username,
                        password: &hashed_password,
                    };

                    diesel::update(users::table.find(login_user.id))
                        .set(&updated_user)
                        .get_result::<User>(&*db)
                        .expect("Error updating user");

                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/")
                        .finish()
                } else {
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/admin/change_password")
                        .finish()
                }
            } else {
                HttpResponse::Found()
                    .header(http::header::LOCATION, "/admin/change_password")
                    .finish()
            }
        }
        Err(_) => HttpResponse::Found()
            .header(http::header::LOCATION, "/admin/change_password")
            .finish(),
    }
}
