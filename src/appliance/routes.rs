use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(super::handler::get_appliances))
        .route("", web::post().to(super::handler::create_appliance));
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{hash}")
            .route(web::get().to(super::handler::get_appliance))
            .route(web::patch().to(super::handler::patch_appliance)),
    );
}
