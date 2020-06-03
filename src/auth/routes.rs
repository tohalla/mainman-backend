use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(super::handler::authenticate));
    cfg.route("", web::delete().to(super::handler::sign_out));
}
