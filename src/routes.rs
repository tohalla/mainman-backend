use actix_web::web;

use crate::account;
use crate::health::handler::get_health;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health)).service(
        web::scope("/api/v1").service(
            web::scope("/accounts").configure(account::routes::routes),
        ),
    );
}
