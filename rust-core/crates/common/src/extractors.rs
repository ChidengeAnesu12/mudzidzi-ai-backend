use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{jwt, AppError, AppState};

/// Extracted from a valid `Authorization: Bearer <access_token>`
/// header. Any handler taking `CurrentUser` as a parameter
/// automatically requires authentication — Axum runs this extractor
/// before the handler body executes. Rejections use the shared
/// `AppError` type (not a bespoke status/string pair), so every 401
/// across the whole API — whether it's a missing header or a wrong
/// password — comes back in the exact same JSON shape.
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub role: String,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header.".to_string()))?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Authorization header must use the Bearer scheme.".to_string()))?;

        let claims = jwt::decode_token(token, &state.config.jwt_secret)
            .map_err(|_| AppError::Unauthorized("Invalid or expired access token.".to_string()))?;

        if claims.token_type != "access" {
            return Err(AppError::Unauthorized("Invalid token type.".to_string()));
        }

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid token subject.".to_string()))?;

        Ok(CurrentUser { user_id, role: claims.role })
    }
}