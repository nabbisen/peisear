//! Personal dashboard component for /me.
//!
//! Shows the authenticated user a small set of self-reflection
//! metrics: current WIP vs. limit, current load vs. capacity, recent
//! throughput, long-stale assigned issues, and a coarse estimation
//! skew. The framing is self-reflection, not performance — see the
//! V2.1 brief §1.2 ("自己調整支援") and §0.2 (no performance
//! evaluation).

use axum::response::Html;
use leptos::prelude::*;

use super::layout::AppShell;
use super::t;
use peisear_core::{
    CurrentUser, DisplayHealthState,
    personal_metrics::{
        PERSONAL_ACTIVITY_WINDOW_DAYS, PersonalMetrics, classify_long_stale, classify_wip,
    },
    user_burnout::{
        DriftDirection, UserBurnoutSignals, classify_overload_streak, classify_stalled, summarize,
    },
};
use peisear_i18n::{Locale, MessageKey};

/// Format a coarse estimation-skew value into prose. Returns
/// `None` if there isn't enough data to show.
fn format_skew(days_per_point: Option<f64>) -> Option<String> {
    let skew = days_per_point?;
    if !skew.is_finite() || skew <= 0.0 {
        return None;
    }
    // Cap displayed precision; this is a coarse number and showing
    // 4 decimal places implies a precision the data does not have.
    //
    // Filter values that would render as "0.0" — these come from
    // issues created and immediately marked done within the same
    // session (julianday delta of seconds, divided by effort). They
    // are not meaningful self-reflection signals.
    if skew < 0.05 {
        return None;
    }
    Some(t(MessageKey::PaceValue {
        days_per_point: skew,
    }))
}

/// One callout's worth of "what to read first." Carries the
/// title and a descriptive sentence; the renderer wraps it in
/// the appropriate visual container.
///
/// The struct (rather than just a `String`) keeps the title and
/// body distinct so the renderer can style them differently —
/// the title gets the `font-medium` weight while the body stays
/// regular weight, mirroring the pattern used elsewhere on
/// `/today`.
struct ReadFirst {
    title: String,
    body: String,
}

/// Compute the single most pressing signal for the "what to
/// read first" callout (Phase B PR3 / B-1).
///
/// The priority chain is **strict** — at most one callout
/// renders. The chain order, from highest to lowest:
///
/// 1. Burnout: overload streak ≥ watch threshold, or
///    stalled-assigned days ≥ watch threshold. These are
///    multi-day patterns; if they're firing, they should
///    be the user's first read.
/// 2. WIP > effective limit: actionable in the moment.
///    "Push something to Done before starting more."
/// 3. Long-stale issues count > 0: older but lower-urgency
///    backlog cleanup signal.
///
/// Returns `None` if none apply — by design, we don't
/// manufacture a callout when nothing's pressing
/// (V2.1 §0.3 "Minimal by Default").
///
/// `current_wip` and `effective_wip_limit` come from the
/// user's `PersonalMetrics`. `long_stale_count` is the
/// number of long-stale assigned issues. `burnout` is the
/// optional snapshot from `user_burnout::for_user`.
fn compute_read_first(
    current_wip: i64,
    effective_wip_limit: i64,
    long_stale_count: i64,
    burnout: Option<&UserBurnoutSignals>,
) -> Option<ReadFirst> {
    if let Some(b) = burnout {
        if b.overload_streak_days >= peisear_core::user_burnout::OVERLOAD_STREAK_WATCH {
            return Some(ReadFirst {
                title: t(MessageKey::ReadFirstOverloadTitle),
                body: t(MessageKey::ReadFirstOverloadBody {
                    overload_streak_days: b.overload_streak_days,
                    window_days: b.window_days,
                }),
            });
        }
        if b.stalled_assigned_max_days >= peisear_core::user_burnout::STALLED_WATCH_DAYS {
            return Some(ReadFirst {
                title: t(MessageKey::ReadFirstStalledTitle),
                body: t(MessageKey::ReadFirstStalledBody {
                    stalled_assigned_max_days: b.stalled_assigned_max_days,
                }),
            });
        }
    }

    // WIP over limit. We use strict `>` (not `>=`) to match
    // the meaning of "over" — being exactly at the limit is
    // the limit, not over it.
    if current_wip > effective_wip_limit {
        return Some(ReadFirst {
            title: t(MessageKey::ReadFirstWipTitle),
            body: t(MessageKey::ReadFirstWipBody {
                current_wip,
                effective_wip_limit,
            }),
        });
    }

    // Long-stale. Threshold of 1 is deliberately permissive
    // — even one stale issue is worth surfacing in the
    // dashboard's most prominent slot when nothing more
    // urgent applies.
    if long_stale_count >= 1 {
        return Some(ReadFirst {
            title: t(MessageKey::ReadFirstLongStaleTitle),
            body: t(MessageKey::ReadFirstLongStaleBody { long_stale_count }),
        });
    }

    None
}

