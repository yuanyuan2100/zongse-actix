use actix_web::web;

pub mod admin_login;
pub mod auth;
pub mod post;
pub mod file;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_login::get_admin_login_page)
        .service(admin_login::admin_login)
        .service(admin_login::get_admin_pages)
        .service(admin_login::change_password)
        .service(post::create_new_post)
        .service(post::get_edit_post_page)
        .service(post::edit_post)
        .service(post::delete_post)
        .service(file::upload_img);
}
