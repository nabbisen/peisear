//! Personal-data JSON endpoints under `/api/users/{user_id}/...`.
//!
//! Phase B PR2 (peisear-feature-spec-v2.1 §11.5): expose the
//! same data the `/today` and `/inbox` HTML pages render, but
//! shaped as JSON for typeahead-style or app-driven callers.
//!
//! ## Authorization
//!
//! All three endpoints in this module enforce the §11.5 "self
//! access only" boundary: a session belonging to user A may
//! only read user A's data, even if A is a team admin or
//! shares a project with the target user. Admins manage; they
//! don't oversee. Cross-user reads return 403, not 404 — there's
//! no presence to leak (the target's existence is implied by
//! the URL the caller already knows), and 403 is more honest
//! about why the request was refused.
//!
//! Unauthenticated requests return 401 (JSON), not a redirect
//! to `/login` — these endpoints are for JS callers that
//! handle auth state themselves.
//!
//! ## Response shape
//!
//! The shape is iterated during Phase B per the development
//! decision B-E3. The current shapes are documented inline on
//! each handler and on the response structs. Field additions
//! are non-breaking; field renames or removals are. If Phase
//! C/D need a different shape, version the route
//! (`/api/v2/users/...`) rather than mutating this one.

use axum::{
    Json,
    extract::{Path, State},
};
use peisear_storage::{notifications, user_burnout, user_capacities};
use serde::Serialize;

use crate::{ApiAppError, ApiAppResult, AppState, extractors::ApiAuthUser};

