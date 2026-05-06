//! Shared helpers for peisear-web integration tests.
//!
//! Each integration test crate (`tests/smoke.rs`,
//! `tests/auth_boundary.rs`, etc.) imports this module via
//! `mod common;` at the crate root. The submodule structure here
//! groups helpers by concern:
//!
//! - [`server`] — `TestApp` factory: builds a fresh DB pool,
//!   applies migrations, constructs `AppState` and a `TestServer`.
//! - [`auth`] — registration, login, role assignment helpers
//!   that produce authenticated [`axum_test::TestServer`] sessions
//!   (Cookie-saving is enabled).
//! - [`fixture`] — domain data factories: project, issue,
//!   sub-issue (post-Phase C), team, sprint, capacity period.
//! - [`assertion`] — shared assertions for the cross-cutting
//!   invariants from the v2.1 spec, especially:
//!   - Authorization (§11.5): "this URL returns 403 for the wrong
//!     user, 401 unauthenticated"
//!   - Optimistic locking (§21.4): "this PUT returns 409 when the
//!     client_updated_at is stale"
//!
//! ## Why this is `tests/common/mod.rs` rather than `tests/common.rs`
//!
//! Rust treats every `.rs` file directly in `tests/` as its own
//! integration test crate. A bare `tests/common.rs` would be
//! compiled as an empty test binary. Putting the helpers in
//! `tests/common/mod.rs` instead opts the directory out of being
//! its own test crate, and lets each real test crate import it as
//! a module via `mod common;`. This is the convention noted in
//! the Rust Book's chapter on test organization.
//!
//! ## Why this lives here, not in a separate `peisear-test-support`
//! crate
//!
//! Today, helpers in this module are only used by peisear-web's
//! integration tests. peisear-notify has its own small helper set
//! local to its `tests/dispatch_integration.rs`, and peisear-storage
//! has no integration tests yet. If a second crate's tests grow to
//! need these helpers, factoring them out into a workspace-wide
//! `peisear-test-support` crate is a one-pass refactor — but it
//! isn't justified by current scope.

#![allow(dead_code)]
// Each submodule is re-exported below. Some helpers are used by
// only some integration test crates, which produces unused-function
// warnings when individual test crates are compiled. The unused
// dead_code allow is the standard mitigation for shared test
// helpers (see e.g. tokio's own test/ tree).

pub mod assertion;
pub mod auth;
pub mod fixture;
pub mod server;
