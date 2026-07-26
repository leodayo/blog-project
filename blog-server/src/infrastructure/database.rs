use sqlx::{Error, PgPool, migrate::MigrateError, postgres::PgPoolOptions};

const MAX_DB_CONNECTIONS: u32 = 5;

pub async fn create_pool(database_url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
