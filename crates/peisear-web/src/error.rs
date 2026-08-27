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
use peisear_i18n::{EntityKind, MessageKey};
use peisear_storage::StorageError;
use serde_json::json;

use crate::components::t;

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
    /// `entity_type` is the closed set of entity kinds this
    /// conflict can name (`I18N-005e`: was `&'static str`, now the
    /// `peisear-i18n` enum that already existed for exactly this
    /// purpose — `I18N-001` seeded `MessageKey::OptimisticLockConflict`
    /// against it but nothing constructed `AppError` with a typed
    /// value until now).
    ///
    /// The HTML response renders an explanatory page urging the
    /// user to refresh and re-apply their edit. The JSON
    /// response (for `/api/*` endpoints) returns the structured
    /// shape from the spec so a future client-side conflict-
    /// resolution UI has the data it needs.
    #[error("stale optimistic lock on {entity_type:?} {entity_id}")]
    OptimisticLockConflict {
        entity_type: EntityKind,
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

    /// The HTTP status an optimistic-lock conflict actually produces
    /// — `JS-003` (RFC 011 step 2). `dm.js`'s and `board.js`'s copy
    /// islands read this value instead of hardcoding `409` a fourth
    /// time; if the mapping above ever changes, the value they check
    /// against changes with it, rather than needing a second, easily
    /// forgotten edit. Constructs a real `OptimisticLockConflict` and
    /// reads `.status()` off it — the same call
    /// [`check_optimistic_lock`] triggers on a genuine stale write —
    /// rather than writing `StatusCode::CONFLICT` out a second time by
    /// hand, which would prove nothing if the match arm above ever
    /// changed. The field values are placeholders; `.status()` matches
    /// only on the variant.
    ///
    /// **`ApiAppError::status()` (below `error.rs`, the `/api/*`
    /// sibling) maps its own `OptimisticLockConflict` arm to
    /// `StatusCode::CONFLICT` too, but as a second, independently
    /// maintained `match` arm on a different type — nothing keeps the
    /// two in sync.** They agree today. This function derives only
    /// from `AppError::status()`, the mapping actually in effect for
    /// the status-change endpoint these two scripts call
    /// (`/projects/{id}/issues/{id}/status`, not an `/api/*` route).
    pub fn conflict_status_code() -> u16 {
        Self::OptimisticLockConflict {
            entity_type: EntityKind::Issue,
            entity_id: String::new(),
            current_updated_at: chrono::Utc::now(),
        }
        .status()
        .as_u16()
    }

    pub fn public_message(&self) -> String {
        match self {
            // `IntoResponse` (below) redirects `Unauthorized` to `/login`
            // before `public_message()` is ever called, so this arm is
            // unreachable in practice — kept as a harmless string rather
            // than `unreachable!()` so a future refactor that removed
            // that guard would degrade to a generic message instead of
            // panicking.
            Self::Unauthorized => "authentication required".to_string(),
            Self::Forbidden => t(MessageKey::Forbidden),
            Self::NotFound => t(MessageKey::NotFound),
            Self::Internal(_) => t(MessageKey::InternalError),
            Self::OptimisticLockConflict { entity_type, .. } => {
                t(MessageKey::OptimisticLockConflict {
                    entity: *entity_type,
                })
            }
            // `Validation`'s `Display` impl (used for tracing/logs) is
            // prefixed "validation failed: " for developer readability.
            // That prefix is failure framing and must not reach the
            // user (§1.7) — return the caller-supplied message as-is.
            Self::Validation(msg) => msg.clone(),
            // `I18N-005e` fix: `Conflict`'s `Display` impl is prefixed
            // "conflict: " the same way `Validation`'s is — but unlike
            // `Validation`, nothing stripped it before this handoff,
            // so every `AppError::Conflict(msg)` rendered as
            // "conflict: {msg}" to the user. Same bug class DEV-001
            // fixed on `Validation`, found here on `Conflict` while
            // converting — fixed, not just converted around.
            Self::Conflict(msg) => msg.clone(),
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
            // Rendered here, at the crossing boundary, so AppError's
            // own Conflict/Validation shapes (already String, per
            // I18N-005e) don't ripple (I18N-006 §5).
            StorageError::Conflict(key) => Self::Conflict(t(key)),
            StorageError::Validation(key) => Self::Validation(t(key)),
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
            tracing::info!(?entity_type, %entity_id, "optimistic-lock conflict");
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
        let html = crate::components::error_page::render_error(status.as_u16(), message.clone());

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
    entity_type: EntityKind,
    entity_id: impl Into<String>,
) -> AppResult<()> {
    let client_dt = chrono::DateTime::parse_from_rfc3339(client_updated_at_str)
        .map_err(|_| {
            // User-visible (§1.7): no failure vocabulary, no raw
            // client input echoed back. A missing, empty, or
            // malformed lock value all land here — from the
            // user's perspective they are the same situation
            // (their page's version stamp can't be verified).
            // Entity-neutral: this helper backs the issue, project,
            // sprint, and capacity form paths, not just the board
            // (DEV-001-004-review.md §1.4) — board.js carries its
            // own board-specific wording for the board context.
            AppError::Validation(t(MessageKey::LockValueUnreadable))
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

// ─────────────────────────────────────────────────────────────────────
// ApiAppError — JSON-rendering sibling of AppError
// ─────────────────────────────────────────────────────────────────────
//
// AppError's `IntoResponse` impl renders HTML error pages and
// redirects unauthenticated requests to /login. That UX is right
// for browser navigation but wrong for API endpoints, where the
// caller is JavaScript (or another service) and expects:
//
// - JSON body it can parse;
// - 401 (not 303 to /login) when unauthenticated;
// - structured 409 conflicts that include `current_updated_at`
//   so a future client-side "retry with fresh value" UX has
//   the data it needs (peisear-feature-spec-v2.1 appendix E.3.3).
//
// This sibling type is wire-shape-compatible with AppError
// (carries the same variants) but its IntoResponse is JSON.
// `/api/*` handlers return Result<Json<T>, ApiAppError>;
// everything else stays on AppError.
//
// We keep `ApiAppError` as its own enum rather than wrapping
// AppError because (a) the cases each map to a specific JSON
// shape, and (b) wrapping would force a `Display` round-trip
// just to peel the JSON fields back out. A direct enum is
// clearer.

/// The `entity_type` JSON field's wire value — a stable identifier
/// clients branch on (`FR-API-004`'s "the code is not copy, it is a
/// contract" applies here the same way it does to the `error`
/// field), not copy, so it does not go through `peisear_i18n`.
/// `I18N-005e`: `entity_type` used to be the `&'static str` passed
/// straight through to `json!`; now that it is a typed `EntityKind`
/// (see `AppError::OptimisticLockConflict`'s doc comment), this
/// mapping keeps the wire value byte-identical to what it was before
/// — `EntityKind` has no `Serialize` derive on purpose, since a
/// derive's default casing (`"CapacityPeriod"`) would silently
/// change the contract.
fn entity_kind_wire_str(entity: EntityKind) -> &'static str {
    match entity {
        EntityKind::Issue => "issue",
        EntityKind::Project => "project",
        EntityKind::Sprint => "sprint",
        EntityKind::Team => "team",
        EntityKind::CapacityPeriod => "capacity_period",
        EntityKind::TeamMembership => "team_membership",
    }
}

/// JSON-shape application error for `/api/*` endpoints.
///
/// Each variant maps to a fixed HTTP status + JSON body. The
/// shapes match peisear-feature-spec-v2.1 §11.5 / appendix E.3.3.
#[derive(Debug, thiserror::Error)]
pub enum ApiAppError {
    /// Caller did not supply a valid session. Status 401, body
    /// `{ "error": "unauthorized", "message": "..." }`.
    #[error("authentication required")]
    Unauthorized,

    /// Caller is authenticated but not authorised to access
    /// this resource. Status 403, body
    /// `{ "error": "forbidden", "message": "..." }`.
    #[error("permission denied")]
    Forbidden,

    /// Resource doesn't exist (or doesn't exist *for this
    /// caller* — we deliberately don't distinguish, to avoid
    /// leaking presence). Status 404, body
    /// `{ "error": "not_found", "message": "..." }`.
    #[error("resource not found")]
    NotFound,

    /// Input validation failed. Status 400, body
    /// `{ "error": "validation", "message": "..." }`.
    #[error("validation failed: {0}")]
    Validation(String),

    /// Optimistic-lock conflict — a future Phase B/D
    /// `/api/*` mutation endpoint will return this when the
    /// caller submits with a stale `client_updated_at`.
    /// Status 409, body
    /// `{ "error": "conflict", "message": "...",
    ///    "current_updated_at": "...", "entity_type": "...",
    ///    "entity_id": "..." }`.
    #[error("stale optimistic lock on {entity_type:?} {entity_id}")]
    OptimisticLockConflict {
        entity_type: EntityKind,
        entity_id: String,
        current_updated_at: DateTime<Utc>,
    },

    /// Unhandled internal failure. Status 500, body
    /// `{ "error": "internal", "message": "..." }`. The full
    /// detail is logged server-side; the client gets a
    /// stable generic message so internals don't leak.
    #[error("internal error: {0}")]
    Internal(String),
}

impl ApiAppError {
    fn status(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::OptimisticLockConflict { .. } => StatusCode::CONFLICT,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<StorageError> for ApiAppError {
    fn from(e: StorageError) -> Self {
        match e {
            StorageError::NotFound => Self::NotFound,
            StorageError::Database(inner) => {
                tracing::error!(error = %inner, "database error (api)");
                Self::Internal("database error".into())
            }
            StorageError::Migration(inner) => {
                tracing::error!(error = %inner, "migration error (api)");
                Self::Internal("migration error".into())
            }
            StorageError::InvalidData(msg) => {
                tracing::error!(%msg, "invalid data in storage (api)");
                Self::Internal("invalid storage state".into())
            }
            StorageError::Bootstrap(msg) => Self::Internal(msg),
            // Same 400-not-409 gap Finding 5 (I18N-005e review §4)
            // already recorded as a precondition for RFC 004 D-1 —
            // not this handoff's to fix, only to carry forward.
            StorageError::Conflict(key) => Self::Validation(t(key)),
            StorageError::Validation(key) => Self::Validation(t(key)),
        }
    }
}

impl From<AuthError> for ApiAppError {
    fn from(e: AuthError) -> Self {
        match e {
            // JWT decode failures (expired or tampered cookie)
            // map to 401 — symmetric with AppError, but here we
            // do NOT redirect to /login: the client is
            // JavaScript expecting JSON.
            AuthError::Jwt(inner) => {
                tracing::warn!(error = %inner, "jwt error (api)");
                Self::Unauthorized
            }
            AuthError::PasswordHash(msg) => {
                tracing::error!(%msg, "password hash error (api)");
                Self::Internal("authentication subsystem error".into())
            }
        }
    }
}

impl IntoResponse for ApiAppError {
    fn into_response(self) -> Response {
        if let Self::Internal(msg) = &self {
            tracing::error!(%msg, "internal error (api)");
        } else if let Self::OptimisticLockConflict {
            entity_type,
            entity_id,
            ..
        } = &self
        {
            tracing::info!(?entity_type, %entity_id, "optimistic-lock conflict (api)");
        } else {
            tracing::debug!(error = %self, "request error (api)");
        }

        let status = self.status();
        // Use a single `error` keyword and a specific `message`
        // so clients can switch on the keyword without parsing
        // human-readable strings. Conflict variant adds the
        // structured fields appendix E.3.3 specifies.
        let body = match &self {
            Self::Unauthorized => json!({
                "error": "unauthorized",
                "message": t(MessageKey::ApiUnauthorizedMessage),
            }),
            Self::Forbidden => json!({
                "error": "forbidden",
                "message": t(MessageKey::ApiForbiddenMessage),
            }),
            Self::NotFound => json!({
                "error": "not_found",
                "message": t(MessageKey::ApiNotFoundMessage),
            }),
            Self::Validation(msg) => json!({
                "error": "validation",
                "message": msg,
            }),
            Self::OptimisticLockConflict {
                entity_type,
                entity_id,
                current_updated_at,
            } => json!({
                "error": "conflict",
                "message": t(MessageKey::ApiOptimisticLockConflictMessage {
                    entity: *entity_type,
                }),
                "entity_type": entity_kind_wire_str(*entity_type),
                "entity_id": entity_id,
                "current_updated_at": current_updated_at.to_rfc3339(),
            }),
            Self::Internal(_) => json!({
                "error": "internal",
                "message": t(MessageKey::InternalError),
            }),
        };

        (status, axum::Json(body)).into_response()
    }
}

pub type ApiAppResult<T> = Result<T, ApiAppError>;
