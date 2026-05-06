# Changelog

All notable changes to peisear are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] — 2026-04-27

### Added

- **Project-health trend over time.** `<HealthStrip>`'s score
  badge now renders an arrow + signed delta — `↑ +6`, `↓ -3`, or
  a flat `→` — comparing today's composite score against the
  median of recent past scores. Trend is hidden entirely
  (`Trend::Unavailable`) when no past data exists, e.g. on a
  fresh install or before the first snapshot tick.
- **`metrics_snapshots` table** — a small append-only history of
  project-level health captured periodically by a background
  task. Stores all nine `ProjectHealthRaw` fields plus the
  composite `score_value` so historical comparisons reflect the
  user's lived experience even if `HealthWeights` changes in a
  future release.
- **Background job runner.** A new `peisear-web::jobs` module
  spawns long-running tokio tasks alongside the web server. The
  first such task is the snapshot writer, which wakes every six
  hours and writes one snapshot per project with at least one
  issue. Failure-tolerant (per-project errors logged, loop
  continues), cooperative-shutdown (oneshot channel from main),
  and lightweight (one tick = one project list query + one
  small INSERT per project).
- New core symbols:
  - `peisear-core::project_health::compute_report_with_trend`
    accepts a slice of past scores and fills `HealthScore.trend`.
  - `peisear-core::project_health::classify_trend` — pure
    function over `(current, &[past])` returning `Trend`.
  - Constants: `TREND_FLAT_THRESHOLD = 5`,
    `TREND_PAST_WINDOW_MIN_DAYS = 7`,
    `TREND_PAST_WINDOW_MAX_DAYS = 14`.
- New storage module `peisear-storage::metrics_snapshots` with
  `insert`, `recent_for_project` (two-bound window), and
  `projects_with_recent_issue_activity` (writer's candidate
  list).

### Design

- **Median, not mean, of past scores.** A single noisy past
  point — a one-off bad day — would skew a mean-based baseline
  more than the rest of the week's evidence justifies. Median
  is the standard fix; with the lazy-snapshot story this
  release also relies on, sample counts can be small (1-3 over
  a week), and median behaves sensibly all the way down to one
  point (it returns the point itself).
- **7-14 day window deliberately *excludes* the very recent
  past.** A baseline that includes today is not a baseline; it
  collapses the trend to current-vs-current and never moves.
  The 7-day lower bound rejects weekend artefacts (Friday vs.
  Thursday score drops are usually pause-driven, not work
  quality). The 14-day upper bound keeps the comparison timely.
- **Trend is rendered without colour.** Improvement and decline
  are facts about change; the score state (Good / Watch /
  Concern) already conveys absolute health with colour. Putting
  green on "score went up" would imply moral approval of
  movement, which gets uncomfortably close to the
  performance-evaluation framing V2.1 §0.2 explicitly disallows.
  The arrow + signed number is information; the user
  contextualises it.
- **`score_value` column denormalised in snapshots.** Storing
  only the raw inputs would mean today's weights are applied
  retroactively to yesterday's data. That is the wrong behaviour:
  the trend should reflect "what the user saw a week ago"
  vs. "what they see now", not a re-scoring narrative. V2.1
  §4.4 explainability is on this side of the line.
- **Background tokio task instead of cron / scheduler.**
  peisear's deployment story is one binary; pulling in cron
  would change that. Lazy-on-page-load was the alternative,
  but the user direction here was that a background job has
  better long-term extensibility — future per-user burnout
  snapshots, optional notification dispatch, and cleanup tasks
  all fit the same shape. The runner is built so adding a
  sibling task is one call.
