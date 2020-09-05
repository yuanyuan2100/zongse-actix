use actix_web::web;

pub mod index;
pub mod post;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(index::post_list)
        .service(index::list_by_tag)
        .service(index::about)
        .service(post::load_post)
        .service(post::guest_login)
        .service(post::create_comment);
}
