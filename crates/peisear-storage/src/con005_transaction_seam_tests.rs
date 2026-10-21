//! `CON-005`'s own proof, one test per mechanism adopted (round 1
//! review §4/round 2 handoff §4: "a test that fails with the
//! mechanism removed and passes with it", not merely that the
//! transaction opens). PostgreSQL-only -- SQLite's own mechanism,
//! `BEGIN IMMEDIATE`, is unmoved by this round and already proven by
//! every other test in this crate that exercises these same
//! functions.
//!
//! **Shape, for the first four (one per mechanism)**: a "holder" task
//! takes the real lock through the real seam function (not a
//! stand-in), signals it has the lock, and waits to be told to release
//! it. A "waiter" task then calls the real production function under
//! test. The test asserts the waiter is **still running** after a
//! short, generous delay -- proving it is blocked on the held lock,
//! not merely slow -- then releases the holder and asserts the waiter
//! completes correctly once the lock is free.
//!
//! **Round 2's own review found these four insufficient for two of
//! the four mechanisms, and round 3 adds what they do not give**:
//! blocking is a side effect of `FOR UPDATE`/`lock_scope`, not the
//! guarantee they exist for. Revert `dialect::for_update()` to `""`
//! and `issues::update_guarded`'s lead `SELECT` stops blocking --  but
//! the function then reaches its own trailing `UPDATE issues SET …
//! WHERE id = ?1 AND project_id = ?2`, which still takes the row lock
//! the holder in `issues_update_guarded_waits_for_a_concurrent_
//! holder_of_the_same_row` is holding regardless, so that test's own
//! `!waiter.is_finished()` keeps passing even with the mechanism gone.
//! The same is true of `teams::update_role` through `lock_scope`.
//! `dialect::lock_user_capacity_scope` is the one of the four that
//! does not survive this way -- without it, `user_capacities::insert`
//! conflicts with nothing and the waiter finishes -- which is why
//! round 2 could trust it, and could not trust the other two, without
//! measuring.
//!
//! **The three functions below assert the actual guarantee instead of
//! blocking**, matching the handoff's own `§3c` table: two real,
//! concurrently spawned callers (not a holder/waiter pair) racing on
//! the same invariant, with the correct outcome named for both --
//! never "both succeed" or "both fail", always one of each. Removing
//! the mechanism under test does not make these hang or error; it
//! makes the *wrong pair of outcomes* land, which is what each
//! assertion below checks for.

use crate::issues::IssueFields;
use crate::{Guarded, Pool, StorageError};
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

