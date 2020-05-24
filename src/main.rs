mod cache;
mod db;
mod health;
mod routes;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    server::start().await
}
