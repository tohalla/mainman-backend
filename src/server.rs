use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    App, HttpServer,
};
use listenfd::ListenFd;

use crate::auth;

pub async fn start() -> std::io::Result<()> {
    let pool = super::db::get_pool();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(auth::middleware::default())
            .configure(super::cache::add_cache)
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(NormalizePath)
            .wrap(Logger::default())
            .configure(super::routes::routes)
    });

    server = if let Some(l) = ListenFd::from_env().take_tcp_listener(0).unwrap()
    {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:8080")?
    };

    server.run().await
}
