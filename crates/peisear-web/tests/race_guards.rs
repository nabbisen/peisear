//! `RACE-001` — guards that read a condition on one connection and
//! write on another, with no transaction across the two, so that two
//! concurrent requests both pass. `CAP-001` fixed the first instance
//! (`user_capacities`) and set the pattern: `BEGIN IMMEDIATE`, the guard
//! read through the transaction, the write on the same transaction.
//! Three more sites had the shape, and this file holds one concurrent
//! test each, plus the sequential and non-conflicting cases that show
//! the fix did not change what a single request sees:
//!
//! 1. **The last-admin guard** (`teams::update_role` and
//!    `teams::remove_member`) — two admins demoting or removing each
//!    other at once left a team with no administrator, and nobody left
//!    who could appoint one.
//! 2. **`sprints::start`** — two starts on one team left two `active`
//!    sprints, a state the function's own doc says cannot exist.
//! 3. **`user_capacities::close_at`** — a read-modify-write that wrote
//!    back a `points` value it had read before a concurrent edit.
//!
//! `RACE-003` and `SPRINT-001` add to the end of this file (`SPRINT-001`'s
//! first half, refusing an unassign from a completed sprint, was reverted
//! by `SPRINT-004` once the sprint's record was captured at completion; its
//! `plan_remove` half stands).
//!
//! `RACE-003` adds two smaller ones at the end of this file: an issue
//! joining a sprint whose status changed under it, and a concurrent
//! duplicate team membership answered with a raw database error.
//!
//! Every concurrent test drives real tasks on a multi-thread runtime,
//! released together by a barrier so the requests genuinely overlap.
//! Tests that need the race to fire *reliably* run many independent
//! pairs (one per team, sprint set or row) in one release, rather than
//! relying on a single pair overlapping.

mod common;

use axum::http::StatusCode;
use chrono::NaiveDate;
use common::auth::{TestUser, login, register};
use common::fixture::{create_issue, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_i18n::MessageKey;
use peisear_storage::{StorageError, sprints, teams, user_capacities};
use std::sync::Arc;
use tokio::sync::Barrier;

/// Independent racing pairs (or sets) per test. Twelve is `CAP-001`'s N.
const N: usize = 12;

/// Release every future in `jobs` at the same instant and collect the
/// results in order.
async fn all_at_once<T: Send + 'static>(
    jobs: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>>,
) -> Vec<T> {
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

fn is_conflict<T>(r: &Result<T, StorageError>) -> bool {
    matches!(r, Err(StorageError::Conflict(_)))
}

async fn user(app: &TestApp, hint: &str) -> (TestUser, String) {
    let u = TestUser::new(hint);
    let id = register(app, &u).await;
    (u, id)
}

/// `N` teams, each with exactly two admins (`a` and `b`). Returns the
/// team ids.
async fn teams_with_two_admins(app: &TestApp, a: &str, b: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for i in 0..N {
        let team_id = create_team_with_admin(&app.db, a, &format!("Team {i}")).await;
        teams::add_member(&app.db, &team_id, b, TeamRole::Admin)
            .await
            .expect("second admin");
        ids.push(team_id);
    }
    ids
}

async fn admins_in(app: &TestApp, team_id: &str) -> i64 {
    teams::admin_count(&app.db, team_id)
        .await
        .expect("admin count")
}

// ─────────────────────────────────────────────────────────────
// 2.1 — the last-admin guard
// ─────────────────────────────────────────────────────────────

/// Each of `N` teams has exactly two admins, and both are demoted at
/// once. One demotion may land; the other must be refused, and **one
/// admin must remain in every team**.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_admins_demoted_at_once_leave_one() {
    let app = TestApp::spawn().await;
    let (_, a) = user(&app, "alice").await;
    let (_, b) = user(&app, "bob").await;
    let team_ids = teams_with_two_admins(&app, &a, &b).await;

    let mut jobs = Vec::new();
    for team_id in &team_ids {
        for target in [&a, &b] {
            let (db, team_id, target) = (app.db.clone(), team_id.clone(), target.clone());
            jobs.push(Box::pin(async move {
                teams::update_role(&db, &team_id, &target, TeamRole::Member).await
            })
                as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>);
        }
    }
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;

    let mut zero_admin_teams = 0;
    for (i, team_id) in team_ids.iter().enumerate() {
        let pair = &results[2 * i..2 * i + 2];
        let ok = pair.iter().filter(|r| r.is_ok()).count();
        let refused = pair.iter().filter(|r| is_conflict(r)).count();
        let admins = admins_in(&app, team_id).await;
        if admins == 0 {
            zero_admin_teams += 1;
        }
        assert_eq!(
            (ok, refused, admins),
            (1, 1, 1),
            "team {i}: one demotion, one refusal, one admin left; got {pair:?}"
        );
    }
    assert_eq!(zero_admin_teams, 0, "teams left with no administrator");
}

