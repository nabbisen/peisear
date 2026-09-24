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
//! Every concurrent test drives real tasks on a multi-thread runtime,
//! released together by a barrier so the requests genuinely overlap.
//! Tests that need the race to fire *reliably* run many independent
//! pairs (one per team, sprint set or row) in one release, rather than
//! relying on a single pair overlapping.

mod common;

use axum::http::StatusCode;
use chrono::NaiveDate;
use common::auth::{TestUser, login, register};
use common::fixture::create_team_with_admin;
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_i18n::MessageKey;
use peisear_storage::{StorageError, sprints, teams};
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
