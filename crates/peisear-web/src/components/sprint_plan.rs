//! Sprint planning page (`PLAN-001` / RFC 001, `PLAN-002` / RFC 004c
//! D-4).
//!
//! `SprintPlanPage` for `/teams/{slug}/sprints/{sprint_id}/plan` —
//! a two-column bulk-assign surface: a filterable, team-wide
//! backlog on the left, the sprint's currently-committed items on
//! the right, with button-driven moves between the two, and —
//! `PLAN-002` — a row dragged from one column to the other as a
//! second affordance over the same two endpoints. The buttons are
//! the no-JavaScript path, the keyboard path and the touch path;
//! `plan.js` enhances rather than replaces them.
//!
//! ## Three shapes, not two (`PLAN-001-review.md` §3.2)
//!
//! | Sprint status | Role | Backlog column | Move buttons | Drag (`PLAN-002`) |
//! |---|---|---|---|---|
//! | Planned | admin / member | shown | shown | attached |
//! | Planned | viewer | shown | hidden | not attached |
//! | Active | any | shown | hidden | not attached |
//! | Completed | any | hidden | hidden | not attached |
//!
//! `can_move` and `show_backlog` are independent flags for exactly
//! this reason: a viewer or an active sprint suppress move forms
//! without hiding the backlog (a member reading a live plan still
//! needs to see what isn't committed yet), while a completed sprint
//! hides the backlog outright — RFC 001's own reasoning, "re-opening
//! a completed sprint to add issues is not a flow we support." The
//! sprint items column is unconditional; only the backlog and the
//! move forms vary. `plan.js`'s drag attachment reads the same
//! `can_move` flag the move buttons do (`PLAN-002` §3.3) — never a
//! second expression that happens to agree with it today.

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::{
    AssigneeOption, CurrentUser, Project,
    sprints::{Sprint, SprintStatus, SprintSummary},
    teams::Team,
};
use peisear_i18n::{Field, MessageKey, NavSection};
use peisear_storage::sprints::BacklogRow;

use super::{grow, t};

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
        add_action.clone(),
        remove_action.clone(),
        can_move,
        active_project.clone(),
        active_priority.clone(),
        active_assignee.clone(),
    );

    let sprint_items_section = render_sprint_items(
        sprint_items,
        add_action,
        remove_action,
        can_move,
        active_project,
        active_priority,
        active_assignee,
    );

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
                    // `LAYOUT-008` shape A — see the "two shapes, two
                    // remedies" note in `components.rs`. Measured at 320:
                    // `break-words` alone 504 (inert), `min-w-0` alone
                    // 418, both 0.
                    <h1 class="text-xl font-semibold min-w-0 break-words">{sprint_name}</h1>
                    <span class=status_class>{status_label}</span>
                </div>
                <p class="text-sm text-base-content/70 mb-4">{committed_total}</p>

                // `PLAN-002` §4.2: a polite and an assertive live
                // region, the same pair and the same ids `issues.rs`
                // renders for `dm.js`/`board.js` (`QA-011` §2,
                // `NFR-A11Y-008`) — a successful move announces here
                // politely; a failed undo announces assertively.
                // Rendered unconditionally, same as `issues.rs`'s
                // pair: empty and hidden until a script writes to
                // them, harmless on the three shapes where nothing
                // ever does.
                <div id="status-announcements" role="status" class="text-sm text-base-content/70 mb-2 empty:hidden"></div>
                <div id="status-announcements-assertive" role="alert" class="text-sm text-base-content/70 mb-2 empty:hidden"></div>

                // The filter only ever narrows the backlog, so it
                // shares `show_backlog`'s gate rather than its own.
                {show_backlog.then_some(filter_form)}

                <main class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {show_backlog.then_some(backlog_section)}
                    {sprint_items_section}
                </main>

                // `PLAN-002` §4.3/§4.5: the copy island and
                // `plan.js`'s own tag, gated on `can_move` (§6: "the
                // script tag asserted, and absent where it would have
                // nothing to enhance") — the other three shapes in
                // the table above render no `data-plan-move`/
                // `data-plan-drop` markers at all, so there is
                // nothing here for the script to attach to.
                {can_move.then(render_plan_copy_assets)}
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
                <select name="project" class=grow("select select-sm select-bordered")>
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
                <select name="priority" class=grow("select select-sm select-bordered")>
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
                <select name="assignee" class=grow("select select-sm select-bordered")>
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

            <button type="submit" class=grow("btn btn-sm btn-primary")>{t(MessageKey::ApplyButton)}</button>
        </form>
    }
}

