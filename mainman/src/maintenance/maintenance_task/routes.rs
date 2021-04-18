use actix_web::web;

use super::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::accept)
        .service(handler::maintenance_task)
        .service(handler::resolve)
        .service(handler::template);
}
