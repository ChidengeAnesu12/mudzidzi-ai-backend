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
}

pub fn create_access_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    ttl_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    create_token(user_id, role, "access", secret, Duration::minutes(ttl_minutes))
}

pub fn create_refresh_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    ttl_days: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    create_token(user_id, role, "refresh", secret, Duration::days(ttl_days))
}

fn create_token(
    user_id: Uuid,
    role: &str,
    token_type: &str,
    secret: &str,
    ttl: Duration,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        token_type: token_type.to_string(),
        iat: now.timestamp(),
        exp: (now + ttl).timestamp(),
    };

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}