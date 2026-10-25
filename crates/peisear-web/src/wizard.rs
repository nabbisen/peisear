//! The first-run wizard (`RFC 020`, `WIZ-001`–`WIZ-003`).
//!
//! `WIZ-001` shipped the skip/run detection. `WIZ-002` is this
//! module's own subject: two questions (`RFC 020` §1's sketch),
//! three statements (`§1a`'s correction — the cookie flag is not
//! silent), and a `.env` that says exactly what the statements said.

use std::io::{self, BufRead, Write};
use std::path::Path;

/// Whether the wizard should run at all, stated as the one predicate
/// `WIZ-001` §2 asks for rather than two scattered `if`s.
///
/// Runs only when stdin has nobody to answer it (`RFC 020` §3: *"a
/// wizard that blocks a `docker run` is an outage"*) **and** nothing
/// has configured this instance yet. `configuration_exists` answers
/// `RFC 020` §5's own open question — *"what happens on the second
/// run after a partial answer?"* — with **existence, not
/// completeness**: [`crate::config::Config::from_env`]'s own defaults
/// already make a `.env` that exists but omits a variable a working
/// configuration, so asking again would surprise an operator who has
/// already configured the instance.
///
/// Takes already-looked-up values, never stdin or the environment
/// itself — the same reason `config::resolve_jwt_secret` takes its
/// value as a parameter rather than calling `env::var` inside: the
/// only way this stays testable without a test reaching into the
/// process's real, shared state.
pub fn should_run_wizard(stdin_is_terminal: bool, configuration_exists: bool) -> bool {
    stdin_is_terminal && !configuration_exists
}

/// Whether any signal already says this instance is configured:
/// `.env`'s own existence (regardless of what it sets — existence,
/// not completeness), or any of the four variables
/// [`crate::config::Config::from_env`] reads already present in the
/// environment.
pub fn configuration_exists(
    dot_env_exists: bool,
    database_url_set: bool,
    jwt_secret_set: bool,
    bind_addr_set: bool,
    cookie_secure_set: bool,
) -> bool {
    dot_env_exists || database_url_set || jwt_secret_set || bind_addr_set || cookie_secure_set
}

/// `WIZ-001`'s own stub body, kept for what it proved and no longer
/// called from `main.rs`: [`run_wizard`] is the real body now.
pub fn run_wizard_stub() {
    println!("(first-run wizard would start here)");
}

/// `RFC 020` §1's own two answers — `database_url` (already in the
/// `sqlite://…` form [`crate::config::Config::from_env`] reads, not
/// the bare path the question asks for) and `bind_addr`. `JWT_SECRET`
/// is absent: it is **generated**, never answered. `COOKIE_SECURE` is
/// absent too, for a different reason (`§1a`): it is written, but at
/// its own built-in default, commented — never a value the wizard
/// asked about.
pub struct Answers {
    pub database_url: String,
    pub bind_addr: String,
}

/// The `.env` file's own text for `answers` — pure, so the write
/// itself can be a thin shell and every claim about the file's
/// content is an ordinary string assertion, never a read of a real
/// `.env` (`WIZ-002` §2: nobody in this pipeline may open one).
///
/// **Three lines, two of them live.** `DATABASE_URL` and `BIND_ADDR`
/// are the two literals [`crate::config::Config::from_env`] reads —
/// **not a shared constant, deliberately**: `docs_scan` rule 4
/// locates `config.rs`'s own default by searching for the literal
/// text `env::var("DATABASE_URL")`, and a constant in its place broke
/// that search silently (`§10.36`, round 1's own finding, fixed in
/// round 2 by making rule 4 assert the lookup itself before trusting
/// it — recorded there, not repeated here as a reason to keep
/// `wizard.rs` and `config.rs` apart; round 2 found the better fix).
/// The third, `COOKIE_SECURE`, is **commented, at its built-in
/// default** (`§1a`): not pinning a value lets a future release
/// change the default without this file silently overriding it, the
/// same reason `0.53.0` commented every line of `.env.example`.
pub fn env_file(answers: &Answers) -> String {
    format!(
        "DATABASE_URL={}\nBIND_ADDR={}\n# COOKIE_SECURE=0   # set to 1 when peisear is served over HTTPS\n",
        answers.database_url, answers.bind_addr,
    )
}

/// `WIZ-002` §2's own thin shell: writes [`env_file`]'s text to
/// `path`. Nothing here is unit-tested beyond what `env_file` itself
/// already proves as a string; the real write is exercised in CI
/// (`WIZ-001`'s own idiom), never by a test opening `.env` back.
pub fn write_env_file(path: &Path, answers: &Answers) -> io::Result<()> {
    std::fs::write(path, env_file(answers))
}

