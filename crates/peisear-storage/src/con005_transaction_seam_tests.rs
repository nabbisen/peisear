//! `CON-005`'s own proof, one test per mechanism adopted (round 1
//! review §4/round 2 handoff §4: "a test that fails with the
//! mechanism removed and passes with it", not merely that the
//! transaction opens). PostgreSQL-only -- SQLite's own mechanism,
//! `BEGIN IMMEDIATE`, is unmoved by this round and already proven by
//! every other test in this crate that exercises these same
//! functions.
//!
//! **Shape, the same for all four**: a "holder" task takes the real
//! lock through the real seam function (not a stand-in), signals it
//! has the lock, and waits to be told to release it. A "waiter" task
//! then calls the real production function under test. The test
//! asserts the waiter is **still running** after a short, generous
//! delay -- proving it is blocked on the held lock, not merely slow --
//! then releases the holder and asserts the waiter completes
//! correctly once the lock is free. Removing the mechanism under test
//! (reverting the corresponding `dialect`/`sprints` function to a
//! no-op) makes the waiter finish immediately instead of blocking, so
//! the "still running" assertion fails -- the fail-then-pass proof
//! this round's own gate asks for, run against a real server rather
//! than reasoned about.

use crate::issues::IssueFields;
use crate::{Guarded, Pool};
use peisear_auth::jwt::{RequesterId, issue, verify};
use peisear_core::{IssueStatus, Priority, teams::TeamRole};
use std::time::Duration;

/// A fresh, fully-migrated PostgreSQL database -- CI's own service
/// container, the same convention every real-server test in this
/// crate uses. Every row these tests create carries a fresh UUID, so
/// unlike `optimistic_lock_guards`'s own fixture there is nothing to
/// reset between runs.
async fn migrated_pool() -> Pool {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must point at a real PostgreSQL server for this test");
    let pool = crate::pool::connect(&url).await.expect("connect");
    crate::pool::migrate(&pool).await.expect("migrate");
    pool
}

/// A real `RequesterId` — obtainable only from [`verify`], exactly as
/// a login would produce one.
fn requester(id: &str) -> RequesterId {
    let token =
        issue(id, &format!("{id}@example.test"), "con005-test-secret").expect("issue a test token");
    verify(&token, "con005-test-secret").expect("verify the token just issued")
}

/// How long the "still blocked" assertion waits before checking the
/// waiter hasn't finished. Generous relative to a local socket round
/// trip (CI's own service container, not a network hop), so the
/// assertion is about the lock, not about scheduling noise.
const STILL_BLOCKED_AFTER: Duration = Duration::from_millis(300);

