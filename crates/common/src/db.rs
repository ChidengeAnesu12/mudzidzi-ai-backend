use sqlx::postgres::{PgPool, PgPoolOptions};

/// Creates a PostgreSQL connection pool. A pool (rather than a single
/// connection) is required because many Axum request handlers run
/// concurrently, and each needs its own connection checked out from
/// the pool for the duration of its query.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}