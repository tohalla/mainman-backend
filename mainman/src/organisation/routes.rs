use actix_web::web;

use crate::{account, entity, maintainer};
use crate::{auth::middleware::RequireAuthentication, template};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{organisation_id}")
            .wrap(RequireAuthentication::default())
            .service(super::handler::get_organisation)
            .service(super::handler::patch_organisation)
            .service(web::scope("/entities").configure(entity::routes::organisation_routes))
            .service(web::scope("/maintainers").configure(maintainer::routes::organisation_routes))
            .service(web::scope("/accounts").configure(account::routes::organisation_routes))
            .service(web::scope("/templates").configure(template::routes::organisation_routes)),
    )
    .service(
        web::scope("")
            .wrap(RequireAuthentication::default())
            .service(super::handler::get_organisations)
            .service(super::handler::create_organisation),
    );
}
