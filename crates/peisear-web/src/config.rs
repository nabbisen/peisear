//! Runtime configuration pulled from environment variables.

use std::env;

/// `SEC-001` §2: no `#[derive(Debug)]` here. `jwt_secret` is a signing
/// key; a derive would print it verbatim the first time anyone writes
/// `tracing::debug!(?config)` during an investigation. [`std::fmt::Debug`]
/// is implemented by hand below instead, redacting the one field that is
/// a credential.
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind_addr: String,
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> Self {
        // Best-effort load of `.env`; absence is not fatal.
        let _ = dotenvy::dotenv();

        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/app.db".to_string());

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::warn!(
                "JWT_SECRET is not set; falling back to a development-only default. \
                 DO NOT use this configuration in production."
            );
            "dev-only-insecure-change-me".to_string()
        });

        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        let cookie_secure = env::var("COOKIE_SECURE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Self {
            database_url,
            jwt_secret,
            bind_addr,
            cookie_secure,
        }
    }

    /// `SEC-001` §1: a form of [`Self::database_url`] safe to log —
    /// scheme, host and path; userinfo dropped entirely, along with any
    /// query string or fragment (a DSN's own business, never a startup
    /// line's). Today's `sqlite://data/app.db` has no userinfo and
    /// round-trips unchanged; the day a `postgres://user:password@host/db`
    /// DSN exists (`RFC 0018` P3), this is what keeps the password out of
    /// every log line that names the database.
    ///
    /// Redact at the source, not at each call site — `main.rs`'s startup
    /// line calls this rather than logging `database_url` directly, so a
    /// future second line cannot reintroduce the leak by forgetting.
    pub fn loggable_database_url(&self) -> String {
        redact_dsn(&self.database_url)
    }
}

/// Free function so [`Config::loggable_database_url`] and the `Debug`
/// impl below share one implementation.
fn redact_dsn(url: &str) -> String {
    let Some((scheme, rest)) = url.split_once("://") else {
        return "<unparseable>".to_string();
    };
    // Drop a query string or fragment first, so a `@` inside one could
    // never be mistaken for userinfo.
    let rest = rest.split(['?', '#']).next().unwrap_or(rest);
    let (authority_or_path, after_first_slash) = match rest.split_once('/') {
        Some((a, b)) => (a, Some(b)),
        None => (rest, None),
    };
    // Userinfo is everything up to the last `@` in the authority —
    // `rsplit_once` so a `@` that legitimately appears in a SQLite
    // filesystem path (after the first `/`, so not reached here) is
    // never at risk of being mistaken for one.
    let host = match authority_or_path.rsplit_once('@') {
        Some((_, host)) => host,
        None => authority_or_path,
    };
    match after_first_slash {
        Some(path) => format!("{scheme}://{host}/{path}"),
        None => format!("{scheme}://{host}"),
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &self.loggable_database_url())
            .field("jwt_secret", &"[redacted]")
            .field("bind_addr", &self.bind_addr)
            .field("cookie_secure", &self.cookie_secure)
            .finish()
    }
}

#[cfg(test)]
mod tests {
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
}
