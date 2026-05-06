# Roadmap

This document lays out where peisear is going, in three time
horizons. The Cargo workspace layout is deliberately designed so that
each roadmap item lands in one well-defined crate — see
[docs/architecture/crate-boundaries.md](docs/architecture/crate-boundaries.md)
for the mapping.

## Near-term

The next few minor releases.

### v2.1 spec implementation (active, multi-release)

The [v2.1 feature specification](docs/spec/peisear-feature-spec-v2.1.md)
defines a UI/UX overhaul (5-entry navigation, sub-issue
hierarchy, calendar, direct-manipulation surfaces, plus the
review-driven additions in §11.5 / §21.4 / §28.6). It rolls
out across five phases, one per minor release:

- **Phase A — Information architecture** *(0.17.0, shipped
  2026-05-03)*. 5-entry navigation (`/me` → `/today`,
  `/notifications` → `/inbox` with HTTP 308 permanent
  redirects — 308 over 301 preserves POST method on the two
  legacy POST endpoints), consolidated breadcrumb/back-link
  components, list filter+sort persistence (URL-primary,
  server view_state secondary), global search (LIKE % over
  project + open-issue scope, typeahead 8 / results 50),
  optimistic-lock contract for issue and project mutations
  (entities whose `updated_at` column predates 0.17.0) plus
  schema preparation (migration 0014) for the four
  remaining tables.
- **Phase B — Key screens** *(0.18.0, shipped 2026-05-03)*.
  `/today` panel collapsing + "what to read first" callout
  (B-1), project-health explainability (B-2), issue-detail
  edit-mode URL split with 308 legacy redirect (B-3), status
  segment UI on the detail page — display-only ahead of the
  Phase D direct-manipulation rollout (B-4). Personal-data
  API endpoints (`/api/users/{id}/burnout`,
  `/api/users/{id}/capacity`,
  `/api/users/{id}/notifications`) ship with `ApiAppError` /
  `ApiAuthUser` and §11.5 self-access enforcement (admin role
  does NOT bypass — verified by integration test). Closes the
  optimistic-lock rollout: `Sprint::updated_at`,
  `Team::updated_at`, `TeamMembership::updated_at`, and
  `CapacityRow::updated_at` are now plumbed through the
  storage SELECTs and into handler-level lock checks for
  sprint, team, membership, and capacity mutations. With
  Phase B done, every domain-entity mutation honours the
  §21.4 contract.
- **Phase C — Missing surfaces** *(in progress, 0.19.0+)*.
  - **PR1 — Sub-issue hierarchy** *(0.19.0, shipped 2026-
    05-04)*. parent_id approach per §8.3 / §8.4: nullable
    `parent_issue_id` column on `issues`, partial indices for
    top-level / parent-children query shapes, two triggers
    enforcing 1-level-only + same-project + no-self-reference
    + no-demotion-with-children. New routes
    `/projects/{id}/issues/{issue_id}/sub-issues/new` (GET
    form + POST create); detail page renders Sub-issues card
    on top-level issues with "+ Add sub-issue" affordance.
    Sprint follow-parent rule: sub-issues inherit the
    parent's sprint without a separate `sprint_issues` row;
    direct sprint assignment to sub-issues is rejected (400).
    Sprint detail listing filters to top-level only so effort
    isn't double-counted. 7 new integration tests cover the
    hierarchy, sprint-follow, and validation paths.
  - **PR2 — Sprint planning page** *(target: 0.20.0)*.
    `/teams/{slug}/sprints/{id}/plan` with backlog → sprint
    list-based assignment UI. (DnD lands in Phase D.)
  - **PR3 — Calendar surfaces** *(target: 0.21.0)*.
    `/today/calendar` (personal axis) and
    `/projects/{id}/calendar` (project axis). No team axis
    per §10.2. Read-only display in this PR; DnD is Phase D.
  - **PR4 — Inbox refinements** *(target: 0.22.0)*.
    Notification preferences UI, mark-all-read, snooze.
- **Phase D — Direct manipulation** *(target: 0.23.0)*. The
  five direct-manipulation surfaces (status click toggle,
  kanban DnD, calendar DnD, sprint-plan DnD, list reorder)
  rolled out in five sub-steps D-1 through D-5.
- **Phase E — Quality consolidation** *(target: 0.24.0)*.
  ABDD QA + Security QA. The §11.5 authorization assertions
  and §21.4 optimistic-lock assertions reach full coverage on
  all relevant endpoints. WCAG AA contrast, mobile completion
  for the four key flows, language consistency.

#### Test infrastructure (preparation, in 0.16.0 → 0.17.0)

