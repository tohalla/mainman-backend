use actix_web::web;

use super::{
    maintenance_event, maintenance_request, maintenance_task,
    maintenance_trigger,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/triggers").configure(maintenance_trigger::routes::routes),
    )
    .service(web::scope("/tasks").configure(maintenance_task::routes::routes));
}

pub fn entity_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/requests").configure(maintenance_request::routes::routes),
    )
    .service(
        web::scope("/events")
            .configure(maintenance_event::routes::entity_routes),
    )
    .service(
        web::scope("/triggers")
            .configure(maintenance_trigger::routes::entity_routes),
    );
}
