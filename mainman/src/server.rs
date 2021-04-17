use actix_cors::Cors;
use actix_web::{
    middleware::{normalize::TrailingSlash, Logger, NormalizePath},
    web::JsonConfig,
    App, HttpServer,
};

use crate::{error::ErrorResponse, events::Broadcaster};

pub async fn start() -> std::io::Result<()> {
    let pool = super::db::get_pool();
    let broadcaster = Broadcaster::create();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(broadcaster.clone())
            .app_data(JsonConfig::default().error_handler(|err, _| ErrorResponse::from(err).into()))
            .configure(super::cache::add_cache)
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .configure(super::routes::routes)
    })
    .bind("[::]:8080")?
    .run()
    .await
}
