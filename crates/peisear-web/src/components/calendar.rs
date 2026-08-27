// peisear-feature-spec-v2.1 §16.6 — this calendar deliberately
// renders no efficiency metrics. No fill rate, no "free hours", no
// comparison to last week. These look helpful and act as pressure.
// If you find yourself adding one, the answer is no; if it survives
// review and you ship it anyway, you owe a CHANGELOG entry
// explaining the override.

//! The two calendar surfaces (`CAL-002` / RFC 002 §16).
//!
//! `CalendarPage` renders both axes through one shared grid — the
//! personal page and the project page differ only in title,
//! breadcrumb, footer text, whether a project name badge shows per
//! block (personal axis spans projects; project axis doesn't need
//! one), and whether a sprint band can appear at all (project axis
//! only, RFC 002 must-have 7).
//!
//! Cells are an HTML table for month view, stacked entries in a flex
//! column for week view, and a single hour-ruled column with
//! percentage-positioned blocks for day view — RFC 002 §Design.

use chrono::{Datelike, NaiveDate};

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::{
    CurrentUser, Issue, Project, calendar::CROWDING_WATCH_THRESHOLD, sprints::Sprint,
};
use peisear_i18n::MessageKey;

use super::layout::AppShell;
use super::{CalendarDay, CalendarView, grow, t};

/// A block's rendered time text, e.g. `"09:00"` or `"09:00–10:30"`.
/// Empty only if `planned_start_at` is somehow `None` — every block
/// reaching this component came from a query that filters on
/// `planned_start_at IS NOT NULL`, so that's defensive, not expected.
fn time_label(issue: &Issue) -> String {
    let Some(start) = issue.planned_start_at else {
        return String::new();
    };
    let start_str = start.format("%H:%M").to_string();
    match issue.planned_end_at {
        Some(end) => format!("{start_str}–{}", end.format("%H:%M")),
        None => start_str,
    }
}

fn cell_aria(date: NaiveDate, count: usize) -> String {
    t(MessageKey::CalendarCellAriaLabel {
        month: date.month(),
        day: date.day(),
        count: count as i64,
    })
}

fn crowding_chip(count: usize) -> Option<impl IntoView> {
    (count > CROWDING_WATCH_THRESHOLD).then(|| {
        let state = peisear_core::DisplayHealthState::Watch;
        let aria = t(MessageKey::CrowdingChipAriaLabel {
            state: state.to_i18n_label(),
        });
        view! {
            <span class=format!("badge badge-xs {}", state.badge_class())
                  aria-label=aria.clone() title=aria>
                <span aria-hidden="true">{state.glyph()}</span>
            </span>
        }
    })
}

/// One block link. `project_badge` is `Some(name)` on the personal
/// axis (distinguishes which project — open question 3's default,
/// text rather than colour-only per this project's established
/// anti-colour-only pattern) and `None` on the project axis, which
/// never shows anything beyond title/span/colour (RFC 002 must-have
/// 6, corrected: **not the assignee**, on either axis, in this
/// component — there is simply no assignee-rendering code path here
/// at all).
fn render_block(issue: &Issue, project_badge: Option<String>) -> impl IntoView + use<> {
    let href = format!("/projects/{}/issues/{}", issue.project_id, issue.id);
    let title = issue.title.clone();
    let time = time_label(issue);
    let badge = project_badge
        .map(|name| view! { <span class="text-[10px] opacity-70 truncate">{name}</span> });
    view! {
        <a href=href class="block text-xs px-1.5 py-0.5 rounded bg-primary/10 hover:bg-primary/20 truncate">
            <span class="tabular-nums opacity-70 mr-1">{time}</span>
            <span>{title}</span>
            {badge}
        </a>
    }
}

