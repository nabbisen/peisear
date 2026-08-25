//! Project CRUD pages: list, new, edit.

use axum::response::Html;
use leptos::prelude::*;

use super::layout::AppShell;
use super::t;
use peisear_core::{CurrentUser, Project};
use peisear_i18n::{Field, MessageKey};

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
        <AppShell title=t(MessageKey::ProjectsListPageTitle) user=user flash=flash>
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="text-2xl font-semibold tracking-tight">{t(MessageKey::ProjectsSectionName)}</h1>
                    <p class="text-sm text-base-content/70">{t(MessageKey::ProjectsSubheading)}</p>
                </div>
                <a href="/projects/new" class="btn btn-primary btn-sm">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none"
                         viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
                    </svg>
                    {t(MessageKey::NewProjectLabel)}
                </a>
            </div>

            {if is_empty {
                view! {
                    <div class="card bg-base-100 border border-base-300 border-dashed">
                        <div class="card-body items-center text-center py-12">
                            <div class="text-base-content/70 text-5xl">"◎"</div>
                            <p class="text-base-content/70 mt-2">{t(MessageKey::ProjectsEmptyMessage)}</p>
                            <a href="/projects/new" class="btn btn-primary btn-sm mt-2">
                                {t(MessageKey::CreateFirstProjectButton)}
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
                <div class="text-xs text-base-content/70 line-clamp-2 min-h-[2rem]">
                    {if desc.is_empty() {
                        view! { <span class="italic opacity-60">{t(MessageKey::NoDescriptionShort)}</span> }.into_any()
                    } else {
                        view! { <span>{desc}</span> }.into_any()
                    }}
                </div>
                <div class="text-[11px] text-base-content/70 mt-2">
                    {t(MessageKey::UpdatedAt { formatted: updated })}
                </div>
            </div>
        </a>
    }
}

/// Blank form for creating a new project.
///
/// `writable_teams` is the user's teams where their role lets
/// them create projects (admin or member; viewer excluded).
/// When the list is empty, the team selector is hidden — the
/// project is unambiguously personal. When non-empty, an
/// optional `<select>` lets the user pick a team or "Personal
/// (no team)".
#[component]
pub fn ProjectNewPage(
    user: CurrentUser,
    writable_teams: Vec<(peisear_core::teams::Team, peisear_core::teams::TeamRole)>,
    flash: Option<String>,
) -> impl IntoView {
    let has_teams = !writable_teams.is_empty();
    let team_options = writable_teams
        .into_iter()
        .map(|(t, _role)| {
            let id = t.id.clone();
            let name = t.name.clone();
            view! {
                <option value=id>{name}</option>
            }
        })
        .collect_view();

    view! {
        <AppShell title=t(MessageKey::ProjectNewPageTitle) user=user flash=flash>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">{t(MessageKey::ProjectsSectionName)}</a></li>
                    <li>{t(MessageKey::NewBreadcrumbWord)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">{t(MessageKey::NewProjectLabel)}</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action="/projects" class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Name })}</span></div>
                            <input type="text" name="name" required=true maxlength="120" autofocus=true
                                   class="input input-bordered input-sm w-full"
                                   placeholder=t(MessageKey::ProjectNamePlaceholder)/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span></div>
                            <textarea name="description" rows="4" maxlength="4000"
                                      class="textarea textarea-bordered textarea-sm w-full"
                                      placeholder=t(MessageKey::ProjectDescriptionPlaceholder)></textarea>
                        </label>
                        {has_teams.then(|| view! {
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::TeamFieldLabel)}</span>
                                    <span class="label-text-alt text-xs opacity-70">
                                        {t(MessageKey::OptionalHint)}
                                    </span>
                                </div>
                                <select name="team_id" class="select select-bordered select-sm w-full">
                                    <option value="">{t(MessageKey::PersonalNoTeamOption)}</option>
                                    {team_options}
                                </select>
                                <div class="label py-1">
                                    <span class="label-text-alt text-xs text-base-content/70">
                                        {t(MessageKey::TeamHelperText)}
                                    </span>
                                </div>
                            </label>
                        })}
                        <div class="card-actions justify-end mt-2">
                            <a href="/projects" class="btn btn-ghost btn-sm">{t(MessageKey::CancelButton)}</a>
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::CreateProjectButton)}</button>
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
    // `CONF-001`: the same path is `GET` for the confirmation
    // interstitial and `POST` for the delete itself, so this one
    // href serves as the originating control's link target.
    let delete_href = format!("/projects/{}/delete", project.id);
    let name = project.name.clone();
    let name_for_breadcrumb = name.clone();
    let name_for_input = name.clone();
    let title = t(MessageKey::ProjectEditPageTitle {
        project_name: name.clone(),
    });
    // Optimistic-lock guard. The handler verifies this against
    // the project's current `updated_at` per
    // peisear-feature-spec-v2.1 §21.4 and returns 409 if a
    // concurrent edit landed first.
    let client_updated_at = project.updated_at.to_rfc3339();

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">{t(MessageKey::ProjectsSectionName)}</a></li>
                    <li><a href=project_href>{name_for_breadcrumb}</a></li>
                    <li>{t(MessageKey::EditWord)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">{t(MessageKey::EditProjectHeading)}</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=edit_action class="card-body gap-3">
                        <input type="hidden" name="client_updated_at" value=client_updated_at/>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Name })}</span></div>
                            <input type="text" name="name" required=true maxlength="120"
                                   value=name_for_input
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span></div>
                            <textarea name="description" rows="4" maxlength="4000"
                                      class="textarea textarea-bordered textarea-sm w-full">
                                {project.description.clone()}
                            </textarea>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <a href=format!("/projects/{}", project.id) class="btn btn-ghost btn-sm">
                                {t(MessageKey::CancelButton)}
                            </a>
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::SaveButton)}</button>
                        </div>
                    </form>
                </div>

                <div class="card bg-base-100 border border-error/30 shadow-sm mt-6">
                    <div class="card-body">
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium text-error">{t(MessageKey::DeleteProjectHeading)}</div>
                                <div class="text-xs text-base-content/70">
                                    {t(MessageKey::DeleteProjectWarning)}
                                </div>
                            </div>
                            <a href=delete_href class="btn btn-error btn-outline btn-sm">
                                {t(MessageKey::DeleteButton)}
                            </a>
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

pub fn render_project_new(
    user: CurrentUser,
    writable_teams: Vec<(peisear_core::teams::Team, peisear_core::teams::TeamRole)>,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <ProjectNewPage user=user writable_teams=writable_teams flash=flash/> }
    })
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
