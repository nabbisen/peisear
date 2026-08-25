//! Teams pages (0.14.0).
//!
//! - `TeamsListPage` for `/teams` (user's teams).
//! - `TeamNewPage` for `/teams/new`.
//! - `TeamDetailPage` for `/teams/{slug}`.
//! - `TeamEditPage` for `/teams/{slug}/edit` (admin only — guarded
//!   by handler).

use axum::response::Html;
use chrono::{DateTime, Utc};
use leptos::prelude::*;
use peisear_i18n::{Field, MessageKey, TeamRoleLabel};

use peisear_core::{
    CurrentUser, Project,
    teams::{Team, TeamRole},
};

use super::layout::AppShell;
use super::t;

#[component]
pub fn TeamsListPage(
    user: CurrentUser,
    teams: Vec<(Team, TeamRole)>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> impl IntoView {
    let has_teams = !teams.is_empty();
    let team_rows = teams.into_iter().map(render_team_card).collect_view();

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    view! {
        <AppShell title=t(MessageKey::NavLinkTeams)
                  user=user
                  flash=flash
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="flex items-center justify-between mb-4">
                    <h1 class="text-xl font-semibold">{t(MessageKey::NavLinkTeams)}</h1>
                    <a href="/teams/new" class="btn btn-primary btn-sm">{t(MessageKey::NewTeamLink)}</a>
                </div>

                {error_block}

                {(!has_teams).then(|| view! {
                    <div class="card bg-base-100 border border-base-300 shadow-sm">
                        <div class="card-body items-center text-center py-12">
                            <div class="text-base-content/70 mb-2">
                                <svg xmlns="http://www.w3.org/2000/svg" width="36" height="36"
                                     viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                     stroke-width="1.5" aria-hidden="true">
                                    <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                                    <circle cx="9" cy="7" r="4"/>
                                    <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                                    <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
                                </svg>
                            </div>
                            <p class="text-sm text-base-content/70">
                                {t(MessageKey::TeamsEmptyIntro)}
                            </p>
                            <p class="text-xs text-base-content/70 mt-1">
                                {t(MessageKey::TeamsEmptyCta)}
                            </p>
                        </div>
                    </div>
                })}

                {has_teams.then(|| view! {
                    <ul class="space-y-3" aria-label=t(MessageKey::YourTeamsAriaLabel)>
                        {team_rows}
                    </ul>
                })}
            </div>
        </AppShell>
    }
}

fn render_team_card((team, role): (Team, TeamRole)) -> impl IntoView {
    let aria = t(MessageKey::TeamRoleAriaLabel {
        team_name: team.name.clone(),
        role: role.to_i18n_label(),
    });
    let role_badge_class = match role {
        TeamRole::Admin => "badge badge-sm badge-primary",
        TeamRole::Member => "badge badge-sm badge-ghost",
        TeamRole::Viewer => "badge badge-sm badge-outline",
    };

    let href = format!("/teams/{}", team.slug);
    let role_label = t(MessageKey::TeamRoleName {
        label: role.to_i18n_label(),
    });
    let description_text = team.description.clone().unwrap_or_default();
    let description = (!description_text.is_empty()).then(|| {
        view! {
            <p class="text-sm text-base-content/70 mt-1">{description_text}</p>
        }
    });

    view! {
        <li>
            <a href=href class="card bg-base-100 border border-base-300 shadow-sm hover:bg-base-200/40 transition-colors block"
               aria-label=aria>
                <div class="card-body p-4">
                    <div class="flex items-center justify-between gap-3">
                        <div class="flex-1 min-w-0">
                            <h3 class="font-medium">{team.name}</h3>
                            {description}
                        </div>
                        <span class=role_badge_class>{role_label}</span>
                    </div>
                </div>
            </a>
        </li>
    }
}

#[component]
pub fn TeamNewPage(user: CurrentUser, unread_count: i64, error: Option<String>) -> impl IntoView {
    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    view! {
        <AppShell title=t(MessageKey::NewTeamLabel)
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/teams">{t(MessageKey::NavLinkTeams)}</a></li>
                    <li>{t(MessageKey::NewBreadcrumbWord)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">{t(MessageKey::NewTeamLabel)}</h1>

                {error_block}

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action="/teams" class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Name })}</span>
                            </div>
                            <input type="text" name="name" required=true maxlength="120" autofocus=true
                                   placeholder=t(MessageKey::TeamNamePlaceholder)
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::SlugFieldLabel)}</span>
                                <span class="label-text-alt text-xs opacity-70">{t(MessageKey::OptionalAutoDerivedHint)}</span>
                            </div>
                            <input type="text" name="slug" maxlength="64"
                                   pattern="[a-z0-9\\-]+"
                                   placeholder=t(MessageKey::SlugPlaceholder)
                                   class="input input-bordered input-sm w-full"/>
                            <div class="label py-1">
                                <span class="label-text-alt text-xs text-base-content/70">
                                    {t(MessageKey::SlugHelperText)}
                                </span>
                            </div>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span>
                                <span class="label-text-alt text-xs opacity-70">{t(MessageKey::OptionalHint)}</span>
                            </div>
                            <textarea name="description" rows="3" maxlength="500"
                                      placeholder=t(MessageKey::TeamDescriptionPlaceholder)
                                      class="textarea textarea-bordered textarea-sm w-full"></textarea>
                        </label>
                        <p class="text-xs text-base-content/70">
                            {t(MessageKey::NewTeamIntro)}
                        </p>
                        <div class="card-actions justify-end mt-2">
                            <a href="/teams" class="btn btn-ghost btn-sm">{t(MessageKey::CancelButton)}</a>
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::CreateTeamButton)}</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

