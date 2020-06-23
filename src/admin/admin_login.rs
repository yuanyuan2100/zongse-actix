use actix_files::Files;

use actix_web::{middleware, App, HttpServer};
use actix_web::{get, post, web, http, HttpRequest, HttpResponse};
use actix_http::cookie::{CookieJar, Cookie, Key, SameSite};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use tera::{Tera, Context};

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
    // mut cookies: web::Data<Cookie<'_>>, 
    db: DB,
) -> HttpResponse {

    let fetched_user = User::find_by_username(&form.username, &db);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&form.password) {

                let user_id = &login_user.id;

                let _key = Key::generate();
                let mut _jar = CookieJar::new();
                let c = Cookie::build("Administrator", user_id.to_string())
                                .max_age(60*60*24*7)
                                .same_site(SameSite::Strict)
                                .path("/")
                                .finish();

                // cookies.add_private(Cookie::build("Administrator", user_id.to_string())
                //                     .max_age(Duration::days(7))
                //                     .same_site(SameSite::Strict)
                //                     .finish());
                println!("{}", &user_id);
                HttpResponse::Found().header(http::header::LOCATION, "/")
                                    .cookie(c)
                                    .finish()
                
            } else {
            Err(HttpResponse::Found().header(http::header::LOCATION, "admin/admin_login").finish()).unwrap()

            }
        }
        Err(_) => Err(HttpResponse::Found().header(http::header::LOCATION, "admin/admin_login").finish()).unwrap()
    }
}
// #[get("/")]
// async fn index(tmpl: web::Data<tera::Tera>, db: DB) -> HttpResponse {
//     use crate::schema::posts::dsl::*;

//     let mut context = Context::new();

//     let results: Vec<_> = posts
//                     .filter(published.eq(true))
//                     .limit(5)
//                     .order(id.desc())
//                     .load::<Post>(&*db)
//                     .expect("Error loading posts");
    
//     context.insert("posts", &results);
    
//     let s = tmpl.render("index.html", &context).unwrap();
   
//    HttpResponse::Ok().content_type("text/html").body(s)
// }