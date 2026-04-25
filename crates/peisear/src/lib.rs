//! peisear — a minimal, sophisticated, solid, and easy issue tracker.
//!
//! This is the public crates.io face of the project. The actual
//! implementation is split across four sibling crates that this crate
//! re-exports:
//!
//! - [`core`] — pure domain types (`User`, `Project`, `Issue`,
//!   `IssueStatus`, `Priority`, `CurrentUser`).
//! - [`auth`] — password hashing (argon2id) and JWT issue/verify,
//!   framework-agnostic.
//! - [`storage`] — sqlx-backed persistence layer with per-table query
//!   modules and a swappable [`storage::Pool`] type alias.
//! - [`web`] — the axum + Leptos SSR HTTP surface, including
//!   [`web::build_router`] and [`web::AppState`].
//!
//! The runnable server binary (`peisear`) is provided by this crate as
//! `cargo install peisear`. End users will normally only need that;
//! the re-exports here exist so consumers integrating peisear's
//! libraries — for instance, a CLI admin tool sharing the domain
//! vocabulary, or a custom front-end on the same storage — can depend
//! on a single crate (`peisear`) instead of pinning every sub-crate
//! individually.
//!
//! ## Example: bootstrapping the same server the binary does
//!
//! ```no_run
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use peisear::storage::pool;
//! use peisear::web::{AppState, Config, build_router};
//! use std::net::SocketAddr;
//!
//! let config = Config::from_env();
//! let db = pool::connect(&config.database_url).await?;
//! pool::migrate(&db).await?;
//!
//! let state = AppState {
//!     db,
//!     jwt_secret: config.jwt_secret,
//!     cookie_secure: config.cookie_secure,
//! };
//! let app = build_router(state);
//!
//! let addr: SocketAddr = config.bind_addr.parse()?;
//! let listener = tokio::net::TcpListener::bind(addr).await?;
//! axum::serve(listener, app).await?;
//! # Ok(())
//! # }
//! ```
//!
//! See [the README](https://github.com/nabbisen/peisear) for the
//! architecture rationale and the
//! [docs/ tree](https://github.com/nabbisen/peisear/tree/main/docs)
//! for guides on installation, configuration, deployment, and
//! security.

pub use peisear_auth as auth;
pub use peisear_core as core;
pub use peisear_storage as storage;
pub use peisear_web as web;
