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

/// A named form field, for validation messages that need to say
/// which field was wrong. Closed and system-controlled for the same
/// reason as [`EntityKind`] — the field being validated is a fixed
/// property of the form, not something a user typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    EffortPoints,
    CapacityPoints,
    CloseDate,
}

impl Field {
    pub fn all() -> [Field; 3] {
        [Field::EffortPoints, Field::CapacityPoints, Field::CloseDate]
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

/// One message this crate can render, in every shipped locale.
///
/// A key enum, not a string constant and not a `HashMap<&str, &str>`:
/// a `match` over this type that omits a variant fails to compile,
/// which is what makes a missing rendering a compile-time error
/// (`RFC 006` requirement 2) rather than a runtime fallback to the
/// key name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ///   [`IndicatorLabel`]) — never open-ended user data, so every
    ///   value is enumerated. This is genuinely "every entry of every
    ///   locale table" for these: the full, finite output space.
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
        ];
        keys.extend(
            EntityKind::all()
                .into_iter()
                .map(|entity| MessageKey::OptimisticLockConflict { entity }),
        );
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldRequired { field }),
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
        keys
    }
}
