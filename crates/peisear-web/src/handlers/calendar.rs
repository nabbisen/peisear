//! The two calendar surfaces (`CAL-002` / RFC 002 §16).
//!
//! `GET /today/calendar` — personal axis, self-only by construction
//! (the assignee filter is the authenticated user's own id; there is
//! no URL parameter naming another user). `GET /projects/{id}/calendar`
//! — project axis, gated by the existing `projects::find_accessible`.
//!
//! View/date parsing and per-day bucketing live here rather than in
//! the component, matching `handlers::issues::project_detail`'s own
//! precedent of shaping data (its `Vec<Column>` board columns) before
// handing it to a renderer that only lays out what it's given.

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use peisear_core::Issue;
use peisear_storage::{issues, notifications as notif_store, projects, sprints};
use serde::Deserialize;

use crate::{
    AppResult, AppState,
    components::{self, CalendarDay, CalendarView},
    extractors::AuthUser,
};

#[derive(Debug, Deserialize)]
pub struct CalendarQuery {
    pub view: Option<String>,
    pub date: Option<String>,
}

/// Unknown/missing → `Week` (RFC 002 must-have 3's default, and
/// `CAL-002` §5 test 3: "an unknown value falls back to week rather
/// than erroring").
fn parse_view(raw: Option<&str>) -> CalendarView {
    match raw {
        Some("day") => CalendarView::Day,
        Some("month") => CalendarView::Month,
        _ => CalendarView::Week,
    }
}

fn parse_anchor(raw: Option<&str>) -> NaiveDate {
    raw.and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive())
}

fn week_start(d: NaiveDate) -> NaiveDate {
    // Monday-start. RFC 002 doesn't name a week-start day; ISO 8601's
    // convention is the least-surprising default and this project has
    // no existing week-start precedent elsewhere to match instead.
    d - Duration::days(d.weekday().num_days_from_monday() as i64)
}

fn first_of_month(d: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(d.year(), d.month(), 1).expect("day 1 is always valid")
}

fn last_of_month(d: NaiveDate) -> NaiveDate {
    let (y, m) = (d.year(), d.month());
    let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
    NaiveDate::from_ymd_opt(ny, nm, 1).expect("the 1st of any month is valid") - Duration::days(1)
}

/// The visible window's first and last day, inclusive.
fn window_days(view: CalendarView, anchor: NaiveDate) -> (NaiveDate, NaiveDate) {
    match view {
        CalendarView::Day => (anchor, anchor),
        CalendarView::Week => {
            let s = week_start(anchor);
            (s, s + Duration::days(6))
        }
        CalendarView::Month => (first_of_month(anchor), last_of_month(anchor)),
    }
}

/// The anchor date `?date=` should carry for the "previous" link —
/// shifted by the current view's span, per RFC 002 must-have 4.
fn prev_anchor(view: CalendarView, anchor: NaiveDate) -> NaiveDate {
    match view {
        CalendarView::Day => anchor - Duration::days(1),
        CalendarView::Week => anchor - Duration::days(7),
        CalendarView::Month => first_of_month(anchor) - Duration::days(1),
    }
}

fn next_anchor(view: CalendarView, anchor: NaiveDate) -> NaiveDate {
    match view {
        CalendarView::Day => anchor + Duration::days(1),
        CalendarView::Week => anchor + Duration::days(7),
        CalendarView::Month => last_of_month(anchor) + Duration::days(1),
    }
}

fn window_utc(
    view: CalendarView,
    anchor: NaiveDate,
) -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
    let (first, last) = window_days(view, anchor);
    let from = first
        .and_hms_opt(0, 0, 0)
        .expect("00:00:00 is always valid")
        .and_utc();
    let to = last
        .and_hms_opt(23, 59, 59)
        .expect("23:59:59 is always valid")
        .and_utc();
    (from, to)
}

/// Group issues by every calendar day they overlap, within
/// `[window_first, window_last]`. A `planned_end_at IS NULL` issue
/// (the half-hour-anchor case, must-have 5) occupies only its start
/// day.
fn bucket_by_day(
    mut issues: Vec<Issue>,
    window_first: NaiveDate,
    window_last: NaiveDate,
) -> Vec<CalendarDay> {
    issues.sort_by_key(|i| i.planned_start_at);
    let mut days = Vec::new();
    let mut d = window_first;
    while d <= window_last {
        let blocks: Vec<Issue> = issues
            .iter()
            .filter(|i| {
                let Some(start) = i.planned_start_at else {
                    return false;
                };
                let start_day = start.date_naive();
                let end_day = i
                    .planned_end_at
                    .map(|e| e.date_naive())
                    .unwrap_or(start_day);
                start_day <= d && d <= end_day
            })
            .cloned()
            .collect();
        days.push(CalendarDay { date: d, blocks });
        d += Duration::days(1);
    }
    days
}

pub async fn personal_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<CalendarQuery>,
) -> AppResult<impl IntoResponse> {
    let view = parse_view(q.view.as_deref());
    let anchor = parse_anchor(q.date.as_deref());
    let (window_first, window_last) = window_days(view, anchor);
    let (from, to) = window_utc(view, anchor);

    let planned = issues::planned_for_user(&state.db, &user.id, from, to).await?;
    let days = bucket_by_day(planned, window_first, window_last);
    // The personal axis spans every project the user's assigned
    // issues live in, so each block names its project (open question
    // 3's default) — `list_for_user` is the same "projects this user
    // can see" set the /projects list page already uses, not a new
    // access-control surface.
    let project_names: std::collections::HashMap<String, String> =
        projects::list_for_user(&state.db, &user.id)
            .await?
            .into_iter()
            .map(|p| (p.id, p.name))
            .collect();
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::calendar::render_personal(
        user,
        view,
        anchor,
        prev_anchor(view, anchor),
        next_anchor(view, anchor),
        days,
        project_names,
        unread_count,
    ))
}

pub async fn project_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Query(q): Query<CalendarQuery>,
) -> AppResult<impl IntoResponse> {
    let project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let view = parse_view(q.view.as_deref());
    let anchor = parse_anchor(q.date.as_deref());
    let (window_first, window_last) = window_days(view, anchor);
    let (from, to) = window_utc(view, anchor);

    let planned = issues::planned_for_project(&state.db, &project_id, from, to).await?;
    let days = bucket_by_day(planned, window_first, window_last);
    let sprint_band = sprints::active_sprints_overlapping(&state.db, &project_id, from, to).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::calendar::render_project(
        user,
        project,
        view,
        anchor,
        prev_anchor(view, anchor),
        next_anchor(view, anchor),
        days,
        sprint_band.into_iter().next(),
        unread_count,
    ))
}
