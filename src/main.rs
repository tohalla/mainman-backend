#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;

mod account;
mod appliance;
mod auth;
mod cache;
mod db;
mod error;
mod health;
mod maintainer;
mod organisation;
mod routes;
mod schema;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    server::start().await
}
