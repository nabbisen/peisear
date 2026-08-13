//! The message key. Every user-visible string in peisear is, or will
//! be, a [`MessageKey`] variant plus typed parameters — never a
//! positional `{0}` and never prose composed inline at the call site
//! (`RFC 006` requirement 7; no concatenation, per the handoff).
//!
//! Seeded here with a small, real set of `AppError`/validation
//! messages (`peisear-web/src/error.rs`, `handlers/settings.rs`,
//! `handlers/issues.rs` as of 0.20.0) — the shortest, least ambiguous
//! copy in the system, per the handoff's guidance not to attempt
//! breadth in this unit. Converting the real call sites to use this
//! crate is out of scope here; that is handoff 4e.

/// The entity kinds `check_optimistic_lock` and its callers name in
/// conflict messages. Matches the documented set in
/// `peisear-web/src/error.rs`'s `OptimisticLockConflict` doc comment.
/// A closed, system-controlled vocabulary — not user data — so it is
/// exhaustively enumerable, unlike an issue title or display name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Issue,
    Project,
    Sprint,
    Team,
    CapacityPeriod,
    TeamMembership,
}

impl EntityKind {
    pub fn all() -> [EntityKind; 6] {
        [
            EntityKind::Issue,
            EntityKind::Project,
            EntityKind::Sprint,
            EntityKind::Team,
            EntityKind::CapacityPeriod,
            EntityKind::TeamMembership,
        ]
    }
}

/// A named form field. Closed and system-controlled for the same
/// reason as [`EntityKind`] — the field itself is a fixed property of
/// the form, not something a user typed.
///
/// `I18N-001` seeded this for validation messages
/// (`FieldRequired`/`FieldMustBePositiveInteger`) only; `I18N-005b`
/// widens it to standalone field labels
/// ([`MessageKey::FieldLabel`]) — a table row heading and a
/// validation message naming the same field are the same word
/// ("Effort" either way), so one enum renders both rather than two
/// enums risking the word drifting between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    EffortPoints,
    CapacityPoints,
    CloseDate,
    Title,
    Description,
    Status,
    Priority,
    Assignee,
    Name,
    /// `I18N-005c`: sprint start/end date, reused between the new-sprint
    /// and edit-sprint forms.
    StartDate,
    EndDate,
    /// A sprint's goal field, same two-form reuse as `StartDate`.
    Goal,
    /// A team member's role, reused between the invite form and the
    /// member table's column heading.
    Role,
    /// Reused between the invite-member form and the member table's
    /// column heading.
    Email,
    /// `PLAN-001`: the sprint plan page's backlog filter has no
    /// existing `Project` field to reuse — every prior project-scoped
    /// surface is nested under one project's own URL and never needed
    /// to name "project" as a filter facet the way a team-wide
    /// backlog does.
    Project,
}

impl Field {
    pub fn all() -> [Field; 15] {
        [
            Field::EffortPoints,
            Field::CapacityPoints,
            Field::CloseDate,
            Field::Title,
            Field::Description,
            Field::Status,
            Field::Priority,
            Field::Assignee,
            Field::Name,
            Field::StartDate,
            Field::EndDate,
            Field::Goal,
            Field::Role,
            Field::Email,
            Field::Project,
        ]
    }
}

/// Which project-health indicator a message is about. Mirrors
/// `peisear_core::project_health::IndicatorKind` in shape — six
/// variants, same order — but is a distinct type, deliberately not
/// reused from `peisear-core`.
///
/// `peisear-i18n` is a leaf crate with no workspace dependencies
/// (`I18N-001` §4.1); `peisear-core` depends on it, never the
/// reverse (`I18N-002` §5.1). If this crate imported
/// `peisear_core::IndicatorKind` directly it would create exactly
/// the cycle that design forbids. `peisear-core`'s `summarize` and
/// `human_explanation`/`format_value` construct a value of *this*
/// type from their own `IndicatorKind` at the call site (see
/// `IndicatorKind::to_i18n_label` in `peisear-core`) when building a
/// `MessageKey`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorLabel {
    Throughput,
    Staleness,
    Activity,
    BusFactor,
    LongStale,
    WipCompliance,
}

impl IndicatorLabel {
    pub fn all() -> [IndicatorLabel; 6] {
        [
            IndicatorLabel::Throughput,
            IndicatorLabel::Staleness,
            IndicatorLabel::Activity,
            IndicatorLabel::BusFactor,
            IndicatorLabel::LongStale,
            IndicatorLabel::WipCompliance,
        ]
    }
}

/// A destination `render_back_link` can name — our own copy, never
/// user data. `I18N-005a-review.md` §2: the original
/// `render_back_link(label: impl Into<String>, ...)` let every
/// caller pass a raw string, so "Projects" and "sprints" — the same
/// construct, the same screen family — drifted to different
/// capitalisation without the guard ever seeing either one. Shaped
/// like [`IndicatorLabel`] rather than kept as an open `String`: the
/// rule the review settles is that a `String` parameter carries user
/// data only, and a back-link's own words ("projects", "issues",
/// "sprints") are never that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavSection {
    Projects,
    Issues,
    Sprints,
}

impl NavSection {
    pub fn all() -> [NavSection; 3] {
        [
            NavSection::Projects,
            NavSection::Issues,
            NavSection::Sprints,
        ]
    }
}

/// Which word `peisear_core::IssueStatus` renders as. Mirrors
/// `IssueStatus` in shape (three variants, same order) but is a
/// distinct type, for the same leaf-crate/no-cycle reason
/// [`IndicatorLabel`] is distinct from `IndicatorKind` — see that
/// type's doc comment. `IssueStatus::to_i18n_label` (`peisear-core`,
/// `I18N-005b`) is the conversion at the boundary; the absorbed
/// `IssueStatus::label()` this replaces is documented there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueStatusLabel {
    Open,
    InProgress,
    Done,
}

impl IssueStatusLabel {
    pub fn all() -> [IssueStatusLabel; 3] {
        [
            IssueStatusLabel::Open,
            IssueStatusLabel::InProgress,
            IssueStatusLabel::Done,
        ]
    }
}

/// Which word `peisear_core::Priority` renders as. See
/// [`IssueStatusLabel`]'s doc comment — same shape, same reason,
/// same handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityLabel {
    Low,
    Medium,
    High,
    Urgent,
}

impl PriorityLabel {
    pub fn all() -> [PriorityLabel; 4] {
        [
            PriorityLabel::Low,
            PriorityLabel::Medium,
            PriorityLabel::High,
            PriorityLabel::Urgent,
        ]
    }
}

/// Which word `peisear_core::sprints::SprintStatus` renders as. See
/// [`IssueStatusLabel`]'s doc comment — same shape, same reason.
/// `SprintStatus::to_i18n_label` (`peisear-core`, `I18N-005c`) is the
/// conversion at the boundary, absorbing `SprintStatus::human_name()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SprintStatusLabel {
    Planned,
    Active,
    Completed,
}

impl SprintStatusLabel {
    pub fn all() -> [SprintStatusLabel; 3] {
        [
            SprintStatusLabel::Planned,
            SprintStatusLabel::Active,
            SprintStatusLabel::Completed,
        ]
    }
}

/// Which word `peisear_core::teams::TeamRole` renders as. See
/// [`IssueStatusLabel`]'s doc comment. `TeamRole::to_i18n_label`
/// (`peisear-core`, `I18N-005c`) absorbs `TeamRole::human_name()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamRoleLabel {
    Admin,
    Member,
    Viewer,
}

impl TeamRoleLabel {
    pub fn all() -> [TeamRoleLabel; 3] {
        [
            TeamRoleLabel::Admin,
            TeamRoleLabel::Member,
            TeamRoleLabel::Viewer,
        ]
    }
}

/// Which phrase `peisear_core::user_burnout::DriftDirection` renders
/// as. See [`IssueStatusLabel`]'s doc comment. `DriftDirection::to_i18n_label`
/// (`peisear-core`, `I18N-005d`) absorbs two local `match` statements
/// in `me.rs` — a short chip word and a longer "trending ..." phrase,
/// both keyed off this one enum via two different `MessageKey`s
/// ([`MessageKey::DriftDirectionWord`], the trend phrase composed
/// inline in [`MessageKey::DriftAriaLabel`]'s render arm).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftDirectionLabel {
    Up,
    Down,
    Steady,
}

impl DriftDirectionLabel {
    pub fn all() -> [DriftDirectionLabel; 3] {
        [
            DriftDirectionLabel::Up,
            DriftDirectionLabel::Down,
            DriftDirectionLabel::Steady,
        ]
    }
}

/// Which word `peisear_core::notifications::kind::human_name` (a
/// free function over string constants, not a Rust enum — the
/// storage layer needs a comma-separated string, per that module's
/// own doc comment) renders as. `I18N-005d` absorbs the function;
/// unlike the enum-backed `*Label` types above, the string-id ->
/// label mapping lives in `peisear-web` (the boundary crossing
/// `peisear-core`'s `to_i18n_label()` methods normally do, but
/// `peisear-i18n` cannot depend on `peisear-core` to provide one, and
/// turning `kind`/`channel` into real enums would be an unrelated,
/// broader storage-layer refactor this handoff doesn't touch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationKindLabel {
    BurnoutOverload,
    BurnoutStalled,
    ProjectTrendDecline,
}

impl NotificationKindLabel {
    pub fn all() -> [NotificationKindLabel; 3] {
        [
            NotificationKindLabel::BurnoutOverload,
            NotificationKindLabel::BurnoutStalled,
            NotificationKindLabel::ProjectTrendDecline,
        ]
    }
}

/// See [`NotificationKindLabel`]'s doc comment — same shape, same
/// reason, absorbing `peisear_core::notifications::channel::human_name`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationChannelLabel {
    InApp,
    Email,
    Webhook,
}

impl NotificationChannelLabel {
    pub fn all() -> [NotificationChannelLabel; 3] {
        [
            NotificationChannelLabel::InApp,
            NotificationChannelLabel::Email,
            NotificationChannelLabel::Webhook,
        ]
    }
}

/// Which word `peisear_core::DisplayHealthState::to_i18n_label`
/// renders as. `I18N-006` §3: `glyph()` used to return this as its
/// second tuple element (`(symbol, word)`) alongside the non-language
/// symbol; the symbol stays in `peisear-core`, the word comes here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStateLabel {
    Insufficient,
    Good,
    Watch,
}

impl HealthStateLabel {
    pub fn all() -> [HealthStateLabel; 3] {
        [
            HealthStateLabel::Insufficient,
            HealthStateLabel::Good,
            HealthStateLabel::Watch,
        ]
    }
}

/// Which word `components/issues.rs::render_trend_chip` renders for
/// a non-flat `peisear_core::project_health::Trend`. `Trend::Flat`
/// has no delta and a different sentence shape ("roughly flat" vs.
/// "up/down by N points"), so it is not a third variant here --
/// `MessageKey::TrendLabelFlat`/`TrendAriaFlat` cover it directly
/// (`I18N-007` §2 item 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirectionLabel {
    Up,
    Down,
}

impl TrendDirectionLabel {
    pub fn all() -> [TrendDirectionLabel; 2] {
        [TrendDirectionLabel::Up, TrendDirectionLabel::Down]
    }
}

