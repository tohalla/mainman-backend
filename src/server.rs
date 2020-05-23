use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

use crate::routes::routes;

pub async fn start() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().wrap(Logger::default()).configure(routes));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
