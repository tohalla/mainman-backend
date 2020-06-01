use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    App, HttpServer,
};
use listenfd::ListenFd;

pub async fn start() -> std::io::Result<()> {
    let pool = super::db::get_pool();
    let state = super::state::new_state::<String>();

    let mut server = HttpServer::new(move || {
        App::new()
            .configure(super::cache::add_cache)
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(Logger::default())
            .wrap(NormalizePath)
            .data(pool.clone())
            .app_data(state.clone())
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
