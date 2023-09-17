use crate::config::get_config;
use sqlx::postgres::PgPoolOptions;
use std::sync::OnceLock;

static PG_POOL: OnceLock<sqlx::PgPool> = OnceLock::new();

pub fn get_pool() -> &'static sqlx::PgPool {
    PG_POOL.get_or_init(|| {
        let config = get_config();
        PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(&config.database_url)
            .expect("Failed to connect to Postgres")
    })
}
