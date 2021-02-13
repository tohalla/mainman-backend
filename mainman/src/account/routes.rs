use actix_web::web;

use super::handler;
use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::create_account).service(
        web::scope("{account_id}")
            .wrap(RequireAuthentication::default())
            .service(handler::account),
    );
}

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::create_account)
        .service(web::scope("").service(handler::organisation_accounts));
}