/// The realistic path: two admins both leave a team at once.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_admins_removed_at_once_leave_one() {
    let app = TestApp::spawn().await;
    let (_, a) = user(&app, "alice").await;
    let (_, b) = user(&app, "bob").await;
    let team_ids = teams_with_two_admins(&app, &a, &b).await;

    let mut jobs = Vec::new();
    for team_id in &team_ids {
        for target in [&a, &b] {
            let (db, team_id, target) = (app.db.clone(), team_id.clone(), target.clone());
            jobs.push(
                Box::pin(async move { teams::remove_member(&db, &team_id, &target).await })
                    as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>,
            );
        }
    }
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;

    for (i, team_id) in team_ids.iter().enumerate() {
        let pair = &results[2 * i..2 * i + 2];
        let ok = pair.iter().filter(|r| r.is_ok()).count();
        let refused = pair.iter().filter(|r| is_conflict(r)).count();
        let admins = admins_in(&app, team_id).await;
        assert_eq!(
            (ok, refused, admins),
            (1, 1, 1),
            "team {i}: one removal, one refusal, one admin left; got {pair:?}"
        );
    }
}

/// The same race through the HTTP handlers, each admin on their own
/// session: two admins pressing *Leave team* at once. This is the path
/// a user can actually reach.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_admins_leaving_at_once_over_http_leave_one() {
    let app = TestApp::spawn().await;
    let (ua, a) = user(&app, "alice").await;
    let (ub, b) = user(&app, "bob").await;
    let team_ids = teams_with_two_admins(&app, &a, &b).await;

    // One session cookie per admin.
    let cookie_of = |resp: axum_test::TestResponse| {
        resp.cookies()
            .iter()
            .next()
            .expect("a session cookie")
            .clone()
            .into_owned()
    };
    let ca = cookie_of(
        app.server
            .post("/login")
            .form(&[
                ("email", ua.email.as_str()),
                ("password", ua.password.as_str()),
            ])
            .await,
    );
    let cb = cookie_of(
        app.server
            .post("/login")
            .form(&[
                ("email", ub.email.as_str()),
                ("password", ub.password.as_str()),
            ])
            .await,
    );
    let slugs: Vec<String> = {
        let mut v = Vec::new();
        for id in &team_ids {
            v.push(teams::find_by_id(&app.db, id).await.unwrap().unwrap().slug);
        }
        v
    };

    let app = Arc::new(app);
    let mut jobs = Vec::new();
    for slug in &slugs {
        for (me, cookie) in [(&a, &ca), (&b, &cb)] {
            let (app, slug, me, cookie) = (app.clone(), slug.clone(), me.clone(), cookie.clone());
            jobs.push(Box::pin(async move {
                app.server
                    .post(&format!("/teams/{slug}/members/{me}/remove"))
                    .clear_cookies()
                    .add_cookie(cookie)
                    .await
                    .header("location")
                    .to_str()
                    .expect("location")
                    .to_string()
            })
                as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>);
        }
    }
    let locations = all_at_once::<String>(jobs).await;

    for (i, team_id) in team_ids.iter().enumerate() {
        let pair = &locations[2 * i..2 * i + 2];
        let admins = admins_in(&app, team_id).await;
        assert_eq!(
            admins, 1,
            "team {i}: exactly one admin must remain; redirects {pair:?}"
        );
        let refused = pair.iter().filter(|l| l.contains("error=")).count();
        assert_eq!(
            refused, 1,
            "team {i}: exactly one refusal; redirects {pair:?}"
        );
    }
}

