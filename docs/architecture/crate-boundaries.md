# Crate Boundaries

peisear is split into six implementation crates plus a thin facade
crate, because the Roadmap has distinct kinds of work — each with a
natural home — and crates.io needs a single name to install.

## The six implementation crates

### `peisear-i18n` — every user-visible string

One enum, `MessageKey`, and one exhaustive `match` per shipped locale
(`Locale::render`) that turns a key into prose — a key without a
rendering fails to **compile**, rather than falling back to a raw key
string at runtime. Depends on no other crate in this workspace: the
one leaf, so every other crate can speak in message keys without a
dependency cycle back to presentation.

The rule: **a user-visible string is a message key, not a literal.**
Two of the three consumer crates carry a `MessageKey` (or a label enum
built from one) as typed data and never render it themselves —
`peisear-core`'s domain types and `StorageError::Conflict`/`::Validation`
are the examples; rendering happens only where the string is about to
be shown, through `peisear-web`'s own `t()` helper, pinned to English
today (`NFR-LANG-005`). `peisear-notify` is the one exception, and
deliberately so: a notification's copy is decided and persisted (the
audit row) before it is ever sent, so it calls `Locale::render` itself,
at the point a `DispatchEvent` is turned into a `Notification` —
there is no later web request to defer rendering to.

This crate also carries the `§1.7` prohibited-vocabulary guard
(`find_violations`), run as a blocking check over every shipped and
fixture locale table.

### `peisear-core` — the vocabulary

Pure domain types: `User`, `Project`, `Issue`, `IssueStatus`,
`Priority`, `CurrentUser`. Dependencies are `serde`, `chrono`,
`thiserror`, and `peisear-i18n` — nothing else. No axum, no sqlx, no
HTTP library. The `peisear-i18n` dependency is for label enums
(`IssueStatusLabel`, `PriorityLabel`, `HealthStateLabel`, …) that a
domain type converts itself into, never for rendering: this crate
calls no `Locale::render`.

The rule: **if it's the name of a concept in peisear, it lives here.**
Anything that speaks "a user, a project, an issue" can depend on core
without pulling in the web stack. A future CLI, admin tool, or
analytics pipeline shares this vocabulary.

### `peisear-auth` — the credential primitives

Password hashing with argon2id, JWT issue/verify, and a single
`AuthError` enum. No HTTP awareness. The rule: **anything that deals
with proving identity, but not transporting it, lives here.**

When OIDC support lands, the verifier goes here, next to the existing
JWT code. The web crate will acquire a callback handler, but the
OIDC verification itself belongs in this crate behind a feature flag.

### `peisear-storage` — the database

SQLite via sqlx today. `sqlite` (default) and `postgres` (off by
default, additive only when `sqlite` is disabled) are Cargo features;
the `dialect` module (`DIA-002`, `RFC 018` P2) owns what differs
between them — `Pool`, `Database`, `Tx`, `Connection` — and re-exports
`Pool`:

```rust
pub type Pool = dialect::Pool; // sqlx::SqlitePool today
```

The rule: **anything that reads or writes persistent state lives
here.** Handlers never touch sqlx; they call `storage::users::find_by_id`,
`storage::issues::list_in_project`, and friends.

**The PostgreSQL backend has real code behind it as of 0.55.0**
(`RFC 018` P3 plus the queries, `PG-001`–`PG-004`): a real
`PgConnectOptions`-backed connection, a ported migration set
(`migrations-postgres/`), the schema's eleven triggers, and every
production query — a parser translates SQLite's `?N`/bare-`?`
placeholders to PostgreSQL's `$N` mechanically, and the handful of
genuinely backend-specific constructs (date functions, the
optimistic-lock guard, one upsert) are rewritten per backend and
justified at their own call sites. **As of 0.56.0, the conformance
suite `RFC 018` P4 asked for exists and is green**: this crate's own
storage suite and `peisear-web`'s 37 lock tests run against a real
PostgreSQL server in CI, on every push, with no test helper left
hardcoding a `sqlite://` pool. **Still off by default and not a
supported configuration**: selecting `postgres` is additive, mutually
exclusive with `sqlite`, and an operator who does not choose it sees
no change; 298 of `peisear-web`'s 335 integration tests still never
run against PostgreSQL, among them the only tests proving two **P0**
privacy controls, and no release documents an install path.
`StorageError` is already abstract enough
(`Database`, `Migration`, `InvalidData`,
`NotFound`, `Bootstrap`, `Conflict`, `Validation`) to hold either
backend's errors. The last two carry a `peisear_i18n::MessageKey`
rather than a `String` — the crate depends on `peisear-auth`,
`peisear-core`, and `peisear-i18n`, never on axum or any web type, so a
storage-layer error can still name *what* went wrong without knowing
how it will be rendered.

### `peisear-notify` — the notification dispatch pipeline

