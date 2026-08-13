//! Domain data factories. Use `peisear-storage` directly to insert
//! fixtures rather than driving the production form handlers,
//! because most test setups don't need to exercise the form-
//! validation path. Tests that *do* want to exercise form
//! validation can call the handler URLs directly via
//! `app.server.post(...)`.

use peisear_core::{IssueStatus, Priority};
use peisear_storage::{Pool, issues, projects, sprints, teams};

/// Create a personal project owned by `owner_id`. Returns the
/// new project id.
pub async fn create_personal_project(db: &Pool, owner_id: &str, name: &str) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    projects::insert(db, &id, owner_id, name, "Test project", None)
        .await
        .expect("insert personal project");
    id
}

/// Create a project owned by `owner_id`, belonging to `team_id`.
/// Unlike [`create_personal_project`], does not require `owner_id`
/// to be a member of `team_id` — `TEAM-001` needs to construct that
/// exact state (an owner who isn't a team member) as a fixture, not
/// just as an incidental side effect.
pub async fn create_team_project(db: &Pool, owner_id: &str, team_id: &str, name: &str) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    projects::insert(db, &id, owner_id, name, "Test project", Some(team_id))
        .await
        .expect("insert team project");
    id
}

/// Create a team with the given user as the initial admin.
/// Returns the team id. The team's slug is derived from the
/// name; tests that need a specific slug should call
/// `teams::insert` directly.
pub async fn create_team_with_admin(db: &Pool, admin_user_id: &str, name: &str) -> String {
    let slug = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();
    teams::insert(db, name, &slug, None, admin_user_id)
        .await
        .expect("insert team")
}

/// Create a planned sprint in a team. Defaults to a 14-day
/// window starting today. Returns the sprint id.
pub async fn create_planned_sprint(db: &Pool, team_id: &str, name: &str) -> String {
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);
    sprints::insert(db, team_id, name, None, today, ends)
        .await
        .expect("insert sprint")
}

/// Create a basic open issue in a project. Defaults to
/// `priority = medium`, no effort, no assignee. Tests that need
/// other settings should construct via `issues::insert` directly.
pub async fn create_issue(db: &Pool, project_id: &str, author_id: &str, title: &str) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        db,
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
        },
    )
    .await
    .expect("insert issue");
    id
}
