//! Web layer: axum router, Leptos SSR components, HTTP handlers.
//!
//! This crate wires the domain and persistence crates to HTTP. It owns
//! [`AppError`], the app‑wide error type whose `IntoResponse` impl turns
//! errors from lower layers into appropriate HTTP responses (redirects
//! for auth, HTML error pages for everything else).

pub mod app;
pub mod components;
pub mod config;
pub mod error;
pub mod extractors;
pub mod handlers;
pub mod state;

pub use app::build_router;
pub use config::Config;
pub use error::{AppError, AppResult};
pub use state::AppState;
