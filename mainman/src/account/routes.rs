use actix_web::web;

use super::handler;
use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::create_account).service(
        web::resource("/{account_id}")
            .wrap(RequireAuthentication::default())
            .route(web::get().to(handler::get_account)),
    );
}
