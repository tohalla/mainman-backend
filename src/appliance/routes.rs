use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(super::handler::get_appliances))
        .route("", web::post().to(super::handler::create_appliance))
        .service(super::handler::get_appliance)
        .service(super::handler::patch_appliance);
}
