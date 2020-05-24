use actix_web::web;

use crate::health::handler::get_health;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health));
}
