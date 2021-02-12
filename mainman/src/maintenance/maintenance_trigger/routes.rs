use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::create_maintenance_request);
}