#[component]
pub fn PersonalDashboard(
    user: CurrentUser,
    metrics: Option<PersonalMetrics>,
    burnout: Option<UserBurnoutSignals>,
    capacity_is_period_bounded: bool,
    flash: Option<String>,
) -> impl IntoView {
    let display_name = user.display_name.clone();

    // Empty state: user has no metrics row (shouldn't really
    // happen, but treat it gracefully).
    let Some(m) = metrics else {
        return view! {
            <AppShell title=t(MessageKey::PersonalDashboardTitle) user=user flash=flash>
                <div class="max-w-2xl mx-auto">
                    <h1 class="text-xl font-semibold mb-4">{t(MessageKey::PersonalDashboardTitle)}</h1>
                    <p class="text-sm text-base-content/60 italic">
                        {t(MessageKey::NothingToShowMessage)}
                    </p>
                </div>
            </AppShell>
        }
        .into_any();
    };

    let wip_state = DisplayHealthState::from(classify_wip(&m));
    let stale_state = DisplayHealthState::from(classify_long_stale(&m));
    let wip_glyph = wip_state.glyph();
    let stale_glyph = stale_state.glyph();

    let wip_badge_class = format!("badge badge-sm {}", wip_state.badge_class());
    let stale_badge_class = format!("badge badge-sm {}", stale_state.badge_class());

    let wip_value = format!("{} / {}", m.current_wip, m.effective_wip_limit);
    let wip_aria_label = t(MessageKey::WipAriaLabel {
        current_wip: m.current_wip,
        effective_wip_limit: m.effective_wip_limit,
        state: wip_state.to_i18n_label(),
    });

    let load_text = match m.capacity_points {
        Some(cap) => format!("{} / {} pt", m.in_flight_points, cap),
        None => format!("{} pt · no limit", m.in_flight_points),
    };
    // 0.12.0: when today's capacity comes from a period-bounded
    // user_capacities row, surface a small "(this period)" hint
    // alongside the value. Tells the user "this number isn't your
    // permanent default — it's specific to a window".
    let load_period_hint = capacity_is_period_bounded && m.capacity_points.is_some();

    let throughput_text = format!(
        "{} done in last {PERSONAL_ACTIVITY_WINDOW_DAYS}d",
        m.recent_done_count
    );

    let stale_text = format!("{}", m.long_stale_count);
    let stale_aria_label = t(MessageKey::LongStaleAriaLabel {
        long_stale_count: m.long_stale_count,
        state: stale_state.to_i18n_label(),
    });

    // The user is "structurally active" if they have any current
    // WIP, any in-flight points (open or in_progress with effort),
    // or any recently-done work. This drives whether the
    // sustainability panel renders insufficient-data chips: a user
    // with active work deserves to see "we're tracking but don't
    // have enough yet"; a fresh user with nothing assigned doesn't.
    let user_is_active = m.current_wip > 0 || m.in_flight_points > 0 || m.recent_done_count > 0;

    let skew_text = format_skew(m.estimation_skew_days_per_point);

    // Phase B PR3 (B-1) "what to read first" callout.
    // Surfaces the single most pressing signal so a user
    // arriving at /today knows where to look. The priority
    // chain reflects which signals warrant most-immediate
    // attention:
    //
    // 1. Sustained burnout signal (overload streak / stalled
    //    assigned beyond watch thresholds) — these are
    //    multi-day patterns that, if real, deserve the user's
    //    next click.
    // 2. WIP over the user's effective limit — actionable
    //    today: pick something to push to Done before
    //    starting more.
    // 3. Long-stale issues count — older but lower-urgency
    //    than the above two.
    //
    // If none of the above, we render no callout and the
    // user sees the standard dashboard. This is deliberate:
    // V2.1 §0.3 "Minimal by default" — when there's nothing
    // urgent, don't manufacture an alarm.
    //
    // The chain stops at the first match. Callouts compete
    // for attention; surfacing two at once dilutes both.
    let read_first: Option<ReadFirst> = compute_read_first(
        m.current_wip,
        m.effective_wip_limit,
        m.long_stale_count,
        burnout.as_ref(),
    );

    view! {
        <AppShell title=t(MessageKey::PersonalDashboardTitle) user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                <h1 class="text-xl font-semibold mb-1">{t(MessageKey::PersonalDashboardTitle)}</h1>
                <p class="text-sm text-base-content/60 mb-4">
                    {t(MessageKey::PersonalDashboardSubtitle { display_name })}
                </p>

                // Phase B PR3 (B-1) "what to read first"
                // callout. Renders only when something is
                // worth surfacing — see compute_read_first
                // for the priority chain. Visually
                // distinguished from the rest of the page
                // by the alert/info background and the
                // dedicated heading inside.
                {read_first.map(|rf| {
                    let title = rf.title;
                    let body = rf.body;
                    view! {
                        <aside role="note"
                               aria-label=t(MessageKey::ReadFirstAriaLabel)
                               class="alert alert-info bg-info/10 border border-info/40 \
                                      text-base-content mb-6 items-start">
                            <div class="grow">
                                <p class="font-medium">{title}</p>
                                <p class="text-sm mt-1">{body}</p>
                            </div>
                        </aside>
                    }
                })}

                <section class="mb-6" aria-label="Current load">
                    <h2 class="text-xs uppercase tracking-wide text-base-content/60 mb-2">
                        {t(MessageKey::RightNowHeading)}
                    </h2>
                    <div class="flex flex-wrap items-center gap-3">
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=wip_aria_label.clone()
                             title=wip_aria_label>
                            <span class="text-xs text-base-content/70">{t(MessageKey::WipChipLabel)}</span>
                            <span class=wip_badge_class>
                                <span class="mr-1" aria-hidden="true">{wip_glyph}</span>
                                {wip_value}
                            </span>
                        </div>

                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             title=t(MessageKey::LoadChipTooltip)>
                            <span class="text-xs text-base-content/70">{t(MessageKey::LoadChipLabel)}</span>
                            <span class="badge badge-sm badge-ghost">{load_text}</span>
                            {load_period_hint.then(|| view! {
                                <span class="text-xs text-base-content/50 italic"
                                      title=t(MessageKey::PeriodHintTooltip)>
                                    {t(MessageKey::ThisPeriodHint)}
                                </span>
                            })}
                        </div>
                    </div>
                </section>

                // Phase B PR3 (B-1): Rhythm panel folded by
                // default. The "Right now" panel above is the
                // primary canvas; Rhythm is a "if you want to
                // dig" surface, not first-glance content.
                // Default-closed `<details>` keeps the page
                // scannable.
                <details class="mb-6">
                    <summary class="cursor-pointer text-xs uppercase tracking-wide \
                                    text-base-content/60 mb-2 inline-block"
                             aria-label=t(MessageKey::RhythmAriaLabel)>
                        {t(MessageKey::RhythmSummaryLabel)}
                    </summary>
                    <div class="flex flex-wrap items-center gap-3 mt-2">
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             title=t(MessageKey::ThroughputTooltip)>
                            <span class="text-xs text-base-content/70">{t(MessageKey::ThroughputChipLabel)}</span>
                            <span class="badge badge-sm badge-ghost">{throughput_text}</span>
                        </div>

                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=stale_aria_label.clone()
                             title=stale_aria_label>
                            <span class="text-xs text-base-content/70">{t(MessageKey::LongStaleChipLabel)}</span>
                            <span class=stale_badge_class>
                                <span class="mr-1" aria-hidden="true">{stale_glyph}</span>
                                {stale_text}
                            </span>
                        </div>

                        {skew_text.map(|s| view! {
                            <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                                 title=t(MessageKey::PaceTooltip)>
                                <span class="text-xs text-base-content/70">{t(MessageKey::PaceChipLabel)}</span>
                                <span class="badge badge-sm badge-ghost">{s}</span>
                            </div>
                        })}
                    </div>
                </details>

                {render_burnout_panel(burnout, user_is_active)}

                <section class="text-xs text-base-content/60">
                    <details>
                        <summary class="cursor-pointer">{t(MessageKey::WhatDoTheseMeanLabel)}</summary>
                        <div class="mt-2 space-y-2">
                            <p>
                                <strong>{t(MessageKey::WipChipLabel)}</strong>
                                {t(MessageKey::WipGlossaryDefinition)}
                            </p>
                            <p>
                                <strong>{t(MessageKey::LoadChipLabel)}</strong>
                                {t(MessageKey::LoadGlossaryDefinition)}
                            </p>
                            <p>
                                <strong>{t(MessageKey::ThroughputChipLabel)}</strong>
                                {t(MessageKey::ThroughputGlossaryDefinition { window_days: PERSONAL_ACTIVITY_WINDOW_DAYS })}
                            </p>
                            <p>
                                <strong>{t(MessageKey::LongStaleChipLabel)}</strong>
                                {t(MessageKey::LongStaleGlossaryDefinition { window_days: PERSONAL_ACTIVITY_WINDOW_DAYS })}
                            </p>
                            <p>
                                <strong>{t(MessageKey::PaceChipLabel)}</strong>
                                {t(MessageKey::PaceGlossaryDefinition)}
                            </p>
                            <p>
                                <strong>{t(MessageKey::SustainabilityHeading)}</strong>
                                {t(MessageKey::SustainabilityGlossaryDefinition)}
                            </p>
                            <p>
                                <strong>{t(MessageKey::PatternsSubheading)}</strong>
                                {t(MessageKey::PatternsGlossaryDefinition)}
                            </p>
                        </div>
                    </details>
                </section>
            </div>
        </AppShell>
    }
    .into_any()
}