- **Snapshots are project-level only.** No per-user data goes
  into this table. Per the V2.1 brief §0.2 ("常時監視を目的と
  しない") and §2.5 ("集計と個別を混同しない"), per-user
  history belongs in a separate user-burnout snapshot table
  (planned 0.10.0) with its own access-control story.

### Changed

- Workspace and all five crates bumped to `0.9.0`.
- `peisear` binary now spawns the snapshot job at startup. The
  oneshot shutdown sender is held by `main` and dropped at
  process exit, signalling cooperative shutdown.
- `peisear-web::handlers::issues::project_detail` now fetches
  `metrics_snapshots::recent_for_project` and threads the past
  scores into `compute_report_with_trend`.
- `peisear-storage::metrics_snapshots::recent_for_project` takes
  two bound parameters (`min_days_ago`, `max_days_ago`) so
  callers can specify a "between N and M days ago" window
  rather than a single "past N days" range.

### Deferred (Phase 2 final piece)

- **`user_burnout` module** — sustained-overload streak,
  stalled-assigned streak, cognitive-switching frequency.
  Builds on both this release's snapshot table (point-in-time
  history) and 0.8.0's event log. Targeted for **0.10.0**.

## [0.8.0] — 2026-04-27

### Added

- **`issue_events` table** — append-only log of issue mutations.
  Every issue create / update / status change / assignee change /
  effort change / delete now writes a corresponding event row in
  the same SQL transaction, recording the actor (current user),
  the previous value, and the new value. Foundation for Phase 2
  of the Health & Burnout extension; see migration
  `0006_issue_events.sql` for the schema discussion.
- **Event-aware long-stale detection.** `<HealthStrip>`'s
  long-stale indicator (project-level) and `/me`'s long-stale
  chip (personal) now use the latest `status_changed` event's
  `occurred_at` as the staleness clock, falling back to
  `updated_at` only when no event log exists for the issue (i.e.,
  pre-0.8.0 issues). Priority bumps and other non-status edits no
  longer reset the staleness clock.
- **Event-aware personal estimation skew.** `/me`'s "Pace" chip
  now reports days-per-point using actual `in_progress` dwell
  time reconstructed from the event log, replacing the calendar
  time approximation that conflated active work with weekends and
  pauses. Issues with no event log fall back to the calendar
  approximation, so legacy data continues to contribute the same
  number to the average.
- New storage module `peisear-storage::issue_events` with:
  - `insert_event` for transactional event writes.
  - `days_since_last_status_change_per_in_flight_issue` for
    per-issue staleness queries.
  - `in_progress_seconds_for_issue` reconstructing dwell time
    from the status_changed timeline (handles still-in-progress
    issues by clipping the open window at "now").
  - `kind` submodule with `&'static str` constants for each
    event type, so call sites get compile-time spelling checks.

### Design

- **Why an event log now.** Phase 1 derived staleness and
  estimation signals from `issues.{updated_at, status}`. That left
  three documented limitations: priority bumps reset the staleness
  clock; cycle time was uncomputable; estimation skew included
  calendar pauses. The event log resolves all three without
  redesigning the user-facing surface.
- **Issue deletion preserves history.** `issue_id` is
  `ON DELETE SET NULL` rather than cascade-delete, so the event
  log is a monotonically-appendable record. The `deleted` event
  itself is written before the cascade fires, recording the
  issue's last-known status. The `project_id` denormalisation in
  the event row keeps deleted-issue events queryable without the
  issue join.
- **`actor_id` is recorded from day one.** Today's owner-equals-
  self model means actor_id is always the project owner, so the
  field is informational rather than discriminating. But making
  the field optional and unindexed for now would mean a breaking
  migration when manager / neutral-third-party roles arrive in a
  future release. Better to ship the field on day one.
- **No backfill of synthetic events for pre-0.8.0 issues.** Only
  `created_at` is real for legacy issues; status transition
  timestamps are unknown. Inventing events would be a lie that
  would contaminate the very metrics this release is trying to
  improve. Instead, the query layer falls back to the 0.7.0
  approximation when no events exist for an issue. Precision
  improves naturally as users work, with no manufactured data.
