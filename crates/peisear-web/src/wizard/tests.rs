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

/// `WIZ-002` §2's third assertion: nothing is written that the
/// wizard did not say. Exactly two lines, exactly two keys -- not
/// `JWT_SECRET` (generated, never answered) and not `COOKIE_SECURE`
/// (correct at its default, never asked about).
#[test]
fn env_file_writes_nothing_but_the_two_answers() {
    let answers = Answers {
        database_url: "sqlite://data/app.db".to_string(),
        bind_addr: "127.0.0.1:3000".to_string(),
    };
    let text = env_file(&answers);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2, "expected exactly two lines, got {lines:?}");
    assert!(!text.contains("JWT_SECRET"));
    assert!(!text.contains("COOKIE_SECURE"));
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

    assert_eq!(
        parsed.len(),
        2,
        "parsed more or fewer keys than written: {parsed:?}"
    );
    assert_eq!(parsed.get("DATABASE_URL"), Some(&answers.database_url));
    assert_eq!(parsed.get("BIND_ADDR"), Some(&answers.bind_addr));
}
