# peisear-core

[![crates.io](https://img.shields.io/crates/v/peisear?label=me)](https://crates.io/crates/peisear)
[![crates.io](https://img.shields.io/crates/v/peisear-core?label=peisear)](https://crates.io/crates/peisear-core)
[![Rust Documentation](https://docs.rs/peisear-core/badge.svg?version=latest)](https://docs.rs/peisear-core)
[![Dependency Status](https://deps.rs/crate/peisear-core/latest/status.svg)](https://deps.rs/crate/peisear-core)

Pure domain types for [peisear](https://crates.io/crates/peisear), a
minimal self-hosted issue management system.

This crate is intentionally tiny and dependency-light: `serde`,
`chrono`, and `thiserror`. No axum, no sqlx, no HTTP machinery. It
defines the shared vocabulary — `User`, `Project`, `Issue`,
`IssueStatus`, `Priority`, `CurrentUser` — that every other crate in
the workspace agrees on.

## When to depend on this crate

- You are building a CLI, admin tool, analytics surface, or alternate
  front-end that needs to speak peisear's domain model without pulling
  in the web stack.
- You want compile-time-checked enum mappings between the database
  storage form and the domain form.

## When not

If you want the running server, depend on
[`peisear`](https://crates.io/crates/peisear) instead and use its
re-exports.
