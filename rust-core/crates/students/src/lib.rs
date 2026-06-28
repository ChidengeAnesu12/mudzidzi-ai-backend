//! Student Service — student profile reading, mastery tracking, and
//! progress summaries, backed by `students`, `mastery_scores`,
//! `attempts`, and `recommendations`.
//!
//! Scope note: `mastery_scores` and `recommendations` are populated by
//! the Python AI service (Bayesian Knowledge Tracing + Recommendation
//! Service — later phases), so these endpoints correctly return
//! empty/zero data for any student until then. Concepts that exist
//! only in the Flutter app's current mock data — weekly goals, a
//! recent-activity feed, and achievements/badges — have no backing
//! table in the original schema design, so they're intentionally out
//! of scope here.

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod router;
pub mod streak;

pub use router::router;