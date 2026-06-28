//! Authentication Service — register, login, and refresh-token
//! issuance/rotation, backed by the `users`/`students`/`teachers`
//! tables and Redis-tracked refresh sessions.

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod router;
pub mod validation;

pub use router::router;