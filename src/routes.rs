use actix_web::web;

use crate::account;
use crate::appliance;
use crate::auth;
use crate::health::handler::get_health;
use crate::maintainer;
use crate::organisation;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health)).service(
        web::scope("/api/v1")
            .service(web::scope("/accounts").configure(account::routes::routes))
            .service(
                web::scope("/organisations")
                    .configure(organisation::routes::routes),
            )
            .service(
                web::scope("/appliances").configure(appliance::routes::routes),
            )
            .service(
                web::scope("/maintainers")
                    .configure(maintainer::routes::routes),
            )
            .service(web::scope("/auth").configure(auth::routes::routes)),
    );
}
