//! `CAL-001` (RFC 002): the data layer for the calendar surfaces —
//! migration `0016`, the trigger pair, and the two window queries.
//! Nothing here renders a calendar (that's CAL-002); every test
//! calls `peisear_storage::issues` directly against a real migrated
//! pool, the same way `assignee_candidates.rs`/`sprint_plan.rs`
//! exercise storage-level behaviour without needing the eventual UI.

mod common;

use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::{StorageError, issues};

fn dt(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s).unwrap().to_utc()
}

async fn insert_issue_with_planned(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    title: &str,
    planned_start_at: Option<chrono::DateTime<chrono::Utc>>,
    planned_end_at: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<String, StorageError> {
    let id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &app.db,
        &id,
        project_id,
        author_id,
        issues::IssueFields {
            title,
            description: "Test issue body.",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at,
            planned_end_at,
        },
    )
    .await
    .map(|()| id)
}

/// Test 1 -- the migration applies cleanly (every `TestApp::spawn`
/// already proves this, since it runs the full migration set to
/// completion or the pool never comes up) and existing rows get
/// `NULL` for both new columns. `pragma_table_info` confirms the
/// schema shape directly, per RFC 002's own test plan item 1.
#[tokio::test]
async fn migration_0016_adds_planned_columns_and_existing_rows_are_null() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let cols: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM pragma_table_info('issues') WHERE name IN (?1, ?2)")
            .bind("planned_start_at")
            .bind("planned_end_at")
            .fetch_all(&app.db)
            .await
            .expect("read pragma_table_info");
    assert_eq!(
        cols.len(),
        2,
        "expected both planned_start_at and planned_end_at columns to exist: {cols:?}"
    );

    // A row inserted with no awareness of the new columns (the
    // fixture doesn't set them) is what "existing data" looks like
    // post-migration.
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Pre-existing").await;
    let issue = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("find issue");
    assert_eq!(issue.planned_start_at, None);
    assert_eq!(issue.planned_end_at, None);
}

/// Test 2 -- the trigger rejects `planned_end_at < planned_start_at`
/// on insert.
#[tokio::test]
async fn trigger_rejects_end_before_start_on_insert() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let err = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "Backwards range",
        Some(dt("2026-06-10T00:00:00Z")),
        Some(dt("2026-06-01T00:00:00Z")),
    )
    .await
    .expect_err("insert with end before start must fail");
    assert!(
        matches!(
            err,
            StorageError::Validation(peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage)
        ),
        "expected IssuePlannedEndBeforeStartMessage, got {err:?}"
    );
}

/// Test 3 -- the same check on update.
#[tokio::test]
async fn trigger_rejects_end_before_start_on_update() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let err = issues::update(
        &app.db,
        &issue_id,
        &project_id,
        &user_id,
        issues::IssueFields {
            title: "T",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: Some(dt("2026-06-10T00:00:00Z")),
            planned_end_at: Some(dt("2026-06-01T00:00:00Z")),
        },
    )
    .await
    .expect_err("update with end before start must fail");
    assert!(
        matches!(
            err,
            StorageError::Validation(peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage)
        ),
        "expected IssuePlannedEndBeforeStartMessage, got {err:?}"
    );
}

/// Test 4 -- either column `NULL` is accepted; the constraint only
/// fires when *both* are set.
#[tokio::test]
async fn either_planned_column_null_is_accepted() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let start_only = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "Start only",
        Some(dt("2026-06-01T00:00:00Z")),
        None,
    )
    .await
    .expect("start-only insert must succeed");
    let end_only = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "End only",
        None,
        Some(dt("2026-06-01T00:00:00Z")),
    )
    .await
    .expect("end-only insert must succeed");
    let neither = insert_issue_with_planned(&app, &project_id, &user_id, "Neither", None, None)
        .await
        .expect("neither-set insert must succeed");

    for id in [start_only, end_only, neither] {
        issues::find(&app.db, &id, &project_id)
            .await
            .expect("row must exist");
    }
}

/// Test 5 -- `translate_trigger_error` maps the migration's `RAISE`
/// text to `IssuePlannedEndBeforeStartMessage`, and the rendered
/// English text is identical to the needle `translate_trigger_error`
/// matches against (`DEC-011`).
#[tokio::test]
async fn trigger_error_maps_to_message_key_with_identical_text() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let err = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "Backwards",
        Some(dt("2026-06-10T00:00:00Z")),
        Some(dt("2026-06-01T00:00:00Z")),
    )
    .await
    .expect_err("must fail");
    let StorageError::Validation(key) = err else {
        panic!("expected Validation, got {err:?}");
    };
    let rendered = peisear_i18n::Locale::English.render(key);
    assert_eq!(
        rendered, "planned end date must be on or after planned start date",
        "the rendered MessageKey text must be the same string translate_trigger_error \
         matched as a needle against the trigger's raw RAISE text"
    );
}

