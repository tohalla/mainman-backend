use actix_web::web;

use super::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::get_trigger)
        .service(handler::create_maintenance_request);
}
