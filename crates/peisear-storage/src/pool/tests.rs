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

/// `PG-001`'s own gate: `connect` opens a connection against a **real**
/// server, not merely that the postgres branch compiles. Reads
/// `DATABASE_URL` rather than hardcoding one -- CI's own
/// `test-postgres-connection` job (`test.yml`) sets it, pointing at
/// the job's PostgreSQL service container. Every other job runs under
/// the default `sqlite` feature, where this function does not even
/// compile (`#[cfg(all(feature = "postgres", not(feature = "sqlite")))]`
/// above), so it cannot accidentally run without a server to reach.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
#[tokio::test]
async fn connect_opens_a_real_connection_against_a_real_postgresql_server() {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must point at a real PostgreSQL server for this test");

    let pool = connect(&url).await.expect("connect");

    let one: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("SELECT 1 against the real connection");
    assert_eq!(one, 1);
}

/// `is_a_postgres_url` is plain string parsing, compiled regardless of
/// which backend feature is active -- unlike `connect`, it needs no
/// real server and no feature gate to test. `PG-001` round 3: this is
/// deliberately **not** symmetric with a `"looks like sqlite"` check
/// -- `data/app.db` and the Windows-style path are exactly the
/// shapes round 2's removed `url_scheme` misread as naming a scheme
/// and rejected; this function must say `false` for both, not `true`
/// for neither.
#[test]
fn is_a_postgres_url_names_only_postgresqls_own_two_schemes() {
    assert!(is_a_postgres_url("postgres://user@host/db"));
    assert!(is_a_postgres_url("postgresql://user@host/db"));
    assert!(!is_a_postgres_url("sqlite://data/app.db"));
    assert!(!is_a_postgres_url("sqlite::memory:"));
    assert!(!is_a_postgres_url("data/app.db"));
    assert!(!is_a_postgres_url("C:\\peisear\\app.db"));
    assert!(!is_a_postgres_url("nonsense"));
}

/// `PG-001` round 2's own finding: before the scheme check existed, a
/// `sqlite`-feature build given a `postgres://` URL read it as a
/// nonsense filename and failed with SQLite's own "unable to open
/// database file" -- reached, but not usefully. This asserts the
/// fast, named failure instead, and that it returns promptly rather
/// than attempting anything with the rest of the URL.
#[cfg(feature = "sqlite")]
#[tokio::test]
async fn connect_rejects_a_postgres_url_immediately_rather_than_misreading_it_as_a_filename() {
    let started = std::time::Instant::now();
    let err = connect("postgres://user@host/db")
        .await
        .expect_err("a postgres:// URL must not be accepted by a sqlite-feature build");
    assert!(
        started.elapsed() < std::time::Duration::from_secs(1),
        "the scheme mismatch must be caught before any network or filesystem attempt, not after \
         one -- took {:?}",
        started.elapsed()
    );
    let msg = err.to_string();
    assert!(
        msg.contains("sqlite") && msg.contains("postgres"),
        "the message must name both the expected and the given scheme, got: {msg}"
    );
}

/// `PG-001` round 2's own finding, the other direction: before the
/// scheme check existed, a `postgres`-feature build given a
/// `sqlite://` URL read it as a nonsense hostname and hung for
/// `PgPoolOptions`'s own 30 s acquire timeout before failing with
/// `PoolTimedOut` -- reached, but not usefully, and not quickly.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
#[tokio::test]
async fn connect_rejects_a_sqlite_url_immediately_rather_than_hanging_on_it() {
    let started = std::time::Instant::now();
    let err = connect("sqlite://data/app.db")
        .await
        .expect_err("a sqlite:// URL must not be accepted by a postgres-feature build");
    assert!(
        started.elapsed() < std::time::Duration::from_secs(1),
        "the scheme mismatch must be caught before any network attempt, not after a 30 s \
         acquire timeout -- took {:?}",
        started.elapsed()
    );
    let msg = err.to_string();
    assert!(
        msg.contains("sqlite") && msg.contains("postgres"),
        "the message must name both the expected and the given scheme, got: {msg}"
    );
}

/// `PG-001` round 3 (the round 2 review's own finding, `§12`): `sqlx`'s
/// own `SqliteConnectOptions::from_str` has always accepted a URL
/// with no scheme at all and treated it as a bare file path --
/// `DATABASE_URL=data/app.db` has worked since before `PG-001`
/// existed. Round 2's own check read this as naming scheme `""` (the
/// whole string, since there is no `:`) and rejected it, which round
/// 2's tests could not have caught -- they asserted rejection only.
/// This is the regression test for the accept case.
#[cfg(feature = "sqlite")]
#[tokio::test]
async fn connect_accepts_a_bare_path_with_no_scheme_at_all() {
    let path = std::env::temp_dir().join(format!("peisear-pg001r3-{}.db", uuid::Uuid::new_v4()));
    let url = path.display().to_string();
    assert!(
        !url.contains("://"),
        "this is the regression case -- no scheme prefix at all"
    );

    let pool = connect(&url)
        .await
        .expect("a schemeless bare path must be accepted, as sqlx itself always has");

    pool.close().await;
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}

/// The Windows case the review named concretely: this project has
/// shipped an `x86_64-pc-windows-msvc` archive since `0.53.0`, and a
/// drive-letter path (`C:\...`) has its own colon, which round 2's
/// check read as naming scheme `"C"` and rejected. A colon is a
/// legal filename character on this (Linux) test runner too, which
/// is what lets this assert the same thing a real Windows run would:
/// the drive letter's colon must not be read as a scheme.
#[cfg(feature = "sqlite")]
#[tokio::test]
async fn connect_accepts_a_windows_style_drive_letter_path() {
    let path =
        std::env::temp_dir().join(format!("C:\\peisear-pg001r3-{}.db", uuid::Uuid::new_v4()));
    let url = path.display().to_string();

    let pool = connect(&url)
        .await
        .expect("a drive-letter path's own colon must not be read as a scheme");

    pool.close().await;
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}
