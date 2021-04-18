use actix_web::web;

use super::handler;

pub fn entity_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::maintenance_events)
        .service(handler::create_maintenance_event);
}
