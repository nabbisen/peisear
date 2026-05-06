//! Email channel — `wasm-smtp` 0.9 + `wasm-smtp-tokio` adapter.
//!
//! ## Design (Phase 1, peisear 0.16.0)
//!
//! - Plain text only. HTML / multipart is Phase 2 if real
//!   demand appears; the 80% case for notification email is
//!   one paragraph of UTF-8.
//! - One-shot connection per send. No pooling. Notification
//!   volume is small; pooling complexity isn't earned yet.
//! - Both implicit TLS (port 465) and STARTTLS (port 587) are
//!   supported via [`super::config::TlsMode`]. Both flavours
//!   are first-class in the `wasm-smtp` family — implicit TLS
//!   uses [`TokioTlsTransport::connect_implicit_tls`],
//!   STARTTLS uses [`TokioPlainTransport::connect`] +
//!   [`SmtpClient::connect_starttls`].
//! - Composition uses [`MessageBuilder`] passed straight to
//!   [`SmtpClient::send_message`]. The `mail-builder` cargo
//!   feature on `wasm-smtp-tokio` enables this convenience.
//! - Authentication uses [`SmtpClient::login`], which
//!   auto-selects the strongest mechanism the server
//!   advertised: SCRAM-SHA-256 > PLAIN > LOGIN as of
//!   `wasm-smtp` 0.9.0.
//!
//! ## Why so little code lives here
//!
//! Per Q3 of the 0.16.0 design ("project management is the
//! business, not email"), the SMTP-related surface area in
//! peisear is deliberately tiny. The wasm-smtp family takes
//! care of:
//!
//! - Transport plumbing (TCP, TLS handshake, SNI, root certs)
//!   → `wasm-smtp-tokio`.
//! - SMTP state machine, parsing, command formatting,
//!   dot-stuffing, error classification → `wasm-smtp`.
//! - RFC 5322 / MIME composition (line folding, encoded-word,
//!   header injection defenses) → `mail-builder`.
//! - SCRAM-SHA-256 / PLAIN / LOGIN authentication → `wasm-smtp`.
//!
//! What's left for us is: mapping our config to the right
//! transport, calling four methods, and wrapping the error.

use mail_builder::MessageBuilder;
use wasm_smtp::SmtpClient;
use wasm_smtp_tokio::{TokioPlainTransport, TokioTlsTransport};

use super::config::{SmtpConfig, TlsMode};

/// Email-channel failure. Wraps `wasm-smtp`'s top-level error
/// so caller-side error formatters (anyhow's `{:#}`, eyre,
/// manual `.source()` walks) see the full diagnostic chain
/// (e.g. `EmailError → SmtpError::Io → io::Error`).
#[derive(thiserror::Error, Debug)]
pub enum EmailError {
    #[error("SMTP error: {0}")]
    Smtp(#[from] wasm_smtp::SmtpError),
}

// `MessageBuilder::write_to_string` may surface std::io::Error;
// we don't currently call that directly (we hand the builder
// to `send_message`), but if a future refactor switches to the
// manual path the From impl below will be useful.
impl From<std::io::Error> for EmailError {
    fn from(err: std::io::Error) -> Self {
        Self::Smtp(wasm_smtp::SmtpError::Io(wasm_smtp::IoError::with_source(
            "message composition I/O failure",
            err,
        )))
    }
}

/// EHLO identifier we send. RFC 5321 §4.1.1.1 expects an FQDN;
/// most submission servers treat it as advisory (they verify
/// the actual connection, not this string). A stable
/// identifier across deployments is fine.
const EHLO_IDENTIFIER: &str = "peisear.local";

/// Send one notification email.
///
/// `to` is a single recipient — notifications are per-user, so
/// no batching. `subject` is plain UTF-8 (RFC 2047
/// encoded-word is handled by `mail-builder` for non-ASCII).
/// `body` is plain UTF-8 with `\n` line endings; `mail-builder`
/// CRLF-normalizes for the wire.
///
/// Returns `Ok(())` on a 250-acknowledged delivery. Errors:
/// transport setup, TLS handshake, AUTH, SMTP envelope
/// rejection, post-DATA `.` rejection — all are surfaced as
/// [`EmailError::Smtp`] with the wasm-smtp variant preserved.
///
/// Retry policy is the caller's concern. We don't retry here.
pub async fn send_email(
    cfg: &SmtpConfig,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), EmailError> {
    // Compose. We hand the builder straight to send_message;
    // wasm-smtp owns serialization.
    let message = match cfg.from_name.as_deref() {
        Some(name) => MessageBuilder::new()
            .from((name, cfg.from_address.as_str()))
            .to(to)
            .subject(subject)
            .text_body(body),
        None => MessageBuilder::new()
            .from(cfg.from_address.as_str())
            .to(to)
            .subject(subject)
            .text_body(body),
    };

    // Pick transport and run the full session: connect, login,
    // send, quit. Different match arms for implicit TLS vs
    // STARTTLS because the entry-point signatures differ — but
    // the inner sequence (login/send/quit) is identical.
    match cfg.tls_mode {
        TlsMode::Implicit => {
            let transport = TokioTlsTransport::connect_implicit_tls(
                cfg.host.as_str(),
                cfg.port,
                cfg.host.as_str(), // SNI / cert hostname
            )
            .await
            .map_err(wasm_smtp::SmtpError::Io)?;

            let mut client = SmtpClient::connect(transport, EHLO_IDENTIFIER).await?;
            client.login(&cfg.username, &cfg.password).await?;
            client
                .send_message(&cfg.from_address, &[to], message)
                .await?;
            client.quit().await?;
            Ok(())
        }
        TlsMode::Starttls => {
            let transport = TokioPlainTransport::connect(
                cfg.host.as_str(),
                cfg.port,
                cfg.host.as_str(), // cert hostname for the eventual upgrade
            )
            .await
            .map_err(wasm_smtp::SmtpError::Io)?;

            let mut client = SmtpClient::connect_starttls(transport, EHLO_IDENTIFIER).await?;
            client.login(&cfg.username, &cfg.password).await?;
            client
                .send_message(&cfg.from_address, &[to], message)
                .await?;
            client.quit().await?;
            Ok(())
        }
    }
}
