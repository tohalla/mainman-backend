use actix_http::Response;
use actix_service::Service;
use actix_web::web::{self, Data};
use futures::future::{ok, Either::Left};

use crate::{
    db::Pool,
    error::{Error, ErrorResponse},
};

pub fn organisation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::get_entities)
        .service(super::handler::create_entity)
        .service(super::handler::get_entity)
        .service(super::handler::patch_entity)
        .service(super::handler::maintainers)
        .service(super::handler::delete_maintainers)
        .service(super::handler::maintenance_triggers)
        .service(super::handler::maintenance_requests)
        .service(super::handler::create_maintenance_trigger)
        .service(super::handler::delete_maintenance_trigger)
        .service(super::handler::add_maintainers)
        .service(
            web::scope("/{uuid}")
                // middleware to check that the entity exists within the accessed organisation
                .wrap_fn(|req, srv| {
                    let path_info =
                        req.match_info().load::<(i64, uuid::Uuid)>();
                    let conn =
                        &req.app_data::<Data<Pool>>().unwrap().get().unwrap();

                    if let Ok((organisation_id, entity)) = path_info {
                        if super::Entity::get(entity, organisation_id, conn)
                            .is_ok()
                        {
                            return srv.call(req);
                        }
                    }

                    Left(Box::pin(ok(req.into_response(
                        Response::Unauthorized()
                            .body(json!(ErrorResponse::new()
                                .add_error(Error::unauthorized())))
                            .into_body(),
                    ))))
                }),
        );
}
