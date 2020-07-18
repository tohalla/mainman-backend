use actix_web::web;

use crate::appliance;
use crate::auth::middleware::RequireAuthentication;

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
                            .configure(appliance::routes::routes),
                    ),
            ),
    );
}
