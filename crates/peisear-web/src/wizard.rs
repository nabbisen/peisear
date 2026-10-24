//! The first-run wizard (`RFC 020`, `WIZ-001`–`WIZ-003`).
//!
//! `WIZ-001` ships only the skip/run detection and a stub body:
//! [`run_wizard_stub`] prints one line and returns. `WIZ-002` replaces
//! that body with the real two-question wizard; the predicate below is
//! already the property this whole series depends on, and it is
//! proven in CI against the real binary rather than only read.
//!
//! **`WIZ-002` is half-built, deliberately.** `ROADMAP.md`'s
//! authorized row says the wizard asks five questions; `RFC 020` §1's
//! own sketch asks two. That disagreement is the owner's to settle,
//! not this round's, so the prompts themselves — and `main.rs`'s own
//! call to [`run_wizard_stub`] — are not built yet. [`Answers`] and
//! [`env_file`] are independent of the question count (both take and
//! write the two variables either sketch agrees on, `DATABASE_URL`
//! and `BIND_ADDR`) and are built and proven here, per the round's own
//! handoff §1: *"do not build the question list until the answer
//! arrives — but do not idle either."*

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

/// The two answers the wizard's own sketch (`RFC 020` §1) asks for —
/// `database_url` and `bind_addr` — independent of whether the owner
/// settles on two questions or five: every sketch on record asks at
/// least these two. `JWT_SECRET` and `COOKIE_SECURE` are deliberately
/// absent: the former is **generated**, never answered
/// (`config::resolve_generated_jwt_secret`, private), and the latter
/// is correct at its default and not written at all, so `.env` never
/// pins a value the wizard did not actually ask about.
pub struct Answers {
    pub database_url: String,
    pub bind_addr: String,
}

/// The `.env` file's own text for `answers` — pure, so the write
/// itself can be a thin shell and every claim about the file's
/// content is an ordinary string assertion, never a read of a real
/// `.env` (`WIZ-002` §2: nobody in this pipeline may open one).
///
/// **Nothing is written that `answers` did not carry.** Exactly two
/// lines, `DATABASE_URL` and `BIND_ADDR` — the same two literals
/// [`crate::config::Config::from_env`] reads. **Not a shared
/// constant, deliberately**: `docs_scan` rule 4 locates
/// `config.rs`'s own default by searching for the literal text
/// `env::var("DATABASE_URL")`, and a constant in its place would have
/// made that search -- an existing, independently useful guard --
/// find nothing and silently stop checking (found by running it after
/// trying the constant first). A rename on *this* side is still
/// caught, by this module's own round-trip test parsing this
/// function's output for the same literal; a rename on `config.rs`'s
/// side is not caught here, and is this
/// round's one named, accepted gap -- closed naturally once a real
/// `.env` this function wrote is round-tripped through a real running
/// `peisear`, which needs the prompts `WIZ-002` §1 is still waiting
/// on.
pub fn env_file(answers: &Answers) -> String {
    format!(
        "DATABASE_URL={}\nBIND_ADDR={}\n",
        answers.database_url, answers.bind_addr,
    )
}

#[cfg(test)]
mod tests;
