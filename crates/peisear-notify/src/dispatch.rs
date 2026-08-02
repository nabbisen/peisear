//! Dispatch loop. Drains [`DispatchEvent`]s, applies filters,
//! persists the audit row, fans out to channels.
//!
//! Moved from `peisear-web::notifications::mod` in 0.16.0. Refer
//! to the crate-level docs in `lib.rs` for the architecture
//! diagram (detection → mpsc → dispatch_loop → channel send).

use chrono::Utc;
use peisear_core::notifications::{Severity, channel as channel_id};
use peisear_storage::{Pool, notifications as notif_store};
use tokio::sync::mpsc;

use crate::channel::{ChannelSendError, send_via_channel};

/// One thing to potentially notify about. Built by callers
/// (typically a snapshot/jobs loop in `peisear-web`); consumed
/// by [`dispatch_loop`].
#[derive(Debug, Clone)]
pub struct DispatchEvent {
    pub user_id: String,
    pub kind: String,
    pub severity: Severity,
    pub title: String,
    pub body: String,
    /// Free-form JSON payload. Today we keep this `None`;
    /// later kinds (project trend, digests) attach structured
    /// detail here.
    pub payload_json: Option<String>,
}

/// Sender side of the dispatch channel. Held by callers so
/// they can enqueue events.
pub type DispatchTx = mpsc::Sender<DispatchEvent>;

/// Receiver side. Owned by [`dispatch_loop`].
pub type DispatchRx = mpsc::Receiver<DispatchEvent>;

/// Channel buffer size. Sized for the worst-case "every active
/// user produces one event in a single tick"; a 256-buffer
/// covers small-to-medium teams without ever back-pressuring
/// the snapshot loop. If a deployment grows beyond this, the
/// fix is per-user back-pressure or batching, not a bigger
/// number.
pub const DISPATCH_CHANNEL_BUFFER: usize = 256;

/// Result of attempting to dispatch one event. Used both for
/// logging and to record which channels were successful in the
/// `notifications.dispatched_via` audit column.
struct DispatchOutcome {
    notification_id: Option<String>,
    delivered_via: Vec<String>,
    skipped: Option<&'static str>,
}

/// Configuration passed into the dispatch loop. Carries
/// references to backing services that the channel layer needs
/// (today: the SMTP config; tomorrow: webhook HTTP client, etc.).
#[derive(Clone)]
pub struct DispatchContext {
    pub db: Pool,
    pub smtp: Option<crate::config::SmtpConfig>,
}

/// The dispatch loop. Receives events, applies filters,
/// persists rows, fans out to channels. Runs until its
/// `Receiver` is closed.
pub async fn dispatch_loop(ctx: DispatchContext, mut rx: DispatchRx) {
    tracing::info!("notification dispatch_loop started");

    while let Some(event) = rx.recv().await {
        let outcome = match process_event(&ctx, &event).await {
            Ok(o) => o,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    user_id = %event.user_id,
                    kind = %event.kind,
                    "dispatch_loop: process_event failed",
                );
                continue;
            }
        };

        if let Some(reason) = outcome.skipped {
            tracing::debug!(
                user_id = %event.user_id,
                kind = %event.kind,
                %reason,
                "dispatch_loop: skipped",
            );
        } else {
            tracing::debug!(
                notification_id = ?outcome.notification_id,
                user_id = %event.user_id,
                kind = %event.kind,
                via = ?outcome.delivered_via,
                "dispatch_loop: delivered",
            );
        }
    }

    tracing::info!("notification dispatch_loop shutting down (channel closed)");
}

