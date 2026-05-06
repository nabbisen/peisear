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
use chrono::{DateTime, Utc};
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

    /// Optimistic-lock conflict: the client submitted an
    /// update against an entity whose `updated_at` no longer
    /// matches what the page render saw. The handler is
    /// expected to construct this with the entity's *current*
    /// `updated_at` so the response can carry it (per
    /// peisear-feature-spec-v2.1 appendix E.3.3).
    ///
    /// `entity_type` is a short kind tag (`"issue"`, `"sprint"`,
    /// `"project"`, `"team"`, `"capacity_period"`,
    /// `"team_membership"`).
    ///
    /// The HTML response renders an explanatory page urging the
    /// user to refresh and re-apply their edit. The JSON
    /// response (for `/api/*` endpoints) returns the structured
    /// shape from the spec so a future client-side conflict-
    /// resolution UI has the data it needs.
    #[error("stale optimistic lock on {entity_type} {entity_id}")]
    OptimisticLockConflict {
        entity_type: &'static str,
        entity_id: String,
        current_updated_at: DateTime<Utc>,
    },

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
            Self::Conflict(_) | Self::OptimisticLockConflict { .. } => StatusCode::CONFLICT,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn public_message(&self) -> String {
        match self {
            Self::Internal(_) => "An internal error occurred. Please try again.".to_string(),
            Self::OptimisticLockConflict { entity_type, .. } => format!(
                "Someone else updated this {entity_type} while you were editing. \
                 Please reload the page and re-apply your change so you don't \
                 overwrite their work."
            ),
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
            StorageError::Conflict(msg) => Self::Conflict(msg),
            StorageError::Validation(msg) => Self::Validation(msg),
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
        } else if let Self::OptimisticLockConflict {
            entity_type,
            entity_id,
            ..
        } = &self
        {
            // Stale-update conflicts are normal during concurrent
            // editing; log at info, not warn — we expect them.
            tracing::info!(%entity_type, %entity_id, "optimistic-lock conflict");
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

        // For optimistic-lock conflicts we render a more
        // informative page than the generic error page — it
        // names the entity type and tells the user to refresh
        // and re-apply.
        //
        // The structured JSON shape from peisear-feature-spec
        // v2.1 appendix E.3.3 is intended for `/api/*` mutation
        // endpoints. Phase A introduces optimistic locking only
        // on HTML form endpoints; when Phase B adds `/api/*`
        // mutations, we'll add a sibling `ApiAppError` type
        // whose `IntoResponse` returns the structured JSON.
        // Until then, HTML-only is consistent with the rest of
        // the error path.
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

/// Verify that a client-supplied `client_updated_at` matches
/// the entity's current `updated_at`. Returns
/// [`AppError::OptimisticLockConflict`] when the two differ —
/// the contract from peisear-feature-spec-v2.1 §21.4 ("the
/// client edited stale data; reject and tell them to refresh").
///
/// `client_updated_at_str` is the raw RFC3339 string the form
/// or JSON body sent. We parse it inside this helper rather
/// than in every handler so the validation message stays
/// consistent. Parse failure is a 400 (bad request) rather
/// than 409 — a missing or malformed timestamp is a client
/// bug, not a real conflict.
///
/// `entity_type` is the static tag carried into the conflict
/// response ("issue", "sprint", etc).
///
/// `entity_id` is the row id used in the conflict response so
/// the structured JSON shape (Phase B `/api/*` work) can echo
/// it back.
///
/// `current_updated_at` is the canonical value the storage
/// layer currently holds. If your handler did any mutating
/// query before calling this, make sure to read `updated_at`
/// **before** the mutation — otherwise you'd be comparing
/// against a fresh timestamp and never detect a stale write.
pub fn check_optimistic_lock(
    client_updated_at_str: &str,
    current_updated_at: chrono::DateTime<chrono::Utc>,
    entity_type: &'static str,
    entity_id: impl Into<String>,
) -> AppResult<()> {
    let client_dt = chrono::DateTime::parse_from_rfc3339(client_updated_at_str)
        .map_err(|_| {
            AppError::Validation(format!(
                "client_updated_at is not a valid RFC3339 timestamp: {client_updated_at_str:?}"
            ))
        })?
        .with_timezone(&chrono::Utc);

    if client_dt != current_updated_at {
        return Err(AppError::OptimisticLockConflict {
            entity_type,
            entity_id: entity_id.into(),
            current_updated_at,
        });
    }
    Ok(())
}
