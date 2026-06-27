//! Shared types and utilities used across every Rust service crate
//! (auth, students, questions, analytics): environment configuration,
//! database/cache connection pools, the shared application state and
//! HTTP error type, JWT issuing/verification, and password hashing.

pub mod cache;
pub mod config;
pub mod db;
pub mod error;
pub mod jwt;
pub mod password;
pub mod state;

pub use config::{Config, ConfigError};
pub use error::{AppError, AppResult};
pub use state::AppState;