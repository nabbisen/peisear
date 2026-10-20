use super::*;
use crate::StorageError;

/// Unwraps the `MessageKey` out of a `StorageError::Validation`,
/// panicking with the actual value otherwise -- so a validation that
/// translates to the wrong `StorageError` variant fails loudly
/// rather than comparing `None == None`.
fn validation_key(e: StorageError) -> peisear_i18n::MessageKey {
    let StorageError::Validation(key) = e else {
        panic!("expected StorageError::Validation, got {e:?}");
    };
    key
}

/// A fresh, fully-migrated PostgreSQL database. Reads `DATABASE_URL`,
/// the same convention every other real-server test in this crate
/// uses (`PG-001`'s own `pool::tests` precedent) -- CI's service
/// container, not a local one.
async fn migrated_pool() -> Pool {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must point at a real PostgreSQL server for this test");
    let pool = crate::pool::connect(&url).await.expect("connect");
    crate::pool::migrate(&pool).await.expect("migrate");
    pool
}

/// `PG-003` §1: eleven triggers, measured against a live, migrated
/// database rather than assumed from the migration files -- the
/// same discipline `DIA-001` round 3 applied to SQLite, now against
/// `pg_trigger` (`NOT tgisinternal`, so this project's own foreign
/// keys' internal RI triggers are excluded, not counted as this
/// schema's own).
#[tokio::test]
async fn eleven_triggers_exist_and_none_of_them_are_foreign_key_internals() {
    let pool = migrated_pool().await;
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_trigger WHERE NOT tgisinternal")
        .fetch_one(&pool)
        .await
        .expect("read pg_trigger");
    assert_eq!(
        count, 11,
        "expected the same eleven triggers SQLite carries"
    );
}

/// `PG-003` §2's own both-directions guard, run against the real
/// database: the live `ERRCODE`s found and the real registry agree,
/// both ways.
#[tokio::test]
async fn the_live_errcodes_and_the_real_registry_agree_both_ways() {
    let pool = migrated_pool().await;
    let codes = live_trigger_errcodes_postgres(&pool).await;
    assert!(
        !codes.is_empty(),
        "found no ERRCODEs at all -- the marker or pg_proc's own shape has drifted"
    );
    let a = every_errcode_is_known_postgres(&codes, TRIGGER_ERROR_CODES_POSTGRES);
    assert!(a.is_empty(), "direction 1: {a:?}");
    let b = every_known_errcode_appears_in_a_live_trigger_postgres(
        &codes,
        TRIGGER_ERROR_CODES_POSTGRES,
    );
    assert!(b.is_empty(), "direction 2: {b:?}");
}

/// Plant 1, the PostgreSQL side of `DIA-001`'s own example: an
/// `ERRCODE` nothing in the registry knows. Planted on the live
/// codes, not a committed file edit.
#[tokio::test]
async fn direction_1_catches_an_errcode_with_no_known_registry_entry() {
    let pool = migrated_pool().await;
    let mut codes = live_trigger_errcodes_postgres(&pool).await;
    codes.push("ZZ999".to_string());
    let v = every_errcode_is_known_postgres(&codes, TRIGGER_ERROR_CODES_POSTGRES);
    assert!(v.iter().any(|m| m.contains("ZZ999")), "got {v:?}");
}

/// Plant 2: remove a code the translation knows, planted on the
/// `known` side rather than edited into the migration -- the removed
/// code is still raised live, so direction 1 now calls it unknown.
#[tokio::test]
async fn direction_1_catches_a_known_code_removed_from_the_registry() {
    let pool = migrated_pool().await;
    let codes = live_trigger_errcodes_postgres(&pool).await;
    let shrunk: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES_POSTGRES
        .iter()
        .filter(|(c, _)| *c != "PE003")
        .cloned()
        .collect();
    let v = every_errcode_is_known_postgres(&codes, &shrunk);
    assert!(v.iter().any(|m| m.contains("PE003")), "got {v:?}");
}

/// Direction 2: a code the registry lists that no live trigger raises
/// at all -- dead rather than mismatched.
#[tokio::test]
async fn direction_2_catches_a_known_code_no_live_trigger_raises() {
    let pool = migrated_pool().await;
    let codes = live_trigger_errcodes_postgres(&pool).await;
    let mut widened: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES_POSTGRES.to_vec();
    widened.push((
        "ZZ998",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ));
    let v = every_known_errcode_appears_in_a_live_trigger_postgres(&codes, &widened);
    assert!(v.iter().any(|m| m.contains("ZZ998")), "got {v:?}");
}

