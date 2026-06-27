use std::env;

/// Application configuration loaded from environment variables.
///
/// `Config::from_env()` calls `dotenvy::dotenv()` first, which searches
/// the current directory *and walks up parent directories* looking for
/// a `.env` file. Since `cargo run -p api` executes with `rust-core/`
/// as the working directory, this correctly finds the `.env` file one
/// level up at the repo root — no extra path configuration needed in
/// development. In production, real environment variables are set
/// directly and `dotenvy::dotenv()` simply finds nothing to load, which
/// is expected and not an error.
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_access_token_ttl_minutes: i64,
    pub jwt_refresh_token_ttl_days: i64,
    pub rust_api_port: u16,
    pub ai_service_internal_url: String,
}

// Manual Debug impl (instead of #[derive(Debug)]) so that accidentally
// logging a Config value (e.g. `tracing::debug!("{:?}", config)`)
// never leaks jwt_secret into logs.
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &self.database_url)
            .field("redis_url", &self.redis_url)
            .field("jwt_secret", &"<redacted>")
            .field("jwt_access_token_ttl_minutes", &self.jwt_access_token_ttl_minutes)
            .field("jwt_refresh_token_ttl_days", &self.jwt_refresh_token_ttl_days)
            .field("rust_api_port", &self.rust_api_port)
            .field("ai_service_internal_url", &self.ai_service_internal_url)
            .finish()
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        Ok(Self {
            database_url: require_env("DATABASE_URL")?,
            redis_url: require_env("REDIS_URL")?,
            jwt_secret: require_env("JWT_SECRET")?,
            jwt_access_token_ttl_minutes: parse_env("JWT_ACCESS_TOKEN_TTL_MINUTES", 15)?,
            jwt_refresh_token_ttl_days: parse_env("JWT_REFRESH_TOKEN_TTL_DAYS", 30)?,
            rust_api_port: parse_env("RUST_API_PORT", 8080)?,
            ai_service_internal_url: env::var("AI_SERVICE_INTERNAL_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
        })
    }
}

fn require_env(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::MissingVar(key.to_string()))
}

fn parse_env<T: std::str::FromStr>(key: &str, default: T) -> Result<T, ConfigError> {
    match env::var(key) {
        Ok(val) => val.parse::<T>().map_err(|_| ConfigError::InvalidVar(key.to_string())),
        Err(_) => Ok(default),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    MissingVar(String),
    #[error("invalid value for environment variable: {0}")]
    InvalidVar(String),
}