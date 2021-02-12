use actix_web::web;

use super::maintenance_trigger;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").configure(maintenance_trigger::routes::routes));
}
