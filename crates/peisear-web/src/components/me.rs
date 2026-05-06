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
use peisear_core::{
    CurrentUser, HealthIndicator,
    personal_metrics::{
        PERSONAL_ACTIVITY_WINDOW_DAYS, PersonalMetrics, classify_long_stale, classify_wip,
    },
    user_burnout::{
        DriftDirection, UserBurnoutSignals, classify_overload_streak, classify_stalled,
        summarize,
    },
};

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
    Some(format!("≈ {:.1} d / pt", skew))
}

/// Aria label and inline icon character for an indicator state.
/// Accessibility: pair the colour-coded badge with text + glyph so
/// users without colour vision still get the signal (V2.1 §3.4).
fn indicator_glyph(state: HealthIndicator) -> (&'static str, &'static str) {
    match state {
        HealthIndicator::Insufficient => ("—", "no data"),
        HealthIndicator::Good => ("✓", "good"),
        HealthIndicator::Watch => ("⚠", "watch"),
        HealthIndicator::Concern => ("✗", "concern"),
    }
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
            <AppShell title="My dashboard".to_string() user=user flash=flash>
                <div class="max-w-2xl mx-auto">
                    <h1 class="text-xl font-semibold mb-4">"My dashboard"</h1>
                    <p class="text-sm text-base-content/60 italic">
                        "Nothing to show yet."
                    </p>
                </div>
            </AppShell>
        }
        .into_any();
    };

    let wip_state = classify_wip(&m);
    let stale_state = classify_long_stale(&m);
    let (wip_glyph, wip_aria) = indicator_glyph(wip_state);
    let (stale_glyph, stale_aria) = indicator_glyph(stale_state);

    let wip_badge_class = format!("badge badge-sm {}", wip_state.badge_class());
    let stale_badge_class = format!("badge badge-sm {}", stale_state.badge_class());

    let wip_value = format!("{} / {}", m.current_wip, m.effective_wip_limit);
    let wip_aria_label = format!(
        "WIP: {} of {} ({}).",
        m.current_wip, m.effective_wip_limit, wip_aria
    );

    let load_text = match m.capacity_points {
        Some(cap) => format!("{} / {} pt", m.in_flight_points, cap),
        None => format!("{} pt · no limit", m.in_flight_points),
    };
    // 0.12.0: when today's capacity comes from a period-bounded
    // user_capacities row, surface a small "(this period)" hint
    // alongside the value. Tells the user "this number isn't your
    // permanent default — it's specific to a window".
    let load_period_hint = capacity_is_period_bounded
        && m.capacity_points.is_some();

    let throughput_text = format!(
        "{} done in last {PERSONAL_ACTIVITY_WINDOW_DAYS}d",
        m.recent_done_count
    );

    let stale_text = format!("{}", m.long_stale_count);
    let stale_aria_label = format!(
        "Long-stale assigned issues: {} ({}).",
        m.long_stale_count, stale_aria
    );

    // The user is "structurally active" if they have any current
    // WIP, any in-flight points (open or in_progress with effort),
    // or any recently-done work. This drives whether the
    // sustainability panel renders insufficient-data chips: a user
    // with active work deserves to see "we're tracking but don't
    // have enough yet"; a fresh user with nothing assigned doesn't.
    let user_is_active =
        m.current_wip > 0 || m.in_flight_points > 0 || m.recent_done_count > 0;

    let skew_text = format_skew(m.estimation_skew_days_per_point);

    view! {
        <AppShell title="My dashboard".to_string() user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                <h1 class="text-xl font-semibold mb-1">"My dashboard"</h1>
                <p class="text-sm text-base-content/60 mb-6">
                    "Personal metrics for " {display_name} ". Visible only to you."
                </p>

                <section class="mb-6" aria-label="Current load">
                    <h2 class="text-xs uppercase tracking-wide text-base-content/60 mb-2">
                        "Right now"
                    </h2>
                    <div class="flex flex-wrap items-center gap-3">
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=wip_aria_label.clone()
                             title=wip_aria_label>
                            <span class="text-xs text-base-content/70">"WIP"</span>
                            <span class=wip_badge_class>
                                <span class="mr-1" aria-hidden="true">{wip_glyph}</span>
                                {wip_value}
                            </span>
                        </div>

                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             title="Sum of effort across your in-flight issues">
                            <span class="text-xs text-base-content/70">"Load"</span>
                            <span class="badge badge-sm badge-ghost">{load_text}</span>
                            {load_period_hint.then(|| view! {
                                <span class="text-xs text-base-content/50 italic"
                                      title="Your capacity for today comes from a row \
                                             with a period; check Settings.">
                                    "(this period)"
                                </span>
                            })}
                        </div>
                    </div>
                </section>

                <section class="mb-6" aria-label="Rhythm">
                    <h2 class="text-xs uppercase tracking-wide text-base-content/60 mb-2">
                        "Rhythm"
                    </h2>
                    <div class="flex flex-wrap items-center gap-3">
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             title="Issues you have moved to Done">
                            <span class="text-xs text-base-content/70">"Throughput"</span>
                            <span class="badge badge-sm badge-ghost">{throughput_text}</span>
                        </div>

                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=stale_aria_label.clone()
                             title=stale_aria_label>
                            <span class="text-xs text-base-content/70">"Long-stale"</span>
                            <span class=stale_badge_class>
                                <span class="mr-1" aria-hidden="true">{stale_glyph}</span>
                                {stale_text}
                            </span>
                        </div>

                        {skew_text.map(|s| view! {
                            <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                                 title="Coarse calendar-time-per-point on recent done issues. Phase 1 approximation; do not over-interpret.">
                                <span class="text-xs text-base-content/70">"Pace"</span>
                                <span class="badge badge-sm badge-ghost">{s}</span>
                            </div>
                        })}
                    </div>
                </section>

                {render_burnout_panel(burnout, user_is_active)}

                <section class="text-xs text-base-content/60">
                    <details>
                        <summary class="cursor-pointer">"What do these mean?"</summary>
                        <div class="mt-2 space-y-2">
                            <p>
                                <strong>"WIP"</strong>
                                " — issues currently In Progress assigned to you, vs. your effective WIP limit. Limit comes from your personal setting, the project default, or the system default of 3."
                            </p>
                            <p>
                                <strong>"Load"</strong>
                                " — sum of effort points across your in-flight (Open or In Progress) issues, vs. your capacity if you've set one."
                            </p>
                            <p>
                                <strong>"Throughput"</strong>
                                " — issues you have moved to Done in the last "
                                {PERSONAL_ACTIVITY_WINDOW_DAYS} " days."
                            </p>
                            <p>
                                <strong>"Long-stale"</strong>
                                " — in-flight issues assigned to you that have not been touched in over "
                                {PERSONAL_ACTIVITY_WINDOW_DAYS} " days."
                            </p>
                            <p>
                                <strong>"Pace"</strong>
                                " — active in-progress time per story point on your recently-completed estimated issues, reconstructed from the issue event log. Treat as a self-reflection prompt rather than a measurement; the number reflects how recent issues actually went, not how future ones will."
                            </p>
                            <p>
                                <strong>"Sustainability"</strong>
                                " — a couple of streak-style signals based on the periodic snapshots taken in the background: how many consecutive snapshots you have been over capacity, and how long your oldest assigned issue has been without a status change. The panel is muted by default and only opens itself when something is worth a glance. Visible to you only."
                            </p>
                            <p>
                                <strong>"Patterns"</strong>
                                " — descriptive numbers about your recent rhythm: whether your dwell-time-per-point is drifting, and how often you switch to in_progress on a typical active day. These are facts about how the last few weeks went, not judgements. They sit inside the Sustainability panel and have no warning palette of their own. Visible to you only."
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

    let overload = classify_overload_streak(&signals);
    let stalled = classify_stalled(&signals);

    // Watch ceiling enforced upstream by classify_*; we map glyphs
    // ourselves to skip the Concern branch entirely for clarity.
    let chip_classes = |ind: HealthIndicator| -> (&'static str, &'static str, &'static str) {
        match ind {
            HealthIndicator::Watch => ("badge badge-sm badge-warning", "⚠", "watch"),
            // Good or Insufficient — both look the same here:
            // the panel is mostly muted unless something is up.
            _ => ("badge badge-sm badge-ghost", "·", "steady"),
        }
    };

    let (overload_badge, overload_glyph, overload_aria) = chip_classes(overload);
    let (stalled_badge, stalled_glyph, stalled_aria) = chip_classes(stalled);

    let overload_value = format!(
        "{} of last {}",
        signals.overload_streak_days, signals.window_days
    );
    let stalled_value = format!("{} d", signals.stalled_assigned_max_days);

    let overload_aria_label = format!(
        "Overload streak: {} consecutive snapshots over capacity ({}).",
        signals.overload_streak_days, overload_aria
    );
    let stalled_aria_label = format!(
        "Oldest stalled assigned issue: {} days ({}).",
        signals.stalled_assigned_max_days, stalled_aria
    );

    let summary = summarize(&signals);
    let any_watch = matches!(overload, HealthIndicator::Watch)
        || matches!(stalled, HealthIndicator::Watch);

    let drift_chip = render_drift_chip(signals.estimation_drift.as_ref());
    let switching_chip = render_switching_chip(signals.cognitive_switching.as_ref());

    view! {
        <section class="mb-6" aria-label="Sustainability">
            <h2 class="text-xs uppercase tracking-wide text-base-content/60 mb-2">
                "Sustainability"
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
                            <span class="text-xs text-base-content/70">"Over-capacity streak"</span>
                            <span class=overload_badge>
                                <span class="mr-1" aria-hidden="true">{overload_glyph}</span>
                                {overload_value}
                            </span>
                        </div>
                        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                             role="group"
                             aria-label=stalled_aria_label.clone()
                             title=stalled_aria_label>
                            <span class="text-xs text-base-content/70">"Oldest stalled"</span>
                            <span class=stalled_badge>
                                <span class="mr-1" aria-hidden="true">{stalled_glyph}</span>
                                {stalled_value}
                            </span>
                        </div>
                    </div>
                })}
                <div class="mt-3">
                    <h3 class="text-xs uppercase tracking-wide text-base-content/50 mb-1">
                        "Patterns"
                    </h3>
                    <div class="flex flex-wrap items-center gap-3">
                        {drift_chip}
                        {switching_chip}
                    </div>
                    <p class="mt-1 text-xs text-base-content/50 italic">
                        "These are descriptions of recent rhythm, \
                         not evaluations. Many patterns have legitimate \
                         reasons behind them."
                    </p>
                </div>
                <p class="mt-2 text-xs text-base-content/60 italic">
                    "These signals are visible to you only. They are not used \
                     for evaluation; they exist so you can pace yourself."
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
        let aria = "Estimation drift: not enough completed estimated work \
                    in the last 28 days to compare halves of the window.";
        return view! {
            <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                 role="group"
                 aria-label=aria
                 title=aria>
                <span class="text-xs text-base-content/70">"Pace drift"</span>
                <span class="badge badge-sm badge-ghost italic">
                    <span class="mr-1" aria-hidden="true">"—"</span>
                    "need more data"
                </span>
            </div>
        }
        .into_any();
    };

    let (glyph, direction_label) = match drift.direction {
        DriftDirection::Up => ("↑", "longer per point"),
        DriftDirection::Down => ("↓", "shorter per point"),
        DriftDirection::Steady => ("→", "steady"),
    };

    // Show both halves on the chip itself (Q4=A: visible context).
    // The arrow is the headline; the two numbers underneath give
    // the user the "by how much" answer without a tooltip.
    let value_line = format!(
        "recent {:.2} vs. {:.2} d / pt",
        drift.recent_median_days_per_point, drift.older_median_days_per_point,
    );

    let aria = format!(
        "Estimation drift: recent {:.2} d/pt vs. older {:.2} d/pt over the last {} days ({}).",
        drift.recent_median_days_per_point,
        drift.older_median_days_per_point,
        drift.window_days,
        match drift.direction {
            DriftDirection::Up => "trending up",
            DriftDirection::Down => "trending down",
            DriftDirection::Steady => "roughly steady",
        }
    );

    view! {
        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria.clone()
             title=aria>
            <span class="text-xs text-base-content/70">"Pace drift"</span>
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
        let aria = "Switching pattern: not enough events in the last 14 days \
                    to characterise a rhythm.";
        return view! {
            <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
                 role="group"
                 aria-label=aria
                 title=aria>
                <span class="text-xs text-base-content/70">"Switching"</span>
                <span class="badge badge-sm badge-ghost italic">
                    <span class="mr-1" aria-hidden="true">"—"</span>
                    "need more data"
                </span>
            </div>
        }
        .into_any();
    };

    // Visible: median per active day + sample period.
    // The two numbers together let the user contextualise the
    // median ("4 / day, but only over 7 active days" reads quite
    // differently from "4 / day over 14 active days").
    let median_value = if s.switches_per_day_median.fract() < 0.05 {
        format!("{:.0} / active day", s.switches_per_day_median)
    } else {
        format!("{:.1} / active day", s.switches_per_day_median)
    };
    let sample_line = format!(
        "{} events over {} d",
        s.total_events_observed, s.window_days
    );

    let aria = format!(
        "Switching pattern: median {} pickups per active day ({} total events over {} days). \
         For context only — high or low here is not a quality judgement.",
        median_value, s.total_events_observed, s.window_days
    );

    view! {
        <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria.clone()
             title=aria>
            <span class="text-xs text-base-content/70">"Switching"</span>
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
