use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject — the user's id.
    pub sub: String,
    /// "student" or "teacher" — lets downstream handlers authorize
    /// role-specific endpoints (e.g. Teacher Dashboard) without an
    /// extra database lookup.
    pub role: String,
    /// "access" or "refresh" — prevents a refresh token being used
    /// directly as an access token if it's ever sent to a protected
    /// endpoint by mistake.
    pub token_type: String,
    pub iat: i64,
    pub exp: i64,
    /// Present only on refresh tokens — a unique id used to track this
    /// specific refresh token's session in Redis, so it can be
    /// invalidated on rotation without affecting the user's other
    /// active sessions on other devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

pub fn create_access_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    ttl_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        token_type: "access".to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::minutes(ttl_minutes)).timestamp(),
        jti: None,
    };

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

/// Creates a refresh token and returns it alongside its `jti`. The
/// caller is responsible for storing that `jti` in Redis (mapped to
/// the user id, with a matching TTL) so the token's session can later
/// be looked up, rotated, or revoked.
pub fn create_refresh_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    ttl_days: i64,
) -> Result<(String, Uuid), jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let jti = Uuid::new_v4();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        token_type: "refresh".to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::days(ttl_days)).timestamp(),
        jti: Some(jti.to_string()),
    };

    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;
    Ok((token, jti))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}