/// Process a single event:
///   1. Look up the user's preference for this kind (or default).
///   2. Filter on `min_severity` and `channels`.
///   3. Apply cooldown.
///   4. Fan out to channels.
///   5. Persist a notification row with the actual delivered_via.
async fn process_event(
    ctx: &DispatchContext,
    event: &DispatchEvent,
) -> Result<DispatchOutcome, peisear_storage::StorageError> {
    let pref = notif_store::preference_for_user_kind(&ctx.db, &event.user_id, &event.kind).await?;
    let global = notif_store::global_preference(&ctx.db, &event.user_id).await?;

    let (channels, min_severity) = effective_channels_and_min_severity(pref, global);

    if !event.severity.meets_minimum(min_severity) {
        return Ok(DispatchOutcome {
            notification_id: None,
            delivered_via: Vec::new(),
            skipped: Some("severity below preference minimum"),
        });
    }

    if channels.is_empty() {
        return Ok(DispatchOutcome {
            notification_id: None,
            delivered_via: Vec::new(),
            skipped: Some("user has no channels for this kind"),
        });
    }

    let last =
        notif_store::last_dispatched_at_for_user_kind(&ctx.db, &event.user_id, &event.kind).await?;
    if let Some(last_at) = last {
        let elapsed_hours = (Utc::now() - last_at).num_hours();
        if elapsed_hours < peisear_core::notifications::COOLDOWN_HOURS {
            return Ok(DispatchOutcome {
                notification_id: None,
                delivered_via: Vec::new(),
                skipped: Some("inside cooldown window"),
            });
        }
    }

    // We need the user's email for email channel sends.
    // Fetch once if email is in the channel list and we have
    // SMTP config — otherwise we can skip the lookup.
    let user_email = if channels.contains(&channel_id::EMAIL) && ctx.smtp.is_some() {
        peisear_storage::users::find_by_id(&ctx.db, &event.user_id)
            .await?
            .map(|u| u.email)
    } else {
        None
    };

    let mut delivered: Vec<String> = Vec::with_capacity(channels.len());
    for chan in &channels {
        match send_via_channel(chan, event, ctx.smtp.as_ref(), user_email.as_deref()).await {
            Ok(()) => delivered.push((*chan).to_string()),
            Err(ChannelSendError::Skipped(reason)) => {
                // Channel was unavailable (e.g. SMTP not configured).
                // Log at debug since this is expected in dev/eval setups.
                tracing::debug!(
                    channel = chan,
                    user_id = %event.user_id,
                    kind = %event.kind,
                    reason = reason,
                    "dispatch_loop: channel skipped",
                );
            }
            Err(err) => {
                tracing::warn!(
                    channel = chan,
                    user_id = %event.user_id,
                    kind = %event.kind,
                    error = %err,
                    "dispatch_loop: channel send failed",
                );
            }
        }
    }

    let delivered_refs: Vec<&str> = delivered.iter().map(|s| s.as_str()).collect();
    let id = notif_store::insert(
        &ctx.db,
        &event.user_id,
        notif_store::NewNotification {
            kind: &event.kind,
            severity: event.severity,
            title: &event.title,
            body: &event.body,
            payload_json: event.payload_json.as_deref(),
            dispatched_via: &delivered_refs,
        },
    )
    .await?;

    Ok(DispatchOutcome {
        notification_id: Some(id),
        delivered_via: delivered,
        skipped: None,
    })
}

/// Resolve the effective channel list and minimum severity for
/// dispatch. Applies the per-kind preference if present;
/// otherwise falls back to the global preference (for the
/// channel list) and the system default minimum severity.
fn effective_channels_and_min_severity(
    pref: Option<peisear_core::notifications::Preference>,
    global: Option<peisear_core::notifications::Preference>,
) -> (Vec<&'static str>, Severity) {
    if let Some(p) = pref {
        let chans: Vec<&'static str> = p
            .channels
            .iter()
            .filter_map(|c| match c.as_str() {
                channel_id::IN_APP => Some(channel_id::IN_APP),
                channel_id::EMAIL => Some(channel_id::EMAIL),
                channel_id::WEBHOOK => Some(channel_id::WEBHOOK),
                _ => None,
            })
            .collect();
        return (chans, p.min_severity);
    }

    let global_channels: Vec<&'static str> = match global {
        Some(g) => g
            .channels
            .iter()
            .filter_map(|c| match c.as_str() {
                channel_id::IN_APP => Some(channel_id::IN_APP),
                channel_id::EMAIL => Some(channel_id::EMAIL),
                channel_id::WEBHOOK => Some(channel_id::WEBHOOK),
                _ => None,
            })
            .collect(),
        None => peisear_core::notifications::DEFAULT_CHANNELS.to_vec(),
    };

    (
        global_channels,
        peisear_core::notifications::DEFAULT_MIN_SEVERITY,
    )
}