async fn seed_one_issue(pool: &Pool, issue_id: &str) {
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, display_name) \
         VALUES ('u', 'u@example.test', 'x', 'U') ON CONFLICT (id) DO NOTHING",
    )
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p', 'u', 'P') ON CONFLICT (id) DO NOTHING")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(&format!(
        "INSERT INTO issues (id, project_id, author_id, title, status) \
         VALUES ('{issue_id}', 'p', 'u', 'T', 'open')"
    ))
    .execute(pool)
    .await
    .unwrap();
}

/// `PG-003` §3: "the same operation, rejected on both backends,
/// producing the same `MessageKey`." This is the PostgreSQL half of
/// `dialect::tests::translate_trigger_error_maps_a_real_self_parent_violation`
/// -- same scenario, same assertion, the other pool. A rejection for
/// a different reason would not be agreement; asserting the
/// `MessageKey`, not merely that an error occurred, is what rules
/// that out.
#[tokio::test]
async fn translate_trigger_error_maps_a_real_self_parent_violation() {
    let pool = migrated_pool().await;
    seed_one_issue(&pool, "i-self-parent").await;

    let err = sqlx::query(
        "UPDATE issues SET parent_issue_id = 'i-self-parent' WHERE id = 'i-self-parent'",
    )
    .execute(&pool)
    .await
    .expect_err("self-parent must be rejected");
    let translated = translate_trigger_error(err);
    let StorageError::Validation(key) = translated else {
        panic!("expected StorageError::Validation, got {translated:?}");
    };
    assert_eq!(key, peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage);
}

/// The remaining four validations, each fired for real and asserted
/// on `MessageKey` -- `§3`'s "same assertion, two pools" for every
/// one of the five, not only the self-parent case the SQLite suite
/// already covers in detail.
#[tokio::test]
async fn translate_trigger_error_maps_all_four_remaining_validations() {
    let pool = migrated_pool().await;

    // E_SUB_ISSUE_DEPTH: a sub-issue of a sub-issue.
    seed_one_issue(&pool, "i-parent-1").await;
    sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status, parent_issue_id) \
         VALUES ('i-child-1', 'p', 'u', 'T', 'open', 'i-parent-1')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let err = sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status, parent_issue_id) \
         VALUES ('i-grandchild-1', 'p', 'u', 'T', 'open', 'i-child-1')",
    )
    .execute(&pool)
    .await
    .expect_err("a sub-issue of a sub-issue must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::SubIssueCannotHaveSubIssueMessage
    );

    // E_SUB_ISSUE_PROJECT_MISMATCH: parent in a different project.
    sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p2', 'u', 'P2')")
        .execute(&pool)
        .await
        .unwrap();
    let err = sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status, parent_issue_id) \
         VALUES ('i-cross-project', 'p2', 'u', 'T', 'open', 'i-parent-1')",
    )
    .execute(&pool)
    .await
    .expect_err("a cross-project parent must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::SubIssueMustShareProjectMessage
    );

    // E_ISSUE_DEMOTE_HAS_SUBISSUES: demoting a parent that has children.
    // The new parent must itself be top-level (no parent of its own),
    // or the depth check (PE001) fires first -- same precedence the
    // SQLite original has, checked in the same order here.
    sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status) \
         VALUES ('i-other-top', 'p', 'u', 'T', 'open')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let err =
        sqlx::query("UPDATE issues SET parent_issue_id = 'i-other-top' WHERE id = 'i-parent-1'")
            .execute(&pool)
            .await
            .expect_err("demoting an issue with children must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::CannotDemoteIssueWithSubIssuesMessage
    );

    // E_ISSUE_PLANNED_DATE_ORDER: end before start.
    seed_one_issue(&pool, "i-dates").await;
    let err = sqlx::query(
        "UPDATE issues SET planned_start_at = '2026-01-02', planned_end_at = '2026-01-01' \
         WHERE id = 'i-dates'",
    )
    .execute(&pool)
    .await
    .expect_err("end before start must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage
    );
}

