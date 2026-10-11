# Peisear

**Issue tracking that fits in a single binary.**
Sophisticated. Solid. Really easy. Good UI.

![Status](https://img.shields.io/badge/status-unstable-red)
[![crates.io](https://img.shields.io/crates/v/peisear?label=rust)](https://crates.io/crates/peisear)
[![Rust Documentation](https://docs.rs/peisear/badge.svg?version=latest)](https://docs.rs/peisear)
[![Dependency Status](https://deps.rs/crate/peisear/latest/status.svg)](https://deps.rs/crate/peisear)
[![License](https://img.shields.io/github/license/nabbisen/peisear)](https://github.com/nabbisen/peisear/blob/main/LICENSE)

![logo](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/logo.png)

---

## Overview

peisear is a self-hostable issue management system — projects, issues,
a kanban board, and a handful of thoughtful edges. One Rust binary,
one SQLite file, no Node toolchain, no external services.

![The kanban board: issue cards in Open, In Progress and Done columns](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/screenshots/board.png)

## Why peisear?

Most issue trackers either bloat into swiss-army platforms or thin
themselves out into note apps. peisear stays in the middle on purpose:
enough to plan real work, little enough that you can read the whole
codebase in an afternoon and own your own instance over a weekend.

- **You own your data.** One SQLite file. `cp` is the backup strategy.
- **You own your uptime.** One static binary. Your ops story is
  `systemctl restart peisear`.
- **You own your roadmap.** Clean Cargo workspace boundaries so a new
  backend, auth provider, or AI helper slots into a single crate.

## Quick Start

**To run it:**

```bash
cargo install peisear
echo "JWT_SECRET=$(openssl rand -base64 48)" > .env

peisear
#   → http://localhost:3000
```

**To read or change it:**

```bash
git clone https://github.com/nabbisen/peisear.git && cd peisear
cp .env.example .env
echo "JWT_SECRET=$(openssl rand -base64 48)" >> .env

cargo run --release -p peisear
#   → http://localhost:3000
```

Register an account, create a project, drag cards across the board.
The full guide is in [`docs/getting-started/`](https://github.com/nabbisen/peisear/blob/main/docs/getting-started/README.md).

## Features

- **Projects & issues with a kanban board.** Drag-and-drop between
  *Open* / *In Progress* / *Done*, or flip to a dense list view.
- **Secure-by-default auth.** Argon2id password hashing, 7-day JWT
  sessions in HTTP-only cookies, timing-attack-hardened login.
- **Server-side rendered UI.** Leptos SSR components with typed props
  and compile-time HTML escaping. First paint is cache-friendly HTML,
  not a blank screen waiting for JavaScript.
- **One binary deploy.** Migrations embedded at compile time,
  templates compiled into the binary. Ship one executable plus
  `static/` and you're live.
- **Workspace-first architecture.** Seven crates — `i18n`, `core`,
  `auth`, `storage`, `notify`, `web`, and a thin `peisear` facade —
  with deliberately thin waists, so swapping SQLite for Postgres or
  adding OIDC is a localised change.

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
