use futures::future::join;

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate validator;

mod account;
mod auth;
mod billing;
mod cache;
mod config;
mod db;
mod entity;
mod error;
mod events;
mod health;
mod initialize;
mod maintainer;
mod maintenance;
mod organisation;
mod response;
mod routes;
mod schema;
mod server;
mod template;

pub type MainmanResult<T> = Result<T, error::ErrorResponse>;
pub type MainmanResponse<T> = MainmanResult<response::Response<T>>;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let (_, server) = join(initialize::initialize(), server::start()).await;
    server
}
