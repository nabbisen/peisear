//! peisear server entry point.
//!
//! This is the binary installed by `cargo install peisear`. It is
//! deliberately tiny: configuration loading, DB pool construction,
//! migration, and `axum::serve` invocation. All actual routing,
//! handlers, and rendering live in the `peisear-web` library, which
//! this crate re-exports.

use peisear::storage::pool;
use peisear::web::{AppState, Config, build_router, jobs, wizard};
use std::io::IsTerminal;
use std::net::SocketAddr;
use std::path::Path;
use std::process::ExitCode;

/// `FIV-001` round 2, `§10`: `main` returning `Result<(), Box<dyn
/// Error>>` prints the error's **`Debug`** on exit (Rust's default
/// `Termination` impl) -- `ConfigError::KeyMalformed`'s carefully
/// written remedy, and every other error this function can produce,
/// would otherwise reach the terminal as `Error: KeyMalformed { path:
/// "data/jwt_secret", expected: 32, found: 3 }` rather than the
/// message itself. `main` now matches on `run`'s result and prints
/// `Display` instead, for any failure here -- not only `Config`'s,
/// since the same printing defect applies equally to a pool-connect
/// or bind failure and there is no reason to fix it for one error type
/// and not the others in a function this size.
#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn,hyper=warn,tower_http=info".into()),
        )
        .init();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // `WIZ-001`: the wizard, not yet built (`WIZ-002`'s own subject) --
    // only the skip/run detection, so the property that matters most
    // (`RFC 020` §3: "a wizard that blocks a `docker run` is an
    // outage") is proven before there is anything to prompt with.
    // `Config::from_env` below calls `dotenvy::dotenv()` again; doing
    // so twice is harmless, since the second call sets nothing a
    // variable here has not already read.
    let configuration_exists = wizard::configuration_exists(
        Path::new(".env").exists(),
        std::env::var("DATABASE_URL").is_ok(),
        std::env::var("JWT_SECRET").is_ok(),
        std::env::var("BIND_ADDR").is_ok(),
        std::env::var("COOKIE_SECURE").is_ok(),
    );
    if wizard::should_run_wizard(std::io::stdin().is_terminal(), configuration_exists) {
        wizard::run_wizard_stub();
    }

    let config = Config::from_env()?;
    // `SEC-001` §1: redacted at the source, not inline here, so a future
    // second startup line that also wants to name the database cannot
    // reintroduce the leak by logging `database_url` directly.
    tracing::info!(
        database = %config.loggable_database_url(),
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

    // `RFC 017` barrier 5 / `FIV-004` §2: a plain-words line on a
    // successful bind, after the structured record above rather than
    // in place of it -- that line is the record a log aggregator
    // reads; this one is for whoever is watching the terminal
    // directly, on their first run, with nobody else's output to
    // tell the two apart by. `0.0.0.0` is correct and also the thing
    // a first-time reader has to translate into `localhost`
    // themselves, which is the barrier this removes -- printed only
    // once binding has actually succeeded, so it is never shown by a
    // process that failed to start.
    if addr.ip().is_loopback() {
        println!("  → http://localhost:{}", addr.port());
    } else {
        // Loopback stopped being the default the moment `BIND_ADDR`
        // is set to anything else (`FIV-001`), which after this
        // release only happens deliberately -- an operator's own
        // choice, or the container image's own `ENV` (`FIV-003`).
        // `0.0.0.0` itself is not a URL anyone can open, so this
        // names the fact rather than offering a link that would not
        // work.
        println!("  Listening on {addr} -- reachable from the network, not just this machine.");
    }

    axum::serve(listener, app).await?;

    Ok(())
}
