//! Unified application error with [`axum::response::IntoResponse`] support.
//!
//! Every fallible code path in the app returns [`AppError`]; the error
//! type knows how to render either a JSON body (for API callers) or an
//! HTML page (for browser callers) based on the `Accept` header.

use askama::Template;
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
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

    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("password hashing error")]
    PasswordHash(String),

    #[error("jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

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
            Self::Database(_) | Self::PasswordHash(_) | Self::Jwt(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub fn public_message(&self) -> String {
        match self {
            Self::Database(_) | Self::PasswordHash(_) | Self::Internal(_) | Self::Jwt(_) => {
                "An internal error occurred. Please try again.".to_string()
            }
            other => other.to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage<'a> {
    status: u16,
    message: &'a str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the internal details regardless of what the client sees.
        match &self {
            Self::Database(e) => tracing::error!(error = %e, "database error"),
            Self::PasswordHash(e) => tracing::error!(error = %e, "password hash error"),
            Self::Jwt(e) => tracing::warn!(error = %e, "jwt error"),
            Self::Internal(e) => tracing::error!(error = %e, "internal error"),
            _ => tracing::debug!(error = %self, "request error"),
        }

        // Unauthorized browser requests are redirected to login instead of
        // rendering a 401 page, which matches typical web UX.
        if matches!(self, Self::Unauthorized) {
            return Redirect::to("/login").into_response();
        }

        let status = self.status();
        let message = self.public_message();

        let page = ErrorPage {
            status: status.as_u16(),
            message: &message,
        };
        match page.render() {
            Ok(body) => (
                status,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                body,
            )
                .into_response(),
            Err(_) => (
                status,
                axum::Json(json!({ "error": message, "status": status.as_u16() })),
            )
                .into_response(),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::PasswordHash(e.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
