use super::*;

#[test]
fn skips_when_stdin_is_not_a_terminal_regardless_of_configuration() {
    assert!(!should_run_wizard(false, false));
    assert!(!should_run_wizard(false, true));
}

#[test]
fn skips_when_configuration_already_exists_even_on_a_terminal() {
    assert!(!should_run_wizard(true, true));
}

#[test]
fn runs_only_when_a_terminal_has_no_existing_configuration() {
    assert!(should_run_wizard(true, false));
}

/// `RFC 020` §5's own open question, answered as existence, not
/// completeness: `.env`'s own presence is enough, regardless of
/// whether any of the four variables is also set.
#[test]
fn configuration_exists_on_dot_env_presence_alone() {
    assert!(configuration_exists(true, false, false, false, false));
}

#[test]
fn configuration_exists_on_any_single_variable_without_a_dot_env() {
    assert!(configuration_exists(false, true, false, false, false));
    assert!(configuration_exists(false, false, true, false, false));
    assert!(configuration_exists(false, false, false, true, false));
    assert!(configuration_exists(false, false, false, false, true));
}

#[test]
fn configuration_does_not_exist_with_no_dot_env_and_nothing_set() {
    assert!(!configuration_exists(false, false, false, false, false));
}

// ---------- WIZ-002: env_file, the independent half ----------

/// Every variable the wizard carried in `Answers` appears in the
/// written text with that same value -- `WIZ-002` §2's first
/// assertion.
#[test]
fn env_file_writes_both_answers_with_their_given_values() {
    let answers = Answers {
        database_url: "sqlite://data/app.db".to_string(),
        bind_addr: "127.0.0.1:3000".to_string(),
    };
    let text = env_file(&answers);
    assert!(text.contains("DATABASE_URL=sqlite://data/app.db"));
    assert!(text.contains("BIND_ADDR=127.0.0.1:3000"));
}

/// `WIZ-002` §2's third assertion, updated by `§1a`'s own correction:
/// nothing is written that the wizard did not say -- which is now
/// *two uncommented lines, plus the one commented `COOKIE_SECURE`
/// line* `§1a` asks for, not "exactly two lines" as round 1 had it.
/// Loosened deliberately, and said so here rather than quietly, per
/// `§1a`'s own instruction: *"a test loosened quietly is worse than
/// the gap it was hiding."* Never `JWT_SECRET` (generated, never
/// answered), and `COOKIE_SECURE` only ever commented, never live.
#[test]
fn env_file_writes_two_uncommented_lines_plus_one_commented_cookie_secure_line() {
    let answers = Answers {
        database_url: "sqlite://data/app.db".to_string(),
        bind_addr: "127.0.0.1:3000".to_string(),
    };
    let text = env_file(&answers);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3, "expected three lines, got {lines:?}");
    assert!(lines[0].starts_with("DATABASE_URL="));
    assert!(lines[1].starts_with("BIND_ADDR="));
    assert!(
        lines[2].starts_with("# COOKIE_SECURE="),
        "COOKIE_SECURE must be commented, got {:?}",
        lines[2]
    );
    assert!(!text.contains("\nJWT_SECRET"));
}

/// `§1a`'s own fix, checked precisely: the commented line's own
/// value is the built-in default (`0`, matching `config.rs`'s own
/// `unwrap_or(false)`), not pinned to anything the wizard asked
/// about -- there was no question to ask.
#[test]
fn env_file_s_cookie_secure_line_is_commented_at_its_built_in_default() {
    let answers = Answers {
        database_url: "sqlite://data/app.db".to_string(),
        bind_addr: "127.0.0.1:3000".to_string(),
    };
    let text = env_file(&answers);
    assert!(text.contains("# COOKIE_SECURE=0"), "got {text:?}");
}