#[component]
pub fn TeamDetailPage(
    user: CurrentUser,
    team: Team,
    role: TeamRole,
    members: Vec<(String, String, String, TeamRole, DateTime<Utc>)>,
    projects: Vec<Project>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> impl IntoView {
    let is_admin = role.can_manage_team();
    let team_name = team.name.clone();
    let team_slug = team.slug.clone();
    let team_description = team.description.clone().unwrap_or_default();
    let has_description = !team_description.is_empty();

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    let edit_link = is_admin.then(|| {
        let edit_href = format!("/teams/{}/edit", team_slug);
        view! {
            <a href=edit_href class="btn btn-sm btn-ghost"
               aria-label=t(MessageKey::EditTeamSettingsAriaLabel)>
                {t(MessageKey::NavLinkSettings)}
            </a>
        }
    });

    let member_rows = {
        let team_slug = team_slug.clone();
        let actor_user_id = user.id.clone();
        members
            .into_iter()
            .map(move |(uid, name, email, member_role, joined)| {
                render_member_row(
                    team_slug.clone(),
                    actor_user_id.clone(),
                    is_admin,
                    uid,
                    name,
                    email,
                    member_role,
                    joined,
                )
            })
            .collect_view()
    };

    let projects_section = render_projects_section(team_slug.clone(), is_admin, projects);
    let members_heading = t(MessageKey::MembersHeading);
    let members_heading_aria = members_heading.clone();

    let add_member_form = is_admin.then(|| {
        view! {
            <details class="card bg-base-100 border border-base-300 shadow-sm mt-4">
                <summary class="card-body cursor-pointer py-3 flex flex-row items-center gap-2">
                    <span class="font-medium">{t(MessageKey::InviteMemberSummary)}</span>
                    <span class="text-xs text-base-content/70">{t(MessageKey::ByEmailHint)}</span>
                </summary>
                <div class="px-4 pb-4">
                    <form method="post"
                          action=format!("/teams/{}/members", team_slug)
                          class="flex flex-wrap items-end gap-3">
                        <label class="form-control flex-1 min-w-[14rem]">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Email })}</span>
                            </div>
                            <input type="email" name="email" required=true
                                   placeholder=t(MessageKey::EmailPlaceholderExample)
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <label class="form-control">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Role })}</span>
                            </div>
                            <select name="role" class="select select-bordered select-sm">
                                <option value="member" selected=true>{t(MessageKey::TeamRoleName { label: TeamRoleLabel::Member })}</option>
                                <option value="admin">{t(MessageKey::TeamRoleName { label: TeamRoleLabel::Admin })}</option>
                                <option value="viewer">{t(MessageKey::TeamRoleName { label: TeamRoleLabel::Viewer })}</option>
                            </select>
                        </label>
                        <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::AddButton)}</button>
                    </form>
                    <p class="text-xs text-base-content/70 mt-2">
                        {t(MessageKey::InviteHelperText)}
                    </p>
                </div>
            </details>
        }
    });

    view! {
        <AppShell title=team_name.clone()
                  user=user
                  flash=flash
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/teams">{t(MessageKey::NavLinkTeams)}</a></li>
                    <li>{team_name.clone()}</li>
                </ul></div>

                <div class="flex items-center justify-between mb-2 gap-3">
                    <div>
                        <h1 class="text-xl font-semibold">{team_name.clone()}</h1>
                        {has_description.then(|| view! {
                            <p class="text-sm text-base-content/70 mt-1">{team_description}</p>
                        })}
                    </div>
                    {edit_link}
                </div>

                {error_block}

                <div class="flex gap-2 flex-wrap mb-4">
                    <a href=format!("/teams/{}/sprints", team_slug.clone())
                       class="btn btn-sm btn-outline">
                        {t(MessageKey::SprintsSectionName)}
                    </a>
                </div>

                {projects_section}

                <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                         aria-label=members_heading_aria>
                    <div class="card-body gap-3">
                        <h2 class="text-base font-medium">{members_heading}</h2>
                        <div class="overflow-x-auto">
                            <table class="table table-sm" aria-label=t(MessageKey::TeamMembersAriaLabel)>
                                <thead>
                                    <tr>
                                        <th>{t(MessageKey::FieldLabel { field: Field::Name })}</th>
                                        <th>{t(MessageKey::FieldLabel { field: Field::Email })}</th>
                                        <th>{t(MessageKey::FieldLabel { field: Field::Role })}</th>
                                        <th>{t(MessageKey::JoinedColumnHeading)}</th>
                                        <th></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {member_rows}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </section>

                {add_member_form}

                <p class="text-xs text-base-content/70 italic mt-4">
                    {t(MessageKey::TeamPrivacyFootnote)}
                </p>
            </div>
        </AppShell>
    }
}

