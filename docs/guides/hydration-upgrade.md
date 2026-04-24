# Upgrading to Hydration

peisear ships with Leptos in `ssr` mode only — the server renders HTML
and the browser displays it. This document describes the path to
adding full hydration, so that signals, effects, and client-side
routing all start working without a page reload.

See [../architecture/leptos-ssr.md](../architecture/leptos-ssr.md)
first for why we started here.

## When to do this

The right trigger is a concrete feature that SSR makes clumsy:

- **Real-time updates** — multiple users watching a board and seeing
  each other's changes without refresh.
- **Inline editing** — modifying a field in place without a full-page
  POST/redirect round-trip.
- **Client-side filtering and search** — instant filter of an issue
  list by keyword or status without a server round trip.
- **Optimistic UI** — showing a drag as complete before the server
  confirms, with graceful rollback on failure.

If none of those motivate a change, the SSR model is serving you; don't
migrate just for its own sake.

## The pieces that change

### 1. Toolchain

Add the wasm target and `cargo-leptos`:

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

### 2. `crates/peisear-web/Cargo.toml`

Add a `hydrate` feature and promote Leptos and axum integration to it:

```toml
[features]
default = ["ssr"]
ssr     = ["leptos/ssr", "dep:leptos_axum"]
hydrate = ["leptos/hydrate"]

[dependencies]
leptos       = { workspace = true, default-features = false }
leptos_axum  = { workspace = true, optional = true }
```

Add the metadata cargo-leptos expects:

```toml
[package.metadata.leptos]
bin-package = "peisear-web"
lib-package = "peisear-web"
output-name = "peisear"
site-root   = "target/site"
site-pkg-dir = "pkg"
assets-dir  = "static"
site-addr   = "0.0.0.0:3000"
```

### 3. Router

`axum::Router` becomes `leptos_axum::LeptosRoutes`:

```rust
let conf    = leptos::config::get_configuration(None)?;
let leptos  = conf.leptos_options;
let routes  = leptos_axum::generate_route_list(App);

let app = Router::new()
    .route("/health", get(root::health))
    .leptos_routes(&leptos, routes, App)
    .with_state(state);
```

The non-Leptos handlers (`/register`, `/login`, `/logout`,
`/projects`, `/projects/{id}/issues/{issue_id}/status` JSON endpoint)
still exist, but most of the page handlers move into `#[server]`
functions or disappear entirely, because Leptos routing handles the
HTML shell.

### 4. State threading

`AppState { db, jwt_secret, cookie_secure }` is passed into the
Leptos context on server creation, and retrieved inside `#[server]`
functions via `use_context::<AppState>()`.

### 5. Server-only gating

Anything that must not cross the wasm boundary — sqlx queries,
argon2, the JWT crate, the DB pool — gets gated behind
`#[cfg(feature = "ssr")]`. The `peisear-core` crate is safe to depend
on from both sides because it's pure data. The `peisear-auth` and
`peisear-storage` crates stay server-only.

### 6. Build

```bash
cargo leptos build --release
```

This produces a server binary plus a `pkg/` directory containing the
hydrated wasm bundle. Both must ship to production; deployment
gains one directory.

## What doesn't change

**Component source code.** Every `#[component]` function under
`crates/peisear-web/src/components/` stays identical. That's the
main payoff of having started in Leptos rather than a templating
language — the move from SSR to hydration is a router change, a
toolchain change, and a handful of feature flags, not a rewrite.

**The core, auth, and storage crates.** `peisear-core`,
`peisear-auth`, and `peisear-storage` are unaffected.

**The systemd unit, env vars, and backup story.** Same binary name,
same working directory layout, same `static/` semantics (with one
extra subdirectory for the wasm bundle).

## Known sharp edges

- **Form submissions.** Classical `<form action="/x" method="post">`
  still works but bypasses reactivity. Converting each form into a
  `<ActionForm>` or `#[server]` submission is an iterative migration.
- **Drag-and-drop.** `static/board.js` can go away once the
  drag handlers live in Leptos with client-side signals. Until then,
  it coexists with the hydrated components harmlessly.
- **Cookie-based auth.** Session cookies and JWT verification still
  work identically; no change needed.

## Reference

The Leptos book's SSR-with-axum template is the best live example:
<https://github.com/leptos-rs/leptos/tree/main/examples/ssr_modes_axum>.
