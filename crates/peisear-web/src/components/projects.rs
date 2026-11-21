//! Project CRUD pages: list, new, edit.

use axum::response::Html;
use leptos::prelude::*;

use super::layout::AppShell;
use super::{grow, grow_text, t};
use peisear_core::{CurrentUser, Project};
use peisear_i18n::{Field, MessageKey};

/// Index page listing the user's projects. Empty-state and grid
/// layouts are both supported.
#[component]
pub fn ProjectsListPage(
    user: CurrentUser,
    projects: Vec<Project>,
    flash: Option<String>,
    unread_count: i64,
) -> impl IntoView {
    let is_empty = projects.is_empty();
    view! {
        <AppShell title=t(MessageKey::ProjectsListPageTitle) user=user flash=flash unread_count=unread_count>
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="text-2xl font-semibold tracking-tight">{t(MessageKey::ProjectsSectionName)}</h1>
                    <p class="text-sm text-base-content/70">{t(MessageKey::ProjectsSubheading)}</p>
                </div>
                <a href="/projects/new" class=grow("btn btn-primary btn-sm")>
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
                            <a href="/projects/new" class=grow("btn btn-primary btn-sm mt-2")>
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
        <a href=href class=grow("card bg-base-100 border border-base-300 hover:border-primary hover:shadow-md transition")>
            <div class="card-body p-4">
                <div class="font-medium truncate">{name}</div>
                // `TYPE-003` (`§10.60`): raised to `text-sm`.
                // `min-h-[2rem]` was chosen as exactly two lines at
                // `text-xs`'s 16px line-height (2 x 16px = 32px =
                // 2rem); two lines at `text-sm`'s 20px line-height is
                // 40px = 2.5rem, not 32px -- `TYPE-001`'s own lesson
                // about a floor keyed to a line-height, on a different
                // surface. **Tried and reverted**: `min-h-[2.5rem]`
                // has no rule in either shipped stylesheet
                // (`css_scan` caught it immediately, by name) --
                // nothing in this codebase has used that exact value
                // before, and this project's CSS is vendored and
                // pinned (`ASSET-001`/`DEC-051`), not compiled from
                // source at build time, so a genuinely new arbitrary
                // value has no rule to resolve to and would silently
                // do nothing in a browser. Extending the vendored
                // stylesheet is a separate, out-of-scope asset-pipeline
                // change, not a text-size decision this round can make.
                // `min-h-[2rem]` is therefore left at its prior value,
                // the reason recorded rather than left to look like an
                // oversight: for an empty or very short description,
                // the card's own reserved space now reads as 1.6 lines
                // (32px / 20px) rather than 2 -- a minor, named
                // inconsistency. It does not matter for anything with
                // real content, confirmed by measurement: the box's
                // own true two-line height already exceeds this floor
                // regardless of its value (130px -> 138px for both the
                // typical and the long description, below).
                <div class="text-sm text-base-content/70 line-clamp-2 min-h-[2rem]">
                    {if desc.is_empty() {
                        view! { <span class="italic">{t(MessageKey::NoDescriptionShort)}</span> }.into_any()
                    } else {
                        view! { <span>{desc}</span> }.into_any()
                    }}
                </div>
                // `TYPE-001` (`§10.60`): raised from 11px to match the
                // description line above. Measured cost: none
                // -- `text-xs` carries its own explicit 16px line-height,
                // tighter than the 16.5px this 11px arbitrary value
                // inherited ambiently, so the card is marginally shorter
                // after the change, not taller, at both 390 and 1280.
                // `TYPE-003`: raised a second time this release, to
                // `text-sm` -- a direct consequence of reading one
                // screen at a time, not churn. Measured cost +4px at
                // both widths, no wrap -- a short date string.
                <div class="text-sm text-base-content/70 mt-2">
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
    unread_count: i64,
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
        <AppShell title=t(MessageKey::ProjectNewPageTitle) user=user flash=flash unread_count=unread_count>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects" class=grow_text("")>{t(MessageKey::ProjectsSectionName)}</a></li>
                    <li>{t(MessageKey::NewBreadcrumbWord)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">{t(MessageKey::NewProjectLabel)}</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action="/projects" class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Name })}</span></div>
                            <input type="text" name="name" required=true maxlength="120" autofocus=true
                                   class=grow("input input-bordered input-sm w-full")
                                   placeholder=t(MessageKey::ProjectNamePlaceholder)/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span></div>
                            <textarea name="description" rows="4" maxlength="4000"
                                      class=grow("textarea textarea-bordered textarea-sm w-full")
                                      placeholder=t(MessageKey::ProjectDescriptionPlaceholder)></textarea>
                        </label>
                        {has_teams.then(|| view! {
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::TeamFieldLabel)}</span>
                                    // `TYPE-003` (`§10.60`): raised to
                                    // `text-sm`, colour (bare
                                    // `opacity-70`) left untouched --
                                    // out of scope, `NFR-A11Y-005` not
                                    // reopened. Measured cost +4px at
                                    // both widths, no wrap -- a single
                                    // word.
                                    <span class="label-text-alt text-sm opacity-70">
                                        {t(MessageKey::OptionalHint)}
                                    </span>
                                </div>
                                <select name="team_id" class=grow("select select-bordered select-sm w-full")>
                                    <option value="">{t(MessageKey::PersonalNoTeamOption)}</option>
                                    {team_options}
                                </select>
                                <div class="label py-1">
                                    // `TYPE-003`: raised to `text-sm`.
                                    // Measured directly (not assumed
                                    // from the hint above it, a much
                                    // shorter string): this sentence
                                    // already wraps across several
                                    // lines and keeps the same line
                                    // count after raising -- 3 lines at
                                    // 390 (48px -> 60px), 2 at 1280
                                    // (32px -> 40px) -- no new wrap at
                                    // either width.
                                    <span class="label-text-alt text-sm text-base-content/70">
                                        {t(MessageKey::TeamHelperText)}
                                    </span>
                                </div>
                            </label>
                        })}
                        <div class="card-actions justify-end mt-2">
                            <a href="/projects" class=grow("btn btn-ghost btn-sm")>{t(MessageKey::CancelButton)}</a>
                            <button type="submit" class=grow("btn btn-primary btn-sm")>{t(MessageKey::CreateProjectButton)}</button>
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
    unread_count: i64,
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
        <AppShell title=title user=user flash=flash unread_count=unread_count>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects" class=grow_text("")>{t(MessageKey::ProjectsSectionName)}</a></li>
                    <li><a href=project_href class=grow_text("")>{name_for_breadcrumb}</a></li>
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
                                   class=grow("input input-bordered input-sm w-full")/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span></div>
                            <textarea name="description" rows="4" maxlength="4000"
                                      class=grow("textarea textarea-bordered textarea-sm w-full")>
                                {project.description.clone()}
                            </textarea>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <a href=format!("/projects/{}", project.id) class=grow("btn btn-ghost btn-sm")>
                                {t(MessageKey::CancelButton)}
                            </a>
                            <button type="submit" class=grow("btn btn-primary btn-sm")>{t(MessageKey::SaveButton)}</button>
                        </div>
                    </form>
                </div>

                <div class="card bg-base-100 border border-error/30 shadow-sm mt-6">
                    <div class="card-body">
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium text-error">{t(MessageKey::DeleteProjectHeading)}</div>
                                // `TYPE-003` (`§10.60`): raised to
                                // `text-sm`. Measured cost +8px at 390
                                // (two lines, both taller, same count),
                                // +4px at 1280 (one line).
                                <div class="text-sm text-base-content/70">
                                    {t(MessageKey::DeleteProjectWarning)}
                                </div>
                            </div>
                            // `CLR-001`: `btn-error` dropped (3.08:1
                            // -> 17.21:1, measured) -- this is the
                            // one site a `text-error` grep misses, the
                            // colour reaching the text through
                            // `btn-outline`+`btn-error` instead. Same
                            // spelling three other outline buttons in
                            // the product already use
                            // (`teams.rs:444`, `me.rs:251`,
                            // `sprints.rs:836`): `btn btn-outline
                            // btn-sm`. `.btn-outline:hover` inverts to
                            // a solid `base-content` background with
                            // `base-100` text (read in the pinned
                            // sheet, not assumed) -- the same two
                            // tokens already measured at 17.21:1, just
                            // swapped, so hover does not reopen this.
                            <a href=delete_href class=grow("btn btn-outline btn-sm")>
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
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <ProjectsListPage user=user projects=projects flash=flash unread_count=unread_count/> }
    })
}

pub fn render_project_new(
    user: CurrentUser,
    writable_teams: Vec<(peisear_core::teams::Team, peisear_core::teams::TeamRole)>,
    flash: Option<String>,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <ProjectNewPage user=user writable_teams=writable_teams flash=flash unread_count=unread_count/> }
    })
}

pub fn render_project_edit(
    user: CurrentUser,
    project: Project,
    flash: Option<String>,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <ProjectEditPage user=user project=project flash=flash unread_count=unread_count/> }
    })
}