Three stages connected by a tokio `mpsc::channel`: detection (a
caller in `peisear-web` observes a state change and sends a
`DispatchEvent`), filtering + delivery (resolves the user's
notification preference, applies severity and cooldown filters,
persists an audit row), and per-channel send (`in_app`, `email`,
`webhook`). Depends on `peisear-core` (domain types), `peisear-storage`
(the audit row, preference lookups), and `peisear-i18n` (the
notification copy itself is message keys, rendered only at the point
of send). The rule:
**anything about deciding whether, and how, to notify a user lives
here** — separate from `peisear-web` so the filtering/cooldown logic
is testable without spinning up HTTP.

### `peisear-web` — the HTTP surface

Depends on all four above. The rule: **anything with a URL, a
status code, or a `Set-Cookie` header lives here.** This is where
axum, Leptos, extractors, handlers, and Leptos components all live.

It owns the app-wide `AppError` and its `IntoResponse` impl, plus
`From<StorageError>` and `From<AuthError>` conversions — so lower
layers get to use purpose-built error types, and handlers still
`?`-propagate uniformly up to a correct HTTP response.

This crate is **library-only**; the runnable binary is owned by the
facade.

## The facade crate

### `peisear` — crates.io entry point

Depends on **five** of the six implementation crates and re-exports
**four** of those as `peisear::core`, `peisear::auth`,
`peisear::storage`, and `peisear::web`. The other two are not
re-exported, for two different reasons: **`peisear-notify`** *is* a
direct dependency of this facade — `main.rs` starts its dispatch loop
itself — but is not re-exported, so external consumers reach it only
through `peisear::web`'s own request handling, never as its own
module. **`peisear-i18n`** is not a direct dependency of this facade
at all; four of the other five crates depend on it
(`peisear-core`, `peisear-storage`, `peisear-notify`, `peisear-web` —
**not `peisear-auth`**, which carries no user-visible string), so it
arrives transitively and the facade has no call site of its own to
name it from. Owns `[[bin]] name = "peisear"`
so that `cargo install peisear` ships the runnable server.

The rule: **the facade is the public face on crates.io; it does not
own logic.** `main.rs` is a fifteen-line bootstrap that calls into
`peisear::web::build_router`. Implementation details remain in the
named crates and may evolve independently; the facade can pin
compatible versions of each at publish time.

This split exists for two independent reasons:

1. **One install command.** `cargo install peisear` is the natural
   thing for end users to type; `cargo install peisear-web` would be
   surprising.
2. **Independent publishability.** `peisear-core` is useful to anyone
   building tooling on the same domain vocabulary, even if they never
   touch the HTTP layer; `peisear-auth` is useful to anyone needing
   the same JWT/argon2 primitives. Keeping them as their own
   published crates lets them be picked up à la carte.

## Why the error split matters

A single `AppError` that knew about HTTP, SQL, and JWT simultaneously
would force `peisear-storage` to depend on axum just to say "row not
found" in a way the web crate understood. With the split:

- `peisear-storage` says `StorageError::NotFound`
- `peisear-auth` says `AuthError::Jwt(e)`
- `peisear-web` converts both via `From` into `AppError`, and
  `AppError: IntoResponse` converts that into either a 404 HTML page
  or a 303 redirect to `/login`.

Each conversion step is the one place where a layer's concerns leak
into the next, and the leaks are one-directional and explicit.

## How the Roadmap maps onto this layout

| Roadmap item | Where it lands |
|---|---|
| Per-issue effort estimates | Column on `issues` (storage migration) + field on `Issue` (core) + form rendering (web) |
| Per-period capacity limits | New table + queries in storage; new pages in web |
| Project-health score | Computed query in storage; component in web |
| AI assistant per user | New `peisear-ai` crate alongside the existing five, depending on core + async HTTP client; web wires it in |
| **PostgreSQL backend** | `postgres` feature flag on `peisear-storage` (`DIA-002`, 0.54.0, the type seam) has real code behind it as of 0.55.0 — connection, migration set, triggers and every production query (`RFC 018` P3 plus the queries, `PG-001`–`PG-004`). `P4`'s conformance suite (0.56.0) proves this crate's own storage and lock tests against both backends in CI; a supported, documented install path remains unscheduled (`ROADMAP.md`'s P5) |
| **OIDC / IDaaS** | New module inside `peisear-auth` behind a feature flag; web adds a callback handler |
| **CI/CD + IaC** | `infra/` directory: Dockerfile, compose.yaml, GitHub Actions, Terraform |

The table is a design tool. Before adding a feature, pick its row. If
its row doesn't exist yet, write one first — including the crate that
will own it.

## Next

- [Leptos SSR](leptos-ssr.md) — the one layer that sits squarely in
  `peisear-web` but deserves its own explanation
- [`ROADMAP.md`](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md) — the items themselves (on
  GitHub; outside this book)
