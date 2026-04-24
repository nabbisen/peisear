# Overview

peisear is a small, typed, self-hosted web application. It aims to be
big enough to be genuinely useful and small enough that one person
can read and reason about the whole thing.

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

## Request lifecycle, in one diagram

```
    browser ─┬─► axum Router ─► extractor (AuthUser, MaybeAuthUser)
             │                   │
             │                   └─► verifies session cookie via
             │                       peisear-auth::jwt, then loads user
             │                       via peisear-storage::users
             │
             └─► handler (handlers/{auth,projects,issues,root}.rs)
                  │
                  ├─► reads/writes via peisear-storage queries
                  ├─► constructs props (CurrentUser, Project, Issue…)
                  └─► returns Leptos view rendered to HTML
                         (components/* → RenderHtml::to_html)
```

The critical property: each arrow crosses exactly one crate boundary,
and that boundary has an explicit error type
([Crate boundaries](crate-boundaries.md) covers the details).

## Key non-stack choices

- **Templates are Rust.** Leptos `#[component]` functions with typed
  props replace what would be `.html.tera` or `.html.askama` files in
  a conventional stack. Auto-escaping is on; props are compile-time
  checked.
- **Migrations are embedded.** `sqlx::migrate!()` bakes the SQL into
  the binary at compile time — no separate migration tool, no runtime
  filesystem lookup.
- **One binary, one file.** `target/release/peisear` + `data/app.db`
  is the whole deployable artifact (plus `static/`).
- **Configuration is env-vars-only.** No config file format, no TOML
  parsing; see [Configuration](../getting-started/configuration.md).

## What peisear is not

- **Not a multi-tenant SaaS.** Access control is per user, and a user
  sees only projects they own. There's no org or team concept yet
  (that's in the [Roadmap](../../ROADMAP.md)).
- **Not an API server.** There's a single JSON endpoint (kanban status
  change); everything else is classical server-rendered HTML with
  form POSTs.
- **Not yet reactive client-side.** Leptos is used in SSR mode only.
  See [Leptos SSR](leptos-ssr.md) for the rationale and
  [../guides/hydration-upgrade.md](../guides/hydration-upgrade.md) for
  the upgrade path.

## Where to next

- [Workspace layout](workspace-layout.md) — the file tree
- [Crate boundaries](crate-boundaries.md) — the module theory
