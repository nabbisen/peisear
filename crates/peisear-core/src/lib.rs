//! Pure domain types shared across the workspace.
//!
//! This crate intentionally depends only on `serde`, `chrono`, and
//! `thiserror`; it has no knowledge of axum, sqlx, jsonwebtoken, or any
//! other runtime concern. Consumers include:
//!
//! - `peisear-storage`: constructs values of these types from DB rows
//! - `peisear-web`: receives them from storage and feeds them to
//!   handlers / Leptos components
//! - future `peisear-cli` or admin tools that will speak the same
//!   domain vocabulary without pulling in the web stack

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    /// Optional global capacity in story points. `None` means "not
    /// set, do not show a workload warning for this user" — opt-in
    /// rather than enforced. Mirrors the same pattern as
    /// [`Issue::effort`]: estimation is gradual.
    ///
    /// When period-scoped capacities land in a future release, this
    /// field migrates to a `user_capacities` table; see migration
    /// `0004_user_capacity.sql` for the planned shape.
    pub capacity_points: Option<i64>,
    /// Optional personal WIP limit (count of in-progress issues).
    /// `None` means use the project-level default, or fall back to
    /// [`personal_metrics::DEFAULT_WIP_LIMIT`]. Distinct from
    /// `capacity_points` — see migration
    /// `0005_personal_limits.sql` for the rationale.
    pub wip_limit: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    /// Optional project-level default WIP limit (count of
    /// in-progress issues). Overrides
    /// [`personal_metrics::DEFAULT_WIP_LIMIT`] for users who have
    /// not set their own [`User::wip_limit`].
    pub wip_limit_default: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    Open,
    InProgress,
    Done,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in_progress",
            Self::Done => "done",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::InProgress => "In Progress",
            Self::Done => "Done",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "open" => Some(Self::Open),
            "in_progress" => Some(Self::InProgress),
            "done" => Some(Self::Done),
            _ => None,
        }
    }

    pub fn all() -> [IssueStatus; 3] {
        [Self::Open, Self::InProgress, Self::Done]
    }
}

impl fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Urgent => "urgent",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "urgent" => Some(Self::Urgent),
            _ => None,
        }
    }

    pub fn all() -> [Priority; 4] {
        [Self::Low, Self::Medium, Self::High, Self::Urgent]
    }

    /// daisyUI badge class mapping — kept in core so any future
    /// read-only surface (email summary, future client, etc.) can
    /// reuse the canonical severity palette.
    pub fn badge_class(&self) -> &'static str {
        match self {
            Self::Low => "badge-ghost",
            Self::Medium => "badge-info",
            Self::High => "badge-warning",
            Self::Urgent => "badge-error",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub title: String,
    pub description: String,
    pub status: IssueStatus,
    pub priority: Priority,
    pub position: i64,
    /// Effort estimate in story points. `None` means the issue has not
    /// been estimated yet — this is the default for newly created
    /// issues and for issues that existed before estimation was
    /// introduced. The DB CHECK constraint guarantees `Some(n)` always
    /// holds `n > 0`.
    pub effort: Option<i64>,
    /// User the issue is assigned to. `None` is "unassigned" — a
    /// normal state for backlog items. The DB foreign key uses
    /// `ON DELETE SET NULL` so removing a user does not cascade-delete
    /// their issues; ownership simply returns to the pool.
    pub assignee_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Effort estimate presets shown in the UI.
///
/// The values follow the well-trodden Fibonacci-ish scale used in
/// agile planning (1, 2, 3, 5, 8, 13). Any positive integer is valid
/// in storage; these are just the suggested presets the form offers.
pub const EFFORT_PRESETS: &[i64] = &[1, 2, 3, 5, 8, 13];

/// One entry in the assignee `<select>` on the issue form.
///
/// Lives in `core` rather than `web` because the candidate set is a
/// domain concept ("users who can be assigned to issues in this
/// project"), not a presentation concept. When team / organisation
/// support lands, the candidate set will broaden, but the shape of
/// each option stays the same.
#[derive(Debug, Clone)]
pub struct AssigneeOption {
    pub id: String,
    pub display_name: String,
}

/// One row of the per-project workload report: how much in-flight
/// effort a user is currently carrying versus their stated capacity.
///
/// "In-flight" today is the sum of `effort` over their assigned
/// issues whose status is `open` or `in_progress`. When a future
/// release introduces sprint / week / month periods, this struct's
/// shape stays the same but the storage query that produces it will
/// take an additional period filter — callers receive the same
/// `UserLoad` either way.
#[derive(Debug, Clone)]
pub struct UserLoad {
    pub user_id: String,
    pub display_name: String,
    /// Sum of effort over the user's assigned in-flight issues.
    /// Issues with no effort estimate contribute 0.
    pub in_flight_points: i64,
    /// Stated capacity. `None` means the user has not set one yet —
    /// in that case [`workload_state`] returns
    /// [`WorkloadState::Unmonitored`] regardless of `in_flight_points`.
    pub capacity_points: Option<i64>,
    /// Number of in-flight issues this user is assigned to.
    /// Useful when several issues lack effort estimates and the
    /// points alone understate the load.
    pub in_flight_issues: i64,
}

/// Coarse-grained workload classification, used for badge colouring
/// and warning surfaces. Computed from a [`UserLoad`] via
/// [`workload_state`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadState {
    /// User has no capacity set; do not warn or colour.
    Unmonitored,
    /// User is at most 80% of their capacity. Healthy.
    Healthy,
    /// User is between 80% and 100% of their capacity. Watch zone.
    Strained,
    /// User is over 100% of their capacity. Warn.
    Overloaded,
}

