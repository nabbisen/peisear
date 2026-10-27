use super::*;

#[test]
fn redact_dsn_strips_userinfo_and_query_but_keeps_scheme_host_and_path() {
    assert_eq!(
        redact_dsn("sqlite://data/app.db"),
        "sqlite://data/app.db",
        "no userinfo to strip -- must round-trip unchanged"
    );
    assert_eq!(
        redact_dsn("postgres://alice:s3cr3t@db.example.com/peisear"),
        "postgres://db.example.com/peisear"
    );
    assert_eq!(
        redact_dsn("postgres://alice:s3cr3t@db.example.com:5432/peisear?sslmode=require"),
        "postgres://db.example.com:5432/peisear",
        "a query string is dropped along with the userinfo"
    );
    assert_eq!(redact_dsn("sqlite://:memory:"), "sqlite://:memory:");
}

/// `SEC-001` §1's own instruction: a redactor only exercised on input
/// with nothing to redact proves nothing (`§10.36`'s shape) — this
/// asserts the sentinel password itself is absent from the result,
/// not merely that the function returns *something*.
#[test]
fn redact_dsn_removes_a_real_looking_password() {
    let dsn = "postgres://produser:hunter2-sentinel@10.0.0.5:5432/peisear";
    let redacted = redact_dsn(dsn);
    assert!(
        !redacted.contains("hunter2-sentinel"),
        "the sentinel password must not appear in the redacted form, got {redacted:?}"
    );
    assert!(
        !redacted.contains("produser"),
        "the username must not appear either, got {redacted:?}"
    );
}

#[test]
fn debug_never_prints_the_jwt_secret_or_the_dsn_password() {
    let config = Config {
        database_url: "postgres://produser:hunter2-sentinel@10.0.0.5:5432/peisear".to_string(),
        jwt_secret: "sentinel-signing-key-do-not-print-me".to_string(),
        bind_addr: "0.0.0.0:3000".parse().expect("valid address"),
        cookie_secure: true,
    };
    let formatted = format!("{config:?}");
    assert!(
        !formatted.contains("sentinel-signing-key-do-not-print-me"),
        "jwt_secret must never appear in Debug output, got {formatted:?}"
    );
    assert!(
        !formatted.contains("hunter2-sentinel"),
        "the DSN password must never appear in Debug output, got {formatted:?}"
    );
    assert!(
        !formatted.contains("produser"),
        "the DSN username must never appear in Debug output, got {formatted:?}"
    );
}

/// `RFC 017` §4's five cases, each over an explicit `TempDir` rather
/// than the real `data/` -- what makes this testable at all without a
/// real filesystem fixture (`FIV-001` §2's own instruction).
#[test]
fn resolve_generated_jwt_secret_generates_and_persists_a_key_when_absent() {
    let dir = tempfile::TempDir::new().expect("temp dir");
    let database_url = format!("sqlite://{}/app.db", dir.path().display());
    let key_path = dir.path().join("jwt_secret");
    assert!(!key_path.exists(), "key file must not pre-exist");

    let secret = resolve_generated_jwt_secret(&database_url).expect("generate a key");
    assert_eq!(secret.len(), JWT_SECRET_BYTES * 2, "hex-encoded 32 bytes");
    let on_disk = fs::read(&key_path).expect("key file now exists");
    assert_eq!(on_disk.len(), JWT_SECRET_BYTES);
    assert_eq!(hex_encode(&on_disk), secret);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&key_path).expect("stat").permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "key file must be owner-read-write-only");
    }

    // No leaked temp file from the atomic-write step.
    let leftovers: Vec<_> = fs::read_dir(dir.path())
        .expect("read dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.contains(".tmp-"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "no temp file should remain: {leftovers:?}"
    );
}

#[test]
fn resolve_generated_jwt_secret_reuses_a_well_formed_existing_file() {
    let dir = tempfile::TempDir::new().expect("temp dir");
    let database_url = format!("sqlite://{}/app.db", dir.path().display());

    let first = resolve_generated_jwt_secret(&database_url).expect("generate once");
    let second = resolve_generated_jwt_secret(&database_url).expect("read it back");
    assert_eq!(
        first, second,
        "a second resolution must read the same key, not regenerate"
    );
}

#[test]
fn resolve_generated_jwt_secret_fails_loudly_on_a_malformed_file_and_does_not_regenerate() {
    let dir = tempfile::TempDir::new().expect("temp dir");
    let database_url = format!("sqlite://{}/app.db", dir.path().display());
    let key_path = dir.path().join("jwt_secret");
    fs::write(&key_path, b"abc").expect("write a damaged key file");

    let result = resolve_generated_jwt_secret(&database_url);
    match result {
        Err(ConfigError::KeyMalformed {
            expected, found, ..
        }) => {
            assert_eq!(expected, JWT_SECRET_BYTES);
            assert_eq!(found, 3);
        }
        other => panic!("expected KeyMalformed, got {other:?}"),
    }
    assert_eq!(
        fs::read(&key_path).expect("still readable"),
        b"abc",
        "a malformed key file must be left exactly as it was, never regenerated over"
    );
}

