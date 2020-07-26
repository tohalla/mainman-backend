use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(super::handler::get_maintainers))
        .route("", web::post().to(super::handler::create_maintainer));
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(super::handler::get_maintainer))
            .route(web::patch().to(super::handler::patch_maintainer)),
    );
}
