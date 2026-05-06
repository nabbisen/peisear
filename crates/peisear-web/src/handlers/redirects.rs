//! Legacy URL → canonical URL redirect handlers (Phase A, v0.17.0).
//!
//! These exist to keep external bookmarks, email links, and webhook
//! callbacks pointing at the old `/me` / `/notifications` paths
//! working indefinitely after the v2.1 information-architecture
//! rename to `/today` / `/inbox`.
//!
//! ## Why HTTP 308 (`Redirect::permanent`) and not 301
//!
//! Three of the four redirected routes are POST endpoints
//! (`/notifications/mark-all-read`, `/notifications/{id}/read`).
//! Per RFC 7538, **308 preserves the request method and body**
//! during the redirect, while 301 historically allowed clients
//! to silently downgrade POST to GET. Browsers vary on this for
//! 301; with 308 the contract is unambiguous.
//!
//! For the GET-only `/me` and `/notifications` routes the choice
//! is cosmetic, but keeping all four redirects on 308 makes the
//! whole rename a single rule for operators and reviewers to
//! remember: "old → new is always 308".
//!
//! ## Why these aren't expressed as `Router::route("/old", any(...))`
//!
//! axum has `Redirect::permanent` returning a static response,
//! but a path-parameterized redirect (e.g. preserving `{id}` in
//! `/notifications/{id}/read` → `/inbox/{id}/read`) needs a
//! tiny handler to extract and reformat the path. Putting the
//! parameterless ones beside the parameterized one keeps the
//! redirect surface in one file.

use axum::{
    extract::Path,
    response::Redirect,
};

/// `/me` (legacy) → `/today` (canonical).
pub async fn me_to_today() -> Redirect {
    Redirect::permanent("/today")
}

/// `/notifications` (legacy) → `/inbox` (canonical).
pub async fn notifications_to_inbox() -> Redirect {
    Redirect::permanent("/inbox")
}

/// `/notifications/mark-all-read` (legacy POST) → `/inbox/mark-all-read`.
pub async fn notifications_mark_all_read_to_inbox() -> Redirect {
    Redirect::permanent("/inbox/mark-all-read")
}

/// `/notifications/{id}/read` (legacy POST) → `/inbox/{id}/read`.
///
/// The `{id}` path parameter is preserved verbatim. We don't
/// validate it here — if a caller hits the legacy URL with an
/// invalid id, the redirected handler at `/inbox/{id}/read`
/// produces the same error response the user would have got
/// from the legacy URL pre-rename.
pub async fn notifications_read_to_inbox(Path(id): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/inbox/{id}/read"))
}