#[test]
fn resolve_generated_jwt_secret_fails_when_the_database_url_has_no_filesystem_path() {
    for url in ["sqlite::memory:", "sqlite://:memory:"] {
        match resolve_generated_jwt_secret(url) {
            Err(ConfigError::NoFilesystemPathForKey { database_url }) => {
                assert_eq!(database_url, url);
            }
            other => panic!("expected NoFilesystemPathForKey for {url:?}, got {other:?}"),
        }
    }
}

#[test]
#[cfg(unix)]
fn resolve_generated_jwt_secret_fails_when_the_key_cannot_be_written() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().expect("temp dir");
    // The directory exists and is readable (so `fs::read` of the
    // not-yet-created key file correctly returns `NotFound`, reaching
    // the generate-and-write path), but is not writable -- the write
    // itself must fail, not silently land the key somewhere else.
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o500)).expect("chmod r-x");
    let database_url = format!("sqlite://{}/app.db", dir.path().display());

    let result = resolve_generated_jwt_secret(&database_url);
    // Restore write access before any assertion can panic and skip it,
    // so the TempDir can still delete itself on drop.
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).expect("chmod rwx");

    match result {
        Err(ConfigError::KeyWriteFailed { path, .. }) => {
            assert_eq!(path, dir.path().join(JWT_SECRET_FILE_NAME));
        }
        other => panic!("expected KeyWriteFailed, got {other:?}"),
    }
}

#[test]
fn jwt_secret_path_is_beside_the_database_file_not_a_fixed_data_dir() {
    assert_eq!(
        jwt_secret_path(Path::new("/srv/peisear/app.db")),
        PathBuf::from("/srv/peisear/jwt_secret"),
        "the key must follow DATABASE_URL's own directory, not a hardcoded data/"
    );
    assert_eq!(
        jwt_secret_path(Path::new("app.db")),
        PathBuf::from("jwt_secret"),
        "a database path with no directory component resolves beside it in the cwd"
    );
}

#[test]
fn database_file_path_has_no_filesystem_path_for_in_memory_urls() {
    assert_eq!(database_file_path("sqlite::memory:"), None);
    assert_eq!(database_file_path("sqlite://:memory:"), None);
    assert_eq!(
        database_file_path("sqlite://data/app.db"),
        Some(PathBuf::from("data/app.db"))
    );
}

/// `FIV-001` round 2 `§9`: the published `.env.example` placeholder
/// must be refused, not used, regardless of `DATABASE_URL` -- the
/// check happens before any key-file resolution is attempted.
#[test]
fn resolve_jwt_secret_refuses_the_published_placeholder() {
    match resolve_jwt_secret(Some(PUBLISHED_PLACEHOLDER), "sqlite://data/app.db") {
        Err(ConfigError::PublishedPlaceholder) => {}
        other => panic!("expected PublishedPlaceholder, got {other:?}"),
    }
    // Even with no filesystem path to resolve a key against -- the
    // placeholder check must come first, not be masked by a different
    // failure that happens to also apply.
    match resolve_jwt_secret(Some(PUBLISHED_PLACEHOLDER), "sqlite::memory:") {
        Err(ConfigError::PublishedPlaceholder) => {}
        other => panic!("expected PublishedPlaceholder, got {other:?}"),
    }
}

#[test]
fn resolve_jwt_secret_uses_a_real_environment_value_unchanged() {
    let secret = resolve_jwt_secret(
        Some("a-real-operator-chosen-secret"),
        "sqlite://data/app.db",
    )
    .expect("a real value must be accepted");
    assert_eq!(secret, "a-real-operator-chosen-secret");
}

#[test]
fn resolve_jwt_secret_generates_when_the_environment_is_unset() {
    let dir = tempfile::TempDir::new().expect("temp dir");
    let database_url = format!("sqlite://{}/app.db", dir.path().display());
    let secret = resolve_jwt_secret(None, &database_url).expect("generate a key");
    assert_eq!(secret.len(), JWT_SECRET_BYTES * 2);
    assert!(dir.path().join(JWT_SECRET_FILE_NAME).exists());
}

/// `FIV-001` round 3 `§13`: an empty value is not a secret, and must
/// be refused rather than used verbatim as an HMAC key.
#[test]
fn resolve_jwt_secret_refuses_an_empty_value() {
    match resolve_jwt_secret(Some(""), "sqlite://data/app.db") {
        Err(ConfigError::EmptySecret) => {}
        other => panic!("expected EmptySecret for an empty value, got {other:?}"),
    }
}