/// One message this crate can render, in every shipped locale.
///
/// A key enum, not a string constant and not a `HashMap<&str, &str>`:
/// a `match` over this type that omits a variant fails to compile,
/// which is what makes a missing rendering a compile-time error
/// (`RFC 006` requirement 2) rather than a runtime fallback to the
/// key name.
///
/// `Clone`, not `Copy`: `I18N-005b` converts project/issue surfaces
/// where issue titles, project names, and display names are
/// genuinely open user data (`RFC 006` D4), so several variants now
/// carry `String` parameters
/// ([`MessageKey::WorkloadTitle`], [`MessageKey::MoveIssueAriaLabel`],
/// [`MessageKey::SubIssueAriaLabel`], the page-title keys,
/// [`MessageKey::CreatedAt`]/[`MessageKey::UpdatedAt`]).
/// `I18N-005a` removed and re-added this derive once already
/// (`BackToLabel` turned out to be carrying our own copy, not user
/// data — `I18N-005a-review.md` §2); this crossing is the real one
/// `RFC 006` D3 anticipated, kept accurate to the variant set as it
/// stands rather than guarded against in advance.
///
/// `PartialEq`, not `Eq`: `I18N-005d` adds `f64` fields (pace and
/// drift figures — genuinely numeric display data, not enumerable
/// closed vocabulary), and `f64` has no `Eq` impl. No code in this
/// crate or its consumers compares `MessageKey` values for equality
/// today (`Debug` and `Clone` cover every real use); `PartialEq` is
/// kept for parity with the rest of the crate's derive conventions,
/// not because something depends on it.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageKey {
    /// `AppError::Forbidden`'s public message.
    Forbidden,
    /// `AppError::NotFound`'s public message.
    NotFound,
    /// `AppError::Internal`'s public message — no internal detail
    /// leaks into this rendering; that stays in the `tracing::error!`
    /// call at the point the error is constructed.
    InternalError,
    /// `AppError::OptimisticLockConflict`'s public message.
    OptimisticLockConflict {
        entity: EntityKind,
    },
    /// `check_optimistic_lock`'s parse-failure message — entity-
    /// neutral since 0.20.0 (`DEV-001-004-review.md` §1.4).
    LockValueUnreadable,
    /// A named field is required and was left blank.
    FieldRequired {
        field: Field,
    },
    /// A named field must be a positive integer.
    FieldMustBePositiveInteger {
        field: Field,
    },
    /// A named field must be a `YYYY-MM-DD` date. `I18N-005e`: reused
    /// between `sprints.rs`'s start/end date validation (`Field::StartDate`/
    /// `Field::EndDate`, matching `I18N-005c`'s field-label wording exactly).
    FieldMustBeDateFormat {
        field: Field,
    },
    /// The `status` form field didn't parse to a known `IssueStatus`.
    InvalidStatus,
    /// The `priority` form field didn't parse to a known `Priority`.
    InvalidPriority,

    // ---- I18N-004: peisear_core::project_health::IndicatorKind ----
    /// The indicator's own name, e.g. "Throughput", "Bus factor" —
    /// what `IndicatorKind::label()` returned before this handoff
    /// absorbed it. `I18N-002` introduced `IndicatorLabel` for use
    /// as a parameter *inside* `summarize`'s sentences, but left
    /// `label()` itself in `peisear-core` holding the same six
    /// strings with nothing keeping the two in sync (`I18N-002`
    /// review §1.4) — the standalone chip label and the sentence
    /// parameter now share one rendering (`en.rs`'s `indicator_label`
    /// helper), so there is exactly one place these strings live.
    IndicatorName {
        label: IndicatorLabel,
    },

    // ---- I18N-002: peisear_core::project_health::format_value ----
    /// No value to show for this indicator (the raw counts it would
    /// divide by are zero). Shared across every indicator's
    /// zero-denominator case — genuinely the same message, not
    /// several concepts collapsed into one (`I18N-001-review.md`
    /// §2 Q1's distinction).
    IndicatorValueUnavailable,
    IndicatorValueThroughput {
        done: i64,
        total: i64,
    },
    IndicatorValueStaleness {
        days: i64,
    },
    IndicatorValueActivity {
        count: i64,
    },
    /// `active_assignees <= 1`. See `ISSUE-006` finding 2 — the
    /// sibling `IndicatorExplanationBusFactorSolo` message built
    /// from this value is grammatically broken, preserved verbatim
    /// pending that ruling. This value key itself ("solo") is not
    /// the defect; the explanation template that embeds it is.
    IndicatorValueBusFactorSolo,
    IndicatorValueBusFactor {
        pct: i64,
    },
    IndicatorValueLongStale {
        stale: i64,
        in_flight: i64,
    },
    /// `wip_violators == 0`. Never actually reachable from
    /// `human_explanation` (that state classifies as `Good`, which
    /// short-circuits before an explanation is built) — included
    /// for completeness of `format_value`'s own value set, which
    /// `indicator_row` renders regardless of state.
    IndicatorValueWipAllWithin,
    IndicatorValueWipOver {
        count: i64,
    },

    // ---- I18N-002: peisear_core::project_health::Indicator::human_explanation ----
    IndicatorExplanationThroughput {
        done: i64,
        total: i64,
    },
    IndicatorExplanationStaleness {
        days: i64,
    },
    IndicatorExplanationActivity {
        count: i64,
    },
    /// **`I18N-004` fix for `ISSUE-006` finding 2.** Was "solo of
    /// in-flight work is concentrated on one person." — a
    /// percentage-shaped template fed the non-percentage value
    /// "solo". Now its own plainly factual sentence, per
    /// `ISSUE-006-decision.md` §3: no evaluation, no implied fault,
    /// no directive ("consider spreading the load" would be a "you
    /// should" in disguise, which §1.7 prohibits).
    IndicatorExplanationBusFactorSolo,
    IndicatorExplanationBusFactor {
        pct: i64,
    },
    IndicatorExplanationLongStale {
        stale: i64,
        in_flight: i64,
    },
    /// **`I18N-004` fix for `ISSUE-006` finding 3.** Was "N over of
    /// active assignees are over their WIP limit." — doubled and
    /// awkward. Now takes the count as a typed parameter rather than
    /// embedding a pre-formatted "N over" string, which is what
    /// produced the doubling in the first place. Reproduced live
    /// before this fix, per the ruling — see `I18N-004`'s review
    /// request.
    IndicatorExplanationWipCompliance {
        count: i64,
    },

    // ---- I18N-002/004: peisear_core::project_health::summarize ----
    HealthSummaryHealthy,
    /// **`I18N-004`**: the only two reachable shapes since the
    /// `ISSUE-006` finding 1 fix — `summarize` now selects between
    /// these based on the clamped `DisplayHealthState`, which has no
    /// `Concern` variant to select on. What was
    /// `HealthSummaryOneConcern` (removed) and this variant now
    /// render identically ("worth a glance") for a `Concern`-tier
    /// lead indicator, same as a `Watch`-tier one — see
    /// `ISSUE-006-decision.md` §3: "no new wording is needed; the
    /// `Watch` sentences already exist and already read correctly."
    HealthSummaryOneWatch {
        label: IndicatorLabel,
    },
    /// **`I18N-004`**: see [`MessageKey::HealthSummaryOneWatch`]'s
    /// doc comment — what was `HealthSummaryConcernPlusOne` (removed)
    /// now renders through this variant too.
    HealthSummaryTwoWatch {
        first: IndicatorLabel,
        second: IndicatorLabel,
    },

    // ---- I18N-002: peisear_core::user_burnout::summarize ----
    BurnoutSummarySteady,
    /// The source function built this by joining 0–2 independent
    /// clauses with `"; "` at runtime — string concatenation, which
    /// `RFC 006` requirement 7 and the `I18N-001` handoff both
    /// prohibit precisely because a guard cannot see through it. The
    /// four reachable combinations are enumerated as four distinct
    /// keys instead: `BurnoutSummarySteady` (neither), this one
    /// (overload only), `BurnoutSummaryStalledOnly` (stalled only),
    /// `BurnoutSummaryBoth` (both) — the same replacement pattern
    /// `I18N-001` used for `HashMap`-style lookups: one concrete
    /// message per reachable shape, not a template assembled from
    /// parts.
    BurnoutSummaryOverloadOnly {
        days: i64,
    },
    BurnoutSummaryStalledOnly {
        days: i64,
    },
    BurnoutSummaryBoth {
        overload_days: i64,
        stalled_days: i64,
    },

    // ---- I18N-003: peisear_notify::edge notification title/body ----
    /// `edge::detect_burnout_overload_edge`'s notification title.
    NotificationBurnoutOverloadTitle,
    /// `edge::detect_burnout_overload_edge`'s notification body.
    /// Corrected from `/me` to `/today` during relocation — `/me`
    /// was renamed in 0.17.0 (`FR-NAV-002`); the old path still
    /// works via a 308 redirect, but user-facing copy shouldn't send
    /// people through a compatibility redirect (`I18N-003` §4). This
    /// is the one deliberate wording change in this handoff; every
    /// other relocated string is byte-identical to its 0.20.0 source.
    NotificationBurnoutOverloadBody {
        streak_snapshots: i64,
    },
    /// `edge::detect_burnout_stalled_edge`'s notification title.
    NotificationBurnoutStalledTitle,
    /// `edge::detect_burnout_stalled_edge`'s notification body. Same
    /// `/me` → `/today` correction as
    /// [`MessageKey::NotificationBurnoutOverloadBody`].
    NotificationBurnoutStalledBody {
        stalled_days: i64,
    },

    // ---- I18N-005a: components/{layout,breadcrumb,error_page} ----
    /// The application's name, standing alone (the navbar brand
    /// link). Also spelled out in full inside composed page titles
    /// such as [`MessageKey::ErrorPageTitle`] — those are each a
    /// single complete message in their own right, not this key
    /// concatenated with a page name at the call site (`RFC 006`
    /// requirement 7).
    AppBrandName,
    /// Navbar bell `aria-label` when there are no unread
    /// notifications. See [`MessageKey::NavBellLabelUnread`] for the
    /// other reachable shape — kept as two keys, not one key with a
    /// zero-count branch, matching the
    /// [`MessageKey::BurnoutSummarySteady`] precedent.
    NavBellLabelNone,
    NavBellLabelUnread {
        count: i64,
    },
    /// The bell badge's own visible number, separate from its
    /// `aria-label`. Capped display ("99+") above 99 is a rendering
    /// concern, not a second message — same shape as
    /// [`MessageKey::IndicatorValueActivity`], a bare count with no
    /// surrounding words.
    NavBellCount {
        count: i64,
    },
    NavSearchFormLabel,
    NavSearchPlaceholder,
    NavSearchQueryLabel,
    NavSearchSuggestionsLabel,
    /// Standalone nav-link words, each reused at more than one call
    /// site (navbar dropdown item; `NavLinkToday` is also
    /// `render_breadcrumb`'s hard-coded leading entry). Flat
    /// variants rather than an enum-parameterised one: unlike
    /// [`EntityKind`]/[`Field`]/[`IndicatorLabel`], no larger
    /// sentence template ever embeds "which destination" as a
    /// parameter — each word only ever stands alone.
    NavLinkToday,
    NavLinkTeams,
    NavLinkInbox,
    NavLinkSettings,
    NavSignOut,
    /// `render_breadcrumb`'s wrapping `<nav>` `aria-label`.
    BreadcrumbNavLabel,
    /// `render_back_link`'s "Back to {section}" — used for both the
    /// link's `aria-label` and its visible text, so the two can
    /// never drift (`I18N-005a` replaces the source's direct
    /// literal-plus-interpolation `"Back to " {label}` in the view
    /// with a render of this key, matching the fixed prefix in the
    /// `aria-label`, which already went through `format!`).
    /// [`NavSection`], not a `String`: see that type's doc comment —
    /// `I18N-005a-review.md` §2 found the original `String` version
    /// let two call sites' copy ("Projects", "sprints") drift to
    /// different capitalisation outside the guard's reach.
    BackToSection {
        section: NavSection,
    },
    /// `ErrorPage`'s `<title>`. One complete string, not
    /// [`MessageKey::AppBrandName`] concatenated with "Error" at the
    /// call site.
    ErrorPageTitle,
    ErrorPageGoHomeLink,

    // ---- I18N-005b: components/{issues,projects} ----
    /// `IssueStatus`'s word ("Open"/"In Progress"/"Done"), absorbing
    /// `IssueStatus::label()` — see that method's doc comment
    /// (`peisear-core`).
    IssueStatusName {
        label: IssueStatusLabel,
    },
    /// `Priority`'s word, absorbing `Priority::label()`. See
    /// [`MessageKey::IssueStatusName`].
    PriorityName {
        label: PriorityLabel,
    },
    /// A form field's name, standing alone — a table column heading,
    /// a `<label>` body, or an attribute `title`. Reuses [`Field`],
    /// widened by this handoff from validation-only to this use too;
    /// see that type's doc comment for why one enum renders both
    /// rather than risking the same word (e.g. "Effort") drifting
    /// between a validation sentence and a field label.
    FieldLabel {
        field: Field,
    },
    /// "Projects" — the section name, reused identically as a
    /// breadcrumb link and as the `/projects` list page's `<h1>`.
    /// Distinct from [`MessageKey::BackToSection`]'s lowercase
    /// "projects": same destination, different grammatical context,
    /// not coupled (`I18N-005a-review.md` §10's "worth watching"
    /// note — this is the second instance, now on record rather than
    /// rediscovered).
    ProjectsSectionName,
    /// The board/list view-toggle buttons on the project detail page.
    ViewToggleBoard,
    ViewToggleList,
    /// "Edit", standing alone — a button and a breadcrumb node reuse
    /// this identically. Distinct from
    /// [`MessageKey::EditProjectHeading`]'s "Edit project", a
    /// different, longer phrase.
    EditWord,
    CancelButton,
    SaveButton,
    DeleteButton,
    /// "—" — the shared "nothing set" placeholder for an optional
    /// select or a table cell with no value (effort, assignee).
    /// Eight identical occurrences across `issues.rs` folded into
    /// one key rather than eight copies of the same glyph.
    NoValuePlaceholder,
    /// "story points", the hint text beside the Effort field.
    StoryPointsHint,
    /// "{points} pt" — a bare quantity with a unit suffix, the same
    /// shape as [`MessageKey::IndicatorValueWipOver`]. Covers both
    /// workload chips (a user's total in-flight points) and an
    /// issue's own effort badge — same unit, same word, same
    /// template; nothing about either use case changes what "pt"
    /// means.
    PointsValue {
        points: i64,
    },
    /// `HealthStrip`'s early-return when a project has no issues yet.
    /// Not part of I18N-002/I18N-004's already-converted health
    /// explanation/summary/indicator-label surface — this specific
    /// string was never touched by either.
    HealthEmptyMessage,
    /// `HealthStrip`'s wrapping `<section>` `aria-label`.
    ProjectHealthSectionLabel,
    HealthHeading,
    /// `HealthStrip`'s `<details>` `<summary>` text.
    IndicatorsSummaryLabel,
    WorkloadHeading,
    /// The workload strip's link to `/settings` for setting one's own
    /// capacity.
    WorkloadSetCapacityLink,
    /// `WorkloadStrip`'s per-chip `title` attribute. `display_name`
    /// is user data (per `RFC 006` D4); `in_flight_issues` is a
    /// count. Replaces a `format!` that composed our copy directly
    /// with a user's display name at the call site.
    WorkloadTitle {
        display_name: String,
        in_flight_issues: i64,
    },
    /// `WorkloadHint`'s inline label, distinct from
    /// [`MessageKey::WorkloadHeading`] by trailing punctuation and
    /// context (a compact inline hint under a form, not a section
    /// heading).
    WorkloadHintLabel,
    /// `BoardView`'s empty-column hint.
    EmptyBoardHint,
    /// The keyboard status-change button's `aria-label`
    /// (`FR-DM-002`). `issue_title` is user data; `target` is the
    /// destination status, a closed set — kept typed rather than a
    /// pre-rendered `String` so the word itself has exactly one
    /// source (`IssueStatusName`'s rendering), same reasoning as
    /// [`MessageKey::SubIssueAriaLabel`].
    MoveIssueAriaLabel {
        issue_title: String,
        target: IssueStatusLabel,
    },
    /// The filter/sort toolbar's `aria-label`.
    FilterSortAriaLabel,
    AllStatusesOption,
    AnyoneOption,
    UnassignedOption,
    SortByFieldLabel,
    SortDefaultOption,
    SortRecentlyCreatedOption,
    SortRecentlyUpdatedOption,
    ApplyButton,
    /// "Reset"'s `aria-label`, explaining what the link does beyond
    /// what its short visible text says.
    ResetFilterAriaLabel,
    ResetLink,
    /// The list view's "Updated" table column heading. Distinct from
    /// [`MessageKey::UpdatedAt`]'s "Updated {value}" — a bare heading
    /// noun versus a sentence fragment naming a value, different
    /// grammatical shapes needing separate keys.
    UpdatedColumnHeading,
    EmptyIssueListMessage,
    /// The board card's and issue-view's effort badge tooltip.
    /// Distinct from [`MessageKey::FieldLabel`]`(Field::EffortPoints)`
    /// ("Effort" alone) — this is the longer phrase "Effort
    /// estimate", not a reuse of the shorter field-label word.
    EffortEstimateTooltip,
    /// `project.name` is user data. Composes the same way
    /// [`MessageKey::ErrorPageTitle`]'s sibling keys do — one
    /// complete title string per page, not a brand-name key
    /// concatenated with a page name at the call site.
    ProjectDetailPageTitle {
        project_name: String,
    },
    /// See [`MessageKey::ProjectDetailPageTitle`].
    IssueNewPageTitle {
        project_name: String,
    },
    /// "New issue", reused identically as a button, a breadcrumb
    /// node, and an `<h1>`.
    NewIssueLabel,
    NewIssueTitlePlaceholder,
    NewIssueDescriptionPlaceholder,
    CreateIssueButton,
    SubIssueNewPageTitle {
        parent_title: String,
    },
    /// "New sub-issue", reused as a breadcrumb node and an `<h1>`.
    NewSubIssueLabel,
    SubIssueNewPageIntro,
    NewSubIssueTitlePlaceholder,
    NewSubIssueDescriptionPlaceholder,
    CreateSubIssueButton,
    IssueDetailPageTitle {
        issue_title: String,
        project_name: String,
    },
    /// "Sub-issues", reused as a section `aria-label` and its
    /// heading.
    SubIssuesLabel,
    /// "+ Add sub-issue" — the leading "+" is part of the link's
    /// accessible name (not a decorative glyph behind
    /// `aria-hidden`, unlike `I18N-005a`'s "●"/"←"), so it stays part
    /// of this one string rather than being split out.
    AddSubIssueLink,
    SubIssuesEmptyMessage,
    /// A sub-issue list row's `aria-label`. `title` is user data;
    /// `status` is the closed-set status word, kept typed for the
    /// same reason as [`MessageKey::MoveIssueAriaLabel`].
    SubIssueAriaLabel {
        title: String,
        status: IssueStatusLabel,
    },
    SprintAssignmentLabel,
    SprintFieldLabel,
    SprintSelectAriaLabel,
    NoSprintOption,
    SprintAssignmentHelperText,
    /// The read-only status segmented control's `aria-label`
    /// (`FR-ISS-005`).
    IssueStatusAriaLabel,
    NoDescriptionProvided,
    /// "Created {formatted}". `formatted` is an already-formatted
    /// timestamp string (data, not copy) — replaces a source that
    /// composed the literal prefix and the value as two adjacent
    /// text nodes directly in the view, the same concatenation shape
    /// `I18N-005a`'s `BackToLabel` correction closed for back-links.
    CreatedAt {
        formatted: String,
    },
    /// "Updated {formatted}". See [`MessageKey::CreatedAt`]; also
    /// reused by `ProjectCard`.
    UpdatedAt {
        formatted: String,
    },
    ProjectsListPageTitle,
    ProjectsSubheading,
    /// "New project", reused as a button and an `<h1>`.
    NewProjectLabel,
    ProjectsEmptyMessage,
    CreateFirstProjectButton,
    /// `ProjectCard`'s empty-description placeholder. Distinct
    /// wording from [`MessageKey::NoDescriptionProvided`] — both
    /// existed independently before conversion; converting
    /// byte-identically preserves the mismatch rather than
    /// silently reconciling it (not this handoff's call to make).
    NoDescriptionShort,
    ProjectNewPageTitle,
    /// `ProjectNewPage`'s breadcrumb terminal node.
    NewBreadcrumbWord,
    ProjectNamePlaceholder,
    ProjectDescriptionPlaceholder,
    TeamFieldLabel,
    OptionalHint,
    PersonalNoTeamOption,
    TeamHelperText,
    /// `ProjectNewPage`'s submit button. Distinct wording from
    /// [`MessageKey::CreateFirstProjectButton`]'s "Create your first
    /// project" — the empty-state CTA and the form's own submit
    /// button are separately authored strings, converted
    /// byte-identically rather than reconciled.
    CreateProjectButton,
    ProjectEditPageTitle {
        project_name: String,
    },
    EditProjectHeading,
    DeleteProjectHeading,
    DeleteProjectWarning,
    /// `handlers/issues.rs`'s post-delete flash banner, reached via
    /// a `?flash=Issue+deleted` redirect query parameter.
    IssueDeletedFlash,
    /// `handlers/projects.rs`'s post-delete flash banner.
    ProjectDeletedFlash,

    // ---- I18N-005c: components/{sprints,teams} ----
    /// `SprintStatus`'s word ("Planned"/"Active"/"Completed"),
    /// absorbing `SprintStatus::human_name()` — see that method's doc
    /// comment (`peisear-core`).
    SprintStatusName {
        label: SprintStatusLabel,
    },
    /// `TeamRole`'s word, absorbing `TeamRole::human_name()`. See
    /// [`MessageKey::SprintStatusName`].
    TeamRoleName {
        label: TeamRoleLabel,
    },
    NewSprintLink,
    SprintsPageTitle {
        team_name: String,
    },
    /// "Sprints", reused as a breadcrumb node, an `<h1>`, and the
    /// team-detail page's link into the sprints list.
    SprintsSectionName,
    SprintsListAriaLabel,
    /// A sprint card's summary line for a completed sprint. The
    /// sibling of [`MessageKey::CaptionWordCarriedOver`]'s lowercase,
    /// mid-sentence use — this is the whole-sentence context
    /// `FLAGGED ITEM #2` (the survey) found first. All three
    /// `SprintCardSummary*` variants are separate keys, one per
    /// reachable `SprintStatus`, rather than one key with a status
    /// parameter — the three sentences share no common template, only
    /// a family resemblance.
    SprintCardSummaryCompleted {
        completed_points: i64,
        committed_points: i64,
        carried_over_points: i64,
    },
    SprintCardSummaryActive {
        completed_points: i64,
        committed_points: i64,
        in_flight_points: i64,
    },
    SprintCardSummaryPlanned {
        committed_points: i64,
        committed_count: i64,
    },
    /// A sprint card's `aria-label`. `name` and `dates` are data
    /// (user text, a formatted range); `status` is the typed
    /// [`SprintStatusLabel`] rather than a pre-rendered `String` —
    /// I18N-005c-review §3's correction. The original shape passed
    /// `status`/`summary` as already-rendered `String`s composed
    /// from [`MessageKey::SprintStatusName`]/the `SprintCardSummary*`
    /// keys above, reasoning that wasn't the `BackToLabel` mistake
    /// (every fragment did come from the table). It was still wrong:
    /// runtime string concatenation assembles the sentence outside
    /// any single render arm, so no guard check ever sees the whole
    /// — the assembled-sentence half of the concatenation
    /// prohibition, not the raw-literal half. The four count fields
    /// below are the same typed data `SprintCardSummary*` render
    /// from; this key's own render arm composes the summary clause
    /// once, matching on `status`, rather than receiving someone
    /// else's pre-rendered output.
    SprintCardAriaLabel {
        name: String,
        status: SprintStatusLabel,
        dates: String,
        completed_points: i64,
        committed_points: i64,
        carried_over_points: i64,
        committed_count: i64,
    },
    /// The velocity bar chart's per-bar-group `aria-label`. `name` is
    /// user data; the two counts are data.
    VelocityBarAriaLabel {
        name: String,
        completed_points: i64,
        carried_over_points: i64,
    },
    /// Two complete sentences, not a shared prefix plus a conditional
    /// continuation — matches the `BurnoutSummary` precedent of one
    /// key per reachable shape rather than splicing fragments in the
    /// view.
    SprintsEmptyMessageAdmin,
    SprintsEmptyMessageNonAdmin,
    SprintsOptionalNote,
    CompletedWorkHeading,
    RecentCompletedSprintsAriaLabel,
    /// Velocity chart caption, split at its existing `<strong>`
    /// boundaries — three plain-text keys plus two shared
    /// emphasised-word keys, rendered in the same relative positions
    /// the source's literal text nodes held. Not concatenation of a
    /// template with a value (rule 1's concern): every piece is our
    /// own copy, split only where the view already split it for
    /// visual emphasis.
    VelocityCaptionLead,
    /// "completed", lowercase and mid-sentence — shared between the
    /// velocity and burndown captions, which both emphasise this
    /// exact word. Distinct from
    /// [`MessageKey::CompletedStatLabel`]'s standalone, title-case
    /// use (same word, different grammatical context, same
    /// non-coupling `I18N-005a-review.md` §10 already noted for
    /// "Projects"/"projects").
    CaptionWordCompleted,
    VelocityCaptionMiddle,
    /// "carried over", lowercase and mid-sentence. Distinct from
    /// [`MessageKey::CarriedOverHeading`]'s standalone, title-case
    /// use.
    CaptionWordCarriedOver,
    VelocityCaptionTail,
    BarChartAriaLabel,
    MedianLabel {
        median: i64,
    },
    /// "New sprint", reused as the page `<title>` and the `<h1>`.
    NewSprintLabel,
    SprintNamePlaceholder,
    GoalFieldPlaceholder,
    /// "The sprint will be created in " — split at the source's
    /// `<strong>` boundary around "planned", same shape as the
    /// velocity/burndown captions.
    SprintPlannedNoticeLead,
    /// "planned", lowercase and mid-sentence. Deliberately **not**
    /// [`MessageKey::SprintStatusName`]`(SprintStatusLabel::Planned)`
    /// reused — that renders "Planned" (title case, standalone badge
    /// use); this is the same non-coupling as
    /// [`MessageKey::CaptionWordCompleted`].
    CaptionWordPlanned,
    SprintPlannedNoticeTail,
    CreateSprintButton,
    /// "Start sprint", identical text reused for both the button's
    /// `aria-label` and its visible label.
    StartSprintLabel,
    /// See [`MessageKey::StartSprintLabel`].
    CompleteSprintLabel,
    /// "Goal: " — deliberately **not** combined with the goal value
    /// into one `"Goal: {goal}"` key the way
    /// [`MessageKey::CreatedAt`]/[`MessageKey::UpdatedAt`] combine
    /// their prefix and value: the source wraps only this prefix in
    /// its own `opacity-60` span, styled distinctly from the value
    /// that follows it — combining would either lose that styling
    /// split or extend it to the value too, a visible change this
    /// handoff isn't authorised to make. Unlike the "Back to " defect
    /// `I18N-005a-review.md` §2 closed, this prefix is a complete,
    /// closed, guard-covered string in its own right — no user data
    /// flows through this key, so nothing here can carry copy the
    /// guard can't see.
    GoalFieldPrefixLabel,
    /// "Summary", reused for both a section `aria-label` and its
    /// `<h2>`.
    SummaryHeading,
    /// Reused between the sprint-detail summary card's stat heading
    /// and the burndown chart's legend — same word, same meaning, in
    /// two places on the same page.
    CommittedStatLabel,
    CompletedStatLabel,
    InFlightStatLabel,
    /// "Carried over", the standalone stat-card heading. See
    /// [`MessageKey::CaptionWordCarriedOver`] for the lowercase,
    /// mid-sentence sibling this is deliberately not coupled to.
    CarriedOverHeading,
    /// "pt", the bare unit suffix rendered as its own DOM node next
    /// to a raw number (not composed via `format!` the way
    /// [`MessageKey::PointsValue`] is) — the sprint summary card's
    /// four stat tiles all share this shape.
    PointsUnitSuffix,
    /// "{count} issues" — reused across the summary card's four stat
    /// tiles.
    IssuesCountText {
        count: i64,
    },
    BurndownHeading,
    BurndownSectionAriaLabel,
    BurndownCaptionLead,
    /// "committed", lowercase and mid-sentence, burndown caption only
    /// — distinct from [`MessageKey::CommittedStatLabel`]'s
    /// standalone use, same non-coupling reasoning as
    /// [`MessageKey::CaptionWordCompleted`].
    CaptionWordCommitted,
    BurndownCaptionMiddle,
    BurndownCaptionTail,
    /// `first_label`/`last_label` are formatted dates (data);
    /// `max_val` is a count (data).
    BurndownChartAriaLabel {
        first_label: String,
        last_label: String,
        max_val: i64,
    },
    IssuesInSprintAriaLabel,
    IssuesHeading,
    NoIssuesInSprintMessage,
    SprintIssuesAriaLabel,
    EditSprintPageTitle {
        sprint_name: String,
    },
    EditSprintHeading,

    // ---- PLAN-001: the sprint planning page (RFC 001) ----
    SprintPlanPageTitle {
        sprint_name: String,
    },
    /// The breadcrumb's terminal word on the plan page, following
    /// [`MessageKey::EditWord`]'s pattern of one bare word per
    /// action-page breadcrumb.
    SprintPlanBreadcrumbWord,
    BacklogHeading,
    SprintItemsHeading,
    BacklogFilterAriaLabel,
    AllPrioritiesOption,
    AllProjectsOption,
    /// The `"→ Sprint"` move button. Literal per RFC 001's own HTML
    /// sketch — a directional glyph plus a short word, not a sentence,
    /// so it doesn't need `Field`-style parameterisation.
    MoveToSprintButton,
    MoveToBacklogButton,
    /// Per-row `aria-label`, RFC 001 §17.4's (title, points, column)
    /// triple for a backlog row. `points` stays a bare `i64` rather
    /// than a boxed [`MessageKey::PointsValue`] — the triple reads as
    /// one sentence a screen reader speaks whole, and `PointsValue`'s
    /// "{n} pt" would nest awkwardly inside it.
    BacklogRowAriaLabel {
        title: String,
        points: i64,
    },
    SprintItemRowAriaLabel {
        title: String,
        points: i64,
    },
    /// The plan page's header stat. Deliberately not built from
    /// [`MessageKey::CommittedStatLabel`] + [`MessageKey::PointsUnitSuffix`]
    /// (the detail page's stat-card composition) — `PLAN-001` §4 test 7
    /// requires the literal substring `"13 pts"` (plural) for two
    /// issues at 5 and 8 points, which the detail page's singular
    /// `"pt"` convention doesn't produce; RFC 001's own sketch already
    /// used the plural for this exact line.
    CommittedTotalLabel {
        committed_points: i64,
    },
    NoBacklogIssuesMessage,
    NoSprintItemsInPlanMessage,
    /// Server-side guard on `plan_add`/`plan_remove`: requirement 8
    /// says an active or completed sprint's plan page renders with no
    /// move buttons, and this is the backend half of that — a forged
    /// POST bypassing the missing button must still fail, the same
    /// defense-in-depth posture this crate already applies elsewhere
    /// (`CannotAssignToCompletedSprintMessage`'s sibling, but this one
    /// also covers active — see `PLAN-001`'s handler doc comment for
    /// why active is included too).
    SprintPlanNotEditableMessage,
    NewTeamLink,
    TeamsEmptyIntro,
    TeamsEmptyCta,
    YourTeamsAriaLabel,
    /// `team_name` is user data; `role` is the closed-set role word,
    /// kept typed for the same reason as
    /// [`MessageKey::MoveIssueAriaLabel`] (`I18N-005b`).
    TeamRoleAriaLabel {
        team_name: String,
        role: TeamRoleLabel,
    },
    /// "New team", reused as the page `<title>` and the `<h1>`.
    NewTeamLabel,
    TeamNamePlaceholder,
    SlugFieldLabel,
    OptionalAutoDerivedHint,
    SlugPlaceholder,
    SlugHelperText,
    TeamDescriptionPlaceholder,
    NewTeamIntro,
    CreateTeamButton,
    EditTeamSettingsAriaLabel,
    /// "Settings", reused as the team-detail page's settings link and
    /// the edit-page breadcrumb — the same bare word
    /// [`MessageKey::NavLinkSettings`] (`I18N-005a`) already renders
    /// for the global settings link. Same word, context-free at the
    /// call site (the surrounding page tells the reader which
    /// settings); reused rather than duplicated.
    InviteMemberSummary,
    ByEmailHint,
    EmailPlaceholderExample,
    AddButton,
    InviteHelperText,
    /// "Members", reused for a section `aria-label` and its `<h2>`.
    MembersHeading,
    TeamMembersAriaLabel,
    JoinedColumnHeading,
    /// `TeamDetailPage`'s `FR-TEAM-005` privacy footnote. Converted
    /// byte-identically from the current source — see the review
    /// request for the discrepancy found between this text and the
    /// handoff's quoted paraphrase of the requirement, escalated
    /// rather than resolved by editing either one.
    TeamPrivacyFootnote,
    DetachFromTeamAriaLabel,
    DetachButton,
    TeamProjectsAriaLabel,
    NoProjectsInTeamMessage,
    ChangeRoleAriaLabel,
    LeaveTeamAriaLabel,
    LeaveButton,
    RemoveMemberAriaLabel,
    RemoveButton,
    /// "(you)", the self-identifying suffix next to a member's name
    /// in the members table.
    YouSuffix,
    EditTeamPageTitle {
        team_name: String,
    },
    TeamSettingsHeading,
    SlugFixedNotice,
    SprintCreatedFlash,
    SprintUpdatedFlash,
    SprintStartedFlash,
    SprintCompletedFlash,
    SprintDeletedFlash,
    SprintAssignmentSavedFlash,
    TeamCreatedFlash,
    MemberAddedFlash,
    RoleUpdatedFlash,
    /// The demotion-path last-admin guard. Distinct wording from
    /// [`MessageKey::LastAdminRemovalError`] — both share a stem but
    /// diverge after it, describing the specific action each guards;
    /// converted as two keys rather than unified, matching the
    /// "no rewording" rule.
    LastAdminDemotionError,
    LastAdminRemovalError,
    YouLeftTeamFlash,
    MemberRemovedFlash,
    TeamUpdatedFlash,
    ProjectDetachedFlash,
    /// `email` is the user-typed search input, echoed back — genuine
    /// user data, per `RFC 006` D4.
    NoUserWithEmailFound {
        email: String,
    },

    // ---- I18N-005d: components/me ----
    /// A coarse estimation-skew value. `days_per_point` is data
    /// (the filtering that decides whether to show it at all —
    /// finite, positive, not near-zero — stays in `peisear-web`;
    /// this key only renders an already-decided-worth-showing
    /// value).
    PaceValue {
        days_per_point: f64,
    },
    ReadFirstOverloadTitle,
    ReadFirstOverloadBody {
        overload_streak_days: i64,
        window_days: i64,
    },
    ReadFirstStalledTitle,
    ReadFirstStalledBody {
        stalled_assigned_max_days: i64,
    },
    ReadFirstWipTitle,
    ReadFirstWipBody {
        current_wip: i64,
        effective_wip_limit: i64,
    },
    ReadFirstLongStaleTitle,
    /// The "issue"/"issues" pluralisation happens inside this key's
    /// own render arm, not as a separate `String` parameter — a
    /// word this crate authors is never data, so it can't cross the
    /// boundary as a `String` (rule 1), and it isn't reused by any
    /// other template (rule 3 doesn't call for an enum either).
    ReadFirstLongStaleBody {
        long_stale_count: i64,
    },
    PersonalDashboardTitle,
    NothingToShowMessage,
    /// `FR-PER-001`/`NFR-PRIV-001`: `/today`'s privacy claim.
    /// `display_name` is genuine user data. Asserted byte-exact by
    /// `personal_dashboard_privacy_subtitle_renders_byte_identically`
    /// per `I18N-005d` §7's explicit requirement.
    PersonalDashboardSubtitle {
        display_name: String,
    },
    ReadFirstAriaLabel,
    RightNowHeading,
    WipChipLabel,
    LoadChipLabel,
    LoadChipTooltip,
    PeriodHintTooltip,
    ThisPeriodHint,
    RhythmAriaLabel,
    RhythmSummaryLabel,
    ThroughputTooltip,
    ThroughputChipLabel,
    LongStaleChipLabel,
    PaceTooltip,
    PaceChipLabel,
    WhatDoTheseMeanLabel,
    /// The glossary's `<strong>"WIP"</strong>` term reuses
    /// [`MessageKey::WipChipLabel`]; this is only the definition
    /// clause that follows it, split at the existing `<strong>`
    /// boundary per the sanctioned-composition rule (`I18N-005c-review`
    /// §6 — markup requires the split, each fragment is independently
    /// meaningful).
    WipGlossaryDefinition,
    LoadGlossaryDefinition,
    ThroughputGlossaryDefinition {
        window_days: i64,
    },
    LongStaleGlossaryDefinition {
        window_days: i64,
    },
    PaceGlossaryDefinition,
    /// Reused as the section `aria-label`, the section `<h2>`, and
    /// the glossary's `<strong>"Sustainability"</strong>` term — the
    /// identical word, standalone role, in three places.
    SustainabilityHeading,
    SustainabilityGlossaryDefinition,
    /// Reused as the `<h3>` and the glossary's
    /// `<strong>"Patterns"</strong>` term.
    PatternsSubheading,
    PatternsGlossaryDefinition,
    OverloadStreakChipLabel,
    OldestStalledChipLabel,
    PatternsDisclaimer,
    /// The panel-level privacy note. Deliberately not unified with
    /// [`MessageKey::PersonalDashboardSubtitle`]'s "Visible only to
    /// you." — different sentence, different wording, both genuine.
    SustainabilityPrivacyNote,
    OverloadStreakValue {
        overload_streak_days: i64,
        window_days: i64,
    },
    StalledDaysValue {
        stalled_assigned_max_days: i64,
    },
    /// `is_watch` is the two-value collapse `me.rs`'s own
    /// `chip_classes` closure already performs locally (`Watch` vs.
    /// `Good`/`Insufficient` collapsed to "watch"/"steady") — not
    /// sourced from `DisplayHealthState::glyph()` (that function is
    /// `I18N-006`'s still-undispatched gap; see the review request's
    /// finding). A `bool` rather than a two-value enum: this word
    /// choice isn't a `peisear-core` domain concept with a name of
    /// its own, just a local presentation collapse, reused across
    /// exactly these two templates.
    OverloadStreakAriaLabel {
        overload_streak_days: i64,
        is_watch: bool,
    },
    StalledAriaLabel {
        stalled_assigned_max_days: i64,
        is_watch: bool,
    },
    /// The 28-day window is `user_burnout::DRIFT_WINDOW_DAYS`,
    /// hardcoded in the source as a literal rather than
    /// interpolated — converted as-is (`I18N-005d` "convert it, do
    /// not improve it"), not switched to a parameter.
    DriftInsufficientDataAriaLabel,
    PaceDriftChipLabel,
    /// "need more data" — reused by both the drift and switching
    /// insufficient-data chips.
    NeedMoreDataLabel,
    /// The chip's own short word ("longer per point" / "shorter per
    /// point" / "steady"). See [`MessageKey::DriftAriaLabel`] for the
    /// longer "trending ..." phrase built from the same
    /// [`DriftDirectionLabel`].
    DriftDirectionWord {
        direction: DriftDirectionLabel,
    },
    DriftValueLine {
        recent_median_days_per_point: f64,
        older_median_days_per_point: f64,
    },
    /// Composes the full sentence — including the long "trending
    /// ..." phrase — from typed data in this one render arm, per
    /// `I18N-005c-review` §3's correction: not a pre-rendered
    /// `String` threaded in from [`MessageKey::DriftDirectionWord`]
    /// or [`MessageKey::DriftValueLine`].
    DriftAriaLabel {
        recent_median_days_per_point: f64,
        older_median_days_per_point: f64,
        window_days: i64,
        direction: DriftDirectionLabel,
    },
    /// The 14-day window is `user_burnout::SWITCHING_WINDOW_DAYS`,
    /// same "converted as-is" note as
    /// [`MessageKey::DriftInsufficientDataAriaLabel`].
    SwitchingInsufficientDataAriaLabel,
    SwitchingChipLabel,
    SwitchingMedianValue {
        median: f64,
    },
    SwitchingSampleLine {
        total_events_observed: i64,
        window_days: i64,
    },
    /// `COPY-001` §2 fix: originally embedded
    /// [`MessageKey::SwitchingMedianValue`]'s own " / active day"
    /// suffix into a template that appended "pickups per active
    /// day", so a screen reader heard "per active day" twice — found
    /// at `I18N-005d`, correctly left unreworded there under the
    /// no-rewording rule, and ruled a real defect here. Now formats
    /// the median number itself (`switching_median_number`, the same
    /// one-decimal rule `switching_median_text` uses for the chip)
    /// with no suffix, composed from the typed `median: f64` in this
    /// arm rather than a pre-rendered `String`, per
    /// `I18N-005c-review` §3.
    SwitchingAriaLabel {
        median: f64,
        total_events_observed: i64,
        window_days: i64,
    },

    // ---- I18N-005d: components/settings ----
    /// Merges what was two sentences at the call site (a static
    /// lead plus a `format!`-composed hint) into one key taking the
    /// raw `default_wip_limit: i64`, avoiding the
    /// pre-rendered-`String`-as-parameter shape `I18N-005c-review`
    /// §3 corrected elsewhere.
    WipLimitExplanation {
        default_wip_limit: i64,
    },
    NoCapacitySetTodayLabel,
    ConflictLabel,
    /// `SCR-22`'s overlap-rejection guidance, split at the existing
    /// `<em>"Close on date"</em>` boundary (sanctioned composition,
    /// `I18N-005c-review` §6). [`MessageKey::CloseOnDateActionWord`]
    /// is the emphasised middle fragment.
    CapacityOverlapGuidanceLead,
    CloseOnDateActionWord,
    CapacityOverlapGuidanceTail,
    /// Reused as the page `<title>`, the breadcrumb leaf, and the
    /// `<h1>` — identical word, three standalone-heading roles.
    SettingsSectionName,
    /// `display_name` is genuine user data.
    SettingsSubtitle {
        display_name: String,
    },
    CapacitySectionAriaLabel,
    WorkloadCapacityHeading,
    CapacityExplanationParagraph,
    /// Composes "Effective capacity today: ..." from the raw
    /// `Option<i64>` in one arm, rather than embedding the
    /// pre-rendered `effective_label` `String` the component used to
    /// build separately (same class of fix as `I18N-005c-review` §3).
    EffectiveCapacityTodayAriaLabel {
        points: Option<i64>,
    },
    EffectiveTodayLabel,
    CapacityRowsTableAriaLabel,
    /// Reused: the table column heading and both forms' field label
    /// (add-row and edit-row) all say bare "Points".
    PointsColumnHeading,
    /// Reused: the table column heading and the edit-row form's
    /// field label both say bare "From".
    FromColumnHeading,
    /// The add-row form's field label spells out the format —
    /// different text from [`MessageKey::FromColumnHeading`], not
    /// unified.
    FromDateFieldLabel,
    ToColumnHeading,
    ToDateFieldLabel,
    /// Reused: the table column heading and both forms' field label.
    NoteColumnHeading,
    ActionsColumnHeading,
    AddCapacityRowSummary,
    AddCapacityRowFormAriaLabel,
    PointsPlaceholderExample,
    NoteFieldPlaceholder,
    AddRowButton,
    CapacityOverlapHelperText,
    /// Reused as the section `aria-label`, the section `<h2>`, and
    /// the WIP-limit form field's own label — identical word, three
    /// standalone roles.
    WipLimitLabel,
    InProgressIssuesHint,
    /// `from`/`to` are formatted dates or [`MessageKey::NoValuePlaceholder`]'s
    /// "—" — formatted data, not copy (`I18N-005c-review` §3's
    /// carve-out for `dates: String` applies the same way here).
    CapacityRowAriaLabel {
        points: i64,
        from: String,
        to: String,
    },
    CloseOnDateSummary,
    CloseThisRowAriaLabel,
    CloseOnLabel,
    CloseButton,
    EditRowAriaLabel,
    RemoveThisRowAriaLabel,

    // ---- I18N-005d: components/{notification_preferences,notifications} ----
    EmailNotificationsHeading,
    FirstTimeEmailPromptAriaLabel,
    EmailOptInPromptBody,
    EmailOptInYesButton,
    EmailOptInNoButton,
    EmailOptInOnStatus,
    EmailOptInOffStatus,
    NotificationPreferencesPageTitle,
    /// Reused as this page's breadcrumb leaf and `<h1>`, and as
    /// `/inbox`'s own page `<title>`/`<h1>` — identical word across
    /// both pages, same standalone-heading role.
    NotificationsSectionName,
    SilenceAllAriaLabel,
    SilenceAllButton,
    DefaultsInAppLead,
    PerKindDeliverySummary,
    ClickToExpandHint,
    NotificationKindsTableAriaLabel,
    KindColumnHeading,
    MinSeverityColumnHeading,
    ChannelStubDisclaimer,
    SavePreferencesButton,
    NotificationKindPreferencesAriaLabel {
        kind: NotificationKindLabel,
    },
    InAppForKindAriaLabel {
        kind: NotificationKindLabel,
    },
    EmailForKindAriaLabel {
        kind: NotificationKindLabel,
    },
    WebhookForKindAriaLabel {
        kind: NotificationKindLabel,
    },
    MinSeverityForKindAriaLabel {
        kind: NotificationKindLabel,
    },
    AllSeverityOption,
    WatchOnlySeverityOption,
    /// Absorbs `peisear_core::notifications::kind::human_name` — see
    /// [`NotificationKindLabel`]'s doc comment. Also reused as the
    /// per-kind row's visible label.
    NotificationKindName {
        kind: NotificationKindLabel,
    },
    /// Absorbs `peisear_core::notifications::channel::human_name` —
    /// see [`NotificationChannelLabel`]'s doc comment. Reused as the
    /// preferences table's In-app/Email/Webhook column headings
    /// (identical words) and the inbox row's "Sent via ..." list.
    NotificationChannelName {
        channel: NotificationChannelLabel,
    },
    NoNotificationsYetStatus,
    UnreadOfTotalStatus {
        unread_count: i64,
        total: i64,
    },
    AllReadStatus {
        total: i64,
    },
    MarkAllReadAriaLabel,
    MarkAllReadButton,
    InboxEmptyMessage,
    /// Split around the `<a href="/settings/notifications">` link
    /// (sanctioned composition, `I18N-005c-review` §6).
    /// [`MessageKey::SettingsLinkWord`] is the link text.
    InboxEmptyFooterLead,
    /// Lowercase "settings" as the inline link text — distinct from
    /// [`MessageKey::SettingsSectionName`]'s capitalised standalone
    /// heading, same non-coupling as `NavLinkToday` vs. the lowercase
    /// back-link "today" (`I18N-005a`).
    SettingsLinkWord,
    InboxEmptyFooterTail,
    NotificationListAriaLabel,
    /// Reused as the unread badge's own `aria-label` and as the
    /// composed row `aria-label`'s internal word choice (with
    /// [`MessageKey::ReadWord`]) — one word, two render sites.
    UnreadWord,
    ReadWord,
    /// `is_unread` drives the internal `Unread`/`Read` word choice
    /// (via [`MessageKey::UnreadWord`]/[`MessageKey::ReadWord`],
    /// rendered inline in this arm rather than threaded in as a
    /// pre-rendered `String`). `title` is genuine user data;
    /// `timestamp` is formatted data, not copy.
    NotificationRowAriaLabel {
        is_unread: bool,
        title: String,
        kind: NotificationKindLabel,
        timestamp: String,
    },
    SentViaPrefix,
    ViewContextLinkLabel,
    MarkAsReadAriaLabel,
    MarkReadButton,

    // ---- I18N-005d: components/search ----
    /// Reused as the page `<title>` (empty-query case), the submit
    /// button, and the search form's own `aria-label` — identical
    /// word, three standalone roles.
    SearchWord,
    /// `q` is genuine user data (the search query, echoed back).
    SearchPageTitleWithQuery {
        q: String,
    },
    SearchFieldLabel,
    SearchPlaceholder,
    ResultsForHeadingPrefix,
    /// `SCR-24`'s blank-query guidance — copy, must survive
    /// conversion intact per `I18N-005d` §5.
    NoQueryGuidanceMessage,
    OpenIssuesSectionName,
    NoMatchesInCategoryMessage,
    PreviousPageLink,
    NextPageLink,
    ProjectHitTypeLabel,
    /// `project_name` is genuine user data.
    OpenIssueHitTypePrefix {
        project_name: String,
    },

    // ---- I18N-005d: handlers/{settings,notification_preferences,notifications} ----
    WipLimitSavedFlash,
    CapacityRowAddedFlash,
    CapacityRowUpdatedFlash,
    CapacityRowRemovedFlash,
    RowClosedFlash,
    PreferencesSavedFlash,
    AllNotificationsSilencedFlash,
    MarkedAsReadFlash {
        count: i64,
    },

    // ---- I18N-005e: error.rs (ApiAppError — the AppError-side keys
    // ---- Forbidden/NotFound/InternalError/OptimisticLockConflict/
    // ---- LockValueUnreadable were already seeded by I18N-001 and are
    // ---- wired into AppError::public_message() by this handoff, not
    // ---- re-declared here) ----
    /// `ApiAppError::Unauthorized`'s JSON `message` field. Distinct
    /// wording from [`MessageKey::Forbidden`] ("permission denied") —
    /// the API surface writes a fuller sentence for its JSON
    /// consumers; not unified, since the two are genuinely different
    /// text for different response shapes (HTML body word vs. JSON
    /// message sentence).
    ApiUnauthorizedMessage,
    ApiForbiddenMessage,
    ApiNotFoundMessage,
    /// `ApiAppError::OptimisticLockConflict`'s JSON `message` field.
    /// Shorter than [`MessageKey::OptimisticLockConflict`]'s HTML
    /// wording ("Reload and re-apply your change." vs. "Please
    /// reload the page and re-apply your change so you don't
    /// overwrite their work.") — genuinely different text in the
    /// current source, kept as two keys per the no-rewording rule.
    ApiOptimisticLockConflictMessage {
        entity: EntityKind,
    },

    // ---- I18N-005e: components/auth.rs, handlers/auth.rs ----
    LoginPageTitle,
    RegisterPageTitle,
    SignInTaglineText,
    RegisterTaglineText,
    /// "Sign in" — reused as the login form's submit button and the
    /// register page's footer link back to `/login`.
    SignInWord,
    CreateAccountButton,
    PasswordFieldLabel,
    DisplayNameFieldLabel,
    PasswordMinLengthHint,
    NoAccountPrompt,
    CreateOneLinkWord,
    AlreadyHaveAccountPrompt,
    /// `FR-AUTH-002`: a failed login must not disclose which field
    /// was wrong. `handlers/auth.rs::login_submit` already converges
    /// unknown-account and wrong-password onto this single message
    /// through one code path (verified, not just converted
    /// faithfully) — asserted indistinguishable by
    /// `login_failure_message_is_identical_for_unknown_account_and_wrong_password`
    /// per `I18N-005e` §7's explicit requirement. Converting this
    /// must not split it into per-field keys, however tempting the
    /// symmetry (`I18N-005e` §3/§9).
    InvalidCredentialsMessage,
    EmailAlreadyExistsMessage,
    /// `handlers.rs::format_validation`'s fallback when a `validator`
    /// error fires without a custom `message` — reachable only
    /// defensively today (every derive rule in this codebase
    /// currently supplies a `message`), but user-facing.
    InvalidInputFallbackMessage,

    // ---- I18N-005e: handlers/issues.rs ----
    InvalidAssigneeMessage,
    /// Used by both `new_sub_issue_form` (GET, form-render-time) and
    /// `create_sub_issue` (POST, submission-time). A distinct, shorter
    /// `SubIssueCannotNestShortMessage` existed at the POST site until
    /// `COPY-001` §3 unified them: external design §10.4 rule 4 says
    /// an error describes what happened *and* what would resolve it,
    /// the long form does both and the short form is the same
    /// rejection with the resolution removed, and both sites are a
    /// full-width error surface with room for the longer sentence.
    /// Not a preference for longer copy — a user just told they can't
    /// do something needs to know what they can do instead.
    SubIssueCannotNestLongMessage,

    // ---- I18N-005e: handlers/sprints.rs ----
    SprintNameRequiredMessage,
    SubIssueFollowsParentSprintMessage,
    SprintsPersonalProjectMessage,
    SprintProjectTeamMismatchMessage,
    CannotAssignToCompletedSprintMessage,

    // ---- I18N-005e: handlers/teams.rs ----
    TeamNameRequiredMessage,
    SlugDerivationFailedMessage,
    InvalidRoleMessage,

    // ---- I18N-005e: handlers/settings.rs ----
    /// "Capacity points are required." — kept as its own literal
    /// rather than routed through [`MessageKey::FieldRequired`]
    /// (`Field::CapacityPoints` renders "Capacity points", and
    /// `FieldRequired`'s template is "{field} is required." — the
    /// grammatical number doesn't match: "Capacity points *are*
    /// required.", not "*is*". Reusing the generic template would
    /// silently reword the sentence.
    CapacityPointsRequiredMessage,
    WipLimitMustBePositiveIntegerMessage,
    PeriodStartMustBeDateFormatMessage,
    PeriodEndMustBeDateFormatMessage,

    // ---- I18N-006: peisear-core/src/lib.rs ----
    /// The six indicator explanation sentences. Replaces
    /// `IndicatorKind::description()`, removed entirely (`I18N-006`
    /// §4) — reuses [`IndicatorLabel`], the same closed-set enum
    /// [`MessageKey::IndicatorName`] already keys off of.
    IndicatorDescription {
        label: IndicatorLabel,
    },
    /// `me.rs`'s WIP chip aria-label. Was `format!("WIP: {} of {}
    /// ({}).", current_wip, effective_wip_limit, wip_aria)` where
    /// `wip_aria` came from `glyph()`'s removed second element —
    /// left unconverted by `I18N-005d` specifically because of that
    /// dependency; `glyph()`'s split (`I18N-006` §3) unblocks it.
    WipAriaLabel {
        current_wip: i64,
        effective_wip_limit: i64,
        state: HealthStateLabel,
    },
    /// `me.rs`'s long-stale chip aria-label. Same `glyph()`
    /// dependency and history as [`MessageKey::WipAriaLabel`].
    LongStaleAriaLabel {
        long_stale_count: i64,
        state: HealthStateLabel,
    },
    /// `HealthStrip`'s composite-score chip aria-label
    /// (`components/issues.rs::composite_row`).
    CompositeAriaLabel {
        state: HealthStateLabel,
    },
    /// `HealthStrip`'s per-indicator chip aria-label
    /// (`components/issues.rs::indicator_row`) — composes the name,
    /// value, state word and description into one sentence. The
    /// value is a boxed `MessageKey` (`ind.value_display`, already
    /// typed since `I18N-002`) rendered recursively in the same
    /// `en.rs` arm, not threaded in as a pre-rendered `String` —
    /// the composition rule this release has enforced since
    /// `I18N-005c` §3 applies here too, even though the value is
    /// itself a message rather than raw data.
    IndicatorAriaLabel {
        label: IndicatorLabel,
        value: Box<MessageKey>,
        state: HealthStateLabel,
    },

    // ---- I18N-006: peisear-storage/src/user_capacities.rs ----
    /// `insert`'s pre-check before the schema `CHECK` constraint.
    /// `COPY-001` §4 fix: used to name the raw field identifiers
    /// (`period_start`/`period_end`) rather than a phrase a user
    /// reads, found at `I18N-006` and correctly left unreworded
    /// there under no-rewording. The normative replacement `COPY-001`
    /// specified assumed the capacity-row form's inputs are labelled
    /// "Start date"/"End date"; they are actually
    /// [`MessageKey::FromDateFieldLabel`]/[`MessageKey::ToDateFieldLabel`]
    /// ("From (YYYY-MM-DD)"/"To (YYYY-MM-DD)") — matched to the
    /// actual labels instead of the assumed ones, per that handoff's
    /// own instruction to do exactly that and report it, which this
    /// comment and the review request both do.
    PeriodStartMustPrecedeEndMessage,
    /// `insert` and `update`'s overlap-conflict message — identical
    /// at both call sites, so one key. `period_start`/`period_end`
    /// carry the conflicting row's already-formatted dates (or "—"
    /// for open-ended), not copy.
    CapacityPeriodOverlapMessage {
        row_id: String,
        period_start: String,
        period_end: String,
        points: i64,
    },

    // ---- I18N-006: peisear-storage/src/sprints.rs ----
    /// `insert` and `update`'s date-order check — identical wording
    /// at both call sites, so one key.
    SprintEndDateMustBeOnOrAfterStartMessage,
    SprintAlreadyActiveMessage,
    SprintCannotRestartCompletedMessage,
    /// `start`'s single-active-sprint-per-team conflict.
    /// `sprint_name` is the other sprint's name — genuine data, not
    /// copy.
    OtherSprintActiveInTeamMessage {
        sprint_name: String,
    },
    SprintNotStartedYetMessage,
    SprintAlreadyCompletedMessage,

    // ---- I18N-006: peisear-storage/src/teams.rs ----
    TeamSlugCannotBeEmptyMessage,
    /// `slug` is the caller-proposed value that collided — genuine
    /// data, not copy.
    TeamSlugAlreadyExistsMessage {
        slug: String,
    },
    /// `user_id` renders the raw id rather than a display name,
    /// because storage has no name to hand at this boundary —
    /// found, not introduced; converted as-is under no-rewording.
    UserAlreadyTeamMemberMessage {
        user_id: String,
    },

    // ---- I18N-006: peisear-storage/src/issues.rs (translate_trigger_error) ----
    /// The four `RAISE`-matched needle strings from migration 0015,
    /// unchanged (`DEC-011` — the match array itself is not this
    /// handoff's to touch, only the returned type). The rendered
    /// text is the needle text verbatim; SQLite's trigger wording
    /// *is* the user-facing message here.
    SubIssueCannotHaveSubIssueMessage,
    SubIssueMustShareProjectMessage,
    IssueCannotBeOwnParentMessage,
    CannotDemoteIssueWithSubIssuesMessage,

    // ---- I18N-006: handlers/api_users.rs (BurnoutSignal.label) ----
    /// `code: "overload_streak"`.
    OverloadStreakSignalMessage {
        overload_streak_days: i64,
        window_days: i64,
    },
    /// `code: "stalled_assigned"`.
    StalledAssignedSignalMessage {
        stalled_assigned_max_days: i64,
    },
    /// `code: "estimation_drift"`, `DriftDirection::Up`. Distinct
    /// wording from [`MessageKey::DriftValueLine`]'s "up"/"down"
    /// phrasing elsewhere — found, not unified, joining `I18N-005d`'s
    /// "per active day" duplication in the queue for a copy pass.
    EstimationDriftUpSignalMessage,
    /// `code: "estimation_drift"`, `DriftDirection::Down`.
    EstimationDriftDownSignalMessage,
    /// `code: "cognitive_switching"`. Distinct wording from
    /// [`MessageKey::SwitchingMedianValue`]'s "per active day"
    /// phrasing — same found-not-unified disposition as the drift
    /// pair above.
    CognitiveSwitchingSignalMessage {
        switches_per_day_median: f64,
    },

    // ---- I18N-007: components/issues.rs (render_trend_chip) ----
    /// `Trend::Flat`'s chip word.
    TrendLabelFlat,
    /// `Trend::Up`/`Down`'s chip word -- `delta` is data (the point
    /// figure), the sign is implied by `direction`.
    TrendLabel {
        direction: TrendDirectionLabel,
        delta: u8,
    },
    /// `Trend::Flat`'s aria/title sentence.
    TrendAriaFlat,
    /// `Trend::Up`/`Down`'s aria/title sentence.
    TrendAriaLabel {
        direction: TrendDirectionLabel,
        delta: u8,
    },

    // ---- I18N-007: components/issues.rs (composite_row) ----
    CompositeLabel,

    // ---- I18N-007: components/sprints.rs (burndown legend) ----
    /// Deliberately NOT [`MessageKey::CaptionWordCommitted`] /
    /// [`MessageKey::CaptionWordCompleted`] (`I18N-005c`), which
    /// render lowercase ("committed"/"completed") for the caption
    /// directly below this legend. Reusing them here would force a
    /// casing compromise on one side or the other -- the fifth
    /// instance of the inflection question this release has hit
    /// (`I18N-006-review.md` §6's precedent for recording rather than
    /// absorbing). Recorded, not bent.
    BurndownLegendCommitted,
    BurndownLegendCompleted,

    // ---- I18N-007: components/me.rs ----
    CurrentLoadSectionLabel,
    /// `load_text` when the user has a capacity figure set.
    LoadWithCapacityValue {
        in_flight_points: i64,
        capacity_points: i64,
    },
    /// `load_text` when the user has no capacity figure set.
    LoadNoCapacityValue {
        in_flight_points: i64,
    },
    RecentThroughputValue {
        recent_done_count: i64,
        window_days: i64,
    },
}