/// Sequential behaviour, unchanged: the last admin cannot demote or
/// remove themselves, and the redirect carries the same message and
/// shape as before. (No earlier test covered either refusal.)
#[tokio::test]
async fn the_last_admin_is_refused_with_the_same_messages() {
    let app = TestApp::spawn().await;
    let (u, admin_id) = user(&app, "alice").await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Solo").await;
    let slug = teams::find_by_id(&app.db, &team_id)
        .await
        .unwrap()
        .unwrap()
        .slug;
    login(&app, &u).await;

    let demote = app
        .server
        .post(&format!("/teams/{slug}/members/{admin_id}/role"))
        .form(&[("role", "member")])
        .await;
    demote.assert_status(StatusCode::SEE_OTHER);
    // Decode the redirect's query string rather than re-implementing the
    // handler's encoder: the message a user sees is what must not change.
    let error_of = |resp: &axum_test::TestResponse| -> (String, String) {
        let loc = resp.header("location").to_str().unwrap().to_string();
        let (path, query) = loc.split_once('?').expect("a query string");
        let q: std::collections::HashMap<String, String> =
            serde_urlencoded::from_str(query).expect("decode query");
        (
            path.to_string(),
            q.get("error").cloned().unwrap_or_default(),
        )
    };
    let render = |key: MessageKey| peisear_i18n::Locale::English.render(key);
    assert_eq!(
        error_of(&demote),
        (
            format!("/teams/{slug}"),
            render(MessageKey::LastAdminDemotionError)
        )
    );

    let remove = app
        .server
        .post(&format!("/teams/{slug}/members/{admin_id}/remove"))
        .await;
    remove.assert_status(StatusCode::SEE_OTHER);
    assert_eq!(
        error_of(&remove),
        (
            format!("/teams/{slug}"),
            render(MessageKey::LastAdminRemovalError)
        )
    );

    assert_eq!(admins_in(&app, &team_id).await, 1, "nothing was written");
}

/// A demotion that leaves an admin behind still works, and so does a
/// non-admin leaving: the guard must not refuse what it always allowed.
#[tokio::test]
async fn demoting_one_of_two_admins_and_a_member_leaving_still_work() {
    let app = TestApp::spawn().await;
    let (u, a) = user(&app, "alice").await;
    let (_, b) = user(&app, "bob").await;
    let (_, c) = user(&app, "carol").await;
    let team_id = create_team_with_admin(&app.db, &a, "Trio").await;
    teams::add_member(&app.db, &team_id, &b, TeamRole::Admin)
        .await
        .unwrap();
    teams::add_member(&app.db, &team_id, &c, TeamRole::Member)
        .await
        .unwrap();
    let slug = teams::find_by_id(&app.db, &team_id)
        .await
        .unwrap()
        .unwrap()
        .slug;
    login(&app, &u).await;

    let demote = app
        .server
        .post(&format!("/teams/{slug}/members/{b}/role"))
        .form(&[("role", "viewer")])
        .await;
    assert!(
        !demote
            .header("location")
            .to_str()
            .unwrap()
            .contains("error="),
        "demoting one of two admins must succeed"
    );
    assert_eq!(admins_in(&app, &team_id).await, 1);

    let remove = app
        .server
        .post(&format!("/teams/{slug}/members/{c}/remove"))
        .await;
    assert!(
        !remove
            .header("location")
            .to_str()
            .unwrap()
            .contains("error=")
    );
    assert!(
        teams::role_for(&app.db, &team_id, &c)
            .await
            .unwrap()
            .is_none()
    );
}

/// Concurrent operations that do not conflict all succeed: two
/// *different* members of a five-admin team demoted at once.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn demoting_two_different_admins_of_a_five_admin_team_both_succeed() {
    let app = TestApp::spawn().await;
    let mut ids = Vec::new();
    for i in 0..5 {
        ids.push(user(&app, &format!("admin{i}")).await.1);
    }
    let team_id = create_team_with_admin(&app.db, &ids[0], "Five").await;
    for id in &ids[1..] {
        teams::add_member(&app.db, &team_id, id, TeamRole::Admin)
            .await
            .unwrap();
    }
    let jobs: Vec<_> = [&ids[1], &ids[2]]
        .into_iter()
        .map(|target| {
            let (db, team_id, target) = (app.db.clone(), team_id.clone(), target.clone());
            Box::pin(
                async move { teams::update_role(&db, &team_id, &target, TeamRole::Member).await },
            ) as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>
        })
        .collect();
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;
    assert!(results.iter().all(|r| r.is_ok()), "{results:?}");
    assert_eq!(admins_in(&app, &team_id).await, 3);
}

// ─────────────────────────────────────────────────────────────
// 2.2 — sprints::start
// ─────────────────────────────────────────────────────────────

