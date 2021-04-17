use actix_web::web;

use crate::{
    account, auth, billing, events,
    health::handler::get_health,
    maintenance,
    organisation::{self, plan},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health)
        .service(web::scope("/accounts").configure(account::routes::routes))
        .service(web::scope("/billing").configure(billing::routes::routes))
        .service(web::scope("/plans").service(plan::handler::get_plans))
        .service(web::scope("/organisations").configure(organisation::routes::routes))
        .service(web::scope("/auth").configure(auth::routes::routes))
        .service(web::scope("/maintenance").configure(maintenance::routes::routes))
        .service(web::scope("/events").configure(events::routes::routes));
}
