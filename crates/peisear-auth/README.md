# peisear-auth

[![crates.io](https://img.shields.io/crates/v/peisear?label=me)](https://crates.io/crates/peisear)
[![crates.io](https://img.shields.io/crates/v/peisear-auth?label=peisear)](https://crates.io/crates/peisear-auth)
[![Rust Documentation](https://docs.rs/peisear-auth/badge.svg?version=latest)](https://docs.rs/peisear-auth)
[![Dependency Status](https://deps.rs/crate/peisear-auth/latest/status.svg)](https://deps.rs/crate/peisear-auth)

Credential primitives for [peisear](https://crates.io/crates/peisear).
Argon2id password hashing, JWT issue/verify, and a single `AuthError`
enum — framework-agnostic, no HTTP awareness.

## API surface

```rust
peisear_auth::password::hash(password)        -> AuthResult<String>
peisear_auth::password::verify(pw, hash)      -> AuthResult<bool>

peisear_auth::jwt::issue(user_id, email, secret) -> AuthResult<String>
peisear_auth::jwt::verify(token, secret)         -> AuthResult<Claims>
```

## Defaults

- **Argon2id** with the `argon2` crate's default parameters
  (19 MiB memory, t=2, p=1).
- **JWT** with HS256 and a 7-day TTL (`SESSION_TTL_SECS`).

## Future

OIDC verifier support is planned to land in this crate behind a
feature flag, alongside the existing JWT primitives. See the
[ROADMAP](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md).
