//! Askama templates used by handlers.
//!
//! Templates are kept thin: handlers build a view-model struct and
//! pass it to Askama. Askama's default HTML escaping neutralises XSS
//! in all interpolated values.
//!
//! `WebTemplate` (from `askama_web`) wires each template struct up to
//! axum 0.8's `IntoResponse` — this replaces the deprecated
//! `askama_axum` crate.

use askama::Template;
use askama_web::WebTemplate;

use crate::models::{CurrentUser, Issue, IssueStatus, Priority, Project};

#[derive(Template, WebTemplate)]
#[template(path = "login.html")]
pub struct LoginPage {
    pub flash: Option<String>,
    pub email: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "register.html")]
pub struct RegisterPage {
    pub flash: Option<String>,
    pub email: String,
    pub display_name: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "projects_list.html")]
pub struct ProjectsListPage {
    pub user: CurrentUser,
    pub projects: Vec<Project>,
    pub flash: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "project_new.html")]
pub struct ProjectNewPage {
    pub user: CurrentUser,
    pub flash: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "project_edit.html")]
pub struct ProjectEditPage {
    pub user: CurrentUser,
    pub project: Project,
    pub flash: Option<String>,
}

pub struct Column {
    pub status: IssueStatus,
    pub issues: Vec<Issue>,
}

#[derive(Template, WebTemplate)]
#[template(path = "project_detail.html")]
pub struct ProjectDetailPage {
    pub user: CurrentUser,
    pub project: Project,
    pub columns: Vec<Column>,
    pub view_mode: String, // "board" | "list"
    pub all_issues: Vec<Issue>,
    pub flash: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "issue_new.html")]
pub struct IssueNewPage {
    pub user: CurrentUser,
    pub project: Project,
    pub priorities: Vec<Priority>,
    pub statuses: Vec<IssueStatus>,
    pub flash: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "issue_detail.html")]
pub struct IssueDetailPage {
    pub user: CurrentUser,
    pub project: Project,
    pub issue: Issue,
    pub priorities: Vec<Priority>,
    pub statuses: Vec<IssueStatus>,
    pub flash: Option<String>,
    pub editing: bool,
}
