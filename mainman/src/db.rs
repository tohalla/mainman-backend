use diesel::{pg::PgConnection, r2d2};

use crate::MainmanResult;

type ConnectionManager = r2d2::ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type Connection = r2d2::PooledConnection<ConnectionManager>;

pub fn get_pool() -> Pool {
    r2d2::Pool::builder()
        .build(r2d2::ConnectionManager::<PgConnection>::new(format!(
            "postgres://mainman_client:{}@db:5432/mainman",
            std::env::var("DB_PASSWORD_CLIENT").unwrap()
        )))
        .expect("Failed to create connection pool for Postgres")
}

pub trait Creatable<T> {
    fn create(&self, conn: &Connection) -> MainmanResult<T>;
}
