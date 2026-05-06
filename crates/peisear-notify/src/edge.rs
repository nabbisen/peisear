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
        title: "Sustained over-capacity streak".to_string(),
        body: format!(
            "Your in-flight load has been over capacity for {current_streak_days} \
             consecutive snapshots. This is a description of the recent rhythm, \
             not an evaluation of your work — many streaks have legitimate causes. \
             You can review at /me."
        ),
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
        title: "Long-stalled assigned work".to_string(),
        body: format!(
            "An assigned issue has been in flight for {current_max_days} days. \
             May be worth a glance — sometimes a quick check-in turns out to be \
             all that's needed. Visit /me for context."
        ),
        payload_json: None,
    })
}