/// `0021`'s own doc comment, demonstrated rather than assumed:
/// `clock_timestamp()` advances between two statements in the same
/// transaction, where `CURRENT_TIMESTAMP`/`NOW()` would not --
/// exactly the shape of the whole-second defect `0021` fixes on
/// SQLite, which this project chose not to reintroduce at the
/// transaction level on PostgreSQL.
#[tokio::test]
async fn two_writes_in_one_postgres_transaction_still_advance_the_lock_token() {
    let pool = migrated_pool().await;
    seed_one_issue(&pool, "i-token").await;

    let mut tx = pool.begin().await.expect("begin");
    sqlx::query("UPDATE issues SET title = 'T1' WHERE id = 'i-token'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let first: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM issues WHERE id = 'i-token'")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    sqlx::query("UPDATE issues SET title = 'T2' WHERE id = 'i-token'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let second: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM issues WHERE id = 'i-token'")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    tx.commit().await.expect("commit");

    assert_ne!(
        first, second,
        "two writes to the same row in one transaction must not share a lock token -- \
         CURRENT_TIMESTAMP would have frozen both at the transaction's own start time"
    );
}

/// `PG-003` round 2, `§8`/`§9`: `TRIGGER_ERROR_CODES_POSTGRES`'s own
/// `MessageKey`s, in order, equal the one shared list both backends'
/// tables are checked against. No database needed -- plain data,
/// same as the SQLite side's own equivalent.
#[test]
fn trigger_error_codes_postgres_match_the_shared_validation_list() {
    assert!(message_keys_match_the_shared_list(
        TRIGGER_ERROR_CODES_POSTGRES
    ));
}

/// `§10`'s own instruction, the PostgreSQL side: the new assertion
/// seen to fail, not just written. A sixth entry added to this table
/// alone, with no matching entry in the shared list, must be caught.
#[test]
fn a_sixth_entry_added_to_only_this_table_is_caught() {
    let mut widened = TRIGGER_ERROR_CODES_POSTGRES.to_vec();
    widened.push((
        "ZZ997",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ));
    assert!(!message_keys_match_the_shared_list(&widened));
}

/// `PG-004` §2: [`sql`]'s own parser, each edge named in the handoff
/// and in this round's evidence, exercised without a server -- plain
/// data in, plain data out. The plain, reachable cases first.

#[test]
fn a_numbered_placeholder_copies_its_digits_verbatim() {
    assert_eq!(sql("WHERE id = ?1 AND x = ?2"), "WHERE id = $1 AND x = $2");
}

#[test]
fn a_reused_number_stays_the_same_number_on_both_occurrences() {
    // `optimistic_lock_guards.rs`'s own shape: the same bind value
    // compared twice, same `?N` both times -- PostgreSQL allows a
    // `$N` to repeat the same way, so this is a verbatim digit copy,
    // not a renumbering.
    assert_eq!(
        sql("AND (?6 IS NULL OR julianday(updated_at) = julianday(?6))"),
        "AND ($6 IS NULL OR julianday(updated_at) = julianday($6))"
    );
}

#[test]
fn bare_placeholders_are_numbered_left_to_right_in_binding_order() {
    assert_eq!(sql("VALUES (?, ?, ?)"), "VALUES ($1, $2, $3)");
}

#[test]
fn a_query_with_no_placeholder_at_all_is_returned_unchanged() {
    assert_eq!(sql("SELECT 1"), "SELECT 1");
}

#[test]
fn an_unchanged_query_borrows_rather_than_allocates() {
    assert!(matches!(sql("SELECT 1"), std::borrow::Cow::Borrowed(_)));
}

#[test]
fn a_rewritten_query_is_owned() {
    assert!(matches!(sql("?1"), std::borrow::Cow::Owned(_)));
}

/// A `--` line comment containing a `?` is real in this crate's own
/// query text (`sprints.rs`'s `ORDER BY` comments) and must not
/// become a bind placeholder.
#[test]
fn a_question_mark_inside_a_line_comment_is_left_alone() {
    assert_eq!(
        sql("-- what status did this start in?\nWHERE id = ?1"),
        "-- what status did this start in?\nWHERE id = $1"
    );
}

/// A string literal containing a `?` does not occur in this crate's
/// query text today (`grep -nE "'[^']*\?[^']*'" src/*.rs`, zero
/// matches, checked 2026-10-20) -- the parser is not allowed to
/// assume that stays true.
#[test]
fn a_question_mark_inside_a_string_literal_is_left_alone() {
    assert_eq!(
        sql("WHERE note = 'what status?' AND id = ?1"),
        "WHERE note = 'what status?' AND id = $1"
    );
}

/// An escaped `''` inside a string literal must not be read as the
/// literal's own closing quote -- `'admin'`-style literals are real
/// here (`teams.rs`), an escaped quote inside one is not, but the
/// scanner's quote-tracking must survive it regardless.
#[test]
fn an_escaped_quote_inside_a_string_literal_does_not_end_it_early() {
    assert_eq!(
        sql("WHERE note = 'it''s a trap? ?1' AND id = ?2"),
        "WHERE note = 'it''s a trap? ?1' AND id = $2"
    );
}