/// Day view: one column, an hour ruler (00–23) down the side, blocks
/// absolute-positioned by percentage of the 24h day. Percentage
/// positioning (not JS) so the layout reflows on screen size —
/// `DEC-021` permits JS only as an enhancement over a working no-JS
/// path, and this is the no-JS path.
fn render_day_view(
    day: &CalendarDay,
    project_badge_for: impl Fn(&Issue) -> Option<String>,
) -> impl IntoView {
    let day_start_secs = 0.0_f64;
    let total_secs = 24.0 * 3600.0;
    let blocks = day
        .blocks
        .iter()
        .map(|issue| {
            let start = issue.planned_start_at?;
            let start_of_day = start
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .expect("00:00:00 valid")
                .and_utc();
            let start_secs = (start - start_of_day).num_seconds() as f64;
            let end_secs = match issue.planned_end_at {
                Some(end) => (end - start_of_day).num_seconds() as f64,
                None => start_secs + 30.0 * 60.0,
            };
            let top = (start_secs.max(day_start_secs) / total_secs * 100.0).min(100.0);
            let height = (((end_secs.min(total_secs) - start_secs.max(day_start_secs))
                / total_secs)
                * 100.0)
                .max(1.5);
            let style = format!("top:{top}%;height:{height}%;");
            let href = format!("/projects/{}/issues/{}", issue.project_id, issue.id);
            let title = issue.title.clone();
            let time = time_label(issue);
            let badge = project_badge_for(issue).map(|name| {
                view! { <span class="text-[10px] opacity-70 block truncate">{name}</span> }
            });
            Some(view! {
                <a href=href style=style
                   class="absolute left-12 right-1 rounded bg-primary/15 hover:bg-primary/25 \
                          border-l-2 border-primary px-1.5 py-0.5 text-xs overflow-hidden">
                    <span class="tabular-nums opacity-70 mr-1">{time}</span>
                    <span>{title}</span>
                    {badge}
                </a>
            })
        })
        .collect_view();

    let hours = (0..24u32)
        .map(|h| {
            let top = h as f64 / 24.0 * 100.0;
            view! {
                <div class="absolute left-0 text-[10px] opacity-70 tabular-nums"
                     style=format!("top:{top}%;")>
                    {format!("{h:02}:00")}
                </div>
            }
        })
        .collect_view();

    let aria = cell_aria(day.date, day.blocks.len());

    view! {
        <div class="relative border border-base-300 rounded bg-base-100" style="height: 960px;"
             role="group" aria-label=aria>
            {hours}
            {blocks}
        </div>
    }
}

/// Week view: 7 columns, blocks stacked in start-time order within
/// each — no hour-precision positioning, matching RFC 002's "a flex
/// column for week view" (distinct from day view's hour-ruled grid).
fn render_week_view(
    days: &[CalendarDay],
    project_badge_for: impl Fn(&Issue) -> Option<String>,
) -> impl IntoView {
    let cols = days
        .iter()
        .map(|day| {
            let aria = cell_aria(day.date, day.blocks.len());
            let chip = crowding_chip(day.blocks.len());
            let date_label = day.date.format("%a %-d").to_string();
            let blocks = day
                .blocks
                .iter()
                .map(|issue| render_block(issue, project_badge_for(issue)))
                .collect_view();
            view! {
                <div class="flex-1 min-w-0 border border-base-300 rounded p-1.5 bg-base-100"
                     role="group" aria-label=aria>
                    <div class="flex items-center gap-1 text-xs font-medium mb-1">
                        <span>{date_label}</span>
                        {chip}
                    </div>
                    <div class="flex flex-col gap-1">{blocks}</div>
                </div>
            }
        })
        .collect_view();
    view! { <div class="flex gap-2 overflow-x-auto">{cols}</div> }
}

/// Month view: an HTML table, one row per week (Monday-start,
/// matching the handler's week-start convention), padded with blank
/// cells so the grid stays aligned — the padding cells carry no data
/// and need none, since they fall outside the query window.
fn render_month_view(
    days: &[CalendarDay],
    project_badge_for: impl Fn(&Issue) -> Option<String>,
) -> impl IntoView {
    let Some(first) = days.first() else {
        return view! { <table class="table table-fixed w-full"></table> }.into_any();
    };
    let leading = first.date.weekday().num_days_from_monday() as usize;
    let last = days.last().expect("checked non-empty via first()").date;
    let trailing = 6 - last.weekday().num_days_from_monday() as usize;

    let mut cells: Vec<AnyView> = Vec::with_capacity(leading + days.len() + trailing);
    for _ in 0..leading {
        cells.push(
            view! { <td class="align-top border border-base-300 p-1 h-24 opacity-30"></td> }
                .into_any(),
        );
    }
    for day in days {
        let aria = cell_aria(day.date, day.blocks.len());
        let chip = crowding_chip(day.blocks.len());
        let day_num = day.date.day().to_string();
        let blocks = day
            .blocks
            .iter()
            .take(3)
            .map(|issue| render_block(issue, project_badge_for(issue)))
            .collect_view();
        let more = (day.blocks.len() > 3).then(|| {
            let label = t(MessageKey::CalendarMoreIssuesLabel {
                count: (day.blocks.len() - 3) as i64,
            });
            view! { <div class="text-[10px] opacity-70">{label}</div> }
        });
        cells.push(
            view! {
                <td class="align-top border border-base-300 p-1 h-24" role="group" aria-label=aria>
                    <div class="flex items-center gap-1 text-xs font-medium">
                        <span>{day_num}</span>
                        {chip}
                    </div>
                    <div class="flex flex-col gap-0.5 mt-0.5">{blocks}</div>
                    {more}
                </td>
            }
            .into_any(),
        );
    }
    for _ in 0..trailing {
        cells.push(
            view! { <td class="align-top border border-base-300 p-1 h-24 opacity-30"></td> }
                .into_any(),
        );
    }

    // Build rows by draining 7 cells at a time -- `AnyView` isn't
    // `Clone`, so a flat `Vec` + `.chunks(7).to_vec()` (which would
    // need to clone each chunk) doesn't work; draining consumes the
    // cells instead of copying them.
    let mut rows_view = Vec::with_capacity(cells.len().div_ceil(7));
    let mut remaining = cells;
    while !remaining.is_empty() {
        let rest = remaining.split_off(remaining.len().min(7));
        let week = remaining;
        remaining = rest;
        rows_view.push(view! { <tr>{week}</tr> });
    }

    view! {
        <table class="table table-fixed w-full">
            <tbody>{rows_view}</tbody>
        </table>
    }
    .into_any()
}

