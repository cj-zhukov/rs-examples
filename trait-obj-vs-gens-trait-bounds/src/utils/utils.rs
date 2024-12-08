use crate::utils::PG_MAX_CONNECTIONS;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn configure_postgresql(url: &str) -> PgPool {
    let pg_pool = get_postgres_pool(url)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(PG_MAX_CONNECTIONS)
        .connect(url)
        .await
}