//! SMTP configuration, sourced from environment.
//!
//! Per Q3 of the 0.16.0 design: SMTP is operator territory,
//! not user territory. Credentials live in environment
//! variables read at startup; there is no in-app form for them.
//! This is the same shape as the existing `JWT_SECRET` /
//! `DATABASE_URL` / `BIND_ADDR` pattern.
//!
//! ## Variables
//!
//! | Variable | Required | Notes |
//! |---|---|---|
//! | `SMTP_HOST` | for email channel | e.g. `smtp.example.com` |
//! | `SMTP_PORT` | optional | default 465 (implicit TLS) |
//! | `SMTP_TLS_MODE` | optional | `implicit` or `starttls`; auto from port if unset |
//! | `SMTP_USER` | for email channel | SMTP AUTH username |
//! | `SMTP_PASSWORD` | for email channel | SMTP AUTH password |
//! | `SMTP_FROM_ADDRESS` | for email channel | `From:` envelope address |
//! | `SMTP_FROM_NAME` | optional | display name for `From:` header |
//!
//! Both implicit TLS (port 465) and STARTTLS (port 587) are
//! supported by `wasm-smtp` 0.9 and the `wasm-smtp-tokio`
//! adapter. We pick the mode either by `SMTP_TLS_MODE` (when
//! set) or, when unset, by port number: 465 → implicit, 587 →
//! STARTTLS, anything else → implicit (the modern default per
//! upstream's recommendation).
//!
//! ## Behaviour when unconfigured
//!
//! Per Q4 of the design: graceful failure at send time, not at
//! startup. [`SmtpConfig::from_env`] returns `Ok(None)` if any
//! required variable is missing; the caller logs a warning at
//! startup and continues. Subsequent send attempts fail at the
//! channel layer (logged), the audit row records `dispatched_via`
//! without `email`, and the in-app channel still works.
//!
//! Rationale: peisear should remain useful in deployments that
//! deliberately don't configure email (single-user instances,
//! evaluation environments). A startup failure would punish
//! them for a non-essential capability.
//!
//! **`SMTP_PORT`/`SMTP_TLS_MODE` are a different case** (`CFG-002`,
//! `RFC 022` §3): absence still degrades exactly as above, but a
//! value the operator *did* set that this crate cannot use now
//! refuses startup via [`SmtpConfigError`] rather than being
//! silently discarded — the malformed-port-silently-changes-
//! transport-security cascade `§10.51` named is what this closes.

use std::env;

/// TLS connection mode for an SMTP submission endpoint.
///
/// Auto-derived from `SMTP_PORT` if `SMTP_TLS_MODE` is not set:
/// 465 → `Implicit`, 587 → `Starttls`, anything else →
/// `Implicit` (modern default per upstream guidance).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsMode {
    /// Implicit TLS — TLS handshake before any SMTP traffic.
    /// The standard "submissions" port is 465.
    Implicit,
    /// STARTTLS — connect plaintext, run EHLO, upgrade in
    /// place. The standard "submission" port is 587.
    Starttls,
}

/// Every spelling `from_str_loose` accepts, paired with the mode it
/// resolves to. **The one source both `from_str_loose` and
/// [`TlsMode::accepted_spellings`] read** (`CFG-002`, `RFC 022` §3):
/// an error message naming a tidier set than the parser actually
/// implements would be its own, smaller version of the defect this
/// round exists to end — a value the operator typed that the
/// message's own "accepted" list did not warn them away from.
const TLS_MODE_SPELLINGS: &[(&str, TlsMode)] = &[
    ("implicit", TlsMode::Implicit),
    ("implicit_tls", TlsMode::Implicit),
    ("smtps", TlsMode::Implicit),
    ("tls", TlsMode::Implicit),
    ("starttls", TlsMode::Starttls),
    ("start_tls", TlsMode::Starttls),
    ("submission", TlsMode::Starttls),
];

impl TlsMode {
    fn from_str_loose(s: &str) -> Option<Self> {
        let lower = s.trim().to_ascii_lowercase();
        TLS_MODE_SPELLINGS
            .iter()
            .find(|(spelling, _)| *spelling == lower)
            .map(|(_, mode)| *mode)
    }