/// `PLAN-002` §3.5: `plan.js`'s copy island, the same JSON-island
/// pattern `dm.js` and `board.js` use
/// (`render_status_enhancement_assets`/`render_board_copy_assets` in
/// `issues.rs`) — every string the script can show is rendered here
/// through `peisear_i18n` rather than authored in the `.js` file
/// itself (`static_js_scan` covers `static/*.js`; §6 requires no
/// allowlist entry for `plan.js`).
///
/// **Deliberately no `outcomes` block.** `dm.js`'s and `board.js`'s
/// islands carry one because both surfaces have an optimistic lock to
/// conflict over; this one does not (§1's third bullet —
/// `sprint_issues` has no `updated_at`, so there is no 409 branch to
/// classify). A future reader should find this sentence rather than a
/// missing-looking key.
///
/// Eight keys: two "moved to" announcements, the undo label, the two
/// empty-state messages (reused byte-for-byte from the no-JavaScript
/// page, not restated), one message for an undo that did not apply,
/// and — round 2 (`PLAN-002-review.md` §6c) — the two move-button
/// labels, so the script can bring a moved row's own form (its
/// fallback path) into line with its new position without authoring
/// a string: `MoveToSprintButton`/`MoveToBacklogButton` already
/// exist and render the buttons themselves, reused rather than
/// restated.
fn render_plan_copy_assets() -> impl IntoView {
    let copy = serde_json::json!({
        "movedToSprint": t(MessageKey::PlanMovedToSprintAnnouncement),
        "movedToBacklog": t(MessageKey::PlanMovedToBacklogAnnouncement),
        "undoLabel": t(MessageKey::UndoButtonLabel),
        "noBacklogIssuesMessage": t(MessageKey::NoBacklogIssuesMessage),
        "noSprintItemsMessage": t(MessageKey::NoSprintItemsInPlanMessage),
        "undoUnavailableMessage": t(MessageKey::PlanUndoUnavailableMessage),
        "sprintButtonLabel": t(MessageKey::MoveToSprintButton),
        "backlogButtonLabel": t(MessageKey::MoveToBacklogButton),
    })
    .to_string();

    view! {
        <script type="application/json" id="plan-copy" inner_html=copy></script>
        <script src="/static/plan.js" defer=true></script>
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

/// `PLAN-002` §3.3/§4.1: the row and column markers `plan.js` reads,
/// every one of them gated on the same `can_move` the move buttons
/// read — never a second expression that happens to agree with it
/// today. `None` on any of these omits the attribute entirely, which
/// is what makes the four-shape table in the module doc true of the
/// rendered markup, not just of the buttons.
///
/// - A row's `data-plan-move` names the action dropping it elsewhere
///   performs — `"add"` for a backlog row, `"remove"` for a sprint
///   item — and doubles as the value a destination column's
///   `data-plan-drop` must match for the drop to be accepted, which
///   is also what makes dropping a row back onto its own column a
///   no-op: a backlog row's `"add"` never matches the backlog
///   column's own `data-plan-drop="remove"`.
/// - `data-plan-project-id` is carried on *every* draggable row,
///   backlog or sprint, because undoing a remove needs it
///   (`plan_add`'s handler requires `project_id`) and the row's own
///   form doesn't always have it — `PlanRemoveForm` has no such
///   field, so a sprint row moved to the backlog and then undone has
///   nowhere else to read it from.
/// - `data-plan-list`/`data-plan-empty` mark the `<ul>` and the
///   empty-state `<p>`, whichever one SSR rendered (`§2d`: exactly
///   one of the two exists at load) — `plan.js` needs to find and
///   replace whichever is there when a column's emptiness changes
///   (§4's "handle the empty and no-longer-empty cases").
fn render_backlog(
    backlog: Vec<BacklogRow>,
    add_action: String,
    remove_action: String,
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
                        <button type="submit" class=grow("btn btn-ghost btn-xs")>
                            {t(MessageKey::MoveToSprintButton)}
                        </button>
                    </form>
                }
            });

            // `PLAN-002` §3.2: the row (`<li>`), not a new grip
            // control, is the drag source; `draggable="false"` on the
            // inner `<a>` is the fix for the nested-drag-source trap
            // `board.js` already paid for (`DEV-002-005-review.md`
            // §1.3) -- an `<a href>` is draggable by browser default.
            let draggable = can_move.then_some("true");
            let link_draggable = can_move.then_some("false");
            let plan_issue_id = can_move.then(|| issue.id.clone());
            let plan_project_id = can_move.then(|| issue.project_id.clone());
            let plan_move = can_move.then_some("add");

            view! {
                <li class="py-2 flex items-center justify-between gap-3" aria-label=aria
                    draggable=draggable
                    data-plan-issue-id=plan_issue_id
                    data-plan-project-id=plan_project_id
                    data-plan-move=plan_move>
                    <div class="min-w-0 flex-1">
                        <a href=href draggable=link_draggable
                           class=grow("link link-hover font-medium truncate flex items-center")>{issue.title}</a>
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

    // `PLAN-002` §4.1: the backlog column is the drop target for a
    // sprint row (`data-plan-move="remove"`), so its own marker is
    // `data-plan-drop="remove"` and its URL is `remove_action` --
    // `plan.js` reads this to undo a just-applied add, too (the
    // opposite column's URL from wherever a row currently sits).
    //
    // Round 2 (`PLAN-002-review.md` §2/§6a): `data-plan-row-move` is
    // the value a row *sitting in this column* carries -- `"add"`
    // here, because a backlog row's next move is always an add. This
    // is deliberately not the same value as `data-plan-drop`
    // (`"remove"`) -- they are opposites on every column by
    // construction, and the review's defect was the script computing
    // one from the other by hand instead of reading this fact
    // straight from the server, which already renders both.
    let plan_drop = can_move.then_some("remove");
    let plan_row_move = can_move.then_some("add");
    let plan_url = can_move.then_some(remove_action);
    let plan_list_marker = can_move.then_some("");
    let plan_empty_marker = can_move.then_some("");

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm" aria-labelledby="backlog-heading">
            <div class="card-body" data-plan-drop=plan_drop data-plan-row-move=plan_row_move data-plan-url=plan_url>
                <h2 id="backlog-heading" class="text-base font-medium">{t(MessageKey::BacklogHeading)}</h2>
                {(!has).then(|| view! {
                    <p class="text-sm text-base-content/70 italic" data-plan-empty=plan_empty_marker>
                        {t(MessageKey::NoBacklogIssuesMessage)}
                    </p>
                })}
                {has.then(|| view! {
                    <ul class="divide-y" data-plan-list=plan_list_marker>{rows.clone()}</ul>
                })}
            </div>
        </section>
    }
}