/// `CON-005` Group A, via `issues::update_guarded`: the holder takes
/// `issues`' own row lock directly (the same `FOR UPDATE` suffix
/// `update_guarded`'s lead `SELECT` carries); the waiter is the real
/// production function. **Fails without `dialect::for_update`'s
/// PostgreSQL body**: reverted to the empty string, the waiter's own
/// `SELECT` would not be blocked by this holder at all, and would
/// return almost immediately instead of waiting — the assertion at
/// [`STILL_BLOCKED_AFTER`] would see it already finished.
#[tokio::test]
async fn issues_update_guarded_waits_for_a_concurrent_holder_of_the_same_row() {
    let pool = migrated_pool().await;
    let user_id = uuid::Uuid::new_v4().to_string();
    crate::users::insert(
        &pool,
        &user_id,
        &format!("{user_id}@example.test"),
        "x",
        "U",
    )
    .await
    .expect("seed user");
    let project_id = uuid::Uuid::new_v4().to_string();
    crate::projects::insert(&pool, &project_id, &user_id, "Project", "", None)
        .await
        .expect("seed project");
    let issue_id = uuid::Uuid::new_v4().to_string();
    crate::issues::insert(
        &pool,
        &issue_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Original title",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("seed issue");

    let (locked_tx, locked_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let holder_pool = pool.clone();
    let holder_issue_id = issue_id.clone();
    let holder = tokio::spawn(async move {
        let mut tx = holder_pool.begin().await.expect("begin");
        sqlx::query("SELECT id FROM issues WHERE id = $1 FOR UPDATE")
            .bind(&holder_issue_id)
            .fetch_one(&mut *tx)
            .await
            .expect("lock the issue row");
        locked_tx.send(()).expect("signal locked");
        release_rx.await.expect("await release");
        tx.commit().await.expect("commit holder tx");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let waiter_pool = pool.clone();
    let waiter_issue_id = issue_id.clone();
    let waiter_project_id = project_id.clone();
    let waiter_actor_id = user_id.clone();
    let waiter = tokio::spawn(async move {
        crate::issues::update_guarded(
            &waiter_pool,
            &waiter_issue_id,
            &waiter_project_id,
            &waiter_actor_id,
            IssueFields {
                title: "Renamed title",
                description: "",
                status: IssueStatus::Open,
                priority: Priority::Medium,
                effort: None,
                assignee_id: None,
                planned_start_at: None,
                planned_end_at: None,
            },
            None,
        )
        .await
    });

    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    assert!(
        !waiter.is_finished(),
        "update_guarded must still be blocked on the row the holder locked"
    );

    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");
    let result = waiter.await.expect("waiter task").expect("update_guarded");
    assert!(
        matches!(result, Guarded::Written(())),
        "expected Written(()), got {result:?}"
    );
}

/// `CON-005` Group B, via `teams::update_role`: the holder takes
/// `dialect::lock_scope` itself (not a stand-in) over this team's own
/// membership rows; the waiter is the real production function,
/// demoting the team's *second* admin so the write itself is legal
/// once the lock is free. **Fails without `lock_scope`'s PostgreSQL
/// body**: the waiter would not be blocked by a holder locking rows
/// the waiter never itself asks to lock.
#[tokio::test]
async fn teams_update_role_waits_for_a_concurrent_holder_of_the_team_scope() {
    let pool = migrated_pool().await;
    let admin_one = uuid::Uuid::new_v4().to_string();
    let admin_two = uuid::Uuid::new_v4().to_string();
    for id in [&admin_one, &admin_two] {
        crate::users::insert(&pool, id, &format!("{id}@example.test"), "x", "U")
            .await
            .expect("seed user");
    }
    let team_id = crate::teams::insert(
        &pool,
        "Team",
        &uuid::Uuid::new_v4().to_string(),
        None,
        &admin_one,
    )
    .await
    .expect("seed team (creator is its first admin)");
    crate::teams::add_member(&pool, &team_id, &admin_two, TeamRole::Admin)
        .await
        .expect("seed second admin");

    let (locked_tx, locked_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let holder_pool = pool.clone();
    let holder_team_id = team_id.clone();
    let holder = tokio::spawn(async move {
        let mut tx = crate::dialect::begin_guarded(&holder_pool)
            .await
            .expect("begin_guarded");
        crate::dialect::lock_scope(
            &mut tx,
            "team_memberships",
            "team_id",
            &holder_team_id,
            "user_id",
        )
        .await
        .expect("lock the team scope");
        locked_tx.send(()).expect("signal locked");
        release_rx.await.expect("await release");
        tx.commit().await.expect("commit holder tx");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let waiter_pool = pool.clone();
    let waiter_team_id = team_id.clone();
    let waiter_admin_two = admin_two.clone();
    let waiter = tokio::spawn(async move {
        crate::teams::update_role(
            &waiter_pool,
            &waiter_team_id,
            &waiter_admin_two,
            TeamRole::Member,
        )
        .await
    });

    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    assert!(
        !waiter.is_finished(),
        "update_role must still be blocked on the team scope the holder locked"
    );

    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");
    waiter
        .await
        .expect("waiter task")
        .expect("update_role must succeed once the scope is free (two admins, demoting one)");
}

/// `CON-005` Group C, via `user_capacities::insert`: the holder takes
/// `dialect::lock_user_capacity_scope` itself, scoped to a user with
/// **zero existing capacity rows** -- the exact edge case this group
/// exists for, since a row lock would have nothing to anchor to here.
/// **Fails without the advisory lock**: with no row to lock either
/// way, a reverted mechanism would let the waiter's own insert run
/// immediately rather than wait.
#[tokio::test]
async fn user_capacities_insert_waits_for_a_concurrent_holder_of_the_user_scope() {
    let pool = migrated_pool().await;
    let user_id = uuid::Uuid::new_v4().to_string();
    crate::users::insert(
        &pool,
        &user_id,
        &format!("{user_id}@example.test"),
        "x",
        "U",
    )
    .await
    .expect("seed user");
    let user = requester(&user_id);

    let (locked_tx, locked_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let holder_pool = pool.clone();
    let holder_user_id = user_id.clone();
    let holder = tokio::spawn(async move {
        let mut tx = crate::dialect::begin_guarded(&holder_pool)
            .await
            .expect("begin_guarded");
        crate::dialect::lock_user_capacity_scope(&mut tx, &holder_user_id)
            .await
            .expect("lock the user capacity scope");
        locked_tx.send(()).expect("signal locked");
        release_rx.await.expect("await release");
        tx.commit().await.expect("commit holder tx");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let waiter_pool = pool.clone();
    let waiter_user = user.clone();
    let waiter = tokio::spawn(async move {
        crate::user_capacities::insert(&waiter_pool, &waiter_user, 10, None, None, None).await
    });

    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    assert!(
        !waiter.is_finished(),
        "insert must still be blocked on the user scope the holder locked, \
         even though this user has zero existing capacity rows"
    );

    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");
    waiter
        .await
        .expect("waiter task")
        .expect("insert must succeed once the scope is free");
}

/// `CON-005` Group D, via `issues::update_status_guarded`: the holder
/// takes `sprints::lock_issue_set_for_sprint` itself -- simulating the
/// window `complete_guarded` holds it during `capture_record`'s own
/// three aggregate reads -- over a sprint carrying one issue; the
/// waiter is a real guarded write to *that* issue. **This is the
/// guarantee round 1's review found missing**: `issues::
/// update_status_guarded` never touches `sprints` at all, so nothing
/// about *its own* locking would have stopped it from landing mid-
/// capture before this round. **Fails without
/// `lock_issue_set_for_sprint`'s PostgreSQL body**: the holder would
/// lock nothing, and the waiter's own row lock (`CON-005` Group A,
/// unaffected by this specific mechanism) would never contend with an
/// empty holder.
#[tokio::test]
async fn issues_update_status_guarded_waits_for_a_concurrent_holder_of_its_sprints_issue_set() {
    let pool = migrated_pool().await;
    let user_id = uuid::Uuid::new_v4().to_string();
    crate::users::insert(
        &pool,
        &user_id,
        &format!("{user_id}@example.test"),
        "x",
        "U",
    )
    .await
    .expect("seed user");
    let team_id = crate::teams::insert(
        &pool,
        "Team",
        &uuid::Uuid::new_v4().to_string(),
        None,
        &user_id,
    )
    .await
    .expect("seed team");
    let project_id = uuid::Uuid::new_v4().to_string();
    crate::projects::insert(&pool, &project_id, &user_id, "Project", "", Some(&team_id))
        .await
        .expect("seed project");
    let issue_id = uuid::Uuid::new_v4().to_string();
    crate::issues::insert(
        &pool,
        &issue_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "An issue",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: Some(&user_id),
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("seed issue");
    use chrono::NaiveDate;
    let sprint_id = crate::sprints::insert(
        &pool,
        &team_id,
        "Sprint",
        None,
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 14).unwrap(),
    )
    .await
    .expect("seed sprint");
    crate::sprints::start(&pool, &sprint_id)
        .await
        .expect("start the sprint");
    crate::sprints::add_issue(&pool, &sprint_id, &issue_id)
        .await
        .expect("add the issue to the sprint");

    let (locked_tx, locked_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let holder_pool = pool.clone();
    let holder_sprint_id = sprint_id.clone();
    let holder = tokio::spawn(async move {
        let mut tx = crate::dialect::begin_guarded(&holder_pool)
            .await
            .expect("begin_guarded");
        crate::sprints::lock_issue_set_for_sprint(&mut tx, &holder_sprint_id)
            .await
            .expect("lock the sprint's issue set");
        locked_tx.send(()).expect("signal locked");
        release_rx.await.expect("await release");
        tx.commit().await.expect("commit holder tx");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let waiter_pool = pool.clone();
    let waiter_issue_id = issue_id.clone();
    let waiter_project_id = project_id.clone();
    let waiter_actor_id = user_id.clone();
    let waiter = tokio::spawn(async move {
        crate::issues::update_status_guarded(
            &waiter_pool,
            &waiter_issue_id,
            &waiter_project_id,
            &waiter_actor_id,
            IssueStatus::Done,
            None,
        )
        .await
    });

    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    assert!(
        !waiter.is_finished(),
        "update_status_guarded must still be blocked on the issue set the holder locked \
         -- this is the write that used to be able to tear capture_record's aggregates"
    );

    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");
    waiter
        .await
        .expect("waiter task")
        .expect("update_status_guarded must succeed once the issue set is free");
}