    /// Every spelling `from_str_loose` accepts, comma-joined, for
    /// `SmtpConfigError::InvalidTlsMode`'s own message. Deliberately
    /// the full alias set, not `configuration.md`'s own two canonical
    /// names (a past review's own call: naming all seven there would
    /// invite treating the alias set as a contract) — a refusal
    /// message is telling the operator what will actually be
    /// accepted *right now*, not documenting a stable surface.
    fn accepted_spellings() -> String {
        TLS_MODE_SPELLINGS
            .iter()
            .map(|(spelling, _)| *spelling)
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Default mode from a port number, used when SMTP_TLS_MODE
    /// is not set explicitly.
    pub fn default_for_port(port: u16) -> Self {
        match port {
            587 => Self::Starttls,
            // Including 465 and any non-standard port.
            // Implicit TLS is the modern default; if an operator
            // is using a non-standard port and STARTTLS, they
            // should set SMTP_TLS_MODE explicitly.
            _ => Self::Implicit,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Implicit => "implicit",
            Self::Starttls => "starttls",
        }
    }
}

/// `CFG-002`, `RFC 022` §3: `peisear-notify` had no error channel at
/// all before this round — [`SmtpConfig::from_env`]'s old `Option<Self>`
/// could not express *"a value was given and it is wrong"*, only
/// *"not configured"*. `Ok(None)` keeps meaning the first fact;
/// `Err(_)` is the second, new one. Collapsing both into one type
/// would re-merge the distinction `RFC 022` exists to draw.
#[derive(Debug, thiserror::Error)]
pub enum SmtpConfigError {
    #[error(
        "SMTP_PORT is set to {value:?}, which is not a valid port number -- expected an \
         integer from 0 to 65535. Unset SMTP_PORT to use the default (465), or set it to a \
         valid port."
    )]
    InvalidPort { value: String },

    #[error(
        "SMTP_TLS_MODE is set to {value:?}, which is not one of the accepted modes -- \
         expected one of: {accepted}. Unset SMTP_TLS_MODE to derive it from SMTP_PORT, or \
         set it to one of the accepted modes."
    )]
    InvalidTlsMode { value: String, accepted: String },
}

/// `SMTP_PORT`'s own resolution (`CFG-002`): absence still degrades
/// to `465`, unchanged. A present value that does not parse as a
/// `u16` -- including an explicitly empty string, which is not one of
/// the five absence-shaped `SMTP_*` variables (`RFC 022` §7) -- now
/// refuses by name rather than being silently discarded by the
/// `.ok()` this used to sit behind. Takes the already-looked-up value
/// as a parameter, the same reason `peisear-web`'s own resolvers do.
fn resolve_smtp_port(raw: Option<&str>) -> Result<u16, SmtpConfigError> {
    match raw {
        None => Ok(465),
        Some(v) => v.parse().map_err(|_| SmtpConfigError::InvalidPort {
            value: v.to_string(),
        }),
    }
}

/// `SMTP_TLS_MODE`'s own resolution (`CFG-002`): absence still
/// derives from `port`, unchanged. A present value `from_str_loose`
/// does not recognise now refuses, listing every spelling the parser
/// actually accepts (`TlsMode::accepted_spellings`) rather than a
/// tidier set it does not implement.
fn resolve_tls_mode(raw: Option<&str>, port: u16) -> Result<TlsMode, SmtpConfigError> {
    match raw {
        None => Ok(TlsMode::default_for_port(port)),
        Some(v) => TlsMode::from_str_loose(v).ok_or_else(|| SmtpConfigError::InvalidTlsMode {
            value: v.to_string(),
            accepted: TlsMode::accepted_spellings(),
        }),
    }
}

/// SMTP transport configuration. `Ok(None)` if any required env var
/// is missing; the caller treats `None` as "email channel
/// unavailable" and logs accordingly. `Err(_)` if `SMTP_PORT` or
/// `SMTP_TLS_MODE` was given a value this crate cannot use.
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub tls_mode: TlsMode,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: Option<String>,
}