/// A `/* */` block comment does not occur in this crate's query text
/// today (`grep -c '/\*' src/*.rs` outside this very doc comment,
/// zero matches, checked 2026-10-20) -- named and tested anyway.
#[test]
fn a_question_mark_inside_a_block_comment_is_left_alone() {
    assert_eq!(
        sql("/* what status? */ WHERE id = ?1"),
        "/* what status? */ WHERE id = $1"
    );
}

/// `??` does not occur in this crate's query text today (`grep '??'
/// src/*.rs`, zero matches, checked 2026-10-20) -- two back-to-back
/// bare placeholders, not one unrecognised token, since SQL has no
/// `??` operator for this parser to mistake it for.
#[test]
fn back_to_back_bare_placeholders_are_each_numbered_independently() {
    assert_eq!(sql("(??)"), "($1$2)");
}

/// A mix of `?N` and bare `?` in one query does not occur in this
/// crate's query text today (measured 2026-10-18, `PG-004`
/// handoff §1) -- named rather than silently relied on: a bare `?`'s
/// assigned number is independent of any `?N` nearby, which is only
/// correct because the two forms never share a query here.
#[test]
fn a_bare_placeholder_numbers_independently_of_a_nearby_numbered_one_this_crate_never_mixes() {
    assert_eq!(sql("?1, ?"), "$1, $1");
}

/// `PG-004` §3: PostgreSQL's own [`stale_guard`] is plain equality --
/// no function wrapping needed, since `updated_at` is never `TEXT`
/// there.
#[test]
fn stale_guard_is_plain_equality_on_postgres() {
    assert_eq!(stale_guard(6), "updated_at = $6");
}

/// `PG-004` §3: `EXTRACT(EPOCH FROM (now() - X)) / 86400.0` against a
/// timestamp a known distance in the past -- the equivalence
/// `issue_events`, `user_burnout`, `project_health` and
/// `personal_metrics` all lean on for SQLite's
/// `julianday('now') - julianday(X)`. A tenth of a day's tolerance
/// (~144 seconds) absorbs the real wall-clock time this test itself
/// takes to run, not a claim the two formulas round differently.
#[tokio::test]
async fn age_in_days_matches_julianday_within_a_tenth_of_a_day() {
    let pool = migrated_pool().await;
    let five_days_ago = chrono::Utc::now() - chrono::Duration::days(5);
    let days: f64 =
        sqlx::query_scalar("SELECT EXTRACT(EPOCH FROM (now() - $1::timestamptz)) / 86400.0")
            .bind(five_days_ago)
            .fetch_one(&pool)
            .await
            .expect("query");
    assert!(
        (days - 5.0).abs() < 0.1,
        "expected ~5.0 days elapsed, got {days}"
    );
}

/// `PG-004` §3: `now() + ?N::interval`, bound the *same* modifier
/// string SQLite's `datetime('now', ?N)` already takes (`"-N days"`),
/// against known-age timestamps on both sides of a 7-day window.
#[tokio::test]
async fn a_relative_time_filter_matches_sqlites_own_window() {
    let pool = migrated_pool().await;

    let within: bool = sqlx::query_scalar("SELECT ($1::timestamptz >= now() + $2::interval)")
        .bind(chrono::Utc::now() - chrono::Duration::days(3))
        .bind("-7 days")
        .fetch_one(&pool)
        .await
        .expect("query");
    assert!(
        within,
        "a timestamp 3 days old must be inside a 7-day window"
    );

    let outside: bool = sqlx::query_scalar("SELECT ($1::timestamptz >= now() + $2::interval)")
        .bind(chrono::Utc::now() - chrono::Duration::days(10))
        .bind("-7 days")
        .fetch_one(&pool)
        .await
        .expect("query");
    assert!(
        !outside,
        "a timestamp 10 days old must be outside a 7-day window"
    );
}

/// `PG-004` §3: `x::date::text`, against SQLite's `DATE(x)` -- both
/// must render `YYYY-MM-DD` for `user_burnout::cognitive_switching_for_user`'s
/// own `GROUP BY day` to mean the same thing on both backends.
#[tokio::test]
async fn date_cast_to_text_matches_sqlites_date_function() {
    use chrono::TimeZone;

    let pool = migrated_pool().await;
    let ts = chrono::Utc.with_ymd_and_hms(2026, 3, 7, 23, 59, 0).unwrap();
    let day: String = sqlx::query_scalar("SELECT ($1::timestamptz::date::text)")
        .bind(ts)
        .fetch_one(&pool)
        .await
        .expect("query");
    assert_eq!(day, "2026-03-07");
}