impl WorkloadState {
    /// daisyUI badge class for rendering. Kept in core so any future
    /// read-only surface (email, CLI, alternate UI) can reuse the
    /// canonical palette.
    pub fn badge_class(&self) -> &'static str {
        match self {
            Self::Unmonitored => "badge-ghost",
            Self::Healthy => "badge-success",
            Self::Strained => "badge-warning",
            Self::Overloaded => "badge-error",
        }
    }
}

/// Classify a workload snapshot. Pure function; no DB access.
///
/// Thresholds (80% / 100%) are deliberate and deliberately simple —
/// they can become configurable in a future release without changing
/// this function's signature.
pub fn workload_state(load: &UserLoad) -> WorkloadState {
    let Some(cap) = load.capacity_points else {
        return WorkloadState::Unmonitored;
    };
    if cap == 0 {
        return WorkloadState::Unmonitored;
    }
    if load.in_flight_points > cap {
        WorkloadState::Overloaded
    } else if load.in_flight_points * 5 >= cap * 4 {
        // ratio >= 0.8 without floats
        WorkloadState::Strained
    } else {
        WorkloadState::Healthy
    }
}

/// Project the post-save workload for a "what if" preview on the
/// issue form. Adds `delta` story points to the existing in-flight,
/// then re-classifies.
///
/// Used for the "currently 8 pt → would become 11 pt" hint shown
/// next to the assignee selector. `delta` is the new effort minus
/// the old effort (or just the new effort when creating).
pub fn projected_workload_state(load: &UserLoad, delta: i64) -> WorkloadState {
    let projected = UserLoad {
        in_flight_points: (load.in_flight_points + delta).max(0),
        ..load.clone()
    };
    workload_state(&projected)
}

/// Compact view of the authenticated user attached to requests.
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: String,
    pub email: String,
    pub display_name: String,
}

impl From<User> for CurrentUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            display_name: u.display_name,
        }
    }
}

/// Coarse-grained classification used across the health / burnout
/// indicator family. The same three-step palette applies whether the
/// subject is a project, a user, or (in the future) a team —
/// callers reuse this enum and its [`HealthIndicator::badge_class`]
/// method instead of inventing parallel ones.
///
/// The semantics are deliberately fuzzy: "Watch" means "worth a
/// glance, may or may not be a problem"; "Concern" means "human
/// attention warranted". Concrete thresholds live with each specific
/// indicator (e.g. [`project_health::classify_staleness`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthIndicator {
    /// Not enough data to classify. Renders as a muted "—".
    Insufficient,
    /// Healthy / on-track.
    Good,
    /// Worth a glance. Borderline.
    Watch,
    /// Human attention warranted.
    Concern,
}

impl HealthIndicator {
    /// daisyUI badge class for rendering. The palette matches
    /// [`WorkloadState::badge_class`] so the two indicator families
    /// look visually coherent on the same page.
    pub fn badge_class(&self) -> &'static str {
        match self {
            Self::Insufficient => "badge-ghost",
            Self::Good => "badge-success",
            Self::Watch => "badge-warning",
            Self::Concern => "badge-error",
        }
    }
}

/// Project-level health indicators.
///
/// Lives in its own module to leave room for `user_burnout` (a
/// planned future addition that will surface per-user fatigue
/// signals such as sustained overload, stalled assigned work, or
/// unbalanced load distribution). The two will share
/// [`HealthIndicator`] for their classification output.
pub mod project_health {
    use super::HealthIndicator;

    /// The window in days over which "recent activity" is counted.
    /// Two weeks is long enough to cover sprint cadences and short
    /// enough that genuine inactivity surfaces quickly.
    ///
    /// Configurability is a future refinement — for now this is a
    /// project-wide constant. When user-level burnout indicators
    /// land they will likely use a different window (a longer
    /// rolling window for streak detection), at which point making
    /// this configurable becomes useful.
    pub const ACTIVITY_WINDOW_DAYS: i64 = 14;

    /// Threshold (in days) for "long-stale" issues — in-flight
    /// issues that have not seen movement in this many days. Reuses
    /// the activity window for symmetry: an issue that nothing has
    /// happened to during the project's activity window is the same
    /// kind of "stuck" that the activity indicator measures the
    /// inverse of.
    pub const LONG_STALE_THRESHOLD_DAYS: i64 = ACTIVITY_WINDOW_DAYS;