/// Test 6 -- the window queries return issues overlapping
/// `[from, to]` and exclude those outside it, including the
/// half-open case (`planned_end_at IS NULL`, treated as a
/// half-hour anchor at `planned_start_at`).
#[tokio::test]
async fn planned_for_project_returns_overlapping_issues_including_half_open() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let window_from = dt("2026-06-01T00:00:00Z");
    let window_to = dt("2026-06-30T23:59:59Z");

    // Fully inside the window.
    let inside = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "Inside",
        Some(dt("2026-06-10T00:00:00Z")),
        Some(dt("2026-06-12T00:00:00Z")),
    )
    .await
    .expect("insert inside");

    // Half-open: start inside the window, no end at all -- must
    // still appear (must-have 5's half-hour-anchor treatment).
    let half_open = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "Half open",
        Some(dt("2026-06-15T00:00:00Z")),
        None,
    )
    .await
    .expect("insert half-open");

    // Entirely before the window -- must be excluded.
    let before = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "Before",
        Some(dt("2026-05-01T00:00:00Z")),
        Some(dt("2026-05-02T00:00:00Z")),
    )
    .await
    .expect("insert before");

    // Entirely after the window -- must be excluded.
    let after = insert_issue_with_planned(
        &app,
        &project_id,
        &user_id,
        "After",
        Some(dt("2026-07-01T00:00:00Z")),
        Some(dt("2026-07-02T00:00:00Z")),
    )
    .await
    .expect("insert after");

    // No planned date at all -- must be excluded.
    let unplanned = create_issue(&app.db, &project_id, &user_id, "Unplanned").await;

    let results = issues::planned_for_project(&app.db, &project_id, window_from, window_to)
        .await
        .expect("query planned_for_project");
    let ids: std::collections::HashSet<&str> = results.iter().map(|i| i.id.as_str()).collect();

    assert!(
        ids.contains(inside.as_str()),
        "inside issue missing: {ids:?}"
    );
    assert!(
        ids.contains(half_open.as_str()),
        "half-open issue missing: {ids:?}"
    );
    assert!(
        !ids.contains(before.as_str()),
        "before issue must be excluded: {ids:?}"
    );
    assert!(
        !ids.contains(after.as_str()),
        "after issue must be excluded: {ids:?}"
    );
    assert!(
        !ids.contains(unplanned.as_str()),
        "unplanned issue must be excluded: {ids:?}"
    );
}

/// Test 7 -- the personal query returns only the given assignee's
/// issues.
#[tokio::test]
async fn planned_for_user_returns_only_that_assignees_issues() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let project_id = create_personal_project(&app.db, &admin_id, "Test").await;

    let other = TestUser::new("bob");
    let other_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(&app.db, &other_id, &other.email, "x", &other.display_name)
        .await
        .expect("insert other user");

    let window_from = dt("2026-06-01T00:00:00Z");
    let window_to = dt("2026-06-30T23:59:59Z");

    let mine_id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &app.db,
        &mine_id,
        &project_id,
        &admin_id,
        issues::IssueFields {
            title: "Mine",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: Some(&admin_id),
            planned_start_at: Some(dt("2026-06-05T00:00:00Z")),
            planned_end_at: Some(dt("2026-06-06T00:00:00Z")),
        },
    )
    .await
    .expect("insert mine");

    let theirs_id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &app.db,
        &theirs_id,
        &project_id,
        &admin_id,
        issues::IssueFields {
            title: "Theirs",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: Some(&other_id),
            planned_start_at: Some(dt("2026-06-05T00:00:00Z")),
            planned_end_at: Some(dt("2026-06-06T00:00:00Z")),
        },
    )
    .await
    .expect("insert theirs");

    let results = issues::planned_for_user(&app.db, &admin_id, window_from, window_to)
        .await
        .expect("query planned_for_user");
    let ids: Vec<&str> = results.iter().map(|i| i.id.as_str()).collect();

    assert_eq!(
        ids,
        vec![mine_id.as_str()],
        "must contain only the given assignee's issues"
    );
}
