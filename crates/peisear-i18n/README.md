# peisear-i18n

[![crates.io](https://img.shields.io/crates/v/peisear?label=me)](https://crates.io/crates/peisear)
[![crates.io](https://img.shields.io/crates/v/peisear-i18n?label=peisear)](https://crates.io/crates/peisear-i18n)
[![Rust Documentation](https://docs.rs/peisear-i18n/badge.svg?version=latest)](https://docs.rs/peisear-i18n)
[![Dependency Status](https://deps.rs/crate/peisear-i18n/latest/status.svg)](https://deps.rs/crate/peisear-i18n)

The single place every user-visible string in [peisear](https://crates.io/crates/peisear)
lives, and the compile-time and test-time guarantees built on top of that:

- A message resolves through a **key enum**, not a string constant or a
  runtime map. A key with no rendering for a locale is a compile error, not
  a fallback.
- A **blocking test** walks every table entry and asserts none of it
  contains vocabulary §1.7 of the requirements baseline prohibits.

This crate is intentionally tiny and dependency-free — no `serde`, no
`chrono`, nothing. It has no workspace dependencies at all: `peisear-core`,
`peisear-notify`, and `peisear-web` depend on it, not the reverse, which is
what lets the domain crate emit message keys without a dependency cycle.

## What this is not

Not a multi-language release. One locale ships (English) — see
`NFR-LANG-005`. The point is a single, checkable place for copy, not
translation. See [RFC 006](https://github.com/nabbisen/peisear/blob/main/rfcs/accepted/006-i18n-architecture.md).

## When to depend on this crate

You're rendering, composing, or generating a user-visible string anywhere
in peisear — HTML, a notification body, an email. Route it through a
[`MessageKey`] rather than writing prose inline.

## When not

If you want the running server, depend on
[`peisear`](https://crates.io/crates/peisear) instead.
