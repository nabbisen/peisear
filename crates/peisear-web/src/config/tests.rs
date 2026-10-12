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
