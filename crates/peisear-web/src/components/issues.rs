//! Project detail (board + list view) and issue CRUD pages.

use axum::response::Html;
use leptos::prelude::*;

use super::{Column, layout::AppShell};
use peisear_core::{CurrentUser, Issue, IssueStatus, Priority, Project};

/// Project-detail page: header + board/list view toggle.
#[component]
pub fn ProjectDetailPage(
    user: CurrentUser,
    project: Project,
    columns: Vec<Column>,
    view_mode: String,
    all_issues: Vec<Issue>,
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

            {if is_board {
                view! { <BoardView project_id=project_id_for_board columns=columns/> }.into_any()
            } else {
                view! { <ListView project_id=project_id_for_list issues=all_issues/> }.into_any()
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

#[component]
fn BoardView(project_id: String, columns: Vec<Column>) -> impl IntoView {
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
                                view! { <IssueCard project_id=project_id.clone() issue=issue/> }
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

#[component]
fn IssueCard(project_id: String, issue: Issue) -> impl IntoView {
    let href = format!("/projects/{}/issues/{}", project_id, issue.id);
    let badge = format!("badge badge-sm {}", issue.priority.badge_class());
    let date = issue.updated_at.format("%m-%d").to_string();
    let issue_id = issue.id.clone();
    view! {
        <a href=href
           data-issue-id=issue_id
           class="issue-card block bg-base-100 border border-base-300 hover:border-primary rounded-md p-3 shadow-sm cursor-grab active:cursor-grabbing transition"
           draggable="true">
            <div class="text-sm font-medium line-clamp-2">{issue.title}</div>
            <div class="flex items-center justify-between mt-2 text-[11px] text-base-content/60">
                <span class=badge>{issue.priority.label()}</span>
                <span>{date}</span>
            </div>
        </a>
    }
}

#[component]
fn ListView(project_id: String, issues: Vec<Issue>) -> impl IntoView {
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
                            <th class="w-32">"Updated"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {issues.into_iter().map(|issue| {
                            let href = format!("/projects/{}/issues/{}", project_id, issue.id);
                            let pri_class = format!("badge badge-sm {}", issue.priority.badge_class());
                            let updated = issue.updated_at.format("%Y-%m-%d %H:%M").to_string();
                            view! {
                                <tr class="hover">
                                    <td>
                                        <a href=href class="link link-hover font-medium">
                                            {issue.title}
                                        </a>
                                    </td>
                                    <td><span class="badge badge-sm badge-ghost">{issue.status.label()}</span></td>
                                    <td><span class=pri_class>{issue.priority.label()}</span></td>
                                    <td class="text-xs text-base-content/60">{updated}</td>
                                </tr>
                            }
                        }).collect_view()}
                        {is_empty.then(|| view! {
                            <tr>
                                <td colspan="4" class="text-center py-8 text-base-content/60 italic">
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
                        </div>

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
            />
        }
        .into_any()
    } else {
        view! {
            <IssueView
                issue=issue.clone()
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
) -> impl IntoView {
    let current_status = issue.status.as_str();
    let current_priority = issue.priority.as_str();
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
                </div>

                <div class="card-actions justify-end mt-2">
                    <a href=issue_href class="btn btn-ghost btn-sm">"Cancel"</a>
                    <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn IssueView(issue: Issue, edit_href: String, delete_action: String) -> impl IntoView {
    let pri_class = format!("badge badge-sm {}", issue.priority.badge_class());
    let created = issue.created_at.format("%Y-%m-%d %H:%M").to_string();
    let updated = issue.updated_at.format("%Y-%m-%d %H:%M").to_string();
    let has_desc = !issue.description.is_empty();
    let description = issue.description.clone();

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
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <IssueNewPage
                user=user
                project=project
                priorities=priorities
                statuses=statuses
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
                flash=flash
                editing=editing
            />
        }
    })
}
