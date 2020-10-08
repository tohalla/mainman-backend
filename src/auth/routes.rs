use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_account)
        .service(super::handler::authenticate)
        .service(super::handler::refresh_session)
        .service(super::handler::sign_out);
}
