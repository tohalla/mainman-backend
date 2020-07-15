use actix_web::web;

use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(RequireAuthentication)
            .route("", web::get().to(super::handler::get_appliances))
            .route("", web::post().to(super::handler::create_appliance))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(super::handler::get_appliance)),
            ),
    );
}
