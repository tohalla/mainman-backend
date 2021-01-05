use actix_web::web;

use super::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::authenticate).service(
        web::scope("")
            .wrap(RequireAuthentication::default())
            .service(super::handler::get_account)
            .service(super::handler::sign_out),
    );
}
