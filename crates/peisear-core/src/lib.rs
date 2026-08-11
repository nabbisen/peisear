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
use peisear_i18n::{IssueStatusLabel, PriorityLabel};
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
    /// Optional team this project belongs to. `None` is a
    /// personal project (the default; see migration 0011 for
    /// the team feature). Members of the linked team get access
    /// to the project's issues per their team role.
    pub team_id: Option<String>,
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

    /// The word this status renders as, via `peisear-i18n`'s message
    /// table. `I18N-005b` §4 calls this "the fifth prose function"
    /// this crate held outside the table (`IndicatorKind::label()`,
    /// `I18N-004`, was the first absorbed this way) — this crate
    /// constructs the closed-set value; `peisear-i18n`'s
    /// `MessageKey::IssueStatusName` owns the actual English word.
    pub fn to_i18n_label(self) -> IssueStatusLabel {
        match self {
            Self::Open => IssueStatusLabel::Open,
            Self::InProgress => IssueStatusLabel::InProgress,
            Self::Done => IssueStatusLabel::Done,
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

    /// See [`IssueStatus::to_i18n_label`] — same absorption, same
    /// reason.
    pub fn to_i18n_label(self) -> PriorityLabel {
        match self {
            Self::Low => PriorityLabel::Low,
            Self::Medium => PriorityLabel::Medium,
            Self::High => PriorityLabel::High,
            Self::Urgent => PriorityLabel::Urgent,
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
    /// Parent issue id when this row is a sub-issue (Phase C
    /// PR1, peisear-feature-spec-v2.1 §8.3). `None` means
    /// "this is a top-level issue" — the common case.
    ///
    /// Constraints (enforced by triggers in migration 0015):
    ///
    /// - The parent must be top-level itself; sub-issues
    ///   cannot have sub-issues (one level only).
    /// - The parent must be in the same project.
    /// - An issue cannot be its own parent, transitively or
    ///   directly.
    /// - An issue with existing children cannot be demoted
    ///   into being a sub-issue itself (would create a 2-
    ///   level chain).
    ///
    /// Sub-issues inherit nothing automatically: assignee,
    /// status, priority, effort, and sprint membership are
    /// all independently settable. The one effective tie is
    /// "parent's sprint determines the sub-issue's sprint" —
    /// a UI-level rule (sub-issues don't get their own sprint
    /// row in the planning surface) rather than a schema
    /// constraint.
    pub parent_issue_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Issue {
    /// True if this issue is a sub-issue of another. Convenience
    /// over checking the field directly so calling sites read
    /// like prose ("if issue.is_sub_issue() { ... }").
    pub fn is_sub_issue(&self) -> bool {
        self.parent_issue_id.is_some()
    }

    /// True if this issue is at the top level of the
    /// hierarchy. The complement of `is_sub_issue`. Top-level
    /// issues are the only ones rendered on the kanban board,
    /// the project list, and the sprint plan view; sub-issues
    /// appear inline in their parent's detail page (§8.5).
    pub fn is_top_level(&self) -> bool {
        self.parent_issue_id.is_none()
    }
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

/// The severity ceiling for presentation (`NFR-LANG-002`, `SPEC §28.2`):
/// no user-visible surface or API response may display a state above
/// `Watch`. `HealthIndicator` keeps `Concern` because it is accurate —
/// computation is not the boundary being fixed here (`FR-HLT-009`). This
/// type is the boundary: presentation code renders `DisplayHealthState`,
/// never `HealthIndicator` directly, and the only way to obtain one is
/// [`DisplayHealthState::clamp`], which folds `Concern` into `Watch`. A
/// render site that forgets to clamp fails to compile rather than
/// silently leaking a state above the ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayHealthState {
    Insufficient,
    Good,
    Watch,
}

impl DisplayHealthState {
    /// Fold the internal four-state model to the three displayable
    /// states. `Concern` becomes `Watch` — the ceiling, not a fourth
    /// bucket.
    pub fn clamp(state: HealthIndicator) -> Self {
        match state {
            HealthIndicator::Insufficient => Self::Insufficient,
            HealthIndicator::Good => Self::Good,
            HealthIndicator::Watch | HealthIndicator::Concern => Self::Watch,
        }
    }

    /// daisyUI badge class. Only the ghost/success/warning palette is
    /// reachable here — there is no danger class to reach for `Watch`,
    /// unlike the unclamped [`HealthIndicator`] this replaces at the
    /// render boundary.
    pub fn badge_class(&self) -> &'static str {
        match self {
            Self::Insufficient => "badge-ghost",
            Self::Good => "badge-success",
            Self::Watch => "badge-warning",
        }
    }

    /// Glyph + accessible-name fragment, paired with the badge so a
    /// screen reader or colour-blind user gets the same signal a
    /// sighted user gets from colour alone (`NFR-A11Y-004`).
    pub fn glyph(&self) -> (&'static str, &'static str) {
        match self {
            Self::Insufficient => ("—", "no data"),
            Self::Good => ("✓", "good"),
            Self::Watch => ("⚠", "watch"),
        }
    }
}

impl From<HealthIndicator> for DisplayHealthState {
    fn from(state: HealthIndicator) -> Self {
        Self::clamp(state)
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
    use super::{DisplayHealthState, HealthIndicator};
    use peisear_i18n::{IndicatorLabel, MessageKey};

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
        /// Convert to `peisear_i18n`'s label enum, the sole remaining
        /// source of this indicator's display name (`I18N-004`).
        ///
        /// This crate used to have its own `label()` returning the
        /// same six strings directly (`&'static str`) — removed here.
        /// `I18N-002` had already added `IndicatorLabel` in
        /// `peisear-i18n` for use *inside* `summarize`'s sentences,
        /// without noticing `label()` was a second copy of the same
        /// six strings with nothing keeping them in sync (flagged in
        /// I18N-002's review §1.4 as a fifth prose-producing function
        /// I18N-002's own survey missed). `peisear-i18n` is a leaf
        /// crate (`I18N-001` §4.1) and cannot know about
        /// `IndicatorKind`, so this conversion method — not `label()`
        /// itself returning an i18n type — is the translation point.
        /// Every caller that used to read `label()` or
        /// `Indicator.label` now goes through this plus
        /// `peisear_i18n::MessageKey::IndicatorName` and a render
        /// call; see `components/issues.rs`'s `indicator_row` in
        /// `peisear-web`.
        pub fn to_i18n_label(self) -> IndicatorLabel {
            match self {
                Self::Throughput => IndicatorLabel::Throughput,
                Self::Staleness => IndicatorLabel::Staleness,
                Self::Activity => IndicatorLabel::Activity,
                Self::BusFactor => IndicatorLabel::BusFactor,
                Self::LongStale => IndicatorLabel::LongStale,
                Self::WipCompliance => IndicatorLabel::WipCompliance,
            }
        }

        /// Short description shown as a tooltip / `<details>` body
        /// to explain what the indicator measures.
        pub fn description(&self) -> &'static str {
            match self {
                Self::Throughput => "Share of issues that have reached Done.",
                Self::Staleness => "Age of the oldest issue still Open or In Progress.",
                Self::Activity => "Issues created or finished in the last 14 days.",
                Self::BusFactor => "Concentration of in-flight work on a single user.",
                Self::LongStale => "Share of in-flight issues untouched for over two weeks.",
                Self::WipCompliance => "Share of active users currently over their WIP limit.",
            }
        }
    }

    /// One slot in the health report. Storage produces a vector of
    /// these via [`compute_report`]; UI iterates over them.
    ///
    /// The `Indicator` shape is deliberately uniform across every
    /// kind. Adding a new metric in a future release means adding a
    /// variant to [`IndicatorKind`], a normalisation case to
    /// `compute_report`, and (optionally) an explanation arm in
    /// [`Indicator::human_explanation`] — UI rendering itself is
    /// shared.
    #[derive(Debug, Clone)]
    pub struct Indicator {
        /// Also the source of this indicator's display name via
        /// [`IndicatorKind::to_i18n_label`] — there is no separate
        /// `label` field here since `I18N-004`; the six name strings
        /// live only in `peisear-i18n` now.
        pub kind: IndicatorKind,
        /// Descriptor for the display value, e.g. "5 / 7 (71%)" or
        /// "8 d" once rendered. `-web` renders it via
        /// `peisear_i18n::Locale::render` — this field carries the
        /// key, not the rendered text (`I18N-002`; was `String`
        /// before this handoff).
        pub value_display: MessageKey,
        /// Coarse-grained classification for badge colour /
        /// accessibility text.
        pub state: HealthIndicator,
        /// Continuous quality on 0.0 (worst) – 1.0 (best). Score
        /// computation uses this.
        pub normalized: f64,
        /// Weight contributed to the composite score.
        pub weight: f64,
    }

    impl Indicator {
        /// Plain-language explanation of the current state of this
        /// indicator, suitable for direct display to a user.
        ///
        /// Phase B (peisear-feature-spec-v2.1 §5.2 / decision B-E5):
        /// project-health explainability prefers human language
        /// over numbers. The brief explicitly favours "what's
        /// happening" over "score breakdown":
        ///
        /// > Don't show: 'long_stale_ratio = 0.30 (-15)'
        /// > Show: '3 issues haven't moved in over two weeks'
        ///
        /// The text adapts to `state` so that:
        ///
        /// - `Good` returns `None` — no story to tell, the UI
        ///   omits the row entirely. (Listing every Good
        ///   indicator turns into self-congratulatory clutter.)
        /// - `Watch` and `Concern` return a concrete sentence
        ///   naming the underlying value (taken from
        ///   `value_display`) and the indicator's domain, so
        ///   the user can act on it.
        /// - `Insufficient` returns `None` — the indicator
        ///   doesn't have enough data to say anything about
        ///   the project yet.
        ///
        /// The phrasing sticks to neutral, descriptive language.
        /// We deliberately avoid evaluative words like "bad",
        /// "concerning", or "you need to" — the user reads
        /// these every day, and §0.2 wants the tool to be a
        /// dashboard, not a coach.
        ///
        /// Takes `raw` — the same [`ProjectHealthRaw`] `format_value`
        /// derives this indicator's value from — rather than reading
        /// `self.value_display`, so the typed parameters on the
        /// returned [`MessageKey`] come from raw numbers, not a
        /// pre-formatted string (`RFC 006` requirement 7: "if you
        /// find yourself passing a `String` that was already
        /// formatted, the formatting belongs on the presentation
        /// side" — `I18N-002` §5.2). This is a signature change from
        /// the pre-`I18N-002` `&self`-only method; every caller
        /// already holds a [`ProjectHealthReport`], which carries
        /// `raw` alongside `indicators`.
        pub fn human_explanation(&self, raw: &ProjectHealthRaw) -> Option<MessageKey> {
            // Good and Insufficient produce no row.
            match self.state {
                HealthIndicator::Good | HealthIndicator::Insufficient => return None,
                HealthIndicator::Watch | HealthIndicator::Concern => {}
            }

            Some(match self.kind {
                IndicatorKind::Throughput => MessageKey::IndicatorExplanationThroughput {
                    done: raw.done_issues,
                    total: raw.total_issues,
                },
                IndicatorKind::Staleness => MessageKey::IndicatorExplanationStaleness {
                    // `Some` is guaranteed here: `classify_staleness`
                    // only reaches `Watch`/`Concern` when
                    // `oldest_in_flight_age_days` is `Some(d)` with
                    // `d >= 14`; `None` classifies as `Good`, which
                    // already returned above.
                    days: raw
                        .oldest_in_flight_age_days
                        .expect("Staleness reaches Watch/Concern only when a value exists"),
                },
                IndicatorKind::Activity => MessageKey::IndicatorExplanationActivity {
                    count: raw.recent_activity_count,
                },
                // ISSUE-006 finding 2, fixed in I18N-004: this used
                // to feed the percentage-shaped template a
                // non-percentage value for the solo case, producing
                // a broken sentence. IndicatorExplanationBusFactorSolo
                // is now its own sentence in peisear-i18n, typed
                // parameters keeping the two cases structurally
                // distinct rather than sharing one template.
                IndicatorKind::BusFactor => {
                    if raw.active_assignees <= 1 {
                        MessageKey::IndicatorExplanationBusFactorSolo
                    } else {
                        let pct = (raw.top_assignee_in_flight_issues * 100) / raw.in_flight_issues;
                        MessageKey::IndicatorExplanationBusFactor { pct }
                    }
                }
                IndicatorKind::LongStale => MessageKey::IndicatorExplanationLongStale {
                    stale: raw.long_stale_in_flight_issues,
                    in_flight: raw.in_flight_issues,
                },
                // ISSUE-006 finding 3, fixed in I18N-004: the count
                // is now a typed parameter rather than an embedded
                // "N over" string, which is what produced the
                // doubling. Reproduced live before this fix — see
                // I18N-004's review request.
                IndicatorKind::WipCompliance => MessageKey::IndicatorExplanationWipCompliance {
                    count: raw.wip_violators,
                },
            })
        }
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
        /// Descriptor for the one- or two-sentence natural-language
        /// summary, foregrounding the worst indicators. Produced by
        /// [`summarize`]; `-web` renders it via
        /// `peisear_i18n::Locale::render` (`I18N-002`; was `String`
        /// before this handoff).
        pub summary: MessageKey,
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

    /// Descriptor for the per-indicator value display, e.g.
    /// `"5 / 7 (71%)"`, `"8 d"`, `"85% on top"` once rendered. `-web`
    /// renders it via `peisear_i18n::Locale::render` (`I18N-002`) —
    /// this crate emits the key, it does not render it (`I18N-002`
    /// §5.1, §8).
    pub fn format_value(kind: IndicatorKind, raw: &ProjectHealthRaw) -> MessageKey {
        match kind {
            IndicatorKind::Throughput => {
                if raw.total_issues == 0 {
                    MessageKey::IndicatorValueUnavailable
                } else {
                    MessageKey::IndicatorValueThroughput {
                        done: raw.done_issues,
                        total: raw.total_issues,
                    }
                }
            }
            IndicatorKind::Staleness => match raw.oldest_in_flight_age_days {
                None => MessageKey::IndicatorValueUnavailable,
                Some(days) => MessageKey::IndicatorValueStaleness { days },
            },
            IndicatorKind::Activity => MessageKey::IndicatorValueActivity {
                count: raw.recent_activity_count,
            },
            IndicatorKind::BusFactor => {
                if raw.in_flight_issues == 0 {
                    MessageKey::IndicatorValueUnavailable
                } else if raw.active_assignees <= 1 {
                    MessageKey::IndicatorValueBusFactorSolo
                } else {
                    let pct = (raw.top_assignee_in_flight_issues * 100) / raw.in_flight_issues;
                    MessageKey::IndicatorValueBusFactor { pct }
                }
            }
            IndicatorKind::LongStale => {
                if raw.in_flight_issues == 0 {
                    MessageKey::IndicatorValueUnavailable
                } else {
                    MessageKey::IndicatorValueLongStale {
                        stale: raw.long_stale_in_flight_issues,
                        in_flight: raw.in_flight_issues,
                    }
                }
            }
            IndicatorKind::WipCompliance => {
                if raw.active_assignees == 0 {
                    MessageKey::IndicatorValueUnavailable
                } else if raw.wip_violators == 0 {
                    MessageKey::IndicatorValueWipAllWithin
                } else {
                    MessageKey::IndicatorValueWipOver {
                        count: raw.wip_violators,
                    }
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
        ((weighted_sum / weight_total) * 100.0)
            .round()
            .clamp(0.0, 100.0) as u8
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

    /// Build the descriptor for the score header's natural-language
    /// summary. Foregrounds at most two indicators that are pulling
    /// the score down; ignores indicators the ceiling doesn't
    /// display at all. `-web` renders the result via
    /// `peisear_i18n::Locale::render` (`I18N-002`; was `-> String`
    /// before this handoff).
    ///
    /// **`I18N-004`, fixing `ISSUE-006` finding 1.** Selection is
    /// filtered by [`DisplayHealthState`], not [`HealthIndicator`] —
    /// `DisplayHealthState` has no `Concern` variant, so a sentence
    /// naming `Concern` directly cannot be constructed here,
    /// structurally rather than by convention: the message-key match
    /// below has nothing left to distinguish a former-`Concern`
    /// indicator from a former-`Watch` one, because that information
    /// was never carried past the filter. `HealthIndicator`'s
    /// four-state ordering is still used, but only internally, in
    /// the *ranking* below — a genuinely `Concern`-tier indicator
    /// still leads over a `Watch`-tier one when both are present, per
    /// `ISSUE-006-decision.md` §3 ("clamp at the point the state
    /// reaches a sentence, not before the ranking"). This is DEV-004's
    /// fix applied one layer down: DEV-004 clamped the type badges
    /// and the composite chip render from; this clamps the type this
    /// function's own output decision is made from.
    pub fn summarize(indicators: &[Indicator]) -> MessageKey {
        let mut bad: Vec<&Indicator> = indicators
            .iter()
            .filter(|i| matches!(DisplayHealthState::from(i.state), DisplayHealthState::Watch))
            .collect();
        bad.sort_by(|a, b| {
            // Internal ranking only, per the doc comment above --
            // never consulted again once `bad` is filtered.
            let order = |s| match s {
                HealthIndicator::Concern => 0,
                HealthIndicator::Watch => 1,
                _ => 2,
            };
            order(a.state).cmp(&order(b.state)).then_with(|| {
                b.weight
                    .partial_cmp(&a.weight)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });

        if bad.is_empty() {
            return MessageKey::HealthSummaryHealthy;
        }

        let lead = bad[0];
        if bad.len() == 1 {
            MessageKey::HealthSummaryOneWatch {
                label: lead.kind.to_i18n_label(),
            }
        } else {
            let second = bad[1];
            MessageKey::HealthSummaryTwoWatch {
                first: lead.kind.to_i18n_label(),
                second: second.kind.to_i18n_label(),
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
    pub const PERSONAL_ACTIVITY_WINDOW_DAYS: i64 = super::project_health::ACTIVITY_WINDOW_DAYS;

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

/// Per-user fatigue / burnout signals.
///
/// Sibling to [`personal_metrics`]. The two modules together back
/// the personal dashboard at `/me`: `personal_metrics` answers
/// "where are you right now?" while `user_burnout` answers "how
/// have you been recently, and is it sustainable?".
///
/// The framing is deliberate. V2.1 §0.2 forbids using these signals
/// for performance evaluation; §1.2 calls for self-reflection
/// support. Concretely this means:
///
/// - Indicators classify only as `Insufficient` / `Good` / `Watch`.
///   We never reach `Concern` here. A "concerning" framing in this
///   territory crosses into "you are burnt out" telling, which is a
///   call for the person to make about themselves, not for software
///   to assert.
/// - The summary text is a question or suggestion, not a diagnosis.
/// - Numbers come with units the user can sanity-check ("over
///   capacity for 4 of the last 4 snapshots").
///
/// Future indicators (estimation drift trend, cognitive switching)
/// will land additively as new fields on
/// [`UserBurnoutSignals`] and new chips in the dashboard. The
/// shape is uniform across all signals so adding one is a
/// localised change.
pub mod user_burnout {
    use super::HealthIndicator;
    use peisear_i18n::MessageKey;

    /// Window (in days) over which the estimation drift comparison
    /// is computed. Four weeks gives two two-week halves: "now"
    /// vs. "earlier", median over each half. Shorter and either
    /// half might be empty for a user who completes a few issues
    /// per fortnight; longer and "earlier" stops being earlier in
    /// any meaningful sense.
    pub const DRIFT_WINDOW_DAYS: i64 = 28;

    /// Threshold (in ratio) below which the estimation drift is
    /// reported as `DriftDirection::Steady` rather than Up / Down.
    /// 25% movement is the "obviously different" line —  smaller
    /// is week-to-week noise from small samples. The number is
    /// dimensionless; computed as `(recent - older) / older`.
    pub const DRIFT_STEADY_THRESHOLD_RATIO: f64 = 0.25;

    /// Window (in days) over which cognitive switching is
    /// observed. Two weeks captures the user's working pattern
    /// without mixing in stale rhythm; same window as other
    /// per-user reflective signals on the dashboard for symmetry.
    pub const SWITCHING_WINDOW_DAYS: i64 = 14;

    /// Minimum number of `status_changed -> in_progress` events
    /// before the cognitive-switching pattern is meaningfully
    /// reportable. Two weeks of work with one or two transitions
    /// total is not enough data to characterise a "rhythm" — the
    /// number would just be noise. The UI hides the chip when
    /// `total_events_observed < SWITCHING_MIN_EVENTS`.
    pub const SWITCHING_MIN_EVENTS: i64 = 5;

    /// Number of consecutive over-capacity snapshots before
    /// `overload_streak_days` graduates to `Watch`. At the
    /// default 6-hour snapshot interval, 8 ≈ 2 days. Exposed
    /// as a public constant so the notification edge-trigger
    /// logic (in `crate::notifications`) doesn't keep its own
    /// copy of the threshold.
    pub const OVERLOAD_STREAK_WATCH: i64 = 8;

    /// Days a single in-flight assigned issue must be stalled
    /// before `stalled_assigned_max_days` reaches `Watch`.
    /// Same exposure rationale as `OVERLOAD_STREAK_WATCH`.
    pub const STALLED_WATCH_DAYS: i64 = 14;

    /// Direction of an estimation drift. `Steady` is reported when
    /// the relative change is below `DRIFT_STEADY_THRESHOLD_RATIO`,
    /// or when there is insufficient data to compute one half.
    ///
    /// Note the deliberate absence of any "good" or "bad"
    /// connotation. Drift up means recent issues took longer per
    /// point than older ones — that *might* signal the user is
    /// hitting harder problems, taking on more thought-heavy work,
    /// or having a rough fortnight. None of these are necessarily
    /// bad and none of them are addressable by software. The
    /// dashboard surfaces the fact and trusts the user to
    /// contextualise it.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DriftDirection {
        Up,
        Down,
        Steady,
    }

    /// Computed estimation drift over the configured window. Built
    /// by `peisear-storage::user_burnout::estimation_drift_for_user`.
    ///
    /// Returns `None` from that function when there isn't enough
    /// completed work in either half of the window to compare; the
    /// UI hides the chip in that case rather than showing
    /// "insufficient data" prominently. We respect the user's
    /// time and don't surface non-information.
    #[derive(Debug, Clone)]
    pub struct EstimationDriftTrend {
        /// Median days-per-point across the recent half of the
        /// window. None of the entries inside that half are
        /// individually surfaced; the median is the only number
        /// that matters at this layer.
        pub recent_median_days_per_point: f64,
        /// Same, for the older half of the window.
        pub older_median_days_per_point: f64,
        /// Direction classification. See [`DriftDirection`].
        pub direction: DriftDirection,
        /// The window total — surfaced so the UI can phrase
        /// the message correctly ("over the last X days").
        pub window_days: i64,
    }

    /// Cognitive-switching pattern over the configured window.
    /// Built by `peisear-storage::user_burnout::cognitive_switching_for_user`.
    ///
    /// "Switching" here is operationalised as the count of
    /// `status_changed -> in_progress` events for issues assigned
    /// to the user. Each such event is a "I'm picking this up
    /// now" moment; multiple per day is one signature of context
    /// switching. We don't try to detect *good* vs. *bad*
    /// switching — debugging sessions involve picking things up
    /// and putting them down too. The chip surfaces the rhythm
    /// without judgement.
    #[derive(Debug, Clone)]
    pub struct CognitiveSwitchingPattern {
        /// Median `-> in_progress` events per active day. "Active
        /// day" means a day on which any switching happened;
        /// quiet days don't count toward the median, which would
        /// otherwise dilute toward zero.
        pub switches_per_day_median: f64,
        /// Total events observed in the window. Surfaced so the
        /// UI can decline to render the chip if the data is too
        /// thin to characterise a rhythm.
        pub total_events_observed: i64,
        /// The window total — surfaced so the UI can phrase
        /// the message ("over the last X days").
        pub window_days: i64,
    }

    /// Snapshot of one user's burnout signals at request time.
    ///
    /// Only used as a return value from
    /// `peisear-storage::user_burnout::for_user`; the storage layer
    /// fills it in from the snapshot and event tables.
    #[derive(Debug, Clone)]
    pub struct UserBurnoutSignals {
        /// Number of consecutive most-recent snapshots where the
        /// user was over their capacity. `0` means either no
        /// snapshots, or the most recent one was within capacity.
        ///
        /// "Snapshots" not "days" because the snapshot interval is
        /// finer than daily (every 6 hours by default). The UI
        /// labels this as "snapshots" or "ticks" rather than
        /// inflating it to a days estimate.
        pub overload_streak_days: i64,

        /// For the user's oldest in-flight assigned issue, the
        /// days since its last status_changed event (or
        /// `updated_at` for legacy data). `0` if the user has no
        /// in-flight assigned work.
        pub stalled_assigned_max_days: i64,

        /// The window over which streaks are measured, surfaced
        /// here so the UI can phrase the message correctly
        /// ("over capacity for X of the last Y").
        pub window_days: i64,

        /// Estimation drift trend. `None` when the user does not
        /// have enough completed work in both halves of the
        /// drift window to compute a comparison. See
        /// [`EstimationDriftTrend`].
        ///
        /// New in 0.11.0.
        pub estimation_drift: Option<EstimationDriftTrend>,

        /// Cognitive-switching pattern. `None` when the user's
        /// total switch events in the window are below
        /// `SWITCHING_MIN_EVENTS`. See [`CognitiveSwitchingPattern`].
        ///
        /// New in 0.11.0.
        pub cognitive_switching: Option<CognitiveSwitchingPattern>,
    }

    /// Classify the overload-streak signal. Note the deliberate
    /// ceiling at `Watch` — see the module docstring.
    pub fn classify_overload_streak(s: &UserBurnoutSignals) -> HealthIndicator {
        // Consecutive snapshots ≥ OVERLOAD_STREAK_WATCH (≈ 2 days
        // at 6h interval) graduates from "busy week" to "worth
        // a glance". Below that, no signal.
        match s.overload_streak_days {
            n if n >= OVERLOAD_STREAK_WATCH => HealthIndicator::Watch,
            _ => HealthIndicator::Good,
        }
    }

    /// Classify the stalled-assigned signal. Same `Watch` ceiling.
    pub fn classify_stalled(s: &UserBurnoutSignals) -> HealthIndicator {
        match s.stalled_assigned_max_days {
            d if d >= STALLED_WATCH_DAYS => HealthIndicator::Watch,
            _ => HealthIndicator::Good,
        }
    }

    /// Classify a drift trend. Returns `Some(direction)` when the
    /// signal exists, `None` when there's no drift trend at all
    /// (insufficient data). Note this returns the direction, not
    /// a `HealthIndicator` — the panel renders drift as a neutral
    /// directional fact, not as a "good / watch" chip. There is
    /// no version of "drift up" that gets a warning palette.
    pub fn classify_drift(drift: &EstimationDriftTrend) -> DriftDirection {
        // Already pre-classified at storage time; this function
        // exists so future logic (e.g., considering both halves'
        // sample sizes) can centralise here.
        drift.direction
    }

    /// Build the natural-language self-reflection prompt for the
    /// user's dashboard. Foregrounds whichever signals are at
    /// `Watch`; if all are `Good`, returns a brief encouraging
    /// note rather than alarmism by default.
    ///
    /// The text is pure data — no behaviour change in the app
    /// flows from it. The whole point of the API is that this
    /// function is a pure function over the signals.
    ///
    /// 0.11.0: drift and switching are deliberately *not* added
    /// to this summary. They are not warnings; they are facts.
    /// The summary line is for warnings ("here's what to glance
    /// at"); the pattern facts get their own distinct chips so
    /// they're visually separated from "this is something to
    /// notice".
    ///
    /// Descriptor for the prompt; `-web` renders it via
    /// `peisear_i18n::Locale::render` (`I18N-002`; was `-> String`
    /// before this handoff). The pre-`I18N-002` version built this by
    /// joining 0–2 independent clauses with `"; "` at runtime — string
    /// concatenation, which `RFC 006` requirement 7 and the
    /// `I18N-001` handoff both prohibit precisely because a guard
    /// cannot see through it. The four reachable
    /// `(overload_watch, stalled_watch)` combinations are enumerated
    /// as four distinct keys instead — see
    /// `MessageKey::BurnoutSummaryOverloadOnly`'s doc comment in
    /// `peisear-i18n` for the replacement pattern.
    pub fn summarize(signals: &UserBurnoutSignals) -> MessageKey {
        let overload = classify_overload_streak(signals);
        let stalled = classify_stalled(signals);
        let overload_watch = matches!(overload, HealthIndicator::Watch);
        let stalled_watch = matches!(stalled, HealthIndicator::Watch);

        match (overload_watch, stalled_watch) {
            (false, false) => MessageKey::BurnoutSummarySteady,
            (true, false) => MessageKey::BurnoutSummaryOverloadOnly {
                days: signals.overload_streak_days,
            },
            (false, true) => MessageKey::BurnoutSummaryStalledOnly {
                days: signals.stalled_assigned_max_days,
            },
            (true, true) => MessageKey::BurnoutSummaryBoth {
                overload_days: signals.overload_streak_days,
                stalled_days: signals.stalled_assigned_max_days,
            },
        }
    }
}

/// Notification subsystem types.
///
/// Two concerns sit here:
///
/// 1. **Vocabulary** — the strings that identify notification
///    kinds, channels, and severity. Kept as a small set of
///    constants so all code references the same spellings and
///    typos surface as compile errors.
///
/// 2. **Domain types** — `Notification`, `Preference`,
///    `Severity`, `Channel`. The storage layer maps rows into
///    these; the dispatch layer consumes them; the web layer
///    renders them.
///
/// ## Design posture
///
/// V2.1 §1.4 says warnings should reach the user. The
/// notifications subsystem realises that by piping
/// `user_burnout` and `project_health` state transitions through
/// a structured pipeline. The posture inherits the rest of the
/// project: signals are *informational*, not evaluative; user
/// has full control over delivery; "all silent" is a respected
/// preference, not a sign of evasion.
///
/// ## Edge-triggered + cooldown
///
/// Notifications are produced when a tracked signal **transitions**
/// (e.g. burnout overload streak crosses from below the watch
/// threshold to at-or-above). The same transition does not
/// re-fire while it persists. A 24-hour cooldown on
/// `(user_id, kind)` further suppresses noisy ping-ponging
/// (state flapping around the threshold during a single day).
/// The cooldown is implemented at dispatch time via a query
/// against the `notifications` audit table; see
/// `peisear-web::notifications::dispatch`.
pub mod notifications {
    use serde::{Deserialize, Serialize};

    /// Severity drives UI palette and the per-kind `min_severity`
    /// preference filter. The set is intentionally small —
    /// "info" and "watch" — because more granularity invites
    /// gaming-the-classifier behaviour we'd rather avoid.
    /// The same `HealthIndicator::Watch` ceiling that other
    /// surfaces honour applies here: `Concern` does not exist.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Severity {
        Info,
        Watch,
    }

    impl Severity {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::Info => "info",
                Self::Watch => "watch",
            }
        }

        /// Parse from the storage-layer string. Unknown values
        /// default to `Info` rather than failing — a future
        /// downgrade that doesn't recognise a higher severity
        /// renders it as informational, which is safer than
        /// silently dropping the row.
        pub fn from_storage_str(s: &str) -> Self {
            match s {
                "watch" => Self::Watch,
                _ => Self::Info,
            }
        }

        /// Used in the preference filter: `min_severity = watch`
        /// suppresses `info` notifications.
        pub fn meets_minimum(self, minimum: Severity) -> bool {
            match (self, minimum) {
                (Self::Watch, _) => true,
                (Self::Info, Self::Info) => true,
                (Self::Info, Self::Watch) => false,
            }
        }
    }

    /// Channel identifiers. String-typed so the storage layer
    /// can store a comma-separated list without an enum<->string
    /// dance, but a small const set so callers can reference
    /// known values.
    pub mod channel {
        pub const IN_APP: &str = "in_app";
        pub const EMAIL: &str = "email";
        pub const WEBHOOK: &str = "webhook";

        /// All channels known to this build, as a `&[&str]`
        /// slice for iteration in handlers / forms. The order
        /// determines how channels are rendered in the
        /// preferences UI (left to right: most recommended
        /// first).
        pub const ALL_CHANNELS: &[&str] = &[IN_APP, EMAIL, WEBHOOK];

        /// Pretty name for UI labels.
        pub fn human_name(id: &str) -> &str {
            match id {
                IN_APP => "In-app",
                EMAIL => "Email",
                WEBHOOK => "Webhook",
                _ => id,
            }
        }

        /// All channels known to this build. Used by the
        /// preferences page to render the full toggle list.
        pub fn all() -> &'static [&'static str] {
            ALL_CHANNELS
        }
    }

    /// Notification kinds. Free-form strings in the database
    /// (so new kinds don't require a migration) but the constants
    /// here are the canonical spellings code uses.
    pub mod kind {
        /// Sentinel row in `notification_preferences` that
        /// records whether the user has been prompted for the
        /// first-login email opt-in. Not a real notification
        /// kind — never appears in the `notifications` table.
        pub const GLOBAL: &str = "_global";

        pub const BURNOUT_OVERLOAD: &str = "burnout_overload";
        pub const BURNOUT_STALLED: &str = "burnout_stalled";
        pub const PROJECT_TREND_DECLINE: &str = "project_trend_decline";

        /// Pretty label for the preferences UI. Unknown kinds
        /// render with their raw id.
        pub fn human_name(k: &str) -> &str {
            match k {
                BURNOUT_OVERLOAD => "Sustained over-capacity streak",
                BURNOUT_STALLED => "Long-stalled assigned work",
                PROJECT_TREND_DECLINE => "Project health decline",
                _ => k,
            }
        }

        /// Canonical kinds shown on the preferences page (in
        /// this order). `GLOBAL` is intentionally absent.
        pub fn all_user_facing() -> &'static [&'static str] {
            &[BURNOUT_OVERLOAD, BURNOUT_STALLED, PROJECT_TREND_DECLINE]
        }
    }

    /// One persisted notification, hydrated from storage.
    /// Visible to the user via the in-app inbox at
    /// `/notifications`; also serves as the audit log for what
    /// was dispatched (rows always exist; the
    /// `dispatched_via` field records which channels actually
    /// delivered).
    #[derive(Debug, Clone)]
    pub struct Notification {
        pub id: String,
        pub user_id: String,
        pub kind: String,
        pub severity: Severity,
        pub title: String,
        pub body: String,
        /// Free-form JSON payload. Application decodes per-kind.
        pub payload_json: Option<String>,
        pub created_at: chrono::DateTime<chrono::Utc>,
        pub read_at: Option<chrono::DateTime<chrono::Utc>>,
        /// Channel ids that successfully delivered.
        pub dispatched_via: Vec<String>,
    }

    /// One per-user, per-kind preference row. Absent rows fall
    /// back to [`DEFAULT_PREFERENCES`].
    #[derive(Debug, Clone)]
    pub struct Preference {
        pub user_id: String,
        pub kind: String,
        /// Channel ids the user wants. Empty = silent.
        pub channels: Vec<String>,
        pub min_severity: Severity,
    }

    /// Effective preferences (storage row OR fallback default).
    /// What dispatch consults to decide what to do with a fresh
    /// notification.
    pub struct EffectivePreference<'a> {
        pub channels: &'a [&'a str],
        pub min_severity: Severity,
    }

    /// System default: in-app delivery for all kinds, all
    /// severities. Smart-defaults posture from design
    /// discussion (Q3=A): the first-time user has working
    /// notifications without configuring anything. They opt in
    /// to email/webhook explicitly.
    pub const DEFAULT_CHANNELS: &[&str] = &[channel::IN_APP];
    pub const DEFAULT_MIN_SEVERITY: Severity = Severity::Info;

    /// Edge-triggered: a notification fires only when the
    /// underlying signal transitions across the threshold.
    /// Concrete transitions today:
    ///
    /// - `burnout_overload`: overload_streak_days transitions
    ///   from < OVERLOAD_STREAK_WATCH to ≥ OVERLOAD_STREAK_WATCH
    /// - `burnout_stalled`: stalled_assigned_max_days
    ///   transitions from < STALLED_WATCH_DAYS to
    ///   ≥ STALLED_WATCH_DAYS
    /// - `project_trend_decline`: composite_score median over
    ///   the past 7 days drops by ≥ 5 points compared to the
    ///   prior 7 days
    ///
    /// Returns true if the transition warrants a notification.
    /// Callers compare prior and current state through this.
    pub fn is_edge_into_watch_burnout_overload(
        prior_streak_days: i64,
        current_streak_days: i64,
    ) -> bool {
        let threshold = crate::user_burnout::OVERLOAD_STREAK_WATCH;
        prior_streak_days < threshold && current_streak_days >= threshold
    }

    pub fn is_edge_into_watch_burnout_stalled(prior_max_days: i64, current_max_days: i64) -> bool {
        let threshold = crate::user_burnout::STALLED_WATCH_DAYS;
        prior_max_days < threshold && current_max_days >= threshold
    }

    /// Cooldown window: the same `(user_id, kind)` will not
    /// fire twice within this many hours, regardless of
    /// edge-triggering. A safety net against threshold
    /// flapping; the bulk of suppression is done by the
    /// edge-trigger logic above.
    pub const COOLDOWN_HOURS: i64 = 24;
}

/// Team / membership / role types (0.14.0).
///
/// Phase 1 ships flat teams; sub-teams (parent_team_id) are
/// reserved for Phase 2. See ROADMAP "Privacy & access control
/// evolution" for the privacy decisions deliberately left for
/// later.
///
/// ## Role semantics
///
/// - `Admin`: political role — manage members, edit team
///   settings, move projects in/out of the team.
/// - `Member`: full project participation in team-owned projects
///   (create / edit issues, be assigned).
/// - `Viewer`: read-only on team projects.
///
/// Per V2.1 §2.5, none of these roles read other users'
/// individual signals (burnout panel, personal dashboard).
/// Admin is a managerial role, not a surveilling one.
pub mod teams {
    use peisear_i18n::TeamRoleLabel;
    use serde::{Deserialize, Serialize};

    /// Per-team role. String-typed in storage; this enum is the
    /// canonical Rust representation.
    ///
    /// Future fixed roles (e.g. `Billing`, `SecurityManager`)
    /// can be added to this enum without breaking storage —
    /// the underlying TEXT column accepts any value the CHECK
    /// constraint allows. Custom (per-team named) roles would
    /// require a `team_roles` table; deferred to Phase 2.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum TeamRole {
        Admin,
        Member,
        Viewer,
    }

    impl TeamRole {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::Admin => "admin",
                Self::Member => "member",
                Self::Viewer => "viewer",
            }
        }

        /// The word this role renders as, via `peisear-i18n`'s message
        /// table. `I18N-005c` §4's "parameterise" instruction absorbs
        /// this the same way `I18N-005b` absorbed
        /// `IssueStatus::label()`/`Priority::label()` — see those
        /// methods' doc comments in this crate.
        pub fn to_i18n_label(self) -> TeamRoleLabel {
            match self {
                Self::Admin => TeamRoleLabel::Admin,
                Self::Member => TeamRoleLabel::Member,
                Self::Viewer => TeamRoleLabel::Viewer,
            }
        }

        /// Parse from storage. Returns `None` for unknown
        /// values — the migration's CHECK should prevent these,
        /// but a future role addition might leave older code
        /// reading new values, and we want a clear "unknown"
        /// path rather than panicking.
        pub fn from_storage_str(s: &str) -> Option<Self> {
            match s {
                "admin" => Some(Self::Admin),
                "member" => Some(Self::Member),
                "viewer" => Some(Self::Viewer),
                _ => None,
            }
        }

        /// Whether this role can write (create / edit issues,
        /// be assigned). Viewer is read-only; member and admin
        /// can write.
        pub fn can_write(self) -> bool {
            matches!(self, Self::Admin | Self::Member)
        }

        /// Whether this role can manage the team itself
        /// (membership, settings). Admin only.
        pub fn can_manage_team(self) -> bool {
            matches!(self, Self::Admin)
        }
    }

    /// One team. Renamable; the `slug` (URL identifier) is
    /// fixed at create time.
    #[derive(Debug, Clone)]
    pub struct Team {
        pub id: String,
        pub name: String,
        pub slug: String,
        pub description: Option<String>,
        pub created_at: chrono::DateTime<chrono::Utc>,
        /// Last mutation timestamp. Populated by the
        /// `teams_updated_at` trigger from migration 0014. Used
        /// by the optimistic-lock contract
        /// (peisear-feature-spec-v2.1 §21.4): handlers re-read
        /// this and compare against the form's
        /// `client_updated_at` before applying a write.
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    /// One row of `team_memberships`. Joined-on-demand with
    /// `users` for member listings.
    #[derive(Debug, Clone)]
    pub struct TeamMembership {
        pub team_id: String,
        pub user_id: String,
        pub role: TeamRole,
        pub joined_at: chrono::DateTime<chrono::Utc>,
        /// Last mutation timestamp (from migration 0014's
        /// trigger). The membership row mutates on role
        /// changes; this is the lock value for those.
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    /// Maximum slug length, enforced by the migration's CHECK.
    /// Exposed here so handler-level validation can refuse a
    /// too-long slug before hitting the database.
    pub const SLUG_MAX_LEN: usize = 64;

    /// Generate a URL slug from a free-form display name.
    /// Lowercases, replaces non-alphanumeric runs with single
    /// hyphens, trims leading/trailing hyphens, and truncates
    /// to `SLUG_MAX_LEN`.
    ///
    /// Returns the empty string if the name has no
    /// alphanumeric characters at all (caller should
    /// reject — slugs cannot be empty per the schema CHECK).
    pub fn slugify(name: &str) -> String {
        let mut out = String::with_capacity(name.len());
        let mut last_was_hyphen = true; // suppress leading hyphens
        for ch in name.chars().flat_map(|c| c.to_lowercase()) {
            if ch.is_ascii_alphanumeric() {
                out.push(ch);
                last_was_hyphen = false;
            } else if !last_was_hyphen {
                out.push('-');
                last_was_hyphen = true;
            }
        }
        // Trim trailing hyphen.
        while out.ends_with('-') {
            out.pop();
        }
        if out.len() > SLUG_MAX_LEN {
            out.truncate(SLUG_MAX_LEN);
            // Avoid trailing hyphen after truncation.
            while out.ends_with('-') {
                out.pop();
            }
        }
        out
    }
}

/// Sprint: a team-scoped, time-boxed unit of planning (0.15.0).
///
/// ## Posture
///
/// peisear's sprint model is informational, not evaluative.
/// V2.1 §0.2 (the non-evaluative stance) shapes the surface:
///
/// - We compute "completed work this period" but don't call it
///   `velocity`. The Jira-popularised term carries
///   "performance" connotations we don't want.
/// - We display burndown as a 2-line chart (cumulative
///   completed vs cumulative committed) without an "ideal"
///   line, without a predicted-finish line, and without a
///   completion-percentage readout. The chart is descriptive,
///   not prescriptive.
/// - "Carried over" issues (in flight at sprint end) are
///   reported as a fact, not a failure. They're shown next to
///   the completed bar so the user sees the full picture.
///
/// ## Lifecycle
///
/// `planned` → `active` (admin starts) → `completed` (admin
/// marks done). All transitions are explicit admin actions —
/// no time-based auto-promotion. The "click to start the
/// sprint" event is the signal we want a person to make
/// deliberately, not the calendar.
pub mod sprints {
    use peisear_i18n::SprintStatusLabel;
    use serde::{Deserialize, Serialize};

    /// Sprint lifecycle. Transitions are admin actions; see
    /// the storage layer for guards (e.g. you can only start a
    /// `planned` sprint, only complete an `active` sprint).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SprintStatus {
        Planned,
        Active,
        Completed,
    }

    impl SprintStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::Planned => "planned",
                Self::Active => "active",
                Self::Completed => "completed",
            }
        }

        /// See `TeamRole::to_i18n_label` — same absorption, same
        /// reason.
        pub fn to_i18n_label(self) -> SprintStatusLabel {
            match self {
                Self::Planned => SprintStatusLabel::Planned,
                Self::Active => SprintStatusLabel::Active,
                Self::Completed => SprintStatusLabel::Completed,
            }
        }

        /// Parse from storage. Unknown values default to
        /// `Planned` rather than failing — defensive against
        /// future status additions read by older code.
        pub fn from_storage_str(s: &str) -> Self {
            match s {
                "active" => Self::Active,
                "completed" => Self::Completed,
                _ => Self::Planned,
            }
        }
    }

    /// One sprint row.
    #[derive(Debug, Clone)]
    pub struct Sprint {
        pub id: String,
        pub team_id: String,
        pub name: String,
        pub goal: Option<String>,
        pub starts_on: chrono::NaiveDate,
        pub ends_on: chrono::NaiveDate,
        pub status: SprintStatus,
        pub started_at: Option<chrono::DateTime<chrono::Utc>>,
        pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
        pub created_at: chrono::DateTime<chrono::Utc>,
        /// Last mutation timestamp (from migration 0014's
        /// trigger). Used by the optimistic-lock contract
        /// (peisear-feature-spec-v2.1 §21.4) for sprint edit /
        /// start / complete / delete handlers.
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    /// Aggregate "what does the sprint look like *right now*?"
    /// computed on demand (no caching).
    ///
    /// Field semantics:
    /// - `committed_points` is the sum of `effort` across all
    ///   issues currently linked to the sprint, regardless of
    ///   their current status.
    /// - `completed_points` is the sum of `effort` across the
    ///   subset whose `status = 'done'`. The natural reading
    ///   is: "of what was committed, this much has finished."
    /// - `committed_count` and `completed_count` are the issue
    ///   counts behind the same numbers.
    /// - `carried_over_points` and `carried_over_count` are
    ///   non-zero only for *completed* sprints (`status =
    ///   Completed`); they record what was still in flight at
    ///   sprint completion. For `Planned` and `Active` sprints
    ///   these are 0 (the sprint isn't done yet, so nothing has
    ///   been "carried" anywhere).
    #[derive(Debug, Clone)]
    pub struct SprintSummary {
        pub sprint_id: String,
        pub committed_points: i64,
        pub completed_points: i64,
        pub committed_count: i64,
        pub completed_count: i64,
        pub carried_over_points: i64,
        pub carried_over_count: i64,
    }

    /// One point on the burndown timeline. Cumulative numbers
    /// (so the chart can plot raw values).
    ///
    /// Two lines are drawn together:
    /// - "Committed" rises whenever issues are added to the
    ///   sprint mid-flight (or stays flat).
    /// - "Completed" rises whenever issues transition to
    ///   `done`. Stays flat on quiet days.
    ///
    /// The visual gap between the two lines is the work still
    /// in progress at that moment. We don't draw a third
    /// "ideal" line — that would be prescriptive.
    #[derive(Debug, Clone, Serialize)]
    pub struct BurndownPoint {
        pub day: chrono::NaiveDate,
        pub cumulative_committed: i64,
        pub cumulative_completed: i64,
    }

    /// Number of recent completed sprints whose `completed_points`
    /// median is shown on the per-team velocity chart's
    /// reference line. Five is enough to make a single
    /// outlier sprint not dominate; small enough that "recent"
    /// still means recent.
    pub const VELOCITY_MEDIAN_WINDOW: usize = 5;
}
