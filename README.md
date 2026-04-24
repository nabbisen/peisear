# peisear

**Issue tracking that fits in a single binary.**
Sophisticated. Solid. Really easy. Good UI.

[![crates.io](https://img.shields.io/crates/v/peisear?label=rust)](https://crates.io/crates/peisear)
[![Rust Documentation](https://docs.rs/peisear/badge.svg?version=latest)](https://docs.rs/peisear)
[![Dependency Status](https://deps.rs/crate/peisear/latest/status.svg)](https://deps.rs/crate/peisear)
[![License](https://img.shields.io/github/license/nabbisen/peisear)](https://github.com/nabbisen/peisear/blob/main/LICENSE)

![logo](docs/src/assets/logo.png)

---

## Overview

peisear is a self-hostable issue management system — projects, issues,
a kanban board, and a handful of thoughtful edges. One Rust binary,
one SQLite file, no Node toolchain, no external services.

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

```bash
tar xzf peisear.tar.gz && cd peisear
cp .env.example .env
echo "JWT_SECRET=$(openssl rand -base64 48)" >> .env

cargo run --release -p peisear-web
#   → http://localhost:3000
```

Register an account, create a project, drag cards across the board.
The full guide is in [`docs/getting-started/`](docs/getting-started/README.md).

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
- **Workspace-first architecture.** Four crates — `core`, `auth`,
  `storage`, `web` — with deliberately thin waists, so swapping
  SQLite for Postgres or adding OIDC is a localised change.

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

- 📘 [**Documentation index**](docs/README.md)
- 🚀 [Getting started](docs/getting-started/README.md) — installation,
  configuration, first run
- 🏗️ [Architecture](docs/architecture/README.md) — workspace layout,
  crate boundaries, Leptos SSR rationale
- 🛠️ [Operations](docs/operations/README.md) — deployment, backup,
  Tailwind self-hosting
- 🔒 [Security](docs/security/README.md) — what we do, what you do
- 🧭 [Roadmap](ROADMAP.md) · 📝 [Changelog](CHANGELOG.md)

---

Built with care, under the [Apache-2.0](LICENSE) license. Contributions
welcome — see [CONTRIBUTING](.github/CONTRIBUTING.md). If peisear makes
your team's day a little calmer, that's the whole point.
