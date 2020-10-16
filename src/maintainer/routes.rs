use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(super::handler::get_maintainers))
        .route("", web::post().to(super::handler::create_maintainer))
        .service(super::handler::get_maintainer)
        .service(super::handler::patch_maintainer);
}
