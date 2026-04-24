//! Unified application error with [`axum::response::IntoResponse`].
//!
//! Upstream crates return their own error types (`StorageError` from
//! `peisear-storage`, `AuthError` from `peisear-auth`). This
//! type is the HTTP‑aware envelope they get converted into via `From`,
//! so handlers can uniformly `?` their way through stacks of calls and
//! still end up with a correct HTTP response.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use peisear_auth::AuthError;
use peisear_storage::StorageError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("authentication required")]
    Unauthorized,

    #[error("permission denied")]
    Forbidden,

    #[error("resource not found")]
    NotFound,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn public_message(&self) -> String {
        match self {
            Self::Internal(_) => "An internal error occurred. Please try again.".to_string(),
            other => other.to_string(),
        }
    }
}

impl From<StorageError> for AppError {
    fn from(e: StorageError) -> Self {
        match e {
            StorageError::NotFound => Self::NotFound,
            StorageError::Database(inner) => {
                tracing::error!(error = %inner, "database error");
                Self::Internal("database error".into())
            }
            StorageError::Migration(inner) => {
                tracing::error!(error = %inner, "migration error");
                Self::Internal("migration error".into())
            }
            StorageError::InvalidData(msg) => {
                tracing::error!(%msg, "invalid data in storage");
                Self::Internal("invalid storage state".into())
            }
            StorageError::Bootstrap(msg) => Self::Internal(msg),
        }
    }
}

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        match e {
            // JWT decode failures almost always mean the cookie is stale
            // or tampered with, which we map to "not signed in" — the
            // IntoResponse impl below converts that into a 303 to /login.
            AuthError::Jwt(inner) => {
                tracing::warn!(error = %inner, "jwt error");
                Self::Unauthorized
            }
            AuthError::PasswordHash(msg) => {
                tracing::error!(%msg, "password hash error");
                Self::Internal("authentication subsystem error".into())
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let Self::Internal(msg) = &self {
            tracing::error!(%msg, "internal error");
        } else {
            tracing::debug!(error = %self, "request error");
        }

        // Unauthorized browser requests are redirected to login instead
        // of rendering a 401 page, which matches typical web UX.
        if matches!(self, Self::Unauthorized) {
            return Redirect::to("/login").into_response();
        }

        let status = self.status();
        let message = self.public_message();
        let html =
            crate::components::error_page::render_error(status.as_u16(), message.clone());

        // Empty body rendered would fall through to JSON; empty is
        // unlikely but we guard against it anyway.
        let axum::response::Html(body) = &html;
        if body.is_empty() {
            (
                status,
                axum::Json(json!({ "error": message, "status": status.as_u16() })),
            )
                .into_response()
        } else {
            (status, html).into_response()
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
