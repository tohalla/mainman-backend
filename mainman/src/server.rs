use actix_cors::Cors;
use actix_web::{
    middleware::{normalize::TrailingSlash, Logger, NormalizePath},
    App, HttpServer,
};

use crate::events::Broadcaster;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(move || {
        let pool = super::db::get_pool();
        let broadcaster = Broadcaster::create();
        App::new()
            .data(pool.clone())
            .app_data(broadcaster.clone())
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
