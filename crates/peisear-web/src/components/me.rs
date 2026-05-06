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

    let throughput_text = format!(
        "{} done in last {PERSONAL_ACTIVITY_WINDOW_DAYS}d",
        m.recent_done_count
    );

    let stale_text = format!("{}", m.long_stale_count);
    let stale_aria_label = format!(
        "Long-stale assigned issues: {} ({}).",
        m.long_stale_count, stale_aria
    );

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
                                " — calendar days per story point on your recently-completed estimated issues. This is a coarse Phase 1 approximation; treat it as a self-reflection prompt rather than a measurement. Phase 2 will replace it with active in-progress time."
                            </p>
                        </div>
                    </details>
                </section>
            </div>
        </AppShell>
    }
    .into_any()
}

pub fn render_dashboard(
    user: CurrentUser,
    metrics: Option<PersonalMetrics>,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <PersonalDashboard
                user=user
                metrics=metrics
                flash=flash
            />
        }
    })
}
