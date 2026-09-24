//! `RACE-002` — the optimistic lock compares a value the handler read
//! earlier, so two saves carrying the same valid `client_updated_at`
//! both passed the comparison and both wrote; the earlier writer was
//! told it had succeeded and was silently overwritten.
//!
//! `optimistic_lock.rs` proves the lock catches a *stale page* (a
//! stamp minutes old). It cannot prove the lock holds inside the
//! request window, because it issues its requests one after another.
//! Every test here releases its requests together with a barrier, on a
//! multi-thread runtime, so they genuinely overlap.
//!
//! ## Why every fixture sleeps first
//!
//! `updated_at` is a one-second `CURRENT_TIMESTAMP`. A write that lands
//! in the *same second* as the row's previous write leaves the stamp
//! unchanged, and a second saver holding that stamp is then
//! indistinguishable from a fresh one -- a resolution limit of the
//! version stamp itself, not of how it is compared, and the review
//! request for this handoff reports it. Each test therefore lets the
//! fixture's own creation second pass before it reads the stamp, which
//! is also the realistic case: a person edits a row that was last
//! touched some time ago.

mod common;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use common::auth::{TestUser, register_and_login};
use common::fixture::{
    create_issue, create_personal_project, create_planned_sprint, create_team_with_admin,
};
use common::server::{TestApp, ensure_distinct_timestamp};
use std::sync::Arc;
use tokio::sync::Barrier;

/// Simultaneous requests per entity, all carrying one stamp.
const N: usize = 8;

/// Independent entities raced in one release. A single entity's race
/// fires only some of the time (the window between a handler's read and
/// its write is short); twelve of them released together make "at least
/// one shows the defect" reliable, and the assertion is per entity, so a
/// pass means *every* one of the twelve held.
const M: usize = 12;

type Job<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;

/// Release every job at the same instant and collect results in order.
async fn all_at_once<T: Send + 'static>(jobs: Vec<Job<T>>) -> Vec<T> {
    let barrier = Arc::new(Barrier::new(jobs.len()));
    let handles: Vec<_> = jobs
        .into_iter()
        .map(|fut| {
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                fut.await
            })
        })
        .collect();
    let mut out = Vec::new();
    for h in handles {
        out.push(h.await.expect("task panicked"));
    }
    out
}

/// What a same-stamp race produced: which request won, and how the
/// rest were answered.
struct Outcome {
    /// Indices whose response was the success shape.
    winners: Vec<usize>,
    /// Indices answered `409 Conflict`.
    conflicts: usize,
    /// Every status, in request order, for the failure message.
    statuses: Vec<StatusCode>,
}

fn tally(statuses: Vec<StatusCode>, success: StatusCode) -> Outcome {
    Outcome {
        winners: statuses
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == success)
            .map(|(i, _)| i)
            .collect(),
        conflicts: statuses
            .iter()
            .filter(|s| **s == StatusCode::CONFLICT)
            .count(),
        statuses,
    }
}

/// One winner, `N - 1` conflicts, and nothing else: the whole point.
fn assert_one_winner(o: &Outcome, what: &str) -> usize {
    assert!(
        o.winners.len() == 1 && o.conflicts == N - 1,
        "{what}: expected exactly one success and {} conflicts from {N} same-stamp \
         requests; got {} successes and {} conflicts -- statuses {:?}",
        N - 1,
        o.winners.len(),
        o.conflicts,
        o.statuses
    );
    o.winners[0]
}

/// Race `N` requests at every `(id, stamp)` entity, all released
/// together, and return one [`Outcome`] per entity in order.
/// `request(app, id, stamp, i)` builds the `i`th request for an entity.
async fn race_same_stamp<F>(
    app: &Arc<TestApp>,
    entities: &[(String, String)],
    success: StatusCode,
    request: F,
) -> Vec<Outcome>
where
    F: Fn(Arc<TestApp>, String, String, usize) -> Job<StatusCode>,
{
    let mut jobs: Vec<Job<StatusCode>> = Vec::new();
    for (id, t0) in entities {
        for i in 0..N {
            jobs.push(request(app.clone(), id.clone(), t0.clone(), i));
        }
    }
    let statuses = all_at_once(jobs).await;
    statuses
        .chunks(N)
        .map(|c| tally(c.to_vec(), success))
        .collect()
}

