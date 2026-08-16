//! Search results page (Phase A Step 4, peisear-feature-spec
//! v2.1 §4.5).
//!
//! Two sections — projects then issues — each paginated
//! independently via the shared `?page=N` parameter. Both
//! sections show "No results" inline when empty, rather than
//! hiding the section entirely; that way a user searching for
//! a term with only project hits can see at-a-glance that no
//! issues matched, and vice versa.

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::CurrentUser;
use peisear_i18n::MessageKey;
use peisear_storage::search::SearchHit;

use super::layout::AppShell;
use super::t;

#[allow(clippy::too_many_arguments)]
pub fn render_results(
    user: CurrentUser,
    q: String,
    projects: Vec<SearchHit>,
    issues: Vec<SearchHit>,
    page: u32,
    has_more_projects: bool,
    has_more_issues: bool,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SearchResultsPage
                user=user
                q=q
                projects=projects
                issues=issues
                page=page
                has_more_projects=has_more_projects
                has_more_issues=has_more_issues
            />
        }
    })
}

#[component]
fn SearchResultsPage(
    user: CurrentUser,
    q: String,
    projects: Vec<SearchHit>,
    issues: Vec<SearchHit>,
    page: u32,
    has_more_projects: bool,
    has_more_issues: bool,
) -> impl IntoView {
    let title = if q.is_empty() {
        t(MessageKey::SearchWord)
    } else {
        t(MessageKey::SearchPageTitleWithQuery { q: q.clone() })
    };
    let q_for_form = q.clone();
    let q_for_heading = q.clone();
    let q_for_pagination_projects = q.clone();
    let q_for_pagination_issues = q.clone();
    let has_query = !q.is_empty();

    view! {
        <AppShell title=title user=user flash=None>
            // Inline form — also reachable when the user
            // landed on /search with no query, or wants to
            // refine. Keeps results page usable without the
            // navbar typeahead.
            <form method="get" action="/search"
                  class="mb-6 flex gap-2 items-end max-w-xl"
                  aria-label=t(MessageKey::SearchWord)>
                <label class="form-control flex-1">
                    <div class="label py-0">
                        <span class="label-text text-sm">{t(MessageKey::SearchFieldLabel)}</span>
                    </div>
                    <input type="search"
                           name="q"
                           value=q_for_form
                           autofocus=true
                           placeholder=t(MessageKey::SearchPlaceholder)
                           class="input input-bordered input-sm w-full"/>
                </label>
                <button type="submit" class="btn btn-sm btn-primary">{t(MessageKey::SearchWord)}</button>
            </form>

            {if has_query {
                view! {
                    <h1 class="text-xl font-semibold mb-4">
                        {t(MessageKey::ResultsForHeadingPrefix)}
                        <span class="font-mono">{format!("\"{q_for_heading}\"")}</span>
                    </h1>
                }.into_any()
            } else {
                view! {
                    <p class="text-sm text-base-content/60">
                        {t(MessageKey::NoQueryGuidanceMessage)}
                    </p>
                }.into_any()
            }}

            {has_query.then(|| view! {
                <SearchSection
                    section_title=t(MessageKey::ProjectsSectionName)
                    hits=projects
                    page=page
                    has_more=has_more_projects
                    q=q_for_pagination_projects
                    section_kind=SectionKind::Projects
                />
                <SearchSection
                    section_title=t(MessageKey::OpenIssuesSectionName)
                    hits=issues
                    page=page
                    has_more=has_more_issues
                    q=q_for_pagination_issues
                    section_kind=SectionKind::Issues
                />
            })}
        </AppShell>
    }
}

#[derive(Clone, Copy)]
enum SectionKind {
    Projects,
    Issues,
}

#[component]
fn SearchSection(
    section_title: String,
    hits: Vec<SearchHit>,
    page: u32,
    has_more: bool,
    q: String,
    #[allow(unused_variables)] section_kind: SectionKind,
) -> impl IntoView {
    let count = hits.len();
    let is_empty = count == 0 && page == 1;
    let prev_page = page.saturating_sub(1);
    let next_page = page + 1;
    let q_prev = q.clone();
    let q_next = q.clone();
    let show_prev = page > 1;

    view! {
        <section class="mb-8">
            <h2 class="text-base font-semibold mb-2">
                {section_title}
                <span class="text-sm font-normal text-base-content/60 ml-2">
                    {format!("({count})")}
                </span>
            </h2>

            {if is_empty {
                view! {
                    <p class="text-sm text-base-content/60 italic mb-2">
                        {t(MessageKey::NoMatchesInCategoryMessage)}
                    </p>
                }.into_any()
            } else {
                view! {
                    <ul class="divide-y divide-base-300 border border-base-300 rounded-md">
                        {hits.into_iter().map(|h| view! { <SearchHitRow hit=h/> }).collect_view()}
                    </ul>
                }.into_any()
            }}

            {(show_prev || has_more).then(|| view! {
                <div class="flex gap-2 mt-3 text-sm">
                    {show_prev.then(|| {
                        let prev_url = format!("/search?q={}&page={}",
                            urlencode(&q_prev), prev_page);
                        view! {
                            <a href=prev_url class="link link-hover">
                                {t(MessageKey::PreviousPageLink)}
                            </a>
                        }
                    })}
                    {has_more.then(|| {
                        let next_url = format!("/search?q={}&page={}",
                            urlencode(&q_next), next_page);
                        view! {
                            <a href=next_url class="link link-hover">
                                {t(MessageKey::NextPageLink)}
                            </a>
                        }
                    })}
                </div>
            })}
        </section>
    }
}

#[component]
fn SearchHitRow(hit: SearchHit) -> impl IntoView {
    match hit {
        SearchHit::Project { id, name } => {
            let url = format!("/projects/{id}");
            view! {
                <li>
                    <a href=url class="block px-3 py-2 hover:bg-base-200">
                        <div class="font-medium">{name}</div>
                        <div class="text-xs text-base-content/60">{t(MessageKey::ProjectHitTypeLabel)}</div>
                    </a>
                </li>
            }
            .into_any()
        }
        SearchHit::Issue {
            id,
            project_id,
            project_name,
            title,
            parent_title,
        } => {
            let url = format!("/projects/{project_id}/issues/{id}");
            let caption = match parent_title {
                // A sub-issue reads in context: project, then the
                // parent it belongs to (`INBOX-001`, RFC 003 D3).
                // The row's own title stays the bold heading.
                Some(parent_title) => t(MessageKey::SubIssueHitTypePrefix {
                    project_name,
                    parent_title,
                }),
                None => t(MessageKey::OpenIssueHitTypePrefix { project_name }),
            };
            view! {
                <li>
                    <a href=url class="block px-3 py-2 hover:bg-base-200">
                        <div class="font-medium">{title}</div>
                        <div class="text-xs text-base-content/60">{caption}</div>
                    </a>
                </li>
            }
            .into_any()
        }
    }
}

/// Minimal URL-encoder for the `q` query parameter. Avoids
/// pulling in a dep just for `?q=...` round-tripping in
/// pagination links. Encodes the characters that matter for a
/// query-string value: `&`, `=`, `+`, `#`, `?`, `%`, ` `, and
/// non-ASCII via percent-encoding bytes of the UTF-8 form.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                out.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    out
}