async fn planned_sprint(app: &TestApp, team_id: &str, name: &str) -> String {
    let day = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
    sprints::insert(
        &app.db,
        team_id,
        name,
        None,
        day,
        day + chrono::Duration::days(14),
    )
    .await
    .expect("insert sprint")
}

async fn active_count(app: &TestApp, team_id: &str) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM sprints WHERE team_id = ?1 AND status = 'active'")
        .bind(team_id)
        .fetch_one(&app.db)
        .await
        .expect("count active")
}

/// `N` planned sprints of one team, all started at once: exactly one
/// becomes active and the rest are refused.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sprints_of_one_team_started_at_once_leave_one_active() {
    let app = TestApp::spawn().await;
    let (_, admin) = user(&app, "alice").await;
    let team_id = create_team_with_admin(&app.db, &admin, "Team").await;
    let mut sprint_ids = Vec::new();
    for i in 0..N {
        sprint_ids.push(planned_sprint(&app, &team_id, &format!("Sprint {i}")).await);
    }
    let jobs: Vec<_> = sprint_ids
        .iter()
        .map(|id| {
            let (db, id) = (app.db.clone(), id.clone());
            Box::pin(async move { sprints::start(&db, &id).await })
                as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>
        })
        .collect();
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;

    let ok = results.iter().filter(|r| r.is_ok()).count();
    let refused = results.iter().filter(|r| is_conflict(r)).count();
    let active = active_count(&app, &team_id).await;
    assert_eq!(
        (ok, refused, active),
        (1, N - 1, 1),
        "one winner, {} refusals, one active sprint; results {results:?}",
        N - 1
    );
}

/// Sequential behaviour, unchanged: a second start is refused naming
/// the sprint that is already active; starting an active sprint says it
/// is already active.
#[tokio::test]
async fn starting_a_second_sprint_is_refused_naming_the_active_one() {
    let app = TestApp::spawn().await;
    let (_, admin) = user(&app, "alice").await;
    let team_id = create_team_with_admin(&app.db, &admin, "Team").await;
    let first = planned_sprint(&app, &team_id, "First").await;
    let second = planned_sprint(&app, &team_id, "Second").await;

    sprints::start(&app.db, &first).await.expect("first start");
    match sprints::start(&app.db, &second).await {
        Err(StorageError::Conflict(MessageKey::OtherSprintActiveInTeamMessage { sprint_name })) => {
            assert_eq!(sprint_name, "First")
        }
        other => panic!("expected the conflict naming the active sprint, got {other:?}"),
    }
    assert!(matches!(
        sprints::start(&app.db, &first).await,
        Err(StorageError::Validation(
            MessageKey::SprintAlreadyActiveMessage
        ))
    ));
    assert_eq!(active_count(&app, &team_id).await, 1);
}

/// Sprints in *different* teams start concurrently without interfering.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sprints_of_different_teams_started_at_once_all_start() {
    let app = TestApp::spawn().await;
    let (_, admin) = user(&app, "alice").await;
    let mut sprint_ids = Vec::new();
    for i in 0..N {
        let team_id = create_team_with_admin(&app.db, &admin, &format!("Team {i}")).await;
        sprint_ids.push(planned_sprint(&app, &team_id, "S").await);
    }
    let jobs: Vec<_> = sprint_ids
        .iter()
        .map(|id| {
            let (db, id) = (app.db.clone(), id.clone());
            Box::pin(async move { sprints::start(&db, &id).await })
                as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>
        })
        .collect();
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;
    assert!(results.iter().all(|r| r.is_ok()), "{results:?}");
}

// ─────────────────────────────────────────────────────────────
// 2.3 — user_capacities::close_at
// ─────────────────────────────────────────────────────────────

