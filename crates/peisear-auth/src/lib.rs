//! Password hashing and JWT primitives, usable from any async runtime
//! and any web framework.
//!
//! Deliberately has no axum dependency so that future non‑HTTP surfaces
//! (CLI admin tools, integration tests, a future OIDC verifier, etc.)
//! can link this crate without bringing in the server stack.

pub mod jwt;
pub mod password;

/// All error cases that this crate can produce. The consumer is
/// expected to map these onto its own application error — the web
/// crate, for instance, turns `Jwt(_)` into a 401 redirect and
/// `PasswordHash(_)` into an internal server error.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("password hashing error: {0}")]
    PasswordHash(String),
}

pub type AuthResult<T> = Result<T, AuthError>;
