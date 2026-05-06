//! Sprint UI (0.15.0).
//!
//! Pages:
//! - `SprintsListPage` for `/teams/{slug}/sprints`
//! - `SprintNewPage` for `/teams/{slug}/sprints/new`
//! - `SprintDetailPage` for `/teams/{slug}/sprints/{id}`
//! - `SprintEditPage` for `/teams/{slug}/sprints/{id}/edit`
//!
//! ## Charts
//!
//! Two custom SVG charts ship here, both designed to be
//! descriptive rather than evaluative (V2.1 §0.2):
//!
//! - **Velocity bar chart** on the listing page. Each
//!   completed sprint contributes one bar group: completed
//!   points (filled) plus carried-over points (lighter,
//!   stacked next to it, not on top). A median reference
//!   line of completed-points across the recent window is
//!   drawn for orientation. We deliberately don't label the
//!   chart "velocity" and we don't say "increasing" or
//!   "decreasing" — the user reads the picture and forms
//!   their own view.
//!
//! - **Burndown line chart** on the detail page. Two cumulative
//!   lines — committed (rises with scope) and completed
//!   (rises with finished work). Their gap is the in-flight
//!   work. We draw no "ideal" diagonal, no projected-finish
//!   curve, and no completion-percentage readout. The chart
//!   shows the *what*; the team interprets the *so what*.
//!
//! Both charts use simple SVG with no JS. Accessible labels
//! describe what the chart shows in words for screen readers.

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::{
    CurrentUser,
    sprints::{BurndownPoint, Sprint, SprintStatus, SprintSummary},
    teams::{Team, TeamRole},
};

use super::layout::AppShell;

// ──────────────────────────────────────────────────────────────
// Listing page
// ──────────────────────────────────────────────────────────────