impl MessageKey {
    /// Every key this crate defines, with representative parameter
    /// values — the set a guard needs to walk to check every static
    /// template this crate renders.
    ///
    /// Two different kinds of parameter, two different enumeration
    /// strategies (an evolution from `I18N-001`, which only had the
    /// first kind):
    ///
    /// - **Closed system vocabulary** ([`EntityKind`], [`Field`],
    ///   [`IndicatorLabel`], [`NavSection`], [`IssueStatusLabel`],
    ///   [`PriorityLabel`], [`SprintStatusLabel`], [`TeamRoleLabel`],
    ///   [`DriftDirectionLabel`], [`NotificationKindLabel`],
    ///   [`NotificationChannelLabel`])
    ///   — never open-ended user data, so every value is enumerated.
    ///   This is genuinely "every entry of every locale table" for
    ///   these: the full, finite output space.
    /// - **Open numeric parameters** (`i64` counts, days, percentages
    ///   — `I18N-002`'s new territory) — cannot be exhaustively
    ///   enumerated and don't need to be. The guard cares about the
    ///   *static* vocabulary surrounding a number, not which digits
    ///   appear (a count can't itself contain prohibited vocabulary).
    ///   One representative sample value is enough to exercise each
    ///   template.
    ///
    /// Every closed-enum *value* still appears at least once somewhere
    /// in this list — `IndicatorLabel`'s six values are each exercised
    /// through [`MessageKey::IndicatorName`] (`I18N-004`), so
    /// `HealthSummaryTwoWatch` only needs one illustrative pair rather
    /// than all 36 combinations.
    pub fn all() -> Vec<MessageKey> {
        let mut keys = vec![
            MessageKey::Forbidden,
            MessageKey::NotFound,
            MessageKey::InternalError,
            MessageKey::LockValueUnreadable,
            MessageKey::InvalidStatus,
            MessageKey::InvalidPriority,
            // -- I18N-002: format_value --
            MessageKey::IndicatorValueUnavailable,
            MessageKey::IndicatorValueThroughput { done: 5, total: 7 },
            MessageKey::IndicatorValueStaleness { days: 8 },
            MessageKey::IndicatorValueActivity { count: 12 },
            MessageKey::IndicatorValueBusFactorSolo,
            MessageKey::IndicatorValueBusFactor { pct: 62 },
            MessageKey::IndicatorValueLongStale {
                stale: 3,
                in_flight: 12,
            },
            MessageKey::IndicatorValueWipAllWithin,
            MessageKey::IndicatorValueWipOver { count: 2 },
            // -- I18N-002: human_explanation --
            MessageKey::IndicatorExplanationThroughput { done: 5, total: 7 },
            MessageKey::IndicatorExplanationStaleness { days: 8 },
            MessageKey::IndicatorExplanationActivity { count: 12 },
            MessageKey::IndicatorExplanationBusFactorSolo,
            MessageKey::IndicatorExplanationBusFactor { pct: 62 },
            MessageKey::IndicatorExplanationLongStale {
                stale: 3,
                in_flight: 12,
            },
            MessageKey::IndicatorExplanationWipCompliance { count: 2 },
            // -- I18N-002/004: project_health::summarize --
            MessageKey::HealthSummaryHealthy,
            MessageKey::HealthSummaryTwoWatch {
                first: IndicatorLabel::Throughput,
                second: IndicatorLabel::Staleness,
            },
            // -- I18N-002: user_burnout::summarize --
            MessageKey::BurnoutSummarySteady,
            MessageKey::BurnoutSummaryOverloadOnly { days: 4 },
            MessageKey::BurnoutSummaryStalledOnly { days: 6 },
            MessageKey::BurnoutSummaryBoth {
                overload_days: 4,
                stalled_days: 6,
            },
            // -- I18N-003: peisear_notify::edge --
            MessageKey::NotificationBurnoutOverloadTitle,
            MessageKey::NotificationBurnoutOverloadBody {
                streak_snapshots: 3,
            },
            MessageKey::NotificationBurnoutStalledTitle,
            MessageKey::NotificationBurnoutStalledBody { stalled_days: 10 },
            // -- I18N-005a: components/{layout,breadcrumb,error_page} --
            MessageKey::AppBrandName,
            MessageKey::NavBellLabelNone,
            MessageKey::NavBellLabelUnread { count: 3 },
            MessageKey::NavBellCount { count: 3 },
            MessageKey::NavSearchFormLabel,
            MessageKey::NavSearchPlaceholder,
            MessageKey::NavSearchQueryLabel,
            MessageKey::NavSearchSuggestionsLabel,
            MessageKey::NavLinkToday,
            MessageKey::NavLinkTeams,
            MessageKey::NavLinkInbox,
            MessageKey::NavLinkSettings,
            MessageKey::NavSignOut,
            MessageKey::BreadcrumbNavLabel,
            MessageKey::ErrorPageTitle,
            MessageKey::ErrorPageGoHomeLink,
            // -- I18N-005b: components/{issues,projects} --
            MessageKey::ProjectsSectionName,
            MessageKey::ViewToggleBoard,
            MessageKey::ViewToggleList,
            MessageKey::EditWord,
            MessageKey::CancelButton,
            MessageKey::SaveButton,
            MessageKey::DeleteButton,
            MessageKey::NoValuePlaceholder,
            MessageKey::StoryPointsHint,
            MessageKey::PointsValue { points: 3 },
            MessageKey::HealthEmptyMessage,
            MessageKey::ProjectHealthSectionLabel,
            MessageKey::HealthHeading,
            MessageKey::IndicatorsSummaryLabel,
            MessageKey::WorkloadHeading,
            MessageKey::WorkloadSetCapacityLink,
            MessageKey::WorkloadTitle {
                display_name: "Alex Rivera".to_string(),
                in_flight_issues: 4,
            },
            MessageKey::WorkloadHintLabel,
            MessageKey::EmptyBoardHint,
            MessageKey::MoveIssueAriaLabel {
                issue_title: "Login error".to_string(),
                target: IssueStatusLabel::InProgress,
            },
            MessageKey::FilterSortAriaLabel,
            MessageKey::AllStatusesOption,
            MessageKey::AnyoneOption,
            MessageKey::UnassignedOption,
            MessageKey::SortByFieldLabel,
            MessageKey::SortDefaultOption,
            MessageKey::SortRecentlyCreatedOption,
            MessageKey::SortRecentlyUpdatedOption,
            MessageKey::ApplyButton,
            MessageKey::ResetFilterAriaLabel,
            MessageKey::ResetLink,
            MessageKey::UpdatedColumnHeading,
            MessageKey::EmptyIssueListMessage,
            MessageKey::EffortEstimateTooltip,
            MessageKey::ProjectDetailPageTitle {
                project_name: "Customer Portal".to_string(),
            },
            MessageKey::IssueNewPageTitle {
                project_name: "Customer Portal".to_string(),
            },
            MessageKey::NewIssueLabel,
            MessageKey::NewIssueTitlePlaceholder,
            MessageKey::NewIssueDescriptionPlaceholder,
            MessageKey::CreateIssueButton,
            MessageKey::SubIssueNewPageTitle {
                parent_title: "Login error".to_string(),
            },
            MessageKey::NewSubIssueLabel,
            MessageKey::SubIssueNewPageIntro,
            MessageKey::NewSubIssueTitlePlaceholder,
            MessageKey::NewSubIssueDescriptionPlaceholder,
            MessageKey::CreateSubIssueButton,
            MessageKey::IssueDetailPageTitle {
                issue_title: "Login error".to_string(),
                project_name: "Customer Portal".to_string(),
            },
            MessageKey::SubIssuesLabel,
            MessageKey::AddSubIssueLink,
            MessageKey::SubIssuesEmptyMessage,
            MessageKey::SubIssueAriaLabel {
                title: "Fix redirect loop".to_string(),
                status: IssueStatusLabel::Open,
            },
            MessageKey::SprintAssignmentLabel,
            MessageKey::SprintFieldLabel,
            MessageKey::SprintSelectAriaLabel,
            MessageKey::NoSprintOption,
            MessageKey::SprintAssignmentHelperText,
            MessageKey::IssueStatusAriaLabel,
            MessageKey::NoDescriptionProvided,
            MessageKey::CreatedAt {
                formatted: "2026-08-10 09:00".to_string(),
            },
            MessageKey::UpdatedAt {
                formatted: "2026-08-10 09:00".to_string(),
            },
            MessageKey::ProjectsListPageTitle,
            MessageKey::ProjectsSubheading,
            MessageKey::NewProjectLabel,
            MessageKey::ProjectsEmptyMessage,
            MessageKey::CreateFirstProjectButton,
            MessageKey::NoDescriptionShort,
            MessageKey::ProjectNewPageTitle,
            MessageKey::NewBreadcrumbWord,
            MessageKey::ProjectNamePlaceholder,
            MessageKey::ProjectDescriptionPlaceholder,
            MessageKey::TeamFieldLabel,
            MessageKey::OptionalHint,
            MessageKey::PersonalNoTeamOption,
            MessageKey::TeamHelperText,
            MessageKey::CreateProjectButton,
            MessageKey::ProjectEditPageTitle {
                project_name: "Customer Portal".to_string(),
            },
            MessageKey::EditProjectHeading,
            MessageKey::DeleteProjectHeading,
            MessageKey::DeleteProjectWarning,
            MessageKey::IssueDeletedFlash,
            MessageKey::ProjectDeletedFlash,
            // -- I18N-005c: components/{sprints,teams} --
            MessageKey::NewSprintLink,
            MessageKey::SprintsPageTitle {
                team_name: "Frontend Engineering".to_string(),
            },
            MessageKey::SprintsSectionName,
            MessageKey::SprintsListAriaLabel,
            MessageKey::SprintCardSummaryCompleted {
                completed_points: 8,
                committed_points: 10,
                carried_over_points: 2,
            },
            MessageKey::SprintCardSummaryActive {
                completed_points: 5,
                committed_points: 10,
                in_flight_points: 5,
            },
            MessageKey::SprintCardSummaryPlanned {
                committed_points: 10,
                committed_count: 4,
            },
            MessageKey::SprintCardAriaLabel {
                name: "Sprint 4".to_string(),
                status: SprintStatusLabel::Active,
                dates: "2026-08-01 → 2026-08-14".to_string(),
                completed_points: 5,
                committed_points: 10,
                carried_over_points: 0,
                committed_count: 4,
            },
            MessageKey::VelocityBarAriaLabel {
                name: "Sprint 3".to_string(),
                completed_points: 8,
                carried_over_points: 2,
            },
            MessageKey::SprintsEmptyMessageAdmin,
            MessageKey::SprintsEmptyMessageNonAdmin,
            MessageKey::SprintsOptionalNote,
            MessageKey::CompletedWorkHeading,
            MessageKey::RecentCompletedSprintsAriaLabel,
            MessageKey::VelocityCaptionLead,
            MessageKey::CaptionWordCompleted,
            MessageKey::VelocityCaptionMiddle,
            MessageKey::CaptionWordCarriedOver,
            MessageKey::VelocityCaptionTail,
            MessageKey::BarChartAriaLabel,
            MessageKey::MedianLabel { median: 5 },
            MessageKey::NewSprintLabel,
            MessageKey::SprintNamePlaceholder,
            MessageKey::GoalFieldPlaceholder,
            MessageKey::SprintPlannedNoticeLead,
            MessageKey::CaptionWordPlanned,
            MessageKey::SprintPlannedNoticeTail,
            MessageKey::CreateSprintButton,
            MessageKey::StartSprintLabel,
            MessageKey::CompleteSprintLabel,
            MessageKey::GoalFieldPrefixLabel,
            MessageKey::SummaryHeading,
            MessageKey::CommittedStatLabel,
            MessageKey::CompletedStatLabel,
            MessageKey::InFlightStatLabel,
            MessageKey::CarriedOverHeading,
            MessageKey::PointsUnitSuffix,
            MessageKey::IssuesCountText { count: 4 },
            MessageKey::BurndownHeading,
            MessageKey::BurndownSectionAriaLabel,
            MessageKey::BurndownCaptionLead,
            MessageKey::CaptionWordCommitted,
            MessageKey::BurndownCaptionMiddle,
            MessageKey::BurndownCaptionTail,
            MessageKey::BurndownChartAriaLabel {
                first_label: "08-01".to_string(),
                last_label: "08-14".to_string(),
                max_val: 20,
            },
            MessageKey::IssuesInSprintAriaLabel,
            MessageKey::IssuesHeading,
            MessageKey::NoIssuesInSprintMessage,
            MessageKey::SprintIssuesAriaLabel,
            MessageKey::EditSprintPageTitle {
                sprint_name: "Sprint 4".to_string(),
            },
            MessageKey::EditSprintHeading,
            MessageKey::SprintPlanPageTitle {
                sprint_name: "Sprint 4".to_string(),
            },
            MessageKey::SprintPlanBreadcrumbWord,
            MessageKey::BacklogHeading,
            MessageKey::SprintItemsHeading,
            MessageKey::BacklogFilterAriaLabel,
            MessageKey::AllPrioritiesOption,
            MessageKey::AllProjectsOption,
            MessageKey::MoveToSprintButton,
            MessageKey::MoveToBacklogButton,
            MessageKey::BacklogRowAriaLabel {
                title: "Fix the login redirect".to_string(),
                points: 5,
            },
            MessageKey::SprintItemRowAriaLabel {
                title: "Fix the login redirect".to_string(),
                points: 5,
            },
            MessageKey::CommittedTotalLabel {
                committed_points: 13,
            },
            MessageKey::NoBacklogIssuesMessage,
            MessageKey::NoSprintItemsInPlanMessage,
            MessageKey::SprintPlanNotEditableMessage,
            MessageKey::NewTeamLink,
            MessageKey::TeamsEmptyIntro,
            MessageKey::TeamsEmptyCta,
            MessageKey::YourTeamsAriaLabel,
            MessageKey::TeamRoleAriaLabel {
                team_name: "Frontend Engineering".to_string(),
                role: TeamRoleLabel::Admin,
            },
            MessageKey::NewTeamLabel,
            MessageKey::TeamNamePlaceholder,
            MessageKey::SlugFieldLabel,
            MessageKey::OptionalAutoDerivedHint,
            MessageKey::SlugPlaceholder,
            MessageKey::SlugHelperText,
            MessageKey::TeamDescriptionPlaceholder,
            MessageKey::NewTeamIntro,
            MessageKey::CreateTeamButton,
            MessageKey::EditTeamSettingsAriaLabel,
            MessageKey::InviteMemberSummary,
            MessageKey::ByEmailHint,
            MessageKey::EmailPlaceholderExample,
            MessageKey::AddButton,
            MessageKey::InviteHelperText,
            MessageKey::MembersHeading,
            MessageKey::TeamMembersAriaLabel,
            MessageKey::JoinedColumnHeading,
            MessageKey::TeamPrivacyFootnote,
            MessageKey::DetachFromTeamAriaLabel,
            MessageKey::DetachButton,
            MessageKey::TeamProjectsAriaLabel,
            MessageKey::NoProjectsInTeamMessage,
            MessageKey::ChangeRoleAriaLabel,
            MessageKey::LeaveTeamAriaLabel,
            MessageKey::LeaveButton,
            MessageKey::RemoveMemberAriaLabel,
            MessageKey::RemoveButton,
            MessageKey::YouSuffix,
            MessageKey::EditTeamPageTitle {
                team_name: "Frontend Engineering".to_string(),
            },
            MessageKey::TeamSettingsHeading,
            MessageKey::SlugFixedNotice,
            MessageKey::SprintCreatedFlash,
            MessageKey::SprintUpdatedFlash,
            MessageKey::SprintStartedFlash,
            MessageKey::SprintCompletedFlash,
            MessageKey::SprintDeletedFlash,
            MessageKey::SprintAssignmentSavedFlash,
            MessageKey::TeamCreatedFlash,
            MessageKey::MemberAddedFlash,
            MessageKey::RoleUpdatedFlash,
            MessageKey::LastAdminDemotionError,
            MessageKey::LastAdminRemovalError,
            MessageKey::YouLeftTeamFlash,
            MessageKey::MemberRemovedFlash,
            MessageKey::TeamUpdatedFlash,
            MessageKey::ProjectDetachedFlash,
            MessageKey::NoUserWithEmailFound {
                email: "alice@example.com".to_string(),
            },
            MessageKey::PaceValue {
                days_per_point: 2.3,
            },
            MessageKey::ReadFirstOverloadTitle,
            MessageKey::ReadFirstOverloadBody {
                overload_streak_days: 3,
                window_days: 5,
            },
            MessageKey::ReadFirstStalledTitle,
            MessageKey::ReadFirstStalledBody {
                stalled_assigned_max_days: 10,
            },
            MessageKey::ReadFirstWipTitle,
            MessageKey::ReadFirstWipBody {
                current_wip: 5,
                effective_wip_limit: 3,
            },
            MessageKey::ReadFirstLongStaleTitle,
            MessageKey::ReadFirstLongStaleBody {
                long_stale_count: 1,
            },
            MessageKey::PersonalDashboardTitle,
            MessageKey::NothingToShowMessage,
            MessageKey::PersonalDashboardSubtitle {
                display_name: "Alex".to_string(),
            },
            MessageKey::ReadFirstAriaLabel,
            MessageKey::RightNowHeading,
            MessageKey::WipChipLabel,
            MessageKey::LoadChipLabel,
            MessageKey::LoadChipTooltip,
            MessageKey::PeriodHintTooltip,
            MessageKey::ThisPeriodHint,
            MessageKey::RhythmAriaLabel,
            MessageKey::RhythmSummaryLabel,
            MessageKey::ThroughputTooltip,
            MessageKey::ThroughputChipLabel,
            MessageKey::LongStaleChipLabel,
            MessageKey::PaceTooltip,
            MessageKey::PaceChipLabel,
            MessageKey::WhatDoTheseMeanLabel,
            MessageKey::WipGlossaryDefinition,
            MessageKey::LoadGlossaryDefinition,
            MessageKey::ThroughputGlossaryDefinition { window_days: 30 },
            MessageKey::LongStaleGlossaryDefinition { window_days: 30 },
            MessageKey::PaceGlossaryDefinition,
            MessageKey::SustainabilityHeading,
            MessageKey::SustainabilityGlossaryDefinition,
            MessageKey::PatternsSubheading,
            MessageKey::PatternsGlossaryDefinition,
            MessageKey::OverloadStreakChipLabel,
            MessageKey::OldestStalledChipLabel,
            MessageKey::PatternsDisclaimer,
            MessageKey::SustainabilityPrivacyNote,
            MessageKey::OverloadStreakValue {
                overload_streak_days: 3,
                window_days: 5,
            },
            MessageKey::StalledDaysValue {
                stalled_assigned_max_days: 10,
            },
            MessageKey::OverloadStreakAriaLabel {
                overload_streak_days: 3,
                is_watch: true,
            },
            MessageKey::StalledAriaLabel {
                stalled_assigned_max_days: 10,
                is_watch: false,
            },
            MessageKey::DriftInsufficientDataAriaLabel,
            MessageKey::PaceDriftChipLabel,
            MessageKey::NeedMoreDataLabel,
            MessageKey::DriftValueLine {
                recent_median_days_per_point: 1.5,
                older_median_days_per_point: 1.2,
            },
            MessageKey::SwitchingInsufficientDataAriaLabel,
            MessageKey::SwitchingChipLabel,
            MessageKey::SwitchingMedianValue { median: 4.0 },
            MessageKey::SwitchingSampleLine {
                total_events_observed: 12,
                window_days: 14,
            },
            MessageKey::SwitchingAriaLabel {
                median: 4.3,
                total_events_observed: 12,
                window_days: 14,
            },
            MessageKey::WipLimitExplanation {
                default_wip_limit: 3,
            },
            MessageKey::NoCapacitySetTodayLabel,
            MessageKey::ConflictLabel,
            MessageKey::CapacityOverlapGuidanceLead,
            MessageKey::CloseOnDateActionWord,
            MessageKey::CapacityOverlapGuidanceTail,
            MessageKey::SettingsSectionName,
            MessageKey::SettingsSubtitle {
                display_name: "Alex".to_string(),
            },
            MessageKey::CapacitySectionAriaLabel,
            MessageKey::WorkloadCapacityHeading,
            MessageKey::CapacityExplanationParagraph,
            MessageKey::EffectiveCapacityTodayAriaLabel { points: Some(8) },
            MessageKey::EffectiveTodayLabel,
            MessageKey::CapacityRowsTableAriaLabel,
            MessageKey::PointsColumnHeading,
            MessageKey::FromColumnHeading,
            MessageKey::FromDateFieldLabel,
            MessageKey::ToColumnHeading,
            MessageKey::ToDateFieldLabel,
            MessageKey::NoteColumnHeading,
            MessageKey::ActionsColumnHeading,
            MessageKey::AddCapacityRowSummary,
            MessageKey::AddCapacityRowFormAriaLabel,
            MessageKey::PointsPlaceholderExample,
            MessageKey::NoteFieldPlaceholder,
            MessageKey::AddRowButton,
            MessageKey::CapacityOverlapHelperText,
            MessageKey::WipLimitLabel,
            MessageKey::InProgressIssuesHint,
            MessageKey::CapacityRowAriaLabel {
                points: 8,
                from: "2026-08-01".to_string(),
                to: "2026-08-31".to_string(),
            },
            MessageKey::CloseOnDateSummary,
            MessageKey::CloseThisRowAriaLabel,
            MessageKey::CloseOnLabel,
            MessageKey::CloseButton,
            MessageKey::EditRowAriaLabel,
            MessageKey::RemoveThisRowAriaLabel,
            MessageKey::EmailNotificationsHeading,
            MessageKey::FirstTimeEmailPromptAriaLabel,
            MessageKey::EmailOptInPromptBody,
            MessageKey::EmailOptInYesButton,
            MessageKey::EmailOptInNoButton,
            MessageKey::EmailOptInOnStatus,
            MessageKey::EmailOptInOffStatus,
            MessageKey::NotificationPreferencesPageTitle,
            MessageKey::NotificationsSectionName,
            MessageKey::SilenceAllAriaLabel,
            MessageKey::SilenceAllButton,
            MessageKey::DefaultsInAppLead,
            MessageKey::PerKindDeliverySummary,
            MessageKey::ClickToExpandHint,
            MessageKey::NotificationKindsTableAriaLabel,
            MessageKey::KindColumnHeading,
            MessageKey::MinSeverityColumnHeading,
            MessageKey::ChannelStubDisclaimer,
            MessageKey::SavePreferencesButton,
            MessageKey::AllSeverityOption,
            MessageKey::WatchOnlySeverityOption,
            MessageKey::NoNotificationsYetStatus,
            MessageKey::UnreadOfTotalStatus {
                unread_count: 2,
                total: 5,
            },
            MessageKey::AllReadStatus { total: 5 },
            MessageKey::MarkAllReadAriaLabel,
            MessageKey::MarkAllReadButton,
            MessageKey::InboxEmptyMessage,
            MessageKey::InboxEmptyFooterLead,
            MessageKey::SettingsLinkWord,
            MessageKey::InboxEmptyFooterTail,
            MessageKey::NotificationListAriaLabel,
            MessageKey::UnreadWord,
            MessageKey::ReadWord,
            MessageKey::NotificationRowAriaLabel {
                is_unread: true,
                title: "Overload streak".to_string(),
                kind: NotificationKindLabel::BurnoutOverload,
                timestamp: "2026-08-11 09:00 UTC".to_string(),
            },
            MessageKey::SentViaPrefix,
            MessageKey::ViewContextLinkLabel,
            MessageKey::MarkAsReadAriaLabel,
            MessageKey::MarkReadButton,
            MessageKey::SearchWord,
            MessageKey::SearchPageTitleWithQuery {
                q: "kanban".to_string(),
            },
            MessageKey::SearchFieldLabel,
            MessageKey::SearchPlaceholder,
            MessageKey::ResultsForHeadingPrefix,
            MessageKey::NoQueryGuidanceMessage,
            MessageKey::OpenIssuesSectionName,
            MessageKey::NoMatchesInCategoryMessage,
            MessageKey::PreviousPageLink,
            MessageKey::NextPageLink,
            MessageKey::ProjectHitTypeLabel,
            MessageKey::OpenIssueHitTypePrefix {
                project_name: "Frontend Engineering".to_string(),
            },
            MessageKey::WipLimitSavedFlash,
            MessageKey::CapacityRowAddedFlash,
            MessageKey::CapacityRowUpdatedFlash,
            MessageKey::CapacityRowRemovedFlash,
            MessageKey::RowClosedFlash,
            MessageKey::PreferencesSavedFlash,
            MessageKey::AllNotificationsSilencedFlash,
            MessageKey::MarkedAsReadFlash { count: 3 },
            MessageKey::ApiUnauthorizedMessage,
            MessageKey::ApiForbiddenMessage,
            MessageKey::ApiNotFoundMessage,
            MessageKey::LoginPageTitle,
            MessageKey::RegisterPageTitle,
            MessageKey::SignInTaglineText,
            MessageKey::RegisterTaglineText,
            MessageKey::SignInWord,
            MessageKey::CreateAccountButton,
            MessageKey::PasswordFieldLabel,
            MessageKey::DisplayNameFieldLabel,
            MessageKey::PasswordMinLengthHint,
            MessageKey::NoAccountPrompt,
            MessageKey::CreateOneLinkWord,
            MessageKey::AlreadyHaveAccountPrompt,
            MessageKey::InvalidCredentialsMessage,
            MessageKey::EmailAlreadyExistsMessage,
            MessageKey::InvalidInputFallbackMessage,
            MessageKey::InvalidAssigneeMessage,
            MessageKey::SubIssueCannotNestLongMessage,
            MessageKey::SprintNameRequiredMessage,
            MessageKey::SubIssueFollowsParentSprintMessage,
            MessageKey::SprintsPersonalProjectMessage,
            MessageKey::SprintProjectTeamMismatchMessage,
            MessageKey::CannotAssignToCompletedSprintMessage,
            MessageKey::TeamNameRequiredMessage,
            MessageKey::SlugDerivationFailedMessage,
            MessageKey::InvalidRoleMessage,
            MessageKey::CapacityPointsRequiredMessage,
            MessageKey::WipLimitMustBePositiveIntegerMessage,
            MessageKey::PeriodStartMustBeDateFormatMessage,
            MessageKey::PeriodEndMustBeDateFormatMessage,
            // -- I18N-006: peisear-core/src/lib.rs --
            MessageKey::WipAriaLabel {
                current_wip: 5,
                effective_wip_limit: 3,
                state: HealthStateLabel::Watch,
            },
            MessageKey::LongStaleAriaLabel {
                long_stale_count: 2,
                state: HealthStateLabel::Watch,
            },
            MessageKey::CompositeAriaLabel {
                state: HealthStateLabel::Good,
            },
            MessageKey::IndicatorAriaLabel {
                label: IndicatorLabel::Throughput,
                value: Box::new(MessageKey::IndicatorValueThroughput { done: 5, total: 7 }),
                state: HealthStateLabel::Good,
            },
            // -- I18N-006: peisear-storage/src/user_capacities.rs --
            MessageKey::PeriodStartMustPrecedeEndMessage,
            MessageKey::CapacityPeriodOverlapMessage {
                row_id: "row-1".to_string(),
                period_start: "2026-08-01".to_string(),
                period_end: "2026-08-14".to_string(),
                points: 10,
            },
            // -- I18N-006: peisear-storage/src/sprints.rs --
            MessageKey::SprintEndDateMustBeOnOrAfterStartMessage,
            MessageKey::SprintAlreadyActiveMessage,
            MessageKey::SprintCannotRestartCompletedMessage,
            MessageKey::OtherSprintActiveInTeamMessage {
                sprint_name: "Sprint 4".to_string(),
            },
            MessageKey::SprintNotStartedYetMessage,
            MessageKey::SprintAlreadyCompletedMessage,
            // -- I18N-006: peisear-storage/src/teams.rs --
            MessageKey::TeamSlugCannotBeEmptyMessage,
            MessageKey::TeamSlugAlreadyExistsMessage {
                slug: "frontend-engineering".to_string(),
            },
            MessageKey::UserAlreadyTeamMemberMessage {
                user_id: "user-1".to_string(),
            },
            // -- I18N-006: peisear-storage/src/issues.rs (translate_trigger_error) --
            MessageKey::SubIssueCannotHaveSubIssueMessage,
            MessageKey::SubIssueMustShareProjectMessage,
            MessageKey::IssueCannotBeOwnParentMessage,
            MessageKey::CannotDemoteIssueWithSubIssuesMessage,
            // -- I18N-006: handlers/api_users.rs (BurnoutSignal.label) --
            MessageKey::OverloadStreakSignalMessage {
                overload_streak_days: 3,
                window_days: 5,
            },
            MessageKey::StalledAssignedSignalMessage {
                stalled_assigned_max_days: 10,
            },
            MessageKey::EstimationDriftUpSignalMessage,
            MessageKey::EstimationDriftDownSignalMessage,
            MessageKey::CognitiveSwitchingSignalMessage {
                switches_per_day_median: 1.8,
            },
            // -- I18N-007: components/issues.rs (render_trend_chip) --
            MessageKey::TrendLabelFlat,
            MessageKey::TrendAriaFlat,
            // -- I18N-007: components/issues.rs (composite_row) --
            MessageKey::CompositeLabel,
            // -- I18N-007: components/sprints.rs (burndown legend) --
            MessageKey::BurndownLegendCommitted,
            MessageKey::BurndownLegendCompleted,
            // -- I18N-007: components/me.rs --
            MessageKey::CurrentLoadSectionLabel,
            MessageKey::LoadWithCapacityValue {
                in_flight_points: 4,
                capacity_points: 10,
            },
            MessageKey::LoadNoCapacityValue {
                in_flight_points: 4,
            },
            MessageKey::RecentThroughputValue {
                recent_done_count: 2,
                window_days: 14,
            },
        ];
        keys.extend(TrendDirectionLabel::all().into_iter().map(|direction| {
            MessageKey::TrendLabel {
                direction,
                delta: 3,
            }
        }));
        keys.extend(TrendDirectionLabel::all().into_iter().map(|direction| {
            MessageKey::TrendAriaLabel {
                direction,
                delta: 3,
            }
        }));
        keys.extend(
            EntityKind::all()
                .into_iter()
                .map(|entity| MessageKey::OptimisticLockConflict { entity }),
        );
        keys.extend(
            EntityKind::all()
                .into_iter()
                .map(|entity| MessageKey::ApiOptimisticLockConflictMessage { entity }),
        );
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldRequired { field }),
        );
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldMustBeDateFormat { field }),
        );
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldMustBePositiveInteger { field }),
        );
        // Every IndicatorLabel value, exercised through the
        // single-label health-summary key and the standalone name.
        keys.extend(
            IndicatorLabel::all()
                .into_iter()
                .map(|label| MessageKey::HealthSummaryOneWatch { label }),
        );
        keys.extend(
            IndicatorLabel::all()
                .into_iter()
                .map(|label| MessageKey::IndicatorName { label }),
        );
        // Every IndicatorLabel value again, through the description
        // key I18N-006 adds (IndicatorKind::description() absorption).
        keys.extend(
            IndicatorLabel::all()
                .into_iter()
                .map(|label| MessageKey::IndicatorDescription { label }),
        );
        // Every NavSection value, per I18N-005a-review.md §2.
        keys.extend(
            NavSection::all()
                .into_iter()
                .map(|section| MessageKey::BackToSection { section }),
        );
        // Every IssueStatusLabel and PriorityLabel value, per I18N-005b's
        // absorption of IssueStatus::label()/Priority::label().
        keys.extend(
            IssueStatusLabel::all()
                .into_iter()
                .map(|label| MessageKey::IssueStatusName { label }),
        );
        keys.extend(
            PriorityLabel::all()
                .into_iter()
                .map(|label| MessageKey::PriorityName { label }),
        );
        // Every Field value, through the standalone label key
        // I18N-005b adds — FieldRequired/FieldMustBePositiveInteger
        // above already exercise every value through the validation
        // sentences; this exercises the same set through FieldLabel.
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldLabel { field }),
        );
        // Every SprintStatusLabel and TeamRoleLabel value, per
        // I18N-005c's absorption of
        // SprintStatus::human_name()/TeamRole::human_name().
        keys.extend(
            SprintStatusLabel::all()
                .into_iter()
                .map(|label| MessageKey::SprintStatusName { label }),
        );
        keys.extend(
            TeamRoleLabel::all()
                .into_iter()
                .map(|label| MessageKey::TeamRoleName { label }),
        );
        // Every DriftDirectionLabel value, per I18N-005d's absorption
        // of DriftDirection's two local matches in me.rs.
        keys.extend(
            DriftDirectionLabel::all()
                .into_iter()
                .map(|direction| MessageKey::DriftDirectionWord { direction }),
        );
        keys.extend(DriftDirectionLabel::all().into_iter().map(|direction| {
            MessageKey::DriftAriaLabel {
                recent_median_days_per_point: 1.5,
                older_median_days_per_point: 1.2,
                window_days: 28,
                direction,
            }
        }));
        // Every NotificationKindLabel and NotificationChannelLabel
        // value, per I18N-005d's absorption of
        // notifications::kind::human_name()/channel::human_name().
        keys.extend(
            NotificationKindLabel::all()
                .into_iter()
                .map(|kind| MessageKey::NotificationKindName { kind }),
        );
        keys.extend(
            NotificationChannelLabel::all()
                .into_iter()
                .map(|channel| MessageKey::NotificationChannelName { channel }),
        );
        keys
    }
}