- e2e basecost: `axum-test 20` workspace dev-dep, shared
  helpers in `peisear-web/tests/common/` (Rust convention),
  `tests/smoke.rs` covering the existing happy paths,
  `tests/auth_boundary.rs` and `tests/optimistic_lock.rs`
  with `#[ignore]`d test inventory that gets unblocked Phase
  by Phase as endpoints land.
- CI: `.github/workflows/test.yml` runs `cargo fmt`, clippy
  with `-D warnings`, each test crate in a separate runner
  job (the combined link step OOMs the default 7 GB runner),
  and `cargo build --workspace`.

#### List filter/sort future enhancements (post-Phase A)

Phase A Step 3 (0.17.0) ships a deliberately small filter/sort
vocabulary on the project-detail issue list: status, assignee,
and sort by priority / created / updated. The schema-less
`user_view_states.state_json` shape is designed to grow without
migrations — these are the candidates a future release should
consider:

- **Priority filter** — pair with the existing priority sort,
  e.g. "show only high+ priority items". Requires only a new
  query parameter and a `retain` predicate.
- **Due-date filter** — depends on Phase C adding the
  `due_date` field to issues (per spec §8.1). Once that lands,
  filters like "due this week" / "overdue" / "no due date"
  become natural.
- **Sub-issue visibility toggle** — Phase C introduces the
  parent/child issue hierarchy. The list view will need a
  toggle: "show top-level only" vs "flatten with indentation".
- **Multi-select filters** — e.g. "in_progress AND open" as
  one filter view. Requires the URL param to accept comma-
  separated values; the `apply_filter_and_sort` predicate
  becomes `contains` instead of `==`.
- **Saved query presets** — let a user name and re-select
  multiple filter combinations ("My this-week view"). This is
  a UI affordance over the same `user_view_states` storage,
  with the key extended from `project_issues:{id}` to
  `project_issues:{id}:preset:{name}`.
- **Sort direction toggle** — currently sort is one-way per
  key. Adding `?dir=asc|desc` is a small enhancement that
  doubles the available orderings.
- **Reset semantics revisit** — the current Reset link
  inherits the saved default rather than wiping it (rationale
  in 0.17.0 changelog). If user testing surfaces confusion,
  consider an explicit "Clear my saved default" affordance
  next to Reset.
- **Apply other lists**: extend the same scheme to
  `/teams/{slug}/sprints/{id}` issue table (Phase B/C scope).

#### Global search future enhancements (post-Phase A)

Phase A Step 4 (0.17.0) ships a deliberately simple global
search: LIKE `%q%` over project names and open issue titles,
fixed scope, no ranking. The endpoint surface is stable enough
that future enhancements can land underneath without breaking
the typeahead's JSON shape:

- **Result ranking** — currently results are ordered by
  `updated_at DESC`. A future revision could score by name
  match position (prefix > substring > fuzzy) and recency,
  but only after user testing surfaces a real ordering
  problem at scale.
- **Search descriptions / comments** — currently only project
  names and issue titles are searched. Description-body
  matches surface a long tail of false positives in early
  testing; revisit when issue templates settle and
  descriptions are more uniformly structured.
- **Search closed (done) issues** — Phase A excludes
  `status = done` from typeahead. A separate "Search archive"
  surface or a "Show completed too" checkbox on /search could
  open this up without polluting the live-work typeahead.
- **Search teams and sprints** — Phase A scope is
  project + open issue. Once Phase C lands sub-issues and
  the calendar, broaden search to those too.
- **FTS5 migration** — at higher data volumes (10k+ issues),
  LIKE goes from "milliseconds" to "noticeable". Migration
  to SQLite FTS5 with the trigram tokenizer (handles
  Japanese without a morphological analyser) is the natural
  upgrade. Plan the migration so the storage-layer API
  (`projects_by_name`, `open_issues_by_title`) stays the
  same — handlers and components shouldn't have to change.
- **`/` keyboard shortcut** to focus the navbar search input
  globally. A small UX win; deferred to Phase E (ABDD QA)
  where keyboard shortcuts are reviewed holistically.
- **Recent searches** — server-side per-user history with a
  small dropdown on focus. Useful for quickly re-running a
  filter; the storage shape (key/value JSON) is already
  there in `user_view_states`.
- **Search on mobile** — the navbar input is hidden below
  the `sm` breakpoint in 0.17.0 because it competes with
  the user menu for space. Phase E mobile QA will rework
  this (likely a drawer-mounted search).

#### Optimistic-lock future enhancements (post-Phase A/B)

0.17.0 ships optimistic-lock for issue and project
mutations using `updated_at` as the version field. 0.18.0
extends it to sprint / team / team-membership / capacity.
Beyond that:

