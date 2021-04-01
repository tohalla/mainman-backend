use actix_web::web;

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_templates)
        .service(super::handler::create_template)
        .service(super::handler::get_template)
        .service(super::handler::patch_template);
}
