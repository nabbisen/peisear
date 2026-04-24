//! Server entry point — keeps only the bootstrap sequence. Router
//! wiring, state, and handlers all live in the library side.

use peisear_storage::pool;
use peisear_web::{AppState, Config, build_router};
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
        "starting issue tracker"
    );

    let db = pool::connect(&config.database_url).await?;
    pool::migrate(&db).await?;

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
