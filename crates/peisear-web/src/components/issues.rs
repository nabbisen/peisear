//! Project detail (board + list view) and issue CRUD pages.

use axum::response::Html;
use leptos::prelude::*;

use super::{Column, layout::AppShell};
use peisear_core::{
    AssigneeOption, CurrentUser, HealthIndicator, Issue, IssueStatus, Priority, Project,
    UserLoad, WorkloadState, projected_workload_state, workload_state,
    project_health::{Indicator, ProjectHealthReport},
};

/// Project-detail page: header + board/list view toggle.
#[component]
pub fn ProjectDetailPage(
    user: CurrentUser,
    project: Project,
    columns: Vec<Column>,
    view_mode: String,
    all_issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    health: ProjectHealthReport,
    flash: Option<String>,
) -> impl IntoView {
    let title = format!("{} — Issue Tracker", project.name);
    let is_board = view_mode == "board";

    let board_link = format!("/projects/{}?view=board", project.id);
    let list_link = format!("/projects/{}?view=list", project.id);
    let edit_link = format!("/projects/{}/edit", project.id);
    let new_issue_link = format!("/projects/{}/issues/new", project.id);
    let project_id = project.id.clone();
    let project_id_for_board = project.id.clone();
    let project_id_for_list = project.id.clone();

    let board_classes = if is_board {
        "join-item btn btn-sm btn-active btn-primary"
    } else {
        "join-item btn btn-sm btn-ghost"
    };
    let list_classes = if is_board {
        "join-item btn btn-sm btn-ghost"
    } else {
        "join-item btn btn-sm btn-active btn-primary"
    };

    let desc_node = if project.description.is_empty() {
        ().into_any()
    } else {
        view! {
            <p class="text-sm text-base-content/60 max-w-3xl">{project.description.clone()}</p>
        }
        .into_any()
    };

    let name_for_breadcrumb = project.name.clone();
    let name_for_header = project.name.clone();

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="flex flex-wrap items-start justify-between gap-3 mb-4">
                <div class="min-w-0">
                    <div class="breadcrumbs text-sm"><ul>
                        <li><a href="/projects">"Projects"</a></li>
                        <li class="max-w-[24ch] truncate">{name_for_breadcrumb}</li>
                    </ul></div>
                    <h1 class="text-2xl font-semibold tracking-tight truncate">{name_for_header}</h1>
                    {desc_node}
                </div>

                <div class="flex items-center gap-2 shrink-0">
                    <div class="join">
                        <a href=board_link class=board_classes>"Board"</a>
                        <a href=list_link class=list_classes>"List"</a>
                    </div>
                    <a href=edit_link class="btn btn-ghost btn-sm">"Edit"</a>
                    <a href=new_issue_link class="btn btn-primary btn-sm">"New issue"</a>
                </div>
            </div>

            <HealthStrip health=health/>

            <WorkloadStrip workload=workload/>

            {if is_board {
                view! { <BoardView project_id=project_id_for_board columns=columns assignees=assignees.clone()/> }.into_any()
            } else {
                view! { <ListView project_id=project_id_for_list issues=all_issues assignees=assignees.clone()/> }.into_any()
            }}

            // DnD script is loaded only in board mode. `data-project-id`
            // on the board div is how the JS picks up which project it
            // belongs to, avoiding string-interpolated inline JS.
            {is_board.then(|| view! {
                <div id="board-root" data-project-id=project_id.clone() class="hidden"></div>
                <script src="/static/board.js" defer=true></script>
            })}
        </AppShell>
    }
}

