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
//! startup. [`SmtpConfig::from_env`] returns `None` if any
//! required variable is missing; the caller logs a warning at
//! startup and continues. Subsequent send attempts fail at the
//! channel layer (logged), the audit row records `dispatched_via`
//! without `email`, and the in-app channel still works.
//!
//! Rationale: peisear should remain useful in deployments that
//! deliberately don't configure email (single-user instances,
//! evaluation environments). A startup failure would punish
//! them for a non-essential capability.

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

impl TlsMode {
    fn from_str_loose(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "implicit" | "implicit_tls" | "smtps" | "tls" => Some(Self::Implicit),
            "starttls" | "start_tls" | "submission" => Some(Self::Starttls),
            _ => None,
        }
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

/// SMTP transport configuration. `None` if any required env var
/// is missing; the caller treats `None` as "email channel
/// unavailable" and logs accordingly.
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
    /// Read from environment. Returns `None` if any required
    /// variable is missing.
    pub fn from_env() -> Option<Self> {
        let host = env::var("SMTP_HOST").ok().filter(|s| !s.is_empty())?;
        let username = env::var("SMTP_USER").ok().filter(|s| !s.is_empty())?;
        let password = env::var("SMTP_PASSWORD").ok().filter(|s| !s.is_empty())?;
        let from_address = env::var("SMTP_FROM_ADDRESS")
            .ok()
            .filter(|s| !s.is_empty())?;
        let port = env::var("SMTP_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(465);
        let tls_mode = env::var("SMTP_TLS_MODE")
            .ok()
            .and_then(|s| TlsMode::from_str_loose(&s))
            .unwrap_or_else(|| TlsMode::default_for_port(port));
        let from_name = env::var("SMTP_FROM_NAME").ok().filter(|s| !s.is_empty());

        Some(SmtpConfig {
            host,
            port,
            tls_mode,
            username,
            password,
            from_address,
            from_name,
        })
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
                tracing::warn!(
                    "SMTP not configured (set SMTP_HOST, SMTP_USER, SMTP_PASSWORD, \
                     SMTP_FROM_ADDRESS to enable the email channel). The email \
                     channel will fail at send time; in-app delivery continues \
                     to work."
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
}
