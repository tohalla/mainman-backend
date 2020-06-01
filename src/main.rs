#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;

mod account;
mod cache;
mod db;
mod error;
mod health;
mod routes;
mod schema;
mod server;
mod state;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    server::start().await
}