/// Render the burnout / sustainability panel section.
///
/// Returns an empty span when `burnout` is `None` (user not found —
/// shouldn't happen at this layer) or when both signals are
/// uneventful (zero values across the board). The panel is
/// `<details>` with `open` only when at least one indicator is at
/// `Watch`, so the calm case stays visually quiet (V2.1 §0.3
/// "Minimal by Default") and the user only sees the question when
/// something is worth glancing at.
///
/// Visually deliberate choices here:
///
/// - **Watch is the ceiling.** No `Concern` palette appears anywhere
///   in this section, ever. This is the V2.1 §0.2 anti-evaluation
///   posture made visible.
///
/// - **Question-form summary.** The `summarize()` text is a
///   suggestion or self-reflection prompt; it does not say "you are
///   over capacity", it says "consider whether some work can wait".
///
/// - **Honest unit labelling.** "Snapshots" not "days" for the
///   overload streak — the snapshot tick rate is configurable and
///   we don't want to fudge precision.
fn render_burnout_panel(
    burnout: Option<UserBurnoutSignals>,
    user_is_active: bool,
) -> impl IntoView {
    let Some(signals) = burnout else {
        return view! { <span class="hidden"></span> }.into_any();
    };

    // Decide whether the panel should appear at all.
    //
    // The panel is the visible affordance for self-reflection.
    // It should appear whenever the user has *anything* to
    // reflect on — assigned work, recent activity, streak data,
    // or pattern data. Hiding it when streaks happen to be at
    // zero would silence the "we're tracking but don't have
    // enough data yet" state, which is itself information
    // (Q3=A from the design discussion: insufficient data is
    // shown explicitly, not hidden).
    //
    // The single case where we *do* hide is the fresh user with
    // no work at all — there's truly nothing to surface.
    let has_streaks = signals.overload_streak_days > 0 || signals.stalled_assigned_max_days > 0;
    let has_patterns = signals.estimation_drift.is_some() || signals.cognitive_switching.is_some();
    if !has_streaks && !has_patterns && !user_is_active {
        return view! { <span class="hidden"></span> }.into_any();
    }

    // Clamped at the source: these two classifiers never actually
    // reach `Concern` today, but typing this as `DisplayHealthState`
    // rather than the raw `HealthIndicator` makes that a structural
    // fact instead of a claim resting on "the classifier never
    // emits it" (the reasoning that produced the Watch-ceiling
    // defect elsewhere — see DEV-004 / §17.1).
    let overload = DisplayHealthState::from(classify_overload_streak(&signals));
    let stalled = DisplayHealthState::from(classify_stalled(&signals));

    // We map glyphs ourselves to skip the Insufficient/Good split
    // entirely for clarity — both look the same here: the panel is
    // mostly muted unless something is up.
    let chip_classes = |ind: DisplayHealthState| -> (&'static str, &'static str, &'static str) {
        match ind {
            DisplayHealthState::Watch => ("badge badge-sm badge-warning", "⚠", "watch"),
            DisplayHealthState::Good | DisplayHealthState::Insufficient => {
                ("badge badge-sm badge-ghost", "·", "steady")
            }
        }
    };

    let (overload_badge, overload_glyph, _) = chip_classes(overload);
    let (stalled_badge, stalled_glyph, _) = chip_classes(stalled);
    let overload_is_watch = matches!(overload, DisplayHealthState::Watch);
    let stalled_is_watch = matches!(stalled, DisplayHealthState::Watch);

    let overload_value = t(MessageKey::OverloadStreakValue {
        overload_streak_days: signals.overload_streak_days,
        window_days: signals.window_days,
    });
    let stalled_value = t(MessageKey::StalledDaysValue {
        stalled_assigned_max_days: signals.stalled_assigned_max_days,
    });

    let overload_aria_label = t(MessageKey::OverloadStreakAriaLabel {
        overload_streak_days: signals.overload_streak_days,
        is_watch: overload_is_watch,
    });
    let stalled_aria_label = t(MessageKey::StalledAriaLabel {
        stalled_assigned_max_days: signals.stalled_assigned_max_days,
        is_watch: stalled_is_watch,
    });

    let summary = Locale::English.render(summarize(&signals));
    let any_watch = matches!(overload, DisplayHealthState::Watch)
        || matches!(stalled, DisplayHealthState::Watch);

    let drift_chip = render_drift_chip(signals.estimation_drift.as_ref());
    let switching_chip = render_switching_chip(signals.cognitive_switching.as_ref());

    let sustainability_heading = t(MessageKey::SustainabilityHeading);
    let sustainability_heading_aria = sustainability_heading.clone();
    let patterns_subheading = t(MessageKey::PatternsSubheading);

    view! {
        <section class="mb-6" aria-label=sustainability_heading_aria>
            <h2 class="text-xs uppercase tracking-wide text-base-content/60 mb-2">
                {sustainability_heading}
            </h2>
            <details open=any_watch>
                <summary class="cursor-pointer text-sm text-base-content/80 mb-2">
                    {summary}
                </summary>
                {has_streaks.then(|| view! {
                    <div class="mt-2 flex flex-wrap items-center gap-3">
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=overload_aria_label.clone()
                             title=overload_aria_label>
                            <span class="text-xs text-base-content/70">{t(MessageKey::OverloadStreakChipLabel)}</span>
                            <span class=overload_badge>
                                <span class="mr-1" aria-hidden="true">{overload_glyph}</span>
                                {overload_value}
                            </span>
                        </div>
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=stalled_aria_label.clone()
                             title=stalled_aria_label>
                            <span class="text-xs text-base-content/70">{t(MessageKey::OldestStalledChipLabel)}</span>
                            <span class=stalled_badge>
                                <span class="mr-1" aria-hidden="true">{stalled_glyph}</span>
                                {stalled_value}
                            </span>
                        </div>
                    </div>
                })}
                <div class="mt-3">
                    <h3 class="text-xs uppercase tracking-wide text-base-content/50 mb-1">
                        {patterns_subheading}
                    </h3>
                    <div class="flex flex-wrap items-center gap-3">
                        {drift_chip}
                        {switching_chip}
                    </div>
                    <p class="mt-1 text-xs text-base-content/50 italic">
                        {t(MessageKey::PatternsDisclaimer)}
                    </p>
                </div>
                <p class="mt-2 text-xs text-base-content/60 italic">
                    {t(MessageKey::SustainabilityPrivacyNote)}
                </p>
            </details>
        </section>
    }
    .into_any()
}

