//! Project CRUD pages: list, new, edit.

use axum::response::Html;
use leptos::prelude::*;

use super::layout::AppShell;
use peisear_core::{CurrentUser, Project};

/// Index page listing the user's projects. Empty-state and grid
/// layouts are both supported.
#[component]
pub fn ProjectsListPage(
    user: CurrentUser,
    projects: Vec<Project>,
    flash: Option<String>,
) -> impl IntoView {
    let is_empty = projects.is_empty();
    view! {
        <AppShell title="Projects — Issue Tracker" user=user flash=flash>
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="text-2xl font-semibold tracking-tight">"Projects"</h1>
                    <p class="text-sm text-base-content/60">"Your issue-tracking workspaces"</p>
                </div>
                <a href="/projects/new" class="btn btn-primary btn-sm">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none"
                         viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
                    </svg>
                    "New project"
                </a>
            </div>

            {if is_empty {
                view! {
                    <div class="card bg-base-100 border border-base-300 border-dashed">
                        <div class="card-body items-center text-center py-12">
                            <div class="text-base-content/40 text-5xl">"◎"</div>
                            <p class="text-base-content/70 mt-2">"No projects yet."</p>
                            <a href="/projects/new" class="btn btn-primary btn-sm mt-2">
                                "Create your first project"
                            </a>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                        {projects.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect_view()}
                    </div>
                }.into_any()
            }}
        </AppShell>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    let href = format!("/projects/{}", project.id);
    let updated = project.updated_at.format("%Y-%m-%d").to_string();
    let desc = project.description.clone();
    let name = project.name.clone();
    view! {
        <a href=href class="card bg-base-100 border border-base-300 hover:border-primary hover:shadow-md transition">
            <div class="card-body p-4">
                <div class="font-medium truncate">{name}</div>
                <div class="text-xs text-base-content/60 line-clamp-2 min-h-[2rem]">
                    {if desc.is_empty() {
                        view! { <span class="italic opacity-60">"No description"</span> }.into_any()
                    } else {
                        view! { <span>{desc}</span> }.into_any()
                    }}
                </div>
                <div class="text-[11px] text-base-content/50 mt-2">
                    "Updated " {updated}
                </div>
            </div>
        </a>
    }
}

/// Blank form for creating a new project.
#[component]
pub fn ProjectNewPage(user: CurrentUser, flash: Option<String>) -> impl IntoView {
    view! {
        <AppShell title="New project — Issue Tracker" user=user flash=flash>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">"Projects"</a></li>
                    <li>"New"</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">"New project"</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action="/projects" class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Name"</span></div>
                            <input type="text" name="name" required=true maxlength="120" autofocus=true
                                   class="input input-bordered input-sm w-full"
                                   placeholder="e.g. Customer Portal"/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Description"</span></div>
                            <textarea name="description" rows="4" maxlength="4000"
                                      class="textarea textarea-bordered textarea-sm w-full"
                                      placeholder="What is this project about?"></textarea>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <a href="/projects" class="btn btn-ghost btn-sm">"Cancel"</a>
                            <button type="submit" class="btn btn-primary btn-sm">"Create project"</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

/// Edit form plus a danger-zone delete card.
#[component]
pub fn ProjectEditPage(
    user: CurrentUser,
    project: Project,
    flash: Option<String>,
) -> impl IntoView {
    let project_href = format!("/projects/{}", project.id);
    let edit_action = format!("/projects/{}/edit", project.id);
    let delete_action = format!("/projects/{}/delete", project.id);
    let name = project.name.clone();
    let name_for_breadcrumb = name.clone();
    let name_for_input = name.clone();
    let title = format!("Edit {name} — Issue Tracker");

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">"Projects"</a></li>
                    <li><a href=project_href>{name_for_breadcrumb}</a></li>
                    <li>"Edit"</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">"Edit project"</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=edit_action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Name"</span></div>
                            <input type="text" name="name" required=true maxlength="120"
                                   value=name_for_input
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Description"</span></div>
                            <textarea name="description" rows="4" maxlength="4000"
                                      class="textarea textarea-bordered textarea-sm w-full">
                                {project.description.clone()}
                            </textarea>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <a href=format!("/projects/{}", project.id) class="btn btn-ghost btn-sm">
                                "Cancel"
                            </a>
                            <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                        </div>
                    </form>
                </div>

                <div class="card bg-base-100 border border-error/30 shadow-sm mt-6">
                    <div class="card-body">
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium text-error">"Delete project"</div>
                                <div class="text-xs text-base-content/60">
                                    "Permanently remove this project and all its issues."
                                </div>
                            </div>
                            <form method="post" action=delete_action
                                  onsubmit="return confirm('Delete this project and all its issues? This cannot be undone.');">
                                <button type="submit" class="btn btn-error btn-outline btn-sm">"Delete"</button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </AppShell>
    }
}

pub fn render_projects_list(
    user: CurrentUser,
    projects: Vec<Project>,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <ProjectsListPage user=user projects=projects flash=flash/> }
    })
}

pub fn render_project_new(user: CurrentUser, flash: Option<String>) -> Html<String> {
    super::render_to_html(move || view! { <ProjectNewPage user=user flash=flash/> })
}

pub fn render_project_edit(
    user: CurrentUser,
    project: Project,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <ProjectEditPage user=user project=project flash=flash/> }
    })
}
