//! The first-run wizard (`RFC 020`, `WIZ-001`–`WIZ-003`).
//!
//! `WIZ-001` ships only the skip/run detection and a stub body:
//! [`run_wizard_stub`] prints one line and returns. `WIZ-002` replaces
//! that body with the real two-question wizard; the predicate below is
//! already the property this whole series depends on, and it is
//! proven in CI against the real binary rather than only read.

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

/// `WIZ-001`'s own stub body. `WIZ-002` replaces this with the real
/// two-question wizard that writes `.env`; printing one line and
/// returning is enough to prove the skip/run detection in CI before
/// the feature it guards exists.
pub fn run_wizard_stub() {
    println!("(first-run wizard would start here)");
}

#[cfg(test)]
mod tests;