/// Render the estimation-drift chip. Uses neutral palette
/// (`badge-ghost`) regardless of direction — drift up is not
/// "bad" and drift down is not "good". The arrow is the
/// information; the absence of colour is the design statement.
///
/// When `drift` is `None` the chip renders with an
/// "insufficient data" label rather than hiding entirely. The
/// rationale: a user looking at the panel should be able to tell
/// the difference between "this signal is steady" and "this
/// signal can't be computed yet" — the latter is information,
/// not noise. V2.1 §4.4 (説明可能性) is on this side of the line.
fn render_drift_chip(
    drift: Option<&peisear_core::user_burnout::EstimationDriftTrend>,
) -> impl IntoView {
    let Some(drift) = drift else {
        // Insufficient-data chip. Neutral palette, italic label,
        // explicit aria so screen readers don't experience this
        // as a vanished element.
        let aria = t(MessageKey::DriftInsufficientDataAriaLabel);
        let aria_title = aria.clone();
        return view! {
            <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                 role="group"
                 aria-label=aria
                 title=aria_title>
                <span class="text-xs text-base-content/70">{t(MessageKey::PaceDriftChipLabel)}</span>
                <span class="badge badge-sm badge-ghost italic">
                    <span class="mr-1" aria-hidden="true">{t(MessageKey::NoValuePlaceholder)}</span>
                    {t(MessageKey::NeedMoreDataLabel)}
                </span>
            </div>
        }
        .into_any();
    };

    let direction = drift.direction.to_i18n_label();
    let glyph = match drift.direction {
        DriftDirection::Up => "↑",
        DriftDirection::Down => "↓",
        DriftDirection::Steady => "→",
    };
    let direction_label = t(MessageKey::DriftDirectionWord { direction });

    // Show both halves on the chip itself (Q4=A: visible context).
    // The arrow is the headline; the two numbers underneath give
    // the user the "by how much" answer without a tooltip.
    let value_line = t(MessageKey::DriftValueLine {
        recent_median_days_per_point: drift.recent_median_days_per_point,
        older_median_days_per_point: drift.older_median_days_per_point,
    });

    let aria = t(MessageKey::DriftAriaLabel {
        recent_median_days_per_point: drift.recent_median_days_per_point,
        older_median_days_per_point: drift.older_median_days_per_point,
        window_days: drift.window_days,
        direction,
    });

    view! {
        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria.clone()
             title=aria>
            <span class="text-xs text-base-content/70">{t(MessageKey::PaceDriftChipLabel)}</span>
            <span class="badge badge-sm badge-ghost">
                <span class="mr-1" aria-hidden="true">{glyph}</span>
                {direction_label}
            </span>
            <span class="text-xs text-base-content/60">{value_line}</span>
        </div>
    }
    .into_any()
}

