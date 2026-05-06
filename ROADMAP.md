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

- **Per-issue effort estimates** — *shipped in 0.3.0.* Storage migration
  `0002_issue_effort.sql` adds `effort INTEGER NULL CHECK (effort > 0)`
  on `issues`; `core::Issue` carries an `Option<i64>` field; web
  renders a `<select>` of Fibonacci-scale presets (1, 2, 3, 5, 8, 13)
  with `—` as "not estimated" on the new/edit forms, plus a compact
  badge on board cards and a column in list view.
- **Per-issue assignee** — *shipped in 0.4.0.* Storage migration
  `0003_issue_assignee.sql` adds `assignee_id TEXT NULL` with
  `ON DELETE SET NULL` against `users(id)`; new storage query
  `list_assignee_candidates` returns the eligible users for a project
  (today: the project owner; when team support lands, all team members);
  `core::Issue` carries an `Option<String>`; web renders an assignee
  selector on new/edit forms, a ghost badge on cards, and an
  "Assignee" column in list view.
- **Per-period capacity limits per assignee** — *capacity primitive
  shipped in 0.5.0; period support deferred.* Users now have a global
  `capacity_points` value (per-user, not per-period); the project
  detail page shows a per-user workload strip with `Healthy` /
  `Strained` / `Overloaded` colour coding and inline `WorkloadHint`
  on issue forms. Saves are soft-warnings, not hard blocks. Period
  support (sprint / week / month-scoped capacities) is the next
  iteration on this primitive: see migration comment in
  `0004_user_capacity.sql` for the planned `user_capacities` table.
  Planned for 0.6.0 or 0.7.0 depending on demand.
- **Project-health score** — *shipped in 0.6.0.* Three indicators
  (Throughput, Oldest in-flight, Activity-14d) rendered as a strip
  on the project detail page, sharing a `HealthIndicator` palette
  (`Good` / `Watch` / `Concern`) with the workload chips. Pure
  SQL aggregates over `issues`, no migration. The deliberately
  *non*-shipped item is a single composite 0–100 score: forming
  one would require uncalibrated weights, and shown numbers tend
  to be optimised against rather than understood.

### Health & burnout extension (active)

Per the V2.1 brief, this is the current concentrated investment.
Phase 1 ships in **0.7.0** (composite project-health score, three
new health indicators, personal dashboard, WIP limits,
accessibility uplift). Phase 2 and beyond:

- **`issue_events` table** for precise transition history.
  *Shipped in 0.8.0.* Append-only log written transactionally
  alongside every issue mutation. Long-stale detection and
  personal estimation skew now use event-based dwell time
  with graceful fallback to the 0.7.0 calendar-time
  approximation for legacy issues.
- **`metrics_snapshots` table + nightly aggregation** to give
  trends. *Shipped in 0.9.0.* Background tokio task runs every
  six hours, captures one snapshot per active project. Trend
  uses the median of recent (7-14 day) snapshots as the past
  baseline. `Trend::Up { delta } / Down { delta } / Flat /
  Unavailable` rendered next to the composite score.
- **`user_burnout` storage + core module** sibling to
  `personal_metrics`, sharing the `HealthIndicator` palette.
  Likely signals: sustained-overload streak, stalled-assigned
  streak, unbalanced load distribution across the team.
- **Period-scoped capacity** (the deferred half of the 0.5.0
  work): a `user_capacities` table with optional
  `period_start`/`period_end` so capacity follows sprint cadence.
- **Sprints**: a `sprints` table with `(project_id, name,
  starts_at, ends_at)`, prerequisite for velocity-stddev,
  burndown lines, and per-sprint completion ratios.
- **Roles**: manager / neutral-third-party scopes with their own
  `/me/{user_id}` (or aggregated dashboard) views. These arrive
  alongside the planned Team / organisation feature so the
  permission model lands once.
- **Notification surfaces**: where the warnings actually reach
  the person — beyond the page they happen to be looking at.
  Settings page toggle, dashboard widget, optional email digest.
- **AI-assisted warnings**: the new `peisear-ai` crate consumes
  the `ProjectHealthReport` and `PersonalMetrics` shapes and
  produces narrative summaries / suggestions.

The foundation laid in 0.3.0–0.7.0 is oriented toward this
trajectory. The next concrete step on the path is the Phase 2
events table; everything else is then a thin layer on top.

### AI assistant per user

A per-user helper that can summarise issues, suggest labels, and
draft responses. Lands as a new `peisear-ai` crate sitting
alongside the existing four and depending on `peisear-core` plus an
async HTTP client. The web crate wires it in via a toggleable panel.
Provider-agnostic by design (Anthropic, OpenAI, local models via
OpenAI-compatible endpoint).

### Inline editing and optimistic updates

*Deferred.* Two technical paths exist:

1. **Leptos hydration**: full reactive client. Requires a wasm32
   build target, a client-side bundle, and a substantial change to
   the way `peisear-web` ships assets. Highest fidelity, highest
   complexity. See [docs/guides/hydration-upgrade.md](docs/guides/hydration-upgrade.md)
   for the migration path that was sketched.
2. **HTMX-style hypermedia partials**: server returns small HTML
   fragments on edit. Single ~12KB JS dependency, no WASM toolchain,
   straightforward additive endpoints. Lower fidelity (no shared
   client state across cells), much lower complexity. Future-compatible
   with path 1 since the markup stays HTML-attribute-driven.

The decision between the two is postponed in favour of the
**Health & burnout extension** above, which has been raised to the
top of the work queue per the user's direction. Inline editing
returns to the queue once health/burnout is in users' hands.

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
