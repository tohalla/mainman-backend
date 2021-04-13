use actix_web::web;

use super::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::maintenance_trigger)
        .service(handler::create_maintenance_request)
        .service(handler::template);
}

pub fn entity_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::maintenance_triggers)
        .service(handler::create_maintenance_trigger)
        .service(handler::delete_maintenance_trigger);
}
