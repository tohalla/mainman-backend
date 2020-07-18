use actix_web::web;

use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(super::handler::create_account))
        .service(
            web::resource("/{account_id}")
                .wrap(RequireAuthentication::default())
                .route(web::get().to(super::handler::get_account)),
        );
}
