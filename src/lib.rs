#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
// #[macro_use]
// extern crate serde_json;
// #[macro_use]
// extern crate log;

mod account;
mod auth;
mod cache;
mod config;
mod db;
mod entity;
mod error;
mod health;
mod maintainer;
mod organisation;
mod response;
mod routes;
mod schema;
mod server;

pub type MainmanResult<T> = Result<T, error::Error>;
pub type MainmanResponse<T> = MainmanResult<response::Response<T>>;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();
    server::start().await
}
