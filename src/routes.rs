use actix_web::web;

use crate::account;
use crate::auth;
use crate::health::handler::get_health;
use crate::organisation;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health)).service(
        web::scope("/api/v1")
            .service(web::scope("/accounts").configure(account::routes::routes))
            .service(
                web::scope("/organisations")
                    .configure(organisation::routes::routes),
            )
            .service(web::scope("/auth").configure(auth::routes::routes)),
    );
}
