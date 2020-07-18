use actix_web::web;

use crate::auth::middleware::RequireAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(super::handler::create_account))
        .service(
            web::resource("/{account_id}")
                .wrap(RequireAuthentication {
                    validate: |path_info, authentication_details| {
                        if let Some(account_id) = path_info.account_id {
                            return account_id
                                == authentication_details.account_id;
                        }
                        false
                    },
                })
                .route(web::get().to(super::handler::get_account)),
        );
}
