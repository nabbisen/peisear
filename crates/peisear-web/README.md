# peisear-web

[![crates.io](https://img.shields.io/crates/v/peisear?label=me)](https://crates.io/crates/peisear)
[![crates.io](https://img.shields.io/crates/v/peisear-web?label=peisear)](https://crates.io/crates/peisear-web)
[![Rust Documentation](https://docs.rs/peisear-web/badge.svg?version=latest)](https://docs.rs/peisear-web)
[![Dependency Status](https://deps.rs/crate/peisear-web/latest/status.svg)](https://deps.rs/crate/peisear-web)

HTTP layer for [peisear](https://crates.io/crates/peisear) — the
axum + Leptos SSR surface. Library only; the runnable binary lives
in the [`peisear`](https://crates.io/crates/peisear) facade crate.

## What's in here

- `build_router(state) -> axum::Router` — the full URL table.
- `AppState { db, jwt_secret, cookie_secure }` — cloneable shared state.
- `Config::from_env()` — environment-variable loader.
- `AppError` with `IntoResponse` and `From<StorageError>` /
  `From<AuthError>` bridges.
- `AuthUser` / `MaybeAuthUser` request extractors.
- The `components/` module — every page is a Leptos `#[component]`
  function with typed props.

## When to depend on this crate

- You are writing integration tests against peisear's HTTP surface.
- You are building an alternate front-end using the same handler
  signatures and Leptos components.
- You are forking peisear and need to add or replace handlers.

## When not

If you just want to run the server, install
[`peisear`](https://crates.io/crates/peisear) instead.
