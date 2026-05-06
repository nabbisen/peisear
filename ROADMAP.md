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
  shipped in 0.5.0; period support shipped in 0.12.0.* The
  `user_capacities` table with optional `period_start` /
  `period_end` is now the source of truth; see the 0.12.0
  CHANGELOG entry for the migration story.
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
  *Phase 1 shipped in 0.10.0; Phase 2 shipped in 0.11.0.* Four
  indicators live: sustained-overload streak, stalled-assigned
  streak, estimation drift trend (median dwell-per-point recent
  vs. older half of a 28-day window), and cognitive switching
  pattern (median pickups per active day). The first two have a
  `Watch` palette ceiling; the latter two render in neutral
  palette because they are descriptive rhythm, not warnings.
  No `Concern` branch exists in any classifier.
- **Period-scoped capacity.** *Shipped in 0.12.0.* New
  `user_capacities` table replaces `users.capacity_points`; rows
  have optional `period_start` / `period_end`. Periods may not
  overlap (application-layer enforced). `/settings` UI provides
  CRUD with a "Close on date" helper for closing open-ended
  rows when adding new periods. Today's effective capacity
  resolves through `user_capacities::effective_for_user`. The
  Load chip on `/me` shows a "(this period)" hint when the
  active row has period bounds. Migration is destructive: the
  old column is dropped after data is moved into the new table.
- **Sprints**: a `sprints` table with `(project_id, name,
  starts_at, ends_at)`, prerequisite for velocity-stddev,
  burndown lines, and per-sprint completion ratios. Now that
  period-scoped capacity (0.12.0) is in place, sprint rows can
  link to capacity rows by date range and the per-sprint
  capacity is naturally computed.
- **Roles**: manager / neutral-third-party scopes with their own
  `/me/{user_id}` (or aggregated dashboard) views. These arrive
  alongside the planned Team / organisation feature so the
  permission model lands once.
