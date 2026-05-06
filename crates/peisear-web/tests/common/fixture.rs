//! Domain data factories. Use `peisear-storage` directly to insert
//! fixtures rather than driving the production form handlers,
//! because most test setups don't need to exercise the form-
//! validation path. Tests that *do* want to exercise form
//! validation can call the handler URLs directly via
//! `app.server.post(...)`.

use peisear_core::{IssueStatus, Priority};
use peisear_storage::{Pool, issues, projects};

/// Create a personal project owned by `owner_id`. Returns the
/// new project id.
pub async fn create_personal_project(
    db: &Pool,
    owner_id: &str,
    name: &str,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    projects::insert(db, &id, owner_id, name, "Test project", None)
        .await
        .expect("insert personal project");
    id
}

/// Create a basic open issue in a project. Defaults to
/// `priority = medium`, no effort, no assignee. Tests that need
/// other settings should construct via `issues::insert` directly.
pub async fn create_issue(
    db: &Pool,
    project_id: &str,
    author_id: &str,
    title: &str,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        db,
        &id,
        project_id,
        author_id,
        title,
        "Test issue body.",
        IssueStatus::Open,
        Priority::Medium,
        None,         // effort
        None,         // assignee
    )
    .await
    .expect("insert issue");
    id
}
