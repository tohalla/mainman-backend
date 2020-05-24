use diesel::{pg::PgConnection, r2d2};

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_pool() -> Pool {
    r2d2::Pool::builder()
        .build(r2d2::ConnectionManager::<PgConnection>::new(format!(
            "postgres://mainman_client:{}@db:5432/mainman",
            std::env::var("DB_PASSWORD_CLIENT").unwrap()
        )))
        .expect("Failed to create connection pool for Postgres")
}
