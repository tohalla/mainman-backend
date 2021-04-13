use actix_web::web;

use super::{maintenance_request, maintenance_trigger};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").configure(maintenance_trigger::routes::routes));
}

pub fn entity_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/requests").configure(maintenance_request::routes::routes),
    )
    .service(
        web::scope("/triggers")
            .configure(maintenance_trigger::routes::entity_routes),
    );
}
