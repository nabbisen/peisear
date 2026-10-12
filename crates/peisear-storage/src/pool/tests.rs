use super::*;

/// `PERF-002`: a used install gains statistics when its connections close,
/// and not before -- so nothing runs at startup or on a request.
#[tokio::test]
async fn statistics_appear_when_a_used_pool_closes_and_not_before() {
    let path = std::env::temp_dir().join(format!("peisear-perf002-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());

    async fn has_stats(pool: &Pool) -> bool {
        let n: i64 =
            sqlx::query_scalar("SELECT count(*) FROM sqlite_master WHERE name = 'sqlite_stat1'")
                .fetch_one(pool)
                .await
                .unwrap();
        n == 1
    }

    let pool = connect(&url).await.expect("connect");
    migrate(&pool).await.expect("migrate");
    assert!(!has_stats(&pool).await, "nothing analyses at startup");

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, display_name) \
         VALUES ('u', 'u@example.test', 'x', 'U')",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p', 'u', 'P')")
        .execute(&pool)
        .await
        .unwrap();
    // Enough rows that there is something for `optimize` to decide about.
    sqlx::query(
        "WITH RECURSIVE n(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM n WHERE x < 600) \
         INSERT INTO issues (id, project_id, author_id, title, status) \
         SELECT 'i' || x, 'p', 'u', 'T', CASE x % 3 WHEN 0 THEN 'done' ELSE 'open' END FROM n",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Use it: queries that could have used statistics, on one connection.
    {
        let mut conn = pool.acquire().await.unwrap();
        for _ in 0..3 {
            sqlx::query("SELECT id FROM issues WHERE project_id = 'p' AND status = 'open'")
                .fetch_all(&mut *conn)
                .await
                .unwrap();
            sqlx::query("SELECT id FROM issues WHERE assignee_id = 'u'")
                .fetch_all(&mut *conn)
                .await
                .unwrap();
        }
    }
    assert!(
        !has_stats(&pool).await,
        "use alone must not analyse -- nothing on the request path"
    );

    pool.close().await;
    let reopened = connect(&url).await.expect("reopen");
    assert!(
        has_stats(&reopened).await,
        "closing a used pool must have run PRAGMA optimize"
    );
    let analysed: i64 =
        sqlx::query_scalar("SELECT count(*) FROM sqlite_stat1 WHERE tbl = 'issues'")
            .fetch_one(&reopened)
            .await
            .unwrap();
    assert!(analysed > 0, "the issues table must have been analysed");
    reopened.close().await;
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}

/// `STORE-001` §3: the migration alone proves nothing if the token's
/// millisecond component is lost anywhere on the way out of SQLite,
/// through the render the application sends to the browser, or back
/// through the parse of the client's own resubmission -- `0021`'s
/// fix would "land" and change nothing observable. Asserts the exact
/// round trip `check_optimistic_lock`
/// (`crates/peisear-web/src/error.rs`) performs, against both
/// timestamp shapes this schema can hold side by side: the
/// whole-second shape every row still carries from its own `INSERT`
/// (the column `DEFAULT`, deliberately untouched by `0021`) and the
/// millisecond shape `0021`'s trigger produces on the row's first
/// `UPDATE`. A decoder that only handles one would break every row
/// on one side of that boundary.
#[tokio::test]
async fn the_lock_token_round_trip_preserves_milliseconds_across_both_timestamp_shapes() {
    use chrono::{DateTime, Utc};

    let path = std::env::temp_dir().join(format!("peisear-store001-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());
    let pool = connect(&url).await.expect("connect");
    migrate(&pool).await.expect("migrate");

    // Round trip exactly the way `check_optimistic_lock` and
    // `stamp_matches` do: render as the form does, parse as a
    // client's resubmission is parsed, compare by equality.
    fn round_trip(dt: DateTime<Utc>) -> DateTime<Utc> {
        let rendered = dt.to_rfc3339();
        chrono::DateTime::parse_from_rfc3339(&rendered)
            .unwrap_or_else(|e| panic!("a rendered lock token must parse back: {rendered:?}: {e}"))
            .with_timezone(&Utc)
    }

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, display_name) \
         VALUES ('u', 'u@example.test', 'x', 'U')",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p', 'u', 'P')")
        .execute(&pool)
        .await
        .unwrap();

    // Shape 1: the row as `INSERT` leaves it -- the column `DEFAULT`
    // is `CURRENT_TIMESTAMP`, whole-second, and `0021` does not touch
    // it (the lock is spent on UPDATE, not INSERT).
    sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status) \
         VALUES ('i', 'p', 'u', 'T', 'open')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let (inserted,): (DateTime<Utc>,) =
        sqlx::query_as("SELECT updated_at FROM issues WHERE id = 'i'")
            .fetch_one(&pool)
            .await
            .expect("decode the whole-second (pre-first-update) shape");
    assert_eq!(
        inserted.timestamp_subsec_millis(),
        0,
        "the INSERT default is whole-second by design -- 0021 deliberately \
         does not touch it"
    );
    assert_eq!(
        round_trip(inserted),
        inserted,
        "the whole-second shape must round-trip too, or every row ever \
         inserted and never updated breaks"
    );

    // Shape 2: `0021`'s trigger fires on this UPDATE and bumps
    // `updated_at` to millisecond precision -- the shape a lock token
    // actually carries from here on.
    sqlx::query("UPDATE issues SET title = 'T2' WHERE id = 'i'")
        .execute(&pool)
        .await
        .unwrap();
    let (updated,): (DateTime<Utc>,) =
        sqlx::query_as("SELECT updated_at FROM issues WHERE id = 'i'")
            .fetch_one(&pool)
            .await
            .expect("decode the millisecond (post-trigger) shape");
    // `SEC-001` fixed this in passing: `timestamp_subsec_millis() !=
    // 0` is flaky, not just unlikely -- once in every roughly 1,000
    // runs the trigger fires in the first millisecond of a second and
    // the true fraction genuinely *is* `.000`, which is a correct
    // result this assertion would have failed. The fraction's own
    // *value* can coincide with zero; whether the trigger used the
    // millisecond `strftime` format at all cannot, so check the raw
    // stored text for the decimal point that format always produces,
    // never the decoded value's own digits.
    let (updated_raw,): (String,) = sqlx::query_as("SELECT updated_at FROM issues WHERE id = 'i'")
        .fetch_one(&pool)
        .await
        .expect("read the post-trigger value as raw text");
    assert!(
        updated_raw.contains('.'),
        "the post-trigger value must be stored with a fractional-seconds \
         component (0021's strftime format always includes one), got {updated_raw:?} \
         -- this is a structural check on the stored text, not the decoded \
         value's own digits, which can coincidentally be zero"
    );
    assert_eq!(
        round_trip(updated),
        updated,
        "round trip must preserve the post-trigger value exactly, \
         milliseconds included -- this is the comparison a real second \
         request within the same wall-clock second depends on"
    );

    // `REG-001` §2: the two checks above do not guard the decode. A
    // decoder that truncates the fraction passes both -- `contains('.')`
    // only reads the trigger's own text, and `round_trip(updated) ==
    // updated` compares two equally-truncated values. Plant a known
    // fraction instead of relying on the clock: this `UPDATE` sets
    // `updated_at` to a literal that differs from its current value, so
    // the trigger's `WHEN OLD.updated_at = NEW.updated_at` guard is
    // false and it does not fire -- confirmed below by reading the raw
    // text back, rather than assumed.
    sqlx::query("UPDATE issues SET updated_at = '2026-01-01 00:00:23.414' WHERE id = 'i'")
        .execute(&pool)
        .await
        .unwrap();
    let (planted_raw,): (String,) = sqlx::query_as("SELECT updated_at FROM issues WHERE id = 'i'")
        .fetch_one(&pool)
        .await
        .expect("read the planted value as raw text");
    assert_eq!(
        planted_raw, "2026-01-01 00:00:23.414",
        "the trigger must not have fired on an UPDATE that itself changed \
         updated_at -- if it did, the WHEN guard no longer means what \
         0021's comment says it means"
    );
    let (planted,): (DateTime<Utc>,) =
        sqlx::query_as("SELECT updated_at FROM issues WHERE id = 'i'")
            .fetch_one(&pool)
            .await
            .expect("decode the planted value");
    assert_eq!(
        planted.timestamp_subsec_millis(),
        414,
        "the decoder must preserve the fraction exactly -- this is the \
         assertion a truncating decoder fails and the text-only check \
         above cannot catch"
    );

    pool.close().await;
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}