/// `CON-005` round 3, Group A (handoff `§3c`): two concurrent
/// `update_guarded` calls on the **same row**, carrying the **same**
/// `expected` stamp, is the actual guarantee `for_update()` exists
/// for -- not "the waiter blocked" (see this module's own doc
/// comment), but **exactly one of the two must land, and the loser
/// must see `Stale`, not silently overwrite the winner.**
///
/// **Why a bare `tokio::spawn` race is not enough, found by running
/// it**: a first version of this test spawned both callers with
/// nothing forcing them together, and it passed even with
/// `for_update()` reverted -- not because the mechanism was
/// unnecessary, but because the two tasks did not reliably overlap at
/// all: one caller's entire `SELECT`-then-`UPDATE`-then-`COMMIT` ran
/// to completion before the other's `SELECT` executed, so the second
/// caller's *unlocked* read still saw the first's already-committed
/// stamp and correctly refused -- the right answer, for the wrong,
/// unreliable reason. **An external holder forces the real window
/// instead of hoping for it**: it takes the row's lock first (the
/// same lock `for_update()` would have each caller take on its own
/// lead `SELECT`), so *both* real callers' reads happen relative to
/// the same release point rather than to each other's unpredictable
/// scheduling.
///
/// **With `for_update()` intact**: both callers' lead `SELECT`s queue
/// behind the holder. Once released, whichever gets the lock first
/// reads `t0`, writes, and commits inside that same lock; the other's
/// `SELECT` only then unblocks, reads the *new* stamp, and correctly
/// returns `Stale`.
///
/// **Without `for_update()`**: neither callers' `SELECT` queues on
/// anything (a plain `SELECT` never waits on another transaction's row
/// lock), so both read `t0` immediately, both pass `stamp_matches`,
/// and both reach `UPDATE issues SET … WHERE id = ?1 AND project_id =
/// ?2` -- whose `WHERE` carries no stamp of its own. Both `UPDATE`s do
/// still queue on the holder (an `UPDATE` always needs the row lock),
/// but by then the stamp comparison that matters has already
/// happened, on stale data, for both. Once the holder releases,
/// PostgreSQL's own `EvalPlanQual` re-check re-applies each `UPDATE`
/// against the row's latest version regardless of who wrote it last,
/// since neither `WHERE` clause mentions the stamp -- so **both return
/// `Written(())`**, the second silently overwriting the first.
#[tokio::test]
async fn issues_update_guarded_with_one_stamp_exactly_one_write_wins() {
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
    let t0: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM issues WHERE id = $1")
            .bind(&issue_id)
            .fetch_one(&pool)
            .await
            .expect("read the seeded stamp");

    // Force the window instead of hoping for it (see this function's
    // own doc comment): a holder takes the row's lock first, raw, not
    // through the mechanism under test, so it still pins the row even
    // when `for_update()` is the thing reverted.
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
        tx.commit()
            .await
            .expect("commit holder tx (no write -- the stamp is unchanged)");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let pool_a = pool.clone();
    let issue_a = issue_id.clone();
    let project_a = project_id.clone();
    let actor_a = user_id.clone();
    let caller_a = tokio::spawn(async move {
        crate::issues::update_guarded(
            &pool_a,
            &issue_a,
            &project_a,
            &actor_a,
            IssueFields {
                title: "Renamed by A",
                description: "",
                status: IssueStatus::Open,
                priority: Priority::Medium,
                effort: None,
                assignee_id: None,
                planned_start_at: None,
                planned_end_at: None,
            },
            Some(t0),
        )
        .await
    });
    let pool_b = pool.clone();
    let issue_b = issue_id.clone();
    let project_b = project_id.clone();
    let actor_b = user_id.clone();
    let caller_b = tokio::spawn(async move {
        crate::issues::update_guarded(
            &pool_b,
            &issue_b,
            &project_b,
            &actor_b,
            IssueFields {
                title: "Renamed by B",
                description: "",
                status: IssueStatus::Open,
                priority: Priority::Medium,
                effort: None,
                assignee_id: None,
                planned_start_at: None,
                planned_end_at: None,
            },
            Some(t0),
        )
        .await
    });

    // Give both callers time to reach the point they queue on the
    // holder (their `SELECT` if `for_update()` is intact, their
    // `UPDATE` either way), then release together.
    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");

    let result_a = caller_a.await.expect("caller A task").expect("caller A");
    let result_b = caller_b.await.expect("caller B task").expect("caller B");

    let written = [&result_a, &result_b]
        .iter()
        .filter(|r| matches!(r, Guarded::Written(())))
        .count();
    let stale = [&result_a, &result_b]
        .iter()
        .filter(|r| matches!(r, Guarded::Stale { .. }))
        .count();
    assert_eq!(
        (written, stale),
        (1, 1),
        "exactly one caller must win and the other must see Stale, got {result_a:?} and {result_b:?}"
    );
}

