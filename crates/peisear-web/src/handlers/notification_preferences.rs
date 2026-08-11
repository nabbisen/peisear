//! `/settings/notifications` — per-kind delivery preferences.
//!
//! Smart-defaults posture (Q3=A in design discussion):
//!
//! - First-time visitors who haven't been "globally
//!   acknowledged" see a small banner suggesting they choose
//!   email opt-in. They can dismiss it with one click; we
//!   record the answer in the global pref row.
//! - Per-kind preferences live in a folded `<details>` — most
//!   users never need to open it.
//! - The "Silence all" button is in the per-kind section,
//!   discoverable but not prominent. No friction for users who
//!   want it.

use axum::{
    Form,
    extract::State,
    response::{IntoResponse, Redirect},
};
use peisear_core::notifications::{
    Severity,
    channel::{ALL_CHANNELS, EMAIL, IN_APP, WEBHOOK},
    kind,
};
use peisear_i18n::{Locale, MessageKey};
use peisear_storage::notifications as notif_store;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{AppResult, AppState, components, extractors::AuthUser};

pub async fn page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let prefs = notif_store::preferences_for_user(&state.db, &user.id).await?;
    let global_acknowledged = notif_store::global_acknowledged(&state.db, &user.id).await?;
    let global = notif_store::global_preference(&state.db, &user.id).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    let email_globally_on = global
        .as_ref()
        .map(|g| g.channels.iter().any(|c| c == EMAIL))
        .unwrap_or(false);

    Ok(components::notification_preferences::render_preferences(
        user,
        prefs,
        global_acknowledged,
        email_globally_on,
        unread_count,
    ))
}

#[derive(Debug, Deserialize)]
pub struct GlobalAckForm {
    /// "yes" to opt in, "no" to acknowledge without opting in.
    /// We always record acknowledgement either way so the
    /// banner stops showing.
    pub email_opt_in: String,
}

pub async fn ack_global(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<GlobalAckForm>,
) -> AppResult<Redirect> {
    let opt_in = form.email_opt_in.eq_ignore_ascii_case("yes");
    notif_store::set_global_acknowledged(&state.db, &user.id, opt_in).await?;
    Ok(Redirect::to("/settings/notifications"))
}

#[derive(Debug, Deserialize)]
pub struct PreferenceForm {
    /// Tabular form input — one set of keys per kind:
    ///
    /// `kinds[burnout_overload][channels][in_app] = on`
    /// `kinds[burnout_overload][min_severity] = info`
    ///
    /// axum's `serde_urlencoded` doesn't natively flatten this
    /// shape, so the form actually uses flatter keys:
    ///
    /// `channel__{kind}__{channel} = on`
    /// `min_severity__{kind} = info|watch`
    ///
    /// And we collect them into a HashMap manually below. Less
    /// idiomatic than a strongly-typed struct, but more flexible
    /// — adding a new kind doesn't require a code change here.
    #[serde(flatten)]
    pub raw: HashMap<String, String>,
}

pub async fn save_preferences(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<PreferenceForm>,
) -> AppResult<Redirect> {
    // Reconstruct (kind, channels, min_severity) from the flat
    // form key/value pairs.
    let mut by_kind: HashMap<&str, (Vec<&str>, Severity)> = HashMap::new();

    for k in kind::all_user_facing() {
        // Default: pull the form's submitted values for this
        // kind. If a checkbox isn't sent (HTML form
        // convention: unchecked checkboxes simply don't appear
        // in the body), it's not in the channel list.
        let mut chans: Vec<&str> = Vec::new();
        for chan in ALL_CHANNELS {
            let key = format!("channel__{k}__{chan}");
            if form.raw.contains_key(&key) {
                chans.push(chan);
            }
        }
        let sev_key = format!("min_severity__{k}");
        let min_sev = match form.raw.get(&sev_key).map(String::as_str) {
            Some("watch") => Severity::Watch,
            _ => Severity::Info,
        };
        by_kind.insert(k, (chans, min_sev));
    }

    for (k, (chans, sev)) in by_kind {
        notif_store::upsert_preference(&state.db, &user.id, k, &chans, sev).await?;
    }

    let flash = Locale::English
        .render(MessageKey::PreferencesSavedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!(
        "/settings/notifications?flash={flash}"
    )))
}

/// "Silence all" convenience: set every user-facing kind's
/// channels to empty.
pub async fn silence_all(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Redirect> {
    for k in kind::all_user_facing() {
        notif_store::upsert_preference(&state.db, &user.id, k, &[], Severity::Info).await?;
    }
    // Don't touch the global pref row — that's only the
    // first-login email opt-in record, conceptually different
    // from per-kind silencing.
    let _ = (IN_APP, WEBHOOK); // silence "unused import" warning shape
    let flash = Locale::English
        .render(MessageKey::AllNotificationsSilencedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!(
        "/settings/notifications?flash={flash}"
    )))
}