/// Enforce the "self access only" boundary. Returns
/// `ApiAppError::Forbidden` if the path's `user_id` doesn't
/// match the authenticated session's user. Does NOT
/// distinguish "user doesn't exist" from "different user" —
/// see the module doc for the rationale.
fn require_self(session_user_id: &str, path_user_id: &str) -> ApiAppResult<()> {
    if session_user_id != path_user_id {
        return Err(ApiAppError::Forbidden);
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────
// /api/users/{user_id}/burnout
// ─────────────────────────────────────────────────────────────────

/// JSON shape for the burnout endpoint.
///
/// Mirrors the data the HTML burnout panel surfaces on
/// `/today`. The `signals` array is human-language reasons,
/// each tagged with a stable `code` string the client can
/// switch on without parsing the message.
///
/// `indicator` follows the `HealthIndicator` ceiling at
/// `Watch` — never `Concern` — per spec §1.4 (signals that
/// reach you, but no alarming).
#[derive(Debug, Serialize)]
pub struct BurnoutResponse {
    pub user_id: String,
    pub indicator: String,
    pub signals: Vec<BurnoutSignal>,
    pub computed_at: String,
}

#[derive(Debug, Serialize)]
pub struct BurnoutSignal {
    /// Stable code so clients can switch on it
    /// (`"overload_streak"`, `"stalled_assigned"`,
    /// `"estimation_drift"`, `"cognitive_switching"`).
    pub code: String,
    /// Human-readable label suitable for direct display.
    pub label: String,
}

pub async fn burnout(
    ApiAuthUser(user): ApiAuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiAppResult<Json<BurnoutResponse>> {
    require_self(&user.id, &user_id)?;

    let signals = user_burnout::for_user(&state.db, &user.id)
        .await?
        .ok_or(ApiAppError::NotFound)?;

    // Map the raw signals into the human-language array. We
    // include only signals that meaningfully fired — empty
    // streak counts and `None` trends produce no row, so the
    // `signals` array doubles as a "what's worth your
    // attention" list.
    let mut signal_list: Vec<BurnoutSignal> = Vec::new();

    if signals.overload_streak_days >= peisear_core::user_burnout::OVERLOAD_STREAK_WATCH {
        signal_list.push(BurnoutSignal {
            code: "overload_streak".into(),
            label: format!(
                "Over capacity for {} of the last {} snapshots.",
                signals.overload_streak_days, signals.window_days
            ),
        });
    }

    if signals.stalled_assigned_max_days >= peisear_core::user_burnout::STALLED_WATCH_DAYS {
        signal_list.push(BurnoutSignal {
            code: "stalled_assigned".into(),
            label: format!(
                "Oldest in-flight assigned issue hasn't moved in {} days.",
                signals.stalled_assigned_max_days
            ),
        });
    }

    if let Some(drift) = &signals.estimation_drift {
        // Only surface when direction is non-Steady. Steady is
        // baseline and not worth a UI row.
        //
        // The variant names `Up` / `Down` describe the
        // dwell-time-per-point trend direction: `Up` = recent
        // issues take longer per point (slowing), `Down` =
        // faster (speeding). The neutral phrasing here mirrors
        // the source's design intent (per the enum's doc
        // comment): no good/bad connotation, just a directional
        // fact for the user to interpret.
        use peisear_core::user_burnout::DriftDirection;
        let label = match drift.direction {
            DriftDirection::Up => {
                Some("Recent issues are taking longer per point than older ones.")
            }
            DriftDirection::Down => {
                Some("Recent issues are completing faster per point than older ones.")
            }
            DriftDirection::Steady => None,
        };
        if let Some(label) = label {
            signal_list.push(BurnoutSignal {
                code: "estimation_drift".into(),
                label: label.to_string(),
            });
        }
    }

    if let Some(switching) = &signals.cognitive_switching {
        signal_list.push(BurnoutSignal {
            code: "cognitive_switching".into(),
            label: format!(
                "Switching between {:.1} issues per active day on average.",
                switching.switches_per_day_median
            ),
        });
    }

    let indicator = peisear_core::DisplayHealthState::from(
        peisear_core::user_burnout::classify_overload_streak(&signals),
    );

    Ok(Json(BurnoutResponse {
        user_id: user.id,
        indicator: indicator_str(indicator).to_string(),
        signals: signal_list,
        computed_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// `indicator` observes the severity ceiling (`NFR-LANG-002`,
/// external design §8.3): `DisplayHealthState` has no `Concern`
/// variant to serialise, so `"concern"` cannot reach this response
/// regardless of whether the underlying classifier could ever
/// produce it.
fn indicator_str(i: peisear_core::DisplayHealthState) -> &'static str {
    use peisear_core::DisplayHealthState::*;
    match i {
        Insufficient => "insufficient",
        Good => "good",
        Watch => "watch",
    }
}

// ─────────────────────────────────────────────────────────────────
// /api/users/{user_id}/capacity
// ─────────────────────────────────────────────────────────────────

/// JSON shape for the capacity endpoint. Mirrors the table
/// the `/settings` page renders.
#[derive(Debug, Serialize)]
pub struct CapacityResponse {
    pub user_id: String,
    /// The points figure that applies to today, computed by
    /// resolving the user's period rows against today's date.
    /// `None` when no row covers today.
    pub effective_today: Option<i64>,
    pub rows: Vec<CapacityPeriod>,
}

#[derive(Debug, Serialize)]
pub struct CapacityPeriod {
    pub id: String,
    pub points: i64,
    /// ISO 8601 date (YYYY-MM-DD); `None` means open-ended.
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub note: Option<String>,
    /// Last mutation timestamp (RFC3339). Useful for clients
    /// that want to render an "updated X ago" timestamp.
    pub updated_at: String,
}

pub async fn capacity(
    ApiAuthUser(user): ApiAuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiAppResult<Json<CapacityResponse>> {
    require_self(&user.id, &user_id)?;

    let effective_today = user_capacities::effective_for_user(&state.db, &user.id).await?;
    let rows = user_capacities::list_for_user(&state.db, &user.id).await?;

    let rows = rows
        .into_iter()
        .map(|r| CapacityPeriod {
            id: r.id,
            points: r.points,
            period_start: r.period_start.map(|d| d.to_string()),
            period_end: r.period_end.map(|d| d.to_string()),
            note: r.note,
            updated_at: r.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(CapacityResponse {
        user_id: user.id,
        effective_today,
        rows,
    }))
}

// ─────────────────────────────────────────────────────────────────
// /api/users/{user_id}/notifications
// ─────────────────────────────────────────────────────────────────

/// JSON shape for the notifications endpoint. Returns the
/// inbox the `/inbox` HTML page renders.
#[derive(Debug, Serialize)]
pub struct NotificationsResponse {
    pub user_id: String,
    pub unread_count: i64,
    pub items: Vec<NotificationItem>,
}

#[derive(Debug, Serialize)]
pub struct NotificationItem {
    pub id: String,
    pub kind: String,
    pub severity: String,
    pub title: String,
    pub body: String,
    pub created_at: String,
    /// `None` when unread.
    pub read_at: Option<String>,
}

/// How many recent items to return. Matches the inbox HTML
/// page's window. Older items are accessible via the legacy
/// `/inbox` page until a paginated `/api/users/{id}/notifications?cursor=...`
/// surface lands (out of scope for PR2).
const NOTIFICATIONS_WINDOW: i64 = 50;

pub async fn list_notifications(
    ApiAuthUser(user): ApiAuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiAppResult<Json<NotificationsResponse>> {
    require_self(&user.id, &user_id)?;

    let unread_count = notifications::unread_count_for_user(&state.db, &user.id).await?;
    let recent = notifications::recent_for_user(&state.db, &user.id, NOTIFICATIONS_WINDOW).await?;

    let items = recent
        .into_iter()
        .map(|n| NotificationItem {
            id: n.id,
            kind: n.kind,
            severity: n.severity.as_str().to_string(),
            title: n.title,
            body: n.body,
            created_at: n.created_at.to_rfc3339(),
            read_at: n.read_at.map(|t| t.to_rfc3339()),
        })
        .collect();

    Ok(Json(NotificationsResponse {
        user_id: user.id,
        unread_count,
        items,
    }))
}