/// `WIZ-002` §2's second, and most important, assertion: the file is
/// loadable by the same path that reads it. `dotenvy::from_read_iter`
/// is the same parser `Config::from_env`'s own `dotenvy::dotenv()`
/// uses, and -- unlike `dotenv()`/`from_path()` -- it never touches
/// the process's real environment, so this is safe to run alongside
/// every other test in this binary rather than racing them for a
/// shared, global `env::var`.
#[test]
fn env_file_round_trips_through_dotenvy_to_the_same_answers() {
    let answers = Answers {
        database_url: "sqlite:///srv/peisear/app.db".to_string(),
        bind_addr: "0.0.0.0:3000".to_string(),
    };
    let text = env_file(&answers);

    let parsed: std::collections::HashMap<String, String> =
        dotenvy::from_read_iter(text.as_bytes())
            .collect::<Result<_, _>>()
            .expect("env_file's own output must be valid .env syntax");

    // Exactly two *active* keys: the commented COOKIE_SECURE line is
    // not a key dotenvy's own parser reads, which is the point of
    // commenting it rather than writing it live (`§1a`).
    assert_eq!(
        parsed.len(),
        2,
        "parsed more or fewer keys than written: {parsed:?}"
    );
    assert_eq!(parsed.get("DATABASE_URL"), Some(&answers.database_url));
    assert_eq!(parsed.get("BIND_ADDR"), Some(&answers.bind_addr));
    assert!(
        !parsed.contains_key("COOKIE_SECURE"),
        "the commented COOKIE_SECURE line must not parse as a live key: {parsed:?}"
    );
}

// ---------- WIZ-002 round 3: the real prompts ----------

#[test]
fn resolve_prompt_answer_keeps_the_default_on_blank_input() {
    assert_eq!(resolve_prompt_answer("\n", "data/app.db"), "data/app.db");
    assert_eq!(resolve_prompt_answer("", "data/app.db"), "data/app.db");
    assert_eq!(resolve_prompt_answer("   \n", "data/app.db"), "data/app.db");
}

#[test]
fn resolve_prompt_answer_uses_the_typed_value_when_present() {
    assert_eq!(
        resolve_prompt_answer("custom/path.db\n", "data/app.db"),
        "custom/path.db"
    );
}

#[test]
fn resolve_prompt_answer_trims_surrounding_whitespace() {
    assert_eq!(
        resolve_prompt_answer("  0.0.0.0:8080  \n", "127.0.0.1:3000"),
        "0.0.0.0:8080"
    );
}

/// `RFC 020` §1's own first property, proven end to end: holding
/// enter (a blank line) for both questions is a complete answer, and
/// the database path is wrapped into the `sqlite://` form
/// `Config::from_env` expects.
#[test]
fn run_wizard_uses_both_defaults_on_blank_input() {
    let input = io::Cursor::new(b"\n\n".to_vec());
    let mut output = Vec::new();
    let answers = run_wizard(input, &mut output).expect("wizard runs to completion");
    assert_eq!(answers.database_url, "sqlite://data/app.db");
    assert_eq!(answers.bind_addr, "127.0.0.1:3000");
}

#[test]
fn run_wizard_uses_typed_answers_when_given() {
    let input = io::Cursor::new(b"custom/db.sqlite\n0.0.0.0:8080\n".to_vec());
    let mut output = Vec::new();
    let answers = run_wizard(input, &mut output).expect("wizard runs to completion");
    assert_eq!(answers.database_url, "sqlite://custom/db.sqlite");
    assert_eq!(answers.bind_addr, "0.0.0.0:8080");
}

#[test]
fn run_wizard_prints_both_questions() {
    let input = io::Cursor::new(b"\n\n".to_vec());
    let mut output = Vec::new();
    run_wizard(input, &mut output).expect("wizard runs to completion");
    let text = String::from_utf8(output).expect("valid utf-8");
    assert!(text.contains("Where should the database live?"));
    assert!(text.contains("What address should it listen on?"));
}

#[test]
fn jwt_secret_display_path_sits_beside_the_database_file() {
    assert_eq!(
        jwt_secret_display_path("sqlite://data/app.db"),
        Some("data/jwt_secret".to_string())
    );
}

#[test]
fn jwt_secret_display_path_is_none_for_an_in_memory_database() {
    assert_eq!(jwt_secret_display_path("sqlite::memory:"), None);
    assert_eq!(jwt_secret_display_path("sqlite://:memory:"), None);
}

#[test]
fn wizard_statements_names_the_jwt_path_and_env_path_when_given() {
    let text = wizard_statements(Some("data/jwt_secret"), ".env");
    assert!(text.contains("saved to data/jwt_secret"));
    assert!(text.contains("Written to .env"));
    assert!(text.contains("COOKIE_SECURE=1"));
}

/// The one edge case `RFC 020` §1's two questions can produce
/// deliberately -- an in-memory database has no filesystem path for
/// a key file to sit beside -- must not claim a path that does not
/// exist.
#[test]
fn wizard_statements_omits_the_signing_key_line_when_there_is_no_filesystem_path() {
    let text = wizard_statements(None, ".env");
    assert!(!text.contains("signing key"));
    assert!(text.contains("COOKIE_SECURE=1"));
    assert!(text.contains("Written to .env"));
}
