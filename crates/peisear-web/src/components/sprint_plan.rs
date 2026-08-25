//! Sprint planning page (`PLAN-001` / RFC 001).
//!
//! `SprintPlanPage` for `/teams/{slug}/sprints/{sprint_id}/plan` —
//! a two-column bulk-assign surface: a filterable, team-wide
//! backlog on the left, the sprint's currently-committed items on
//! the right, with button-driven moves between the two (no drag and
//! drop; that's Phase D-4, RFC 004).
//!
//! ## Three shapes, not two (`PLAN-001-review.md` §3.2)
//!
//! | Sprint status | Role | Backlog column | Move buttons |
//! |---|---|---|---|
//! | Planned | admin / member | shown | shown |
//! | Planned | viewer | shown | hidden |
//! | Active | any | shown | hidden |
//! | Completed | any | hidden | hidden |
//!
//! `can_move` and `show_backlog` are independent flags for exactly
//! this reason: a viewer or an active sprint suppress move forms
//! without hiding the backlog (a member reading a live plan still
//! needs to see what isn't committed yet), while a completed sprint
//! hides the backlog outright — RFC 001's own reasoning, "re-opening
//! a completed sprint to add issues is not a flow we support." The
//! sprint items column is unconditional; only the backlog and the
//! move forms vary.

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::{
    AssigneeOption, CurrentUser, Project,
    sprints::{Sprint, SprintStatus, SprintSummary},
    teams::Team,
};
use peisear_i18n::{Field, MessageKey, NavSection};
use peisear_storage::sprints::BacklogRow;

use super::t;

#[component]
#[allow(clippy::too_many_arguments)]
pub fn SprintPlanPage(
    user: CurrentUser,
    team: Team,
    sprint: Sprint,
    summary: SprintSummary,
    backlog: Vec<BacklogRow>,
    sprint_items: Vec<(String, String, String, Option<i64>, String)>,
    team_projects: Vec<Project>,
    assignees: Vec<AssigneeOption>,
    active_project: String,
    active_priority: String,
    active_assignee: String,
    can_move: bool,
    show_backlog: bool,
    unread_count: i64,
) -> impl IntoView {
    let team_slug = team.slug.clone();
    let team_name = team.name.clone();
    let team_href = format!("/teams/{}", team_slug);
    let sprints_href = format!("/teams/{}/sprints", team_slug);
    let detail_href = format!("/teams/{}/sprints/{}", team_slug, sprint.id);
    let plan_action = format!("/teams/{}/sprints/{}/plan", team_slug, sprint.id);
    let add_action = format!("/teams/{}/sprints/{}/plan/add", team_slug, sprint.id);
    let remove_action = format!("/teams/{}/sprints/{}/plan/remove", team_slug, sprint.id);
    let sprint_name = sprint.name.clone();

    let status_class = match sprint.status {
        SprintStatus::Active => "badge badge-primary",
        SprintStatus::Planned => "badge badge-ghost",
        SprintStatus::Completed => "badge badge-outline",
    };
    let status_label = t(MessageKey::SprintStatusName {
        label: sprint.status.to_i18n_label(),
    });

    let committed_total = t(MessageKey::CommittedTotalLabel {
        committed_points: summary.committed_points,
    });

    let filter_form = render_filter_form(
        plan_action,
        team_projects,
        assignees,
        active_project.clone(),
        active_priority.clone(),
        active_assignee.clone(),
    );

    let backlog_section = render_backlog(
        backlog,
        add_action,
        can_move,
        active_project,
        active_priority,
        active_assignee,
    );

    let sprint_items_section = render_sprint_items(sprint_items, remove_action, can_move);

    view! {
        <AppShell title=t(MessageKey::SprintPlanPageTitle { sprint_name: sprint_name.clone() })
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-5xl mx-auto">
                {super::breadcrumb::render_breadcrumb(vec![
                    super::breadcrumb::BreadcrumbItem::link(t(MessageKey::NavLinkTeams), "/teams"),
                    super::breadcrumb::BreadcrumbItem::link(team_name, team_href),
                    super::breadcrumb::BreadcrumbItem::link(t(MessageKey::SprintsSectionName), sprints_href.clone()),
                    super::breadcrumb::BreadcrumbItem::link(sprint_name.clone(), detail_href.clone()),
                    super::breadcrumb::BreadcrumbItem::current(t(MessageKey::SprintPlanBreadcrumbWord)),
                ])}
                {super::breadcrumb::render_back_link(NavSection::Sprints, detail_href)}

                <div class="flex items-center gap-3 mb-1">
                    <h1 class="text-xl font-semibold">{sprint_name}</h1>
                    <span class=status_class>{status_label}</span>
                </div>
                <p class="text-sm text-base-content/70 mb-4">{committed_total}</p>

                // The filter only ever narrows the backlog, so it
                // shares `show_backlog`'s gate rather than its own.
                {show_backlog.then_some(filter_form)}

                <main class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {show_backlog.then_some(backlog_section)}
                    {sprint_items_section}
                </main>
            </div>
        </AppShell>
    }
}

