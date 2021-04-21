use actix_web::web;

use super::handler;
use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(RequireAuthentication::default())
            .service(handler::get_customer_details)
            .service(handler::get_payment_methods)
            .service(handler::detach_payment_method)
            .service(handler::create_payment_method),
    );
}
