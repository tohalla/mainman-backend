use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        super::maintenance_trigger::handler::create_maintenance_request,
    );
}
