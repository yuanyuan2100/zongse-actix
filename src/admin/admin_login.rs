use actix_web::{get, post, web, http, HttpResponse};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::utils::connections::*;
use crate::model::model_user::*;

#[derive(Serialize, Deserialize)]
pub struct AdminLoginForm {
    pub username: String,
    pub password: String,
}

#[get("admin/admin_login")]
pub async fn get_admin_login_page(tmpl: web::Data<tera::Tera>) -> HttpResponse {

    let s = tmpl.render("admin/admin_login.html", &Context::new()).unwrap();
    
    HttpResponse::Ok().content_type("text/html").body(s)
}

#[post("admin/login")]
pub async fn admin_login(
    form: web::Form<AdminLoginForm>,
    id: Identity, 
    db: DB,
) -> HttpResponse {

    let fetched_user = User::find_by_username(&form.username, &db);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&form.password) {

                let user_id = &login_user.id;

                id.remember(user_id.to_owned().to_string());

                HttpResponse::Found().header(http::header::LOCATION, "admin_panel")
                                    .finish()
                
            } else {
            Err(HttpResponse::Found().header(http::header::LOCATION, "admin/admin_login").finish()).unwrap()

            }
        }
        Err(_) => Err(HttpResponse::Found().header(http::header::LOCATION, "admin/admin_login").finish()).unwrap()
    }
}

#[get("admin/admin_panel")]
pub async fn get_admin_panel_page(tmpl: web::Data<tera::Tera>, id: Identity) -> HttpResponse {

    match id.identity() {
        Some(_) => {
            let s = tmpl.render("admin/admin_panel.html", &Context::new()).unwrap();
    
            HttpResponse::Ok().content_type("text/html").body(s)
        }
        None => {
            HttpResponse::Found().header(http::header::LOCATION, "/").finish()
        }
    }
}