use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "student".to_string()
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub role: String,
    pub school_name: Option<String>,
    pub grade_level: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}