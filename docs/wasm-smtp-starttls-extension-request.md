# wasm-smtp 拡張依頼: STARTTLS support

## Title

Add STARTTLS support for SMTP submission (port 587) connection model

## Background

wasm-smtp 0.6 currently scopes connection model to **implicit TLS** only (port 465). The crate documentation states:

> STARTTLS is intentionally out of scope for the initial release.

This is reasonable for an initial release, but in practice STARTTLS on port 587 is the dominant submission model for many SMTP relays:

- **Gmail / Google Workspace**: SMTP submission via `smtp.gmail.com:587` (STARTTLS)
- **Microsoft 365 / Outlook**: `smtp.office365.com:587` (STARTTLS)
- **AWS SES**: `email-smtp.<region>.amazonaws.com:587` (STARTTLS) is one of the supported endpoints
- **SendGrid**: `smtp.sendgrid.net:587` (STARTTLS)
- **Many corporate / self-hosted servers** default to 587 + STARTTLS

Operators of WASM / constrained-runtime applications using `wasm-smtp` who deploy against any of these relays cannot connect today, and would have to either:

1. Switch the relay account to port 465 if available (often not possible without admin approval)
2. Implement their own STARTTLS handshake parallel to `wasm-smtp` (defeats the point of using the crate)

## Issue / problem

Adding STARTTLS to `wasm-smtp` core is a **transport-level state transition**, not a MIME or message-level concern, so it falls within the existing design scope of "SMTP state machine, response parsing, command formatting, dot-stuffing, and error classification" that the crate already covers.

The challenge is that the TLS upgrade itself is the `Transport` implementation's responsibility (per existing docs):

> wasm-smtp deliberately knows nothing about TLS, certificates, or peer identity. The transport implementation is the entire trust boundary for the encrypted byte stream.

So the core can't directly call into TLS code. The proposal below addresses this.

## Proposed expected behavior

### High-level

Allow `SmtpClient::connect` (or a sibling `connect_starttls`) to:

1. Open the transport in **plain TCP** (no TLS yet).
2. Perform the SMTP greeting and `EHLO` exchange.
3. Verify the server advertises `STARTTLS` in its EHLO response.
4. Send the `STARTTLS` command (`STARTTLS\r\n`).
5. Read the server's `220` response.
6. Hand control back to the caller's `Transport` to **upgrade the existing byte stream to TLS** in place.
7. After TLS upgrade is complete, re-issue `EHLO` (per RFC 3207 §4.2) on the now-encrypted stream and continue the session as normal (login, send_mail, quit).

The handoff at step 6 is the critical design point. Two options:

#### Option A: extension trait on Transport

Introduce a `StartTlsCapable` trait that `Transport` implementations may also implement:

```rust
pub trait StartTlsCapable: Transport {
    /// Upgrade the underlying byte stream to TLS in place.
    /// After this returns Ok, all subsequent reads/writes go
    /// over the encrypted stream.
    async fn upgrade_tls(&mut self, hostname: &str) -> Result<(), IoError>;
}
```

`SmtpClient::connect_starttls<T: StartTlsCapable>(transport, host, hostname)` would call `upgrade_tls` at the appropriate point in the protocol flow. Transports that don't support TLS upgrade simply don't implement the trait.

The `wasm-smtp-cloudflare` adapter doesn't need to implement `StartTlsCapable` initially (Cloudflare Workers don't directly expose raw TCP sockets that can be upgraded mid-stream); the adapter could opt out, and `connect_starttls` becomes a compile-time error for Cloudflare callers — surfacing the limitation clearly.

#### Option B: connect-side callback

`SmtpClient::connect_starttls(transport, host, hostname, upgrade: impl FnOnce(&mut T, &str) -> Future<Result<(), IoError>>)` — pass the TLS upgrade closure at connect time. This avoids requiring a new trait but is awkward to use (callers carry around a closure that captures TLS config).

I prefer **Option A** for cleanness; the trait makes the capability explicit at the type level.

### Behavioral expectations

- **EHLO advertisement validation**: if the server's EHLO response does not include `STARTTLS`, `connect_starttls` must return a clear error (e.g. `SmtpError::Protocol(ProtocolError::StartTlsNotAdvertised)`). Silently downgrading to plain SMTP is a security regression and must not happen.
- **Post-upgrade EHLO re-issue**: per RFC 3207 §4.2, the client must re-issue `EHLO` after the TLS handshake. The new `EHLO` response replaces any prior server capability list (this is important — pre-TLS server greetings can be modified by an attacker, so they must not be trusted).
- **Graceful failure on upgrade error**: if `upgrade_tls` returns `Err`, the connection state must be considered poisoned. Subsequent reads/writes on that transport are not safe; `SmtpClient::connect_starttls` should propagate the error and not attempt recovery.
- **Hostname matching**: as with implicit-TLS connect, the SNI / hostname presented during the upgrade handshake must match the `hostname` argument passed to `connect_starttls`. This is the `StartTlsCapable` implementor's responsibility (consistent with existing TLS responsibility delegation).

### API surface

Roughly:

```rust
// New trait (in transport module).
pub trait StartTlsCapable: Transport {
    async fn upgrade_tls(&mut self, hostname: &str) -> Result<(), IoError>;
}

// New connect helper (in client module).
impl SmtpClient {
    pub async fn connect_starttls<T: StartTlsCapable>(
        transport: T,
        host: &str,        // EHLO hostname (client identifier)
        server_hostname: &str,  // server hostname for SNI / cert match
    ) -> Result<SmtpClient<T>, SmtpError>;
}

// New error variant.
pub enum ProtocolError {
    // ... existing variants
    StartTlsNotAdvertised,
}
```

### Test surface

A test using a local SMTP server that advertises STARTTLS (e.g. mailhog with TLS, or a stdlib-based test fixture) should demonstrate:

1. Successful EHLO → STARTTLS → TLS upgrade → EHLO → AUTH → MAIL FROM → RCPT TO → DATA → QUIT cycle.
2. Failure when the server doesn't advertise STARTTLS.
3. Failure when the TLS upgrade itself fails (cert mismatch, handshake error).
4. Failure when EHLO after TLS upgrade returns an error.

### Backward compatibility

Adding `StartTlsCapable` and `connect_starttls` is purely additive. Existing implicit-TLS users continue to use `connect` unchanged. No existing API signature changes.

## Acceptance criteria

- [ ] `StartTlsCapable` trait exposed in the transport module.
- [ ] `SmtpClient::connect_starttls<T: StartTlsCapable>` exposed in the client module.
- [ ] `ProtocolError::StartTlsNotAdvertised` (or equivalent) added.
- [ ] All four test scenarios above pass.
- [ ] Documentation updated to indicate STARTTLS is now in scope, with the same SNI / hostname-matching warnings as the existing implicit-TLS path.
- [ ] CHANGELOG entry under a new minor version (e.g. 0.7.0).

## Notes

- This proposal treats STARTTLS as a separate connection helper rather than a flag on the existing `connect`, because the trait-bound approach preserves the existing "transport is entirely responsible for TLS" boundary cleanly.
- Should the implementation prefer a single `connect_with_options(...)` shape (e.g. taking an enum of `ConnectionMode { ImplicitTls, StartTls }`), that's also fine — the externally-observable behaviour is what matters.

Filed by: peisear maintainer (downstream user). Happy to discuss design alternatives or contribute a PR if direction is agreed.
