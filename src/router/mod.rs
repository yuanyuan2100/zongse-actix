use actix_web::web;

pub mod index;
pub mod post;

pub fn routes(cfg: &mut web::ServiceConfig) {
    
    cfg.service(index::index)
        .service(post::load_post)
        .service(post::guest_login)
        .service(post::create_comment);
}