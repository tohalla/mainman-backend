#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

mod cache;
mod db;
mod health;
mod routes;
mod schema;
mod server;

mod account;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    server::start().await
}