/// `close_at` used to `find` the row and then `update` it with the
/// `points` it had just read. A concurrent edit of `points` landing
/// between the two was overwritten with the stale value.
///
/// `N` rows, each with a `close_at` and a `points` edit racing. The
/// edit keeps the row's period, so either order is legal and **the
/// edit's `points` must survive in every row**: close-then-edit ends
/// with 99 and an open period; edit-then-close ends with 99 and a
/// closed one. The lost update ends with the old value.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn close_at_does_not_overwrite_a_concurrent_points_edit() {
    let app = TestApp::spawn().await;
    let (_, uid) = user(&app, "alice").await;
    let day = |m: u32, d: u32| NaiveDate::from_ymd_opt(2026, m, d).unwrap();

    let mut row_ids = Vec::new();
    for m in 1..=N as u32 {
        // Twelve disjoint periods, one per row, so no row conflicts with another.
        let id =
            user_capacities::insert(&app.db, &uid, 10, Some(day(m, 1)), Some(day(m, 28)), None)
                .await
                .expect("seed row");
        row_ids.push(id);
    }

    let mut jobs = Vec::new();
    for (i, id) in row_ids.iter().enumerate() {
        let m = i as u32 + 1;
        let (db, uid, id) = (app.db.clone(), uid.clone(), id.clone());
        // close_at moves the end from the 28th to the 20th
        jobs.push(Box::pin({
            let (db, uid, id) = (db.clone(), uid.clone(), id.clone());
            async move {
                user_capacities::close_at(&db, &uid, &id, day(m, 20))
                    .await
                    .map(|_| ())
            }
        })
            as std::pin::Pin<Box<dyn std::future::Future<Output = _> + Send>>);
        // the edit changes points only, keeping the row's period
        jobs.push(Box::pin(async move {
            user_capacities::update(&db, &uid, &id, 99, Some(day(m, 1)), Some(day(m, 28)), None)
                .await
        }));
    }
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;
    assert!(
        results.iter().all(|r| r.is_ok()),
        "neither operation conflicts with the other: {results:?}"
    );

    let mut lost = Vec::new();
    for id in &row_ids {
        let row = user_capacities::find(&app.db, &uid, id)
            .await
            .expect("find")
            .expect("row exists");
        if row.points != 99 {
            lost.push((row.period_end, row.points));
        }
    }
    assert!(
        lost.is_empty(),
        "{} of {N} rows lost the concurrent points edit (period_end, points): {lost:?}",
        lost.len()
    );
}

// ─────────────────────────────────────────────────────────────
// RACE-003 §2 — teams::add_member: the right state, the right words
// ─────────────────────────────────────────────────────────────

/// `N` simultaneous adds of one user to one team: one succeeds, every
/// other is refused with **`UserAlreadyTeamMemberMessage`** -- the message,
/// not a raw database error -- and exactly one membership row exists.
/// The primary key already kept the *state* right; the message is what
/// this asserts.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_duplicate_adds_get_the_already_a_member_message() {
    let app = TestApp::spawn().await;
    let (_, admin) = user(&app, "alice").await;
    let (_, newcomer) = user(&app, "bob").await;
    let team_id = create_team_with_admin(&app.db, &admin, "Team").await;

    let jobs: Vec<_> = (0..N)
        .map(|_| {
            let (db, team_id, newcomer) = (app.db.clone(), team_id.clone(), newcomer.clone());
            Box::pin(
                async move { teams::add_member(&db, &team_id, &newcomer, TeamRole::Member).await },
            )
                as std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), StorageError>> + Send>,
                >
        })
        .collect();
    let results = all_at_once::<Result<(), StorageError>>(jobs).await;

    let ok = results.iter().filter(|r| r.is_ok()).count();
    let owed = results
        .iter()
        .filter(|r| {
            matches!(
                r,
                Err(StorageError::Conflict(MessageKey::UserAlreadyTeamMemberMessage { user_id }))
                    if *user_id == newcomer
            )
        })
        .count();
    let rows: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM team_memberships WHERE team_id = ?1 AND user_id = ?2",
    )
    .bind(&team_id)
    .bind(&newcomer)
    .fetch_one(&app.db)
    .await
    .unwrap();
    assert_eq!(
        (ok, owed, rows),
        (1, N - 1, 1),
        "one add, {} refusals carrying the already-a-member message, one row; got {results:?}",
        N - 1
    );
}

/// Sequential behaviour, unchanged: adding an existing member is refused
/// with the same message, and the existing role is not touched.
#[tokio::test]
async fn adding_an_existing_member_is_refused_with_the_same_message() {
    let app = TestApp::spawn().await;
    let (_, admin) = user(&app, "alice").await;
    let (_, member) = user(&app, "bob").await;
    let team_id = create_team_with_admin(&app.db, &admin, "Team").await;
    teams::add_member(&app.db, &team_id, &member, TeamRole::Viewer)
        .await
        .expect("first add");

    match teams::add_member(&app.db, &team_id, &member, TeamRole::Admin).await {
        Err(StorageError::Conflict(MessageKey::UserAlreadyTeamMemberMessage { user_id })) => {
            assert_eq!(user_id, member)
        }
        other => panic!("expected the already-a-member message, got {other:?}"),
    }
    assert_eq!(
        teams::role_for(&app.db, &team_id, &member).await.unwrap(),
        Some(TeamRole::Viewer),
        "the refused add must not change the existing role"
    );
}

