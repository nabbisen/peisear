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