impl SmtpConfig {
    /// Read from environment. `Ok(None)` if any required variable is
    /// missing -- checked first and still unconditional, so an
    /// unconfigured channel never reaches `SMTP_PORT`/`SMTP_TLS_MODE`
    /// and never fails a boot over them (`RFC 022` §3, `CFG-002`).
    /// `Err(_)` if `SMTP_PORT` or `SMTP_TLS_MODE` was set to a value
    /// this crate cannot use.
    ///
    /// Every `env::var` call stays here, deliberately -- the only
    /// place in this module that reaches into the process's real,
    /// shared environment. `from_values` (`pub(crate)`, not linked --
    /// a public doc comment may not link to a private item) is the
    /// actual resolution, pure, so a test can exercise the whole
    /// channel's absence/refusal composition without mutating it
    /// (`CFG-002` round 2).
    pub fn from_env() -> Result<Option<Self>, SmtpConfigError> {
        Self::from_values(
            env::var("SMTP_HOST").ok().as_deref(),
            env::var("SMTP_USER").ok().as_deref(),
            env::var("SMTP_PASSWORD").ok().as_deref(),
            env::var("SMTP_FROM_ADDRESS").ok().as_deref(),
            env::var("SMTP_PORT").ok().as_deref(),
            env::var("SMTP_TLS_MODE").ok().as_deref(),
            env::var("SMTP_FROM_NAME").ok().as_deref(),
        )
    }

    /// [`Self::from_env`]'s body, extracted rather than rewritten
    /// (`CFG-002` round 2): every early `Ok(None)` return, the
    /// `.filter(|s| !s.is_empty())` absence treatment, the two
    /// resolver calls and their order relative to the four early
    /// returns, and the struct assembly are unchanged from round 1 --
    /// only the seven `env::var` calls moved to the one caller.
    ///
    /// **The four early returns run before either resolver, which is
    /// a deliberate property, not an accident of ordering**: a wholly
    /// unconfigured channel with an additionally malformed
    /// `SMTP_PORT` still yields `Ok(None)` and never reaches the port
    /// resolver at all (`RFC 022` §3's first half — absence degrades
    /// unconditionally, regardless of what else happens to be set to
    /// nonsense nobody will ever read).
    ///
    /// `pub(crate)`, not `pub`: only this module's own tests need to
    /// call it directly.
    pub(crate) fn from_values(
        host: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
        from_address: Option<&str>,
        port_raw: Option<&str>,
        tls_mode_raw: Option<&str>,
        from_name: Option<&str>,
    ) -> Result<Option<Self>, SmtpConfigError> {
        let Some(host) = host.filter(|s| !s.is_empty()) else {
            return Ok(None);
        };
        let Some(username) = username.filter(|s| !s.is_empty()) else {
            return Ok(None);
        };
        let Some(password) = password.filter(|s| !s.is_empty()) else {
            return Ok(None);
        };
        let Some(from_address) = from_address.filter(|s| !s.is_empty()) else {
            return Ok(None);
        };
        let port = resolve_smtp_port(port_raw)?;
        let tls_mode = resolve_tls_mode(tls_mode_raw, port)?;
        let from_name = from_name.filter(|s| !s.is_empty());

        Ok(Some(SmtpConfig {
            host: host.to_string(),
            port,
            tls_mode,
            username: username.to_string(),
            password: password.to_string(),
            from_address: from_address.to_string(),
            from_name: from_name.map(str::to_string),
        }))
    }

    /// Log the configuration status at startup. If `None`, log
    /// a warning so an operator notices in dev/staging.
    pub fn log_startup(config: Option<&SmtpConfig>) {
        match config {
            Some(c) => {
                tracing::info!(
                    host = %c.host,
                    port = c.port,
                    tls_mode = c.tls_mode.as_str(),
                    from = %c.from_address,
                    "SMTP configured; email channel ready"
                );
            }
            None => {
                // `SUP-004`: names no variable, rather than four of
                // the seven as though they were the set. The warning
                // used to name `SMTP_HOST`/`SMTP_USER`/`SMTP_PASSWORD`/
                // `SMTP_FROM_ADDRESS` -- the four [`from_env`] checks
                // directly -- and an operator who set exactly those
                // four and found the channel still wrong had been
                // taught the subset by the one message this crate
                // volunteers. Pointing at the docs instead costs
                // nothing when the set changes; a name list here
                // would need to stay in sync with `from_env` by hand,
                // the same `§10.36`-shaped drift this project already
                // avoids in its own checks.
                tracing::warn!(
                    "SMTP not configured -- see Configuration's own \"Email (SMTP)\" \
                     section for all seven variables. The email channel will fail \
                     at send time; in-app delivery continues to work."
                );
            }
        }
    }
}

#[cfg(test)]
mod tests;
