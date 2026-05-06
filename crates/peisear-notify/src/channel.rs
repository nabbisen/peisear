//! Channel dispatch. Per-channel I/O, called by
//! `dispatch::process_event` once filters and cooldown have
//! cleared.
//!
//! ## Channels
//!
//! - **`in_app`**: no-op. The audit row inserted by
//!   `dispatch::process_event` *is* the in-app artefact; the
//!   inbox query reads from `notifications`.
//! - **`email`**: SMTP via [`crate::email::send_email`]. Returns
//!   [`ChannelSendError::Skipped`] when the SMTP config is
//!   absent (graceful — see Q4 in 0.16.0 design notes), or
//!   [`ChannelSendError::Send`] with the upstream error
//!   wrapped if the SMTP exchange itself fails.
//! - **`webhook`**: still a log stub. Real implementation is a
//!   separate piece of work (Phase 2).

use peisear_core::notifications::channel as channel_id;

use crate::config::SmtpConfig;
use crate::dispatch::DispatchEvent;
use crate::email::EmailError;

/// One-channel send outcome. The dispatch loop distinguishes
/// `Skipped` (channel was unavailable but that's fine) from
/// `Send` (channel was supposed to work and didn't) so it can
/// log at the right level.
#[derive(Debug, thiserror::Error)]
pub enum ChannelSendError {
    /// Channel is intentionally not available right now (e.g.
    /// SMTP not configured, no destination address known).
    /// The dispatch loop logs this at debug level.
    #[error("channel skipped: {0}")]
    Skipped(&'static str),

    /// Channel attempted to deliver and failed. The dispatch
    /// loop logs this at warn level.
    #[error("send error: {0}")]
    Send(String),

    /// Email-specific failure (network, auth, protocol).
    #[error(transparent)]
    Email(#[from] EmailError),

    /// Channel name not recognised. Indicates a bug.
    #[error("unknown channel: {0}")]
    UnknownChannel(String),
}

/// Dispatch one event via one channel. The function knows
/// nothing about preferences, cooldown, or audit rows; those
/// are the dispatch loop's job.
pub async fn send_via_channel(
    channel: &str,
    event: &DispatchEvent,
    smtp: Option<&SmtpConfig>,
    user_email: Option<&str>,
) -> Result<(), ChannelSendError> {
    match channel {
        channel_id::IN_APP => {
            // The audit row carries this. Nothing to send.
            Ok(())
        }
        channel_id::EMAIL => {
            // Graceful degradation per Q4: if SMTP isn't
            // configured, skip rather than fail. The send-time
            // log line tells an operator what's happening.
            let Some(cfg) = smtp else {
                tracing::debug!(
                    user_id = %event.user_id,
                    kind = %event.kind,
                    "email channel: SMTP not configured, skipping",
                );
                return Err(ChannelSendError::Skipped("SMTP not configured"));
            };
            let Some(to) = user_email else {
                tracing::debug!(
                    user_id = %event.user_id,
                    kind = %event.kind,
                    "email channel: no recipient address, skipping",
                );
                return Err(ChannelSendError::Skipped("recipient email unknown"));
            };

            crate::email::send_email(cfg, to, &event.title, &event.body).await?;
            tracing::info!(
                user_id = %event.user_id,
                kind = %event.kind,
                "email channel: delivered",
            );
            Ok(())
        }
        channel_id::WEBHOOK => {
            // Stub. Real impl reads a per-user webhook URL
            // from preferences (column to be added) and POSTs.
            // Today we log and succeed; webhook configuration
            // UI will land alongside this in a follow-up.
            tracing::info!(
                user_id = %event.user_id,
                kind = %event.kind,
                title = %event.title,
                "[webhook-stub] would POST notification",
            );
            Ok(())
        }
        other => Err(ChannelSendError::UnknownChannel(other.to_string())),
    }
}