#[allow(clippy::too_many_arguments)]
fn render_filter_form(
    action: String,
    team_projects: Vec<Project>,
    assignees: Vec<AssigneeOption>,
    active_project: String,
    active_priority: String,
    active_assignee: String,
) -> impl IntoView + Clone + use<> {
    view! {
        <form method="get" action=action
              class="flex flex-wrap items-end gap-2 mb-4"
              aria-label=t(MessageKey::BacklogFilterAriaLabel)>
            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">{t(MessageKey::FieldLabel { field: Field::Project })}</span>
                </div>
                <select name="project" class="select select-sm select-bordered">
                    <option value="" selected=active_project.is_empty()>{t(MessageKey::AllProjectsOption)}</option>
                    {team_projects.into_iter().map(|p| {
                        let selected = active_project == p.id;
                        view! {
                            <option value=p.id.clone() selected=selected>{p.name}</option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">{t(MessageKey::FieldLabel { field: Field::Priority })}</span>
                </div>
                <select name="priority" class="select select-sm select-bordered">
                    <option value="" selected=active_priority.is_empty()>{t(MessageKey::AllPrioritiesOption)}</option>
                    {peisear_core::Priority::all().into_iter().map(|p| {
                        let p_str = p.as_str().to_string();
                        let selected = active_priority == p_str;
                        let label = t(MessageKey::PriorityName { label: p.to_i18n_label() });
                        view! {
                            <option value=p_str.clone() selected=selected>{label}</option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">{t(MessageKey::FieldLabel { field: Field::Assignee })}</span>
                </div>
                <select name="assignee" class="select select-sm select-bordered">
                    <option value="" selected=active_assignee.is_empty()>{t(MessageKey::AnyoneOption)}</option>
                    <option value="unassigned" selected={active_assignee == "unassigned"}>
                        {t(MessageKey::UnassignedOption)}
                    </option>
                    {assignees.into_iter().map(|a| {
                        let a_id = a.id.clone();
                        let selected = active_assignee == a_id;
                        view! {
                            <option value=a_id selected=selected>{a.display_name}</option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <button type="submit" class="btn btn-sm btn-primary">{t(MessageKey::ApplyButton)}</button>
        </form>
    }
}

fn row_aria(title: &str, points: Option<i64>, in_backlog: bool) -> String {
    let points = points.unwrap_or(0);
    if in_backlog {
        t(MessageKey::BacklogRowAriaLabel {
            title: title.to_string(),
            points,
        })
    } else {
        t(MessageKey::SprintItemRowAriaLabel {
            title: title.to_string(),
            points,
        })
    }
}

fn render_backlog(
    backlog: Vec<BacklogRow>,
    add_action: String,
    can_move: bool,
    active_project: String,
    active_priority: String,
    active_assignee: String,
) -> impl IntoView + use<> {
    let has = !backlog.is_empty();
    let rows = backlog
        .into_iter()
        .map(|row| {
            let issue = row.issue;
            let href = format!("/projects/{}/issues/{}", issue.project_id, issue.id);
            let points = issue.effort;
            let points_text = points
                .map(|e| t(MessageKey::PointsValue { points: e }))
                .unwrap_or_default();
            let priority_label = t(MessageKey::PriorityName {
                label: issue.priority.to_i18n_label(),
            });
            let aria = row_aria(&issue.title, points, true);

            let move_form = can_move.then(|| {
                view! {
                    <form method="post" action=add_action.clone()>
                        <input type="hidden" name="issue_id" value=issue.id.clone()/>
                        <input type="hidden" name="project_id" value=issue.project_id.clone()/>
                        <input type="hidden" name="project" value=active_project.clone()/>
                        <input type="hidden" name="priority" value=active_priority.clone()/>
                        <input type="hidden" name="assignee" value=active_assignee.clone()/>
                        <button type="submit" class="btn btn-ghost btn-xs">
                            {t(MessageKey::MoveToSprintButton)}
                        </button>
                    </form>
                }
            });

            view! {
                <li class="py-2 flex items-center justify-between gap-3" aria-label=aria>
                    <div class="min-w-0 flex-1">
                        <a href=href class="link link-hover font-medium truncate block">{issue.title}</a>
                        <div class="flex items-center gap-2 text-xs text-base-content/70 mt-0.5">
                            <span>{row.project_name}</span>
                            <span class="badge badge-xs badge-ghost">{priority_label}</span>
                            <span class="tabular-nums">{points_text}</span>
                        </div>
                    </div>
                    {move_form}
                </li>
            }
        })
        .collect_view();

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm" aria-labelledby="backlog-heading">
            <div class="card-body">
                <h2 id="backlog-heading" class="text-base font-medium">{t(MessageKey::BacklogHeading)}</h2>
                {(!has).then(|| view! {
                    <p class="text-sm text-base-content/70 italic">{t(MessageKey::NoBacklogIssuesMessage)}</p>
                })}
                {has.then(|| view! {
                    <ul class="divide-y">{rows.clone()}</ul>
                })}
            </div>
        </section>
    }
}

fn render_sprint_items(
    sprint_items: Vec<(String, String, String, Option<i64>, String)>,
    remove_action: String,
    can_move: bool,
) -> impl IntoView {
    let has = !sprint_items.is_empty();
    let rows = sprint_items
        .into_iter()
        .map(|(issue_id, project_id, title, effort, _status)| {
            let href = format!("/projects/{project_id}/issues/{issue_id}");
            let points_text = effort
                .map(|e| t(MessageKey::PointsValue { points: e }))
                .unwrap_or_default();
            let aria = row_aria(&title, effort, false);

            let move_form = can_move.then(|| {
                view! {
                    <form method="post" action=remove_action.clone()>
                        <input type="hidden" name="issue_id" value=issue_id.clone()/>
                        <button type="submit" class="btn btn-ghost btn-xs">
                            {t(MessageKey::MoveToBacklogButton)}
                        </button>
                    </form>
                }
            });

            view! {
                <li class="py-2 flex items-center justify-between gap-3" aria-label=aria>
                    <div class="min-w-0 flex-1">
                        <a href=href class="link link-hover font-medium truncate block">{title}</a>
                        <div class="text-xs text-base-content/70 mt-0.5 tabular-nums">{points_text}</div>
                    </div>
                    {move_form}
                </li>
            }
        })
        .collect_view();

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm" aria-labelledby="sprint-items-heading">
            <div class="card-body">
                <h2 id="sprint-items-heading" class="text-base font-medium">{t(MessageKey::SprintItemsHeading)}</h2>
                {(!has).then(|| view! {
                    <p class="text-sm text-base-content/70 italic">{t(MessageKey::NoSprintItemsInPlanMessage)}</p>
                })}
                {has.then(|| view! {
                    <ul class="divide-y">{rows.clone()}</ul>
                })}
            </div>
        </section>
    }
}

use super::layout::AppShell;

#[allow(clippy::too_many_arguments)]
pub fn render_plan(
    user: CurrentUser,
    team: Team,
    sprint: Sprint,
    summary: SprintSummary,
    backlog: Vec<BacklogRow>,
    sprint_items: Vec<(String, String, String, Option<i64>, String)>,
    team_projects: Vec<Project>,
    assignees: Vec<AssigneeOption>,
    active_project: String,
    active_priority: String,
    active_assignee: String,
    can_move: bool,
    show_backlog: bool,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SprintPlanPage
                user=user
                team=team
                sprint=sprint
                summary=summary
                backlog=backlog
                sprint_items=sprint_items
                team_projects=team_projects
                assignees=assignees
                active_project=active_project
                active_priority=active_priority
                active_assignee=active_assignee
                can_move=can_move
                show_backlog=show_backlog
                unread_count=unread_count
            />
        }
    })
}
