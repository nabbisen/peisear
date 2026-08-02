//! Router / application factory. Kept separate from `main.rs` so that
//! integration tests can call [`build_router`] with a test pool and
//! exercise the same wiring the binary uses.

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

use crate::{
    AppState,
    handlers::{
        api_users, auth, issues, me, notification_preferences, notifications, projects, redirects,
        root, search, settings, sprints, teams,
    },
};

/// Build the full axum router given an already‑initialised state.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Public
        .route("/", get(root::index))
        .route("/health", get(root::health))
        .route("/login", get(auth::login_page).post(auth::login_submit))
        .route(
            "/register",
            get(auth::register_page).post(auth::register_submit),
        )
        .route("/logout", post(auth::logout))
        // Settings
        .route("/settings", get(settings::page))
        .route("/settings/capacity", post(settings::insert_capacity))
        .route("/settings/capacity/{id}", post(settings::update_capacity))
        .route(
            "/settings/capacity/{id}/delete",
            post(settings::delete_capacity),
        )
        .route(
            "/settings/capacity/{id}/close",
            post(settings::close_capacity),
        )
        .route("/settings/wip-limit", post(settings::update_wip_limit))
        // Notification preferences
        .route(
            "/settings/notifications",
            get(notification_preferences::page).post(notification_preferences::save_preferences),
        )
        .route(
            "/settings/notifications/ack-global",
            post(notification_preferences::ack_global),
        )
        .route(
            "/settings/notifications/silence-all",
            post(notification_preferences::silence_all),
        )
        // Inbox (formerly /notifications, renamed in 0.17.0 per
        // peisear-feature-spec-v2.1 §4.2). Old paths return 308
        // Permanent Redirect; see handlers::redirects for rationale.
        .route("/inbox", get(notifications::page))
        .route("/inbox/mark-all-read", post(notifications::mark_all_read))
        .route("/inbox/{id}/read", post(notifications::mark_read))
        // Legacy /notifications redirects → /inbox (308, preserves
        // POST method for the two POST endpoints).
        .route("/notifications", get(redirects::notifications_to_inbox))
        .route(
            "/notifications/mark-all-read",
            post(redirects::notifications_mark_all_read_to_inbox),
        )
        .route(
            "/notifications/{id}/read",
            post(redirects::notifications_read_to_inbox),
        )
        // Teams (0.14.0)
        .route("/teams", get(teams::list_page).post(teams::create))
        .route("/teams/new", get(teams::new_page))
        .route("/teams/{slug}", get(teams::detail))
        .route(
            "/teams/{slug}/edit",
            get(teams::edit_page).post(teams::update),
        )
        .route("/teams/{slug}/members", post(teams::add_member))
        .route(
            "/teams/{slug}/members/{user_id}/role",
            post(teams::update_member_role),
        )
        .route(
            "/teams/{slug}/members/{user_id}/remove",
            post(teams::remove_member),
        )
        .route(
            "/teams/{slug}/projects/{project_id}/unassign",
            post(teams::unassign_project),
        )
        // Sprints (0.15.0)
        .route(
            "/teams/{slug}/sprints",
            get(sprints::list_page).post(sprints::create),
        )
        .route("/teams/{slug}/sprints/new", get(sprints::new_page))
        .route("/teams/{slug}/sprints/{sprint_id}", get(sprints::detail))
        .route(
            "/teams/{slug}/sprints/{sprint_id}/edit",
            get(sprints::edit_page).post(sprints::update),
        )
        .route(
            "/teams/{slug}/sprints/{sprint_id}/start",
            post(sprints::start),
        )
        .route(
            "/teams/{slug}/sprints/{sprint_id}/complete",
            post(sprints::complete),
        )
        .route(
            "/teams/{slug}/sprints/{sprint_id}/delete",
            post(sprints::delete_sprint),
        )
        .route(
            "/projects/{project_id}/issues/{issue_id}/sprint",
            post(sprints::assign_issue),
        )
        // Personal dashboard. Renamed from /me to /today in 0.17.0
        // per peisear-feature-spec-v2.1 §4.2; legacy /me returns
        // 308 Permanent Redirect.
        .route("/today", get(me::page))
        .route("/me", get(redirects::me_to_today))
        // Global search (Phase A Step 4, peisear-feature-spec-v2.1 §4.5).
        // /search is the HTML results page (form submission, direct URL).
        // /api/search is the JSON typeahead used by the navbar input.
        .route("/search", get(search::results_page))
        .route("/api/search", get(search::typeahead))
        // Personal-data JSON API (Phase B PR2,
        // peisear-feature-spec-v2.1 §11.5). All three return
        // the requesting user's own data; cross-user reads are
        // 403, unauth requests are 401 (JSON, not redirect).
        .route("/api/users/{user_id}/burnout", get(api_users::burnout))
        .route("/api/users/{user_id}/capacity", get(api_users::capacity))
        .route(
            "/api/users/{user_id}/notifications",
            get(api_users::list_notifications),
        )
        // Projects
        .route("/projects", get(projects::list_page).post(projects::create))
        .route("/projects/new", get(projects::new_page))
        .route("/projects/{id}", get(issues::project_detail))
        .route(
            "/projects/{id}/edit",
            get(projects::edit_page).post(projects::update),
        )
        .route("/projects/{id}/delete", post(projects::delete))
        // Issues
        .route(
            "/projects/{id}/issues/new",
            get(issues::new_page).post(issues::create),
        )
        .route(
            "/projects/{id}/issues/{issue_id}",
            get(issues::detail_page).post(issues::update),
        )
        // Phase B PR3 (B-3): edit mode is now an explicit URL,
        // not `?edit=1`. The legacy parameter on the detail
        // route 308-redirects here so bookmarks/links still
        // work. The handler shares its data-loading code with
        // `detail_page`; only the `is_edit_mode` flag changes.
        .route(
            "/projects/{id}/issues/{issue_id}/edit",
            get(issues::edit_page),
        )
        // Phase C PR1: sub-issue creation form lives at
        // `/sub-issues/new` under the parent. GET renders the
        // form; POST creates the row and redirects back to
        // the parent's detail page.
        .route(
            "/projects/{id}/issues/{issue_id}/sub-issues/new",
            get(issues::new_sub_issue_form).post(issues::create_sub_issue),
        )
        .route(
            "/projects/{id}/issues/{issue_id}/delete",
            post(issues::delete),
        )
        .route(
            "/projects/{id}/issues/{issue_id}/status",
            post(issues::change_status),
        )
        // Static assets served from the directory named "static" in the
        // working directory of the running binary. For typical
        // `cargo run`‑from‑workspace‑root usage this resolves to
        // `<workspace>/static/`. See README for deployment guidance.
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
