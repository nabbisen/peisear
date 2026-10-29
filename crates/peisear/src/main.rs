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
    // `WIZ-004`: ask the loader, not a second guess at where `.env`
    // lives. `dotenvy::dotenv()` searches cwd and every ancestor
    // directory (its own `find.rs`) -- the same resolution
    // `Config::from_env` below relies on -- so detection uses that
    // search's own result rather than `Path::new(".env").exists()`,
    // which only ever looked in cwd and missed a `.env` an ancestor
    // directory already supplied. No circularity: if the load sets
    // one of the four variables below, `configuration_exists`'s own
    // OR is true for that reason instead, not a spurious one -- the
    // two signals agree because they are now the same signal.
    // `Config::from_env` below calls `dotenvy::dotenv()` again; by
    // default it never overwrites a variable already set, so a
    // second call here is harmless whether this one found a file or
    // not -- it finds the same file, or the one the wizard just
    // wrote, in the same cwd.
    let env_path = Path::new(".env");
    let dot_env_loaded = dotenvy::dotenv().is_ok();
    let configuration_exists = wizard::configuration_exists(
        dot_env_loaded,
        std::env::var("DATABASE_URL").is_ok(),
        std::env::var("JWT_SECRET").is_ok(),
        std::env::var("BIND_ADDR").is_ok(),
        std::env::var("COOKIE_SECURE").is_ok(),
    );

    // `TTY-001`, `§10.49`: "is a terminal attached" is not "can a
    // human answer right now" -- a background job has the former and
    // not the latter, and the gap stops the process dead (`SIGTTIN`)
    // rather than starting it. `PEISEAR_SETUP` is read here, not
    // through `Config::from_env`: it is launch-environment vocabulary,
    // never `.env` vocabulary (`wizard` module docs), and an
    // unrecognised value is refused by name rather than silently read
    // as `auto`.
    let setup_mode = wizard::resolve_setup_mode(std::env::var("PEISEAR_SETUP").ok().as_deref())?;
    let is_terminal = std::io::stdin().is_terminal();
    let is_foreground = is_foreground_stdin();
    let can_prompt = wizard::can_prompt(setup_mode, is_terminal, is_foreground);

    if wizard::background_notice_needed(
        setup_mode,
        is_terminal,
        is_foreground,
        configuration_exists,
    ) {
        eprintln!("{}", wizard::background_notice());
    }

    if wizard::should_run_wizard(can_prompt, configuration_exists) {
        let answers = wizard::run_wizard(std::io::stdin().lock(), std::io::stdout())?;
        wizard::write_env_file(env_path, &answers)?;
        let jwt_path = wizard::jwt_secret_display_path(&answers.database_url);
        print!(
            "{}",
            wizard::wizard_statements(jwt_path.as_deref(), &env_path.display().to_string())
        );
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

    // `WIZ-005` §3: `Config::from_env` already parsed and validated
    // `BIND_ADDR` by name -- no second, unnamed `.parse()` here to
    // fail with an error naming neither the variable nor the value.
    let addr = config.bind_addr;
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

/// `TTY-001`, `§10.49`: "can we actually prompt" needs one more
/// signal than `IsTerminal` gives on Unix -- *is a terminal attached*
/// and *is this process allowed to read it* are different questions,
/// and the gap between them is `SIGTTIN`. Lives here, the one caller,
/// rather than in `peisear-web`: the syscall is the edge, the
/// decision it feeds ([`wizard::can_prompt`]) is the pure function a
/// test can call with booleans (`§6`).
#[cfg(unix)]
fn is_foreground_stdin() -> bool {
    use std::os::fd::AsFd;
    let stdin = std::io::stdin();
    let comparison = rustix::termios::tcgetpgrp(stdin.as_fd())
        .map(|tty_pgrp| tty_pgrp == rustix::process::getpgrp());
    resolve_is_foreground(comparison)
}

/// `TTY-001` round 2, `§1`: the honest unit-level form of the contract
/// `is_foreground_stdin` cannot be tested at directly (no real
/// terminal in a test process) -- a failure to even *ask* whether we
/// may read the terminal means we cannot establish that we may, so
/// we answer `false`, not `true`. `RFC 020` §7.3's own rule ("being
/// unable to prompt is not an error") applied one level down, to
/// being unable to ask whether we can prompt at all.
///
/// Round 1's own `Err(_) => true` reopened `§10.49` here: `tcgetpgrp`
/// fails with `ENOTTY` not only when stdin is not a terminal at all
/// (already screened out by [`wizard::can_prompt`]'s own `&&`, so a
/// value here is genuinely unconsulted in that case) **but also when
/// stdin is a terminal that is not our controlling one** -- `setsid`,
/// and several init/supervisor shapes, detach a process from its
/// controlling terminal while stdin still refers to a real tty
/// device. There, `true` let the wizard attempt a read that blocked
/// forever with no `SIGTTIN` to even show `Stopped` -- worse than the
/// defect this round fixes, not merely as bad -- and suppressed the
/// one-line notice too, since `is_foreground` reporting `true` made
/// `wizard::background_notice_needed` false.
#[cfg(unix)]
fn resolve_is_foreground<E>(terminal_pgrp_matches_ours: Result<bool, E>) -> bool {
    terminal_pgrp_matches_ours.unwrap_or(false)
}

/// `RFC 020` §7.2: no foreground-process-group equivalent exists on
/// Windows, and `is_terminal` is already the complete answer there --
/// a user who double-clicks `peisear.exe` gets a console
/// indistinguishable, by any other signal, from the `Start-Process`
/// case that never has a reader, and that user *is* one. Always
/// `true`, so [`wizard::can_prompt`] degenerates to `is_terminal`
/// alone, exactly as it did before this round.
#[cfg(windows)]
fn is_foreground_stdin() -> bool {
    true
}

#[cfg(all(test, unix))]
mod tests {
    use super::resolve_is_foreground;

    #[test]
    fn resolves_to_false_on_any_failure_to_ask() {
        // `tcgetpgrp` failing (`ENOTTY`, or anything else) must not
        // default to "yes, we may read" -- the planted regression
        // `TTY-001` round 1 shipped, reproduced here without a real
        // terminal.
        assert!(!resolve_is_foreground::<&str>(Err("ENOTTY")));
    }

    #[test]
    fn resolves_to_the_successful_comparison_when_the_syscall_answers() {
        assert!(resolve_is_foreground::<&str>(Ok(true)));
        assert!(!resolve_is_foreground::<&str>(Ok(false)));
    }
}
