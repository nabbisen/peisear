//! `CAP-001` — the capacity overlap check and the write are one
//! atomic operation.
//!
//! `user_capacities::insert` / `update` used to run `overlaps_existing`
//! and then the write as two statements on two pooled connections with
//! no transaction between them, so concurrent requests could all pass
//! the check before any wrote. Twelve simultaneous open-ended saves
//! stored six rows. Sequentially the guard always worked, which is why
//! nothing caught it.
//!
//! Every test here drives the storage functions from real concurrent
//! tasks on a multi-thread runtime, released together by a barrier so
//! the requests genuinely overlap rather than queueing behind one
//! another.

mod common;

use chrono::NaiveDate;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;
use peisear_i18n::MessageKey;
use peisear_storage::{StorageError, user_capacities};
use std::sync::Arc;
use tokio::sync::Barrier;

/// Simultaneous requests per test. Twelve is the shape the dev team
/// measured (six rows stored); see the review request for how often
/// the unfixed code fails at this N.
const N: usize = 12;

fn day(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).expect("valid date")
}

async fn new_user(app: &TestApp) -> String {
    register_and_login(app, &TestUser::new("alice")).await
}

/// Run `N` copies of `job(i)` at the same instant and collect results.
async fn all_at_once<T, F, Fut>(job: F) -> Vec<T>
where
    T: Send + 'static,
    F: Fn(usize) -> Fut,
    Fut: std::future::Future<Output = T> + Send + 'static,
{
    let barrier = Arc::new(Barrier::new(N));
    let handles: Vec<_> = (0..N)
        .map(|i| {
            let barrier = barrier.clone();
            let fut = job(i);
            tokio::spawn(async move {
                barrier.wait().await;
                fut.await
            })
        })
        .collect();
    let mut out = Vec::with_capacity(N);
    for h in handles {
        out.push(h.await.expect("task panicked"));
    }
    out
}

fn is_conflict<T>(r: &Result<T, StorageError>) -> bool {
    matches!(r, Err(StorageError::Conflict(_)))
}

/// `N` simultaneous saves of the same open-ended period: exactly one
/// wins, the rest are refused with `Conflict` (not a busy/other error),
/// and exactly one row is stored.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_overlapping_inserts_store_exactly_one_row() {
    let app = TestApp::spawn().await;
    let user_id = new_user(&app).await;

    let results = all_at_once(|i| {
        let db = app.db.clone();
        let user_id = user_id.clone();
        async move { user_capacities::insert(&db, &user_id, i as i64 + 1, None, None, None).await }
    })
    .await;

    let ok = results.iter().filter(|r| r.is_ok()).count();
    let conflicts = results.iter().filter(|r| is_conflict(r)).count();
    let stored = user_capacities::list_for_user(&app.db, &user_id)
        .await
        .expect("list")
        .len();
    assert_eq!(
        (ok, conflicts, stored),
        (1, N - 1, 1),
        "expected one winner, {} refusals and one stored row; results: {results:?}",
        N - 1
    );
}

/// The same for `update`. `N` rows each own a different month of 2026
/// and never overlap. Every one is then moved to the *same* January
/// 2027 window at once. Each move is legal on its own (nothing is in
/// 2027 yet); together they are not, so exactly one may land.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_overlapping_updates_leave_exactly_one_row_in_the_window() {
    let app = TestApp::spawn().await;
    let user_id = new_user(&app).await;

    let mut ids = Vec::new();
    for m in 1..=N as u32 {
        let id = user_capacities::insert(
            &app.db,
            &user_id,
            m as i64,
            Some(day(2026, m, 1)),
            Some(day(2026, m, 28)),
            None,
        )
        .await
        .expect("seed a non-overlapping month");
        ids.push(id);
    }

    let results = all_at_once(|i| {
        let db = app.db.clone();
        let user_id = user_id.clone();
        let id = ids[i].clone();
        async move {
            user_capacities::update(
                &db,
                &user_id,
                &id,
                99,
                Some(day(2027, 1, 1)),
                Some(day(2027, 1, 31)),
                None,
            )
            .await
        }
    })
    .await;

    let ok = results.iter().filter(|r| r.is_ok()).count();
    let conflicts = results.iter().filter(|r| is_conflict(r)).count();
    let in_window = user_capacities::list_for_user(&app.db, &user_id)
        .await
        .expect("list")
        .into_iter()
        .filter(|r| r.period_start == Some(day(2027, 1, 1)))
        .count();
    assert_eq!(
        (ok, conflicts, in_window),
        (1, N - 1, 1),
        "expected one winner, {} refusals and one row in the window; results: {results:?}",
        N - 1
    );
}

/// The guard must not become a lock on unrelated rows: `N` simultaneous
/// saves of `N` *different*, non-overlapping months all succeed.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_non_overlapping_inserts_all_succeed() {
    let app = TestApp::spawn().await;
    let user_id = new_user(&app).await;

    let results = all_at_once(|i| {
        let db = app.db.clone();
        let user_id = user_id.clone();
        let m = i as u32 + 1;
        async move {
            user_capacities::insert(
                &db,
                &user_id,
                m as i64,
                Some(day(2026, m, 1)),
                Some(day(2026, m, 28)),
                None,
            )
            .await
        }
    })
    .await;

    assert!(
        results.iter().all(|r| r.is_ok()),
        "every non-overlapping save must succeed: {results:?}"
    );
    let stored = user_capacities::list_for_user(&app.db, &user_id)
        .await
        .expect("list")
        .len();
    assert_eq!(stored, N);
}

/// Sequential behaviour is unchanged: the second overlapping save is
/// refused with the message that names the conflicting row, its dates
/// and its points. (No earlier test covered this message — the
/// handler-level tests only reach the *success* path — so it is
/// pinned here.)
#[tokio::test]
async fn sequential_overlap_is_refused_naming_the_conflicting_row() {
    let app = TestApp::spawn().await;
    let user_id = new_user(&app).await;

    let first = user_capacities::insert(
        &app.db,
        &user_id,
        10,
        Some(day(2026, 3, 1)),
        Some(day(2026, 3, 31)),
        None,
    )
    .await
    .expect("first save");

    let refused =
        user_capacities::insert(&app.db, &user_id, 20, Some(day(2026, 3, 15)), None, None).await;

    match refused {
        Err(StorageError::Conflict(MessageKey::CapacityPeriodOverlapMessage {
            row_id,
            period_start,
            period_end,
            points,
        })) => {
            assert_eq!(row_id, first);
            assert_eq!(period_start, "2026-03-01");
            assert_eq!(period_end, "2026-03-31");
            assert_eq!(points, 10);
        }
        other => panic!("expected the overlap conflict naming the first row, got {other:?}"),
    }
    let stored = user_capacities::list_for_user(&app.db, &user_id)
        .await
        .expect("list")
        .len();
    assert_eq!(stored, 1, "a refused save must store nothing");
}