// ─────────────────────────────────────────────────────────────
// RACE-003 §1 — an issue must not join a sprint whose status has
// changed under it
// ─────────────────────────────────────────────────────────────
//
// Two routes add an issue to a sprint and both read the sprint's status
// first: `plan_add` (only a `planned` sprint), and `assign_issue` (any
// sprint but a `completed` one). Neither carries an optimistic lock, so
// nothing else closes the gap between the read and the `INSERT`.
//
// **Why these are not "N pairs released together".** For an add racing a
// status change, both serial orders can be legal *and leave the same
// state*: the add landing just before the change and the add landing just
// after it (the bug) both end with the issue in the sprint, and a barrier
// cannot tell which happened. So the interleaving is **forced**: the test
// holds the database write lock on one connection, fires the requests
// (each passes its status read, then blocks at its write), changes the
// sprint's status inside the held transaction, and only then commits -- so
// every request's write runs strictly after the status change, which is
// exactly the interleaving the bug needs.

/// Requests held at the write per test. Fewer than the pool's eight
/// connections minus the one holding the lock, so every request gets past
/// its own status read before the change lands.
const HELD: usize = 6;

/// A team, its admin (logged in), a team project, `HELD` open issues
/// and a sprint in `status`.
async fn sprint_with_issues(
    app: &TestApp,
    status: &str,
) -> (String, String, String, String, Vec<String>) {
    let (_, admin) = user(app, "alice").await;
    let team_id = create_team_with_admin(&app.db, &admin, "Team").await;
    let slug = teams::find_by_id(&app.db, &team_id)
        .await
        .unwrap()
        .unwrap()
        .slug;
    let project_id = create_team_project(&app.db, &admin, &team_id, "Proj").await;
    let sprint_id = planned_sprint(app, &team_id, "S").await;
    if status != "planned" {
        // seed one member so a sprint can complete; then walk the state machine
        let seed = create_issue(&app.db, &project_id, &admin, "seed").await;
        sprints::add_issue(&app.db, &sprint_id, &seed)
            .await
            .unwrap();
        sprints::start(&app.db, &sprint_id).await.unwrap();
    }
    let mut issue_ids = Vec::new();
    for i in 0..HELD {
        issue_ids.push(create_issue(&app.db, &project_id, &admin, &format!("I{i}")).await);
    }
    (slug, team_id, project_id, sprint_id, issue_ids)
}

/// Fire `requests` while a write lock is held, run `change` inside the held
/// transaction, commit, and return the responses.
async fn requests_landing_after<T: Send + 'static>(
    app: &Arc<TestApp>,
    requests: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>>,
    change: &str,
    sprint_id: &str,
) -> Vec<T> {
    let mut tx = app
        .db
        .begin_with("BEGIN IMMEDIATE")
        .await
        .expect("hold the write lock");
    let handles: Vec<_> = requests.into_iter().map(tokio::spawn).collect();
    // Let every request run its status read and block at its write.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    sqlx::query(change)
        .bind(sprint_id)
        .execute(&mut *tx)
        .await
        .expect("change the sprint's status under them");
    tx.commit().await.expect("release the lock");
    let mut out = Vec::new();
    for h in handles {
        out.push(h.await.expect("request task"));
    }
    out
}

async fn members_of_sprint(app: &TestApp, sprint_id: &str) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM sprint_issues WHERE sprint_id = ?1")
        .bind(sprint_id)
        .fetch_one(&app.db)
        .await
        .expect("count sprint members")
}

