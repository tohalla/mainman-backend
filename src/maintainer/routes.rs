use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_maintainers)
        .service(super::handler::create_maintainer)
        .service(super::handler::get_maintainer)
        .service(super::handler::patch_maintainer);
}
