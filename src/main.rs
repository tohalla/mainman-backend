#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
// #[macro_use]
// extern crate log;

mod account;
mod appliance;
mod auth;
mod cache;
mod config;
mod db;
mod error;
mod health;
mod maintainer;
mod organisation;
mod routes;
mod schema;
mod server;

pub type MainmanResult<T> = Result<T, error::Error>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();
    server::start().await
}