/// What one line of raw prompt input resolves to against its own
/// default — pure, so `RFC 020` §1's first property (*"every question
/// has a working default, so holding enter is a complete answer"*)
/// is tested without a real stdin.
pub fn resolve_prompt_answer(raw_line: &str, default: &str) -> String {
    let trimmed = raw_line.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

/// Resolves the database question's own raw answer, or says why it
/// cannot be used — checked before it is written anywhere, per round
/// 3's own review: an unconditional `sqlite://` prefix turned the
/// *documented* `sqlite://data/app.db` form into
/// `sqlite://sqlite://data/app.db`, which creates a directory
/// literally named `sqlite:`, **writes a generated signing key inside
/// it**, and only then fails to start — the opposite of *"every
/// question has a working default"* holding for an answer that is
/// not the default.
///
/// - An answer already carrying a scheme (`sqlite://`, `postgres://`)
///   is taken as given, never re-prefixed.
/// - An answer with none is prefixed with `sqlite://`, as before.
/// - Either way, the result must have a filesystem path for a
///   generated signing key to sit beside
///   ([`crate::config::database_file_path`]) — `None` covers both an
///   in-memory target and a backend this build does not have.
///   `peisear_storage::pool::connect`'s own deny-list already refuses
///   a PostgreSQL URL on a SQLite build; this restates that refusal
///   by name before a secret is ever generated against the answer,
///   rather than inventing a second rule.
fn validate_database_answer(raw: &str) -> Result<String, String> {
    let database_url = if raw.contains("://") {
        raw.to_string()
    } else {
        format!("sqlite://{raw}")
    };
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        return Err(format!(
            "{database_url} is a PostgreSQL connection string, and this build only has the sqlite backend."
        ));
    }
    if crate::config::database_file_path(&database_url).is_none() {
        return Err(format!(
            "{database_url} has no file for peisear to store data in (an in-memory database is not supported here)."
        ));
    }
    Ok(database_url)
}

/// Asks `RFC 020` §1's own two questions and returns the answers —
/// the one place in this module real I/O happens, and generic over
/// `BufRead`/`Write` rather than hardcoding `io::stdin()`/`io::stdout()`
/// so a test can hand it an in-memory buffer instead of the process's
/// real, shared streams.
///
/// **Re-asks rather than failing** when [`validate_database_answer`]
/// rejects an answer — the operator is sitting at the prompt, and
/// correcting it immediately is the one thing this wizard can do that
/// a startup error later cannot.
pub fn run_wizard<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<Answers> {
    writeln!(output)?;
    writeln!(
        output,
        "  No configuration found. Setting up a new instance."
    )?;
    writeln!(output)?;

    let database_url = loop {
        write!(output, "  Where should the database live?  [data/app.db] ")?;
        output.flush()?;
        let mut line = String::new();
        input.read_line(&mut line)?;
        let raw = resolve_prompt_answer(&line, "data/app.db");
        match validate_database_answer(&raw) {
            Ok(url) => break url,
            Err(reason) => writeln!(output, "    {reason} Try again.")?,
        }
    };

    writeln!(
        output,
        "  What address should it listen on?  [127.0.0.1:3000]"
    )?;
    writeln!(
        output,
        "    (press enter to keep it private to this machine)"
    )?;
    output.flush()?;
    let mut line = String::new();
    input.read_line(&mut line)?;
    let bind_addr = resolve_prompt_answer(&line, "127.0.0.1:3000");

    Ok(Answers {
        database_url,
        bind_addr,
    })
}

/// The generated key's own display path for `database_url`, or
/// `None` for the one case `RFC 020` §1's two questions can produce
/// deliberately — an in-memory database has no filesystem path for a
/// key file to sit beside. Thin, `pub` wrapper around
/// `crate::config`'s own `pub(crate)` parsing (`database_file_path`,
/// `jwt_secret_path`): `main.rs`, in the `peisear` binary crate, is
/// outside `peisear_web` and cannot reach a `pub(crate)` item there
/// directly, and this module is the natural place for the one caller
/// that needs it, not a reason to widen `config`'s own public surface.
pub fn jwt_secret_display_path(database_url: &str) -> Option<String> {
    let db_path = crate::config::database_file_path(database_url)?;
    Some(
        crate::config::jwt_secret_path(&db_path)
            .display()
            .to_string(),
    )
}

/// The three statements that follow the two questions — pure, so
/// `RFC 020` §1's own exact wording is tested as a string. `jwt_path`
/// is `None` only in the edge case the two questions cannot produce
/// deliberately (`database_url` pointing at an in-memory database,
/// which has no filesystem path for a key file to sit beside) —
/// everything a real wizard run writes always has one.
pub fn wizard_statements(jwt_path: Option<&str>, env_path: &str) -> String {
    let mut s = String::new();
    if let Some(path) = jwt_path {
        s.push_str(&format!(
            "\n  A signing key has been generated and saved to {path}.\n  Back it up with the database.\n"
        ));
    }
    s.push_str(
        "\n  Cookies are not marked Secure. Set COOKIE_SECURE=1 when peisear is served over HTTPS.\n",
    );
    s.push_str(&format!(
        "\n  Written to {env_path} — edit it any time, or delete it to start over.\n"
    ));
    s
}

#[cfg(test)]
mod tests;
