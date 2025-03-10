use actix_web::web;

use crate::maintenance;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_entities)
        .service(super::handler::create_entity)
        .service(super::handler::get_entity)
        .service(super::handler::patch_entity)
        .service(super::handler::maintainers)
        .service(super::handler::delete_maintainers)
        .service(super::handler::add_maintainers)
        .service(web::scope("/{uuid}/maintenance").configure(maintenance::routes::entity_routes));
}
