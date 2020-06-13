use actix_web::web;

use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(RequireAuthentication)
            .route("", web::get().to(super::handler::get_organisations))
            .route("", web::post().to(super::handler::create_organisation))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(super::handler::get_organisation))
                    .route(web::patch().to(super::handler::patch_organisation)),
            ),
    );
}
