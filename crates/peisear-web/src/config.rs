//! Runtime configuration pulled from environment variables.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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
    /// Fails loudly (`RFC 017` §4) rather than falling back to a
    /// constant: a `JWT_SECRET` that cannot be resolved is a reason to
    /// refuse to start, not a reason to sign sessions with something
    /// every copy of the source already knows.
    pub fn from_env() -> Result<Self, ConfigError> {
        // Best-effort load of `.env`; absence is not fatal.
        let _ = dotenvy::dotenv();

        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/app.db".to_string());

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(v) => v,
            Err(_) => resolve_generated_jwt_secret(&database_url)?,
        };

        // `RFC 017` §5: loopback by default. A first run that skips
        // configuration entirely must not also publish the instance to
        // the local network -- the container image sets this explicitly
        // to `0.0.0.0:3000`, where an open bind is correct and expected.
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

        let cookie_secure = env::var("COOKIE_SECURE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Ok(Self {
            database_url,
            jwt_secret,
            bind_addr,
            cookie_secure,
        })
    }

    /// `SEC-001` §1: a form of [`Self::database_url`] safe to log —
    /// scheme, host and path; userinfo dropped entirely, along with any
    /// query string or fragment (a DSN's own business, never a startup
    /// line's). Today's `sqlite://data/app.db` has no userinfo and
    /// round-trips unchanged; the day a `postgres://user:password@host/db`
    /// DSN exists (`RFC 018` P3), this is what keeps the password out of
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

/// `RFC 017` §4. `from_env` fails with one of these rather than ever
/// falling back to a constant or silently regenerating over a file
/// that might only be damaged, not absent.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(
        "JWT_SECRET is not set, and DATABASE_URL ({database_url:?}) has no filesystem path \
         to persist a generated signing key beside. Set JWT_SECRET explicitly."
    )]
    NoFilesystemPathForKey { database_url: String },

    #[error("failed to read the JWT signing key at {}: {source}", path.display())]
    KeyReadFailed { path: PathBuf, source: io::Error },

    #[error(
        "the JWT signing key at {} is malformed (expected {expected} bytes, found {found}) -- \
         refusing to regenerate over what may be a damaged file. Restore it from backup, or \
         delete it deliberately to generate a new one (this invalidates every existing session).",
        path.display()
    )]
    KeyMalformed {
        path: PathBuf,
        expected: usize,
        found: usize,
    },

    #[error("failed to write the generated JWT signing key to {}: {source}", path.display())]
    KeyWriteFailed { path: PathBuf, source: io::Error },
}

/// Bytes a generated key file must hold to be well-formed: 32 random
/// bytes from a CSPRNG (`RFC 017` §4), nothing else.
const JWT_SECRET_BYTES: usize = 32;

/// The generated key's filename, beside the database. `RFC 017` §4:
/// `data/jwt_secret` when the database is `data/app.db` -- named here
/// once rather than built from a fixed `data/`, so an operator who
/// points `DATABASE_URL` elsewhere gets the key inside the directory
/// they actually back up.
const JWT_SECRET_FILE_NAME: &str = "jwt_secret";

/// `database_url`'s own filesystem path, if it has one -- the same
/// `sqlite://` parsing `peisear_storage::pool::connect` uses, kept in
/// step with it rather than reimplemented differently. `None` for
/// `sqlite::memory:`, `sqlite://:memory:`, or any URL with no
/// filesystem path: there is nothing to put a key file beside.
fn database_file_path(database_url: &str) -> Option<PathBuf> {
    let path = database_url.strip_prefix("sqlite://")?;
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(PathBuf::from(path))
}

/// The generated key's own path, beside `database_path`.
fn jwt_secret_path(database_path: &Path) -> PathBuf {
    match database_path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.join(JWT_SECRET_FILE_NAME),
        _ => PathBuf::from(JWT_SECRET_FILE_NAME),
    }
}

/// `RFC 017` §4's five cases, in order: the file exists and is
/// well-formed (use it); it doesn't exist (generate, persist, use it);
/// it exists but is not well-formed (fail, do not regenerate); it
/// cannot be written (fail); `database_url` has no filesystem path to
/// put it beside (fail, naming that instead of a path). Returns the
/// key hex-encoded -- `peisear_auth::jwt` takes the secret as bytes via
/// `str::as_bytes`, and hex keeps arbitrary key bytes valid UTF-8
/// without the encoding ever needing to be reversed.
fn resolve_generated_jwt_secret(database_url: &str) -> Result<String, ConfigError> {
    let Some(db_path) = database_file_path(database_url) else {
        return Err(ConfigError::NoFilesystemPathForKey {
            database_url: database_url.to_string(),
        });
    };
    let key_path = jwt_secret_path(&db_path);

    match fs::read(&key_path) {
        Ok(bytes) if bytes.len() == JWT_SECRET_BYTES => Ok(hex_encode(&bytes)),
        Ok(bytes) => Err(ConfigError::KeyMalformed {
            path: key_path,
            expected: JWT_SECRET_BYTES,
            found: bytes.len(),
        }),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let bytes = peisear_auth::jwt::generate_signing_key();
            write_key_file_atomically(&key_path, &bytes)?;
            Ok(hex_encode(&bytes))
        }
        Err(source) => Err(ConfigError::KeyReadFailed {
            path: key_path,
            source,
        }),
    }
}

/// Write `bytes` to `path` atomically: a temp file in `path`'s own
/// directory (so the rename stays on one filesystem and is therefore
/// atomic), permissioned *before* the rename so the file visible at
/// `path` is never briefly world-readable, then renamed into place.
/// `RFC 017` §4's security review, item 1: two processes starting
/// concurrently (a restart loop, a crashed first boot) must not be
/// able to read a partially-written key.
fn write_key_file_atomically(
    path: &Path,
    bytes: &[u8; JWT_SECRET_BYTES],
) -> Result<(), ConfigError> {
    let to_error = |source: io::Error| ConfigError::KeyWriteFailed {
        path: path.to_path_buf(),
        source,
    };
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(to_error)?;
    }
    let tmp_path = path.with_file_name(format!(
        "{}.tmp-{}",
        JWT_SECRET_FILE_NAME,
        std::process::id()
    ));
    let write_result: io::Result<()> = (|| {
        fs::write(&tmp_path, bytes)?;
        set_key_file_permissions(&tmp_path)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    })();
    write_result.map_err(|source| {
        let _ = fs::remove_file(&tmp_path);
        to_error(source)
    })
}

/// `mode 0600` on Unix -- owner read/write only. **POSIX-only**
/// (`RFC 017` §4's security review, item 3): Windows has no equivalent
/// of Unix permission bits, and `std::fs::Permissions` on Windows only
/// ever exposes a read-only flag, so there is no bit pattern this
/// function could set that would mean the same thing there. A no-op on
/// that platform is a documented difference, not a silent gap --
/// `docs/security/hardening.md` says so.
#[cfg(unix)]
fn set_key_file_permissions(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
fn set_key_file_permissions(_path: &Path) -> io::Result<()> {
    Ok(())
}

/// Lower-case hex, no separator. No `hex` crate dependency for an
/// eight-line function.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests;
