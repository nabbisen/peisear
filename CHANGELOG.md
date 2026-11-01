# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **`README.md`'s Quick Start now leads with a path that needs
  nothing installed** — download the binary for your platform and run
  it — followed by `docker run` as a one-command alternative, then
  `cargo install` marked *if you already have Rust*, then the clone,
  which is for reading or changing the source and says so. **It had
  offered only the two paths that need Rust, one of them also `git`,
  for seven releases**, while [Install](docs/getting-started/install.md)
  had carried a prebuilt binary and a container image since `0.53.0`.
  The front door contradicted the install page, and
  `NFR-REL-007` is why nothing caught it: it required the README's
  *ordering* and said nothing about what the quick start must
  contain. That entry now carries the missing clause.

**`CFG-001` and `CFG-002`, the first two of three rounds applying
`RFC 022` §3's one rule**: *absence degrades; a value the operator
supplied and the product cannot use refuses startup, by name,
naming what would have been accepted.* `COOKIE_SECURE` and
`RUST_LOG` (`CFG-001`) already had a named error type and a
`Result` to refuse through; `SMTP_PORT` and `SMTP_TLS_MODE`
(`CFG-002`) did not — `peisear-notify` gained one. `CFG-003` (the
rule, enforced) is the remaining round.

### Changed

- **`COOKIE_SECURE` no longer treats every value other than `1`/
  `true` as `false`.** It still does, for exactly `1`, `0`, `true`,
  or `false` (case-insensitive for the words) — unset still degrades
  to `false`, unchanged. Anything else now refuses startup, naming
  the value given and the accepted set, instead of silently running
  with cookie security the operator did not actually ask for.
  `yes`/`no`/`on`/`off` are deliberately not accepted alongside
  them: a set with eight spellings is a set nobody remembers, and
  `1`/`true` must keep working exactly as before regardless (`RFC
  022` §5) — narrowing the accepting direction would be a worse
  regression than the one this round fixes.
- **`RUST_LOG` no longer silently falls back to the built-in filter
  on a value it cannot parse.** Unset still takes the built-in
  filter, unchanged. A value `tracing_subscriber::EnvFilter` rejects
  now refuses startup, naming `RUST_LOG` and carrying `EnvFilter`'s
  own parse error rather than a message this round would have had to
  invent.
- **`docs/getting-started/configuration.md`**'s reference table
  states `COOKIE_SECURE`'s accepted set and notes that both
  variables now refuse rather than silently default on a bad value.
- **`SmtpConfig::from_env` is now fallible** (`Result<Option<Self>,
  SmtpConfigError>`, was `Option<Self>`) — a signature change and one
  `?` at its single call site, not a refactor. `Ok(None)` still means
  "email channel not configured" and still never blocks a boot;
  `Err(_)` is the new fact, "a value was given and it is wrong."
- **`SMTP_PORT` no longer silently falls back to `465` on a value
  that does not parse as a port number.** Unset still takes `465`,
  unchanged. A present, unparseable value (including an explicitly
  empty one) now refuses startup, naming `SMTP_PORT` and the value
  given.
- **`SMTP_TLS_MODE` no longer silently derives a mode from the port
  on a value `from_str_loose` does not recognise.** Unset still
  derives from `SMTP_PORT`, unchanged. A present, unrecognised value
  now refuses startup, naming `SMTP_TLS_MODE` and listing every
  spelling the parser actually accepts.
- **The cascade this closes**: a malformed `SMTP_PORT` used to
  silently fall back to `465`, and that defaulted port then silently
  decided `SMTP_TLS_MODE` too — changing transport security with no
  sign anything was wrong. After this round a malformed `SMTP_PORT`
  refuses before any `SMTP_TLS_MODE` decision is ever made.

## Earlier series

Older release notes are archived, one file per series of ten minor
versions:

- [0.50 to 0.59](changelog/0.50-0.59.md)
- [0.40 to 0.49](changelog/0.40-0.49.md)
- [0.30 to 0.39](changelog/0.30-0.39.md)
- [0.20 to 0.29](changelog/0.20-0.29.md)
- [0.10 to 0.19](changelog/0.10-0.19.md)
- [0.1 to 0.9](changelog/0.1-0.9.md)
