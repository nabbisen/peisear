# Peisear

[![crates.io](https://img.shields.io/crates/v/peisear?label=rust)](https://crates.io/crates/peisear)
[![Rust Documentation](https://docs.rs/peisear/badge.svg?version=latest)](https://docs.rs/peisear)
[![Dependency Status](https://deps.rs/crate/peisear/latest/status.svg)](https://deps.rs/crate/peisear)
[![License](https://img.shields.io/github/license/nabbisen/peisear)](https://github.com/nabbisen/peisear/blob/main/LICENSE)

![logo](docs/src/assets/logo.png)

A minimal, self-hostable issue management system written in Rust. Organized as a Cargo workspace with a clean separation
between domain, auth, storage, and web layers.

- **Sophisticated** — Typed domain model, server-side rendering,
  robust error handling with `IntoResponse`, argon2id password
  hashing, JWT sessions.
- **Solid** — `sqlx` parameterized queries (no string concatenation,
  no injection), `CHECK` constraints on enum columns, foreign keys
  with `ON DELETE CASCADE`, WAL-mode SQLite for concurrent reads.
- **Really Easy** — Single binary + a single `.db` file. No Node.js
  toolchain, no external services. Backups are just `cp app.db backup.db`.
- **Good UI/UX** — Tailwind + daisyUI, board/list toggle,
  drag-and-drop kanban, mobile-responsive layout.

## Stack

| Layer | Choice |
|---|---|
| Language | Rust 1.85+ (Edition 2024) |
| Web | `axum` 0.8 |
| Async | `tokio` |
| Storage | `sqlx` 0.8 + SQLite (WAL, FKs on) |
| UI | `leptos` 0.8 (SSR-only feature) — server-rendered reactive components |
| Styling | Tailwind CSS + daisyUI (via CDN by default) |
| Auth | `jsonwebtoken` 9, `argon2` 0.5, HTTP-only cookies |
| Validation | `validator` 0.18 |

## A note on Leptos modes

Leptos has three compile-time modes:

| Feature | Role | Target(s) required |
|---|---|---|
| `csr` | Client-side rendering (SPA) | `wasm32-unknown-unknown` only |
| `hydrate` | Server render + browser hydration | Both `x86_64` *and* `wasm32` |
| **`ssr`** | **Server renders HTML, no client reactivity** | `x86_64` only |

peisear uses **`ssr` only**. The server assembles HTML from
`#[component]` functions on each request and ships it to the browser —
the same pattern as askama or Jinja, but the templates are real Rust
components with typed props and the `view!{}` macro. The `RenderHtml`
trait's `.to_html()` method turns a view into a `String` and we hand
that back through axum's `Html` response.

No `wasm32` target is needed, so peisear builds on a stock `rustc`
from apt. Every page — login, register, projects, board/list, issues
— is a Leptos component under `crates/peisear-web/src/components/`.

Upgrading later to full `hydrate` mode for client-side reactivity is
a two-step change: install the `wasm32-unknown-unknown` target
(`rustup target add wasm32-unknown-unknown`), switch the feature from
`ssr` to `hydrate`, and add a second build step for the client bundle
(`cargo-leptos`). Component code is unchanged; only the server wiring
moves from plain axum handlers to `leptos_axum::LeptosRoutes`. See
**Upgrading to hydration** at the bottom.

## Workspace layout

peisear is a Cargo workspace split into four published crates plus
supporting directories. The split is designed to support the Roadmap
(see below): each future item slots into exactly one crate boundary.

```
peisear/
├── Cargo.toml                       # [workspace] + [workspace.dependencies]
├── Cargo.lock
├── README.md · LICENSE-MIT · LICENSE-APACHE
├── .env.example · .gitignore
│
├── crates/
│   ├── peisear-core/                # Pure domain types.
│   │   └── src/lib.rs               #   User, Project, Issue, IssueStatus, Priority,
│   │                                #   CurrentUser. Deps: serde, chrono, thiserror only.
│   │
│   ├── peisear-auth/                # Password hashing + JWT. No axum.
│   │   └── src/{lib,jwt,password}.rs
│   │                                #   AuthError, argon2id hash/verify, JWT issue/verify.
│   │
│   ├── peisear-storage/             # Persistence.
│   │   ├── migrations/0001_initial.sql
│   │   └── src/{lib,pool,users,projects,issues}.rs
│   │                                #   StorageError, Pool alias (SqlitePool today),
│   │                                #   per-table query modules. sqlx + core only.
│   │
│   └── peisear-web/                 # HTTP surface. Depends on all three above.
│       ├── Cargo.toml               #   [[bin]] name = "peisear"
│       └── src/
│           ├── main.rs              # Binary entry — bootstrap + serve.
│           ├── lib.rs               # Re-exports for integration tests.
│           ├── app.rs               # `build_router(AppState) -> Router`.
│           ├── state.rs             # AppState.
│           ├── config.rs            # Environment loader.
│           ├── error.rs             # AppError + IntoResponse; From<StorageError>,
│           │                        # From<AuthError> bridges.
│           ├── extractors.rs        # AuthUser, MaybeAuthUser, AUTH_COOKIE.
│           ├── handlers.rs + handlers/*.rs  # Axum handlers per resource.
│           └── components.rs + components/*.rs  # Leptos SSR components.
│
├── static/
│   ├── app.css                      # Supplemental CSS.
│   └── board.js                     # Kanban drag-and-drop.
│
└── infra/                           # Placeholder for roadmap CI/CD & IaC work.
    └── README.md                    # Describes planned contents (Dockerfile,
                                     # compose.yaml, terraform/, github/, k8s/).
```

The source tree follows the Rust 2018+ module layout: a module `foo` is
declared in `src/foo.rs`, and any submodules live in `src/foo/`. No
`mod.rs` files.

### Crate boundaries and why they exist

- **`peisear-core`** is the vocabulary of the product. Anything that
  speaks "a user, a project, an issue" can depend on it without
  pulling in axum, sqlx, or a JSON library at runtime. This is where
  any future CLI, admin tool, or analytics surface will share a
  contract with the web app.
- **`peisear-auth`** is the credential primitives — password hashing
  with argon2id and JWT issue/verify. It has no awareness of HTTP.
  When OIDC support lands (per roadmap), the verifier goes here, next
  to the existing JWT code, behind a feature flag.
- **`peisear-storage`** owns the DB. Today the concrete pool is a
  `SqlitePool`; the crate exports a public `Pool` type alias so that
  when a PostgreSQL backend lands, either a feature flag or a sibling
  `peisear-storage-postgres` crate can swap it. Query signatures
  (`users::find_by_id`, `issues::list_in_project`, …) are the natural
  seam for either route.
- **`peisear-web`** is where everything becomes HTTP. It owns
  `AppError: IntoResponse` and `From<StorageError>`/`From<AuthError>`
  conversions, so lower layers get to use their own purpose-built
  error types and handlers can still `?`-propagate uniformly.

### How the Roadmap maps onto this layout

| Roadmap item | Where it lands |
|---|---|
| Per-issue effort estimates | Column on `issues` (storage migration) + field on `Issue` (core) + form rendering (web) |
| Per-period capacity limits | New table + queries in storage; new pages in web |
| Project-health score | Computed query in storage; component in web |
| AI assistant per user | New `peisear-ai` crate alongside the existing four, depending on core + async HTTP client; web wires it in |
| **PostgreSQL backend** | Feature flag on `peisear-storage` or a sibling `…-postgres` crate. `Pool` alias and `StorageError` already in place |
| **OIDC / IDaaS** | New module inside `peisear-auth` (no web changes required for the verifier itself; web adds a callback handler) |
| **CI/CD + IaC** | `infra/` directory: Dockerfile, compose.yaml, GitHub Actions, Terraform |

## Getting started

### 1. Install Rust

Any Rust with Edition 2024 support (1.85+). On Debian/Ubuntu 24.04:

```bash
sudo apt install rustc-1.91 cargo-1.91
sudo ln -sf /usr/bin/rustc-1.91  /usr/local/bin/rustc
sudo ln -sf /usr/bin/cargo-1.91  /usr/local/bin/cargo
```

Or with rustup (recommended when you need extra targets like `wasm32`):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### 2. Configure

```bash
cp .env.example .env
# Generate a real JWT secret for production:
echo "JWT_SECRET=$(openssl rand -base64 48)" >> .env
```

### 3. Build and run

Run Cargo from the workspace root. It will build the binary target
defined in `crates/peisear-web/Cargo.toml`:

```bash
cargo run --release -p peisear-web
```

Or, because there is exactly one runnable binary in the workspace,
`cargo run --release` works too. The binary itself is named `peisear`.

Open <http://localhost:3000>. The SQLite file is created at
`./data/app.db` on first run, and migrations run automatically at
startup.

### 4. Register and use

1. `/register` — create an account (8+ character password)
2. `/projects` — create a project
3. Inside a project — create issues, toggle between **Board** (kanban)
   and **List** views, drag cards across columns to change status.

## Configuration

All configuration is via environment variables (or `.env`):

| Variable | Default | Notes |
|---|---|---|
| `DATABASE_URL` | `sqlite://data/app.db` | Parent directory is auto-created. |
| `JWT_SECRET` | insecure dev default (with a warning) | **MUST** be set to a long random string in production. |
| `BIND_ADDR` | `0.0.0.0:3000` | Use `127.0.0.1:3000` to bind to localhost only. |
| `COOKIE_SECURE` | `0` | Set to `1` when serving over HTTPS. |
| `RUST_LOG` | `info,sqlx=warn,…` | Any `tracing_subscriber` env filter works. |

## Security notes

- **SQL injection** — every query uses `?N` parameter binding through `sqlx`. No string interpolation.
- **XSS** — Leptos automatically HTML-escapes every string interpolation in `view!{}` macros. User-supplied values in project names, issue titles, and descriptions are safe by default. The kanban drag-and-drop script lives in `static/board.js` and reads the project id from a `data-project-id` attribute, so no user-controllable data is ever written into a JavaScript literal position.
- **CSRF** — state-changing routes are all `POST` and require the `it_session` cookie (not a bearer token), which the browser sends only from same-site requests when `SameSite=Lax` is set (our default).
- **Password storage** — `argon2id` via the official `argon2` crate, default parameters (19 MiB, t=2, p=1).
- **Session cookie** — `HttpOnly`, `SameSite=Lax`, and `Secure` when `COOKIE_SECURE=1`. TTL 7 days.
- **Timing attacks on login** — if the email is not found we still run a dummy verification against a fixed hash so the response time is indistinguishable from a wrong-password case.
- **Access control** — all DB mutations scope by `(owner_id, project_id)`. Even if a handler misses a check, the query itself will return 0 rows.

## Operations

### Backup

```bash
# While the server is running WAL does concurrent readers; this is safe:
sqlite3 data/app.db ".backup data/app.backup.db"

# Or just stop the server and cp:
cp data/app.db data/app.backup.db
```

### Cross-compiling / single-binary deploy

Run `cargo build --release` and ship `target/release/peisear` + the
`static/` folder. Templates are compiled into the binary (they are
Leptos components) and migrations are embedded at compile time via
`sqlx::migrate!()`, so only `static/` needs to travel alongside the
binary. Sample systemd unit:

```ini
[Unit]
Description=peisear
After=network.target

[Service]
WorkingDirectory=/var/lib/peisear
ExecStart=/usr/local/bin/peisear
EnvironmentFile=/etc/peisear.env
Restart=on-failure
User=peisear

[Install]
WantedBy=multi-user.target
```

The `WorkingDirectory` must contain the `static/` subdirectory so the
runtime `ServeDir::new("static")` mount resolves. For an even simpler
deploy, baking assets into the binary with `include_dir!()` or
`rust-embed` removes that requirement entirely.

### Shipping Tailwind locally instead of via CDN

The default `<Base>` component (in
`crates/peisear-web/src/components/layout.rs`) references
`cdn.tailwindcss.com` (Tailwind Play CDN) and a daisyUI CSS bundle.
This is fine for self-hosted deployments where outbound HTTPS is
available. To remove the CDN dependency entirely:

```bash
npm install -D tailwindcss@3 daisyui@4
npx tailwindcss -i ./input.css -o ./static/app.css --minify
```

Then change the `<link>` / `<script>` tags in `components/layout.rs`
to point at `/static/app.css`.

## Upgrading to hydration (future work)

Hydration turns the server-rendered HTML into a fully reactive
client-side app: signals, effects, and client-side routing all start
working without a page reload. The code you have is most of the way
there; what's left is the toolchain and a second compile target for
the browser.

1. Install the wasm target and `cargo-leptos`:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install cargo-leptos
   ```
2. In `crates/peisear-web/Cargo.toml`, add a `hydrate` feature that
   pulls `leptos/hydrate` + `leptos_axum`, keep `ssr` for the server
   binary, and split the current crate into a `[package.metadata.leptos]`
   block the cargo-leptos tool recognises.
3. Router changes: `axum::Router` is replaced by
   `leptos_axum::LeptosRoutes` so the server can deliver both the HTML
   shell and the hydration payload. The `AppState { db, jwt_secret,
   cookie_secure }` becomes a Leptos context, threaded into handlers
   via `use_context`.
4. Server-only logic (`auth`, `storage`, `error` modules and the
   extractors) stays put behind `#[cfg(feature = "ssr")]`. Form
   submissions can optionally become `#[server]` functions so the same
   Rust code runs on both sides.

Component code under `components/` compiles without changes — that's
the benefit of using real Leptos components from the start rather
than a templating language. The `core`, `auth`, and `storage` crates
are unaffected.

## Roadmap

Lifted from the spec:

- Workload-fairness features: per-issue effort estimates, per-period capacity limits per assignee, project-health score, AI assistant per user.
- Pluggable backends (PostgreSQL via the same sqlx layer — the core queries are already portable).
- IdP / IDaaS integration (OIDC).
- CI/CD integration and IaC support.
