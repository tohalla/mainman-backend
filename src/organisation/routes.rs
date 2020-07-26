use actix_web::web;

use crate::appliance;
use crate::auth::middleware::RequireAuthentication;
use crate::maintainer;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/{organisation_id}")
                    .wrap(RequireAuthentication::default())
                    .route("", web::get().to(super::handler::get_organisation))
                    .route(
                        "",
                        web::patch().to(super::handler::patch_organisation),
                    )
                    .service(
                        web::scope("/appliances")
                            .configure(appliance::routes::organisation_routes),
                    )
                    .service(
                        web::scope("/maintainers")
                            .configure(maintainer::routes::organisation_routes),
                    ),
            )
            .wrap(RequireAuthentication::default())
            .route("", web::get().to(super::handler::get_organisations))
            .route("", web::post().to(super::handler::create_organisation)),
    );
}