    /// Raw numeric inputs collected by storage, before classification
    /// or scoring. Six fields, one per indicator; the first three
    /// are the original 0.6.0 set, the last three are added in 0.7.0
    /// to widen the health view.
    ///
    /// Kept as a struct of plain numbers so it stays cheap to store
    /// in a future `metrics_snapshots` table for trend tracking
    /// (Phase 2). Classification and scoring derive everything from
    /// this struct via pure functions.
    #[derive(Debug, Clone, Default)]
    pub struct ProjectHealthRaw {
        /// Total issues in the project (any status).
        pub total_issues: i64,
        /// Issues in `done`.
        pub done_issues: i64,
        /// Age in days of the oldest issue still in `open` or
        /// `in_progress`. `None` means no in-flight issues exist.
        pub oldest_in_flight_age_days: Option<i64>,
        /// Number of issues created OR moved-to-done within the
        /// activity window (see [`ACTIVITY_WINDOW_DAYS`]).
        pub recent_activity_count: i64,
        /// Issues currently in flight (open / in_progress).
        pub in_flight_issues: i64,
        /// Of those in-flight issues, how many are assigned to the
        /// single most-loaded user. The Bus-Factor indicator
        /// compares this against `in_flight_issues`.
        pub top_assignee_in_flight_issues: i64,
        /// Of in-flight issues, how many have not been touched in
        /// at least [`LONG_STALE_THRESHOLD_DAYS`].
        pub long_stale_in_flight_issues: i64,
        /// Number of users whose current WIP exceeds their effective
        /// WIP limit (project default or personal override).
        pub wip_violators: i64,
        /// Number of users with at least one in-flight issue
        /// assigned. Denominator for the WIP-compliance ratio.
        pub active_assignees: i64,
    }

    /// Backwards-compatible alias kept for the existing 0.6.0
    /// surface. The 0.7.0 design promotes [`ProjectHealthRaw`] +
    /// [`ProjectHealthReport`] as the main types, but
    /// [`ProjectHealth`] still exists so the original three
    /// classification functions keep working unchanged.
    pub type ProjectHealth = ProjectHealthRaw;

    // ------------------------------------------------------------
    // Original three classifiers (0.6.0). Kept as-is so existing
    // call sites continue to compile. The 0.7.0 path goes through
    // [`compute_report`] below, which also calls these.
    // ------------------------------------------------------------

    /// Throughput classification: the share of issues that have
    /// reached `done`.
    pub fn classify_throughput(h: &ProjectHealthRaw) -> HealthIndicator {
        if h.total_issues == 0 {
            return HealthIndicator::Insufficient;
        }
        let pct = (h.done_issues * 100) / h.total_issues;
        if pct >= 60 {
            HealthIndicator::Good
        } else if pct >= 30 {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Concern
        }
    }

    /// Staleness classification: how old is the oldest in-flight
    /// issue?
    pub fn classify_staleness(h: &ProjectHealthRaw) -> HealthIndicator {
        match h.oldest_in_flight_age_days {
            None => HealthIndicator::Good,
            Some(d) if d >= 28 => HealthIndicator::Concern,
            Some(d) if d >= 14 => HealthIndicator::Watch,
            Some(_) => HealthIndicator::Good,
        }
    }

    /// Activity classification: did the project see work in the
    /// activity window?
    pub fn classify_activity(h: &ProjectHealthRaw) -> HealthIndicator {
        if h.total_issues == 0 {
            return HealthIndicator::Insufficient;
        }
        if h.recent_activity_count >= 5 {
            HealthIndicator::Good
        } else if h.recent_activity_count >= 1 {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Concern
        }
    }

    // ------------------------------------------------------------
    // Three new classifiers (0.7.0).
    // ------------------------------------------------------------

    /// Bus-factor classification: what share of in-flight work is
    /// concentrated on the single most-loaded user? Concentration
    /// is a single-point-of-failure risk.
    ///
    /// Sole-assignee projects (only one active assignee at all) are
    /// trivially "100% concentrated" but classifying them as
    /// `Concern` would just shout the obvious — solo work is
    /// expected for solo projects. We classify these as `Watch`
    /// instead so the chip says "yes, this is a one-person project,
    /// add a teammate to reduce risk" without alarmism.
    pub fn classify_bus_factor(h: &ProjectHealthRaw) -> HealthIndicator {
        if h.in_flight_issues == 0 {
            return HealthIndicator::Insufficient;
        }
        if h.active_assignees <= 1 {
            return HealthIndicator::Watch;
        }
        let pct = (h.top_assignee_in_flight_issues * 100) / h.in_flight_issues;
        if pct >= 80 {
            HealthIndicator::Concern
        } else if pct >= 60 {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Good
        }
    }

