use actix_web::web;

use super::handler;
use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::create_account).service(
        web::scope("{account_id}")
            .wrap(RequireAuthentication::default())
            .service(handler::get_account)
            .service(handler::get_customer_details)
            .service(handler::create_card),
    );
}