- **Conflict-resolution UI** — today, a 409 surfaces an
  error page asking the user to refresh and re-apply.
  A future revision could render a side-by-side diff
  ("their version" / "your edit") and let the user merge.
  Useful for long-form fields like issue descriptions; less
  useful for status/priority where the right move is
  usually "their change wins, mine moves on".
- **Toast notification for conflict on direct-manipulation
  surfaces** — Phase D's drag-and-drop and inline edits
  produce 409s the user can't reasonably anticipate. A
  toast with "Refresh — someone changed this" is a better
  UX than an error page when the user didn't even click
  Save.
- **`/api/*` JSON conflict response** — the `IntoResponse`
  for `AppError::OptimisticLockConflict` currently always
  emits HTML. The structured shape from spec appendix
  E.3.3 (`{error, message, current_updated_at,
  entity_type, entity_id}`) is wired into the type already;
  Phase B/D will plumb an `ApiAppError` sibling type for
  `/api/*` mutation handlers so conflicts there return JSON
  for client-side handling.
- **Same-second race window** — SQLite's `CURRENT_TIMESTAMP`
  is whole-second precision, so two writes inside the same
  second land with equal `updated_at` values and the lock
  can't tell them apart. This is rare in practice but not
  zero. Mitigation options: (1) switch to `strftime('%s%f')`
  for sub-second precision in the trigger, (2) add an
  explicit `version BIGINT` column that increments on each
  UPDATE. The integration tests use a 1.1s sleep
  (`ensure_distinct_timestamp`) to side-step the issue;
  production traffic almost never sees it. Revisit if user
  reports surface concrete instances.
- **Lock for delete operations** — 0.17.0 doesn't gate
  DELETE endpoints (issue delete, project delete) on
  `client_updated_at`. Their contention model differs:
  the user clicked Delete deliberately, and a stale read
  on a row about to be deleted just raises NotFound on the
  next mutation rather than overwriting work. Adding the
  lock here would convert a NotFound into a Conflict, which
  is arguably more accurate but doesn't add user-visible
  value. Revisit if there's evidence that delete races
  cause real surprise.

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

### Search refinement (future revisit)

The Phase A search shipping in 0.17.0 is deliberately the simplest
working version: SQL `LIKE %query%` against `projects.name`,
`projects.description`, and open issues' `title` / `description`
columns; typeahead returning 8 hits, results page returning 50
with pagination. This is enough to make peisear navigable while
the project's scale remains in the "individual contributor +
small team" regime the product is designed for.

Three threads to revisit when the product or its scale changes:

- **Search engine**: SQLite **FTS5** is the obvious next step
  if `LIKE %…%` becomes too slow or misses too much. Adopting
  it adds three things: a virtual-table schema mirror per
  searchable entity, INSERT / UPDATE / DELETE triggers to keep
  the mirror in sync, and a tokenizer choice. The tokenizer is
  the real decision: the default whitespace tokenizer doesn't
  index Japanese (no spaces, so an entire issue body becomes
  one token), so Japanese support requires either the built-in
  `trigram` tokenizer or an external morphological analyzer
  (mecab / lindera). `trigram` is the lower-cost path and is
  what to plan for first, with the constraint that queries
  shorter than 3 characters won't hit the index. We also need
  to confirm Cloudflare D1's FTS5 support if peisear ever runs
  on D1 — sqlite extension support there has been a moving
  target. Not a near-term need; revisit when issue counts cross
  the order of 10⁵ per project, or when users start asking for
  ranked relevance instead of substring matching.

- **Search scope**: Phase A intentionally limits search to
  projects and *open* issues, not closed issues, sub-issues,
  sprints, or teams. The scope was chosen to keep the
  typeahead responsive and to match the most common query —
  "what was that issue about X". Revisit as user feedback
  accumulates: closed-issue search is the most-likely-asked
  extension; sprint and team search would matter once a single
  account has dozens of either. Sub-issue search becomes
  meaningful after Phase C lands the sub-issue hierarchy.

- **Result-count defaults & UI/UX**: typeahead 8 / results
  page 50 is a starting point chosen by gut, not measurement.
  Things to revisit once we have real usage signal: whether
  typeahead 8 is a useful preview or just truncation noise,
  whether the results page benefits from facets (project,
  status, assignee), whether keyboard navigation + recent
  history needs surface real estate beyond the current
  dropdown, and whether mobile typeahead deserves its own
  treatment rather than the desktop layout shrunk down. None
  of this is urgent; flagged so it isn't forgotten under
  "we shipped search, done".

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
