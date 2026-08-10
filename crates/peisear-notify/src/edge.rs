//! Edge-trigger detection helpers. Called by callers (typically
//! a snapshot loop) after each tick. Each helper takes prior
//! and current state and returns `Some(DispatchEvent)` if a
//! transition occurred, `None` otherwise.
//!
//! Centralised here so the snapshot loop's tick body stays
//! short and the trigger logic has obvious unit-test surface.
//!
//! Moved from `peisear-web::notifications::mod` in 0.16.0.

use peisear_core::notifications::{Severity, kind as kind_id};
use peisear_i18n::{Locale, MessageKey};

use crate::dispatch::DispatchEvent;

pub fn detect_burnout_overload_edge(
    user_id: &str,
    prior_streak_days: i64,
    current_streak_days: i64,
) -> Option<DispatchEvent> {
    if !peisear_core::notifications::is_edge_into_watch_burnout_overload(
        prior_streak_days,
        current_streak_days,
    ) {
        return None;
    }
    Some(DispatchEvent {
        user_id: user_id.to_string(),
        kind: kind_id::BURNOUT_OVERLOAD.to_string(),
        severity: Severity::Watch,
        title: Locale::English.render(MessageKey::NotificationBurnoutOverloadTitle),
        body: Locale::English.render(MessageKey::NotificationBurnoutOverloadBody {
            streak_snapshots: current_streak_days,
        }),
        payload_json: None,
    })
}

pub fn detect_burnout_stalled_edge(
    user_id: &str,
    prior_max_days: i64,
    current_max_days: i64,
) -> Option<DispatchEvent> {
    if !peisear_core::notifications::is_edge_into_watch_burnout_stalled(
        prior_max_days,
        current_max_days,
    ) {
        return None;
    }
    Some(DispatchEvent {
        user_id: user_id.to_string(),
        kind: kind_id::BURNOUT_STALLED.to_string(),
        severity: Severity::Watch,
        title: Locale::English.render(MessageKey::NotificationBurnoutStalledTitle),
        body: Locale::English.render(MessageKey::NotificationBurnoutStalledBody {
            stalled_days: current_max_days,
        }),
        payload_json: None,
    })
}
