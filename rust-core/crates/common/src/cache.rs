use redis::aio::ConnectionManager;

/// Creates a Redis connection manager — a cheaply-cloneable,
/// auto-reconnecting async connection. Unlike PostgreSQL, Redis
/// doesn't need a traditional connection pool here: ConnectionManager
/// multiplexes many concurrent commands over one connection and is
/// safe to clone into every request handler via AppState.
pub async fn create_redis_manager(redis_url: &str) -> Result<ConnectionManager, redis::RedisError> {
    let client = redis::Client::open(redis_url)?;
    client.get_connection_manager().await
}