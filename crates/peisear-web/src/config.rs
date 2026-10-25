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

        let jwt_secret = resolve_jwt_secret(env::var("JWT_SECRET").ok().as_deref(), &database_url)?;

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

/// The literal `.env.example` ships (uncommented) for `JWT_SECRET` --
/// not a credential in source, since nothing uses it silently, but a
/// value a user can install by following the documented
/// `cp .env.example .env` path and never be told to replace. `FIV-001`
/// round 2 `§9`: refused rather than warned about, per the owner's own
/// ruling that the hypothetical cost (breaking an instance that was
/// running before an upgrade) does not apply while there is no
/// production use yet, and that a refusal cannot be learned around the
/// way a per-start warning can.
const PUBLISHED_PLACEHOLDER: &str = "change-me-to-a-long-random-string";

/// `RFC 017` §4. `from_env` fails with one of these rather than ever
/// falling back to a constant or silently regenerating over a file
/// that might only be damaged, not absent.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(
        "JWT_SECRET is set to an empty or whitespace-only value -- that is not a \
         secret, and every session would be signed with it exactly as given \
         (an empty HMAC key, if JWT_SECRET is empty). Unset JWT_SECRET entirely \
         (a real key will be generated for you and persisted beside the \
         database), or set it to a real secret of your own."
    )]
    EmptySecret,

    #[error(
        "JWT_SECRET is set to the published placeholder from .env.example \
         (\"change-me-to-a-long-random-string\") -- this exact value is public \
         in the repository and in every copy of the source, so anyone can forge \
         a session against an instance using it. Unset JWT_SECRET (a real key \
         will be generated for you and persisted beside the database), or set \
         it to a real secret of your own."
    )]
    PublishedPlaceholder,

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

/// The full `JWT_SECRET` resolution: refuses an empty or
/// whitespace-only value (`FIV-001` round 3 `§13`) or the published
/// placeholder (round 2 `§9`) if the environment carries one; uses the
/// environment variable otherwise, if set at all (no migration for
/// anyone who already configures a real one); resolves the generated
/// key file otherwise. Takes the already-looked-up environment value
/// as a parameter, deliberately, rather than calling `env::var` itself
/// — the only way `Config::from_env`'s own cases stay testable without
/// a test reaching into the process's real, shared environment.
///
/// **Both checks compare `v.trim()`, not `v`** (round 3 `§14`): a
/// trailing space makes `v == PUBLISHED_PLACEHOLDER` false by exact
/// inequality, and a whitespace-only value is not "empty" by `str::
/// is_empty` alone. **This is a trim, not the start of normalisation**
/// — no case-folding, no quote-stripping, no Unicode handling. If the
/// accepted value differs from `v` because of this trim, the trimmed
/// form is what is actually used as the key, so there is no gap
/// between what was checked and what signs sessions.
fn resolve_jwt_secret(env_value: Option<&str>, database_url: &str) -> Result<String, ConfigError> {
    match env_value {
        Some(v) => {
            let trimmed = v.trim();
            // A predicate ("is this a usable secret at all"), not a
            // blocklist ("is this one of the bad values we know
            // about") -- `is_empty` holds for every value in its
            // class, not a finite set someone could extend later.
            if trimmed.is_empty() {
                return Err(ConfigError::EmptySecret);
            }
            // A blocklist of exactly one literal, deliberately (round
            // 2 `§9`'s own "not a list" rule): the one value that has
            // ever actually been published. A second literal would be
            // a different design, not an extension of this one.
            if trimmed == PUBLISHED_PLACEHOLDER {
                return Err(ConfigError::PublishedPlaceholder);
            }
            Ok(trimmed.to_string())
        }
        None => resolve_generated_jwt_secret(database_url),
    }
}

/// `database_url`'s own filesystem path, if it has one -- the same
/// `sqlite://` parsing `peisear_storage::pool::connect` uses, kept in
/// step with it rather than reimplemented differently. `None` for
/// `sqlite::memory:`, `sqlite://:memory:`, or any URL with no
/// filesystem path: there is nothing to put a key file beside.
///
/// `pub(crate)`, not `pub`: `wizard::run_wizard` reuses this and
/// [`jwt_secret_path`] to tell the operator where the key it is about
/// to generate will live, without a second implementation of the same
/// parsing to keep in step.
pub(crate) fn database_file_path(database_url: &str) -> Option<PathBuf> {
    let path = database_url.strip_prefix("sqlite://")?;
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(PathBuf::from(path))
}

/// The generated key's own path, beside `database_path`.
pub(crate) fn jwt_secret_path(database_path: &Path) -> PathBuf {
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
/// atomic), restricted *from the moment it is created* (`FIV-001`
/// round 2 `§11`: a create-then-`chmod` leaves a window where the temp
/// file holds the key at the default, more permissive mode — this
/// creates it already restricted, so there is no window at all), then
/// renamed into place. `RFC 017` §4's security review, item 1: two
/// processes starting concurrently (a restart loop, a crashed first
/// boot) must not be able to read a partially-written key.
///
/// **Noted, not fixed**: two processes starting concurrently against
/// one empty data directory each generate their own key and write
/// their own temp file; whichever `rename` lands last wins, silently.
/// Nothing reads a partial file either way, which is the guarantee the
/// security review asked for — this is a weaker, separate property
/// (last-writer-wins, not corruption) recorded here so a future reader
/// does not assume this function also serializes concurrent *writers*.
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
        create_restricted_file(&tmp_path, bytes)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    })();
    write_result.map_err(|source| {
        let _ = fs::remove_file(&tmp_path);
        to_error(source)
    })
}

/// Create `path` and write `bytes` to it, restricted to owner
/// read/write **from creation**, not permissioned afterward. `mode
/// 0600` plus `create_new` on Unix: the latter also turns a leftover
/// temp file from an earlier, pid-wrapped run into an error rather
/// than a silent overwrite of it. **POSIX-only** (`RFC 017` §4's
/// security review, item 3): Windows has no equivalent of Unix
/// permission bits, and `std::fs::Permissions` on Windows only ever
/// exposes a read-only flag, so there is no bit pattern this function
/// could set that would mean the same thing there. A plain create on
/// that platform is a documented difference, not a silent gap --
/// `docs/security/hardening.md` says so.
#[cfg(unix)]
fn create_restricted_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(bytes)
}

/// `create_new` needs no platform-specific API -- only the mode bit
/// does (`FIV-001` round 3 `§16`: the first version of this function
/// used a plain `fs::write` here, which silently dropped the
/// leftover-temp-file protection on this platform along with the
/// permission bits, when only the latter actually needed to differ).
#[cfg(not(unix))]
fn create_restricted_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(bytes)
}

/// Lower-case hex, no separator. No `hex` crate dependency for an
/// eight-line function.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests;
