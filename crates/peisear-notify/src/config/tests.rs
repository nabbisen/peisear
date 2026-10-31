use super::*;

#[test]
fn tls_mode_default_465_is_implicit() {
    assert_eq!(TlsMode::default_for_port(465), TlsMode::Implicit);
}

#[test]
fn tls_mode_default_587_is_starttls() {
    assert_eq!(TlsMode::default_for_port(587), TlsMode::Starttls);
}

#[test]
fn tls_mode_from_str_accepts_aliases() {
    assert_eq!(TlsMode::from_str_loose("implicit"), Some(TlsMode::Implicit));
    assert_eq!(TlsMode::from_str_loose("smtps"), Some(TlsMode::Implicit));
    assert_eq!(TlsMode::from_str_loose("starttls"), Some(TlsMode::Starttls));
    assert_eq!(
        TlsMode::from_str_loose("submission"),
        Some(TlsMode::Starttls)
    );
    assert_eq!(TlsMode::from_str_loose("garbage"), None);
}

// ---------- CFG-002: resolve_smtp_port ----------

#[test]
fn resolve_smtp_port_degrades_to_465_when_unset() {
    assert_eq!(
        resolve_smtp_port(None).expect("absence must not refuse"),
        465
    );
}

/// `RFC 022` §7: an explicitly empty value is not one of the five
/// absence-shaped `SMTP_*` variables, so it is a present value this
/// function must try to use, not a second spelling of absence.
#[test]
fn resolve_smtp_port_refuses_an_empty_value_rather_than_treating_it_as_absent() {
    match resolve_smtp_port(Some("")) {
        Err(SmtpConfigError::InvalidPort { value }) => assert_eq!(value, ""),
        other => panic!("expected InvalidPort for an empty value, got {other:?}"),
    }
}

#[test]
fn resolve_smtp_port_refuses_a_value_that_does_not_parse_as_u16() {
    for bad in ["not-a-port", "99999", "-1", "465.0"] {
        match resolve_smtp_port(Some(bad)) {
            Err(SmtpConfigError::InvalidPort { value }) => assert_eq!(value, bad),
            other => panic!("expected InvalidPort for {bad:?}, got {other:?}"),
        }
    }
}

/// `§3`'s own gate: the error must name the variable and the value --
/// not a bare, unnamed `ParseIntError` display.
#[test]
fn invalid_port_message_names_the_variable_and_the_value() {
    let message = SmtpConfigError::InvalidPort {
        value: "not-a-port".to_string(),
    }
    .to_string();
    assert!(message.contains("SMTP_PORT"), "got {message:?}");
    assert!(message.contains("not-a-port"), "got {message:?}");
}

// ---------- CFG-002: resolve_tls_mode ----------

#[test]
fn resolve_tls_mode_degrades_to_the_port_derived_default_when_unset() {
    assert_eq!(
        resolve_tls_mode(None, 465).expect("absence must not refuse"),
        TlsMode::Implicit
    );
    assert_eq!(
        resolve_tls_mode(None, 587).expect("absence must not refuse"),
        TlsMode::Starttls
    );
}

#[test]
fn resolve_tls_mode_accepts_every_spelling_from_str_loose_accepts() {
    for (spelling, mode) in [
        ("implicit", TlsMode::Implicit),
        ("implicit_tls", TlsMode::Implicit),
        ("smtps", TlsMode::Implicit),
        ("tls", TlsMode::Implicit),
        ("starttls", TlsMode::Starttls),
        ("start_tls", TlsMode::Starttls),
        ("submission", TlsMode::Starttls),
    ] {
        assert_eq!(
            resolve_tls_mode(Some(spelling), 465).expect("must be accepted"),
            mode,
            "{spelling:?} must be accepted"
        );
    }
}

#[test]
fn resolve_tls_mode_refuses_an_empty_value_rather_than_treating_it_as_absent() {
    match resolve_tls_mode(Some(""), 465) {
        Err(SmtpConfigError::InvalidTlsMode { value, .. }) => assert_eq!(value, ""),
        other => panic!("expected InvalidTlsMode for an empty value, got {other:?}"),
    }
}

/// `§3`'s own gate, and `§2`'s: the message names the variable,
/// quotes the value, and lists what `from_str_loose` **actually**
/// accepts -- the full alias set, not `configuration.md`'s own
/// tidier two canonical names.
#[test]
fn invalid_tls_mode_message_names_the_variable_the_value_and_every_accepted_spelling() {
    let message = SmtpConfigError::InvalidTlsMode {
        value: "bogus".to_string(),
        accepted: TlsMode::accepted_spellings(),
    }
    .to_string();
    assert!(message.contains("SMTP_TLS_MODE"), "got {message:?}");
    assert!(message.contains("bogus"), "got {message:?}");
    for spelling in [
        "implicit",
        "implicit_tls",
        "smtps",
        "tls",
        "starttls",
        "start_tls",
        "submission",
    ] {
        assert!(message.contains(spelling), "got {message:?}");
    }
}

// ---------- CFG-002: the cascade this round closes ----------

/// Before this round, a malformed `SMTP_PORT` fell back to `465` via
/// `.unwrap_or(465)`, and that defaulted port then silently decided
/// `SMTP_TLS_MODE` too (`TlsMode::default_for_port`) -- changing
/// transport security with no sign anything was wrong. `from_env`'s
/// own `?` after `resolve_smtp_port` means `resolve_tls_mode` is
/// never reached on that path. Proven here by composing the two
/// exactly as `from_env` does, rather than by reading the control
/// flow and trusting it.
#[test]
fn a_malformed_port_refuses_before_any_tls_mode_is_derived() {
    fn resolve_port_then_tls_mode(
        port_raw: Option<&str>,
    ) -> Result<(u16, TlsMode), SmtpConfigError> {
        let port = resolve_smtp_port(port_raw)?;
        let tls_mode = resolve_tls_mode(None, port)?;
        Ok((port, tls_mode))
    }

    match resolve_port_then_tls_mode(Some("not-a-port")) {
        Err(SmtpConfigError::InvalidPort { value }) => assert_eq!(value, "not-a-port"),
        other => {
            panic!("expected InvalidPort before any TlsMode was derived, got {other:?}")
        }
    }
}

// ---------- CFG-002: from_env itself ----------

/// `RFC 022` §3: an unconfigured channel must not block a boot. The
/// one test in this module that calls the real [`SmtpConfig::from_env`]
/// rather than a pure resolver -- deliberately, since this is exactly
/// the property `main`'s own `?` depends on: `Ok(None)`, never `Err`,
/// when nothing is configured at all. Explicitly clears every `SMTP_*`
/// variable first rather than assuming the ambient process environment
/// has none of them set.
#[test]
fn an_entirely_unconfigured_channel_still_yields_ok_none() {
    for var in [
        "SMTP_HOST",
        "SMTP_USER",
        "SMTP_PASSWORD",
        "SMTP_FROM_ADDRESS",
        "SMTP_PORT",
        "SMTP_TLS_MODE",
        "SMTP_FROM_NAME",
    ] {
        // SAFETY: `cargo test -p peisear-notify` runs with
        // `--test-threads=1` (`.github/CONTRIBUTING.md`), so no other
        // test observes the environment concurrently with this one.
        unsafe {
            std::env::remove_var(var);
        }
    }
    match SmtpConfig::from_env() {
        Ok(None) => {}
        other => panic!("expected Ok(None) for a wholly unconfigured channel, got {other:?}"),
    }
}
