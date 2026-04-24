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
    handlers::{auth, issues, projects, root},
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
        // Projects
        .route(
            "/projects",
            get(projects::list_page).post(projects::create),
        )
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
