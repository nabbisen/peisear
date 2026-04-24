//! Server entry point.

use axum::{
    Router,
    routing::{get, post},
};
use peisear::{
    AppState, config::Config, db, handlers::{auth, issues, projects, root},
};
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn,hyper=warn,tower_http=info".into()),
        )
        .init();

    let config = Config::from_env();
    tracing::info!(
        database = %config.database_url,
        addr = %config.bind_addr,
        "starting issue tracker"
    );

    let pool = db::pool::connect(&config.database_url).await?;
    db::pool::migrate(&pool).await?;

    let state = AppState {
        db: pool,
        jwt_secret: config.jwt_secret,
    };

    let app = Router::new()
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
        .route(
            "/projects/{id}",
            get(issues::project_detail),
        )
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
        // Static assets
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = config.bind_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "listening");
    axum::serve(listener, app).await?;

    Ok(())
}