async fn stamp(app: &TestApp, table: &str, id: &str) -> String {
    let (updated_at,): (DateTime<Utc>,) =
        sqlx::query_as(&format!("SELECT updated_at FROM {table} WHERE id = ?1"))
            .bind(id)
            .fetch_one(&app.db)
            .await
            .expect("read updated_at");
    updated_at.to_rfc3339()
}

async fn column(app: &TestApp, table: &str, col: &str, id: &str) -> String {
    let (v,): (String,) = sqlx::query_as(&format!(
        "SELECT CAST({col} AS TEXT) FROM {table} WHERE id = ?1"
    ))
    .bind(id)
    .fetch_one(&app.db)
    .await
    .expect("read column");
    v
}

async fn app_with_user() -> (Arc<TestApp>, String) {
    let app = TestApp::spawn().await;
    let user_id = register_and_login(&app, &TestUser::new("alice")).await;
    (Arc::new(app), user_id)
}

// ─────────────────────────────────────────────────────────────
// Same stamp, N requests per entity: one winner, N - 1 conflicts, and
// the loser's value is not in the database
// ─────────────────────────────────────────────────────────────

/// `(id, stamp)` for each id, after the fixtures' creation second has
/// passed.
async fn stamped(app: &TestApp, table: &str, ids: Vec<String>) -> Vec<(String, String)> {
    ensure_distinct_timestamp().await;
    let mut out = Vec::new();
    for id in ids {
        let t0 = stamp(app, table, &id).await;
        out.push((id, t0));
    }
    out
}

