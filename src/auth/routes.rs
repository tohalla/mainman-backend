use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(super::handler::get_account))
        .route("", web::post().to(super::handler::authenticate))
        .route("", web::delete().to(super::handler::sign_out));
}
