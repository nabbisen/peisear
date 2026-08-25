//! Web layer: axum router, Leptos SSR components, HTTP handlers.
//!
//! This crate wires the domain and persistence crates to HTTP. It owns
//! [`AppError`], the app‑wide error type whose `IntoResponse` impl turns
//! errors from lower layers into appropriate HTTP responses (redirects
//! for auth, HTML error pages for everything else).
//!
//! `recursion_limit` raised from the 128 default: Leptos's `view!` macro
//! encodes each nested HTML element as another layer of generic type
//! (`HtmlElement<Div, ..., HtmlElement<Ul, ..., HtmlElement<Li, ...>>>`),
//! and `I18N-005d`'s `/today` dashboard (`components/me.rs`) is deep
//! enough — panels, chips, and conditional sections nested several
//! levels into `AppShell` — that the default limit overflows during
//! test-binary codegen's layout computation (`cargo check` doesn't hit
//! it; full codegen for `cargo test`/`cargo build` does). Not a logic
//! bug — rustc's own suggested fix for exactly this error.
#![recursion_limit = "256"]

pub mod app;
pub mod components;
pub mod config;
#[cfg(test)]
mod dec_007_ci_scan;
#[cfg(test)]
mod dec_007_scan;
pub mod error;
pub mod extractors;
pub mod handlers;
pub mod jobs;
#[cfg(test)]
mod prose_scan;
pub mod state;
#[cfg(test)]
mod static_js_scan;
#[cfg(test)]
mod test_harness_scan;

pub use app::build_router;
pub use config::Config;
pub use error::{ApiAppError, ApiAppResult, AppError, AppResult};
pub use state::AppState;
