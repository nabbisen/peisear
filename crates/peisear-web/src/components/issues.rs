//! Project detail (board + list view) and issue CRUD pages.

use axum::response::Html;
use leptos::prelude::*;

use super::{Column, layout::AppShell};
use peisear_core::{
    AssigneeOption, CurrentUser, DisplayHealthState, Issue, IssueStatus, Priority, Project,
    UserLoad, WorkloadState,
    project_health::{HealthScore, Indicator, ProjectHealthReport},
    projected_workload_state, workload_state,
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
    /// Currently-applied status filter, e.g. "open" /
    /// "in_progress" / "done". Empty string = no filter.
    active_status: String,
    /// Currently-applied assignee filter: a user id, the literal
    /// string "unassigned", or empty for no filter.
    active_assignee: String,
    /// Currently-applied sort key: "priority" / "created" /
    /// "updated". Empty string = storage-default order.
    active_sort: String,
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

    let breadcrumb = super::breadcrumb::render_breadcrumb(vec![
        super::breadcrumb::BreadcrumbItem::link("Projects", "/projects"),
        super::breadcrumb::BreadcrumbItem::current(name_for_breadcrumb),
    ]);
    let back_link = super::breadcrumb::render_back_link("Projects", "/projects");

    view! {
        <AppShell title=title user=user flash=flash>
            {breadcrumb}
            {back_link}
            <div class="flex flex-wrap items-start justify-between gap-3 mb-4">
                <div class="min-w-0">
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

            // Populated by board.js when a status-change request is
            // rejected or conflicts; empty (and hidden) otherwise.
            {is_board.then(|| view! {
                <div id="board-status" role="status" class="text-sm text-base-content/70 mb-2 empty:hidden"></div>
            })}

            {if is_board {
                view! { <BoardView project_id=project_id_for_board columns=columns assignees=assignees.clone()/> }.into_any()
            } else {
                view! {
                    <ListView
                        project_id=project_id_for_list
                        issues=all_issues
                        assignees=assignees.clone()
                        active_status=active_status
                        active_assignee=active_assignee
                        active_sort=active_sort
                    />
                }.into_any()
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

    // §17.1 / FR-HLT-008: no headline score, no 0-100 figure
    // anywhere in health presentation. The composite keeps its
    // state badge, trend chip, and summary sentence — it just
    // renders as one more chip alongside the individual
    // indicators (composite_row below) instead of a separate,
    // more prominent box carrying a number.
    let summary = health.score.summary.clone();

    // Phase B PR3 (B-2): explainability — collect human-language
    // sentences describing each indicator that's not at Good.
    // The list is computed before consuming `health.indicators`
    // for the chip row below.
    let explanations: Vec<String> = health
        .indicators
        .iter()
        .filter_map(|i| i.human_explanation())
        .collect();

    let composite_chip = composite_row(&health.score);

    let indicator_rows = health
        .indicators
        .into_iter()
        .map(indicator_row)
        .collect_view();

    view! {
        <section class="mb-4" aria-label="Project health">
            <div class="flex items-center gap-2 mb-1">
                <h3 class="text-xs uppercase tracking-wide text-base-content/60">
                    "Health"
                </h3>
            </div>

            <p class="text-sm text-base-content/70 mb-2">{summary}</p>

            <details class="text-xs">
                <summary class="cursor-pointer text-base-content/60 hover:text-base-content">
                    "Indicators"
                </summary>
                // Phase B PR3 (B-2): human-language
                // explanation list. Each non-Good indicator
                // contributes a sentence describing what's
                // happening, in the user's own terms ("3 issues
                // haven't moved in over two weeks") rather than
                // the score's terms ("long_stale_ratio = 0.3").
                // Per decision B-E5, readability beats
                // calculation transparency.
                //
                // The list comes before the chip row so the
                // user reads the story first, then can
                // double-check against the numbers if they
                // want.
                {(!explanations.is_empty()).then(|| view! {
                    <ul class="mt-2 ml-4 list-disc text-base-content/80 leading-relaxed space-y-1">
                        {explanations.into_iter().map(|line| view! {
                            <li>{line}</li>
                        }).collect_view()}
                    </ul>
                })}
                // The composite sits first but at the same visual
                // weight as the six individual indicators —
                // FR-HLT-008 requires "alongside", not "above".
                <div class="mt-2 flex flex-wrap items-center gap-3">
                    {composite_chip}
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

/// Render the composite score as one more chip, at the same visual
/// weight as [`indicator_row`]'s output — `FR-HLT-008` requires it
/// sit alongside the individual indicators, not above them as a
/// headline, and carry no 0–100 figure. It keeps its state badge
/// (clamped to the `Watch` ceiling) and its trend chip.
fn composite_row(score: &HealthScore) -> impl IntoView {
    let state = DisplayHealthState::from(score.state);
    let badge_class = format!("badge badge-sm {}", state.badge_class());
    let (glyph, aria_state) = state.glyph();
    let aria_label = format!("Composite: {aria_state}.");
    let trend_chip = render_trend_chip(score.trend);
    view! {
        <div class="flex items-center gap-2 px-2 py-1 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria_label.clone()
             title=aria_label>
            <span class="text-xs text-base-content/70">"Composite"</span>
            <span class=badge_class>
                <span aria-hidden="true">{glyph}</span>
            </span>
            {trend_chip}
        </div>
    }
}

/// Render one indicator chip (label + value + state badge with
/// glyph). The glyph + aria-label combination satisfies the
/// "color-only" anti-pattern check.
fn indicator_row(ind: Indicator) -> impl IntoView {
    let state = DisplayHealthState::from(ind.state);
    let badge_class = format!("badge badge-sm {}", state.badge_class());
    let (glyph, aria_state) = state.glyph();
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
                        WorkloadState::Overloaded => Some(format!(
                            " — already at {} pt over capacity",
                            u.in_flight_points - u.capacity_points.unwrap_or(0)
                        )),
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
fn IssueCard(project_id: String, issue: Issue, assignees: Vec<AssigneeOption>) -> impl IntoView {
    let href = format!("/projects/{}/issues/{}", project_id, issue.id);
    let badge = format!("badge badge-sm {}", issue.priority.badge_class());
    let date = issue.updated_at.format("%m-%d").to_string();
    let issue_id = issue.id.clone();
    // The optimistic-lock value the page rendered this card with.
    // `board.js` reads it and sends it back on drop (DEV-001); the
    // keyboard status form below sends the same value in a hidden
    // field (DEV-002). §21.4 rejects a status change whose value no
    // longer matches the stored row, on either path — both go
    // through the single `apply_status_change` lock check.
    let updated_at = issue.updated_at.to_rfc3339();
    let updated_at_for_form = updated_at.clone();
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

    // Keyboard-operable status control (DEV-002, `FR-DM-002`): one
    // submit button per status other than the card's current one.
    // Plain form POST, no scripting (`DEC-021`). Each button names
    // both the issue and the target status so a screen-reader user
    // stepping through a column doesn't hear "Done, Done, Done"
    // with no idea which issue each belongs to.
    let status_form_action = format!("/projects/{}/issues/{}/status/board", project_id, issue.id);
    let title_for_status = issue.title.clone();
    let current_status = issue.status;
    let status_buttons = IssueStatus::all()
        .into_iter()
        .filter(|s| *s != current_status)
        .map(|target| {
            let label = target.label();
            let aria_label = format!("Move \"{title_for_status}\" to {label}");
            view! {
                <button
                    type="submit"
                    name="status"
                    value=target.as_str()
                    class="btn btn-ghost btn-xs min-h-11 min-w-11 px-2 normal-case"
                    aria-label=aria_label>
                    {label}
                </button>
            }
        })
        .collect_view();

    view! {
        // The outer element carries the drag identity
        // (`board.js` matches `.issue-card`, reads
        // `data-issue-id`/`data-updated-at`, and drags this whole
        // node). It must be the draggable unit rather than the
        // inner `<a>`, because a `<form>` cannot nest inside an
        // `<a>` (invalid HTML) — the link and the form are siblings
        // here, both children of this div, so dragging moves them
        // together and the two controls stay paired after a drop.
        <div
            class="issue-card bg-base-100 border border-base-300 hover:border-primary rounded-md p-3 shadow-sm cursor-grab active:cursor-grabbing transition"
            data-issue-id=issue_id
            data-updated-at=updated_at
            draggable="true">
            <a href=href class="block">
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
            <form
                method="post"
                action=status_form_action
                class="flex flex-wrap items-center gap-1 mt-2 pt-2 border-t border-base-300">
                <input type="hidden" name="client_updated_at" value=updated_at_for_form/>
                {status_buttons}
            </form>
        </div>
    }
}

#[component]
fn ListView(
    project_id: String,
    issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
    /// Active status filter (empty = "all").
    active_status: String,
    /// Active assignee filter (user id, "unassigned", or empty).
    active_assignee: String,
    /// Active sort key.
    active_sort: String,
) -> impl IntoView {
    let is_empty = issues.is_empty();
    let toolbar_action = format!("/projects/{project_id}");
    let reset_href = toolbar_action.clone();
    let assignees_for_toolbar = assignees.clone();
    view! {
        <>
        // Filter & sort toolbar. Submits as a normal HTML GET
        // form: the browser appends the chosen values as query
        // parameters on the project URL, and the handler parses
        // them out of `ProjectViewQuery`. This is the URL-primary
        // half of v2.1 §4.4. The handler also persists the
        // selection as the user's saved default for this project.
        //
        // No JavaScript: a plain `<select onchange="form.submit()">`
        // would be slightly nicer UX but adds JS for marginal
        // gain. The "Apply" button keeps the page accessible to
        // any user agent.
        <form method="get" action=toolbar_action
              class="flex flex-wrap items-end gap-2 mb-3"
              aria-label="Filter and sort issues">
            // Hidden field so toolbar submission keeps us in list
            // view. Without this, picking a filter would bounce
            // the user back to the board view default.
            <input type="hidden" name="view" value="list"/>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">"Status"</span>
                </div>
                <select name="status" class="select select-sm select-bordered">
                    <option value="" selected=active_status.is_empty()>"All statuses"</option>
                    {IssueStatus::all().into_iter().map(|s| {
                        let s_str = s.as_str().to_string();
                        let selected = active_status == s_str;
                        view! {
                            <option value=s_str.clone() selected=selected>{s.label()}</option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">"Assignee"</span>
                </div>
                <select name="assignee" class="select select-sm select-bordered">
                    <option value="" selected=active_assignee.is_empty()>"Anyone"</option>
                    <option value="unassigned"
                            selected={active_assignee == "unassigned"}>
                        "Unassigned"
                    </option>
                    {assignees_for_toolbar.into_iter().map(|a| {
                        let a_id = a.id.clone();
                        let selected = active_assignee == a_id;
                        view! {
                            <option value=a_id selected=selected>
                                {a.display_name}
                            </option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">"Sort by"</span>
                </div>
                <select name="sort" class="select select-sm select-bordered">
                    <option value="" selected=active_sort.is_empty()>"Default"</option>
                    <option value="priority"
                            selected={active_sort == "priority"}>"Priority"</option>
                    <option value="created"
                            selected={active_sort == "created"}>"Recently created"</option>
                    <option value="updated"
                            selected={active_sort == "updated"}>"Recently updated"</option>
                </select>
            </label>

            <button type="submit" class="btn btn-sm btn-primary">"Apply"</button>
            // "Reset" links back to the bare list URL with no
            // filter/sort params. Per the handler logic, a bare
            // URL does NOT clear the saved server default — the
            // user would need to explicitly choose "All / Anyone
            // / Default" and click Apply to overwrite the saved
            // state. This is a deliberate trade-off: the
            // alternative ("clicking Reset wipes saved state")
            // would conflict with users navigating via generic
            // links, who would otherwise lose their filter
            // every time.
            <a href=reset_href class="btn btn-sm btn-ghost"
               aria-label="Show this list with no filter or sort applied">
                "Reset"
            </a>
        </form>

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
        </>
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

/// Form for creating a new sub-issue under a parent issue
/// (Phase C PR1, peisear-feature-spec-v2.1 §8.3).
///
/// Mirrors `IssueNewPage` but:
///
/// - Posts to the parent's `/sub-issues/new` endpoint, so the
///   handler can wire the new row to its parent at insert
///   time.
/// - Has no Sprint picker — sub-issues follow the parent's
///   sprint (decision: parent sprint follow-up rule), so
///   exposing a separate selector here would imply
///   independence the system doesn't actually grant.
/// - The breadcrumb threads through the parent: Projects →
///   Project → Parent issue → "New sub-issue", so the user
///   can navigate up either way.
/// - The "Cancel" link returns to the parent detail page,
///   not the project board, since that's where the user came
///   from.
#[component]
pub fn SubIssueNewPage(
    user: CurrentUser,
    project: Project,
    parent: Issue,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    flash: Option<String>,
) -> impl IntoView {
    let title = format!("New sub-issue — {}", parent.title);
    let project_href = format!("/projects/{}", project.id);
    let parent_href = format!("/projects/{}/issues/{}", project.id, parent.id);
    let submit_action = format!(
        "/projects/{}/issues/{}/sub-issues/new",
        project.id, parent.id
    );
    let project_name = project.name.clone();
    let parent_title = parent.title.clone();
    let parent_href_for_cancel = parent_href.clone();

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-2xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">"Projects"</a></li>
                    <li><a href=project_href>{project_name}</a></li>
                    <li><a href=parent_href>{parent_title}</a></li>
                    <li>"New sub-issue"</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-1">"New sub-issue"</h1>
                <p class="text-sm text-base-content/60 mb-4">
                    "This sub-issue follows its parent's sprint. \
                     You can give it its own assignee, status, priority, and effort."
                </p>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=submit_action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Title"</span>
                            </div>
                            <input type="text" name="title" required=true maxlength="200" autofocus=true
                                   class="input input-bordered input-sm w-full"
                                   placeholder="What needs to happen for this part?"/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Description"</span>
                            </div>
                            <textarea name="description" rows="6" maxlength="10000"
                                      class="textarea textarea-bordered textarea-sm w-full font-mono text-xs"
                                      placeholder="Describe this sub-task in more detail if useful."></textarea>
                        </label>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"Status"</span>
                                </div>
                                <select name="status"
                                        class="select select-bordered select-sm w-full">
                                    {statuses.into_iter().map(|s| {
                                        let selected = s.as_str() == "open";
                                        view! {
                                            <option value=s.as_str() selected=selected>{s.label()}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">"Priority"</span>
                                </div>
                                <select name="priority"
                                        class="select select-bordered select-sm w-full">
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
                                <select name="effort"
                                        class="select select-bordered select-sm w-full">
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
                                <select name="assignee_id"
                                        class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>"—"</option>
                                    {assignees.into_iter().map(|a| {
                                        view! {
                                            <option value=a.id>{a.display_name}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>
                        </div>

                        <div class="card-actions justify-end mt-2">
                            <a href=parent_href_for_cancel class="btn btn-ghost btn-sm">"Cancel"</a>
                            <button type="submit" class="btn btn-primary btn-sm">"Create sub-issue"</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

/// Issue detail page. Read-only by default; the parent `editing`
/// parameter switches to the edit form. Phase B PR3 (B-3) split
/// view and edit modes into separate URLs:
/// `/projects/{id}/issues/{issue_id}` for view, `/edit` suffix for
/// edit. The legacy `?edit=1` query parameter 308-redirects to
/// the new edit URL.
#[component]
pub fn IssueDetailPage(
    user: CurrentUser,
    project: Project,
    issue: Issue,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    sprint_options: Vec<(String, String)>,
    current_sprint_id: Option<String>,
    /// Sub-issues of this issue (Phase C PR1).
    sub_issues: Vec<Issue>,
    /// The parent issue if this row is a sub-issue. None for
    /// top-level issues.
    parent_issue: Option<Issue>,
    flash: Option<String>,
    editing: bool,
) -> impl IntoView {
    let title = format!("{} — {}", issue.title, project.name);
    let project_href = format!("/projects/{}", project.id);
    let issue_href = format!("/projects/{}/issues/{}", project.id, issue.id);
    // Phase B PR3 (B-3): edit URL is explicit, not a query
    // parameter. Refresh, browser-back, and "Open in new tab"
    // now consistently land on the right mode.
    let edit_href = format!("/projects/{}/issues/{}/edit", project.id, issue.id);
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

    let has_sprint_options = !sprint_options.is_empty();
    let sprint_action = format!("/projects/{}/issues/{}/sprint", project.id, issue.id);
    let current_sprint_for_select = current_sprint_id.clone();
    let sprint_options_view = sprint_options
        .into_iter()
        .map(|(id, name)| {
            let selected = current_sprint_for_select.as_deref() == Some(id.as_str());
            view! {
                <option value=id selected=selected>{name}</option>
            }
        })
        .collect_view();
    let no_sprint_selected = current_sprint_id.is_none();
    // Phase C PR1: sub-issues don't get their own sprint
    // selector — they follow the parent's sprint per
    // peisear-feature-spec-v2.1 §8.5. The sprint membership is
    // still visible on the detail page (rendered in the meta
    // row up top), so the user can see what sprint they're in;
    // they just can't change it from this page. To re-sprint
    // a sub-issue, the user changes the parent's sprint
    // (which propagates).
    let show_sprint_picker = has_sprint_options && issue.is_top_level();

    // Phase C PR1: sub-issue card. Top-level issues with no
    // children get a "+ Add sub-issue" affordance; ones with
    // children get the list. Sub-issues themselves don't show
    // this section at all (1-level rule means they can't have
    // children).
    let new_sub_issue_href = format!(
        "/projects/{}/issues/{}/sub-issues/new",
        project.id, issue.id
    );
    let sub_issues_card = if issue.is_top_level() {
        Some(view! {
            <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                     aria-label="Sub-issues">
                <div class="card-body py-3">
                    <div class="flex items-center justify-between mb-2">
                        <h2 class="text-sm font-medium">"Sub-issues"</h2>
                        <a href=new_sub_issue_href class="btn btn-ghost btn-xs">
                            "+ Add sub-issue"
                        </a>
                    </div>
                    {if sub_issues.is_empty() {
                        view! {
                            <p class="text-xs italic text-base-content/50">
                                "No sub-issues yet. Break this work into smaller pieces \
                                 if it helps you track them — they share this issue's project \
                                 and sprint, but can have their own assignee, status, and effort."
                            </p>
                        }.into_any()
                    } else {
                        let project_id_for_links = project.id.clone();
                        view! {
                            <ul class="divide-y divide-base-200">
                                {sub_issues.into_iter().map(|si| {
                                    let detail_href = format!(
                                        "/projects/{}/issues/{}",
                                        project_id_for_links, si.id
                                    );
                                    let status_badge_class = format!(
                                        "badge badge-xs {}",
                                        match si.status {
                                            IssueStatus::Open => "badge-ghost",
                                            IssueStatus::InProgress => "badge-primary",
                                            IssueStatus::Done => "badge-success",
                                        }
                                    );
                                    let aria = format!(
                                        "{}, status {}",
                                        si.title,
                                        si.status.label(),
                                    );
                                    view! {
                                        <li class="py-2 flex items-center gap-2"
                                            aria-label=aria>
                                            <span class=status_badge_class>
                                                {si.status.label()}
                                            </span>
                                            <a href=detail_href class="text-sm hover:underline flex-1">
                                                {si.title}
                                            </a>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }}
                </div>
            </section>
        })
    } else {
        None
    };

    let sprint_card = show_sprint_picker.then(|| view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                 aria-label="Sprint assignment">
            <div class="card-body py-3">
                <form method="post" action=sprint_action
                      class="flex items-center gap-2 flex-wrap">
                    <label class="text-sm font-medium" for="sprint-select">"Sprint:"</label>
                    <select id="sprint-select" name="sprint_id"
                            class="select select-bordered select-sm flex-1 min-w-[14rem]"
                            aria-label="Select sprint for this issue">
                        <option value="" selected=no_sprint_selected>"(no sprint)"</option>
                        {sprint_options_view}
                    </select>
                    <button type="submit" class="btn btn-ghost btn-sm">"Save"</button>
                </form>
                <p class="text-xs text-base-content/60 mt-1">
                    "Sprint assignment is independent from this issue's status and priority — \
                     adding to a sprint commits the work; the team decides what 'committed' means."
                </p>
            </div>
        </section>
    });

    // Phase C PR1: parent-aware breadcrumb.
    // For sub-issues, insert the parent issue as a link
    // between the project and the current issue. The user
    // sees "Projects / FooProject / Parent Title / This sub-
    // issue" and can navigate up either to the project or to
    // the parent.
    let breadcrumb_items = {
        let mut items = vec![
            super::breadcrumb::BreadcrumbItem::link("Projects", "/projects"),
            super::breadcrumb::BreadcrumbItem::link(
                project_name_for_breadcrumb,
                project_href_for_breadcrumb.clone(),
            ),
        ];
        if let Some(parent) = &parent_issue {
            let parent_href = format!("/projects/{}/issues/{}", project.id, parent.id);
            items.push(super::breadcrumb::BreadcrumbItem::link(
                parent.title.clone(),
                parent_href,
            ));
        }
        items.push(super::breadcrumb::BreadcrumbItem::current(
            issue_title_for_breadcrumb,
        ));
        items
    };

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                {super::breadcrumb::render_breadcrumb(breadcrumb_items)}
                {super::breadcrumb::render_back_link(
                    "issues",
                    project_href_for_breadcrumb,
                )}
                {body}
                {sub_issues_card}
                {sprint_card}
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
    // RFC3339 captured at render time. The handler verifies
    // this against the issue's current `updated_at` per
    // peisear-feature-spec-v2.1 §21.4 and rejects with 409 if
    // someone else has saved an edit between this render and
    // the form submission.
    let client_updated_at = issue.updated_at.to_rfc3339();

    view! {
        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <form method="post" action=submit_action class="card-body gap-3">
                // Optimistic-lock guard. If a concurrent edit
                // lands first, this stale value triggers the
                // 409 conflict path. See §21.4.
                <input type="hidden" name="client_updated_at" value=client_updated_at/>

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
                <a href=edit_href.clone() class="btn btn-ghost btn-sm">"Edit"</a>
                <form method="post" action=delete_action
                      onsubmit="return confirm('Delete this issue? This cannot be undone.');">
                    <button type="submit" class="btn btn-ghost btn-sm text-error">"Delete"</button>
                </form>
            </div>
        </div>

        // Phase B PR3 (B-4): status segment control. Three
        // mutually-exclusive segments (Open / In Progress /
        // Done), with the current status highlighted. Display
        // only — clicking does NOT mutate; the user clicks
        // Edit to change. Direct-manipulation status changes
        // come in Phase D.
        //
        // Visually: the active segment uses `btn-primary` so
        // it stands out against the inactive `btn-ghost`
        // siblings. The shared `btn-disabled` cursor keeps it
        // clear that this is a read-only display, not an
        // interactive switch.
        //
        // Accessibility: the wrapping div carries
        // `role="group"` + `aria-label`, and each segment is
        // a button with `aria-pressed` reflecting whether it
        // matches the current status. Screen readers
        // announce "Open, pressed; In Progress, not pressed;
        // Done, not pressed" so the segmented semantics carry
        // through.
        <div class="join mb-3" role="group" aria-label="Issue status">
            {IssueStatus::all().into_iter().map(|s| {
                let is_current = s == issue.status;
                let pressed = if is_current { "true" } else { "false" };
                // `btn-primary` highlights the active segment;
                // others are `btn-ghost` so they recede.
                // `cursor-default` signals "click does nothing
                // here." All segments stay enabled for screen
                // reader navigation; their `aria-pressed`
                // attribute carries the active/inactive
                // semantics. The buttons have `type="button"`
                // (not submit) and no click handler, so they
                // are inert by design.
                let cls = if is_current {
                    "join-item btn btn-sm btn-primary cursor-default"
                } else {
                    "join-item btn btn-sm btn-ghost cursor-default"
                };
                view! {
                    <button type="button"
                            class=cls
                            aria-pressed=pressed
                            tabindex="-1">
                        {s.label()}
                    </button>
                }
            }).collect_view()}
        </div>

        <div class="flex flex-wrap items-center gap-2 text-xs text-base-content/70 mb-4">
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

#[allow(clippy::too_many_arguments)]
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
    // Phase A Step 3 (v2.1 §4.4): the currently-active filter
    // and sort, computed by the handler from URL params merged
    // with server-saved defaults. Empty string means "no
    // constraint" — the UI dropdown shows that as the default
    // option.
    active_status: String,
    active_assignee: String,
    active_sort: String,
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
                active_status=active_status
                active_assignee=active_assignee
                active_sort=active_sort
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

/// Render the sub-issue creation form (Phase C PR1). Wrapper
/// around [`SubIssueNewPage`] that supplies the priority and
/// status enum lists from `peisear_core` defaults — handlers
/// don't need to know about those.
pub fn render_new_sub_issue_form(
    user: CurrentUser,
    project: Project,
    parent: Issue,
    assignees: Vec<AssigneeOption>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SubIssueNewPage
                user=user
                project=project
                parent=parent
                priorities=Priority::all().to_vec()
                statuses=IssueStatus::all().to_vec()
                assignees=assignees
                flash=None
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
    // Sprints in the project's team that the user can pick
    // from. Empty vec when the project is personal (no team)
    // or the team has no `planned`/`active` sprints.
    sprint_options: Vec<(String, String)>,
    // The sprint id this issue is currently in, if any.
    current_sprint_id: Option<String>,
    // Sub-issues of this issue (Phase C PR1). Always empty for
    // sub-issues themselves (one-level rule); may be empty for
    // top-level issues that haven't been broken down yet.
    sub_issues: Vec<Issue>,
    // The parent issue if this row is a sub-issue. Used for
    // breadcrumb context.
    parent_issue: Option<Issue>,
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
                sprint_options=sprint_options
                current_sprint_id=current_sprint_id
                sub_issues=sub_issues
                parent_issue=parent_issue
                flash=flash
                editing=editing
            />
        }
    })
}
