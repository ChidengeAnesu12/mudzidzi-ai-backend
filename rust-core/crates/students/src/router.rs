use axum::{routing::get, Router};
use common::AppState;

use crate::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/students/me", get(handlers::get_profile))
        .route("/students/me/mastery", get(handlers::get_mastery))
        .route("/students/me/progress", get(handlers::get_progress))
        .route("/students/me/recommendations", get(handlers::get_recommendations))
}