async fn issues(app: &TestApp, user_id: &str) -> (String, Vec<String>) {
    let project_id = create_personal_project(&app.db, user_id, "P").await;
    let mut ids = Vec::new();
    for i in 0..M {
        ids.push(create_issue(&app.db, &project_id, user_id, &format!("I{i}")).await);
    }
    (project_id, ids)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn issue_edits_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let (project_id, ids) = issues(&app, &user_id).await;
    let entities = stamped(&app, "issues", ids).await;

    let outcomes = race_same_stamp(&app, &entities, StatusCode::SEE_OTHER, |app, id, t0, i| {
        let project_id = project_id.clone();
        Box::pin(async move {
            app.server
                .post(&format!("/projects/{project_id}/issues/{id}"))
                .form(&[
                    ("title", format!("RACE-{i}").as_str()),
                    ("description", "body"),
                    ("status", "open"),
                    ("priority", "medium"),
                    ("effort", ""),
                    ("assignee_id", ""),
                    ("client_updated_at", t0.as_str()),
                ])
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "issue edit");
        assert_eq!(
            column(&app, "issues", "title", id).await,
            format!("RACE-{winner}"),
            "the stored title must be the winner's, not a loser's"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn issue_status_changes_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let (project_id, ids) = issues(&app, &user_id).await;
    let entities = stamped(&app, "issues", ids).await;
    let statuses = ["in_progress", "done"];

    let outcomes = race_same_stamp(&app, &entities, StatusCode::OK, |app, id, t0, i| {
        let project_id = project_id.clone();
        Box::pin(async move {
            app.server
                .post(&format!("/projects/{project_id}/issues/{id}/status"))
                .json(&serde_json::json!({
                    "status": statuses[i % 2],
                    "client_updated_at": t0,
                }))
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "status change");
        assert_eq!(
            column(&app, "issues", "status", id).await,
            statuses[winner % 2]
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn issue_reschedules_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let (project_id, ids) = issues(&app, &user_id).await;
    let entities = stamped(&app, "issues", ids).await;

    let outcomes = race_same_stamp(&app, &entities, StatusCode::OK, |app, id, t0, i| {
        let project_id = project_id.clone();
        Box::pin(async move {
            app.server
                .post(&format!("/projects/{project_id}/issues/{id}/schedule"))
                .json(&serde_json::json!({
                    "planned_start_at": format!("2026-03-01T09:{i:02}"),
                    "planned_end_at": "",
                    "client_updated_at": t0,
                }))
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "reschedule");
        let stored = column(&app, "issues", "planned_start_at", id).await;
        assert!(
            stored.contains(&format!("09:{winner:02}")),
            "the stored start ({stored}) must be the winner's (09:{winner:02})"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn project_edits_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let mut ids = Vec::new();
    for i in 0..M {
        ids.push(create_personal_project(&app.db, &user_id, &format!("P{i}")).await);
    }
    let entities = stamped(&app, "projects", ids).await;

    let outcomes = race_same_stamp(&app, &entities, StatusCode::SEE_OTHER, |app, id, t0, i| {
        Box::pin(async move {
            app.server
                .post(&format!("/projects/{id}/edit"))
                .form(&[
                    ("name", format!("RACE-{i}").as_str()),
                    ("description", "d"),
                    ("team_id", ""),
                    ("client_updated_at", t0.as_str()),
                ])
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "project edit");
        assert_eq!(
            column(&app, "projects", "name", id).await,
            format!("RACE-{winner}")
        );
    }
}

/// `M` capacity rows with disjoint periods, so no row conflicts with
/// another on overlap.
async fn capacity_rows(app: &TestApp, user_id: &str) -> Vec<String> {
    let day = |m: u32, d: u32| chrono::NaiveDate::from_ymd_opt(2026, m, d).unwrap();
    let mut ids = Vec::new();
    for m in 1..=M as u32 {
        ids.push(
            peisear_storage::user_capacities::insert(
                &app.db,
                user_id,
                5,
                Some(day(m, 1)),
                Some(day(m, 10)),
                None,
            )
            .await
            .expect("row"),
        );
    }
    ids
}

/// Each row's `(period_start, period_end)`, read *before* a race: a
/// request that read them itself could find its row already deleted.
async fn periods_of(
    app: &TestApp,
    entities: &[(String, String)],
) -> std::collections::HashMap<String, (String, String)> {
    let mut out = std::collections::HashMap::new();
    for (id, _) in entities {
        out.insert(
            id.clone(),
            (
                column(app, "user_capacities", "period_start", id).await,
                column(app, "user_capacities", "period_end", id).await,
            ),
        );
    }
    out
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn capacity_edits_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let ids = capacity_rows(&app, &user_id).await;
    let entities = stamped(&app, "user_capacities", ids).await;
    let periods = periods_of(&app, &entities).await;

    let outcomes = race_same_stamp(&app, &entities, StatusCode::SEE_OTHER, |app, id, t0, i| {
        let (start, end) = periods[&id].clone();
        Box::pin(async move {
            app.server
                .post(&format!("/settings/capacity/{id}"))
                .form(&[
                    ("points", (100 + i).to_string().as_str()),
                    ("period_start", start.as_str()),
                    ("period_end", end.as_str()),
                    ("note", ""),
                    ("client_updated_at", t0.as_str()),
                ])
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "capacity edit");
        assert_eq!(
            column(&app, "user_capacities", "points", id).await,
            (100 + winner).to_string()
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn capacity_closes_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let ids = capacity_rows(&app, &user_id).await;
    let entities = stamped(&app, "user_capacities", ids).await;

    let outcomes = race_same_stamp(&app, &entities, StatusCode::SEE_OTHER, |app, id, t0, i| {
        Box::pin(async move {
            let start = column(&app, "user_capacities", "period_start", &id).await;
            // month-01 .. month-10 seeded; close on a distinct later day
            let end = format!("{}-{:02}", &start[..7], 11 + i);
            app.server
                .post(&format!("/settings/capacity/{id}/close"))
                .form(&[
                    ("period_end", end.as_str()),
                    ("client_updated_at", t0.as_str()),
                ])
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "capacity close");
        let start = column(&app, "user_capacities", "period_start", id).await;
        assert_eq!(
            column(&app, "user_capacities", "period_end", id).await,
            format!("{}-{:02}", &start[..7], 11 + winner)
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sprint_edits_with_one_stamp_leave_one_winner() {
    let (app, user_id) = app_with_user().await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Team").await;
    let slug = column(&app, "teams", "slug", &team_id).await;
    let mut ids = Vec::new();
    for i in 0..M {
        ids.push(create_planned_sprint(&app.db, &team_id, &format!("S{i}")).await);
    }
    let entities = stamped(&app, "sprints", ids).await;
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);

    let outcomes = race_same_stamp(&app, &entities, StatusCode::SEE_OTHER, |app, id, t0, i| {
        let slug = slug.clone();
        Box::pin(async move {
            app.server
                .post(&format!("/teams/{slug}/sprints/{id}/edit"))
                .form(&[
                    ("name", format!("RACE-{i}").as_str()),
                    ("starts_on", today.to_string().as_str()),
                    ("ends_on", ends.to_string().as_str()),
                    ("client_updated_at", t0.as_str()),
                ])
                .await
                .status_code()
        })
    })
    .await;
    for ((id, _), o) in entities.iter().zip(&outcomes) {
        let winner = assert_one_winner(o, "sprint edit");
        assert_eq!(
            column(&app, "sprints", "name", id).await,
            format!("RACE-{winner}")
        );
    }
}

// ─────────────────────────────────────────────────────────────
// A delete must not land on a row that changed since the stamp
// ─────────────────────────────────────────────────────────────
//
// Two same-stamp *deletes* were never the problem (the second finds
// the row gone). The delete routes lock for another reason: `QA-006`
// finding 1 -- an issue delete's confirmation names its sub-issue
// count, and a delete after a concurrent edit could remove more than
// the count the user confirmed. So the race that matters is an edit
// and a delete, both carrying one stamp: whichever lands first moves
// it, and the other must then be refused. **Both succeeding is the
// defect**, and so is either answering with a server error.

/// One edit and one delete raced at each of `M` entities. Returns the
/// number of entities where both succeeded and the number where either
/// was answered with a `5xx`.
async fn edit_against_delete<F, G>(
    app: &Arc<TestApp>,
    entities: &[(String, String)],
    edit: F,
    delete: G,
) -> (usize, usize)
where
    F: Fn(Arc<TestApp>, String, String) -> Job<StatusCode>,
    G: Fn(Arc<TestApp>, String, String) -> Job<StatusCode>,
{
    let mut jobs: Vec<Job<StatusCode>> = Vec::new();
    for (id, t0) in entities {
        jobs.push(edit(app.clone(), id.clone(), t0.clone()));
        jobs.push(delete(app.clone(), id.clone(), t0.clone()));
    }
    let statuses = all_at_once(jobs).await;
    let ok = |s: &StatusCode| s.is_success() || s.is_redirection();
    let both = statuses
        .chunks(2)
        .filter(|p| ok(&p[0]) && ok(&p[1]))
        .count();
    let errors = statuses
        .chunks(2)
        .filter(|p| p.iter().any(|s| s.is_server_error()))
        .count();
    (both, errors)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_issue_edit_and_delete_with_one_stamp_never_both_succeed() {
    let (app, user_id) = app_with_user().await;
    let (project_id, ids) = issues(&app, &user_id).await;
    let entities = stamped(&app, "issues", ids).await;
    let (both, errors) = edit_against_delete(
        &app,
        &entities,
        {
            let project_id = project_id.clone();
            move |app, id, t0| {
                let project_id = project_id.clone();
                Box::pin(async move {
                    app.server
                        .post(&format!("/projects/{project_id}/issues/{id}"))
                        .form(&[
                            ("title", "edited"),
                            ("description", "b"),
                            ("status", "open"),
                            ("priority", "medium"),
                            ("effort", ""),
                            ("assignee_id", ""),
                            ("client_updated_at", t0.as_str()),
                        ])
                        .await
                        .status_code()
                })
            }
        },
        {
            let project_id = project_id.clone();
            move |app, id, t0| {
                let project_id = project_id.clone();
                Box::pin(async move {
                    app.server
                        .post(&format!("/projects/{project_id}/issues/{id}/delete"))
                        .form(&[("client_updated_at", t0.as_str())])
                        .await
                        .status_code()
                })
            }
        },
    )
    .await;
    assert_eq!(
        (both, errors),
        (0, 0),
        "issues where an edit and a delete with one stamp both succeeded / where either got a 5xx"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_project_edit_and_delete_with_one_stamp_never_both_succeed() {
    let (app, user_id) = app_with_user().await;
    let mut ids = Vec::new();
    for i in 0..M {
        ids.push(create_personal_project(&app.db, &user_id, &format!("P{i}")).await);
    }
    let entities = stamped(&app, "projects", ids).await;
    let (both, errors) = edit_against_delete(
        &app,
        &entities,
        |app, id, t0| {
            Box::pin(async move {
                app.server
                    .post(&format!("/projects/{id}/edit"))
                    .form(&[
                        ("name", "edited"),
                        ("description", "d"),
                        ("team_id", ""),
                        ("client_updated_at", t0.as_str()),
                    ])
                    .await
                    .status_code()
            })
        },
        |app, id, t0| {
            Box::pin(async move {
                app.server
                    .post(&format!("/projects/{id}/delete"))
                    .form(&[("client_updated_at", t0.as_str())])
                    .await
                    .status_code()
            })
        },
    )
    .await;
    assert_eq!((both, errors), (0, 0), "projects: both succeeded / 5xx");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_capacity_edit_and_delete_with_one_stamp_never_both_succeed() {
    let (app, user_id) = app_with_user().await;
    let ids = capacity_rows(&app, &user_id).await;
    let entities = stamped(&app, "user_capacities", ids).await;
    let periods = periods_of(&app, &entities).await;
    let (both, errors) = edit_against_delete(
        &app,
        &entities,
        |app, id, t0| {
            let (start, end) = periods[&id].clone();
            Box::pin(async move {
                app.server
                    .post(&format!("/settings/capacity/{id}"))
                    .form(&[
                        ("points", "77"),
                        ("period_start", start.as_str()),
                        ("period_end", end.as_str()),
                        ("note", ""),
                        ("client_updated_at", t0.as_str()),
                    ])
                    .await
                    .status_code()
            })
        },
        |app, id, t0| {
            Box::pin(async move {
                app.server
                    .post(&format!("/settings/capacity/{id}/delete"))
                    .form(&[("client_updated_at", t0.as_str())])
                    .await
                    .status_code()
            })
        },
    )
    .await;
    assert_eq!(
        (both, errors),
        (0, 0),
        "capacity rows: both succeeded / 5xx"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_sprint_edit_and_delete_with_one_stamp_never_both_succeed() {
    let (app, user_id) = app_with_user().await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Team").await;
    let slug = column(&app, "teams", "slug", &team_id).await;
    let mut ids = Vec::new();
    for i in 0..M {
        ids.push(create_planned_sprint(&app.db, &team_id, &format!("S{i}")).await);
    }
    let entities = stamped(&app, "sprints", ids).await;
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);
    let (both, errors) = edit_against_delete(
        &app,
        &entities,
        {
            let slug = slug.clone();
            move |app, id, t0| {
                let slug = slug.clone();
                Box::pin(async move {
                    app.server
                        .post(&format!("/teams/{slug}/sprints/{id}/edit"))
                        .form(&[
                            ("name", "edited"),
                            ("starts_on", today.to_string().as_str()),
                            ("ends_on", ends.to_string().as_str()),
                            ("client_updated_at", t0.as_str()),
                        ])
                        .await
                        .status_code()
                })
            }
        },
        {
            let slug = slug.clone();
            move |app, id, t0| {
                let slug = slug.clone();
                Box::pin(async move {
                    app.server
                        .post(&format!("/teams/{slug}/sprints/{id}/delete"))
                        .form(&[("client_updated_at", t0.as_str())])
                        .await
                        .status_code()
                })
            }
        },
    )
    .await;
    assert_eq!((both, errors), (0, 0), "sprints: both succeeded / 5xx");
}

// ─────────────────────────────────────────────────────────────
// What the guard must not change
// ─────────────────────────────────────────────────────────────

/// The guarded write's predicate compares the stamp *before* `0017`'s
/// `AFTER UPDATE` trigger runs, so the trigger must not interfere -- and
/// the stamp the trigger writes must be one the *next* guarded write
/// matches. Four steps on one project, each after the second has moved:
/// a write with the fresh stamp lands and advances it; a write with the
/// stamp that write produced lands; a write with the *first* stamp is
/// refused as `Stale` carrying the current one; and the stamp is stable
/// when read again (the trigger fired once, not on every read).
#[tokio::test]
async fn a_guarded_write_matches_the_stamp_the_trigger_wrote() {
    use peisear_storage::{Guarded, projects};
    let (app, user_id) = app_with_user().await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let read = |app: Arc<TestApp>, id: String| async move {
        let (t,): (DateTime<Utc>,) =
            sqlx::query_as("SELECT updated_at FROM projects WHERE id = ?1")
                .bind(id)
                .fetch_one(&app.db)
                .await
                .unwrap();
        t
    };

    ensure_distinct_timestamp().await;
    let t0 = read(app.clone(), project_id.clone()).await;
    let landed = projects::update_guarded(&app.db, &project_id, &user_id, "one", "d", Some(t0))
        .await
        .unwrap();
    assert_eq!(landed, Guarded::Written(()));
    let t1 = read(app.clone(), project_id.clone()).await;
    assert!(
        t1 > t0,
        "a successful write advances the stamp ({t0} -> {t1})"
    );
    assert!(t1 <= Utc::now(), "and to the present, not beyond it");

    ensure_distinct_timestamp().await;
    let landed = projects::update_guarded(&app.db, &project_id, &user_id, "two", "d", Some(t1))
        .await
        .unwrap();
    assert_eq!(
        landed,
        Guarded::Written(()),
        "the trigger's own stamp must match"
    );
    let t2 = read(app.clone(), project_id.clone()).await;
    assert!(t2 > t1);

    let stale = projects::update_guarded(&app.db, &project_id, &user_id, "three", "d", Some(t0))
        .await
        .unwrap();
    assert_eq!(
        stale,
        Guarded::Stale {
            current_updated_at: t2
        }
    );
    assert_eq!(
        read(app.clone(), project_id.clone()).await,
        t2,
        "a refused write moves nothing"
    );
    assert_eq!(column(&app, "projects", "name", &project_id).await, "two");
}

/// Zero rows is either "not there" or "there, and moved" -- different
/// answers, on every guarded path. A missing row, or one that is not the
/// caller's (the concealment `projects::delete` documents), is still
/// `NotFound` (a `404`); only a present, moved row is `Stale`.
#[tokio::test]
async fn a_guarded_write_tells_a_missing_row_from_a_moved_one() {
    use peisear_storage::{Guarded, StorageError, projects, sprints, user_capacities};
    let (app, user_id) = app_with_user().await;
    let now = Utc::now();

    // missing
    assert!(matches!(
        projects::update_guarded(&app.db, "no-such", &user_id, "n", "d", Some(now)).await,
        Err(StorageError::NotFound)
    ));
    assert!(matches!(
        projects::delete_guarded(&app.db, "no-such", &user_id, Some(now)).await,
        Err(StorageError::NotFound)
    ));
    assert!(matches!(
        user_capacities::delete_guarded(&app.db, &user_id, "no-such", Some(now)).await,
        Err(StorageError::NotFound)
    ));
    assert!(matches!(
        sprints::delete_guarded(&app.db, "no-such", Some(now)).await,
        Err(StorageError::NotFound)
    ));

    // present but somebody else's: concealed as NotFound, never Stale
    let project_id = create_personal_project(&app.db, &user_id, "Mine").await;
    assert!(matches!(
        projects::delete_guarded(&app.db, &project_id, "someone-else", Some(now)).await,
        Err(StorageError::NotFound)
    ));

    // present and moved: Stale, with the current stamp, and nothing written
    let row = user_capacities::insert(&app.db, &user_id, 5, None, None, None)
        .await
        .expect("row");
    let current = {
        let (t,): (DateTime<Utc>,) =
            sqlx::query_as("SELECT updated_at FROM user_capacities WHERE id = ?1")
                .bind(&row)
                .fetch_one(&app.db)
                .await
                .unwrap();
        t
    };
    let long_ago = current - chrono::Duration::days(1);
    assert_eq!(
        user_capacities::delete_guarded(&app.db, &user_id, &row, Some(long_ago))
            .await
            .unwrap(),
        Guarded::Stale {
            current_updated_at: current
        }
    );
    assert!(
        user_capacities::find(&app.db, &user_id, &row)
            .await
            .unwrap()
            .is_some(),
        "a stale delete must leave the row"
    );
}

/// The three surfaces that keep working without a reload -- the board's
/// drag and `dm.js` (status) and `calendar.js` (reschedule) -- act on the
/// stamp the previous response handed back. **The no-reload sequence**
/// (`PLAN-002` round 1's defect was exactly a stamp that was not renewed,
/// and a state check did not catch it): act, act again on the returned
/// stamp with no reload between, act a third time on *that* one; then a
/// request holding the first stamp is a conflict. Each step waits out the
/// second so the stamp really moves.
#[tokio::test]
async fn the_stamp_a_status_or_reschedule_returns_works_for_the_next_action() {
    let (app, user_id) = app_with_user().await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Original").await;
    ensure_distinct_timestamp().await;
    let t0 = stamp(&app, "issues", &issue_id).await;

    let status = |stamp: String, to: &'static str| {
        let (app, project_id, issue_id) = (app.clone(), project_id.clone(), issue_id.clone());
        async move {
            app.server
                .post(&format!("/projects/{project_id}/issues/{issue_id}/status"))
                .json(&serde_json::json!({ "status": to, "client_updated_at": stamp }))
                .await
        }
    };
    let schedule = |stamp: String, start: &'static str| {
        let (app, project_id, issue_id) = (app.clone(), project_id.clone(), issue_id.clone());
        async move {
            app.server
                .post(&format!(
                    "/projects/{project_id}/issues/{issue_id}/schedule"
                ))
                .json(&serde_json::json!({
                    "planned_start_at": start,
                    "planned_end_at": "",
                    "client_updated_at": stamp,
                }))
                .await
        }
    };
    let returned = |resp: &axum_test::TestResponse| -> String {
        resp.json::<serde_json::Value>()["updated_at"]
            .as_str()
            .expect("the response carries updated_at")
            .to_string()
    };

    let r1 = status(t0.clone(), "in_progress").await;
    assert_eq!(r1.status_code(), StatusCode::OK);
    let t1 = returned(&r1);
    assert_ne!(t1, t0, "the returned stamp is the new one");
    assert_eq!(
        t1,
        stamp(&app, "issues", &issue_id).await,
        "and it is the row's current one"
    );

    ensure_distinct_timestamp().await;
    let r2 = schedule(t1.clone(), "2026-03-01T09:00").await;
    assert_eq!(
        r2.status_code(),
        StatusCode::OK,
        "second action on the returned stamp, no reload"
    );
    let t2 = returned(&r2);
    assert_ne!(t2, t1);

    ensure_distinct_timestamp().await;
    let r3 = status(t2.clone(), "done").await;
    assert_eq!(
        r3.status_code(),
        StatusCode::OK,
        "third action on the stamp the second returned"
    );
    assert_eq!(returned(&r3), stamp(&app, "issues", &issue_id).await);

    let stale = status(t0, "open").await;
    assert_eq!(
        stale.status_code(),
        StatusCode::CONFLICT,
        "the first stamp is now stale"
    );
    assert_eq!(column(&app, "issues", "status", &issue_id).await, "done");
}
