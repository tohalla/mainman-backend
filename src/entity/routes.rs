use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_entities)
        .service(super::handler::create_entity)
        .service(super::handler::get_entity)
        .service(super::handler::patch_entity)
        .service(super::handler::add_maintainers);
}