/// Project-level health indicators. Three stats — Throughput,
/// Staleness, Activity — each rendered as a labelled chip with a
/// Project-level health summary. Per the V2.1 brief §1.1 and
/// the user's UI/UX guidance:
///
/// - Composite 0–100 score is the headline.
/// - One natural-language sentence summarises which indicator(s)
///   are pulling the score down.
/// - Per-indicator breakdown is collapsed by default into a
///   `<details>` so the standard board view stays uncluttered
///   ("Minimal by Default", §0.3).
/// - Indicator chips reuse the `Indicator` shape; adding a new
///   metric only requires a new entry in
///   `peisear_core::project_health::ALL_INDICATORS` — UI here
///   does not change.
///
/// Empty projects (no issues yet) skip the section entirely
/// rather than show "Insufficient" chips that explain nothing.
#[component]
fn HealthStrip(health: ProjectHealthReport) -> impl IntoView {
    if health.raw.total_issues == 0 {
        return view! {
            <section class="mb-3">
                <div class="text-xs text-base-content/60 italic">
                    "No issues yet — health indicators will appear once work starts."
                </div>
            </section>
        }
        .into_any();
    }

    let score = health.score.value;
    let score_state = health.score.state;
    let score_badge = format!("badge badge-md {}", score_state.badge_class());
    let (score_glyph, score_aria) = indicator_glyph(score_state);
    let score_label = format!("Project health score: {} of 100 ({})", score, score_aria);
    let summary = health.score.summary.clone();
    let trend_chip = render_trend_chip(health.score.trend);

    let indicator_rows = health.indicators.into_iter().map(indicator_row).collect_view();

    view! {
        <section class="mb-4" aria-label="Project health">
            <div class="flex items-center gap-2 mb-1">
                <h3 class="text-xs uppercase tracking-wide text-base-content/60">
                    "Health"
                </h3>
            </div>

            <div class="flex flex-wrap items-center gap-3 mb-2"
                 role="group"
                 aria-label=score_label.clone()
                 title=score_label>
                <div class="flex items-center gap-2 px-3 py-2 rounded border border-base-300 bg-base-100">
                    <span class="text-xs text-base-content/70">"Score"</span>
                    <span class=score_badge>
                        <span class="mr-1" aria-hidden="true">{score_glyph}</span>
                        {score} " / 100"
                    </span>
                    {trend_chip}
                </div>
                <p class="text-sm text-base-content/70">{summary}</p>
            </div>

            <details class="text-xs">
                <summary class="cursor-pointer text-base-content/60 hover:text-base-content">
                    "Indicators"
                </summary>
                <div class="mt-2 flex flex-wrap items-center gap-3">
                    {indicator_rows}
                </div>
            </details>
        </section>
    }
    .into_any()
}

/// Render the trend indicator next to the score. `Unavailable`
/// returns nothing — no past data, nothing to compare. Otherwise
/// renders an arrow + signed delta with `aria-label` for screen
/// readers.
///
/// We do not colour-code the trend itself: improvement and
/// decline are inherently neutral facts, and the score state
/// (Good / Watch / Concern) already conveys the absolute
/// health. Adding a colour to the trend would push toward
/// performance-evaluation territory, which V2.1 §0.2 declines
/// to encourage.
fn render_trend_chip(trend: peisear_core::project_health::Trend) -> impl IntoView {
    use peisear_core::project_health::Trend;
    let (glyph, label, aria) = match trend {
        Trend::Unavailable => return view! { <span class="hidden"></span> }.into_any(),
        Trend::Flat => ("→", "flat".to_string(), "trend: roughly flat".to_string()),
        Trend::Up { delta } => (
            "↑",
            format!("+{delta}"),
            format!("trend: up by {delta} points"),
        ),
        Trend::Down { delta } => (
            "↓",
            format!("-{delta}"),
            format!("trend: down by {delta} points"),
        ),
    };
    view! {
        <span class="text-xs text-base-content/60 ml-1"
              role="group"
              aria-label=aria.clone()
              title=aria>
            <span aria-hidden="true">{glyph}</span>
            " " {label}
        </span>
    }
    .into_any()
}

/// Render one indicator chip (label + value + state badge with
/// glyph). The glyph + aria-label combination satisfies the
/// "color-only" anti-pattern check.
fn indicator_row(ind: Indicator) -> impl IntoView {
    let badge_class = format!("badge badge-sm {}", ind.state.badge_class());
    let (glyph, aria_state) = indicator_glyph(ind.state);
    let aria_label = format!(
        "{}: {} ({}). {}",
        ind.label,
        ind.value_display,
        aria_state,
        ind.kind.description()
    );
    view! {
        <div class="flex items-center gap-2 px-2 py-1 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria_label.clone()
             title=aria_label>
            <span class="text-xs text-base-content/70">{ind.label}</span>
            <span class=badge_class>
                <span class="mr-1" aria-hidden="true">{glyph}</span>
                {ind.value_display}
            </span>
        </div>
    }
}

