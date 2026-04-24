# Crate Boundaries

peisear is four crates because the Roadmap has four kinds of work,
and each kind has a natural home.

## The four crates

### `peisear-core` — the vocabulary

Pure domain types: `User`, `Project`, `Issue`, `IssueStatus`,
`Priority`, `CurrentUser`. Dependencies are `serde`, `chrono`, and
`thiserror` — nothing else. No axum, no sqlx, no HTTP library.

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

SQLite via sqlx today. The crate exports a public `Pool` type alias:

```rust
pub type Pool = sqlx::SqlitePool;
```

The rule: **anything that reads or writes persistent state lives
here.** Handlers never touch sqlx; they call `storage::users::find_by_id`,
`storage::issues::list_in_project`, and friends.

When a PostgreSQL backend lands, either a feature flag swaps the
`Pool` alias or a sibling `peisear-storage-postgres` crate takes the
same function signatures and the web crate depends on whichever is
selected. `StorageError` is already abstract enough (`Database`,
`Migration`, `InvalidData`, `NotFound`, `Bootstrap`) to hold either
backend's errors.

### `peisear-web` — the HTTP surface

Depends on all three above. The rule: **anything with a URL, a
status code, or a `Set-Cookie` header lives here.** This is where
axum, Leptos, extractors, handlers, and Leptos components all live.

It owns the app-wide `AppError` and its `IntoResponse` impl, plus
`From<StorageError>` and `From<AuthError>` conversions — so lower
layers get to use purpose-built error types, and handlers still
`?`-propagate uniformly up to a correct HTTP response.

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
| AI assistant per user | New `peisear-ai` crate alongside the existing four, depending on core + async HTTP client; web wires it in |
| **PostgreSQL backend** | Feature flag on `peisear-storage` or a sibling `…-postgres` crate. `Pool` alias and `StorageError` already in place |
| **OIDC / IDaaS** | New module inside `peisear-auth` behind a feature flag; web adds a callback handler |
| **CI/CD + IaC** | `infra/` directory: Dockerfile, compose.yaml, GitHub Actions, Terraform |

The table is a design tool. Before adding a feature, pick its row. If
its row doesn't exist yet, write one first — including the crate that
will own it.

## Next

- [Leptos SSR](leptos-ssr.md) — the one layer that sits squarely in
  `peisear-web` but deserves its own explanation
- [../ROADMAP.md](../../ROADMAP.md) — the items themselves
