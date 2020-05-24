use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

pub async fn start() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(super::db::add_pool)
            .configure(super::cache::add_cache)
            .configure(super::routes::routes)
    });

    server = if let Some(l) = ListenFd::from_env().take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:8080")?
    };

    server.run().await
}
