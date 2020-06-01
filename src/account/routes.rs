use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(super::handler::create_account))
        .route("/{id}", web::get().to(super::handler::get_account));
}
