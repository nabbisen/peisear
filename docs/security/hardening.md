# Security Hardening Notes

What peisear defends against by default, and what the operator is
expected to handle.

This document is about the **architectural and operational security
posture** of peisear. For reporting a vulnerability in peisear
itself, see [`.github/SECURITY.md`](https://github.com/nabbisen/peisear/blob/main/.github/SECURITY.md) (on
GitHub; outside this book).

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
  production. **`BIND_ADDR` defaults to `127.0.0.1:3000`** (`RFC 017`
  §5, `0.53.0`) — loopback, so an otherwise-untouched instance behind a
  reverse proxy is not also reachable directly on port 3000,
  **bypassing the proxy and with it the TLS termination and rate
  limiting this section delegates to it.** The one place an open bind
  is correct is the container image, which sets
  `BIND_ADDR=0.0.0.0:3000` itself because the host's own port mapping
  is the boundary there. *Amended 0.53.0: the default changed from
  `0.0.0.0:3000` to `127.0.0.1:3000` — see the `0.53.0` changelog entry
  for the remedy if you relied on the previous default to reach an
  instance from another machine.*
- **Rate limiting.** Configure at the proxy (`limit_req_zone` in
  nginx, equivalents in HAProxy / Envoy / Caddy).
- **DDoS absorption.** Not peisear's job.
- **Data at rest.** `data/app.db`, its `-wal` and `-shm` siblings, and
  every backup taken from it are not encrypted by peisear.
  **Anything that can read that file has every user's personal
  sustainability data** — the load, rhythm and WIP figures
  `NFR-PRIV-001`'s inventory restricts to the person they describe —
  in the clear, on disk. **The control is volume encryption**: LUKS,
  `dm-crypt`, or your cloud provider's encrypted volume, applied
  underneath the database file rather than inside it. **Declined, on
  the record** (owner, 2026-10-11): database-level encryption.
  SQLCipher is a driver-stack change underneath `sqlx`; column-level
  encryption would break the equality comparisons the personal
  metrics are computed from (`NFR-CONC-002`'s own `updated_at`
  comparison is one of them). **This does not change with
  PostgreSQL** — core PostgreSQL has no transparent data encryption
  either, so volume encryption is the control on both backends. This
  bullet documents a gap and names its mitigation; it is not a claim
  that peisear protects data at rest on its own.

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

**Generated automatically; there is nothing to set.** `RFC 017` §4
(`0.53.0`): on first run, if `JWT_SECRET` is unset, peisear generates
32 random bytes, writes them to a file named `jwt_secret` beside the
database file (`mode 0600` on Unix — Windows has no equivalent
permission bit to set, so the file relies on the filesystem's own
default ACL there), and reuses that file on every later run. Losing
the file logs out every existing session, the same consequence as
rotating a manually-configured secret.

**The file must be backed up with the database.** `NFR-CMP-003`'s
backup unit is the database's own directory, not just the `.db` file —
see [`../operations/backup.md`](../operations/backup.md). A restore
that brings back the database without the key file it was running
against still works (a new key is generated on the next start), but
invalidates every session that existed at backup time.

**Setting `JWT_SECRET` explicitly still works and still wins** — for a
multi-instance deployment sharing one secret, or an operator who
prefers to manage it themselves. Keep it out of source control if you
do. Rotate on suspected compromise, by deleting the generated file (or
changing the explicit variable) — every existing session is
invalidated the moment the secret changes, which is the same property
either path has.

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
[`.github/SECURITY.md`](https://github.com/nabbisen/peisear/blob/main/.github/SECURITY.md) (on GitHub;
outside this book).
