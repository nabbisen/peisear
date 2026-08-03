//! Shared test-only support for `peisear-i18n`'s integration tests.
//!
//! ## Why `tests/common/mod.rs` rather than `tests/common.rs`
//!
//! Rust treats every `.rs` file directly in `tests/` as its own
//! integration test crate. A bare `tests/common.rs` would compile as
//! its own (empty) test binary. Putting shared code in
//! `tests/common/mod.rs` instead opts the directory out of that, and
//! lets each real test crate (`tests/guard.rs`, `tests/rendering.rs`)
//! pull it in via `mod common;`.

pub mod fixture_locale;
