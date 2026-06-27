use axum::{routing::get, Router};
use common::{cache, db, AppState, Config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env().expect("failed to load configuration from environment");

    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("failed to connect to PostgreSQL");
    tracing::info!("Connected to PostgreSQL");

    let redis_manager = cache::create_redis_manager(&config.redis_url)
        .await
        .expect("failed to connect to Redis");
    tracing::info!("Connected to Redis");

    // Touch each domain crate's placeholder so the workspace wiring is
    // verified end-to-end — replaced by real service routers as each
    // service is implemented (auth in Phase 4, students in Phase 5, etc).
    tracing::info!("{}", auth::placeholder());
    tracing::info!("{}", students::placeholder());
    tracing::info!("{}", questions::placeholder());
    tracing::info!("{}", analytics::placeholder());

    let port = config.rust_api_port;

    let state = AppState {
        db: db_pool,
        redis: redis_manager,
        config: std::sync::Arc::new(config),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind to configured port");

    tracing::info!("Mudzidzi AI core API listening on :{port}");

    axum::serve(listener, app).await.expect("server error");
}

async fn health_check() -> &'static str {
    "Mudzidzi AI Core API is running"
}