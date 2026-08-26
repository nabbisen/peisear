//! Notifications inbox handlers.
//!
//! `/inbox`                  — inbox list
//! `/inbox/{id}/read`        — mark one read
//! `/inbox/mark-all-read`    — clear inbox
//! `/inbox/resume`           — resume from `silence_all` (`INBOX-001`)
//! `/inbox/email-opt-in`     — the first-notification email prompt,
//!                             moved here from `/settings/notifications`
//!                             (`INBOX-001`, RFC 003 D2)
//!
//! Renamed from `/notifications` in v0.17.0; legacy URLs are
//! kept as 308 Permanent Redirects (see [`crate::handlers::redirects`]).

use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use peisear_i18n::{Locale, MessageKey};
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
///
/// Two banners can render above the list, independently of each
/// other (`INBOX-001`):
///
/// - The silence-resume banner, when every user-facing kind is
///   silenced (RFC 003 D1).
/// - The email opt-in prompt, when the user has never been
///   prompted and has received at least one notification — read
///   or unread both count (RFC 003 open question 1's default)
///   (RFC 003 D2).
pub async fn page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let items = notif_store::recent_for_user(&state.db, &user.id, 200).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    let is_silenced = notif_store::all_kinds_silenced(&state.db, &user.id).await?;
    let show_email_prompt =
        !notif_store::global_acknowledged(&state.db, &user.id).await? && !items.is_empty();
    Ok(components::notifications::render_inbox(
        user,
        items,
        unread_count,
        q.flash,
        is_silenced,
        show_email_prompt,
    ))
}

/// Resume: delete this user's per-kind preference rows for every
/// user-facing kind, restoring the default (`INBOX-001`, RFC 003
/// D1). The exact inverse of `silence_all`
/// (`notification_preferences::silence_all`) — see
/// [`notif_store::delete_user_facing_preferences`]'s doc comment
/// for why this deletes rather than writes defaults back.
pub async fn resume(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Redirect> {
    notif_store::delete_user_facing_preferences(&state.db, &user.id).await?;
    Ok(Redirect::to("/inbox"))
}

#[derive(Debug, Deserialize)]
pub struct EmailOptInForm {
    /// "yes" to opt in, "no" to acknowledge without opting in.
    /// Either answer records acknowledgement so the prompt does
    /// not reappear.
    pub email_opt_in: String,
}

/// Record the first-notification email opt-in answer
/// (`INBOX-001`, RFC 003 D2). Moved here from
/// `/settings/notifications/ack-global` — the prompt now lives
/// at `/inbox` only, gated on the user having received at least
/// one notification, which the settings page had no way to
/// enforce.
pub async fn email_opt_in(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<EmailOptInForm>,
) -> AppResult<Redirect> {
    let opt_in = form.email_opt_in.eq_ignore_ascii_case("yes");
    notif_store::set_global_acknowledged(&state.db, &user.id, opt_in).await?;
    Ok(Redirect::to("/inbox"))
}

/// Mark one notification as read. Re-clicking a row that's
/// already read is idempotent.
pub async fn mark_read(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Redirect> {
    notif_store::mark_read(&state.db, &user.id, &id).await?;
    Ok(Redirect::to("/inbox"))
}

/// Clear the inbox: mark every unread row as read at once.
pub async fn mark_all_read(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Redirect> {
    let n = notif_store::mark_all_read(&state.db, &user.id).await?;
    let flash = super::percent_encode_query(
        &Locale::English.render(MessageKey::MarkedAsReadFlash { count: n }),
    );
    Ok(Redirect::to(&format!("/inbox?flash={flash}")))
}
