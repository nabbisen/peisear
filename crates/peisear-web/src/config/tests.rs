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
        bind_addr: "0.0.0.0:3000".to_string(),
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