/// `§13`'s own distinction from `§9`'s placeholder check: whitespace
/// alone is not `str::is_empty`, so this needs its own case, not reuse
/// of the empty-string one.
#[test]
fn resolve_jwt_secret_refuses_a_whitespace_only_value() {
    match resolve_jwt_secret(Some("   \t  "), "sqlite://data/app.db") {
        Err(ConfigError::EmptySecret) => {}
        other => panic!("expected EmptySecret for a whitespace-only value, got {other:?}"),
    }
}

/// `FIV-001` round 3 `§14`: the exact bypass the review demonstrated
/// against round 2 -- a trailing space must not let the placeholder
/// through as if it were a different, real value.
#[test]
fn resolve_jwt_secret_refuses_the_placeholder_with_trailing_whitespace() {
    let padded = format!("{PUBLISHED_PLACEHOLDER} ");
    match resolve_jwt_secret(Some(&padded), "sqlite://data/app.db") {
        Err(ConfigError::PublishedPlaceholder) => {}
        other => panic!("expected PublishedPlaceholder for the padded value, got {other:?}"),
    }
}

/// Same shape, leading whitespace, so the fix is confirmed to be a
/// trim and not a `trim_end`/`strip_suffix` special case.
#[test]
fn resolve_jwt_secret_refuses_the_placeholder_with_leading_whitespace() {
    let padded = format!(" {PUBLISHED_PLACEHOLDER}");
    match resolve_jwt_secret(Some(&padded), "sqlite://data/app.db") {
        Err(ConfigError::PublishedPlaceholder) => {}
        other => panic!("expected PublishedPlaceholder for the padded value, got {other:?}"),
    }
}

/// A real value with incidental surrounding whitespace (a trailing
/// newline from an editor, say) is still accepted -- the trim is not
/// rejecting everything with whitespace near it, only checking against
/// it. The *stored* secret is the trimmed form, so what was checked is
/// what actually signs sessions.
#[test]
fn resolve_jwt_secret_trims_a_real_value_before_using_it() {
    let secret = resolve_jwt_secret(
        Some("  a-real-secret-with-stray-whitespace  \n"),
        "sqlite://data/app.db",
    )
    .expect("a real, padded value must still be accepted");
    assert_eq!(secret, "a-real-secret-with-stray-whitespace");
}

// ---------- WIZ-005: resolve_bind_addr ----------

#[test]
fn resolve_bind_addr_accepts_a_numeric_address_and_port() {
    let addr = resolve_bind_addr("127.0.0.1:3000").expect("a valid address and port");
    assert_eq!(addr.to_string(), "127.0.0.1:3000");
}

/// The round's own `§1` measurement, all four answers: not typos,
/// but the four forms a person is most likely to type when shown
/// `[127.0.0.1:3000]` -- a hostname, a bare port, the publish value
/// with no port, and the shown address with no port.
#[test]
fn resolve_bind_addr_refuses_each_of_the_four_measured_answers() {
    for bad in ["localhost:3000", "3000", "0.0.0.0", "127.0.0.1"] {
        match resolve_bind_addr(bad) {
            Err(ConfigError::InvalidBindAddr { value }) => assert_eq!(value, bad),
            other => panic!("expected InvalidBindAddr for {bad:?}, got {other:?}"),
        }
    }
}

/// `§3`'s own gate: the error must name the variable and the value --
/// not the bare, unnamed `AddrParseError` display a `.parse()?`
/// upstream of this function would have surfaced instead.
#[test]
fn invalid_bind_addr_message_names_the_variable_and_the_value() {
    let message = ConfigError::InvalidBindAddr {
        value: "localhost:3000".to_string(),
    }
    .to_string();
    assert!(message.contains("BIND_ADDR"), "got {message:?}");
    assert!(message.contains("localhost:3000"), "got {message:?}");
}

/// `FIV-001` round 3 `§16`: `thiserror`'s `#[error(...)]` cannot
/// interpolate a `const`, so `PublishedPlaceholder`'s message and
/// `PUBLISHED_PLACEHOLDER` itself are two copies of the same literal
/// with nothing to keep them in sync. This test is that sync check --
/// if the constant ever changes, this fails until the message is
/// updated to match, which a compiler cannot enforce here.
#[test]
fn published_placeholder_message_names_the_actual_constant() {
    let message = ConfigError::PublishedPlaceholder.to_string();
    assert!(
        message.contains(PUBLISHED_PLACEHOLDER),
        "the error message must quote the exact literal it refuses, got {message:?}"
    );
}
