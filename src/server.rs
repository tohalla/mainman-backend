use actix_cors::Cors;
use actix_web::{
    middleware::{normalize::TrailingSlash, Logger, NormalizePath},
    App, HttpServer,
};

use crate::auth;

pub async fn start() -> std::io::Result<()> {
    let pool = super::db::get_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(auth::middleware::default())
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
