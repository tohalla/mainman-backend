#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;

mod account;
mod auth;
mod cache;
mod db;
mod error;
mod health;
mod organisation;
mod routes;
mod schema;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    server::start().await
}
