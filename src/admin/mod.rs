use actix_web::web;

pub mod admin_login;
pub mod auth;
// pub mod change_password;
// pub mod admin_panel;
pub mod post;

pub fn routes(cfg: &mut web::ServiceConfig) {
    
    cfg.service(post::get_new_post_inputpage)
        .service(post::create_new_post)
        .service(post::get_edit_post_page)
        .service(post::edit_post)
        .service(post::delete_post);
}