//! peisear server entry point.
//!
//! This is the binary installed by `cargo install peisear`. It is
//! deliberately tiny: configuration loading, DB pool construction,
//! migration, and `axum::serve` invocation. All actual routing,
//! handlers, and rendering live in the `peisear-web` library, which
//! this crate re-exports.

use peisear::storage::pool;
use peisear::web::{AppState, Config, build_router, jobs};
use std::net::SocketAddr;

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
        "starting peisear"
    );

    let db = pool::connect(&config.database_url).await?;
    pool::migrate(&db).await?;

    // Read SMTP config from environment for the email channel.
    // Returns None if any required variable is missing; the
    // dispatcher gracefully degrades (in-app channel keeps
    // working, email sends are skipped). Q4 of 0.16.0 design.
    let smtp = peisear_notify::config::SmtpConfig::from_env();
    peisear_notify::config::SmtpConfig::log_startup(smtp.as_ref());

    // Spawn background jobs (snapshot writer today; future:
    // user-burnout snapshot, optional cleanup). The returned
    // shutdown senders are dropped at process exit, which signals
    // the tasks to stop their loops.
    let _shutdown_handles = jobs::spawn_all(db.clone(), smtp);

    let state = AppState {
        db,
        jwt_secret: config.jwt_secret,
        cookie_secure: config.cookie_secure,
    };

    let app = build_router(state);

    let addr: SocketAddr = config.bind_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "listening");
    axum::serve(listener, app).await?;

    Ok(())
}