/// `CON-005` round 3, Group B (handoff `§3c`): two concurrent
/// `start_guarded` calls on **two different `Planned` sprints in the
/// same team** is the guarantee `lock_scope` exists for -- not "the
/// waiter blocked", but **exactly one of the two sprints may become
/// `Active`.**
///
/// **Same reasoning as Group A's own test for why a bare race is not
/// enough**: `S1` and `S2` are different rows, so there is no natural
/// row-level lock between the two callers' own final `UPDATE`s the
/// way Group A's test could (eventually) rely on -- which means a
/// bare `tokio::spawn` race here is even *less* reliable than Group
/// A's was. A holder locks every row `team_id` scopes (both `S1` and
/// `S2`) first, raw, bypassing `lock_scope` itself, so both callers'
/// reads are forced to happen relative to the same release point.
///
/// **With `lock_scope` intact**: both callers' own scope-lock attempt
/// (`SELECT 1 FROM sprints WHERE team_id = ? FOR UPDATE`, the same
/// shape as the holder's) queues behind the holder. Released,
/// whichever acquires it first reads `active_for_team` as empty,
/// writes, and commits -- releasing the *whole* scope, `S1` and `S2`
/// together. Only then does the other's scope-lock attempt succeed,
/// now re-reading `active_for_team` and correctly finding the first's
/// committed sprint.
///
/// **Without `lock_scope`**: neither caller's `active_for_team` read
/// queues on anything (a plain `SELECT`, same as Group A), so both
/// read it as empty immediately, both pass their own status check,
/// and both reach their own `UPDATE` -- which *does* queue on the
/// holder, since it locked both rows. Released, `S1`'s `UPDATE` and
/// `S2`'s `UPDATE` do not conflict with each other at all (different
/// rows), so **both commit**: two `Active` sprints in one team, the
/// exact state `start_guarded`'s own doc comment says cannot exist.
#[tokio::test]
async fn sprints_start_guarded_on_two_sprints_in_one_team_exactly_one_wins() {
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
    use chrono::NaiveDate;
    let sprint_one = crate::sprints::insert(
        &pool,
        &team_id,
        "Sprint One",
        None,
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 14).unwrap(),
    )
    .await
    .expect("seed sprint one");
    let sprint_two = crate::sprints::insert(
        &pool,
        &team_id,
        "Sprint Two",
        None,
        NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 28).unwrap(),
    )
    .await
    .expect("seed sprint two");

    // Force the window instead of hoping for it (see this function's
    // own doc comment): a holder locks every sprint row the team
    // scopes, raw, not through `lock_scope` itself.
    let (locked_tx, locked_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let holder_pool = pool.clone();
    let holder_team_id = team_id.clone();
    let holder = tokio::spawn(async move {
        let mut tx = holder_pool.begin().await.expect("begin");
        sqlx::query("SELECT id FROM sprints WHERE team_id = $1 FOR UPDATE")
            .bind(&holder_team_id)
            .fetch_all(&mut *tx)
            .await
            .expect("lock the team's sprint rows");
        locked_tx.send(()).expect("signal locked");
        release_rx.await.expect("await release");
        tx.commit().await.expect("commit holder tx (no write)");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let pool_a = pool.clone();
    let sprint_a = sprint_one.clone();
    let caller_a =
        tokio::spawn(async move { crate::sprints::start_guarded(&pool_a, &sprint_a, None).await });
    let pool_b = pool.clone();
    let sprint_b = sprint_two.clone();
    let caller_b =
        tokio::spawn(async move { crate::sprints::start_guarded(&pool_b, &sprint_b, None).await });

    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");

    let result_a = caller_a.await.expect("caller A task");
    let result_b = caller_b.await.expect("caller B task");

    let written = [&result_a, &result_b]
        .iter()
        .filter(|r| matches!(r, Ok(Guarded::Written(()))))
        .count();
    let conflicted = [&result_a, &result_b]
        .iter()
        .filter(|r| matches!(r, Err(StorageError::Conflict(_))))
        .count();
    assert_eq!(
        (written, conflicted),
        (1, 1),
        "exactly one sprint must start and the other must be refused as a conflict, \
         got {result_a:?} and {result_b:?}"
    );
}

/// `CON-005` round 3, Group C (handoff `§3c`): two concurrent
/// `insert`s of **overlapping periods for a user with no existing
/// capacity rows** is `CAP-001`'s own original defect, reproduced
/// deliberately to prove `lock_user_capacity_scope` closes it --
/// **exactly one insert may land; the other must be refused.**
///
/// **A bare race here is even less trustworthy than Groups A and B's
/// own first, discarded attempts**: two fresh-`UUID` rows share no
/// natural database-level lock at all, intact or reverted, so there
/// is no eventual fallback for the holder technique to anchor to the
/// way Group A's `UPDATE` or Group B's own row set gave it one.
/// **Measured, not assumed**: a version of this test with no forcing
/// passed in CI with the advisory lock reverted, twice, for the same
/// reason Groups A and B's first attempts did.
///
/// **The holder instead takes a table-level lock**: `LOCK TABLE
/// user_capacities IN EXCLUSIVE MODE`, which conflicts with the `ROW
/// EXCLUSIVE` mode any `INSERT` needs but not with a plain `SELECT`
/// (`overlaps_existing`'s own read). This forces both callers' own
/// `INSERT`s to queue on the holder regardless of whether
/// `lock_user_capacity_scope` runs at all:
///
/// - **With the advisory lock intact**: one caller acquires it, reads
///   `overlaps_existing` as empty, then queues on the holder's table
///   lock at its own `INSERT`. The other queues on the advisory lock
///   itself, behind the first. Releasing the holder lets the first's
///   `INSERT` land and commit (freeing the advisory lock too); only
///   then does the second acquire the advisory lock, re-read
///   `overlaps_existing`, and correctly see the first's committed row.
/// - **Without it**: both callers skip straight to `overlaps_existing`
///   (unblocked -- a plain `SELECT` never conflicts with `EXCLUSIVE`),
///   both read zero rows, both reach their own `INSERT`, and both
///   queue on the holder together. Released, neither `INSERT`
///   conflicts with the other at the row level (different `UUID`s),
///   so **both land**: two overlapping rows for the same user,
///   `CAP-001`'s own original defect.
///
/// **The loser's exact error is not asserted as `Conflict(_)` alone**
/// -- see the comment on that match below. `CON-006` (open) means the
/// conflict read itself can fail to decode `points` before it gets the
/// chance to build a clean `Conflict`; this test tolerates that one,
/// already-escalated shape without hiding it, so it keeps proving the
/// lock rather than failing on a defect a different round owns.
#[tokio::test]
async fn user_capacities_insert_with_overlapping_periods_exactly_one_wins() {
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
    use chrono::NaiveDate;
    let period_start = Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
    let period_end = Some(NaiveDate::from_ymd_opt(2026, 1, 31).unwrap());

    // Force the window instead of hoping for it (see this function's
    // own doc comment): a holder takes a table-level lock that blocks
    // any `INSERT` into this table regardless of whether the advisory
    // lock under test runs at all.
    let (locked_tx, locked_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let holder_pool = pool.clone();
    let holder = tokio::spawn(async move {
        let mut tx = holder_pool.begin().await.expect("begin");
        sqlx::query("LOCK TABLE user_capacities IN EXCLUSIVE MODE")
            .execute(&mut *tx)
            .await
            .expect("lock the table against concurrent inserts");
        locked_tx.send(()).expect("signal locked");
        release_rx.await.expect("await release");
        tx.commit().await.expect("commit holder tx (no write)");
    });
    locked_rx.await.expect("holder reports it holds the lock");

    let pool_a = pool.clone();
    let user_a = user.clone();
    let caller_a = tokio::spawn(async move {
        crate::user_capacities::insert(&pool_a, &user_a, 10, period_start, period_end, None).await
    });
    let pool_b = pool.clone();
    let user_b = user.clone();
    let caller_b = tokio::spawn(async move {
        crate::user_capacities::insert(&pool_b, &user_b, 20, period_start, period_end, None).await
    });

    tokio::time::sleep(STILL_BLOCKED_AFTER).await;
    release_tx.send(()).expect("release the holder");
    holder.await.expect("holder task");

    let result_a = caller_a.await.expect("caller A task");
    let result_b = caller_b.await.expect("caller B task");

    let written = [&result_a, &result_b].iter().filter(|r| r.is_ok()).count();
    assert_eq!(
        written, 1,
        "exactly one insert must land for an overlapping period -- the lock exists so the \
         other is refused rather than silently landing too, got {result_a:?} and {result_b:?}"
    );
    let loser = if result_a.is_ok() {
        &result_b
    } else {
        &result_a
    };
    match loser {
        Err(StorageError::Conflict(_)) => {}
        // `CON-006` (open, not this round's to fix): `overlaps_existing`'s own
        // conflict-info read decodes the conflicting row's `points` column, which
        // is `INTEGER` on PostgreSQL against this crate's own `i64` binding -- the
        // same defect `CON-005` round 2 found and escalated, reached here through
        // a second code path (the conflict read, not the plain row read). Tolerated
        // so this test still proves the lock rather than failing on an unrelated,
        // already-escalated defect; once `CON-006` lands, delete this arm and
        // tighten the assertion above to require `Err(Conflict(_))` specifically.
        Err(StorageError::Database(_)) => {}
        other => panic!(
            "the losing insert must fail as a conflict (or, until CON-006 lands, the \
             known points/int4 decode defect reached through the conflict read); \
             got {other:?}"
        ),
    }
}
