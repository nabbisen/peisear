# peisear Documentation

Welcome. This directory holds the full documentation for peisear,
organised by what you're trying to do.

## Getting Started

You're new here and want to run peisear.

- [Installation](getting-started/installation.md) — getting Rust and
  the workspace onto a machine
- [Configuration](getting-started/configuration.md) — environment
  variables and `.env`
- [First run](getting-started/first-run.md) — build, launch, register,
  create your first issue

## Architecture

You're about to change the code and want to understand the shape of it
first.

- [Overview](architecture/overview.md) — the stack at a glance
- [Workspace layout](architecture/workspace-layout.md) — every file,
  every directory, why it's there
- [Crate boundaries](architecture/crate-boundaries.md) — why there are
  four crates and what belongs in each
- [Leptos SSR](architecture/leptos-ssr.md) — why SSR-only mode, what it
  buys us, and what it rules out

## Operations

You're running peisear in production (or trying to).

- [Deployment](operations/deployment.md) — single-binary deploy,
  systemd unit, directory layout
- [Backup](operations/backup.md) — SQLite online backup and cold copy
- [Tailwind self-hosting](operations/tailwind-local.md) — removing the
  CDN dependency

## Security

- [Hardening notes](security/hardening.md) — what peisear defends
  against by default, and what's left to the operator

For reporting a vulnerability, see the repository's
[SECURITY.md](../.github/SECURITY.md).

## Guides

Deeper topics that don't fit cleanly into the other sections.

- [Upgrading to hydration](guides/hydration-upgrade.md) — path from
  SSR to full Leptos reactivity

## Elsewhere in the repo

- [README](../README.md) — the elevator pitch and quickstart
- [ROADMAP](../ROADMAP.md) — what's next and where it will land
- [CHANGELOG](../CHANGELOG.md) — what has changed and when
- [TERMS_OF_USE](../TERMS_OF_USE.md) — end-user terms template for
  operators deploying peisear
- [LICENSE](../LICENSE) — Apache-2.0
- [.github/](../.github/) — community health files
