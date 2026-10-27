# Peisear

**Project and issue tracking that also shows how the project — and
the people in it — are holding up.**
Sophisticated. Solid. Really easy. Good UI.

![Status](https://img.shields.io/badge/status-unstable-red)
[![crates.io](https://img.shields.io/crates/v/peisear?label=rust)](https://crates.io/crates/peisear)
[![Rust Documentation](https://docs.rs/peisear/badge.svg?version=latest)](https://docs.rs/peisear)
[![Dependency Status](https://deps.rs/crate/peisear/latest/status.svg)](https://deps.rs/crate/peisear)
[![License](https://img.shields.io/github/license/nabbisen/peisear)](https://github.com/nabbisen/peisear/blob/main/LICENSE)

![logo](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/logo.png)

---

## Overview

peisear is a self-hostable project and issue management service for
individuals and small teams. It does the ordinary things — projects,
issues, sub-issues, a kanban board, teams and sprints — and then
answers two questions ordinary trackers leave invisible until they
become crises: **how is this project trending?** and **am I
overloaded?**

It answers them quietly, and without turning the answer into a
judgement. Personal sustainability data is visible only to the person
it describes — a team administrator holds a management role, not an
oversight role.

One Rust binary, one SQLite file, no Node toolchain, no external
services.

![The kanban board: issue cards in Open, In Progress and Done columns, with the project health indicators expanded to show the full breakdown](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/screenshots/board.png)

## Why peisear?

Most trackers can tell you what is left to do. Few can tell you that
one person is carrying the project, that three issues have not moved
in a fortnight, or that you have been over your own capacity for a
week — and fewer still will tell *you* first.

- **You own your data.** One SQLite file — `sqlite3 … ".backup"` while
  it runs, `cp` once it is stopped. No database server to operate.
- **You own your uptime.** One static binary. Your ops story is
  `systemctl restart peisear`.
- **You own your roadmap.** Clean Cargo workspace boundaries so a new
  backend, auth provider, or AI helper slots into a single crate.

## Quick Start

**To run it:**

```bash
cargo install peisear
peisear
#   asks two quick questions on first run (enter twice for the defaults)
#   → http://localhost:3000
```

**To read or change it:**

```bash
git clone https://github.com/nabbisen/peisear.git && cd peisear
cargo run --release -p peisear
#   asks two quick questions on first run (enter twice for the defaults)
#   → http://localhost:3000
```

Register an account, create a project, drag cards across the board.
The full guide is in [`docs/getting-started/`](https://github.com/nabbisen/peisear/blob/main/docs/getting-started/README.md).

## Features

- **Project health in plain language.** Six indicators; a non-healthy
  one explains itself in a sentence rather than a number.
- **A dashboard that is yours alone.** `/today`'s load, rhythm and
  sustainability panels, private to you. No route exists by which
  anyone else, including a team administrator, can read it.
- **Quiet by design.** Notifications fire once, at the moment a
  threshold is crossed, with a cooldown.
- **Teams, sprints and planning.** With carried-over work described
  as a fact, never as a shortfall.
- **Projects & issues with a kanban board.** Drag-and-drop between
  *Open* / *In Progress* / *Done*, or flip to a dense list view.
- **Secure-by-default auth.** Argon2id password hashing, 7-day JWT
  sessions in HTTP-only cookies, timing-attack-hardened login.
- **One binary deploy.** Migrations embedded at compile time,
  templates and static assets compiled into the binary. Ship one
  executable and you're live.

![The personal dashboard at /today: WIP and load chips under "Right now", the Rhythm panel expanded to show throughput and stale-issue counts, and a steady sustainability reading](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/screenshots/today.png)

## Design Notes

Three ideas shape every choice in this repo:

> **Keep the surface area small.** Small is easier to reason about,
> easier to audit, easier to move. Every feature has to pay for its
> complexity out of the project's limited budget.

> **Prefer types over conventions.** Where an invariant can be made
> unrepresentable, it is. Where a boundary can be enforced by the
> compiler, it is. The SQL layer, the HTTP layer, and the domain
> layer each own a distinct error type; conversions between them are
> explicit.

> **Treat operators with respect.** The person running the server is
> also a user. Config is environment variables. State is one file.
> Backups are `cp`. Logs go to stdout. No surprises on day one, no
> surprises on day 365.

Two more, about the shape of the code rather than the product:

> **Server-rendered, not client-rendered.** Leptos SSR components
> with typed props and compile-time HTML escaping. First paint is
> cache-friendly HTML, not a blank screen waiting for JavaScript.

> **One workspace, seven thin crates.** `i18n`, `core`, `auth`,
> `storage`, `notify`, `web`, and a `peisear` facade, each with a
> single clear job — so a new backend or identity provider has a
> well-bounded place to land. A PostgreSQL backend now has real code
> behind it, selectable at build time (`--features postgres`) and
> proven against a real server in CI — **not yet a supported
> configuration**, and not reachable from any published archive,
> container image, or `cargo install peisear`, which all still build
> SQLite only; see
> [ROADMAP.md](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md#postgresql-backend)
> for what shipped, what didn't, and what it actually required, which
> was more than the crate split alone suggested.

---

## Documentation

For architecture deep-dives, operations guides, and security notes,
see the full documentation:

- 📘 [**Documentation index**](https://github.com/nabbisen/peisear/blob/main/docs/README.md)
- 🚀 [Getting started](https://github.com/nabbisen/peisear/blob/main/docs/getting-started/README.md) — installation,
  configuration, first run
- 🏗️ [Architecture](https://github.com/nabbisen/peisear/blob/main/docs/architecture/README.md) — workspace layout,
  crate boundaries, Leptos SSR rationale
- 🛠️ [Operations](https://github.com/nabbisen/peisear/blob/main/docs/operations/README.md) — deployment, backup,
  Tailwind self-hosting
- 🔒 [Security](https://github.com/nabbisen/peisear/blob/main/docs/security/README.md) — what we do, what you do
- 🧭 [Roadmap](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md) · 📝 [Changelog](https://github.com/nabbisen/peisear/blob/main/CHANGELOG.md)

---

Built with care, under the [Apache-2.0](https://github.com/nabbisen/peisear/blob/main/LICENSE) license. Contributions
welcome — see [CONTRIBUTING](https://github.com/nabbisen/peisear/blob/main/.github/CONTRIBUTING.md). If peisear makes
your team's day a little calmer, that's the whole point.