#[component]
#[allow(clippy::too_many_arguments)]
pub fn SprintsListPage(
    user: CurrentUser,
    team: Team,
    role: TeamRole,
    sprints: Vec<(Sprint, SprintSummary)>,
    velocity_data: Vec<(Sprint, SprintSummary)>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> impl IntoView {
    let team_slug = team.slug.clone();
    let team_name = team.name.clone();
    let is_admin = role.can_manage_team();
    let new_href = format!("/teams/{}/sprints/new", team_slug);
    let team_href = format!("/teams/{}", team_slug);
    let has_sprints = !sprints.is_empty();
    let has_velocity = velocity_data.len() >= 2;

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    let new_button = is_admin.then(|| {
        view! {
            <a href=new_href class="btn btn-primary btn-sm">"+ New sprint"</a>
        }
    });

    let velocity_chart = has_velocity.then(|| render_velocity_chart(velocity_data));

    let sprint_rows = {
        let team_slug = team_slug.clone();
        sprints
            .into_iter()
            .map(move |(s, sum)| render_sprint_card(team_slug.clone(), s, sum))
            .collect_view()
    };

    view! {
        <AppShell title=format!("Sprints — {}", team_name.clone())
                  user=user
                  flash=flash
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/teams">"Teams"</a></li>
                    <li><a href=team_href>{team_name.clone()}</a></li>
                    <li>"Sprints"</li>
                </ul></div>

                <div class="flex items-center justify-between mb-4">
                    <h1 class="text-xl font-semibold">"Sprints"</h1>
                    {new_button}
                </div>

                {error_block}

                {velocity_chart}

                {(!has_sprints).then(|| view! {
                    <div class="card bg-base-100 border border-base-300 shadow-sm">
                        <div class="card-body items-center text-center py-12">
                            <p class="text-sm text-base-content/60">
                                "No sprints yet. " {if is_admin {
                                    "Create one to start time-boxing your team's work."
                                } else {
                                    "An admin can create one when the team is ready to time-box work."
                                }}
                            </p>
                            <p class="text-xs text-base-content/50 mt-1">
                                "Sprints are optional — you can use peisear without them."
                            </p>
                        </div>
                    </div>
                })}

                {has_sprints.then(|| view! {
                    <ul class="space-y-3 mt-4" aria-label="Sprint list">
                        {sprint_rows}
                    </ul>
                })}
            </div>
        </AppShell>
    }
}

fn render_sprint_card(team_slug: String, s: Sprint, sum: SprintSummary) -> impl IntoView {
    let href = format!("/teams/{}/sprints/{}", team_slug, s.id);
    let status_class = match s.status {
        SprintStatus::Active => "badge badge-sm badge-primary",
        SprintStatus::Planned => "badge badge-sm badge-ghost",
        SprintStatus::Completed => "badge badge-sm badge-outline",
    };
    let status_label = s.status.human_name();
    let dates = format!(
        "{} → {}",
        s.starts_on.format("%Y-%m-%d"),
        s.ends_on.format("%Y-%m-%d")
    );
    let summary_text = match s.status {
        SprintStatus::Completed => format!(
            "{} of {} pt completed · {} carried over",
            sum.completed_points, sum.committed_points, sum.carried_over_points
        ),
        SprintStatus::Active => format!(
            "{} of {} pt completed · {} pt in flight",
            sum.completed_points,
            sum.committed_points,
            sum.committed_points - sum.completed_points
        ),
        SprintStatus::Planned => {
            format!("{} pt committed across {} issues", sum.committed_points, sum.committed_count)
        }
    };
    let aria = format!(
        "{} ({}, {}). {}",
        s.name, status_label, dates, summary_text
    );

    view! {
        <li>
            <a href=href
               class="card bg-base-100 border border-base-300 shadow-sm hover:bg-base-200/40 transition-colors block"
               aria-label=aria>
                <div class="card-body p-4">
                    <div class="flex items-center justify-between gap-3">
                        <div class="flex-1 min-w-0">
                            <h3 class="font-medium">{s.name.clone()}</h3>
                            <p class="text-xs text-base-content/60 mt-1">
                                {dates}
                            </p>
                            <p class="text-sm text-base-content/70 mt-1">
                                {summary_text}
                            </p>
                        </div>
                        <span class=status_class>{status_label}</span>
                    </div>
                </div>
            </a>
        </li>
    }
}

/// SVG bar chart of recent completed sprints. Two bars per
/// sprint (completed + carried-over), neutral colours, median
/// reference line. Wraps in a card with a descriptive
/// caption.
fn render_velocity_chart(data: Vec<(Sprint, SprintSummary)>) -> impl IntoView {
    // Layout constants. Tuned for a card body ~700px wide.
    let chart_w: i32 = 680;
    let chart_h: i32 = 180;
    let margin_l: i32 = 30;
    let margin_r: i32 = 12;
    let margin_t: i32 = 16;
    let margin_b: i32 = 30;
    let plot_w = chart_w - margin_l - margin_r;
    let plot_h = chart_h - margin_t - margin_b;

    let n = data.len();
    let max_val: i64 = data
        .iter()
        .map(|(_, s)| (s.completed_points + s.carried_over_points).max(s.committed_points))
        .max()
        .unwrap_or(1)
        .max(1);

    // Median of completed_points across the window. Used as
    // the reference line.
    let mut completed_vals: Vec<i64> = data.iter().map(|(_, s)| s.completed_points).collect();
    completed_vals.sort_unstable();
    let median = if completed_vals.is_empty() {
        0
    } else if completed_vals.len() % 2 == 1 {
        completed_vals[completed_vals.len() / 2]
    } else {
        let mid = completed_vals.len() / 2;
        (completed_vals[mid - 1] + completed_vals[mid]) / 2
    };
    let median_y = margin_t + plot_h - ((median as f64 / max_val as f64) * plot_h as f64) as i32;

    // Bar groups (one per sprint).
    let group_w = plot_w / n.max(1) as i32;
    let bar_w = (group_w / 3).max(8);
    let gap = ((group_w - bar_w * 2) / 3).max(2);

    let bars = data
        .iter()
        .enumerate()
        .map(|(i, (sprint, summ))| {
            let x0 = margin_l + i as i32 * group_w + gap;
            let completed_h =
                ((summ.completed_points as f64 / max_val as f64) * plot_h as f64) as i32;
            let carried_h =
                ((summ.carried_over_points as f64 / max_val as f64) * plot_h as f64) as i32;
            let completed_y = margin_t + plot_h - completed_h;
            let carried_y = margin_t + plot_h - carried_h;
            let label_y = margin_t + plot_h + 14;
            let name = sprint.name.clone();
            let aria = format!(
                "{}: {} pt completed, {} pt carried over",
                name, summ.completed_points, summ.carried_over_points
            );

            view! {
                <g aria-label=aria>
                    <rect x=x0 y=completed_y width=bar_w height=completed_h
                          fill="oklch(60% 0.12 240)" rx="2"/>
                    <rect x=x0 + bar_w + gap y=carried_y width=bar_w height=carried_h
                          fill="oklch(80% 0.04 240)" rx="2"/>
                    <text x=x0 + bar_w + gap / 2 y=label_y
                          font-size="10" fill="currentColor"
                          opacity="0.6" text-anchor="middle">
                        {name}
                    </text>
                </g>
            }
        })
        .collect_view();

    // Y-axis label for max value.
    let y_max_label_y = margin_t + 4;
    let median_label_y = median_y - 4;

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mb-4"
                 aria-label="Recent completed sprints">
            <div class="card-body">
                <h2 class="text-base font-medium">"Completed work this period"</h2>
                <p class="text-xs text-base-content/60">
                    "Each pair of bars: " <strong>"completed"</strong>
                    " (filled) and " <strong>"carried over"</strong>
                    " (light). The dotted line is the median completed across these sprints. \
                     Numbers describe what happened — they don't grade it."
                </p>
                <div role="img" aria-label="Bar chart of recent sprint outcomes">
                    <svg viewBox=format!("0 0 {} {}", chart_w, chart_h)
                         xmlns="http://www.w3.org/2000/svg"
                         class="w-full h-auto">
                        // Y-axis tick (max).
                        <text x="4" y=y_max_label_y
                              font-size="10" fill="currentColor" opacity="0.5">
                            {max_val.to_string()}
                        </text>
                        // Baseline.
                        <line x1=margin_l y1=margin_t + plot_h
                              x2=chart_w - margin_r y2=margin_t + plot_h
                              stroke="currentColor" stroke-opacity="0.3"/>
                        // Median reference.
                        <line x1=margin_l y1=median_y
                              x2=chart_w - margin_r y2=median_y
                              stroke="currentColor" stroke-opacity="0.5"
                              stroke-dasharray="3 3"/>
                        <text x=chart_w - margin_r y=median_label_y
                              font-size="10" fill="currentColor"
                              opacity="0.6" text-anchor="end">
                            {format!("median {}", median)}
                        </text>
                        {bars}
                    </svg>
                </div>
            </div>
        </section>
    }
}