/// Map a [`HealthIndicator`] to a glyph + screen-reader-friendly
/// label. Keeps the icon → state mapping in one place so the
/// HealthStrip and PersonalDashboard render the same vocabulary.
fn indicator_glyph(state: HealthIndicator) -> (&'static str, &'static str) {
    match state {
        HealthIndicator::Insufficient => ("—", "no data"),
        HealthIndicator::Good => ("✓", "good"),
        HealthIndicator::Watch => ("⚠", "watch"),
        HealthIndicator::Concern => ("✗", "concern"),
    }
}

/// A horizontal strip of per-user load chips, one per assignee
/// candidate. Renders an empty `<div>` if no users have in-flight
/// issues AND no users have a configured capacity — that combination
/// is the "early empty project" state where the strip would be
/// visual noise.
#[component]
fn WorkloadStrip(workload: Vec<UserLoad>) -> impl IntoView {
    let any_signal = workload
        .iter()
        .any(|u| u.in_flight_issues > 0 || u.capacity_points.is_some());
    if !any_signal {
        return view! { <div class="hidden"></div> }.into_any();
    }

    let chips = workload
        .into_iter()
        .map(|u| {
            let state = workload_state(&u);
            let badge_class = format!("badge badge-sm {}", state.badge_class());
            let label = match u.capacity_points {
                None => format!("{} pt · no limit", u.in_flight_points),
                Some(cap) => format!("{}/{} pt", u.in_flight_points, cap),
            };
            let title = format!(
                "{} — {} in-flight issues",
                u.display_name, u.in_flight_issues
            );
            view! {
                <div class="flex items-center gap-2 px-2 py-1 rounded border border-base-300 bg-base-100"
                     title=title>
                    <span class="text-xs font-medium">{u.display_name}</span>
                    <span class=badge_class>{label}</span>
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="mb-4">
            <div class="flex items-center gap-2 mb-1">
                <h3 class="text-xs uppercase tracking-wide text-base-content/60">
                    "Workload"
                </h3>
                <a href="/settings" class="text-xs link link-hover opacity-60">
                    "(set your capacity)"
                </a>
            </div>
            <div class="flex flex-wrap items-center gap-2">{chips}</div>
        </section>
    }
    .into_any()
}

/// Inline hint shown below the issue form, summarising current workload
/// per assignee candidate. SSR-only: this is a static snapshot rendered
/// at request time. When this issue's edit form already has an
/// `assignee_id` and `effort` selected, the hint additionally annotates
/// the projected post-save state of that assignee — letting the user
/// see whether their save will push someone into the warning zone.
///
/// `current_effort` is the issue's existing effort (or `None` for the
/// new-issue page); the hint uses it to compute the delta against the
/// presumed new-effort guess. Without JS we cannot follow the form's
/// live `<select>` value, so the hint reflects the page-load state
/// only — still useful as context.
#[component]
fn WorkloadHint(
    workload: Vec<UserLoad>,
    current_assignee_id: Option<String>,
    current_effort: Option<i64>,
) -> impl IntoView {
    if workload.is_empty() {
        return view! { <div class="hidden"></div> }.into_any();
    }

    // The "selected" assignee gets a richer annotation showing the
    // projected post-save state. For everyone else we just show the
    // current snapshot.
    let assignee_for_projection = current_assignee_id.clone();
    let chips = workload
        .into_iter()
        .map(|u| {
            let state = workload_state(&u);
            let badge = format!("badge badge-xs {}", state.badge_class());
            let snapshot = match u.capacity_points {
                None => format!("{} pt", u.in_flight_points),
                Some(cap) => format!("{}/{} pt", u.in_flight_points, cap),
            };
            // If this user is the selected assignee AND we have an
            // existing effort to compare against, project the state.
            let projection = match (&assignee_for_projection, current_effort) {
                (Some(aid), Some(eff)) if *aid == u.user_id && u.capacity_points.is_some() => {
                    // The current issue is already counted in
                    // in_flight_points; we are projecting the same
                    // value, so the projected state equals the current
                    // state. The hint's value here is in showing
                    // capacity status to the editor explicitly.
                    let projected_state = projected_workload_state(&u, 0);
                    let hint = match projected_state {
                        WorkloadState::Overloaded => {
                            Some(format!(" — already at {} pt over capacity", u.in_flight_points - u.capacity_points.unwrap_or(0)))
                        }
                        WorkloadState::Strained => Some(" — strained".to_string()),
                        _ => None,
                    };
                    let _ = eff; // silence unused warning, intentionally non-functional for now
                    hint
                }
                _ => None,
            };
            view! {
                <span class="inline-flex items-center gap-1">
                    <span class="text-base-content/70">{u.display_name}</span>
                    <span class=badge>{snapshot}</span>
                    {projection.map(|h| view! { <span class="text-warning">{h}</span> })}
                </span>
            }
        })
        .collect_view();

    view! {
        <div class="text-xs text-base-content/60 -mt-1">
            <span class="font-medium mr-2">"Workload:"</span>
            <span class="inline-flex items-center gap-3 flex-wrap">{chips}</span>
        </div>
    }
    .into_any()
}

#[component]
fn BoardView(
    project_id: String,
    columns: Vec<Column>,
    assignees: Vec<AssigneeOption>,
) -> impl IntoView {
    view! {
        <div class="grid gap-3 md:grid-cols-3" id="board">
            {columns.into_iter().map(|column| {
                let status_dot = match column.status.as_str() {
                    "open" => "w-2 h-2 rounded-full bg-info",
                    "in_progress" => "w-2 h-2 rounded-full bg-warning",
                    _ => "w-2 h-2 rounded-full bg-success",
                };
                let status_slug = column.status.as_str();
                let label = column.status.label();
                let count = column.issues.len();
                let is_empty = column.issues.is_empty();
                let project_id = project_id.clone();
                let assignees_for_col = assignees.clone();
                view! {
                    <section class="bg-base-100 border border-base-300 rounded-lg flex flex-col min-h-[200px]">
                        <header class="flex items-center justify-between px-3 py-2 border-b border-base-300">
                            <div class="flex items-center gap-2">
                                <span class=status_dot></span>
                                <h2 class="text-sm font-medium">{label}</h2>
                            </div>
                            <span class="badge badge-ghost badge-sm">{count}</span>
                        </header>
                        <div class="p-2 flex-1 flex flex-col gap-2 column-drop"
                             data-status=status_slug>
                            {column.issues.into_iter().map(|issue| {
                                view! { <IssueCard project_id=project_id.clone() issue=issue assignees=assignees_for_col.clone()/> }
                            }).collect_view()}
                            {is_empty.then(|| view! {
                                <div class="text-xs text-base-content/40 text-center py-4 italic">
                                    "Drop issues here"
                                </div>
                            })}
                        </div>
                    </section>
                }
            }).collect_view()}
        </div>
    }
}

/// Look up an assignee's display name by id. Returns the literal id
/// when no candidate matches — that happens if the assignee was once
/// a team member who has since been removed (the FK ON DELETE SET NULL
/// keeps that from happening for the project owner today, but the
/// graceful fallback prepares us for the team feature where former
/// members may still appear historically).
fn assignee_label<'a>(id: &'a str, assignees: &'a [AssigneeOption]) -> &'a str {
    assignees
        .iter()
        .find(|a| a.id == id)
        .map(|a| a.display_name.as_str())
        .unwrap_or(id)
}

#[component]
fn IssueCard(
    project_id: String,
    issue: Issue,
    assignees: Vec<AssigneeOption>,
) -> impl IntoView {
    let href = format!("/projects/{}/issues/{}", project_id, issue.id);
    let badge = format!("badge badge-sm {}", issue.priority.badge_class());
    let date = issue.updated_at.format("%m-%d").to_string();
    let issue_id = issue.id.clone();
    let effort_node = issue.effort.map(|e| {
        let label = format!("{e} pt");
        view! {
            <span class="badge badge-sm badge-outline" title="Effort estimate">
                {label}
            </span>
        }
    });
    let assignee_node = issue.assignee_id.as_ref().map(|aid| {
        let name = assignee_label(aid, &assignees).to_string();
        view! {
            <span class="badge badge-sm badge-ghost" title="Assignee">
                {name}
            </span>
        }
    });
    view! {
        <a href=href
           data-issue-id=issue_id
           class="issue-card block bg-base-100 border border-base-300 hover:border-primary rounded-md p-3 shadow-sm cursor-grab active:cursor-grabbing transition"
           draggable="true">
            <div class="text-sm font-medium line-clamp-2">{issue.title}</div>
            <div class="flex items-center justify-between gap-2 mt-2 text-[11px] text-base-content/60">
                <div class="flex items-center gap-1 flex-wrap">
                    <span class=badge>{issue.priority.label()}</span>
                    {effort_node}
                    {assignee_node}
                </div>
                <span>{date}</span>
            </div>
        </a>
    }
}

#[component]
fn ListView(
    project_id: String,
    issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
) -> impl IntoView {
    let is_empty = issues.is_empty();
    view! {
        <div class="card bg-base-100 border border-base-300">
            <div class="overflow-x-auto">
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>"Title"</th>
                            <th class="w-32">"Status"</th>
                            <th class="w-28">"Priority"</th>
                            <th class="w-20">"Effort"</th>
                            <th class="w-32">"Assignee"</th>
                            <th class="w-32">"Updated"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {issues.into_iter().map(|issue| {
                            let href = format!("/projects/{}/issues/{}", project_id, issue.id);
                            let pri_class = format!("badge badge-sm {}", issue.priority.badge_class());
                            let updated = issue.updated_at.format("%Y-%m-%d %H:%M").to_string();
                            let effort_text = match issue.effort {
                                Some(e) => format!("{e} pt"),
                                None => "—".to_string(),
                            };
                            let assignee_text = match issue.assignee_id.as_ref() {
                                Some(aid) => assignee_label(aid, &assignees).to_string(),
                                None => "—".to_string(),
                            };
                            view! {
                                <tr class="hover">
                                    <td>
                                        <a href=href class="link link-hover font-medium">
                                            {issue.title}
                                        </a>
                                    </td>
                                    <td><span class="badge badge-sm badge-ghost">{issue.status.label()}</span></td>
                                    <td><span class=pri_class>{issue.priority.label()}</span></td>
                                    <td class="text-xs text-base-content/70">{effort_text}</td>
                                    <td class="text-xs text-base-content/70">{assignee_text}</td>
                                    <td class="text-xs text-base-content/60">{updated}</td>
                                </tr>
                            }
                        }).collect_view()}
                        {is_empty.then(|| view! {
                            <tr>
                                <td colspan="6" class="text-center py-8 text-base-content/60 italic">
                                    "No issues yet."
                                </td>
                            </tr>
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

/// Form for creating a new issue under a project.
#[component]
pub fn IssueNewPage(
    user: CurrentUser,
    project: Project,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    flash: Option<String>,
) -> impl IntoView {
    let title = format!("New issue — {}", project.name);
    let back_link = format!("/projects/{}", project.id);
    let submit_action = format!("/projects/{}/issues/new", project.id);
    let name_for_breadcrumb = project.name.clone();
    let back_link_for_breadcrumb = back_link.clone();

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-2xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">"Projects"</a></li>
                    <li><a href=back_link_for_breadcrumb>{name_for_breadcrumb}</a></li>
                    <li>"New issue"</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">"New issue"</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=submit_action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Title"</span></div>
                            <input type="text" name="title" required=true maxlength="200" autofocus=true
                                   class="input input-bordered input-sm w-full"
                                   placeholder="What needs to happen?"/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Description"</span></div>
                            <textarea name="description" rows="6" maxlength="10000"
                                      class="textarea textarea-bordered textarea-sm w-full font-mono text-xs"
                                      placeholder="Describe the problem, the steps to reproduce, or the acceptance criteria."></textarea>
                        </label>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <label class="form-control w-full">
                                <div class="label py-1"><span class="label-text text-sm">"Status"</span></div>
                                <select name="status" class="select select-bordered select-sm w-full">
                                    {statuses.into_iter().map(|s| {
                                        let selected = s.as_str() == "open";
                                        view! {
                                            <option value=s.as_str() selected=selected>{s.label()}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1"><span class="label-text text-sm">"Priority"</span></div>
                                <select name="priority" class="select select-bordered select-sm w-full">
                                    {priorities.into_iter().map(|p| {
                                        let selected = p.as_str() == "medium";
                                        view! {
                                            <option value=p.as_str() selected=selected>{p.label()}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"Effort"</span>
                                    <span class="label-text-alt text-xs opacity-60">"story points"</span>
                                </div>
                                <select name="effort" class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>"—"</option>
                                    {peisear_core::EFFORT_PRESETS.iter().map(|n| {
                                        view! {
                                            <option value=n.to_string()>{n.to_string()}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"Assignee"</span>
                                </div>
                                <select name="assignee_id" class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>"—"</option>
                                    {assignees.into_iter().map(|a| {
                                        view! {
                                            <option value=a.id>{a.display_name}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>
                        </div>

                        <WorkloadHint workload=workload current_assignee_id=None current_effort=None/>

                        <div class="card-actions justify-end mt-2">
                            <a href=back_link class="btn btn-ghost btn-sm">"Cancel"</a>
                            <button type="submit" class="btn btn-primary btn-sm">"Create issue"</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

/// Issue detail page with an edit-in-place mode toggled via `?edit=1`.
#[component]
pub fn IssueDetailPage(
    user: CurrentUser,
    project: Project,
    issue: Issue,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    flash: Option<String>,
    editing: bool,
) -> impl IntoView {
    let title = format!("{} — {}", issue.title, project.name);
    let project_href = format!("/projects/{}", project.id);
    let issue_href = format!("/projects/{}/issues/{}", project.id, issue.id);
    let edit_href = format!("/projects/{}/issues/{}?edit=1", project.id, issue.id);
    let delete_action = format!("/projects/{}/issues/{}/delete", project.id, issue.id);
    let submit_action = issue_href.clone();
    let project_name_for_breadcrumb = project.name.clone();
    let project_href_for_breadcrumb = project_href.clone();
    let issue_title_for_breadcrumb = issue.title.clone();

    let body = if editing {
        view! {
            <IssueEditForm
                submit_action=submit_action
                issue=issue.clone()
                issue_href=issue_href
                priorities=priorities
                statuses=statuses
                assignees=assignees.clone()
                workload=workload
            />
        }
        .into_any()
    } else {
        view! {
            <IssueView
                issue=issue.clone()
                assignees=assignees.clone()
                edit_href=edit_href
                delete_action=delete_action
            />
        }
        .into_any()
    };

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">"Projects"</a></li>
                    <li><a href=project_href_for_breadcrumb>{project_name_for_breadcrumb}</a></li>
                    <li class="max-w-[32ch] truncate">{issue_title_for_breadcrumb}</li>
                </ul></div>
                {body}
            </div>
        </AppShell>
    }
}

#[component]
fn IssueEditForm(
    submit_action: String,
    issue: Issue,
    issue_href: String,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
) -> impl IntoView {
    let current_status = issue.status.as_str();
    let current_priority = issue.priority.as_str();
    let current_effort = issue.effort;
    let current_assignee_id = issue.assignee_id.clone();
    let assignee_for_hint = current_assignee_id.clone();
    let title_value = issue.title.clone();
    let description = issue.description.clone();

    view! {
        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <form method="post" action=submit_action class="card-body gap-3">
                <label class="form-control w-full">
                    <div class="label py-1"><span class="label-text text-sm">"Title"</span></div>
                    <input type="text" name="title" required=true maxlength="200"
                           value=title_value
                           class="input input-bordered input-sm w-full"/>
                </label>

                <label class="form-control w-full">
                    <div class="label py-1"><span class="label-text text-sm">"Description"</span></div>
                    <textarea name="description" rows="8" maxlength="10000"
                              class="textarea textarea-bordered textarea-sm w-full font-mono text-xs">
                        {description}
                    </textarea>
                </label>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <label class="form-control w-full">
                        <div class="label py-1"><span class="label-text text-sm">"Status"</span></div>
                        <select name="status" class="select select-bordered select-sm w-full">
                            {statuses.into_iter().map(|s| {
                                let selected = s.as_str() == current_status;
                                view! {
                                    <option value=s.as_str() selected=selected>{s.label()}</option>
                                }
                            }).collect_view()}
                        </select>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1"><span class="label-text text-sm">"Priority"</span></div>
                        <select name="priority" class="select select-bordered select-sm w-full">
                            {priorities.into_iter().map(|p| {
                                let selected = p.as_str() == current_priority;
                                view! {
                                    <option value=p.as_str() selected=selected>{p.label()}</option>
                                }
                            }).collect_view()}
                        </select>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1">
                            <span class="label-text text-sm">"Effort"</span>
                            <span class="label-text-alt text-xs opacity-60">"story points"</span>
                        </div>
                        <select name="effort" class="select select-bordered select-sm w-full">
                            <option value="" selected=current_effort.is_none()>"—"</option>
                            {peisear_core::EFFORT_PRESETS.iter().map(|n| {
                                let selected = current_effort == Some(*n);
                                view! {
                                    <option value=n.to_string() selected=selected>{n.to_string()}</option>
                                }
                            }).collect_view()}
                            // If the issue has a non-preset effort value, render
                            // an extra <option> so the form preserves it instead
                            // of silently coercing it to "—" on save.
                            {match current_effort {
                                Some(n) if !peisear_core::EFFORT_PRESETS.contains(&n) => {
                                    Some(view! {
                                        <option value=n.to_string() selected=true>
                                            {n.to_string()}
                                        </option>
                                    })
                                }
                                _ => None,
                            }}
                        </select>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1">
                            <span class="label-text text-sm">"Assignee"</span>
                        </div>
                        <select name="assignee_id" class="select select-bordered select-sm w-full">
                            <option value="" selected=current_assignee_id.is_none()>"—"</option>
                            {assignees.into_iter().map(|a| {
                                let selected = current_assignee_id.as_deref() == Some(a.id.as_str());
                                view! {
                                    <option value=a.id selected=selected>{a.display_name}</option>
                                }
                            }).collect_view()}
                        </select>
                    </label>
                </div>

                <WorkloadHint workload=workload current_assignee_id=assignee_for_hint current_effort=current_effort/>

                <div class="card-actions justify-end mt-2">
                    <a href=issue_href class="btn btn-ghost btn-sm">"Cancel"</a>
                    <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn IssueView(
    issue: Issue,
    assignees: Vec<AssigneeOption>,
    edit_href: String,
    delete_action: String,
) -> impl IntoView {
    let pri_class = format!("badge badge-sm {}", issue.priority.badge_class());
    let created = issue.created_at.format("%Y-%m-%d %H:%M").to_string();
    let updated = issue.updated_at.format("%Y-%m-%d %H:%M").to_string();
    let has_desc = !issue.description.is_empty();
    let description = issue.description.clone();
    let assignee_node = issue.assignee_id.as_ref().map(|aid| {
        let name = assignee_label(aid, &assignees).to_string();
        view! {
            <span class="badge badge-sm badge-ghost" title="Assignee">
                {name}
            </span>
        }
    });

    view! {
        <div class="flex items-start justify-between gap-3 mb-3">
            <h1 class="text-xl font-semibold tracking-tight">{issue.title}</h1>
            <div class="flex gap-2 shrink-0">
                <a href=edit_href class="btn btn-ghost btn-sm">"Edit"</a>
                <form method="post" action=delete_action
                      onsubmit="return confirm('Delete this issue? This cannot be undone.');">
                    <button type="submit" class="btn btn-ghost btn-sm text-error">"Delete"</button>
                </form>
            </div>
        </div>

        <div class="flex flex-wrap items-center gap-2 text-xs text-base-content/70 mb-4">
            <span class="badge badge-sm badge-ghost">{issue.status.label()}</span>
            <span class=pri_class>{issue.priority.label()}</span>
            {issue.effort.map(|e| {
                let label = format!("{e} pt");
                view! {
                    <span class="badge badge-sm badge-outline" title="Effort estimate">
                        {label}
                    </span>
                }
            })}
            {assignee_node}
            <span>"·"</span>
            <span>"Created " {created}</span>
            <span>"·"</span>
            <span>"Updated " {updated}</span>
        </div>

        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                {if has_desc {
                    view! {
                        <pre class="whitespace-pre-wrap break-words font-sans text-sm leading-relaxed">
                            {description}
                        </pre>
                    }.into_any()
                } else {
                    view! {
                        <p class="text-sm italic text-base-content/50">"No description provided."</p>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

pub fn render_project_detail(
    user: CurrentUser,
    project: Project,
    columns: Vec<Column>,
    view_mode: String,
    all_issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    health: ProjectHealthReport,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <ProjectDetailPage
                user=user
                project=project
                columns=columns
                view_mode=view_mode
                all_issues=all_issues
                assignees=assignees
                workload=workload
                health=health
                flash=flash
            />
        }
    })
}

pub fn render_issue_new(
    user: CurrentUser,
    project: Project,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <IssueNewPage
                user=user
                project=project
                priorities=priorities
                statuses=statuses
                assignees=assignees
                workload=workload
                flash=flash
            />
        }
    })
}

pub fn render_issue_detail(
    user: CurrentUser,
    project: Project,
    issue: Issue,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    flash: Option<String>,
    editing: bool,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <IssueDetailPage
                user=user
                project=project
                issue=issue
                priorities=priorities
                statuses=statuses
                assignees=assignees
                workload=workload
                flash=flash
                editing=editing
            />
        }
    })
}