- **Notification surfaces**. *Shipped in 0.13.0 (Phase 1).
  Real email delivery shipped in 0.16.0.*
  `notifications` and `notification_preferences` tables back
  an inbox at `/notifications` (with the navbar bell + unread
  badge) and a preferences page at `/settings/notifications`
  (with smart defaults and a one-question first-login email
  prompt). Three channels through a `Channel`-shaped
  abstraction: in-app via audit row; **email via the
  wasm-smtp 0.9 family (real delivery as of 0.16.0)**;
  webhook still a log stub awaiting per-user URL UI.
  Edge-triggered detection with a 24-hour cooldown.
  Architecture: snapshot loop and dispatch loop are
  independent tokio tasks connected by
  `mpsc::channel<DispatchEvent>`. The dispatch pipeline now
  lives in its own crate, `peisear-notify`, so transport
  dependencies (SMTP today, webhook HTTP client tomorrow,
  AI digests later) don't bleed into the web crate's
  compile graph.

  **Email integration**: *Shipped in 0.16.0.* Real SMTP
  delivery via the wasm-smtp 0.9 family (`wasm-smtp` core,
  `wasm-smtp-tokio` Transport adapter, `wasm-smtp-cloudflare`
  kept in tree as future option, plus `mail-builder` for
  RFC 5322 / MIME composition). Both implicit TLS (port 465)
  and STARTTLS (port 587) are supported. SMTP credentials
  read from environment (operator territory). Graceful
  degradation when unconfigured: in-app channel continues
  working; email send attempts fail with a logged warning
  and the audit row records `dispatched_via` without
  `email`.

  **Phase 2 notification candidates (deferred):**
  - HTML email (`multipart/alternative`). `mail-builder`
    makes adding HTML straightforward when a use case
    appears.
  - Connection pooling (multi-message sessions on a single
    SMTP connection — `wasm-smtp` already supports this in
    its API; we'd refactor our send path).
  - Digest mode (bundle multiple notifications per day or
    per week into one email).
  - Per-team `From:` address override.
  - Webhook channel real implementation (per-user URL UI
    + outbound HTTP client).
  - `project_trend_decline` notification kind detection.
  - Per-page nav unread badge coverage (today the badge is
    in the navbar bell only).

The foundation laid in 0.3.0–0.7.0 oriented toward this
trajectory; Phase 2 (events 0.8.0 → snapshots 0.9.0 → user
burnout 0.10.0 → drift & switching 0.11.0 → period-scoped
capacity 0.12.0 → notification surfaces 0.13.0) realises the
core of it. The remaining items above (sprints, roles,
AI assistance) are now a sequence of thin layers on top of
the foundation rather than further structural work.

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

### Team model (Phase 1: flat teams)

The 0.14.0 design adopts the **Linear-style flat team** approach
discussed in design notes: optional teams that small users can
ignore entirely. `team_id` on projects is nullable (a project
without a team is a *personal project*, owned by its creator
just as today). Users can join multiple teams; each membership
carries a `role` (`admin` / `member` / `viewer`).

Role extensibility (future):

- The 3-tier `admin / member / viewer` set is what 0.14.0
  ships. The schema reserves `role TEXT` (not `enum`) so
  future fixed roles (e.g. `billing` for paid hosted
  scenarios, `security_manager` for compliance) can be added
  without migration.
- Custom roles (per-team named roles with selectable
  capability sets) are an open design question — Linear has
  no equivalent today; GitHub Enterprise does. We'd add
  `team_roles` + `team_role_capabilities` tables only if
  operator feedback justifies the complexity. Until then,
  the three fixed values cover the design intent.

Phase 2 candidates (deferred):

- **Sub-teams** (one-level parent / nested teams). Linear
  added this in 2025; we'd consider it after observing
  operator usage of flat teams. The data model already
  reserves `team_id` for this — adding `parent_team_id`
  later is non-breaking.
- **Per-team configuration**: cycle / sprint / label
  defaults inherited by projects in the team.

### Privacy & access control evolution

V2.1 §2.5 ("集計と個別を混同しない") sets the boundary that
0.14.0 ships: project-level metrics are visible to all team
members; per-user signals (burnout panel, personal dashboard)
are visible to that user only. The team admin role is
**political** (manage members, configure team settings), not
**surveilling** (read other people's burnout / dwell time /
streak data).

Future considerations on this surface:

- **Per-team privacy policy**. A team could opt for a stricter
  posture ("burnout panels are silent across the team")
  without needing user-by-user opt-out. Default is the 0.14.0
  posture (member workload visible, individual signals
  private); admins of a team might restrict further but never
  loosen below this floor.
- **Per-user privacy controls**. A user could opt to hide
  their per-user workload chip from team members (the
  workload section on a project page would render as
  "Hidden by user" rather than the load number). The
  in-flight points are still computed and the user still
  sees their own; what's suppressed is team-mates' visibility.
- **Anonymous aggregation surface**. A project's overall
  health composes individual workload signals; a future view
  could show only the *aggregate* without naming specific
  users (median/p90 instead of per-user chips). Useful where
  the team is large enough that individual chips create
  pressure.
- **Deletion + retention semantics**. Deleting a user
  currently `ON DELETE CASCADE`s their snapshots and
  notifications. We should document and surface this as the
  user's privacy guarantee: leaving a team is a clean
  forget. Today's behaviour is correct; the surfacing is
  what's missing.
- **Audit trail visibility**. `issue_events` records actor_id
  for every event; admins should not be able to retroactively
  reconstruct who did what beyond what the issue's
  history-pane already shows. A privacy-conscious
  admin-tooling layer would expose less, not more.

These are not 0.14.0 work — the Phase 1 floor (project
metrics public to team, individual private) is the design
this release ships against. They are **the privacy decisions
we will need to make as the team feature matures**, recorded
here so they don't get lost when the team feature does start
demanding answers.

### Exports and imports

CSV, JSON, and GitHub-compatible Markdown. Lands in `peisear-web` as
a cluster of new handlers; the heavy lifting (SQL → struct → format)
is storage + core.

### Server-migration support

A planned and supported way to move an entire peisear installation
from one host to another. Distinct from *Exports and imports*
above (which targets per-project data leaving the system in
neutral formats); this is about the operator picking up the whole
state and putting it down somewhere else.

The current story is "stop the process, scp the SQLite file and
the env vars to the new box, start the process there". That works
but is undocumented as a supported path and has sharp edges
around session continuity (JWT secret) and asset paths. Concrete
work to make this a first-class feature:

- **Bundled export.** A `peisear export <path>` subcommand that
  produces a single archive (`.tar.zst` is the obvious choice)
  containing: a hot-copy of the SQLite database via
  `sqlite3 ".backup"`; a manifest with the version, schema
  revision, and a checksum; and the contents of `static/` if it
  has been customised. Specifically excludes secrets — those
  must be redeployed out-of-band, see below.
- **Bundled import.** Mirrored `peisear import <path>` that
  validates the manifest version against the running binary,
  refuses to overwrite a non-empty database without a force flag,
  and applies the archive transactionally (move the existing
  file aside, write the new one, run pending migrations, swap
  back on success).
- **Secrets-out-of-band by design.** `JWT_SECRET` is *not* in the
  bundle. Migrating the secret moves all live sessions across
  with it; rotating it forces every user to re-authenticate.
  Both are defensible, neither is universally correct. The
  documentation surfaces the choice and the operator decides.
  Argon2 password hashes are in the database itself, so the
  password file moves with the export — that's correct.
- **Encrypted archive option.** For environments where the
  archive lives on intermediate storage (S3, a desktop, a
  thumb drive), an `--encrypt-with` flag taking a public key
  produces an age-encrypted tar so the archive is not readable
  in transit. Plain-text export remains the default for
  trusted-network migrations where encryption is overhead.
- **Cross-version compatibility window.** The manifest's schema
  revision lets the new binary know whether it can run pending
  migrations on the imported database (yes within the
  forward-compatible window; refuse with a clear message if
  someone tries to import a 0.12.0 archive into a 0.11.0 binary).
- **Documentation.** A new `docs/operations/migrate-host.md`
  walks through the full sequence including the secrets
  decision, with rollback notes consistent with
  [upgrade-runbook.md](docs/operations/upgrade-runbook.md).

The deliberate alternative to a bundle command is to keep saying
"the database file is the migration unit, here's how to copy it
safely". That's documented in `backup.md` today and works. The
case for shipping a dedicated tool is mostly that it removes the
foot-guns (forgot to stop the process; copied a partial WAL;
missed the static directory). Which path we pick depends on
whether operator feedback says "the manual way is fine" or "we
keep getting bitten". Currently leaning toward the bundle
command, but holding off until at least one operator has made a
real cross-host migration and reports back.

### Deployment guide expansion in `docs/`

*Shipped in 0.10.1.* `docs/operations/` now includes
`background-jobs.md`, `data-retention.md`, `upgrade-runbook.md`,
`observability.md`, and `scaling.md`. The operations README is
organised into "Day-one" (deployment, backup, Tailwind
self-hosting) and "Day-two" (the new five) sections.

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