    /// Long-stale classification: what share of in-flight issues
    /// have not been touched in at least
    /// [`LONG_STALE_THRESHOLD_DAYS`]?
    pub fn classify_long_stale(h: &ProjectHealthRaw) -> HealthIndicator {
        if h.in_flight_issues == 0 {
            return HealthIndicator::Insufficient;
        }
        let pct = (h.long_stale_in_flight_issues * 100) / h.in_flight_issues;
        if pct >= 40 {
            HealthIndicator::Concern
        } else if pct >= 20 {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Good
        }
    }

    /// WIP-compliance classification: what share of active assignees
    /// are over their WIP limit right now?
    pub fn classify_wip_compliance(h: &ProjectHealthRaw) -> HealthIndicator {
        if h.active_assignees == 0 {
            return HealthIndicator::Insufficient;
        }
        let pct = (h.wip_violators * 100) / h.active_assignees;
        if pct >= 50 {
            HealthIndicator::Concern
        } else if pct >= 1 {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Good
        }
    }

    // ------------------------------------------------------------
    // 0.7.0 scoring layer: normalisation, weighting, summary.
    // ------------------------------------------------------------

    /// Identifier for the indicator family, used for stable iteration
    /// order and for matching specific indicators in tests / UI.
    /// Adding a new variant is the canonical way to extend the
    /// health view in future releases.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IndicatorKind {
        Throughput,
        Staleness,
        Activity,
        BusFactor,
        LongStale,
        WipCompliance,
    }

    impl IndicatorKind {
        pub fn label(&self) -> &'static str {
            match self {
                Self::Throughput => "Throughput",
                Self::Staleness => "Oldest in-flight",
                Self::Activity => "Activity (14d)",
                Self::BusFactor => "Bus factor",
                Self::LongStale => "Long-stale",
                Self::WipCompliance => "WIP compliance",
            }
        }

        /// Short description shown as a tooltip / `<details>` body
        /// to explain what the indicator measures.
        pub fn description(&self) -> &'static str {
            match self {
                Self::Throughput => {
                    "Share of issues that have reached Done."
                }
                Self::Staleness => {
                    "Age of the oldest issue still Open or In Progress."
                }
                Self::Activity => {
                    "Issues created or finished in the last 14 days."
                }
                Self::BusFactor => {
                    "Concentration of in-flight work on a single user."
                }
                Self::LongStale => {
                    "Share of in-flight issues untouched for over two weeks."
                }
                Self::WipCompliance => {
                    "Share of active users currently over their WIP limit."
                }
            }
        }
    }

    /// One slot in the health report. Storage produces a vector of
    /// these via [`compute_report`]; UI iterates over them.
    ///
    /// The `Indicator` shape is deliberately uniform across every
    /// kind. Adding a new metric in a future release means adding a
    /// variant to [`IndicatorKind`], a normalisation case to
    /// [`normalize`], a default weight in [`HealthWeights`], and the
    /// raw input in [`ProjectHealthRaw`] — *not* changing the UI or
    /// the report-rendering code.
    #[derive(Debug, Clone)]
    pub struct Indicator {
        pub kind: IndicatorKind,
        /// Display label, e.g. "Throughput".
        pub label: &'static str,
        /// Pre-formatted display value, e.g. "5 / 7 (71%)" or "8 d".
        pub value_display: String,
        /// Coarse-grained classification for badge colour /
        /// accessibility text.
        pub state: HealthIndicator,
        /// Continuous quality on 0.0 (worst) – 1.0 (best). Score
        /// computation uses this.
        pub normalized: f64,
        /// Weight contributed to the composite score.
        pub weight: f64,
    }

    /// Per-indicator weights. Must sum (approximately) to 1.0.
    ///
    /// 0.7.0 ships a single fixed `DEFAULT` set. The roadmap notes
    /// project-type-specific weights (new vs. maintenance, small vs.
    /// large team) as a future refinement; that arrives as a
    /// `weights_for(project_type)` lookup, not as a change to this
    /// struct.
    #[derive(Debug, Clone, Copy)]
    pub struct HealthWeights {
        pub throughput: f64,
        pub staleness: f64,
        pub activity: f64,
        pub bus_factor: f64,
        pub long_stale: f64,
        pub wip_compliance: f64,
    }

    impl HealthWeights {
        pub const DEFAULT: HealthWeights = HealthWeights {
            throughput: 0.20,
            staleness: 0.20,
            activity: 0.15,
            bus_factor: 0.15,
            long_stale: 0.15,
            wip_compliance: 0.15,
        };

        fn for_kind(&self, kind: IndicatorKind) -> f64 {
            match kind {
                IndicatorKind::Throughput => self.throughput,
                IndicatorKind::Staleness => self.staleness,
                IndicatorKind::Activity => self.activity,
                IndicatorKind::BusFactor => self.bus_factor,
                IndicatorKind::LongStale => self.long_stale,
                IndicatorKind::WipCompliance => self.wip_compliance,
            }
        }
    }

    /// Trend direction for the composite score relative to the
    /// previous snapshot. Phase 1 has no history — every report is
    /// [`Trend::Unavailable`] until a `metrics_snapshots` table
    /// arrives in Phase 2.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Trend {
        Unavailable,
        Up { delta: u8 },
        Down { delta: u8 },
        Flat,
    }

    /// Composite project-health score on 0–100, plus enough metadata
    /// to render a single sentence summary above the indicator
    /// breakdown.
    #[derive(Debug, Clone)]
    pub struct HealthScore {
        pub value: u8,
        pub state: HealthIndicator,
        /// One- or two-sentence natural-language summary,
        /// foregrounding the worst indicators. Produced by
        /// [`summarize`].
        pub summary: String,
        pub trend: Trend,
    }

    /// Full report: composite score plus per-indicator breakdown.
    /// This is what storage returns and the UI renders.
    #[derive(Debug, Clone)]
    pub struct ProjectHealthReport {
        pub score: HealthScore,
        pub indicators: Vec<Indicator>,
        pub raw: ProjectHealthRaw,
    }

    /// Map a quality classification to a continuous 0–1 value used
    /// for score weighting. The split-points reuse the
    /// classifier thresholds where natural; intermediate values are
    /// piecewise-linear interpolations so a project doesn't
    /// rubber-band between scores when an issue ages by one day.
    pub fn normalize(kind: IndicatorKind, raw: &ProjectHealthRaw) -> f64 {
        match kind {
            IndicatorKind::Throughput => normalize_throughput(raw),
            IndicatorKind::Staleness => normalize_staleness(raw),
            IndicatorKind::Activity => normalize_activity(raw),
            IndicatorKind::BusFactor => normalize_bus_factor(raw),
            IndicatorKind::LongStale => normalize_long_stale(raw),
            IndicatorKind::WipCompliance => normalize_wip_compliance(raw),
        }
    }

    fn normalize_throughput(h: &ProjectHealthRaw) -> f64 {
        // Empty project → neutral 0.5, scoring shouldn't punish or
        // reward a project that hasn't started. The classifier
        // returns Insufficient and the UI hides such chips, but the
        // score has to put *some* number in.
        if h.total_issues == 0 {
            return 0.5;
        }
        let pct = (h.done_issues * 100) / h.total_issues;
        // 0% → 0.0, 30% → 0.5, 60% → 0.85, 100% → 1.0
        let pct = pct as f64;
        if pct >= 60.0 {
            0.85 + (pct - 60.0) / 60.0 * 0.15
        } else if pct >= 30.0 {
            0.5 + (pct - 30.0) / 30.0 * 0.35
        } else {
            pct / 60.0
        }
        .clamp(0.0, 1.0)
    }

    fn normalize_staleness(h: &ProjectHealthRaw) -> f64 {
        match h.oldest_in_flight_age_days {
            None => 1.0,
            Some(d) if d <= 7 => 1.0,
            Some(d) if d < 14 => 0.85 - (d - 7) as f64 / 7.0 * 0.15,
            Some(d) if d < 28 => 0.7 - (d - 14) as f64 / 14.0 * 0.5,
            Some(_) => 0.0,
        }
        .clamp(0.0, 1.0)
    }

    fn normalize_activity(h: &ProjectHealthRaw) -> f64 {
        if h.total_issues == 0 {
            return 0.5;
        }
        match h.recent_activity_count {
            n if n >= 5 => 1.0,
            n if n >= 1 => 0.5 + (n - 1) as f64 / 4.0 * 0.4,
            _ => 0.0,
        }
        .clamp(0.0, 1.0)
    }

    fn normalize_bus_factor(h: &ProjectHealthRaw) -> f64 {
        if h.in_flight_issues == 0 {
            return 0.5;
        }
        if h.active_assignees <= 1 {
            // Solo work — neutral-low rather than zero. Solo
            // projects shouldn't be punished out of all proportion;
            // they just can't score well on this metric.
            return 0.4;
        }
        let pct = (h.top_assignee_in_flight_issues * 100) / h.in_flight_issues;
        let pct = pct as f64;
        // 0–60% → great, 60–80% → linearly degrade, 80%+ → bad
        if pct < 60.0 {
            1.0
        } else if pct < 80.0 {
            1.0 - (pct - 60.0) / 20.0 * 0.7
        } else {
            (0.3 - (pct - 80.0) / 20.0 * 0.3).max(0.0)
        }
        .clamp(0.0, 1.0)
    }

    fn normalize_long_stale(h: &ProjectHealthRaw) -> f64 {
        if h.in_flight_issues == 0 {
            return 1.0;
        }
        let pct = (h.long_stale_in_flight_issues * 100) / h.in_flight_issues;
        let pct = pct as f64;
        if pct < 20.0 {
            1.0 - pct / 20.0 * 0.15
        } else if pct < 40.0 {
            0.85 - (pct - 20.0) / 20.0 * 0.55
        } else {
            (0.3 - (pct - 40.0) / 60.0 * 0.3).max(0.0)
        }
        .clamp(0.0, 1.0)
    }

    fn normalize_wip_compliance(h: &ProjectHealthRaw) -> f64 {
        if h.active_assignees == 0 {
            return 1.0;
        }
        let pct = (h.wip_violators * 100) / h.active_assignees;
        let pct = pct as f64;
        if pct == 0.0 {
            1.0
        } else if pct < 50.0 {
            0.85 - pct / 50.0 * 0.55
        } else {
            (0.3 - (pct - 50.0) / 50.0 * 0.3).max(0.0)
        }
        .clamp(0.0, 1.0)
    }

    /// Format the per-indicator value for display, e.g.
    /// `"5 / 7 (71%)"`, `"8 d"`, `"85% on top assignee"`.
    pub fn format_value(kind: IndicatorKind, raw: &ProjectHealthRaw) -> String {
        match kind {
            IndicatorKind::Throughput => {
                if raw.total_issues == 0 {
                    "—".to_string()
                } else {
                    let pct = (raw.done_issues * 100) / raw.total_issues;
                    format!("{} / {} ({}%)", raw.done_issues, raw.total_issues, pct)
                }
            }
            IndicatorKind::Staleness => match raw.oldest_in_flight_age_days {
                None => "—".to_string(),
                Some(d) => format!("{d} d"),
            },
            IndicatorKind::Activity => format!("{}", raw.recent_activity_count),
            IndicatorKind::BusFactor => {
                if raw.in_flight_issues == 0 {
                    "—".to_string()
                } else if raw.active_assignees <= 1 {
                    "solo".to_string()
                } else {
                    let pct = (raw.top_assignee_in_flight_issues * 100) / raw.in_flight_issues;
                    format!("{}% on top", pct)
                }
            }
            IndicatorKind::LongStale => {
                if raw.in_flight_issues == 0 {
                    "—".to_string()
                } else {
                    format!(
                        "{} / {}",
                        raw.long_stale_in_flight_issues, raw.in_flight_issues
                    )
                }
            }
            IndicatorKind::WipCompliance => {
                if raw.active_assignees == 0 {
                    "—".to_string()
                } else if raw.wip_violators == 0 {
                    "all within".to_string()
                } else {
                    format!("{} over", raw.wip_violators)
                }
            }
        }
    }

    /// Map a kind to its 0.6.0-style coarse classification. Kept as
    /// a small dispatcher so all per-kind branching lives next to
    /// the kind enum.
    pub fn classify(kind: IndicatorKind, raw: &ProjectHealthRaw) -> HealthIndicator {
        match kind {
            IndicatorKind::Throughput => classify_throughput(raw),
            IndicatorKind::Staleness => classify_staleness(raw),
            IndicatorKind::Activity => classify_activity(raw),
            IndicatorKind::BusFactor => classify_bus_factor(raw),
            IndicatorKind::LongStale => classify_long_stale(raw),
            IndicatorKind::WipCompliance => classify_wip_compliance(raw),
        }
    }

    /// All indicator kinds in canonical order. The UI iterates this
    /// list rather than hard-coding a sequence; new indicators
    /// added here automatically appear in the report.
    pub const ALL_INDICATORS: &[IndicatorKind] = &[
        IndicatorKind::Throughput,
        IndicatorKind::Staleness,
        IndicatorKind::Activity,
        IndicatorKind::BusFactor,
        IndicatorKind::LongStale,
        IndicatorKind::WipCompliance,
    ];

    /// Build the full health report from raw inputs.
    pub fn compute_report(raw: ProjectHealthRaw) -> ProjectHealthReport {
        compute_report_with_weights(raw, HealthWeights::DEFAULT)
    }

    /// Variant of [`compute_report`] that accepts custom weights —
    /// the entry point for the future per-project-type weight set.
    pub fn compute_report_with_weights(
        raw: ProjectHealthRaw,
        weights: HealthWeights,
    ) -> ProjectHealthReport {
        compute_report_full(raw, weights, &[])
    }

    /// Variant of [`compute_report`] that classifies the trend
    /// against a slice of recent past `score_value`s. The trend is
    /// computed via [`classify_trend`]; an empty `past_scores`
    /// slice yields [`Trend::Unavailable`] so this function is
    /// safe to call on day one when no snapshots exist.
    pub fn compute_report_with_trend(
        raw: ProjectHealthRaw,
        past_scores: &[u8],
    ) -> ProjectHealthReport {
        compute_report_full(raw, HealthWeights::DEFAULT, past_scores)
    }

    fn compute_report_full(
        raw: ProjectHealthRaw,
        weights: HealthWeights,
        past_scores: &[u8],
    ) -> ProjectHealthReport {
        let indicators: Vec<Indicator> = ALL_INDICATORS
            .iter()
            .map(|&kind| Indicator {
                kind,
                label: kind.label(),
                value_display: format_value(kind, &raw),
                state: classify(kind, &raw),
                normalized: normalize(kind, &raw),
                weight: weights.for_kind(kind),
            })
            .collect();

        let score_value = composite_score(&indicators);
        let state = classify_score(score_value);
        let summary = summarize(&indicators);
        let trend = classify_trend(score_value, past_scores);

        ProjectHealthReport {
            score: HealthScore {
                value: score_value,
                state,
                summary,
                trend,
            },
            indicators,
            raw,
        }
    }

    /// Threshold (in points on the 0–100 score) below which a
    /// score change is reported as `Trend::Flat` rather than
    /// Up/Down. Five points is small enough not to swallow
    /// genuine improvement / decline signals, large enough that
    /// week-over-week noise from a single closed issue doesn't
    /// register as movement.
    pub const TREND_FLAT_THRESHOLD: u8 = 5;

    /// Lower bound (days ago) of the trend's "past baseline"
    /// window. Snapshots more recent than this are excluded —
    /// week-over-week comparison would otherwise be polluted by
    /// the present; a Friday-vs-Thursday score drop is a
    /// weekend-pause artefact, not a real trend.
    pub const TREND_PAST_WINDOW_MIN_DAYS: i64 = 7;

    /// Upper bound (days ago) of the trend's past baseline window.
    /// Snapshots older than this are excluded — they don't
    /// represent "how things are right now". Two weeks gives
    /// enough samples for a stable median while keeping the
    /// baseline timely.
    pub const TREND_PAST_WINDOW_MAX_DAYS: i64 = 14;

    /// Classify a trend from the current score against a slice of
    /// past scores. The "past baseline" is the median of
    /// `past_scores`; comparing against the median rather than the
    /// mean keeps an outlier point from skewing the trend.
    ///
    /// Empty `past_scores` ⇒ `Trend::Unavailable`. A single past
    /// value works (it's its own median). Two values ⇒ average of
    /// the two (standard median behaviour).
    ///
    /// `delta` in `Up { delta }` / `Down { delta }` is `|current -
    /// median|` clamped to 0–100.
    pub fn classify_trend(current: u8, past_scores: &[u8]) -> Trend {
        if past_scores.is_empty() {
            return Trend::Unavailable;
        }
        let mut sorted: Vec<u8> = past_scores.to_vec();
        sorted.sort_unstable();
        let n = sorted.len();
        let median: u16 = if n % 2 == 1 {
            sorted[n / 2] as u16
        } else {
            // Average of the two middle elements; integer arithmetic
            // is fine since we floor to u8 anyway.
            (sorted[n / 2 - 1] as u16 + sorted[n / 2] as u16) / 2
        };

        let diff = (current as i16) - (median as i16);
        if diff.unsigned_abs() < TREND_FLAT_THRESHOLD as u16 {
            Trend::Flat
        } else if diff > 0 {
            Trend::Up {
                delta: diff.min(100) as u8,
            }
        } else {
            Trend::Down {
                delta: (-diff).min(100) as u8,
            }
        }
    }

    /// Weighted-sum 0–100 composite. Indicators with `Insufficient`
    /// state are excluded from both numerator and denominator so an
    /// empty project doesn't pull the score down with neutral 0.5
    /// fillers; the score reflects only the indicators that have
    /// real signal.
    fn composite_score(indicators: &[Indicator]) -> u8 {
        let mut weighted_sum = 0.0;
        let mut weight_total = 0.0;
        for ind in indicators {
            if matches!(ind.state, HealthIndicator::Insufficient) {
                continue;
            }
            weighted_sum += ind.normalized * ind.weight;
            weight_total += ind.weight;
        }
        if weight_total == 0.0 {
            return 50; // every indicator is Insufficient — neutral.
        }
        ((weighted_sum / weight_total) * 100.0).round().clamp(0.0, 100.0) as u8
    }

    fn classify_score(value: u8) -> HealthIndicator {
        if value >= 75 {
            HealthIndicator::Good
        } else if value >= 50 {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Concern
        }
    }

    /// Build the natural-language summary for the score header.
    /// Foregrounds at most two indicators that are pulling the
    /// score down; ignores `Insufficient` ones.
    pub fn summarize(indicators: &[Indicator]) -> String {
        // Collect concerning + watch states, sorted by severity then
        // by weight (heavier indicators surface first).
        let mut bad: Vec<&Indicator> = indicators
            .iter()
            .filter(|i| {
                matches!(i.state, HealthIndicator::Concern | HealthIndicator::Watch)
            })
            .collect();
        bad.sort_by(|a, b| {
            // Concern outranks Watch, then heavier weight first.
            let order = |s| match s {
                HealthIndicator::Concern => 0,
                HealthIndicator::Watch => 1,
                _ => 2,
            };
            order(a.state)
                .cmp(&order(b.state))
                .then_with(|| {
                    b.weight
                        .partial_cmp(&a.weight)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        if bad.is_empty() {
            return "Looking healthy.".to_string();
        }

        let lead = bad[0];
        match (bad.len(), lead.state) {
            (1, HealthIndicator::Concern) => {
                format!("{} is a concern.", lead.label)
            }
            (1, _) => format!("{} is worth a glance.", lead.label),
            (_, HealthIndicator::Concern) => {
                let second = bad[1];
                format!(
                    "{} is a concern; {} also needs attention.",
                    lead.label, second.label
                )
            }
            _ => {
                let second = bad[1];
                format!("{} and {} are worth a glance.", lead.label, second.label)
            }
        }
    }
}

// ============================================================
// Personal metrics (0.7.0).
// ============================================================

/// Per-user metrics scoped to a single user, intended for that
/// user's personal dashboard. By design this is *not* exposed to
/// other users — see the brief in
/// `peisear-プロジェクトヘルス最適化とヒューマンバーンダウン防止機能向け拡張開発指示書.md`
/// (V2.1) §0.2 and §2.5: "個人活動の詳細は必要最小限だけ共有する".
///
/// In 0.7.0 every user sees only their own metrics; the planned
/// manager / neutral-third-party roles arrive with the Team feature.
pub mod personal_metrics {
    use super::HealthIndicator;

    /// System-wide default WIP cap. Resolution order:
    /// 1. user.wip_limit if set, else
    /// 2. project.wip_limit_default if set, else
    /// 3. this constant.
    pub const DEFAULT_WIP_LIMIT: i64 = 3;

    /// Window over which "I finished N issues" is counted on the
    /// personal throughput chip. Same window as
    /// [`super::project_health::ACTIVITY_WINDOW_DAYS`] for symmetry.
    pub const PERSONAL_ACTIVITY_WINDOW_DAYS: i64 =
        super::project_health::ACTIVITY_WINDOW_DAYS;

    /// Snapshot of a single user's current load and recent rhythm,
    /// scoped to one project.
    ///
    /// "Scoped to one project" because today peisear has no
    /// cross-project notion of a user's total work — adding it is a
    /// future refinement that the V2.1 brief alludes to (§2.4
    /// "本人優先"). The struct shape stays the same when a global
    /// view arrives; the storage query becomes a
    /// `for_user_global(user_id)` sibling.
    #[derive(Debug, Clone)]
    pub struct PersonalMetrics {
        pub user_id: String,
        pub display_name: String,
        /// Resolved WIP limit for this user-in-this-project.
        pub effective_wip_limit: i64,
        /// Number of issues currently in `in_progress` assigned to
        /// this user. (Open issues that are not yet started don't
        /// count — the WIP framing is about *active* work.)
        pub current_wip: i64,
        /// Sum of effort over assigned in-flight issues
        /// (`open` + `in_progress`). Mirrors the load shown in
        /// [`super::UserLoad`] for parity with the project workload
        /// strip.
        pub in_flight_points: i64,
        /// User's optional capacity cap (story points). Mirrors
        /// [`super::UserLoad::capacity_points`].
        pub capacity_points: Option<i64>,
        /// Issues this user moved to `done` within
        /// [`PERSONAL_ACTIVITY_WINDOW_DAYS`].
        pub recent_done_count: i64,
        /// In-flight issues assigned to this user that have not
        /// been touched in at least [`PERSONAL_ACTIVITY_WINDOW_DAYS`].
        pub long_stale_count: i64,
        /// Crude estimation skew: average days-per-point spent on
        /// recently-completed estimated issues. `None` when no
        /// recent done-with-effort issues exist to fit a curve to.
        ///
        /// "Crude" because the source signal is
        /// `updated_at - created_at`, which conflates time-on-issue
        /// with calendar time during which the user wasn't even
        /// looking. Phase 2 (the planned `issue_events` table)
        /// replaces this with the actual `in_progress → done`
        /// elapsed time. The number is shown to the user as a soft
        /// reflection prompt, not as a performance indicator.
        pub estimation_skew_days_per_point: Option<f64>,
    }

    /// Three-state classification of WIP usage. Mirrors the
    /// `HealthIndicator` palette used elsewhere on the page so the
    /// personal dashboard looks visually coherent with project
    /// health.
    pub fn classify_wip(m: &PersonalMetrics) -> HealthIndicator {
        if m.current_wip == 0 {
            return HealthIndicator::Good;
        }
        if m.current_wip > m.effective_wip_limit {
            HealthIndicator::Concern
        } else if m.current_wip == m.effective_wip_limit {
            HealthIndicator::Watch
        } else {
            HealthIndicator::Good
        }
    }

    /// Classification of long-stale issues assigned to the user.
    /// "It's normal to have one stale issue you haven't picked
    /// up yet" → Watch above zero. Two or more is `Concern` because
    /// it usually means a backlog is forming, not just one stuck
    /// task.
    pub fn classify_long_stale(m: &PersonalMetrics) -> HealthIndicator {
        match m.long_stale_count {
            0 => HealthIndicator::Good,
            1 => HealthIndicator::Watch,
            _ => HealthIndicator::Concern,
        }
    }
}
