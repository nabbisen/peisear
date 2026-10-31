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

// ---------- CFG-002 round 2: from_values, the whole channel ----------

/// `RFC 022` §3: an unconfigured channel must not block a boot. A
/// statement about seven input values, tested as one -- `from_values`
/// takes every looked-up value as a parameter (`CFG-002` round 2),
/// the same reason `resolve_smtp_port`/`resolve_tls_mode` already do,
/// so this needs no real environment and no `unsafe`.
#[test]
fn an_entirely_unconfigured_channel_still_yields_ok_none() {
    match SmtpConfig::from_values(None, None, None, None, None, None, None) {
        Ok(None) => {}
        other => panic!("expected Ok(None) for a wholly unconfigured channel, got {other:?}"),
    }
}

/// The symmetric case, and a property worth recording rather than
/// discovering later: the four early `Ok(None)` returns run before
/// either resolver, so a wholly unconfigured channel with an
/// *additionally* malformed `SMTP_PORT` still degrades to `Ok(None)`
/// -- the bad port is never looked at, because `RFC 022` §3's first
/// half (absence degrades) does not care what else was typed.
#[test]
fn an_unconfigured_channel_still_degrades_even_with_a_malformed_port_present() {
    match SmtpConfig::from_values(None, None, None, None, Some("not-a-port"), None, None) {
        Ok(None) => {}
        other => panic!(
            "expected Ok(None) -- the early absence returns must run before the port \
             resolver ever sees the malformed value, got {other:?}"
        ),
    }
}

/// `CFG-002` round 2's own review: `from_values` takes seven
/// parameters, all `Option<&str>` -- a transposition at the call site
/// (`username` and `password`, say) compiles silently, and before
/// this test nothing in the suite would have caught it, since every
/// other `from_values` call here passes `None`s except one
/// `Some("not-a-port")`, and no test asserted any of the struct's own
/// string fields. Seven *distinct* values, each field asserted
/// against the parameter it came from, so a transposition fails this
/// test specifically rather than compiling clean and running green.
#[test]
fn from_values_assembles_every_field_from_its_own_named_parameter_not_a_transposed_one() {
    let config = SmtpConfig::from_values(
        Some("mail.example.com"),
        Some("the-username"),
        Some("the-password"),
        Some("sender@example.com"),
        Some("587"),
        Some("starttls"),
        Some("Example Sender"),
    )
    .expect("all seven values are present and well-formed")
    .expect("a fully configured channel must be Some");

    assert_eq!(config.host, "mail.example.com");
    assert_eq!(config.username, "the-username");
    assert_eq!(config.password, "the-password");
    assert_eq!(config.from_address, "sender@example.com");
    assert_eq!(config.port, 587);
    assert_eq!(config.tls_mode, TlsMode::Starttls);
    assert_eq!(config.from_name, Some("Example Sender".to_string()));
}
