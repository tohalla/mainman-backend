use actix_web::web;

use super::handler;
use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::create_account).service(
        web::scope("{account_id}")
            .wrap(RequireAuthentication::default())
            .service(handler::account)
            .service(handler::invites),
    );
}

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::organisation_accounts)
        .service(handler::organisation_invites)
        .service(handler::invite_account)
        .service(handler::accept_invite)
        .service(handler::delete_invite);
}