- **Transactional writes (selected over SQL triggers).** Triggers
  would be more bullet-proof against future call sites that
  forget to write events, but they cannot record the actor (the
  DB doesn't know who made the request). Application-level
  writes inside a transaction give us actor tracking and clearer
  debugging at the cost of having to remember to write the event
  whenever a mutation happens. The four mutating functions in
  `peisear-storage::issues` are the only places this matters, and
  they all live in one file.

### Changed

- Workspace and all five crates bumped to `0.8.0`.
- `peisear-storage::issues::{insert, update, update_status,
  delete}` are now transactional (`BEGIN; UPDATE; INSERT events;
  COMMIT;`) and take an `actor_id: &str` parameter so callers
  must pass who made the change. Web handlers now pass the
  authenticated user's id.
- `peisear-storage::project_health::for_project`'s long-stale
  subquery now uses `COALESCE` between the latest
  `status_changed` event time and `updated_at`, picking event
  data when present.
- `peisear-storage::personal_metrics::{for_user_in_project,
  for_user_global}` use a new private helper
  `active_estimation_skew` that reconstructs in_progress dwell
  time per issue from the event log, with the same
  calendar-time fallback for legacy issues.

### Deferred (Phase 2 continued)

- **Trend over time** (the `Trend::Up { delta }` etc. side of
  `HealthScore.trend`) — needs `metrics_snapshots` table, which
  ships in **0.9.0**.
- **`user_burnout` module** — sustained-overload streak,
  stalled-assigned streak, cognitive-switching frequency. These
  build on both the event log (this release) and the snapshot
  table (next release), so they belong in **0.10.0**.

## [0.7.0] — 2026-04-26

### Added

- **Composite project-health score (0–100)** with progressive disclosure.
  The project detail page header now shows a single score plus a one
  or two sentence natural-language summary that foregrounds whichever
  indicators are pulling the score down. The per-indicator
  breakdown is collapsed inside `<details>` for users who want the
  numbers — the default board-watching experience stays uncluttered.
- **Three new health indicators**, lifting the total from three to six:
  - **Bus factor** — share of in-flight work concentrated on the
    single most-loaded user; flags single-point-of-failure risk
    in the team. Solo projects render `solo` and a neutral-low
    score rather than alarming red.
  - **Long-stale** — share of in-flight issues that have not been
    touched in over two weeks (reuses
    `ACTIVITY_WINDOW_DAYS` for symmetry with the Activity
    indicator).
  - **WIP compliance** — count of users currently over their
    effective WIP limit, against the count of active assignees.
- **Personal dashboard** at `/me`. Shows the authenticated user
  their current WIP vs. limit, current effort load vs. capacity,
  recent throughput, long-stale assigned issues, and a coarse
  estimation skew (calendar days per story point on recent done
  issues). Visible only to the user themselves; the manager and
  neutral-third-party scopes the V2.1 brief calls for arrive with
  the planned Team feature. Includes a built-in `<details>` "What
  do these mean?" explainer.
- **WIP limits** with a three-tier resolution path:
  1. `users.wip_limit` if set (personal override, on `/settings`),
  2. else `projects.wip_limit_default` if set,
  3. else the system default of 3 (per the V2.1 brief §1.2).
  Surfaced as a soft warning, not a hard block — the V2.1 brief
  §0.2 explicitly forbids the system from "強制実行".
- New storage migration `0005_personal_limits.sql` adds
  `users.wip_limit` and `projects.wip_limit_default`.
- New core types:
  - `peisear-core::project_health::{ProjectHealthRaw, ProjectHealthReport,
    HealthScore, Trend, Indicator, IndicatorKind, HealthWeights}`,
    six normalisation functions, six classification functions,
    and `compute_report` / `compute_report_with_weights` /
    `summarize`.
  - `peisear-core::personal_metrics::{PersonalMetrics,
    DEFAULT_WIP_LIMIT, classify_wip, classify_long_stale}`.
- New storage modules:
  - `peisear-storage::personal_metrics` with
    `for_user_in_project` and `for_user_global`.
  - Existing `peisear-storage::project_health` extended to compute
    all nine raw inputs in two round-trips.
- New web routes: `GET /me`, `POST /settings/wip-limit`.
- **Accessibility:** every indicator chip now pairs its colour-coded
  badge with a glyph (✓ / ⚠ / ✗ / —) and an `aria-label` that
  reads "{label}: {value} ({state}). {description}". Colour is no
  longer the sole conveyor of state, per the V2.1 brief §3.4.

### Design

- **Why a composite score now**, after deliberately not shipping
  one in 0.6.0: the V2.1 brief §1.1 explicitly calls for a
  weighted-sum overall score with explanation, and §0.3
  ("Insight on Demand") is satisfied by collapsing the breakdown.
  The 0.6.0 concern about Goodhart effects is mitigated by always
  rendering the summary sentence ("Bus Factor is high; Activity is
  low.") alongside the number, so the team does not see a bare
  metric to optimise against.
- **`Insufficient` indicators excluded from the score denominator.**
  An empty project has no signal to read; a brand-new project
  shouldn't score 50% just because the empty fields normalise to
  neutral 0.5. The composite score uses only indicators with real
  data.
- **Composite score `Insufficient` floor.** When every indicator
  is `Insufficient` (very early projects), the score returns 50
  rather than crashing or showing 0. The `<HealthStrip>` further
  hides itself entirely when `total_issues == 0`, so this floor
  matters only for edge cases.
- **WIP limit and capacity are distinct concepts** that intentionally
  coexist. WIP is about cognitive load (count of things actively
  in flight); capacity is about effort budget (story-point total).
  A user can violate either independently. The `/me` dashboard
  surfaces both side by side.
- **The `Indicator` shape is uniform across all six metrics,
  collected via `ALL_INDICATORS` iteration.** Adding a seventh
  indicator in a future release means: add a variant to
  `IndicatorKind`, a normalisation arm, a default weight, the
  raw input field — *not* changing UI code or the report
  rendering. This was an explicit user requirement during the
  V2.1 design discussion.
- **Estimation skew uses `(updated_at - created_at) / effort`,
  which is calendar time rather than active-on-issue time.** This
  is a coarse Phase 1 approximation. Phase 2's `issue_events`
  table will replace it with the actual `in_progress → done`
  elapsed time. The `<details>` explainer on `/me` flags this so
  users do not over-interpret the number.
- **Trend is `Trend::Unavailable` in 0.7.0.** No history is stored
  (Phase 2 will add `metrics_snapshots`), but the field exists in
  `HealthScore` so that future trend display does not change the
  consumer signature.

### Changed

- Workspace and all five crates bumped to `0.7.0`.
- `peisear-storage::project_health::for_project` now returns
  `ProjectHealthRaw` (structurally a superset of the old
  `ProjectHealth`). The old name remains as a `pub type` alias so
  call sites that referenced it still compile.
- `render_project_detail` now takes a `ProjectHealthReport` (the
  full computed report) rather than the raw struct. The web
  handler wraps the storage call with `compute_report(raw)` so
  the storage layer stays purely about data extraction.

### Deferred (Phase 2)

The V2.1 brief covers more than this release ships. Postponed:

- `issue_events` table for precise transition history; Phase 1
  derives signals from `issues.{updated_at, status}`.
- Sprint / period-scoped capacities; Phase 1 uses rolling 14-day
  windows.
- Manager / neutral-third-party roles with their own scoped
  views. Phase 1 keeps the existing owner/self model and
  documents the intended split.
- Trend over time (history snapshots, `Trend::Up { delta }` etc.).
- Focus-time estimation.
- AI-assisted warnings (lands with the planned `peisear-ai` crate).
- The improvement-suggestion engine beyond the
  "summary sentence" we ship today.

These are spelled out in the ROADMAP "Health & burnout extension"
section so the next phase has a concrete starting point.

## [0.6.0] — 2026-04-26

### Added

- **Project-health indicators.** Three at-a-glance signals shown
  above the workload strip on the project detail page:
  - **Throughput** — share of issues that have reached `done`.
  - **Oldest in-flight** — age in days of the oldest issue still
    `open` or `in_progress`.
  - **Activity (14d)** — issues created or finished in the last
    14 days.
- New `peisear-core::HealthIndicator` enum (`Insufficient` / `Good`
  / `Watch` / `Concern`) shared by the indicator family. Same
  three-step palette will be reused by the planned per-user
  burnout indicators.
- New `peisear-core::project_health` submodule containing the
  `ProjectHealth` struct, `ACTIVITY_WINDOW_DAYS` constant, and three
  pure classification functions (`classify_throughput`,
  `classify_staleness`, `classify_activity`). Thresholds are
  hard-coded (60% / 30% for throughput, 14d / 28d for staleness,
  5+ / 1+ for activity); configurability is a future refinement.
- New `peisear-storage::project_health::for_project` query: a single
  round-trip that aggregates total / done / oldest-in-flight /
  recent-activity from `issues` using SQLite `julianday()` for date
  arithmetic. No migration — pure read-side feature.
- New `<HealthStrip>` web component sitting above `<WorkloadStrip>`
  on the project detail page. Empty projects render a single muted
  "no issues yet" note rather than three "Insufficient" chips.

### Design

- **No single 0–100 score.** Three named indicators with semantic
  colour coding, by deliberate choice. A composite score would
  require weights I have no calibrated basis for, and once a number
  is shown people start optimising for the number rather than the
  underlying work. The team forms its own mental summary from the
  three signals.
- **Module layout for future user_burnout.** The
  `peisear-core::project_health` submodule and
  `peisear-storage::project_health` module are deliberately scoped
  to leave room for a sibling `user_burnout` module that will
  surface per-user fatigue signals (sustained overload streak,
  stalled assigned work, unbalanced load distribution). The
  `HealthIndicator` enum is shared, so future UI can render both
  families with the same colour palette.

### Changed

- Workspace and all five crates bumped to `0.6.0`. The change is a
  pure UI/read-side addition (no migration, no breaking core type
  changes other than the new public `HealthIndicator` symbol), but
  the visible feature surface earns a minor bump.
- `render_project_detail` (and `ProjectDetailPage`) now take a
  `ProjectHealth` parameter alongside the existing workload list.

## [0.5.0] — 2026-04-26

### Added

- **Per-user workload capacity.** The third workload-fairness primitive
  after effort estimates (0.3.0) and assignee (0.4.0). Each user now
  has an optional capacity in story points; project pages show how
  full each person's plate is, with colour-coded warnings when load
  approaches or exceeds capacity:
  - Storage migration `0004_user_capacity.sql` adds a
    `users.capacity_points INTEGER NULL CHECK (capacity_points > 0)`
    column. NULL means "not set, do not warn" — opt-in.
  - `peisear-core::User.capacity_points: Option<i64>`, plus three new
    public types: `UserLoad { user_id, display_name, in_flight_points,
    capacity_points, in_flight_issues }`, `WorkloadState` enum
    (`Unmonitored` / `Healthy` / `Strained` / `Overloaded`), and
    pure-function `workload_state(&UserLoad) -> WorkloadState`.
    Thresholds 80% (Strained) and 100% (Overloaded) are hard-coded
    by design for this release; configurability is a future refinement.
  - New storage query `peisear-storage::issues::project_workload`
    aggregates per-user in-flight effort and issue count for a project.
    SQLite `LEFT JOIN` so users with no in-flight work appear with
    zero counts.
  - New storage function `peisear-storage::users::set_capacity` for
    the settings form to call.
  - New web routes `GET /settings` and `POST /settings/capacity`
    backed by a new `peisear-web::handlers::settings` module and
    `peisear-web::components::settings::SettingsPage` component.
    The form rejects zero and negative values; empty string clears
    the capacity (opts back out).
  - "Settings" link added to the navbar dropdown alongside "Sign out".
  - Project detail page renders a new `<WorkloadStrip>` of per-user
    chips above the board / list view, each chip showing
    `Alice 7/10 pt` with a `success` / `warning` / `error` badge per
    [`WorkloadState`]. Users with no capacity set render with a
    neutral ghost badge and `7 pt · no limit`.
  - Issue new and edit forms render a compact `<WorkloadHint>` line
    summarising current per-user load, so the editor sees the
    consequence of an assignment choice in context.

### Design

- **Soft warnings, not hard blocks.** Saving an issue that pushes a
  user over capacity is allowed. The colour-coded chip surfaces the
  fact, and the team decides. Hard-blocking the save would create
  workflows that route around the system rather than respect it.
- **Global capacity for now.** A single integer per user, not
  per-project. This tells the truth across multi-project participation;
  the alternative ("each project shows me green at 8 pt, but my
  actual load across three projects is 24") understates load. When
  period-scoped capacities land in a future release, the planned
  migration is documented inline in `0004_user_capacity.sql`:
  `users.capacity_points` becomes a row in a new `user_capacities`
  table with optional `period_start`/`period_end`. The
  [`UserLoad`] struct shape stays the same — only the storage query
  that produces it gains a period filter.

### Changed

- Workspace and all five crates bumped to `0.5.0`. User-visible
  domain extension warrants a minor bump.
- Render functions
  (`render_project_detail`, `render_issue_new`, `render_issue_detail`)
  now take a `Vec<UserLoad>` parameter alongside the existing
  assignee list, so workload context can render in the page header
  and inline near forms without each component re-querying the DB.

## [0.4.0] — 2026-04-26

### Added

- **Per-issue assignee.** Issues can now be assigned to a user, the
  second workload-fairness primitive after effort estimates and the
  upstream step toward per-period capacity limits (planned for 0.5.0):
  - `peisear-core::Issue.assignee_id: Option<String>` plus a new
    `AssigneeOption { id, display_name }` DTO for UI selectors.
  - Storage migration `0003_issue_assignee.sql` adds an
    `assignee_id TEXT NULL REFERENCES users(id) ON DELETE SET NULL`
    column on `issues`, with an index for future per-user queries.
    `ON DELETE SET NULL` is deliberate: removing a user must not
    cascade-delete their issues; ownership returns to the pool.
  - New storage query `peisear-storage::issues::list_assignee_candidates`
    returns the users eligible for assignment in a given project.
    Today that is just the project owner (single-tenant model);
    when team / organisation support lands (Medium-term roadmap),
    this function will broaden without callers changing.
  - `peisear-storage::issues::insert` and `update` accept an
    `Option<&str>` assignee parameter; SELECT queries read it.
  - Web form `<select name="assignee_id">` on new and edit issue
    pages; empty string maps to `None`. `validate_assignee` strictly
    rejects unknown ids (HTTP 400) rather than silently coercing —
    the alternative would lose user-submitted data.
  - Board card and issue detail show a small "Assignee" ghost badge
    when set; list view gains an "Assignee" column rendering `—`
    for unassigned issues.

### Changed

- Workspace and all five crates bumped to `0.4.0`. Per SemVer the
  user-visible domain extension warrants a minor bump.
- Render functions
  (`render_project_detail`, `render_issue_new`, `render_issue_detail`)
  now take a `Vec<AssigneeOption>` parameter so components can
  resolve `assignee_id → display_name` for cards, list rows, and the
  detail header without each component making its own DB call.

## [0.3.0] — 2026-04-25

### Added

- **Per-issue effort estimates.** Issues now carry an optional
  story-point effort estimate, exposed throughout the stack:
  - `peisear-core::Issue.effort: Option<i64>` and the
    `EFFORT_PRESETS = &[1, 2, 3, 5, 8, 13]` constant for UI use.
  - Storage migration `0002_issue_effort.sql` adds an
    `effort INTEGER NULL CHECK (effort IS NULL OR effort > 0)`
    column on `issues`. Existing issues remain unchanged with
    `effort = NULL`.
  - `peisear-storage::issues::insert` and `update` accept an
    `Option<i64>` effort parameter; SELECT queries read it.
  - Web form `<select>` on new/edit issue pages, with the empty
    string mapping to `None`. The edit form preserves any non-preset
    legacy value as an extra option so saves are non-destructive.
  - Board card shows a compact `N pt` outline badge when set;
    list view gains an "Effort" column rendering `—` when not
    estimated; issue detail page shows the badge inline with status
    and priority.

### Changed

- Workspace and all five crates bumped to `0.3.0`. Per SemVer the
  user-visible domain extension warrants a minor bump even though
  the change is technically additive on the wire.

## [0.2.3] — 2026-04-25

### Added

- New facade crate `peisear` at `crates/peisear/`. It is the
  crates.io public entry point: `cargo install peisear` installs the
  runnable server, and the `peisear` library re-exports the four
  implementation crates as `peisear::core`, `peisear::auth`,
  `peisear::storage`, and `peisear::web`.
- Per-crate `README.md` files for each of the five crates, with
  crates.io / docs.rs / deps.rs badges and crate-specific
  descriptions. `readme = "README.md"` declared in each sub-crate
  manifest so crates.io picks up the correct README per crate.
- Workspace-wide crates.io publishing metadata in `[workspace.package]`:
  `description`, `repository`, `documentation`, `categories`,
  `keywords`. Sub-crates inherit these via `*.workspace = true`.

### Changed

- The `[[bin]] name = "peisear"` target moved from `peisear-web` to
  the new `peisear` facade crate. `peisear-web` is now library-only.
- The user-facing run command is now `cargo run --release -p peisear`
  (was `-p peisear-web`). Documentation updated throughout.
- Workspace inter-crate version pins bumped to `0.2.3`,
  matching `[workspace.package].version`.
- `thiserror` workspace pin updated from `1` to `2`.
- README heading capitalised to "Peisear".

## [0.2.2] — yanked

A draft release that was published with an incomplete facade
scaffold (empty `lib.rs`, missing workspace dependency entry).
Users should skip 0.2.2 and use 0.2.3 instead. The version number
on crates.io is unavailable for re-use per crates.io policy; the
scope originally planned for 0.2.2 has been shipped under 0.2.3.

## [0.2.1] — 2026-04-24

### Added

- `docs/` tree organised by reader intent: `getting-started/`,
  `architecture/`, `operations/`, `security/`, `guides/`. Each
  section has its own `README.md` landing page.
- Root-level governance files: `CHANGELOG.md`, `ROADMAP.md`,
  `NOTICE`, `TERMS_OF_USE.md`.
- Community health files in `.github/`: `SECURITY.md` and
  `CONTRIBUTING.md`.

### Changed

- `README.md` slimmed to hero section, overview, quickstart,
  features, and design notes. Detailed content migrated into `docs/`.
- Licence simplified from `MIT OR Apache-2.0` to `Apache-2.0`. The
  two licence files `LICENSE-MIT` and `LICENSE-APACHE` are replaced
  by a single `LICENSE` containing the Apache-2.0 terms.

### Removed

- `LICENSE-MIT` (see above).

## [0.2.0] — previous release

### Added

- Cargo workspace layout with four crates: `peisear-core`,
  `peisear-auth`, `peisear-storage`, `peisear-web`.
- Leptos 0.8 in SSR-only mode as the template engine.
- `infra/` directory reserved for CI/CD and IaC artifacts.

### Changed

- Project renamed to **peisear**; binary renamed to `peisear`.
- axum upgraded from 0.7 to 0.8 (path syntax `/:id` → `/{id}`,
  removal of `#[async_trait]` on `FromRequestParts`).
- Error handling split into three layered types: `AuthError` in
  `peisear-auth`, `StorageError` in `peisear-storage`, and
  `AppError` in `peisear-web` with `From` bridges.

### Removed

- askama / askama_axum dependency.

## [0.1.0] — initial release

### Added

- Initial implementation of a minimal issue-tracking web application
  with projects, issues, and a kanban board.
- User registration, login, and logout backed by argon2id and JWT.
- axum 0.7 + askama templating + sqlx on SQLite.

[Unreleased]: https://github.com/nabbisen/peisear/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/nabbisen/peisear/releases/tag/v0.9.0
[0.8.0]: https://github.com/nabbisen/peisear/releases/tag/v0.8.0
[0.7.0]: https://github.com/nabbisen/peisear/releases/tag/v0.7.0
[0.6.0]: https://github.com/nabbisen/peisear/releases/tag/v0.6.0
[0.5.0]: https://github.com/nabbisen/peisear/releases/tag/v0.5.0
[0.4.0]: https://github.com/nabbisen/peisear/releases/tag/v0.4.0
[0.3.0]: https://github.com/nabbisen/peisear/releases/tag/v0.3.0
[0.2.3]: https://github.com/nabbisen/peisear/releases/tag/v0.2.3
[0.2.1]: https://github.com/nabbisen/peisear/releases/tag/v0.2.1
[0.2.0]: https://github.com/nabbisen/peisear/releases/tag/v0.2.0
[0.1.0]: https://github.com/nabbisen/peisear/releases/tag/v0.1.0
