# Roadmap

This document lays out where peisear is going, in three time
horizons. The Cargo workspace layout is deliberately designed so that
each roadmap item lands in one well-defined crate — see
[docs/architecture/crate-boundaries.md](docs/architecture/crate-boundaries.md)
for the mapping.

## Near-term

The next few minor releases.

### Workload fairness

A cluster of features that together let a team distribute work
without one person silently drowning.

- **Per-issue effort estimates** — storage migration adds an `effort`
  column on `issues`; core adds a field to `Issue`; web renders a
  selector on the issue form. Pure additive change.
- **Per-period capacity limits per assignee** — new `capacities`
  table keyed by `(user_id, period)`; queries for per-user sum of
  in-flight effort; a simple warn-on-overload UX.
- **Project-health score** — a computed aggregate (fraction of
  issues on-track, age of oldest open issue, churn rate). Purely a
  SELECT query; rendered as a component on the project detail page.

### AI assistant per user

A per-user helper that can summarise issues, suggest labels, and
draft responses. Lands as a new `peisear-ai` crate sitting
alongside the existing four and depending on `peisear-core` plus an
async HTTP client. The web crate wires it in via a toggleable panel.
Provider-agnostic by design (Anthropic, OpenAI, local models via
OpenAI-compatible endpoint).

### Inline editing and optimistic updates

Requires Leptos hydration; see
[docs/guides/hydration-upgrade.md](docs/guides/hydration-upgrade.md)
for the migration path.

## Medium-term

Bigger moves that require more scaffolding but are well-scoped.

### PostgreSQL backend

Two implementation paths are on the table:

1. A `backend` feature flag on `peisear-storage` itself, swapping the
   `Pool` type alias between `SqlitePool` and `PgPool`.
2. A sibling `peisear-storage-postgres` crate with identical query
   function signatures, selected at link time.

The `Pool` alias and `StorageError` abstraction are already shaped
for either route. SQLite will remain the default for single-node
self-hosting; PostgreSQL unlocks multi-node, multi-user-at-scale
deployments.

### OIDC / IDaaS integration

Land the OIDC verifier alongside the JWT code in `peisear-auth`
behind a feature flag. The web crate grows an OIDC callback
handler; the rest of the architecture is unchanged. Supports
discovery, PKCE, and refresh flows.

### Team / organisation model

Currently a user owns their projects. A multi-user team concept
requires:

- `teams` / `team_members` tables and queries in storage.
- Scoping every existing `owner_id` to `team_id` with per-member
  role (owner / member / viewer).
- Access-control helpers in a new module, with query-level
  enforcement preserved as the second line of defence.

### Exports and imports

CSV, JSON, and GitHub-compatible Markdown. Lands in `peisear-web` as
a cluster of new handlers; the heavy lifting (SQL → struct → format)
is storage + core.

## Long-term / vision

Directional commitments, not promises.

### CI/CD and IaC support

The `infra/` directory is the staging ground. Eventually:

- **`infra/docker/`** — scratch-based container image built in CI.
- **`infra/compose.yaml`** — end-to-end local environment including
  PostgreSQL once that backend lands.
- **`infra/terraform/`** — minimal IaC for a single-node VM
  deployment with TLS, backups, and health probes.
- **`infra/github/`** — GitHub Actions workflows for test, lint,
  build, and release.
- **`infra/k8s/`** — Helm chart or kustomize overlay for clusters.

### Pluggable backends beyond relational

Object storage for attachments, a search index (Meilisearch or
PostgreSQL full-text), maybe a message queue for outbound
notifications. Each would be a new crate alongside `peisear-storage`.

### Plugin interface

A thin stable contract — probably WebAssembly component model —
letting operators add custom fields, validations, or sidebar panels
without forking peisear itself.

## Out of scope

A few things peisear has deliberately decided not to become:

- **A swiss-army project suite.** No wikis, no calendars, no chat.
  If you need them, integrate them.
- **A SaaS.** peisear is self-hosted. There will be no
  peisear.cloud.
- **A mobile app.** The web UI is responsive; a native app isn't in
  scope.

## Contributing to the roadmap

If a feature you want isn't here, or if a priority seems wrong,
please open a discussion issue. Roadmap changes are public by
design — see [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md).