fn render_projects_section(
    team_slug: String,
    is_admin: bool,
    projects: Vec<Project>,
) -> impl IntoView {
    let has_projects = !projects.is_empty();
    let team_slug_owned = team_slug;

    let project_rows = projects
        .into_iter()
        .map(|p| {
            let href = format!("/projects/{}", p.id);
            let unassign_action = format!("/teams/{}/projects/{}/unassign", team_slug_owned, p.id);
            let unassign = is_admin.then(|| {
                view! {
                    <form method="post" action=unassign_action
                          onsubmit="return confirm('Detach this project from the team? \
                                                    It will become a personal project.')">
                        <button type="submit" class="btn btn-ghost btn-xs"
                                aria-label=t(MessageKey::DetachFromTeamAriaLabel)>
                            {t(MessageKey::DetachButton)}
                        </button>
                    </form>
                }
            });
            view! {
                <tr>
                    <td>
                        <a href=href class="link link-hover font-medium">
                            {p.name}
                        </a>
                    </td>
                    <td class="text-right">{unassign}</td>
                </tr>
            }
        })
        .collect_view();

    let projects_heading = t(MessageKey::ProjectsSectionName);
    let projects_heading_aria = projects_heading.clone();
    view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                 aria-label=projects_heading_aria>
            <div class="card-body gap-3">
                <h2 class="text-base font-medium">{projects_heading}</h2>
                {has_projects.then(|| view! {
                    <div class="overflow-x-auto">
                        <table class="table table-sm" aria-label=t(MessageKey::TeamProjectsAriaLabel)>
                            <tbody>
                                {project_rows}
                            </tbody>
                        </table>
                    </div>
                })}
                {(!has_projects).then(|| view! {
                    <p class="text-sm text-base-content/70 italic">
                        {t(MessageKey::NoProjectsInTeamMessage)}
                    </p>
                })}
            </div>
        </section>
    }
}

