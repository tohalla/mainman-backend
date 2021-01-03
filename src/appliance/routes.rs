use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_appliances)
        .service(super::handler::create_appliance)
        .service(super::handler::get_appliance)
        .service(super::handler::patch_appliance);
}