/// `PG-004` §3: `sprints::capture_record`'s own `ON CONFLICT ... DO
/// UPDATE` names `captured_at = CURRENT_TIMESTAMP` explicitly in its
/// `SET` list -- without it, a second capture would leave the first
/// capture's `captured_at` untouched, since `DO UPDATE` only touches
/// columns its own `SET` list names (unlike SQLite's `INSERT OR
/// REPLACE`, which reset every unlisted column to its `DEFAULT` by
/// deleting and re-inserting). Two real captures, in two real
/// transactions -- `now()` is frozen per-transaction, so this would
/// not catch a same-transaction staleness bug, only the one this SQL
/// actually has: a `SET` list that forgot the column.
#[tokio::test]
async fn recapturing_a_sprint_record_still_bumps_captured_at() {
    let pool = migrated_pool().await;
    let user_id = uuid::Uuid::new_v4().to_string();
    crate::users::insert(
        &pool,
        &user_id,
        &format!("{user_id}@example.test"),
        "not-a-real-hash",
        "PG-004 Recapture Tester",
    )
    .await
    .expect("insert user");
    let team_id = crate::teams::insert(
        &pool,
        "PG-004 recapture team",
        &format!("pg-004-recapture-{user_id}"),
        None,
        &user_id,
    )
    .await
    .expect("insert team");
    let sprint_id = crate::sprints::insert(
        &pool,
        &team_id,
        "PG-004 recapture sprint",
        None,
        chrono::Utc::now().date_naive(),
        chrono::Utc::now().date_naive(),
    )
    .await
    .expect("insert sprint");

    let mut tx = pool.begin().await.expect("begin");
    crate::sprints::capture_record(&mut tx, &sprint_id)
        .await
        .expect("first capture");
    tx.commit().await.expect("commit");

    let first: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT captured_at FROM sprint_records WHERE sprint_id = $1")
            .bind(&sprint_id)
            .fetch_one(&pool)
            .await
            .expect("read first captured_at");

    let mut tx = pool.begin().await.expect("begin");
    crate::sprints::capture_record(&mut tx, &sprint_id)
        .await
        .expect("second capture");
    tx.commit().await.expect("commit");

    let second: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT captured_at FROM sprint_records WHERE sprint_id = $1")
            .bind(&sprint_id)
            .fetch_one(&pool)
            .await
            .expect("read second captured_at");

    assert!(
        second > first,
        "a second capture must bump captured_at forward, got first={first:?} second={second:?}"
    );
}

/// `PG-004` §3: "the column-list spelling differs between the two
/// call styles already in use here (`ON CONFLICT(col)` and
/// `ON CONFLICT (col)`)... Check, do not assume." Both already
/// appear unchanged in production SQL (`sprints.rs`'s own
/// `ON CONFLICT(issue_id)`, `view_states.rs`'s own
/// `ON CONFLICT (user_id, view_key)`); this proves the space is
/// insignificant directly, on a throwaway table, rather than through
/// one production path exercising only one spelling.
#[tokio::test]
async fn on_conflicts_column_list_parses_the_same_with_or_without_a_leading_space() {
    let pool = migrated_pool().await;
    sqlx::query("CREATE TEMP TABLE pg_004_on_conflict_spelling (id INT PRIMARY KEY, n INT)")
        .execute(&pool)
        .await
        .expect("create temp table");
    sqlx::query(
        "INSERT INTO pg_004_on_conflict_spelling (id, n) VALUES (1, 1) \
         ON CONFLICT(id) DO UPDATE SET n = excluded.n",
    )
    .execute(&pool)
    .await
    .expect("insert, no-space spelling");
    sqlx::query(
        "INSERT INTO pg_004_on_conflict_spelling (id, n) VALUES (1, 2) \
         ON CONFLICT (id) DO UPDATE SET n = excluded.n",
    )
    .execute(&pool)
    .await
    .expect("upsert, spaced spelling");
    let n: i32 = sqlx::query_scalar("SELECT n FROM pg_004_on_conflict_spelling WHERE id = 1")
        .fetch_one(&pool)
        .await
        .expect("read back");
    assert_eq!(n, 2, "both spellings must act on the same conflict target");
}