/// Render the cognitive-switching chip. Neutral palette by
/// design — "high switching" can be a debugging session or a
/// legitimate cross-task day, and the panel makes no judgement.
///
/// Insufficient-data branch renders an explicit chip rather than
/// hiding, for the same reason as `render_drift_chip`.
fn render_switching_chip(
    switching: Option<&peisear_core::user_burnout::CognitiveSwitchingPattern>,
) -> impl IntoView {
    let Some(s) = switching else {
        let aria = t(MessageKey::SwitchingInsufficientDataAriaLabel);
        let aria_title = aria.clone();
        return view! {
            <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                 role="group"
                 aria-label=aria
                 title=aria_title>
                <span class="text-xs text-base-content/70">{t(MessageKey::SwitchingChipLabel)}</span>
                <span class="badge badge-sm badge-ghost italic">
                    <span class="mr-1" aria-hidden="true">{t(MessageKey::NoValuePlaceholder)}</span>
                    {t(MessageKey::NeedMoreDataLabel)}
                </span>
            </div>
        }
        .into_any();
    };

    // Visible: median per active day + sample period.
    // The two numbers together let the user contextualise the
    // median ("4 / day, but only over 7 active days" reads quite
    // differently from "4 / day over 14 active days").
    let median_value = t(MessageKey::SwitchingMedianValue {
        median: s.switches_per_day_median,
    });
    let sample_line = t(MessageKey::SwitchingSampleLine {
        total_events_observed: s.total_events_observed,
        window_days: s.window_days,
    });

    let aria = t(MessageKey::SwitchingAriaLabel {
        median: s.switches_per_day_median,
        total_events_observed: s.total_events_observed,
        window_days: s.window_days,
    });

    view! {
        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria.clone()
             title=aria>
            <span class="text-xs text-base-content/70">{t(MessageKey::SwitchingChipLabel)}</span>
            <span class="badge badge-sm badge-ghost">
                {median_value}
            </span>
            <span class="text-xs text-base-content/60">{sample_line}</span>
        </div>
    }
    .into_any()
}

pub fn render_dashboard(
    user: CurrentUser,
    metrics: Option<PersonalMetrics>,
    burnout: Option<UserBurnoutSignals>,
    capacity_is_period_bounded: bool,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <PersonalDashboard
                user=user
                metrics=metrics
                burnout=burnout
                capacity_is_period_bounded=capacity_is_period_bounded
                flash=flash
            />
        }
    })
}
