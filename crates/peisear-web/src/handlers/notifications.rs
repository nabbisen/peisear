//! Notifications inbox handlers.
//!
//! `/notifications`         — inbox list
//! `/notifications/{id}/read`   — mark one read
//! `/notifications/mark-all-read` — clear inbox

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use peisear_storage::notifications as notif_store;
use serde::Deserialize;

use crate::{AppResult, AppState, components, extractors::AuthUser};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
}

/// Inbox page. Renders the user's notifications, newest first.
/// Limit is generous (200) — for the inbox display, beyond that
/// the user isn't really reviewing anything, just scrolling.
pub async fn page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let items = notif_store::recent_for_user(&state.db, &user.id, 200).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    Ok(components::notifications::render_inbox(
        user,
        items,
        unread_count,
        q.flash,
    ))
}

/// Mark one notification as read. Re-clicking a row that's
/// already read is idempotent.
pub async fn mark_read(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Redirect> {
    notif_store::mark_read(&state.db, &user.id, &id).await?;
    Ok(Redirect::to("/notifications"))
}

/// Clear the inbox: mark every unread row as read at once.
pub async fn mark_all_read(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Redirect> {
    let n = notif_store::mark_all_read(&state.db, &user.id).await?;
    let target = format!("/notifications?flash=Marked+{n}+as+read");
    Ok(Redirect::to(&target))
}