// ──────────────────────────────────────────────────────────────
// New sprint page
// ──────────────────────────────────────────────────────────────

#[component]
pub fn SprintNewPage(
    user: CurrentUser,
    team: Team,
    unread_count: i64,
    error: Option<String>,
) -> impl IntoView {
    let team_slug = team.slug.clone();
    let team_name = team.name.clone();
    let action = format!("/teams/{}/sprints", team_slug);
    let team_href = format!("/teams/{}", team_slug);
    let sprints_href = format!("/teams/{}/sprints", team_slug);

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    view! {
        <AppShell title="New sprint".to_string()
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/teams">"Teams"</a></li>
                    <li><a href=team_href>{team_name}</a></li>
                    <li><a href=sprints_href>"Sprints"</a></li>
                    <li>"New"</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">"New sprint"</h1>

                {error_block}

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Name"</span>
                            </div>
                            <input type="text" name="name" required=true maxlength="120" autofocus=true
                                   placeholder="e.g. Sprint 5"
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <div class="grid grid-cols-2 gap-3">
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"Start date"</span>
                                </div>
                                <input type="date" name="starts_on" required=true
                                       class="input input-bordered input-sm w-full"/>
                            </label>
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"End date"</span>
                                </div>
                                <input type="date" name="ends_on" required=true
                                       class="input input-bordered input-sm w-full"/>
                            </label>
                        </div>
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Goal"</span>
                                <span class="label-text-alt text-xs opacity-60">"optional"</span>
                            </div>
                            <textarea name="goal" rows="3" maxlength="500"
                                      placeholder="What's this sprint trying to achieve?"
                                      class="textarea textarea-bordered textarea-sm w-full"></textarea>
                        </label>
                        <p class="text-xs text-base-content/60">
                            "The sprint will be created in " <strong>"planned"</strong>
                            " state. Start it explicitly when you're ready — the calendar \
                             dates don't auto-promote."
                        </p>
                        <div class="card-actions justify-end mt-2">
                            <button type="submit" class="btn btn-primary btn-sm">"Create sprint"</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

// ──────────────────────────────────────────────────────────────
// Detail page
// ──────────────────────────────────────────────────────────────

#[component]
#[allow(clippy::too_many_arguments)]
pub fn SprintDetailPage(
    user: CurrentUser,
    team: Team,
    role: TeamRole,
    sprint: Sprint,
    summary: SprintSummary,
    issues: Vec<(String, String, String, Option<i64>, String)>,
    burndown: Vec<BurndownPoint>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> impl IntoView {
    let team_slug = team.slug.clone();
    let team_name = team.name.clone();
    let team_href = format!("/teams/{}", team_slug);
    let sprints_href = format!("/teams/{}/sprints", team_slug);
    let edit_href = format!("/teams/{}/sprints/{}/edit", team_slug, sprint.id);
    let start_action = format!("/teams/{}/sprints/{}/start", team_slug, sprint.id);
    let complete_action = format!("/teams/{}/sprints/{}/complete", team_slug, sprint.id);
    let delete_action = format!("/teams/{}/sprints/{}/delete", team_slug, sprint.id);

    let is_admin = role.can_manage_team();
    let sprint_name = sprint.name.clone();
    let sprint_status = sprint.status;
    let dates = format!(
        "{} → {}",
        sprint.starts_on.format("%Y-%m-%d"),
        sprint.ends_on.format("%Y-%m-%d")
    );
    let goal_text = sprint.goal.clone();

    let status_class = match sprint_status {
        SprintStatus::Active => "badge badge-primary",
        SprintStatus::Planned => "badge badge-ghost",
        SprintStatus::Completed => "badge badge-outline",
    };
    let status_label = sprint_status.human_name();

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    // Lifecycle action buttons (admin only).
    let lifecycle = is_admin.then(|| match sprint_status {
        SprintStatus::Planned => view! {
            <div class="flex gap-2 flex-wrap">
                <form method="post" action=start_action>
                    <button type="submit" class="btn btn-primary btn-sm"
                            aria-label="Start sprint">
                        "Start sprint"
                    </button>
                </form>
                <a href=edit_href class="btn btn-ghost btn-sm">"Edit"</a>
                <form method="post" action=delete_action.clone()
                      onsubmit="return confirm('Delete this planned sprint? \
                                                Issues currently linked to it will be unlinked.')">
                    <button type="submit" class="btn btn-ghost btn-sm text-error">"Delete"</button>
                </form>
            </div>
        }.into_any(),
        SprintStatus::Active => view! {
            <div class="flex gap-2 flex-wrap">
                <form method="post" action=complete_action>
                    <button type="submit" class="btn btn-primary btn-sm"
                            aria-label="Complete sprint">
                        "Complete sprint"
                    </button>
                </form>
                <a href=edit_href class="btn btn-ghost btn-sm">"Edit"</a>
            </div>
        }.into_any(),
        SprintStatus::Completed => view! {
            <div class="flex gap-2 flex-wrap">
                <form method="post" action=delete_action
                      onsubmit="return confirm('Delete this completed sprint? \
                                                Historical numbers will be lost.')">
                    <button type="submit" class="btn btn-ghost btn-sm text-error">"Delete"</button>
                </form>
            </div>
        }.into_any(),
    });

    let burndown_card = (!matches!(sprint_status, SprintStatus::Planned) && !burndown.is_empty())
        .then(|| render_burndown(burndown));

    let summary_card = render_summary_card(sprint_status, summary);

    let issues_table = render_issues_table(issues, sprint_status);

    view! {
        <AppShell title=sprint_name.clone()
                  user=user
                  flash=flash
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                {super::breadcrumb::render_breadcrumb(vec![
                    super::breadcrumb::BreadcrumbItem::link("Teams", "/teams"),
                    super::breadcrumb::BreadcrumbItem::link(
                        team_name,
                        team_href.clone(),
                    ),
                    super::breadcrumb::BreadcrumbItem::link("Sprints", sprints_href.clone()),
                    super::breadcrumb::BreadcrumbItem::current(sprint_name.clone()),
                ])}
                {super::breadcrumb::render_back_link("sprints", sprints_href)}

                <div class="flex items-center justify-between gap-3 mb-2">
                    <div>
                        <div class="flex items-center gap-3">
                            <h1 class="text-xl font-semibold">{sprint_name}</h1>
                            <span class=status_class>{status_label}</span>
                        </div>
                        <p class="text-sm text-base-content/70 mt-1">{dates}</p>
                        {goal_text.map(|g| view! {
                            <p class="text-sm text-base-content/80 mt-2 italic">
                                <span class="opacity-60">"Goal: "</span>{g}
                            </p>
                        })}
                    </div>
                    {lifecycle}
                </div>

                {error_block}

                {summary_card}

                {burndown_card}

                {issues_table}
            </div>
        </AppShell>
    }
}

fn render_summary_card(status: SprintStatus, sum: SprintSummary) -> impl IntoView {
    let in_flight_pt = (sum.committed_points - sum.completed_points).max(0);
    let in_flight_count = (sum.committed_count - sum.completed_count).max(0);
    let label = "Summary";

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                 aria-label=label>
            <div class="card-body">
                <h2 class="text-base font-medium">{label}</h2>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mt-2">
                    <div>
                        <div class="text-xs text-base-content/60">"Committed"</div>
                        <div class="text-lg font-semibold tabular-nums">
                            {sum.committed_points} <span class="text-sm font-normal opacity-60">"pt"</span>
                        </div>
                        <div class="text-xs text-base-content/50">
                            {format!("{} issues", sum.committed_count)}
                        </div>
                    </div>
                    <div>
                        <div class="text-xs text-base-content/60">"Completed"</div>
                        <div class="text-lg font-semibold tabular-nums">
                            {sum.completed_points} <span class="text-sm font-normal opacity-60">"pt"</span>
                        </div>
                        <div class="text-xs text-base-content/50">
                            {format!("{} issues", sum.completed_count)}
                        </div>
                    </div>
                    <div>
                        <div class="text-xs text-base-content/60">"In flight"</div>
                        <div class="text-lg font-semibold tabular-nums">
                            {in_flight_pt} <span class="text-sm font-normal opacity-60">"pt"</span>
                        </div>
                        <div class="text-xs text-base-content/50">
                            {format!("{} issues", in_flight_count)}
                        </div>
                    </div>
                    {(matches!(status, SprintStatus::Completed)).then(|| view! {
                        <div>
                            <div class="text-xs text-base-content/60">"Carried over"</div>
                            <div class="text-lg font-semibold tabular-nums">
                                {sum.carried_over_points} <span class="text-sm font-normal opacity-60">"pt"</span>
                            </div>
                            <div class="text-xs text-base-content/50">
                                {format!("{} issues", sum.carried_over_count)}
                            </div>
                        </div>
                    })}
                </div>
            </div>
        </section>
    }
}

fn render_burndown(points: Vec<BurndownPoint>) -> impl IntoView {
    let chart_w: i32 = 680;
    let chart_h: i32 = 200;
    let margin_l: i32 = 32;
    let margin_r: i32 = 12;
    let margin_t: i32 = 16;
    let margin_b: i32 = 32;
    let plot_w = chart_w - margin_l - margin_r;
    let plot_h = chart_h - margin_t - margin_b;

    let n = points.len();
    let max_val: i64 = points
        .iter()
        .map(|p| p.cumulative_committed.max(p.cumulative_completed))
        .max()
        .unwrap_or(1)
        .max(1);

    let x_for = |i: usize| -> i32 {
        if n <= 1 {
            margin_l + plot_w / 2
        } else {
            margin_l + (i as i32 * plot_w) / (n as i32 - 1)
        }
    };
    let y_for = |v: i64| -> i32 {
        margin_t + plot_h - ((v as f64 / max_val as f64) * plot_h as f64) as i32
    };

    let path_for = |key: &str| -> String {
        let mut path = String::new();
        for (i, p) in points.iter().enumerate() {
            let v = match key {
                "committed" => p.cumulative_committed,
                _ => p.cumulative_completed,
            };
            let x = x_for(i);
            let y = y_for(v);
            if i == 0 {
                path.push_str(&format!("M {} {}", x, y));
            } else {
                path.push_str(&format!(" L {} {}", x, y));
            }
        }
        path
    };

    let committed_path = path_for("committed");
    let completed_path = path_for("completed");

    let first_label = points
        .first()
        .map(|p| p.day.format("%m-%d").to_string())
        .unwrap_or_default();
    let last_label = points
        .last()
        .map(|p| p.day.format("%m-%d").to_string())
        .unwrap_or_default();
    let aria_label_text = format!(
        "Burndown chart from {} to {}, max value {}",
        first_label, last_label, max_val
    );
    let max_label_y = margin_t + 4;
    let baseline_y = margin_t + plot_h;
    let date_label_y = baseline_y + 14;

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                 aria-label="Sprint burndown">
            <div class="card-body">
                <h2 class="text-base font-medium">"Burndown"</h2>
                <p class="text-xs text-base-content/60">
                    "Two cumulative lines: " <strong>"committed"</strong>
                    " (the work added to the sprint) and " <strong>"completed"</strong>
                    " (work finished). The gap between them is in flight. \
                     No ideal line, no prediction — what's happening is what you see."
                </p>
                <div role="img" aria-label=aria_label_text>
                    <svg viewBox=format!("0 0 {} {}", chart_w, chart_h)
                         xmlns="http://www.w3.org/2000/svg"
                         class="w-full h-auto">
                        // Y-axis max label.
                        <text x="4" y=max_label_y
                              font-size="10" fill="currentColor" opacity="0.5">
                            {max_val.to_string()}
                        </text>
                        // Baseline.
                        <line x1=margin_l y1=baseline_y
                              x2=chart_w - margin_r y2=baseline_y
                              stroke="currentColor" stroke-opacity="0.3"/>
                        // Date labels.
                        <text x=margin_l y=date_label_y
                              font-size="10" fill="currentColor"
                              opacity="0.6">
                            {first_label}
                        </text>
                        <text x=chart_w - margin_r y=date_label_y
                              font-size="10" fill="currentColor"
                              opacity="0.6" text-anchor="end">
                            {last_label}
                        </text>
                        // Committed line.
                        <path d=committed_path fill="none"
                              stroke="oklch(70% 0.04 240)"
                              stroke-width="2"/>
                        // Completed line.
                        <path d=completed_path fill="none"
                              stroke="oklch(55% 0.14 240)"
                              stroke-width="2"/>
                    </svg>
                </div>
                <div class="text-xs text-base-content/60 flex gap-4 mt-2">
                    <span>
                        <span class="inline-block w-3 h-0.5 mr-1"
                              style="background: oklch(70% 0.04 240)"></span>
                        "Committed"
                    </span>
                    <span>
                        <span class="inline-block w-3 h-0.5 mr-1"
                              style="background: oklch(55% 0.14 240)"></span>
                        "Completed"
                    </span>
                </div>
            </div>
        </section>
    }
}

fn render_issues_table(
    issues: Vec<(String, String, String, Option<i64>, String)>,
    _status: SprintStatus,
) -> impl IntoView {
    let has = !issues.is_empty();
    let rows = issues
        .into_iter()
        .map(|(issue_id, project_id, title, effort, status)| {
            let href = format!("/projects/{}/issues/{}", project_id, issue_id);
            let status_label = match status.as_str() {
                "open" => "Open",
                "in_progress" => "In progress",
                "done" => "Done",
                _ => "—",
            };
            let status_class = match status.as_str() {
                "done" => "badge badge-xs badge-outline",
                "in_progress" => "badge badge-xs badge-primary",
                _ => "badge badge-xs badge-ghost",
            };
            let effort_text = effort.map(|e| format!("{} pt", e)).unwrap_or_default();
            view! {
                <tr>
                    <td>
                        <a href=href class="link link-hover">{title}</a>
                    </td>
                    <td class="tabular-nums text-sm text-base-content/70">{effort_text}</td>
                    <td>
                        <span class=status_class>{status_label}</span>
                    </td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                 aria-label="Issues in sprint">
            <div class="card-body">
                <h2 class="text-base font-medium">"Issues"</h2>
                {(!has).then(|| view! {
                    <p class="text-sm text-base-content/60 italic">
                        "No issues in this sprint yet. Open an issue and select this \
                         sprint from the sprint dropdown to add it."
                    </p>
                })}
                {has.then(|| view! {
                    <div class="overflow-x-auto">
                        <table class="table table-sm" aria-label="Sprint issues">
                            <thead>
                                <tr>
                                    <th>"Title"</th>
                                    <th>"Effort"</th>
                                    <th>"Status"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {rows}
                            </tbody>
                        </table>
                    </div>
                })}
            </div>
        </section>
    }
}

// ──────────────────────────────────────────────────────────────
// Edit page
// ──────────────────────────────────────────────────────────────

#[component]
pub fn SprintEditPage(
    user: CurrentUser,
    team: Team,
    sprint: Sprint,
    unread_count: i64,
    error: Option<String>,
) -> impl IntoView {
    let team_slug = team.slug.clone();
    let team_name = team.name.clone();
    let team_href = format!("/teams/{}", team_slug);
    let sprints_href = format!("/teams/{}/sprints", team_slug);
    let detail_href = format!("/teams/{}/sprints/{}", team_slug, sprint.id);
    let action = format!("/teams/{}/sprints/{}/edit", team_slug, sprint.id);
    let sprint_name = sprint.name.clone();
    let sprint_goal = sprint.goal.clone().unwrap_or_default();
    let starts_on = sprint.starts_on.format("%Y-%m-%d").to_string();
    let ends_on = sprint.ends_on.format("%Y-%m-%d").to_string();

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    view! {
        <AppShell title=format!("Edit {}", sprint_name.clone())
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/teams">"Teams"</a></li>
                    <li><a href=team_href>{team_name}</a></li>
                    <li><a href=sprints_href>"Sprints"</a></li>
                    <li><a href=detail_href.clone()>{sprint_name.clone()}</a></li>
                    <li>"Edit"</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">"Edit sprint"</h1>

                {error_block}

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Name"</span>
                            </div>
                            <input type="text" name="name" required=true maxlength="120"
                                   value=sprint_name
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <div class="grid grid-cols-2 gap-3">
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"Start date"</span>
                                </div>
                                <input type="date" name="starts_on" required=true
                                       value=starts_on
                                       class="input input-bordered input-sm w-full"/>
                            </label>
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"End date"</span>
                                </div>
                                <input type="date" name="ends_on" required=true
                                       value=ends_on
                                       class="input input-bordered input-sm w-full"/>
                            </label>
                        </div>
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Goal"</span>
                                <span class="label-text-alt text-xs opacity-60">"optional"</span>
                            </div>
                            <textarea name="goal" rows="3" maxlength="500"
                                      class="textarea textarea-bordered textarea-sm w-full">
                                {sprint_goal}
                            </textarea>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <a href=detail_href class="btn btn-ghost btn-sm">"Cancel"</a>
                            <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

// ──────────────────────────────────────────────────────────────
// Render entry points
// ──────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
pub fn render_list(
    user: CurrentUser,
    team: Team,
    role: TeamRole,
    sprints: Vec<(Sprint, SprintSummary)>,
    velocity_data: Vec<(Sprint, SprintSummary)>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SprintsListPage
                user=user
                team=team
                role=role
                sprints=sprints
                velocity_data=velocity_data
                unread_count=unread_count
                flash=flash
                error=error
            />
        }
    })
}

pub fn render_new(
    user: CurrentUser,
    team: Team,
    unread_count: i64,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <SprintNewPage user=user team=team unread_count=unread_count error=error/> }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn render_detail(
    user: CurrentUser,
    team: Team,
    role: TeamRole,
    sprint: Sprint,
    summary: SprintSummary,
    issues: Vec<(String, String, String, Option<i64>, String)>,
    burndown: Vec<BurndownPoint>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SprintDetailPage
                user=user
                team=team
                role=role
                sprint=sprint
                summary=summary
                issues=issues
                burndown=burndown
                unread_count=unread_count
                flash=flash
                error=error
            />
        }
    })
}

pub fn render_edit(
    user: CurrentUser,
    team: Team,
    sprint: Sprint,
    unread_count: i64,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SprintEditPage
                user=user
                team=team
                sprint=sprint
                unread_count=unread_count
                error=error
            />
        }
    })
}