/// `plan_add` reads a `planned` sprint; a `start` lands before its write.
/// The plan is no longer editable, so **no issue may be added**.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn plan_add_does_not_add_to_a_sprint_started_under_it() {
    let app = TestApp::spawn().await;
    let (slug, _team, project_id, sprint_id, issue_ids) = sprint_with_issues(&app, "planned").await;
    let app = Arc::new(app);

    let requests: Vec<_> = issue_ids
        .iter()
        .map(|issue_id| {
            let (app, slug, sprint_id, project_id, issue_id) = (
                app.clone(),
                slug.clone(),
                sprint_id.clone(),
                project_id.clone(),
                issue_id.clone(),
            );
            Box::pin(async move {
                app.server
                    .post(&format!("/teams/{slug}/sprints/{sprint_id}/plan/add"))
                    .form(&[
                        ("issue_id", issue_id.as_str()),
                        ("project_id", project_id.as_str()),
                    ])
                    .await
                    .status_code()
            })
                as std::pin::Pin<Box<dyn std::future::Future<Output = StatusCode> + Send>>
        })
        .collect();
    let statuses = requests_landing_after(
        &app,
        requests,
        "UPDATE sprints SET status = 'active', started_at = CURRENT_TIMESTAMP WHERE id = ?1",
        &sprint_id,
    )
    .await;

    assert_eq!(
        (members_of_sprint(&app, &sprint_id).await, statuses.clone()),
        (0, vec![StatusCode::BAD_REQUEST; HELD]),
        "no issue may join a sprint that started under the request, and each is \
         refused with the plan-not-editable validation (400)"
    );
}

/// `assign_issue` reads an `active` sprint; a `complete` lands before its
/// write. A completed sprint's summary must stay stable, so **no issue may
/// be added**.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn assigning_an_issue_does_not_join_a_sprint_completed_under_it() {
    let app = TestApp::spawn().await;
    let (_slug, _team, project_id, sprint_id, issue_ids) = sprint_with_issues(&app, "active").await;
    let seeded = members_of_sprint(&app, &sprint_id).await;
    let app = Arc::new(app);

    let requests: Vec<_> = issue_ids
        .iter()
        .map(|issue_id| {
            let (app, sprint_id, project_id, issue_id) = (
                app.clone(),
                sprint_id.clone(),
                project_id.clone(),
                issue_id.clone(),
            );
            Box::pin(async move {
                app.server
                    .post(&format!("/projects/{project_id}/issues/{issue_id}/sprint"))
                    .form(&[("sprint_id", sprint_id.as_str())])
                    .await
                    .status_code()
            })
                as std::pin::Pin<Box<dyn std::future::Future<Output = StatusCode> + Send>>
        })
        .collect();
    let statuses = requests_landing_after(
        &app,
        requests,
        "UPDATE sprints SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
        &sprint_id,
    )
    .await;

    assert_eq!(
        (members_of_sprint(&app, &sprint_id).await, statuses.clone()),
        (seeded, vec![StatusCode::BAD_REQUEST; HELD]),
        "no issue may join a sprint completed under the request, and each is refused (400)"
    );
}

/// Sequential behaviour and copy, unchanged, for the two refusals the
/// storage check now also makes: assigning to a completed sprint, and
/// adding to a sprint that is no longer planned. (`sprint_plan.rs`'s
/// `plan_add_rejects_a_completed_sprint` covers the completed case of the
/// second by its status; the messages are pinned here.)
#[tokio::test]
async fn the_sequential_refusals_keep_their_messages() {
    let app = TestApp::spawn().await;
    let (slug, _team, project_id, sprint_id, issue_ids) = sprint_with_issues(&app, "active").await;

    // plan_add on an active sprint: the plan-not-editable message
    let resp = app
        .server
        .post(&format!("/teams/{slug}/sprints/{sprint_id}/plan/add"))
        .form(&[
            ("issue_id", issue_ids[0].as_str()),
            ("project_id", project_id.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::BAD_REQUEST);
    assert!(
        resp.text().contains(
            &peisear_i18n::Locale::English.render(MessageKey::SprintPlanNotEditableMessage)
        ),
        "plan_add's refusal must carry its message"
    );

    // assign_issue to a completed sprint: the cannot-assign message
    sprints::complete(&app.db, &sprint_id).await.unwrap();
    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{}/sprint",
            issue_ids[1]
        ))
        .form(&[("sprint_id", sprint_id.as_str())])
        .await;
    resp.assert_status(StatusCode::BAD_REQUEST);
    assert!(
        resp.text().contains(
            &peisear_i18n::Locale::English.render(MessageKey::CannotAssignToCompletedSprintMessage)
        ),
        "assign_issue's refusal must carry its message"
    );
}

// ─────────────────────────────────────────────────────────────
// SPRINT-001 §2 (and what it left standing) -- see DEC-054
// ─────────────────────────────────────────────────────────────