#[allow(clippy::too_many_arguments)]
fn render_nav(
    base_href: String,
    view: CalendarView,
    anchor: NaiveDate,
    prev_date: NaiveDate,
    next_date: NaiveDate,
) -> impl IntoView + use<> {
    let view_links = [CalendarView::Day, CalendarView::Week, CalendarView::Month]
        .into_iter()
        .map(|v| {
            let href = format!(
                "{base_href}?view={}&date={}",
                v.as_str(),
                anchor.format("%Y-%m-%d")
            );
            let label = t(MessageKey::CalendarViewName {
                view: v.to_i18n_label(),
            });
            let class = if v == view {
                grow("btn btn-xs btn-primary")
            } else {
                grow("btn btn-xs btn-ghost")
            };
            view! { <a href=href class=class>{label}</a> }
        })
        .collect_view();

    let prev_href = format!(
        "{base_href}?view={}&date={}",
        view.as_str(),
        prev_date.format("%Y-%m-%d")
    );
    let next_href = format!(
        "{base_href}?view={}&date={}",
        view.as_str(),
        next_date.format("%Y-%m-%d")
    );

    view! {
        <div class="flex flex-wrap items-center justify-between gap-2 mb-3"
             role="group" aria-label=t(MessageKey::CalendarViewSwitcherAriaLabel)>
            <div class="flex items-center gap-1">
                <a href=prev_href class=grow("btn btn-xs btn-ghost")>{t(MessageKey::PreviousPageLink)}</a>
                <a href=next_href class=grow("btn btn-xs btn-ghost")>{t(MessageKey::NextPageLink)}</a>
            </div>
            <div class="flex items-center gap-1">{view_links}</div>
        </div>
    }
}

fn render_grid(
    view: CalendarView,
    days: Vec<CalendarDay>,
    project_badge_for: impl Fn(&Issue) -> Option<String>,
) -> impl IntoView {
    let is_empty = days.iter().all(|d| d.blocks.is_empty());
    let grid = match view {
        CalendarView::Day => {
            let day = days.into_iter().next();
            match day {
                Some(day) => render_day_view(&day, project_badge_for).into_any(),
                None => view! { <div></div> }.into_any(),
            }
        }
        CalendarView::Week => render_week_view(&days, project_badge_for).into_any(),
        CalendarView::Month => render_month_view(&days, project_badge_for).into_any(),
    };
    view! {
        {is_empty.then(|| view! {
            <p class="text-sm text-base-content/70 italic mb-2">{t(MessageKey::NoPlannedIssuesMessage)}</p>
        })}
        {grid}
    }
}

fn render_sprint_band(sprint: Sprint) -> impl IntoView + use<> {
    let aria = t(MessageKey::SprintBandAriaLabel {
        sprint_name: sprint.name.clone(),
    });
    let dates = format!(
        "{} → {}",
        sprint.starts_on.format("%Y-%m-%d"),
        sprint.ends_on.format("%Y-%m-%d")
    );
    let name = sprint.name;
    view! {
        <div class="alert py-1.5 px-3 mb-3 text-sm" role="group" aria-label=aria>
            <span class="font-medium">{name}</span>
            <span class="opacity-70">{dates}</span>
        </div>
    }
}

