# Security Hardening Notes

What peisear defends against by default, and what the operator is
expected to handle.

This document is about the **architectural and operational security
posture** of peisear. For reporting a vulnerability in peisear
itself, see [../../.github/SECURITY.md](../../.github/SECURITY.md).

## Threat model

peisear is a small self-hosted application. The threats it takes
seriously:

- **External attackers** accessing the HTTP endpoint (SQLi, XSS,
  CSRF, session hijacking, enumeration).
- **Local users on a shared host** with shell access (mitigated by
  the systemd hardening directives, not by peisear itself).
- **A lost or compromised session token** (mitigated by JWT rotation
  via `JWT_SECRET` changes and short TTL).

Threats it does **not** try to handle on its own:

- **TLS termination.** Put a reverse proxy in front of peisear in
  production. `BIND_ADDR=127.0.0.1:3000` is the default-safe choice.
- **Rate limiting.** Configure at the proxy (`limit_req_zone` in
  nginx, equivalents in HAProxy / Envoy / Caddy).
- **DDoS absorption.** Not peisear's job.

## What peisear does

### SQL injection

Every query uses `?N` parameter binding through sqlx. There is no
string interpolation into SQL, anywhere, in any crate. A grep will
confirm this.

### XSS

Leptos's `view!{}` macro HTML-escapes every string interpolation by
default. User-controlled content (project names, issue titles,
descriptions, email addresses) is safe the moment it flows through
any component.

The one piece of hand-written JS, `static/board.js`, reads the
project id from a `data-project-id` attribute rather than a string
interpolated into a JS literal position. User content is never
emitted into a JavaScript code position.

### CSRF

All state-changing routes are `POST` and require the `it_session`
cookie. That cookie is set with `SameSite=Lax`, which means the
browser does **not** send it on cross-site non-navigation requests.
Combined with the lack of a wildcard CORS policy, this closes the
classical cross-site form-submission attack.

If you run peisear under a scheme that weakens this (e.g. wrapping it
in an iframe from a different origin), you're outside the
defaults' safety net.

### Password storage

Argon2id via the official `argon2` crate, with the crate's default
parameters (19 MiB memory, t=2, p=1). Salts are generated per hash
with `OsRng`. The hash is stored alone (no pepper), so rotating the
`JWT_SECRET` does not require a password rehash.

### Session cookies

The `it_session` cookie carries a JWT signed with `JWT_SECRET`:

| Attribute | Value |
|---|---|
| `HttpOnly` | always |
| `SameSite` | `Lax` |
| `Secure` | when `COOKIE_SECURE=1` |
| `Path` | `/` |
| `Max-Age` | 7 days |

The JWT payload contains `{ sub: user_id, email, iat, exp }`. On every
authenticated request, peisear re-fetches the user row from the
database — so deleting a user invalidates their sessions immediately,
without waiting for the JWT to expire.

### Timing-attack mitigation on login

If the submitted email doesn't match any user, peisear **still**
performs an argon2id verification against a fixed dummy hash. This
equalises the response time between "wrong password" and "no such
user", so an attacker can't enumerate registered email addresses by
looking at response latency.

### Access control at the query level

All mutations scope by `(owner_id, project_id)`. For example:

```rust
// From peisear-storage::projects::update
UPDATE projects
SET name = ?3, description = ?4, updated_at = CURRENT_TIMESTAMP
WHERE id = ?1 AND owner_id = ?2
```

Even if a handler forgot to verify ownership, the query would simply
affect zero rows. This is defence in depth: two separate layers have
to fail for an authorization bypass.

### Error surface

`AppError::Internal`, `AppError::Database`, and related variants
render a generic "An internal error occurred" to the client while
logging the detail at `tracing::error!` level internally. Database
errors never leak table names, constraint names, or row contents to
the HTTP response.

## What the operator does

### TLS

Terminate TLS at a reverse proxy. Set `COOKIE_SECURE=1` so the
session cookie is only sent over HTTPS. Set HSTS at the proxy.

### `JWT_SECRET` hygiene

Generate with `openssl rand -base64 48` (or stronger). Keep it out of
source control. Rotate on suspected compromise — every existing
session is invalidated the moment the secret changes.

### Security headers

A reasonable baseline at the reverse proxy:

```
Strict-Transport-Security: max-age=63072000; includeSubDomains; preload
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: strict-origin-when-cross-origin
Content-Security-Policy: default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'self'; frame-ancestors 'none';
Permissions-Policy: accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()
```

The `'unsafe-inline'` on `style-src` accommodates inline `style=`
attributes emitted by some Leptos components. Self-hosting Tailwind
(see [../operations/tailwind-local.md](../operations/tailwind-local.md))
lets you remove the CDN origins from `script-src` and `style-src`.

### Rate limiting

The login endpoint in particular is worth a request-per-second limit
at the proxy. Something like nginx's `limit_req_zone` or HAProxy's
`stick-table` is the right tool.

### Operating system hardening

The systemd unit in [../operations/deployment.md](../operations/deployment.md)
includes `NoNewPrivileges`, `ProtectSystem=strict`,
`MemoryDenyWriteExecute`, and a zeroed `CapabilityBoundingSet`. Those
directives narrow the blast radius if the process is somehow
exploited, without requiring any changes to peisear itself.

### Backups

Don't forget them. See [../operations/backup.md](../operations/backup.md).

## Reporting vulnerabilities

If you find a security issue in peisear, please report it privately
via the process described in
[../../.github/SECURITY.md](../../.github/SECURITY.md).