/// What must still work: unassigning from a planned sprint and from an
/// active one, `plan_remove` on a planned sprint, and assigning an issue
/// across sprints. (Membership of a *completed* sprint may also change,
/// since `SPRINT-004`; that is covered in `sprint_record.rs`.)
#[tokio::test]
async fn removal_and_reassignment_still_work_on_planned_and_active_sprints() {
    let app = TestApp::spawn().await;
    let (u, admin) = user(&app, "alice").await;
    let team_id = create_team_with_admin(&app.db, &admin, "Team").await;
    let slug = teams::find_by_id(&app.db, &team_id)
        .await
        .unwrap()
        .unwrap()
        .slug;
    let project_id = create_team_project(&app.db, &admin, &team_id, "Proj").await;
    login(&app, &u).await;

    let unassign = |issue: String| {
        let (project_id,) = (project_id.clone(),);
        let app = &app;
        async move {
            app.server
                .post(&format!("/projects/{project_id}/issues/{issue}/sprint"))
                .form(&[("sprint_id", "")])
                .await
                .status_code()
        }
    };

    // planned sprint: unassign
    let planned = planned_sprint(&app, &team_id, "planned").await;
    let i1 = create_issue(&app.db, &project_id, &admin, "i1").await;
    sprints::add_issue(&app.db, &planned, &i1).await.unwrap();
    assert_eq!(unassign(i1.clone()).await, StatusCode::SEE_OTHER);
    assert_eq!(members_of_sprint(&app, &planned).await, 0);

    // planned sprint: plan_remove
    let i2 = create_issue(&app.db, &project_id, &admin, "i2").await;
    sprints::add_issue(&app.db, &planned, &i2).await.unwrap();
    let resp = app
        .server
        .post(&format!("/teams/{slug}/sprints/{planned}/plan/remove"))
        .form(&[("issue_id", i2.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    assert_eq!(members_of_sprint(&app, &planned).await, 0);

    // assign across sprints: the issue moves
    let other = planned_sprint(&app, &team_id, "other").await;
    let i3 = create_issue(&app.db, &project_id, &admin, "i3").await;
    sprints::add_issue(&app.db, &planned, &i3).await.unwrap();
    let resp = app
        .server
        .post(&format!("/projects/{project_id}/issues/{i3}/sprint"))
        .form(&[("sprint_id", other.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    assert_eq!(
        (
            members_of_sprint(&app, &planned).await,
            members_of_sprint(&app, &other).await
        ),
        (0, 1)
    );

    // active sprint: unassign
    sprints::start(&app.db, &other).await.unwrap();
    assert_eq!(unassign(i3.clone()).await, StatusCode::SEE_OTHER);
    assert_eq!(members_of_sprint(&app, &other).await, 0);

    // an issue in no sprint: unassign is still a no-op, not an error
    assert_eq!(unassign(i3).await, StatusCode::SEE_OTHER);
}

/// **§2** -- `plan_remove` reads a `planned` sprint; a `start` lands before
/// its write. The plan is no longer editable, so **no issue may be removed**.
/// Same forced interleaving as `plan_add`'s (see the RACE-003 block above
/// for why a barrier cannot see this race).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn plan_remove_does_not_remove_from_a_sprint_started_under_it() {
    let app = TestApp::spawn().await;
    let (slug, _team, _project, sprint_id, issue_ids) = sprint_with_issues(&app, "planned").await;
    for id in &issue_ids {
        sprints::add_issue(&app.db, &sprint_id, id).await.unwrap();
    }
    let app = Arc::new(app);

    let requests: Vec<_> = issue_ids
        .iter()
        .map(|issue_id| {
            let (app, slug, sprint_id, issue_id) = (
                app.clone(),
                slug.clone(),
                sprint_id.clone(),
                issue_id.clone(),
            );
            Box::pin(async move {
                app.server
                    .post(&format!("/teams/{slug}/sprints/{sprint_id}/plan/remove"))
                    .form(&[("issue_id", issue_id.as_str())])
                    .await
                    .status_code()
            })
                as std::pin::Pin<Box<dyn std::future::Future<Output = StatusCode> + Send>>
        })
        .collect();
    let statuses = requests_landing_after(
        &app,
        requests,
        "UPDATE sprints SET status = 'active', started_at = CURRENT_TIMESTAMP WHERE id = ?1",
        &sprint_id,
    )
    .await;

    assert_eq!(
        (members_of_sprint(&app, &sprint_id).await, statuses.clone()),
        (HELD as i64, vec![StatusCode::BAD_REQUEST; HELD]),
        "no issue may be removed from a sprint that started under the request, and each is \
         refused with the plan-not-editable validation (400)"
    );
}
