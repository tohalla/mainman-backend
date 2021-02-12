use actix_cors::Cors;
use actix_web::{
    middleware::{normalize::TrailingSlash, Logger, NormalizePath},
    App, HttpServer,
};

use crate::events::Broadcaster;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(super::db::get_pool())
            .app_data(Broadcaster::create())
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