#[allow(clippy::too_many_arguments)]
fn render_member_row(
    team_slug: String,
    actor_user_id: String,
    is_admin: bool,
    user_id: String,
    display_name: String,
    email: String,
    role: TeamRole,
    joined: DateTime<Utc>,
) -> impl IntoView {
    let is_self = user_id == actor_user_id;
    let role_action = format!("/teams/{}/members/{}/role", team_slug, user_id);
    let remove_action = format!("/teams/{}/members/{}/remove", team_slug, user_id);
    let joined_text = joined.format("%Y-%m-%d").to_string();
    let admin_selected = matches!(role, TeamRole::Admin);
    let member_selected = matches!(role, TeamRole::Member);
    let viewer_selected = matches!(role, TeamRole::Viewer);

    let role_cell = if is_admin && !is_self {
        // Admins can change others' roles via inline form.
        view! {
            <td>
                <form method="post" action=role_action class="inline-block">
                    <select name="role" onchange="this.form.submit()"
                            class="select select-bordered select-xs"
                            aria-label=t(MessageKey::ChangeRoleAriaLabel)>
                        <option value="admin" selected=admin_selected>{t(MessageKey::TeamRoleName { label: TeamRoleLabel::Admin })}</option>
                        <option value="member" selected=member_selected>{t(MessageKey::TeamRoleName { label: TeamRoleLabel::Member })}</option>
                        <option value="viewer" selected=viewer_selected>{t(MessageKey::TeamRoleName { label: TeamRoleLabel::Viewer })}</option>
                    </select>
                </form>
            </td>
        }
        .into_any()
    } else {
        view! {
            <td>
                <span class="text-sm">{t(MessageKey::TeamRoleName { label: role.to_i18n_label() })}</span>
            </td>
        }
        .into_any()
    };

    let action_cell = if is_self {
        // Self can leave the team regardless of admin status.
        view! {
            <td class="text-right">
                <form method="post" action=remove_action
                      onsubmit="return confirm('Leave this team?')">
                    <button type="submit" class="btn btn-ghost btn-xs text-base-content/70"
                            aria-label=t(MessageKey::LeaveTeamAriaLabel)>
                        {t(MessageKey::LeaveButton)}
                    </button>
                </form>
            </td>
        }
        .into_any()
    } else if is_admin {
        view! {
            <td class="text-right">
                <form method="post" action=remove_action
                      onsubmit="return confirm('Remove this member from the team?')">
                    <button type="submit" class="btn btn-ghost btn-xs text-error"
                            aria-label=t(MessageKey::RemoveMemberAriaLabel)>
                        {t(MessageKey::RemoveButton)}
                    </button>
                </form>
            </td>
        }
        .into_any()
    } else {
        view! { <td></td> }.into_any()
    };

    view! {
        <tr>
            <td>{display_name}{is_self.then(|| view! {
                <span class="text-xs opacity-70 ml-1">{t(MessageKey::YouSuffix)}</span>
            })}</td>
            <td class="text-sm text-base-content/70">{email}</td>
            {role_cell}
            <td class="text-sm text-base-content/70">{joined_text}</td>
            {action_cell}
        </tr>
    }
}

#[component]
pub fn TeamEditPage(
    user: CurrentUser,
    team: Team,
    unread_count: i64,
    error: Option<String>,
) -> impl IntoView {
    let team_slug = team.slug.clone();
    let team_name = team.name.clone();
    let team_description = team.description.clone().unwrap_or_default();
    let edit_action = format!("/teams/{}/edit", team_slug);
    let back_href = format!("/teams/{}", team_slug);

    let error_block = error.map(|msg| {
        view! {
            <div role="alert" class="alert alert-warning text-sm mb-4">{msg}</div>
        }
    });

    view! {
        <AppShell title=t(MessageKey::EditTeamPageTitle { team_name: team_name.clone() })
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/teams">{t(MessageKey::NavLinkTeams)}</a></li>
                    <li><a href=back_href.clone()>{team_name.clone()}</a></li>
                    <li>{t(MessageKey::NavLinkSettings)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">{t(MessageKey::TeamSettingsHeading)}</h1>

                {error_block}

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=edit_action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Name })}</span>
                            </div>
                            <input type="text" name="name" required=true maxlength="120"
                                   value=team_name
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span>
                            </div>
                            <textarea name="description" rows="3" maxlength="500"
                                      class="textarea textarea-bordered textarea-sm w-full">
                                {team_description}
                            </textarea>
                        </label>
                        <p class="text-xs text-base-content/70">
                            {t(MessageKey::SlugFixedNotice)}
                        </p>
                        <div class="card-actions justify-end mt-2">
                            <a href=back_href class="btn btn-ghost btn-sm">{t(MessageKey::CancelButton)}</a>
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::SaveButton)}</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

pub fn render_list(
    user: CurrentUser,
    teams: Vec<(Team, TeamRole)>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <TeamsListPage
                user=user
                teams=teams
                unread_count=unread_count
                flash=flash
                error=error
            />
        }
    })
}

pub fn render_new(user: CurrentUser, unread_count: i64, error: Option<String>) -> Html<String> {
    super::render_to_html(move || {
        view! { <TeamNewPage user=user unread_count=unread_count error=error/> }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn render_detail(
    user: CurrentUser,
    team: Team,
    role: TeamRole,
    members: Vec<(String, String, String, TeamRole, DateTime<Utc>)>,
    projects: Vec<Project>,
    unread_count: i64,
    flash: Option<String>,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <TeamDetailPage
                user=user
                team=team
                role=role
                members=members
                projects=projects
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
    unread_count: i64,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <TeamEditPage user=user team=team unread_count=unread_count error=error/> }
    })
}
