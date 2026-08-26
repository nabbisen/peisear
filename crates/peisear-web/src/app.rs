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
        api_users, auth, calendar, issues, me, notification_preferences, notifications, projects,
        redirects, root, search, settings, sprints, teams,
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
            "/settings/notifications/silence-all",
            post(notification_preferences::silence_all),
        )
        // Inbox (formerly /notifications, renamed in 0.17.0 per
        // peisear-feature-spec-v2.1 §4.2). Old paths return 308
        // Permanent Redirect; see handlers::redirects for rationale.
        .route("/inbox", get(notifications::page))
        .route("/inbox/mark-all-read", post(notifications::mark_all_read))
        .route("/inbox/{id}/read", post(notifications::mark_read))
        // INBOX-001 (RFC 003): the silence-resume banner's action,
        // and the email opt-in prompt moved here from
        // /settings/notifications/ack-global.
        .route("/inbox/resume", post(notifications::resume))
        .route("/inbox/email-opt-in", post(notifications::email_opt_in))
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
        // CONF-001 (RFC 010): GET renders the confirmation
        // interstitial (serves both planned and completed sprints);
        // POST performs the delete, unchanged.
        .route(
            "/teams/{slug}/sprints/{sprint_id}/delete",
            get(sprints::delete_confirm).post(sprints::delete_sprint),
        )
        // Sprint planning page (PLAN-001 / RFC 001).
        .route(
            "/teams/{slug}/sprints/{sprint_id}/plan",
            get(sprints::plan_page),
        )
        .route(
            "/teams/{slug}/sprints/{sprint_id}/plan/add",
            post(sprints::plan_add),
        )
        .route(
            "/teams/{slug}/sprints/{sprint_id}/plan/remove",
            post(sprints::plan_remove),
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
        // Calendar, personal axis (CAL-002 / RFC 002).
        .route("/today/calendar", get(calendar::personal_page))
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
        // CONF-001 (RFC 010): GET renders the confirmation
        // interstitial; POST performs the delete, unchanged.
        .route(
            "/projects/{id}/delete",
            get(projects::delete_confirm).post(projects::delete),
        )
        // Calendar, project axis (CAL-002 / RFC 002).
        .route("/projects/{id}/calendar", get(calendar::project_page))
        // `HLT-001` (RFC 008 §1): a health indicator's basis set.
        .route(
            "/projects/{id}/health/{indicator}/basis",
            get(issues::health_indicator_basis),
        )
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
        // CONF-001 (RFC 010): GET renders the confirmation
        // interstitial; POST performs the delete, unchanged.
        .route(
            "/projects/{id}/issues/{issue_id}/delete",
            get(issues::delete_confirm).post(issues::delete),
        )
        .route(
            "/projects/{id}/issues/{issue_id}/status",
            post(issues::change_status),
        )
        // Keyboard-operable sibling of the JSON endpoint above
        // (DEV-002, `FR-DM-002`): a plain form POST, no JavaScript
        // required. Shares `apply_status_change`'s lock check with
        // the drag path — see that function's doc comment.
        .route(
            "/projects/{id}/issues/{issue_id}/status/board",
            post(issues::change_status_form),
        )
        // `STATUS-001` (RFC 004a step 1): the same shape as
        // `/status/board` above, one route per surface (handoff §5's
        // route shape (b)). No script anywhere in either path.
        .route(
            "/projects/{id}/issues/{issue_id}/status/detail",
            post(issues::change_status_form_detail),
        )
        .route(
            "/projects/{id}/issues/{issue_id}/status/list",
            post(issues::change_status_form_list),
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
