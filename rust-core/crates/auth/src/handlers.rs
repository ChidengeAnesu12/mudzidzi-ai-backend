use anyhow::anyhow;
use axum::{extract::State, Json};
use common::{password, AppError, AppResult, AppState};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::dto::{AuthResponse, LoginRequest, RefreshRequest, RegisterRequest, UserResponse};
use crate::repository;
use crate::validation;

const REFRESH_SESSION_PREFIX: &str = "refresh_session:";

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    validation::validate_register(&payload)?;

    let password_hash = password::hash_password(&payload.password)
        .map_err(|_| AppError::Internal(anyhow!("password hashing failed")))?;

    let user_id = repository::create_user_with_profile(
        &state.db,
        &payload.full_name,
        &payload.email,
        &password_hash,
        &payload.role,
    )
    .await
    .map_err(|err| {
        if is_unique_violation(&err) {
            AppError::Conflict("An account with this email already exists.".to_string())
        } else {
            AppError::Database(err)
        }
    })?;

    tracing::info!(email = %payload.email, role = %payload.role, "new user registered");

    issue_auth_response(&state, user_id, &payload.full_name, &payload.email, &payload.role).await
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    validation::validate_login(&payload)?;

    let user = repository::find_user_by_email(&state.db, &payload.email)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Unauthorized("Incorrect email or password.".to_string()))?;

    if !password::verify_password(&payload.password, &user.password_hash) {
        return Err(AppError::Unauthorized("Incorrect email or password.".to_string()));
    }

    repository::touch_last_login(&state.db, user.id)
        .await
        .map_err(AppError::Database)?;

    issue_auth_response(&state, user.id, &user.full_name, &user.email, &user.role).await
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> AppResult<Json<AuthResponse>> {
    let claims = common::jwt::decode_token(&payload.refresh_token, &state.config.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token.".to_string()))?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Invalid token type.".to_string()));
    }

    let jti = claims
        .jti
        .clone()
        .ok_or_else(|| AppError::Unauthorized("Invalid refresh token.".to_string()))?;

    let mut redis = state.redis.clone();
    let redis_key = format!("{REFRESH_SESSION_PREFIX}{jti}");

    // Atomically read-and-delete the session entry: if it's missing,
    // this refresh token was already used (rotation reuse) or expired
    // — either way, reject it. Sending the raw GETDEL command directly
    // (rather than a convenience wrapper method) guarantees correct
    // behavior regardless of exact method-naming across crate versions.
    let stored_user_id: Option<String> = redis::cmd("GETDEL")
        .arg(&redis_key)
        .query_async(&mut redis)
        .await
        .map_err(AppError::Cache)?;

    let stored_user_id = stored_user_id.ok_or_else(|| {
        AppError::Unauthorized("Refresh token has already been used or has expired.".to_string())
    })?;

    let user_id = Uuid::parse_str(&stored_user_id)
        .map_err(|_| AppError::Unauthorized("Invalid refresh token.".to_string()))?;

    if user_id.to_string() != claims.sub {
        return Err(AppError::Unauthorized("Invalid refresh token.".to_string()));
    }

    let user = repository::find_user_by_id(&state.db, user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Unauthorized("User no longer exists.".to_string()))?;

    issue_auth_response(&state, user.id, &user.full_name, &user.email, &user.role).await
}

/// Issues a fresh access + refresh token pair, stores the refresh
/// token's session in Redis (keyed by its jti), and builds the full
/// AuthResponse — including the user's school/grade profile — returned
/// to the Flutter app. Shared by register, login, and refresh so all
/// three produce an identical response shape.
async fn issue_auth_response(
    state: &AppState,
    user_id: Uuid,
    full_name: &str,
    email: &str,
    role: &str,
) -> AppResult<Json<AuthResponse>> {
    let access_token = common::jwt::create_access_token(
        user_id,
        role,
        &state.config.jwt_secret,
        state.config.jwt_access_token_ttl_minutes,
    )
    .map_err(|_| AppError::Internal(anyhow!("failed to create access token")))?;

    let (refresh_token, jti) = common::jwt::create_refresh_token(
        user_id,
        role,
        &state.config.jwt_secret,
        state.config.jwt_refresh_token_ttl_days,
    )
    .map_err(|_| AppError::Internal(anyhow!("failed to create refresh token")))?;

    let mut redis = state.redis.clone();
    let redis_key = format!("{REFRESH_SESSION_PREFIX}{jti}");
    let ttl_seconds = (state.config.jwt_refresh_token_ttl_days * 24 * 60 * 60) as u64;

    redis
        .set_ex::<_, _, ()>(&redis_key, user_id.to_string(), ttl_seconds)
        .await
        .map_err(AppError::Cache)?;

    let profile = repository::find_profile(&state.db, user_id, role)
        .await
        .map_err(AppError::Database)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            id: user_id,
            full_name: full_name.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            school_name: profile.school_name,
            grade_level: profile.grade_level,
        },
    }))
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    err.as_database_error()
        .and_then(|db_err| db_err.code())
        .map(|code| code == "23505")
        .unwrap_or(false)
}