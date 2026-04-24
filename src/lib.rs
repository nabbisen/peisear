//! Shared library crate for the issue tracker.
//!
//! This exposes the modules so that integration tests (future work) can
//! exercise the same code paths as the binary.

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod views;

use sqlx::SqlitePool;

/// Application state shared with every handler.
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
}
