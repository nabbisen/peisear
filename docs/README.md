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
  six crates and what belongs in each
- [Leptos SSR](architecture/leptos-ssr.md) — why SSR-only mode, what it
  buys us, and what it rules out
- [Verifying the shipped JavaScript](static-js-verification.md) — what
  `cargo test` does not execute in `static/`, and the evidence runs that
  stand in for it

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
[SECURITY.md](https://github.com/nabbisen/peisear/blob/main/.github/SECURITY.md)
(on GitHub, outside this book).

## Guides

Deeper topics that don't fit cleanly into the other sections.

- [Upgrading to hydration](guides/hydration-upgrade.md) — path from
  SSR to full Leptos reactivity

## Specification

You want to know what the product is required to do, or what it presents.

- [Specification](specification/README.md) — the normative source of truth:
  [requirements](specification/requirements.md) (including `§10`, the
  compliance register) and [external design](specification/external-design.md)
  (including `§17`, where design and implementation have diverged). English
  only; amended at every release.

## Development

You're changing peisear and want to know how a release is written down, or
are looking into an upstream dependency's own gap.

- [Changelog, release notes and tags](development/changelog-and-releases.md) —
  where release notes live, what a release section opens with, the link a tag's
  message carries, and how older series are archived
- [wasm-smtp STARTTLS extension request](wasm-smtp-starttls-extension-request.md) —
  a proposal filed against the `wasm-smtp` crate this project depends on, for
  the SMTP submission port peisear's own email channel needs

## Elsewhere in the repo

- [README](https://github.com/nabbisen/peisear/blob/main/README.md) — the
  elevator pitch and quickstart
- [ROADMAP](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md) —
  what's next and where it will land
- [CHANGELOG](https://github.com/nabbisen/peisear/blob/main/CHANGELOG.md) —
  what has changed and when (the current series; older ones are in
  [`changelog/`](https://github.com/nabbisen/peisear/tree/main/changelog))
- [TERMS_OF_USE](https://github.com/nabbisen/peisear/blob/main/TERMS_OF_USE.md) —
  end-user terms template for operators deploying peisear
- [LICENSE](https://github.com/nabbisen/peisear/blob/main/LICENSE) — Apache-2.0
- [.github/](https://github.com/nabbisen/peisear/tree/main/.github) —
  community health files

These six are repository files, not pages of this book; the links above
leave the site and open them on GitHub, pinned to `main`.