#[allow(clippy::too_many_arguments)]
#[component]
pub fn PersonalCalendarPage(
    user: CurrentUser,
    view: CalendarView,
    anchor: NaiveDate,
    prev_date: NaiveDate,
    next_date: NaiveDate,
    days: Vec<CalendarDay>,
    /// project id → name, for every project any rendered block
    /// belongs to. The personal axis spans projects (unlike the
    /// project axis), so each block names its project — open
    /// question 3's default (project-distinguishing on the personal
    /// axis), as text rather than colour-only, matching this
    /// project's established anti-colour-only pattern
    /// (`NFR-A11Y-004`) rather than a hash-derived hue.
    project_names: std::collections::HashMap<String, String>,
    unread_count: i64,
) -> impl IntoView {
    let nav = render_nav(
        "/today/calendar".to_string(),
        view,
        anchor,
        prev_date,
        next_date,
    );
    let grid = render_grid(view, days, move |issue: &Issue| {
        project_names.get(&issue.project_id).cloned()
    });
    view! {
        <AppShell title=t(MessageKey::PersonalCalendarPageTitle)
                  user=user flash={None::<String>} unread_count=unread_count>
            <div class="max-w-5xl mx-auto">
                {super::breadcrumb::render_breadcrumb(vec![
                    super::breadcrumb::BreadcrumbItem::current(t(MessageKey::CalendarBreadcrumbWord)),
                ])}
                <h1 class="text-xl font-semibold mb-1">{t(MessageKey::PersonalCalendarPageTitle)}</h1>
                <p class="text-xs text-base-content/70 mb-3">{t(MessageKey::CalendarUtcNote)}</p>
                {nav}
                {grid}
                <p class="text-xs text-base-content/70 mt-4">{t(MessageKey::PersonalCalendarPrivacyFootnote)}</p>
            </div>
        </AppShell>
    }
}

#[allow(clippy::too_many_arguments)]
#[component]
pub fn ProjectCalendarPage(
    user: CurrentUser,
    project: Project,
    view: CalendarView,
    anchor: NaiveDate,
    prev_date: NaiveDate,
    next_date: NaiveDate,
    days: Vec<CalendarDay>,
    sprint: Option<Sprint>,
    unread_count: i64,
) -> impl IntoView {
    let base_href = format!("/projects/{}/calendar", project.id);
    let project_name = project.name.clone();
    let project_href = format!("/projects/{}", project.id);
    let nav = render_nav(base_href.clone(), view, anchor, prev_date, next_date);
    let band = sprint.map(render_sprint_band);
    // Project axis: no per-block badge at all (single project; no
    // assignee, no other label — RFC 002 must-have 6 corrected).
    let grid = render_grid(view, days, |_issue: &Issue| None);
    view! {
        <AppShell title=t(MessageKey::ProjectCalendarPageTitle { project_name: project_name.clone() })
                  user=user flash={None::<String>} unread_count=unread_count>
            <div class="max-w-5xl mx-auto">
                {super::breadcrumb::render_breadcrumb(vec![
                    super::breadcrumb::BreadcrumbItem::link(t(MessageKey::ProjectsSectionName), "/projects"),
                    super::breadcrumb::BreadcrumbItem::link(project_name.clone(), project_href),
                    super::breadcrumb::BreadcrumbItem::current(t(MessageKey::CalendarBreadcrumbWord)),
                ])}
                <h1 class="text-xl font-semibold mb-1">
                    {t(MessageKey::ProjectCalendarPageTitle { project_name })}
                </h1>
                <p class="text-xs text-base-content/70 mb-3">{t(MessageKey::CalendarUtcNote)}</p>
                {nav}
                {band}
                {grid}
                <p class="text-xs text-base-content/70 mt-4">{t(MessageKey::ProjectCalendarPrivacyFootnote)}</p>
            </div>
        </AppShell>
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_personal(
    user: CurrentUser,
    view: CalendarView,
    anchor: NaiveDate,
    prev_date: NaiveDate,
    next_date: NaiveDate,
    days: Vec<CalendarDay>,
    project_names: std::collections::HashMap<String, String>,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <PersonalCalendarPage
                user=user view=view anchor=anchor prev_date=prev_date next_date=next_date
                days=days project_names=project_names unread_count=unread_count
            />
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn render_project(
    user: CurrentUser,
    project: Project,
    view: CalendarView,
    anchor: NaiveDate,
    prev_date: NaiveDate,
    next_date: NaiveDate,
    days: Vec<CalendarDay>,
    sprint: Option<Sprint>,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <ProjectCalendarPage
                user=user project=project view=view anchor=anchor prev_date=prev_date
                next_date=next_date days=days sprint=sprint unread_count=unread_count
            />
        }
    })
}
