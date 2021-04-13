use actix_web::web;

use super::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::maintenance_requests)
        .service(handler::maintenance_request);
}