fn render_sprint_items(
    sprint_items: Vec<(String, String, String, Option<i64>, String)>,
    add_action: String,
    remove_action: String,
    can_move: bool,
    active_project: String,
    active_priority: String,
    active_assignee: String,
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

            // `PLAN-002` §2c/§4.6: this form used to omit the three
            // filter fields the add form already carries, so a
            // button-driven remove silently dropped the backlog
            // filter. `PlanRemoveForm` already accepts all three
            // (`#[serde(default)]`) -- only the markup was short.
            let move_form = can_move.then(|| {
                view! {
                    <form method="post" action=remove_action.clone()>
                        <input type="hidden" name="issue_id" value=issue_id.clone()/>
                        <input type="hidden" name="project" value=active_project.clone()/>
                        <input type="hidden" name="priority" value=active_priority.clone()/>
                        <input type="hidden" name="assignee" value=active_assignee.clone()/>
                        <button type="submit" class=grow("btn btn-ghost btn-xs")>
                            {t(MessageKey::MoveToBacklogButton)}
                        </button>
                    </form>
                }
            });

            let draggable = can_move.then_some("true");
            let link_draggable = can_move.then_some("false");
            let plan_issue_id = can_move.then(|| issue_id.clone());
            let plan_project_id = can_move.then(|| project_id.clone());
            let plan_move = can_move.then_some("remove");

            view! {
                <li class="py-2 flex items-center justify-between gap-3" aria-label=aria
                    draggable=draggable
                    data-plan-issue-id=plan_issue_id
                    data-plan-project-id=plan_project_id
                    data-plan-move=plan_move>
                    <div class="min-w-0 flex-1">
                        <a href=href draggable=link_draggable
                           class=grow("link link-hover font-medium truncate flex items-center")>{title}</a>
                        <div class="text-xs text-base-content/70 mt-0.5 tabular-nums">{points_text}</div>
                    </div>
                    {move_form}
                </li>
            }
        })
        .collect_view();

    // The sprint column is the drop target for a backlog row
    // (`data-plan-move="add"`), so its marker is
    // `data-plan-drop="add"` and its URL is `add_action`. Round 2:
    // `data-plan-row-move="remove"` -- a row sitting in the sprint
    // always moves away by a remove. See the backlog column's own
    // comment above for why this is a second attribute rather than
    // the script inverting `data-plan-drop`.
    let plan_drop = can_move.then_some("add");
    let plan_row_move = can_move.then_some("remove");
    let plan_url = can_move.then_some(add_action);
    let plan_list_marker = can_move.then_some("");
    let plan_empty_marker = can_move.then_some("");

    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm" aria-labelledby="sprint-items-heading">
            <div class="card-body" data-plan-drop=plan_drop data-plan-row-move=plan_row_move data-plan-url=plan_url>
                <h2 id="sprint-items-heading" class="text-base font-medium">{t(MessageKey::SprintItemsHeading)}</h2>
                {(!has).then(|| view! {
                    <p class="text-sm text-base-content/70 italic" data-plan-empty=plan_empty_marker>
                        {t(MessageKey::NoSprintItemsInPlanMessage)}
                    </p>
                })}
                {has.then(|| view! {
                    <ul class="divide-y" data-plan-list=plan_list_marker>{rows.clone()}</ul>
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
