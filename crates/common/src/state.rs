use std::sync::Arc;

use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::config::Config;

/// Shared application state injected into every Axum router across
/// every service crate (auth, students, questions, analytics) via
/// `Router::with_state`. Defined here in `common` — rather than in the
/// `api` crate — so every service crate can build routers against this
/// same type without depending on `api` (which would create a circular
/// dependency, since `api` depends on all of them).
///
/// Cheap to clone: `PgPool` and `ConnectionManager` are both internally
/// reference-counted, and `Config` is wrapped in an `Arc`.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub config: Arc<Config>,
}