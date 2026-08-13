# Changelog

All notable changes to peisear are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.22.0] — 2026-08-13

### Fixed

- **In a team-owned project, only the project owner could be assigned an
  issue** (`TEAM-001`, RFC 009). Every other team member was rejected as an
  invalid assignee, so per-person workload — the product's central
  per-person sustainability signal — was empty for everyone except project
  owners. It failed by showing nothing rather than by erroring, which is
  why it survived two releases, an external design document, a
  requirements baseline, and a compliance pass. `list_assignee_candidates`
  and `project_workload` now derive from one shared definition: a
  project's owner plus its team's `admin`/`member` roles (`viewer`
  excluded — read-only by the team schema's own design). A user removed
  from a team keeps issues already assigned to them; the workload report
  reflects that even though they are no longer a valid new assignee.
- **`SwitchingAriaLabel` doubled "per active day"** (`COPY-001`). The
  chip's median text already appended the suffix; the aria sentence
  appended it again, producing "median 2.4 / active day pickups per
  active day". Fixed at the source — the median value and its suffix no
  longer travel together into a caller that adds its own.
- **The two sub-issue nesting rejection messages disagreed on wording**
  (`COPY-001`). Unified on the clearer of the two at both the form and
  the submit path.
- **A capacity-period validation message named internal field
  identifiers instead of the form's own labels** (`COPY-001`). Now names
  "From"/"To", matching what the form shows.
- **`TestApp::spawn` could collide with itself under concurrent test
  runs** (`QA-001`, RFC 005 §9). Its temporary SQLite file's name was
  derived from the system clock, so two tests starting in the same
  instant could open the same file — an intermittent test failure
  unrelated to what either test was actually checking. The name is now
  derived from a per-process counter instead. A new structural test
  (`test_harness_scan`) checks every test harness in the workspace for
  the pattern's reappearance, rather than trusting a fixed list of two
  known files.

### Added

- **The sprint planning page** (`PLAN-001`, RFC 001):
  `/teams/{slug}/sprints/{sprint_id}/plan`. A two-column surface — a
  filterable, team-wide backlog of open issues on the left, the sprint's
  committed items on the right — with button-driven moves between them,
  replacing the one-issue-at-a-time round trip through each issue's own
  sprint dropdown. Filters (project, priority, assignee) persist across a
  move via the redirect's query string. Any team member, including a
  `viewer`, may read the page; moving issues requires `admin`/`member`.
  An active sprint's plan renders without move controls; a completed
  sprint additionally stops showing the backlog, since re-opening a
  finished sprint to add issues is not a supported flow.
  - **The remove route's authorisation boundary was hardened before it
    shipped, not after.** The route takes an issue id, and the
    underlying storage removal carries no sprint scoping of its own —
    without an added check, any authenticated member of any team could
    have removed an issue from a different team's sprint by naming its
    id directly in the request. The check was written and tested inside
    the same piece of work that created the route; the route never
    shipped without it.
  - **The capacity hint RFC 001 originally specified is not in this
    release, and that is deliberate.** It sums each participating
    member's capacity, and the page that would show it also names who
    those members are — with one participant the sum is that person's
    capacity outright; with two, a member can subtract their own and
    have the other's. Capacity is self-only data. A design that survives
    a two-person team is being worked out separately; the committed-
    points total (a sum of effort on issues already visible on the same
    page, not a capacity aggregate) ships in its place.

### Changed

- `CONTRIBUTING.md` now documents `DEC-007`'s three-consecutive-run
  `cargo test --workspace` procedure for changes touching
  `crates/peisear-web/tests/` or `crates/peisear-notify/tests/`, or
  before cutting a release. Followed since 0.20.0; this is the first time
  it has been written down outside an internal file.

## [0.21.0] — 2026-08-11

This is not "internationalisation" — `NFR-LANG-005` keeps a second locale
deferred, and one locale ships. What changed is that `§1.7`'s vocabulary
constraint, recorded for two releases as *"Implemented by convention; no
automated guard exists"*, **became checkable**. 0.20.0 and 0.20.1 were the
first evidence that checking finds real defects a person reading code had
already missed twice; this release finishes the job and adds a second guard
that keeps checking after the conversion is done.

### Added

- **`peisear-i18n`, a new crate: the message table, the English locale, and
  a vocabulary guard** (`I18N-001`). `MessageKey` is an enum, not a string
  constant or a `HashMap` — a `match` over it that omits a variant fails to
  compile, so a locale that doesn't handle every key is a build error, not
  a runtime gap. `find_violations` walks every key's rendered text against
  `§1.7`'s prohibited-vocabulary list (evaluative and failure-framed
  language) as a test, not a convention.
- **Every user-visible string `peisear-core`, `peisear-notify`, and
  `peisear-web` construct now renders from that table** (`I18N-002`,
  `I18N-003`, `I18N-005a`–`e`, `I18N-006`): shell and navigation; project,
  issue, team, and sprint pages; the today/inbox/settings dashboards and
  search; every error, validation, and auth message; `peisear-core`'s
  health-indicator descriptions and the `DisplayHealthState` accessible-name
  word; `peisear-storage`'s validation and conflict messages (now typed
  `MessageKey`, not `String`, carried across the `peisear-web` boundary and
  rendered there); the personal-burnout JSON endpoint's signal labels. The
  seven `validator`-derive messages that a proc-macro attribute cannot
  route through a function call are scanned by `find_violations` directly
  instead, closing the one category the table itself cannot reach.
- **A test that scans `peisear-web`'s own source for hardcoded copy**
  (`I18N-007`). `peisear-web/src/prose_scan.rs` walks
  `src/components/**` and `src/handlers/**` at test time and fails on a
  literal `aria-label`/`title`/`placeholder`/`onsubmit` attribute value or
  a bare text node in a `view!` body, with a nine-entry allowlist for the
  `onsubmit="return confirm(...)"` dialogs pending a separate decision on
  the confirmation pattern. Four earlier handoffs each declared their
  surface complete in prose and each was followed by another find; this
  replaces that claim with something that keeps checking.

### Fixed

- **`AppError::Conflict` rendered its internal `"conflict: "` log prefix
  to users** (found during `I18N-005e`). `public_message()` had no arm for
  `Conflict`, so it fell through to the `Display` impl meant for
  tracing/logs, and every conflict response — including the duplicate-
  email registration rejection — read `"conflict: {message}"` instead of
  the message alone. The same bug class as 0.20.0's `Validation` prefix
  leak, on a sibling variant nobody had checked. Fixed by making every
  `AppError` arm in `public_message()` explicit, with no catch-all: a
  catch-all silently absorbs a future variant the same way a wildcard
  match arm does, which is the exact failure this release exists to make
  impossible rather than watch for.

### What the guard covers, and what it does not

Stated so it is not over-trusted, extending the queue README's original
list with what `prose_scan.rs` adds and cannot see:

- It covers copy, not interpolated data — an issue titled with a term the
  guard's own vocabulary list prohibits is user data, not a violation.
- It catches vocabulary, not tone.
- It cannot see through runtime concatenation, which is why composing a
  user-visible sentence from more than one rendered fragment is prohibited
  by convention (`RFC 006` §D6).
- `prose_scan.rs` sees a literal sitting in a template attribute or text
  position. It cannot see copy assembled by `format!()`/`match` into a
  `String` binding before that binding reaches markup — a real blind spot,
  evaluated and left open rather than papered over with a noisier
  heuristic (a wider filter was tried and produced twelve false positives
  for every one real defect it would have caught).
- `prose_scan.rs` scans `src/components/**` and `src/handlers/**` only.
  `static/search.js`'s type-ahead copy is not Rust and is outside it,
  permanently.
- Nine `onsubmit="return confirm(...)"` dialogs are allowlisted, not
  converted, pending a decision on the confirmation pattern itself.

**No claim that every user-visible string is covered.** The scan test and
its allowlist are the check going forward; this entry points at them
instead of asserting completeness in prose.

## [0.20.1] — 2026-08-10

### Fixed

- **The project-health summary sentence could still name a severity above
  `Watch`** (`ISSUE-006`, `I18N-004` — a patch on RFC 007's DEV-004 fix,
  `NFR-LANG-002`, P0). 0.20.0's `DisplayHealthState` clamp was wired into
  badge, glyph, and API rendering, but `project_health::summarize` — which
  builds the summary paragraph directly beneath the health heading —
  still selected from the internal four-state model directly, so a
  project reaching the worst internal severity rendered that state's name
  in the summary sentence itself. It survived 0.20.0's own ceiling test,
  which matched an exact-case substring against the whole page body and
  happened to pass against a page that rendered the word in lowercase
  prose rather than the capitalised form the test checked for. Fixed
  structurally rather than by rewording: the two message-table entries
  that could render the unclamped severity are removed from
  `peisear-i18n` outright, so no caller in any crate can construct that
  sentence — not just `summarize`. The ceiling test is now case-insensitive
  and checks the summary sentence specifically, not only the page as a
  whole.
- **Two health-indicator explanation sentences were grammatically
  broken** (`ISSUE-006`, `I18N-004`). The bus-factor explanation for a
  one-person project read as two sentence fragments concatenated
  incorrectly, and the WIP-compliance explanation repeated its own
  qualifier. Both were live defects, not edge cases — the WIP-compliance
  one was confirmed reachable by reproducing it against a project with
  one assignee over the default WIP limit before it was corrected. Both
  now read as plain sentences; no wording elsewhere in either indicator's
  explanation changed.

## [0.20.0] — 2026-08-03

### Fixed (privacy)

- **Workload chips disclosed a member's capacity and over-capacity state to
  other project viewers** (RFC 007 `NFR-PRIV-001`, DEV-003). The project
  detail screen's workload strip and the issue create/edit forms' workload
  hint rendered `{in_flight}/{capacity} pt`, an "— already at N pt over
  capacity" annotation, and a capacity-derived danger badge for every
  member shown — all private data per `NFR-PRIV-001`, visible to anyone
  with project access. Fixed by showing only in-flight load (permitted by
  `NFR-PRIV-002`) on these three surfaces; the subject's own `/today` and
  `/settings` are unaffected. An initial version of this fix also
  suppressed the strip whenever it would show only one member, reasoning
  from `NFR-PRIV-007` (aggregate suppression) — that reasoning was wrong
  and the suppression has been removed: a chip labelled with one person's
  name and their in-flight load is not an aggregate that could resolve to
  an individual, it *is* individual workload, which `NFR-PRIV-002`
  explicitly permits regardless of how many members a surface lists (see
  `ISSUE-003`). Separately, and not addressed here: the query backing all
  three surfaces returns only a project's owner, never other team
  members — a pre-existing functional defect with its own consequences,
  tracked for a dedicated RFC rather than folded into this privacy fix.

### Added (accessibility)

- **Keyboard-operable status control on the kanban board** (RFC 007
  `FR-DM-002`, DEV-002). The board's only status-change path used to be a
  mouse drag; `FR-DM-002` (P0) requires every direct-manipulation action to
  have a keyboard equivalent, with no mouse-only action remaining. Each
  card now also carries a plain `<form method="post">` with one submit
  button per reachable target status — no JavaScript involved
  (`DEC-021`), Post/Redirect/Get back to the board, targets ≥ 44×44px, and
  an accessible name naming both the issue and the target status so a
  screen reader doesn't read "Done" twenty times with no context. The new
  form-encoded entry point (`POST .../status/board`) shares the same
  `apply_status_change` lock check the drag path (`change_status`) uses —
  one implementation, two entry points, so they cannot drift apart the way
  the optimistic-lock bypass above did. One structural change was required
  to add it: the card's outer element changed from `<a>` to `<div>`,
  because a `<form>` cannot nest inside an `<a>` (invalid HTML) — the link
  and the new form are now siblings, both children of the draggable
  wrapper, so a drag still moves both together. The inner `<a>` carries
  `draggable="false"`: an anchor is draggable by browser default, and
  without it the restructure left two nested drag sources, so a drag
  would have carried the link's `href` instead of the card.
- **Kanban status endpoint bypassed the optimistic-lock contract**
  (RFC 007 §10.6, DEV-001). `POST /projects/{id}/issues/{issue_id}/status`
  carried a "Phase A rollout" bypass that accepted and applied a status
  mutation with no `client_updated_at` at all — the shipped kanban board
  never sent one, so every drag-and-drop status change bypassed the
  optimistic-lock contract (`NFR-CONC-001`, `NFR-CONC-005`) across four
  releases (Phase A closed at 0.17.0; the bypass outlived it). Fixed by
  routing every request, empty or not, through the same
  `check_optimistic_lock` the form-based paths already use. A missing or
  empty value now returns 400 and leaves the row unchanged, matching every
  other mutation path. The board now renders `data-updated-at` on each
  card and sends it back on drop; a stale value now surfaces as a real
  409 conflict for the first time, so `static/board.js` gained rollback
  (revert the card to its original column), a `role="status"` message
  region in place of `alert()`, and removal of the failure-framed status
  string §1.7 prohibits. `check_optimistic_lock`'s parse-failure message
  no longer echoes the raw client value or uses developer vocabulary, and
  was later reworded to be entity-neutral — the helper backs the issue,
  project, sprint, and capacity form paths, not just the board, and the
  original wording said "board" unconditionally.
- **Project-health presentation exceeded the `Watch` severity ceiling and
  rendered a 0–100 score** (RFC 007 §10.2/§17.1, DEV-004). The project
  detail screen rendered `"Score N / 100"` as a headline badge, and a
  `Concern` severity — including danger (`badge-error`) colouring — was
  reachable in presentation on both the health strip and `/today`'s WIP /
  long-stale indicators, contradicting `FR-HLT-008` and `NFR-LANG-002`
  (no user-visible label may exceed `Watch`; no 0–100 gauge). Fixed by
  introducing `peisear_core::DisplayHealthState`, a three-state
  (`Insufficient`/`Good`/`Watch`) presentation type that is the only shape
  render code may use — `HealthIndicator::badge_class()` (which had the
  `Concern → badge-error` mapping) is gone, so a render site that skips the
  clamp fails to compile rather than silently leaking a state above the
  ceiling. The internal four-state model is unchanged (`Concern` stays, for
  computational accuracy — `FR-HLT-009`); only the render boundary changed.
  The composite score no longer carries a number or heads the section — it
  renders as one more chip at equal weight beside the six individual
  indicators, keeping its badge, trend, and summary sentence, per
  `FR-HLT-008` and external design §6 SCR-08. `/api/users/{id}/burnout`'s
  `indicator` field can no longer serialise `"concern"`.

  > **Correction (0.20.1):** this clamp covered badge, glyph, and API
  > rendering only. The health strip's summary sentence — the paragraph
  > directly beneath the heading — was not routed through it, and could
  > still name the unclamped severity directly in prose. See
  > `[0.20.1]` below.
- **`AppError::Validation`'s public message leaked "validation failed: "**
  (found while fixing the above). `public_message()` fell through to the
  `Display` impl used for tracing/logs, which is intentionally prefixed
  for developer readability — but that made every validation error in the
  application render with a "failed" prefix on the page the user sees,
  which is failure framing prohibited by §1.7. `public_message()` now
  returns the caller-supplied message as-is for this variant; the log-only
  `Display` impl is unchanged.

### Changed (quality gates)

- **`cargo fmt` and `cargo clippy -D warnings` had never passed** (`ISSUE-001`,
  DEV-006, DEV-007). `NFR-MNT-007` records both as "Implemented in CI"; in
  fact CI failed both checks on every push since it was introduced — 14 of
  16 jobs (build, all test crates) passed throughout, so this was
  formatting/lint debt, not a correctness regression. DEV-006 ran
  `cargo fmt --all` once, mechanically, across 44 files (no semantic
  change — verified by an unchanged clippy failure set and unchanged test
  pass counts). DEV-007 then cleared all 21 pre-existing clippy errors in
  `crates/peisear-storage` (14 `type_complexity`, 4 `too_many_arguments`,
  1 `ptr_arg`, 1 `redundant_pattern_matching`) with named row types and
  parameter structs, no behaviour or schema change. Fixing `peisear-storage`
  exposed 3 further, previously invisible `clippy` findings in
  `peisear-web` (masked until now by `peisear-storage`'s compile failure
  blocking the lint pass from reaching it) — tracked separately as
  `ISSUE-002` and resolved by DEV-008, below.
- **`peisear-web` had never actually been linted** (`ISSUE-002`, DEV-008).
  With `peisear-storage` clean, clippy proceeded far enough to reveal
  `peisear-web`'s own findings for the first time — 2 known in advance
  (`too_many_arguments` on `render_issue_detail`, `unnecessary_sort_by` ×2
  in `apply_filter_and_sort`) plus one more that stayed masked until those
  were cleared and the 13 integration test targets could compile at all
  (`doc_lazy_continuation` in a test's module doc comment). Fixed:
  `render_issue_detail`'s 13 positional arguments replaced by one
  `IssueDetailView` parameter struct; the two descending sorts rewritten
  `sort_by_key(Reverse(..))`, preserving stable ordering including for
  equal keys; the doc comment's under-indented continuation line given
  its own paragraph instead of the deeper indentation clippy suggested,
  which would have nested it under the wrong bullet. No behaviour change.
  `cargo clippy --workspace --all-targets -- -D warnings` reached exit 0
  for the first time in the project's history.
- **`cargo fmt`/`cargo clippy` were never exercised at a pinned, reproducible
  toolchain, and the declared MSRV was never built in CI** (DEV-005 item A,
  `TRK-022`, `RSK-002`, `NFR-CMP-001`). `rust-toolchain.toml` now pins
  `1.97.1` for `fmt`/`clippy` determinism across contributors and CI, kept
  separate from the MSRV itself; `rust-version` in `Cargo.toml` is raised
  from `1.85` (which does not build — `ISSUE-004`) to `1.88.0` (owner-
  ratified, `DEC-044`), and a new CI job builds the workspace on `1.88.0`
  so the MSRV claim is tested rather than asserted. Raising the declared
  MSRV enlarged clippy's lint surface — `collapsible_if` is MSRV-aware and
  only suggests let-chain syntax once the crate declares support for it —
  surfacing 7 findings across two crates (6 in `peisear-storage`, 1 in
  `peisear-web`'s `jobs.rs`), none pre-existing debt, all a direct
  consequence of this handoff's own change (`ISSUE-005`). Rewritten as
  let-chains rather than suppressed; `pool.rs`'s three-deep nesting
  collapsed into one flat condition rather than a mechanical three-level
  translation. No behaviour change; `cargo clippy --workspace --all-targets
  -- -D warnings` remains at exit 0.

### Removed

- **Withdrew the `#[ignore]`d `cross_user_settings_post_returns_403` test**
  (DEV-005 item B, `RSK-003`). It asserted a boundary that cannot exist
  while settings mutations (`/settings/wip-limit`, `/settings/capacity/*`)
  are session-scoped rather than addressed by `user_id` in the path —
  `FR-API-006`, which would introduce such a user-scoped POST surface, is
  unscheduled. An `#[ignore]`d test on a privacy boundary reads as coverage
  that does not exist, so it's withdrawn rather than left in place.
  Reinstate an equivalent test if `FR-API-006` ever lands. Confirmed no
  other test covers cross-user settings access before removing it.

### Fixed (documentation)

- **"Four crates" was wrong everywhere it appeared** (DEV-005 item C,
  `TRK-024`). The workspace has six crates (`peisear-core`, `peisear-auth`,
  `peisear-storage`, `peisear-notify`, `peisear-web`, `peisear`); `README.md`
  and three `docs/architecture/` files said four and, in `crate-boundaries.md`
  and `workspace-layout.md`'s case, didn't mention `peisear-notify` at all.
  Corrected the counts and added the missing crate's entry to both documents
  (short description in `crate-boundaries.md`; a tree entry in
  `workspace-layout.md`). Left the kanban drag-and-drop description in
  `README.md` untouched — it's accurate; the previously-recorded
  `FR-DM-001` status was the error, corrected separately in the
  requirements baseline amendments. `crates/peisear/src/lib.rs`'s own doc
  comment carried the same "four sibling crates" claim; fixed separately
  once this item's non-change scope was confirmed to permit doc comments
  under `crates/*/src` — the implementation is five crates, of which the
  facade re-exports four (`peisear-notify` is a dependency, not
  re-exported).

## [0.19.1] — 2026-05-05

### Added (documentation)

Patch release with no code changes — adds the `rfcs/`
folder and five design documents covering the upcoming
Phase C–E themes, so the next implementation work has a
concrete contract to start from.

- **`rfcs/README.md`** — folder index, lightweight + detailed
  template, lifecycle, language note. The detailed template
  adds Background / Requirements / Design / Test plan /
  Security & privacy considerations on top of the
  lightweight shape, triggered when the change crosses a
  crate boundary, touches a schema migration, affects the
  §11.5 privacy boundary or §21.4 optimistic-lock contract,
  or introduces a new public surface.
- **`rfcs/0001-sprint-planning-page.md`** (Phase C PR2,
  target 0.20.0). Bulk-assign issues to a sprint via
  `/teams/{slug}/sprints/{sprint_id}/plan`. Two-column
  layout, button-based moves (DnD deferred to D-4),
  capacity hint as a soft signal not a hard limit. Resolves
  open questions left by the spec (backlog scope = team-
  scoped projects only, capacity hint formula, sprint-side
  ordering).
- **`rfcs/0002-calendar-surfaces.md`** (Phase C PR3, target
  0.21.0). Personal axis `/today/calendar` and project
  axis `/projects/{id}/calendar`; no team axis (§10.2).
  Pins migration `0016_issue_planned_dates.sql` to two
  columns (`planned_start_at` / `planned_end_at`) instead
  of the spec's four — reasoning recorded in the RFC. The
  spec's "no efficiency metrics" rule (§16.6) is encoded
  as a guard test (`calendar_does_not_render_efficiency_metrics`)
  so future drift is caught.
- **`rfcs/0003-inbox-refinements.md`** (Phase C PR4, target
  0.22.0). Silence-resume banner, prominent mark-all-read,
  email opt-in deferred to first-notification, sub-issue
  parent breadcrumb in search. Migration
  `0017_users_email_opt_in.sql` covers the deferred prompt
  state. Decided to *not* grandfather existing users into
  "never prompted"; they see the banner on next inbox
  visit.
- **`rfcs/0004-direct-manipulation.md`** (Phase D umbrella,
  target 0.23.0). Cross-cutting contract for the five
  substeps (D-1 status click, D-2 kanban DnD, D-3 calendar
  DnD, D-4 sprint-plan DnD, D-5 list reorder). Locks in
  the optimistic-update + 5-second undo + rollback-on-409
  pattern, the no-celebratory-language rule, and the
  ordering of substep delivery (D-1 → D-2 → D-4 → D-3 →
  D-5). Each substep gets its own follow-up RFC when it
  becomes the next thing on the table.
- **`rfcs/0005-quality-consolidation.md`** (Phase E, target
  0.24.0). Audit-format RFC: tables for authorization
  endpoints, optimistic-lock mutations, language strings,
  colour contrast, and aggregate-inferability suppression.
  Resolves the lone `cross_user_settings_post_returns_403`
  ignored test from 0.19.0 (activate or remove with cause).
  Includes proposed defaults for audit-log retention
  (30 days) and `issue_events` retention (90 days).

### Updated

- **`ROADMAP.md`** — Phase C PR2-4 and Phase D / E entries
  now link to the RFCs that detail them.
- **Workspace version** bumped 0.19.0 → 0.19.1 across the
  six workspace crates' inter-crate dependency
  declarations.

## [0.19.0] — 2026-05-04

### Added (Phase C PR1 — sub-issue hierarchy)

A 1-level parent/child relationship on issues. Big work that
splits naturally into smaller pieces can now live as a parent
issue with sub-issues attached, instead of being either a
single bloated issue or a fan-out of unrelated peers. Per
peisear-feature-spec-v2.1 §8.3 / §8.4, the design is
deliberately minimal — one new column on `issues`, one new
form, no new tables.

- **Schema** (`crates/peisear-storage/migrations/0015_sub_issues.sql`):
  - New nullable `parent_issue_id` column on the `issues`
    table, foreign-key referencing `issues(id)` with
    `ON DELETE CASCADE` (deleting a parent removes its
    children — clearer for the user than dangling references,
    and matches "delete the whole subtree" mental model).
  - Two partial indices for the two query shapes that
    matter:
    - `idx_issues_parent` on the non-NULL rows, for
      "give me the children of issue X".
    - `idx_issues_top_level` on the NULL rows, for
      "list/kanban this project's top-level issues" — the
      partial form lets the planner skip sub-issue rows
      without filtering them out post-fetch.
  - Two triggers enforcing the spec invariants:
    - `prevent_sub_issue_nesting_insert` / `..._update`:
      sub-issues cannot have sub-issues (1-level only); a
      sub-issue must share its parent's `project_id`; an
      issue cannot be its own parent (self-reference);
      an issue with existing children cannot be demoted
      (would create a 2-level chain — promote children
      first).
- **Domain model** (`crates/peisear-core/src/lib.rs`):
  - `Issue` gains a `parent_issue_id: Option<String>` field
    with extensive documentation of the constraints.
  - New helpers `Issue::is_sub_issue()` and
    `Issue::is_top_level()` so call sites read like prose.
- **Storage layer** (`crates/peisear-storage/src/issues.rs`):
  - `IssueRow` and `into_issue` round-trip the new column.
  - All SELECTs that build `Issue` values widened to include
    `parent_issue_id`.
  - `list_in_project` semantically narrows to **top-level
    only** — this is the function the project board, list
    view, and kanban use, and per §8.5 only top-level issues
    appear there. The previous all-issues behaviour is
    preserved as a new `list_all_in_project`, which the
    project_health and personal_metrics analytics paths can
    use if they ever need to (currently they go through a
    different aggregation path that already includes
    sub-issues).
  - New `list_sub_issues_of(parent_id)` returns children in
    creation order.
  - New `insert_sub_issue` mutation. Same shape as `insert`
    but takes a `parent_issue_id` and routes trigger
    violations through `translate_trigger_error` so the
    handler sees a `StorageError::Validation` (400) instead
    of a raw 500.
  - New `promote_to_top_level` and `demote_to_sub_issue`
    helpers for the future "convert this issue to/from a
    sub-issue" workflow (the form path lands in a later
    PR — the storage primitives are here so tests can
    exercise the paths).
- **Sprint follow-parent rule**
  (`crates/peisear-storage/src/sprints.rs`):
  - `sprint_for_issue` now does a coalescing query: if the
    issue itself has a `sprint_issues` row return it,
    otherwise look up the parent's row. Sub-issues thus
    "see" their parent's sprint without ever needing their
    own join row. Single round-trip whether top-level or
    sub.
  - `issues_in_sprint` filters to `parent_issue_id IS NULL`
    so the sprint detail surface lists each piece of work
    exactly once — listing both parent and child would
    double-count effort against the sprint's commitment.
  - The sprint-assignment handler
    (`handlers/sprints.rs::assign_issue`) now refuses
    sub-issue targets with a 400 ("Sub-issues follow the
    parent's sprint. Change the parent's sprint instead.").
- **HTTP routes** (`crates/peisear-web/src/app.rs`):
  - `GET /projects/{id}/issues/{issue_id}/sub-issues/new`
    renders the sub-issue creation form.
  - `POST /projects/{id}/issues/{issue_id}/sub-issues/new`
    creates the row and redirects back to the parent's
    detail page (so the user immediately sees the new
    child rendered in the parent's "Sub-issues" list).
- **Handlers** (`crates/peisear-web/src/handlers/issues.rs`):
  - `render_detail_or_edit` now loads (a) the issue's sub-
    issues if it's top-level, and (b) the parent issue if
    it's a sub-issue. Both lookups are cheap (single index
    hit each). Empty inputs are propagated cleanly.
  - New `new_sub_issue_form` GET handler validates that the
    parent isn't itself a sub-issue (1-level rule) and
    short-circuits with a clear validation error if so.
  - New `create_sub_issue` POST handler runs the same
    validation, then calls `insert_sub_issue`.
- **UI** (`crates/peisear-web/src/components/issues.rs`):
  - New "Sub-issues" card on the issue detail page,
    rendered only for top-level issues (one-level rule
    means sub-issues can't have children themselves so the
    section would always be empty).
  - When a top-level issue has no sub-issues, the card
    shows a brief explainer with the "+ Add sub-issue"
    button: "Break this work into smaller pieces if it
    helps you track them — they share this issue's project
    and sprint, but can have their own assignee, status,
    and effort."
  - When sub-issues exist, they render as a compact
    `<ul>` with status badge + title link. Each row has
    an `aria-label` like "Title, status In Progress" so
    screen readers get the same information.
  - New `SubIssueNewPage` component for the create form.
    Mirrors `IssueNewPage` minus the sprint picker (the
    sprint follow-parent rule means a separate selector
    here would falsely imply independence).
  - Parent-aware breadcrumb: on a sub-issue's detail page
    the breadcrumb threads through the parent ("Projects /
    FooProject / Parent issue / This sub-issue"). The
    parent in the chain is a link, so the user can
    navigate up either to the project or to the parent.
  - Sprint card on the issue detail page is hidden for
    sub-issues — sub-issues follow the parent's sprint, so
    exposing a selector would be confusing. The user can
    still see what sprint a sub-issue is in (via the
    parent's view) and change it via the parent.
- **Tests** (`crates/peisear-web/tests/sub_issues.rs`,
  7 new tests):
  - `list_in_project_returns_top_level_only` — project
    list returns 2 rows for a project with 2 top-level + 2
    sub-issues; `list_sub_issues_of` returns the 2 children.
  - `detail_page_renders_sub_issues_section_for_top_level`
    — the section appears on a top-level issue's detail
    page with the right `aria-label`, the child title,
    and the "+ Add sub-issue" affordance.
  - `detail_page_omits_sub_issues_section_for_sub_issue`
    — the section is absent on a sub-issue's own detail
    page, and the breadcrumb includes the parent's title.
  - `create_sub_issue_via_form_links_to_parent` — POSTing
    to `/sub-issues/new` succeeds (303 SEE_OTHER), and
    the parent detail page renders the new child.
  - `cannot_create_sub_issue_under_a_sub_issue` — POSTing
    against a sub-issue's `/sub-issues/new` URL returns
    400 ahead of the SQL trigger (handler-level
    validation gives a better error path).
  - `sub_issue_inherits_parent_sprint` —
    `sprint_for_issue(child_id)` returns the parent's
    sprint id when the child has no `sprint_issues` row.
  - `cannot_assign_sprint_directly_to_sub_issue` — POST
    to a sub-issue's `/sprint` URL returns 400 with the
    "follow the parent's sprint" message.
  - Total suite: 65 active / 1 ignored.

## [0.18.0] — 2026-05-03

### Added (Phase B PR3 — UI changes for /today + project detail + issue detail)

Four UI surfaces get user-facing improvements per the V2.1
brief's "minimal by default, signals reach you" principle. None
of these change behaviour — they reorganise what's shown so
the day-to-day surface stays calm and a user looking for
detail can find it without scrolling.

- **B-1 `/today` panel collapsing + "what to read first"
  callout** (`crates/peisear-web/src/components/me.rs`):
  - The "Right now" panel (WIP / Load) stays always-visible:
    most-actionable, smallest surface, the answer to "should I
    pick up another issue right now?"
  - The "Rhythm" panel (Throughput / Long-stale / Pace) now
    sits inside a default-closed `<details>`. Same content,
    one click away. Per V2.1 §0.3 "Minimal by Default" — this
    is "if you want to dig" data, not first-glance.
  - The "Sustainability" / burnout panel keeps its existing
    self-folding behaviour.
  - New `compute_read_first()` helper picks at most ONE
    callout from a strict priority chain:
    1. Sustained burnout (overload streak ≥ watch threshold,
       or stalled-assigned days ≥ watch threshold);
    2. WIP > effective limit;
    3. Long-stale issues count ≥ 1.
    First match wins; nothing renders if none apply (the
    default-quiet state). Callouts compete for attention;
    surfacing two at once dilutes both.
  - The callout renders as an `<aside role="note"
    aria-label="What to read first">` above "Right now",
    with a title and a one-sentence body. Phrasing is
    descriptive, not evaluative ("WIP is over your limit"
    rather than "you have too much WIP" — V2.1 §0.2).
- **B-2 project-health explainability**
  (`crates/peisear-core/src/lib.rs` +
  `crates/peisear-web/src/components/issues.rs`):
  - New `Indicator::human_explanation() -> Option<String>`
    method. Returns `None` for Good and Insufficient (no
    row); returns one neutral, descriptive sentence per
    `IndicatorKind` for Watch and Concern.
  - Examples:
    - Throughput Concern → "Throughput is 5 / 12 (42%) —
      fewer issues are reaching Done than the rest of the
      project's history."
    - LongStale Watch → "30% of in-flight issues haven't been
      touched in over two weeks."
    - BusFactor Watch → "67% of in-flight work is concentrated
      on one person."
  - The detail panel on the project page (the existing
    `<details>` inside the HealthStrip) now renders these
    explanations as a `<ul>` above the chip row — story
    first, numbers underneath. Per decision B-E5: prefer
    readability over calculation transparency.
  - A duplicated `Indicator` struct that had crept in is
    removed; there is now one canonical definition.
- **B-3 issue edit URL split**
  (`crates/peisear-web/src/handlers/issues.rs` +
  `app.rs` + `components/issues.rs`):
  - `GET /projects/{id}/issues/{issue_id}` now always renders
    read-only.
  - `GET /projects/{id}/issues/{issue_id}/edit` is the new
    edit-mode URL.
  - `?edit=1` on the legacy URL 308-redirects to the new
    `/edit` URL — bookmarks and external links from before
    0.18.0 still work. 308 (not 301) preserves the request
    method, symmetric with the `/me`→`/today` migration that
    landed in 0.17.0.
  - URL-driven edit mode means refresh, browser-history back,
    and "Open in new tab" all behave consistently. `/edit` is
    a place, not a state hidden in a query parameter.
  - Internally, the two handlers share `render_detail_or_edit`,
    differing only in the `is_edit_mode` flag.
- **B-4 status segment UI** (`components/issues.rs`):
  - The single status badge on the issue detail page is
    replaced with a three-segment Open / In Progress / Done
    control. The current status is highlighted via
    `btn-primary`; the other two are `btn-ghost` (recede).
  - Display-only by design: `cursor-default`, `tabindex="-1"`,
    `type="button"` with no click handler, `aria-pressed`
    semantically conveys the active state. Direct-manipulation
    status changes (clicking a segment to mutate) are
    deliberately deferred to Phase D — this PR sets up the UI
    affordance without the wiring, so the change can be
    reviewed and lived with for a while before the
    mutation path lands.
  - The dedicated Edit button (top-right) remains the path to
    change status. The edit form keeps its existing
    `<select name="status">` widget, guarded by a regression
    test that fails if the segment ever leaks into edit mode
    ahead of Phase D.
- **Tests**: 4 new test crates + 10 new tests:
  - `tests/issue_edit_url.rs` — 3 tests (read-only render,
    edit URL renders form, legacy `?edit=1` 308 redirect).
  - `tests/status_segment.rs` — 2 tests (3-segment
    aria-pressed render, edit form keeps `<select>`).
  - `tests/health_explainability.rs` — 2 tests (project
    health strip renders, explanation list omits Good/
    Insufficient indicators).
  - `tests/today_panel.rs` — 3 tests (Right now visible,
    Rhythm folded by default, fresh user sees no callout).
  - Total suite: 58 active / 1 ignored.

### Added (Phase B PR2 — Personal Data API + ApiAppError)

Three read-only JSON endpoints surface the same data the
`/today` and `/inbox` HTML pages show, for JavaScript callers
or future app-driven dashboards. Per
[v2.1 spec §11.5](docs/spec/peisear-feature-spec-v2.1.md): the
"self access only" boundary is enforced — admin status doesn't
bypass it, and unauthenticated callers get JSON 401 (not an
HTML redirect to /login).

- **New endpoints**
  - `GET /api/users/{user_id}/burnout` — JSON shape `{ user_id,
    indicator, signals: [...], computed_at }`. The `signals`
    array contains only signals that meaningfully fired
    (overload streak above the watch threshold, stalled
    assignments above the watch threshold, non-Steady drift
    direction, cognitive switching), each with a stable
    `code` (`"overload_streak"`,`"stalled_assigned"`,
    `"estimation_drift"`, `"cognitive_switching"`) so
    JS clients can switch on it without parsing the human
    label. `indicator` follows the §1.4 / §4.4 ceiling at
    Watch — never `Concern`.
  - `GET /api/users/{user_id}/capacity` — JSON shape
    `{ user_id, effective_today, rows: [...] }`. Mirrors the
    `/settings` capacity table. Each row carries
    `updated_at` (RFC3339) so clients can render
    "updated X ago" without a separate request.
  - `GET /api/users/{user_id}/notifications` — JSON shape
    `{ user_id, unread_count, items: [...] }`. Returns the
    most recent 50 notifications, mirroring the `/inbox`
    HTML page. Older items remain accessible via the HTML
    inbox until a paginated `?cursor=` shape lands (deferred).
- **`ApiAppError` JSON-rendering sibling of `AppError`** in
  `error.rs`. Same variants (`Unauthorized` → 401,
  `Forbidden` → 403, `NotFound` → 404, `Validation` → 400,
  `OptimisticLockConflict` → 409, `Internal` → 500) but the
  `IntoResponse` impl emits JSON with a stable `error`
  keyword and a `message` string, instead of HTML and login
  redirects. `OptimisticLockConflict` includes the
  appendix E.3.3 structured fields (`entity_type`,
  `entity_id`, `current_updated_at`) so a future client-side
  retry-with-fresh-value UX has the data it needs. `From<
  StorageError>` and `From<AuthError>` conversions mirror
  `AppError`'s — handlers using `ApiAppResult` get the same
  `?` ergonomics.
- **`ApiAuthUser` extractor** in `extractors.rs`. Delegates to
  the existing `AuthUser` and translates its
  `AppError::Unauthorized → 303 redirect` into
  `ApiAppError::Unauthorized → 401 JSON`. The session-cookie
  parsing path is shared, so 401 here means the same thing
  it does on HTML pages — just with a different response
  shape.
- **`require_self()` helper** in `handlers/api_users.rs`.
  Returns `Forbidden` if the path's `user_id` doesn't match
  the authenticated session's user. Deliberately doesn't
  distinguish "user doesn't exist" from "different user" —
  the URL the caller already typed implies the existence
  they're testing for, and 403 is more honest about why the
  request was refused (per §11.5.2 wording).
- **`/api/search` typeahead migrated** from `AuthUser` /
  `AppResult` to `ApiAuthUser` / `ApiAppResult`. Phase A
  shipped this endpoint with HTML-style auth (which
  redirected unauthed JS clients to `/login`, leaking HTML
  into the JSON parser); now it returns 401 JSON consistently
  with the new endpoints.
- **Tests** (`crates/peisear-web/tests/auth_boundary.rs`):
  - 4 previously-`#[ignore]`d tests activated:
    `burnout_endpoint_walls_off_other_users`,
    `capacity_endpoint_walls_off_other_users`,
    `notifications_endpoint_walls_off_other_users`,
    `team_admin_cannot_read_member_personal_data` (the last
    one was a `todo!()` placeholder; now implemented as
    "Alice is admin of a team Bob is in, Alice's request
    against Bob's `/burnout` returns 403").
  - 4 new tests:
    - `self_can_read_own_burnout`,
    - `self_can_read_own_capacity`,
    - `self_can_read_own_notifications` (positive cases that
      a self-read returns 200 with a JSON body containing
      the documented fields),
    - `unauthed_api_users_returns_401_not_redirect` (the
      key UX win of `ApiAppError`: validates that the
      response is JSON `{"error":"unauthorized"}`, not an
      HTML redirect).
  - Test count: 10 active / 1 ignored (was 2 / 5). The
    remaining ignored test (`cross_user_settings_post_returns_403`)
    waits for an explicit user-scoped POST endpoint to land.
  - Search test `search_endpoints_require_authentication`
    tightened: was "303 OR 401" tolerated; now asserts
    401 specifically for `/api/search` after the migration.
  - Total suite: 48 active / 1 ignored.

### Added (Phase B PR1 — optimistic-lock rollout completion)

Closes the optimistic-lock rollout that 0.17.0 left
half-finished: sprint, team, team-membership, and capacity-
period mutations now honour the same §21.4 contract as issue
and project mutations. The schema infrastructure (migration
0014, `updated_at` columns + auto-bump triggers) shipped in
0.17.0; this PR plumbs it through the application layer.

- **Rust struct fields** for `Sprint::updated_at`,
  `Team::updated_at`, `TeamMembership::updated_at`,
  `CapacityRow::updated_at` — the columns added by migration
  0014 are now exposed to the application layer.
- **Storage SELECT widening** across nine query sites:
  - `peisear-storage::sprints`: `find_by_id`, `list_for_team`,
    `active_for_team`, the completed-sprints query (4 sites).
    `map_sprint_row` signature widened to take 11 args.
  - `peisear-storage::teams`: `find_by_id`, `find_by_slug`,
    `teams_for_user`, `membership` (4 sites). The auth-only
    `role_for` is unchanged — it returns just the role enum
    and doesn't construct a TeamMembership.
  - `peisear-storage::user_capacities`:
    `effective_row_for_user`, `list_for_user`, `find` (3
    sites). The points-only `effective_for_user` and the
    join-detection `overlaps_existing` are unchanged.
- **Sprint handlers** (`peisear-web::handlers::sprints`):
  `update`, `start`, `complete`, `delete_sprint` now
  re-read the sprint after the access check and call
  `check_optimistic_lock` against `form.client_updated_at`.
  `start` / `complete` / `delete_sprint` got a new
  `LifecycleForm` body type carrying the lock value.
  `assign_issue` is documented as join-table contention
  (mutates `sprint_issues`, not the issue or sprint
  directly) and deliberately doesn't lock — adding it would
  require a `version` column on the join, which is out of
  scope until concrete contention shows up in practice.
- **Capacity handlers** (`peisear-web::handlers::settings`):
  `update_capacity`, `delete_capacity`, `close_capacity`
  re-read the row and lock-check before mutating. New
  `CapacityDeleteForm` carries the lock value for the
  body-less delete. `insert_capacity` is creation, not
  mutation, so no lock check.
- **UI form templates**:
  - `SprintEditPage` renders the hidden `client_updated_at`
    input from `sprint.updated_at.to_rfc3339()`.
  - `SprintDetailPage` renders one hidden input per
    lifecycle form (start / complete / delete planned /
    delete completed) — each gets its own clone of the lock
    value so the move semantics are clean.
  - `render_capacity_row` adds hidden inputs to all three
    capacity forms (update / close / delete) per row.
- **Tests**:
  - `crates/peisear-web/tests/optimistic_lock.rs` activates
    the two `#[ignore]`d tests:
    - `sprint_start_with_stale_timestamp_returns_409` —
      creates a planned sprint, edits it (advancing
      `updated_at` via the trigger), then POSTs `/start`
      with the stale `client_updated_at`. Asserts 409.
    - `capacity_period_edit_with_stale_timestamp_returns_409`
      — parallel to the issue test: create row, update with
      valid t0, update again with stale t0. Asserts 409.
  - New fixture helpers `create_team_with_admin` and
    `create_planned_sprint` in `tests/common/fixture.rs`.
  - Test count: 6 active / 0 ignored (was 4 / 2). Total
    suite: 40 active / 5 ignored.

## [0.17.0] — 2026-05-03

This release closes Phase A of the v2.1 spec ([information
architecture](docs/spec/peisear-feature-spec-v2.1.md)). Five
user-facing changes ship together: navigation rename
(`/me`→`/today`, `/notifications`→`/inbox`), consolidated
breadcrumbs, list filter+sort persistence, global search, and
optimistic-lock for issue and project mutations. Two
infrastructure decisions ship in support: an `axum-test`-based
integration test scaffold, and `updated_at` columns on the
remaining domain tables for the Phase B optimistic-lock
rollout.

### Added (Phase A Step 5 — optimistic-lock for issue and project mutations)

This is the architectural commitment from
[v2.1 spec §21.4](docs/spec/peisear-feature-spec-v2.1.md):
every mutation endpoint compares a client-supplied
`client_updated_at` against the row's canonical `updated_at`
and rejects with 409 Conflict on mismatch. 0.17.0 ships the
contract for issue and project mutations (the entities whose
`updated_at` column already existed in 0.16.0). Sprint, team,
team-membership, and capacity mutations get the schema
infrastructure now (migration 0014 below) and the
handler-level checks in Phase B (0.18.0).

- **`AppError::OptimisticLockConflict`** new variant carrying
  `entity_type`, `entity_id`, and `current_updated_at`. The
  HTML response renders an explanatory error page urging the
  user to refresh and re-apply. The structured JSON shape
  from spec appendix E.3.3 is wired up here so when Phase B's
  `/api/*` mutation endpoints land, the conflict response
  payload is one `IntoResponse` impl away.
- **`crate::error::check_optimistic_lock(...)`** centralises
  the comparison so every handler site is one call:
  parse client RFC3339 (400 on malformed) → compare to current
  `updated_at` → 409 on mismatch. RFC3339 was chosen over
  Unix milliseconds for unambiguity (the `T`-and-`Z` shape
  self-identifies as a timestamp; ms vs. seconds vs. micros
  unit confusion that bites Unix epoch values goes away);
  language-portable parse paths in Rust, Python, JS,
  PostgreSQL all agree on RFC3339; and SQLite's
  `CURRENT_TIMESTAMP` already feeds chrono's `DateTime<Utc>`,
  which renders to RFC3339 with `to_rfc3339()` for free.
- **Issue mutations** carry `client_updated_at` end-to-end:
  - The edit form (`IssueEditForm`) renders a hidden input
    populated with `issue.updated_at.to_rfc3339()` at render
    time.
  - The `update` handler extends `IssueForm` with a
    `client_updated_at: String` field, re-reads the issue
    after the access check (so we compare against the
    canonical value, not a stale read from the page render),
    and calls `check_optimistic_lock` before any state-
    mutating SQL.
  - The kanban DnD JSON endpoint (`change_status`) extends
    `StatusChange` with the same field. During the Phase A
    rollout window the kanban JS hasn't been updated to
    embed `data-updated-at` per card yet, so we accept an
    empty string with a `tracing::debug!` line to track real
    traffic; Phase D's direct-manipulation work upgrades
    this to required.
- **Project mutations** carry the same plumbing: hidden input
  in `ProjectEditPage`, `client_updated_at` field on
  `ProjectForm`, lock check in the `update` handler.
- **Tests**:
  - `crates/peisear-web/tests/optimistic_lock.rs` activates
    4 tests (was 4 ignored in 0.16.0): issue update, issue
    status change, issue update with missing
    `client_updated_at`, and project update — each verifies
    that a stale value returns 409 (or 400 for missing). The
    sprint/capacity tests stay `#[ignore]` with notes
    pointing at Phase B.

### Added (migration 0014 — `updated_at` columns + auto-bump triggers)

Schema preparation for the Phase B optimistic-lock rollout to
sprints, teams, team_memberships, and user_capacities. Adds:

- `updated_at` `DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP`
  on each of the four tables.
- `*_updated_at` AFTER UPDATE trigger that bumps
  `updated_at` to `CURRENT_TIMESTAMP`. The
  `WHEN OLD.updated_at = NEW.updated_at` guard prevents
  recursion and documents that the application layer never
  sets `updated_at` directly — the trigger is the single
  point of truth.
- Backfill: existing rows get `updated_at` set to the latest
  meaningful event timestamp the row already carries
  (`COALESCE(completed_at, started_at, created_at)` for
  sprints; `created_at` for teams and capacities;
  `joined_at` for memberships). Using `CURRENT_TIMESTAMP`
  for backfill would invent a fictional "this row was
  updated when the migration ran" event that any future
  audit surface would have to apologise for.

The Rust struct fields (`Sprint::updated_at`, `Team::updated_at`,
`TeamMembership::updated_at`, `CapacityRow::updated_at`) and
the storage-layer SELECT widening to fetch them are
deliberately deferred to Phase B (0.18.0). Reasoning: they
form one cluster of changes (~30 mechanical edits across
struct definitions, query tuples, and handler call sites),
and shipping them together with the Phase B endpoint
authorization work is more reviewable than spreading them
across two releases. Live data accumulates correct
`updated_at` values from the moment 0.17.0 deploys, so when
Phase B turns on the handler-level checks, even rows
untouched in Phase A have a meaningful timestamp to compare
against.

### Added (Phase A Step 4 — global search)

A search box in the navbar takes the user to project and open
issue matches anywhere they have access. The implementation
follows the v2.1 spec §4.5 ("Search by simple LIKE %; project +
open issue scope; typeahead 8 / results 50").

- **New module `peisear-storage::search`** with two queries:
  - `projects_by_name(user_id, q, limit)` — projects the user
    can access, name LIKE `%q%`. Access is "personal projects
    they own + team projects of teams they belong to" (the
    same predicate as `projects::list_for_user`).
  - `open_issues_by_title(user_id, q, limit)` — issues in the
    same accessible project set, title LIKE `%q%`, **with
    `status != 'done'`** per the spec. The completed-work
    surface is sprint summaries and project-detail filters,
    not search.
  - LIKE meta-character escaping: `%`, `_`, and `\` in the
    user's input are escaped before the LIKE pattern is
    bound, so a search for "100%" matches the literal `%`
    rather than acting as a wildcard. Backslash is escaped
    first so we don't double-escape the introducers in
    subsequent replacements. Two unit tests cover the
    escaping logic; integration test
    `typeahead_handles_like_meta_characters` covers the
    end-to-end behaviour.
- **New handler module `peisear-web::handlers::search`** with
  two endpoints:
  - `GET /search?q=...&page=N` — HTML results page. Renders
    Projects and Open issues sections side-by-side, paginated
    at 50 hits per category per page. Each section
    independently shows "Next →" / "← Previous" depending on
    whether more rows exist beyond the current page; the
    "Next" detection is done by fetching one extra row beyond
    the page window, avoiding a `COUNT(*)` round-trip.
  - `GET /api/search?q=...` — JSON typeahead for the navbar
    input. Returns up to 8 hits total, balanced 4 projects +
    4 issues with overflow back-fill. Echoes `q` in the
    response so the client can drop stale responses if the
    user has typed more characters since the request was
    sent.
- **New navbar input box** in `components/layout.rs`. A plain
  HTML form so the search works without JavaScript (Enter
  submits to /search). With JS, vanilla
  `static/search.js` attaches a typeahead dropdown:
  - 250ms debounce
  - Aborts in-flight requests on new keystrokes
  - Skips queries shorter than 2 characters
  - Keyboard navigation (Down/Up to cycle items, Enter to
    activate the focused one or fall through to form submit,
    Escape to close)
  - Click-outside closes the dropdown
  - Server-side `q` echo + client-side comparison guards
    against stale-response races
  - HTML-escaping on both ends (server JSON encoding +
    client innerHTML render) for defence-in-depth
- **Tests**: 2 unit tests for the LIKE escaper; 9 integration
  tests in `tests/search.rs` covering empty-query handling,
  project name match, issue title match, exclusion of
  completed issues, cross-user isolation (a user does not see
  another user's personal project), LIKE-meta escaping, the
  HTML results page, the empty-query results page, and the
  authentication requirement.
- **CI**: `test-peisear-web-search` job added.

### Added (Phase A Step 3 — list filter/sort persistence)

The project-detail issue list now remembers each user's filter
and sort preferences per-project. The scheme is "URL primary,
server default secondary" (decision A-3 = C in the v2.1 session
record); the user-facing rules are described in
[v2.1 spec §4.4](docs/spec/peisear-feature-spec-v2.1.md).

- **New table `user_view_states`** (migration 0013). Stores a
  per-user, per-view JSON blob. The schema-less blob shape
  decouples future view-state fields from migrations — adding
  a new filter dimension does not require an `ALTER TABLE`.
  Documented in detail in the migration file's leading comment.
- **New module `peisear-storage::view_states`**: `get`,
  `upsert`, `delete`, plus `project_issues_key()` that mints
  the canonical view key `project_issues:{project_id}` so
  handlers don't have to remember the namespace by hand.
- **`ProjectViewQuery` extended** with `status`, `assignee`,
  and `sort` query params, and methods to (a) detect whether
  any of them were URL-supplied, (b) merge with a saved
  default such that URL-supplied fields win, (c) serialise
  the persistence-worthy subset to JSON, and (d) parse back
  defensively (a corrupt JSON row falls through to factory
  defaults rather than crashing the page).
- **`apply_filter_and_sort` in the issue handler**. Status
  filter, assignee filter (including `unassigned` literal),
  and sort by priority / created / updated. Stable sort
  preserves the storage-layer default order as a tiebreaker.
  Board view is **not** filtered — the kanban columns are
  themselves the status structure, and hiding columns based
  on a status filter would be surprising. Filtering applies
  only to the list view.
- **List view toolbar**: a plain `<form method="get">` with
  status / assignee / sort selects and Apply / Reset buttons.
  No JavaScript — Apply re-submits the form to the project
  URL with the new query params. Reset links to the bare URL,
  which inherits the saved default; explicitly choosing
  "All / Anyone / Default" + Apply is the way to overwrite
  the saved state. This trade-off is documented in the
  component file: the alternative ("Reset wipes saved
  default") would clash with users navigating via generic
  links and losing their context every time.
- **Persistence semantics**: the merged state is upserted
  iff the URL contributed at least one filter/sort field. A
  bare URL (no filter/sort params) does NOT overwrite the
  saved default — that would erase the user's preference
  every time they followed a generic link.
- **Per-user isolation**: the storage key is namespaced by
  `user_id`. Two users on the same project don't share view
  state. (Phase A only ships personal projects; team-project
  isolation arrives in Phase C, but the key shape is already
  ready for it.)
- **Tests** (`crates/peisear-web/tests/view_state.rs`): 5
  cases covering toolbar render, URL filter applied, explicit
  filter persists across bare-URL revisit, URL overrides
  saved default, per-user defaults isolated.
- **CI**: `test-peisear-web-view-state` job added.

### Added (Phase A Step 2 — breadcrumb consolidation and back-link)

This adds a shared breadcrumb component with consistent ARIA
semantics on the three detail pages where users spend most of
their time. See [v2.1 spec §4.4](docs/spec/peisear-feature-spec-v2.1.md)
for the navigation-context-preservation rationale.

- **`crates/peisear-web/src/components/breadcrumb.rs`** is the
  new home for the breadcrumb / back-link markup. The previous
  inline copies in `ProjectDetailPage`, `IssueDetailPage`, and
  `SprintDetailPage` had drifted: some led with `Projects`,
  others with `Teams`, none had a `Today` entry-point link, and
  none tagged the terminal node with `aria-current="page"`. The
  consolidation fixes all three.
- **`Today` is the leading entry on every detail-page
  breadcrumb**. It links to the v0.17.0 personal-dashboard URL
  (`/today`), reinforcing the v2.1 navigation entry-point story.
  `BreadcrumbItem` callers pass *intermediate* and *terminal*
  nodes only — the leading `Today` link is prepended by
  `render_breadcrumb` so it can't be forgotten.
- **The terminal node carries `aria-current="page"`**. Screen
  readers announce the user's current location; sighted users
  get a non-link node visually distinct from the link siblings.
- **A `← Back to {parent}` link sits beneath the breadcrumb**
  on each detail page. Implemented as an `<a>` to a canonical
  parent URL rather than `history.back()`, so the behaviour is
  predictable for users arriving via deep links (e.g. an email
  link). On mobile, where the breadcrumb often has to be
  truncated to fit, this gives a finger-friendly tap target.
- **Pages migrated**: `ProjectDetailPage`, `IssueDetailPage`,
  `SprintDetailPage`. The inline `<div class="breadcrumbs">`
  markup is replaced with a single call to
  `render_breadcrumb(vec![…])` followed by
  `render_back_link(label, href)`.
- **Tests**: `crates/peisear-web/tests/breadcrumb.rs` covers
  the structural invariants — `/today` link present, terminal
  node tagged `aria-current="page"`, back-link rendered — by
  substring-checking the SSR output. The substring approach is
  intentionally loose: it asserts the *contract* (what a screen
  reader will read) rather than the visual styling (which Phase
  B will rework).

### Added (Phase A Step 1 — URL rename: /me → /today, /notifications → /inbox)

This is the first user-visible v2.1 change in the Phase A roadmap
(see [v2.1 spec §4.2](docs/spec/peisear-feature-spec-v2.1.md) for
the rationale).

- **`/today`** is now the canonical personal-dashboard URL. The
  same handler (`handlers::me::page`) serves it, and internal
  links in the navbar dropdown and notification deep-links have
  been updated.
- **`/inbox`** is now the canonical notifications-inbox URL.
  The same handlers (`handlers::notifications::*`) serve all
  three routes (`GET /inbox`, `POST /inbox/{id}/read`,
  `POST /inbox/mark-all-read`). The bell icon in the navbar,
  the notification component's mark-read forms, and the
  redirect target after marking read have all been updated.
- **Legacy URLs return HTTP 308 Permanent Redirect**:
  - `GET /me` → `/today`
  - `GET /notifications` → `/inbox`
  - `POST /notifications/mark-all-read` → `/inbox/mark-all-read`
  - `POST /notifications/{id}/read` → `/inbox/{id}/read`

  308 (rather than 301) is used uniformly so the four redirects
  follow a single rule. 308 preserves the request method and
  body across the redirect — required for the two POST routes,
  unambiguous on the GET ones. RFC 7538.
- **`crates/peisear-web/src/handlers/redirects.rs`** is the new
  home for these redirect handlers. They're parameterless except
  for `/notifications/{id}/read`, which preserves `{id}` to
  the new path.
- **Tests**: smoke.rs covers the happy-path GETs at the new
  URLs and the redirect status of the legacy URLs.
  auth_boundary.rs distinguishes "the redirect itself runs
  before any auth check" (so unauthenticated `/me` returns 308
  to `/today`, not 401/303 to `/login`) from "the destination
  enforces auth" (so unauthenticated `/today` returns 303 to
  `/login`). This split is documented in the test cases so a
  reviewer can see the design intent: redirects don't depend
  on session state.

### Added (Phase A preparation — e2e basecost integration test infrastructure)

- **`axum-test 20`** as a workspace-level dev-dependency.
  Selected over `reqwest` + a real-port server because it
  shares the test runtime, doesn't allocate ports (so test
  parallelism isn't bottlenecked on port table), and integrates
  with `axum::Router` directly. Cookie support is built in,
  which peisear's session-based authentication requires.
- **`crates/peisear-web/tests/common/`** integration test
  helper module, structured as `mod.rs + submodules` per the
  Rust convention for shared test helpers (a bare `common.rs`
  would be compiled as its own empty test crate). Submodules:
  - `server::TestApp::spawn()` — fresh DB pool, migrations
    applied, JWT secret fixed for tests, `TestServer` configured
    with cookie persistence.
  - `auth::register / login / register_and_login / new_authed_app`
    — production-flow user setup with cookie jar saved.
  - `fixture::create_personal_project / create_issue` — domain
    data factories that bypass form validation.
  - `assertion::personal_data_endpoint_is_walled_off` and
    `stale_update_returns_409` — shared invariants for §11.5
    and §21.4.
- **`crates/peisear-web/tests/smoke.rs`** — 11 baseline tests
  exercising the canonical happy paths and the legacy URL
  redirects. Covers health, login/register pages, the
  register-then-redirect-to-/projects flow, the unauthenticated
  redirect-to-/login behaviour, the `/today` dashboard for an
  authed user, the `/inbox` dashboard for an authed user, logout,
  and the four 308 redirects from the legacy `/me` /
  `/notifications` URLs (including path-parameter preservation
  on `/notifications/{id}/read`).
- **`crates/peisear-web/tests/auth_boundary.rs`** — 7 test
  entries for v2.1 §11.5 enforcement. 2 active after Phase A
  Step 1 (`today_unauthenticated_redirects_to_login` and
  `me_unauthenticated_redirects_to_today_not_login`); 5 marked
  `#[ignore]` until their Phase B endpoints exist
  (`/api/users/{id}/burnout`, `/api/users/{id}/capacity`,
  `/api/users/{id}/notifications`, the team-admin-cannot-read
  case, and explicit user-scoped POSTs). The `#[ignore]`
  attribute is removed in the same PR that introduces the
  corresponding production endpoint, so the test inventory
  stays in lock-step with the API surface.
- **`crates/peisear-web/tests/optimistic_lock.rs`** — 4 test
  inventory entries for v2.1 §21.4 (issue update, issue
  status, sprint start, capacity period). All `#[ignore]`d
  pending Phase A's `client_updated_at` plumbing rollout.
- **`.github/workflows/test.yml`** — CI runs `cargo fmt`,
  `cargo clippy --all-targets -D warnings`, the integration
  test crates one-at-a-time (the combined link step has been
  observed to peak above 7 GB RAM on the default runner), and
  `cargo build --workspace`. Each test crate gets its own
  job for clear failure attribution.

### Decisions

These v2.1 implementation-strategy decisions, recorded for
future reference:

- **Versioning**: minor-bumps per Phase (Phase A → 0.17.0,
  Phase B → 0.18.0, ..., Phase E → 0.21.0). Major-version 1.0
  is reserved for a later definition-of-done milestone, not
  for v2.1 spec completion.
- **URL renames**: `/me` → `/today` and `/notifications` → `/inbox`
  use **HTTP 308 Permanent Redirects** rather than dual handlers.
  Old handlers are deleted; only the redirect remains. 308 (over
  301) is chosen because two of the four legacy routes are POSTs
  (`/notifications/mark-all-read`, `/notifications/{id}/read`),
  and 308 preserves the request method across the redirect where
  301 historically allowed clients to silently downgrade POST to
  GET. Using 308 uniformly across all four redirects keeps the
  rule simple. External bookmarks and links keep working
  indefinitely.
- **Optimistic locking rollout**: applied to all mutation
  endpoints whose entities have an `updated_at` column —
  issues and projects in 0.17.0; sprints, teams, team
  memberships, and capacity periods in 0.18.0 once the
  Phase B endpoint authorization work brings the rest of the
  schema in line. Migration 0014 ships the `updated_at`
  columns and triggers in 0.17.0 so live data accumulates
  meaningful timestamps from day one. Mixed-state APIs are
  bounded to one release window; the `updated_at` schema
  itself is uniform.
- **Frontend stack**: Leptos hydration / island. The crate is
  already in the dependency tree; islands cover the interactive
  needs (drag-and-drop, optimistic updates, conflict toasts)
  without introducing a second framework.
- **e2e infrastructure**: integrated into `peisear-web/tests/`
  before Phase A implementation begins, rather than added later
  Phase E. Phase A's optimistic-lock plumbing needs the test
  scaffolding from day one.

## [0.16.0] — 2026-04-29

### Added

- **`peisear-notify` crate.** A new workspace member — the
  sixth crate, joining `peisear-core`, `peisear-auth`,
  `peisear-storage`, `peisear-web`, and the `peisear` binary.
  This crate owns the notification dispatch pipeline (edge
  detection, channel routing, audit log) that previously
  lived inside `peisear-web::notifications`.
- **Real email delivery via the wasm-smtp 0.9 family.** The
  email channel — log-stub since 0.13.0 — now performs real
  SMTP delivery when `SMTP_*` environment variables are
  configured. Built on:
  - `wasm-smtp 0.9` (SMTP protocol core)
  - `wasm-smtp-tokio 0.9` with the `mail-builder` feature
    (production tokio + rustls Transport, plus
    `SmtpClient::send_message` convenience)
  - `wasm-smtp-cloudflare 0.9` (kept in the dependency tree
    unused, so a future Cloudflare Workers deployment is a
    transport swap, not a dependency change)
  - `mail-builder 0.4` (RFC 5322 / MIME composition)
- **STARTTLS support.** Both implicit TLS (port 465) and
  STARTTLS (port 587) work out of the box — `wasm-smtp 0.9`
  ships `SmtpClient::connect_starttls`; we pick the right
  transport flavour based on configuration.
- **`SmtpConfig::from_env`** reads operator config from
  environment:

  | Variable | Required | Notes |
  |---|---|---|
  | `SMTP_HOST` | for email | e.g. `smtp.example.com` |
  | `SMTP_PORT` | optional | default 465 (implicit TLS) |
  | `SMTP_TLS_MODE` | optional | `implicit` or `starttls`; auto from port if unset |
  | `SMTP_USER` | for email | SMTP AUTH username |
  | `SMTP_PASSWORD` | for email | SMTP AUTH password |
  | `SMTP_FROM_ADDRESS` | for email | `From:` header |
  | `SMTP_FROM_NAME` | optional | display name; falls back to address |

  When any required variable is missing, the email channel
  is unavailable but the in-app channel continues to work
  (see "Design" below).
- **Three integration tests** in
  `peisear-notify/tests/dispatch_integration.rs` verifying:
  - `smtp_unconfigured_records_in_app_only_in_dispatched_via`
    — the Q4 graceful-degradation contract.
  - `smtp_configured_but_unreachable_records_in_app_only`
    — failure at send time doesn't break in-app delivery.
  - `cooldown_suppresses_second_dispatch_within_window`
    — sanity that the cooldown filter still works after the
    dispatch pipeline moved crates.

### Design

- **Why a verb-form crate name (`peisear-notify`).** Existing
  crate names follow a noun-form pattern (`core`, `auth`,
  `storage`, `web`); plural nouns like `peisear-notifications`
  would commit the crate to a "list of notification objects"
  framing. A verb form keeps the responsibility shape open —
  this crate *notifies*, and may grow to *broadcast*,
  *summarise*, etc. without the noun-plural baggage.
- **What stays out of `peisear-notify`.** HTTP routes for
  `/notifications` and `/settings/notifications` (those live
  in `peisear-web::handlers`); UI rendering (in
  `peisear-web::components`); domain types like
  `Notification`, `Preference`, `Severity` (these stay in
  `peisear-core` so future crates like `peisear-ai` can
  produce notification events without depending on the
  dispatch pipeline); storage CRUD (in `peisear-storage`).
- **Why the wasm-smtp family is the right partner.** Per the
  pre-implementation Q&A: peisear is project management
  software, not an email product. The wasm-smtp family
  isolates the SMTP-related code surface — protocol,
  transport, authentication mechanisms (PLAIN, LOGIN,
  SCRAM-SHA-256, XOAUTH2), error classification, MIME
  composition (via `mail-builder`) — into an external
  ecosystem maintained by people who think about SMTP for a
  living. peisear-notify carries about 150 lines of email
  code; the rest delegates upward to the wasm-smtp family.
- **Q4 graceful degradation.** When SMTP env vars aren't set,
  startup logs a single `warn` line ("SMTP not configured;
  the email channel will fail at send time") and continues.
  Send attempts fail at the channel layer (logged at `warn`),
  the audit row records `dispatched_via` without `email`, and
  the in-app channel is unaffected. Rationale: peisear should
  remain useful in deployments that deliberately don't
  configure email (single-user instances, evaluation
  environments). A startup failure would punish them for a
  non-essential capability.
- **Default port 465, not 587.** Implicit TLS is the modern
  recommendation. `SMTP_TLS_MODE` lets operators override
  explicitly, and the auto-derivation from port number
  (587 → STARTTLS) means typical SMTP submission deployments
  Just Work without extra configuration.
- **Q5 test strategy: integration test, not docker mailpit.**
  The test verifies env-var diff (configured/unconfigured/
  unreachable produce correct `dispatched_via` outcomes)
  rather than full SMTP-on-the-wire delivery. Docker mailpit
  was considered and rejected — added CI complexity for
  marginal extra coverage. Real SMTP correctness is the
  operator's verification when they configure their own
  server.
- **Q6 from-address policy: global only.** Phase 1 has a
  single `SMTP_FROM_ADDRESS`; per-team overrides are Phase 2
  if real demand appears. Multiple From addresses complicate
  SPF/DKIM setup; not justified by current usage.

### Changed

- All six workspace crates bumped to `0.16.0`.
- `peisear-web::notifications::*` (the dispatch pipeline,
  channel impls, edge detection helpers) moved to the new
  `peisear-notify` crate. Public API surface preserved at
  the `peisear_notify::*` module path; internal callers
  (`peisear-web::jobs`, the binary's `main`) updated.
- The email channel's send call site went from
  `tracing::info!("[email-stub] would dispatch via email", ...)`
  to a real
  `SmtpClient::connect[_starttls]` →
  `SmtpClient::login` → `SmtpClient::send_message` → `quit`
  pipeline using `wasm-smtp-tokio` adapters. The function's
  signature in [`peisear_notify::channel::send_via_channel`]
  is unchanged.
- `.env.example` updated: STARTTLS noted as supported,
  `SMTP_TLS_MODE` documented, default port flipped 587 → 465.

### Deferred (Phase 2 candidates)

- **HTML email** (`multipart/alternative`). Phase 1 is plain
  text only; the 80% case for notification email.
  `mail-builder` makes adding HTML straightforward when a
  use case appears.
- **Connection pooling.** Today each email send opens, logs
  in, sends one message, and quits. Notification volume is
  small; pooling complexity isn't earned yet. If volume
  grows, `wasm-smtp` already supports multi-message sessions
  on a single connection (documented in upstream's
  `connection-reuse.md`); it's a refactor of our send path,
  not a new dependency.
- **Digest mode.** Bundle multiple notifications into a
  single per-day or per-week email. Requires per-user
  preference UI changes and storage shape changes; not
  blocking anything today.
- **Per-team `From:` address.** See Q6 in design notes
  above.
- **Webhook channel real implementation.** Still a log stub
  since 0.13.0. Requires per-user webhook URL UI + outbound
  HTTP client setup; tracked as a separate Phase 2 item in
  ROADMAP.

## [0.15.0] — 2026-04-28

### Added

- **Sprints (Phase 1: flat, Jira-style).** Time-boxed planning
  units scoped to a team. Optional — teams that don't want
  sprints continue working without them, and personal projects
  remain unaffected (sprints require a team).
- **`sprints` and `sprint_issues` tables.** A sprint has a
  team-scoped name, optional goal, inclusive `starts_on` /
  `ends_on` dates, and a lifecycle status: `planned` →
  `active` → `completed`. Issues join via a single-issue-
  per-sprint constraint (`sprint_issues.issue_id` is the
  primary key, so re-assigning an issue moves it).
- **`/teams/{slug}/sprints`** — sprint listing for a team.
  Members see; admins see + create. Includes the velocity
  chart (when there are 2+ completed sprints).
- **`/teams/{slug}/sprints/new`** — admin-only creation form
  with name, dates, optional goal.
- **`/teams/{slug}/sprints/{id}`** — sprint detail page with
  summary card, burndown chart (when active or completed),
  issues table, and admin lifecycle buttons (Start /
  Complete / Edit / Delete).
- **`/teams/{slug}/sprints/{id}/edit`** — admin-only sprint
  settings.
- **Sprint lifecycle transitions** are explicit admin POSTs:
  - `start`: `planned` → `active`. Refuses if another sprint
    in the same team is already active (one-at-a-time
    invariant enforced at the application layer).
  - `complete`: `active` → `completed`. Sets `completed_at`;
    carry-over numbers freeze at the moment of completion.
  - `delete`: removes a sprint at any state (admin
    judgment).
- **Issue assignment to sprints** via a dropdown card on the
  issue detail page. The dropdown appears only for issues in
  team-shared projects. Personal projects don't show it
  (sprints are a team feature). Completed sprints don't
  appear in the dropdown — historical numbers are immutable.
- **Velocity bar chart** on the sprint listing. Each
  completed sprint contributes a pair of bars: completed
  points (filled, distinct colour) plus carried-over points
  (lighter, alongside). A median reference line spans the
  recent window. Heading: "**Completed work this period**" —
  the bars are described as fact, never as performance.
- **Burndown line chart** on the sprint detail (active and
  completed states). Two cumulative lines: `committed`
  (added scope) and `completed` (finished work). The visible
  gap between them is in-flight work. **No ideal line**, no
  predicted-finish curve, no completion-percentage readout.
- **Sprint summary card** showing `Committed`, `Completed`,
  `In flight`, and (for completed sprints only) `Carried
  over`. Plain numbers; no "achievement %" or "velocity
  index".
- **Sprints link** added to the team detail page so members
  reach the listing in one tap.
- New core types in `peisear-core::sprints`:
  `SprintStatus` enum (`Planned` / `Active` / `Completed`),
  `Sprint` struct, `SprintSummary` struct, `BurndownPoint`
  struct (serialisable for the chart), and
  `VELOCITY_MEDIAN_WINDOW = 5`.
- New storage module `peisear-storage::sprints`: full
  lifecycle CRUD (`find_by_id`, `list_for_team`,
  `active_for_team`, `insert`, `update`, `delete`,
  `start`, `complete`, `add_issue`, `remove_issue`,
  `sprint_for_issue`, `issues_in_sprint`, `summary`,
  `burndown`, `recent_completed_for_team`).

### Design

- **Jira-style variable-length sprints, not Linear-style
  rolling cadence.** Each sprint is created with explicit
  start/end dates by an admin. Auto-rolling cadence
  (Linear's "every 2 weeks, automatically") is deferred to
  Phase 2. The reasoning, from design notes: rolling cadence
  encodes a fixed expectation that the team must keep up with
  the calendar; variable-length sprints let the team set
  their own pace per cycle.
- **Velocity chart deliberately doesn't say "velocity".**
  The Jira-popularised word carries "performance" baggage we
  want no part of. The chart heading reads "Completed work
  this period". Bars are neutral colours, not green/red. The
  median reference line is for orientation, not target. We
  don't say "increasing" or "decreasing" anywhere — the user
  reads the picture and forms their own view.
- **Burndown shows facts only.** Two cumulative lines
  (committed and completed); their gap is in-flight. No
  ideal-line diagonal — that would be prescriptive. No
  predicted finish — that would be presumptuous. No
  completion percentage — "70% done" is closer to a verdict
  than to information. The chart shows the *what*; the team
  decides the *so what*.
- **Carry-over is fact, not failure.** Issues remaining when
  a sprint completes are reported as "{N} carried over"
  alongside the completion numbers. The same neutral framing
  used elsewhere in peisear (`Watch` ceiling, no `Concern`).
- **Lifecycle transitions are explicit admin events.** No
  time-based auto-promotion: a sprint stays in `planned`
  until an admin clicks "Start sprint", and stays in
  `active` until an admin clicks "Complete sprint". This is
  V2.1 §4.4 (the *moment of decision* should be a click,
  not a tick of the clock).
- **One active sprint per team at a time.** The application
  refuses to start a second active sprint with a friendly
  conflict message ("Another sprint (X) is currently active
  in this team."). A team that wants two parallel work
  streams should use two teams; the alternative — multi-
  active — turns "is this in flight?" into a multi-answer
  question.
- **Single-sprint-per-issue.** `sprint_issues.issue_id`
  is the primary key, so an issue can be in at most one
  sprint. Moving between sprints is allowed (UPSERT
  ON CONFLICT updates the row). Multi-sprint commitment
  ("this issue is part of sprint A and sprint B") is a
  Phase 2 feature behind a use case we haven't seen.
- **Sprints are a team feature, end of.** Personal projects
  (`team_id IS NULL`) don't get sprint UI: the dropdown
  doesn't appear on their issue pages, and POST attempts
  return a Validation error. The sprint feature is one of
  the things you opt into when you create or join a team.
- **Burndown computed from `issue_events`.** The `done`-event
  history (added in 0.8.0) is the source of truth for "when
  did this issue finish?". Issues that predate the event log
  fall back to `updated_at`. Computed live; no caching.

### Changed

- All five workspace crates bumped to `0.15.0`.
- `IssueDetailPage` now accepts `sprint_options` (a `Vec<(id,
  name)>` of the project's team's planned-or-active sprints)
  and `current_sprint_id`. Personal-project pages pass an
  empty vec. The sprint dropdown card renders only when there
  are options.
- `handlers::issues::detail_page` resolves the project's team
  and lists its sprints when applicable. Cost: one extra
  query per detail-page load for team projects.
- ROADMAP updated: the Sprint feature is shipped (was a
  Medium-term entry). Added a "next phase, ready to start"
  block with a 6-step plan for the **wasm-smtp v0.6** /
  **wasm-smtp-cloudflare v0.6** integration now that those
  releases are out.

### Deferred (future)

- **Auto-rolling cadence (Linear-style).** "Every 2 weeks,
  generate the next sprint" as a per-team setting.
- **Sprint planning page.** A drag-and-drop UI for moving
  multiple issues between the backlog and a planned sprint.
  Today's per-issue dropdown handles the same job at smaller
  scale.
- **Carry-over policies.** Today, completing a sprint leaves
  unfinished issues attached to the now-completed sprint
  (which is why "Carried over" stays meaningful). A future
  setting could move them automatically to the next planned
  sprint, or to the backlog.
- **Cross-team sprints.** A single sprint spanning multiple
  teams is rejected today. If a real use case appears, a
  `sprint_team_assignments` join would be the addition.
- **Multi-sprint commitment.** One issue in two sprints
  simultaneously. The PK on `sprint_issues.issue_id` blocks
  this; relaxing it would be the change.
- **Velocity trend chart on `/me`.** Currently velocity is
  team-level only. A per-user "completed this period across
  all my teams" view would be parallel work.

## [0.14.0] — 2026-04-28

### Added

- **Teams (Phase 1: flat).** Multi-user collaboration via
  optional teams. The design is Linear-inspired: teams are an
  affordance for users who need them, not a requirement.
  Personal projects (no team) continue to work exactly as
  before — the team feature is purely additive for individual
  workflows.
- **`teams` and `team_memberships` tables.** Teams have a
  human name, a URL slug (immutable post-create), and an
  optional description. Memberships carry a role: `admin`,
  `member`, or `viewer`.
- **`projects.team_id` (nullable).** A project either belongs
  to a team (members of the team have access per their role)
  or stays personal (the owner is the only person with
  access — current 0.13.0 behaviour). Null means personal.
- **Three roles**:
  - **Admin**: manage members and team settings (rename,
    update description), move projects in/out, invite
    members, change roles, remove members. Admin is a
    *political* role. **Admin does not gain access to other
    members' personal sustainability data** (burnout panel,
    `/me`); see ROADMAP "Privacy & access control evolution"
    for the rationale.
  - **Member**: full project participation — create / edit
    issues, be assigned, modify capacity within issues.
  - **Viewer**: read-only on team projects. Cannot create
    issues or be assigned.
- **`/teams` inbox**: list of the user's teams with role
  badges. Empty-state copy explicitly tells solo users they
  don't need to create a team.
- **`/teams/new`**: team creation form. Slug is auto-derived
  from the name via `slugify()` (lowercase, hyphenated,
  truncated at 64 chars) but explicitly settable. Creating a
  team auto-adds the creator as `admin` in the same
  transaction.
- **`/teams/{slug}`**: team detail page with members table
  (with inline role-change dropdowns for admins) and projects
  list. Privacy footnote at the bottom of the page reminds
  users about V2.1 §2.5: "admin role is a management role,
  not an oversight role".
- **`/teams/{slug}/edit`**: admin-only settings page (rename,
  update description; slug is immutable).
- **`/teams/{slug}/members`** (POST): admin-only invite by
  email + role assignment.
- **`/teams/{slug}/members/{user_id}/role`** (POST):
  admin-only role change. Refuses to demote the last admin.
- **`/teams/{slug}/members/{user_id}/remove`** (POST): admin
  removes any member, OR self-removal regardless of role.
  Refuses to remove the last admin.
- **`/teams/{slug}/projects/{project_id}/unassign`** (POST):
  admin detaches a project from the team back to personal.
- **Project create form** now offers an optional team
  dropdown when the user belongs to teams where they have
  write capability (admin or member; viewer excluded). The
  form post-validates that the actor really is allowed to
  put a project in the chosen team (defence against
  forged form values).
- **`Teams` link** in the user dropdown nav.
- New core types in `peisear-core::teams`:
  - `TeamRole` enum (`Admin` / `Member` / `Viewer`) with
    `as_str` / `human_name` / `from_storage_str` and
    `can_write` / `can_manage_team` capability helpers.
  - `Team` and `TeamMembership` structs.
  - `slugify(name)` helper with `SLUG_MAX_LEN = 64`.
- New storage module `peisear-storage::teams`:
  `find_by_id` / `find_by_slug` / `teams_for_user` /
  `members_of_team` / `role_for` / `membership` /
  `insert` (transactional with first-admin) /
  `update_team` / `add_member` / `update_role` /
  `remove_member` / `admin_count` (last-admin guard) /
  `assign_project_to_team` / `unassign_project`.

### Design

- **Linear-inspired flat teams, not GitLab-style nested
  groups.** The design discussion considered three patterns:
  fixed N-tier, arbitrary nested, and flat-with-future-parent.
  Linear's "start flat, add structure when it hurts" matches
  peisear's preference for the smallest correct design and
  is what we're shipping. Sub-teams (`parent_team_id`) are
  Phase 2 territory — the schema reserves `team_id` such that
  this is non-breaking.
- **Personal projects remain unchanged.** Existing 0.13.0
  workflows continue to work. The user with one personal
  project doesn't need to know teams exist; the user managing
  five projects across two collaborative groups gets the
  feature when they need it.
- **`team_id` nullable.** "No team" is a valid state, not a
  defaulted-to placeholder. Personal projects are
  distinguishable from team projects in the schema and in
  query results. The list-projects query explicitly handles
  both paths (`p.team_id IS NULL AND p.owner_id = ?` OR
  `p.team_id IS NOT NULL AND member_join.user_id = ?`).
- **Three roles, not five.** GitHub Repo has Triage and
  Maintain in addition to Read/Write/Admin; GitLab has five.
  We picked three because the difference between "can edit
  issues" and "can also do project admin" maps cleanly onto
  the per-team admin role; finer gradations multiply
  complexity faster than they earn it.
- **Role storage as TEXT, not enum.** Adding a fixed role
  later (e.g. `Billing`) is a CHECK-constraint addition, not
  a migration; adding custom (per-team named) roles would be
  a `team_roles` table. Both paths kept open.
- **Last-admin guard at the application layer.** A team
  without admins cannot be managed; the guard is enforced in
  `update_member_role` and `remove_member` before issuing
  SQL. SQLite triggers could express this but are harder to
  test and easier to bypass (e.g. via direct DB access).
- **Privacy floor (V2.1 §2.5).** The team feature ships with
  one absolute boundary: per-user signals (burnout panel,
  personal dashboard) remain visible only to the user
  themselves, regardless of team role. Project-level
  metrics are visible to all team members. Admin role is
  political (manage), not surveilling (read other people's
  data). Future privacy controls (per-team policy, per-user
  hide) are documented in ROADMAP "Privacy & access control
  evolution".
- **Non-members see 404, not 403, on team URLs.** A user
  who's typed `/teams/some-slug` shouldn't be able to confirm
  the team exists. The same posture as GitHub private repos.
- **Self-removal is allowed for any member.** A non-admin
  who joined a team can leave at any time — no need to ask
  permission. The same last-admin guard still applies if the
  leaving user happens to be the last admin.
- **Slug collision is a redirect, not a 4xx.** Form input
  errors round-trip through query string so the user can
  see what was wrong without losing their other inputs. Same
  pattern as the 0.12.0 capacity-conflict UI.

### Changed

- All five workspace crates bumped to `0.14.0`.
- `Project` struct (peisear-core) gained
  `team_id: Option<String>`. Existing callers that don't
  consult this field continue to work (it's just an
  additional field).
- `peisear_storage::projects::insert` now takes
  `team_id: Option<&str>`. The web layer's project-create
  handler validates team membership before passing it
  through; tests confirm forged team_id values are rejected.
- `peisear_storage::projects::find_accessible` now resolves
  team membership via LEFT JOIN: a user accesses a project
  if it's their personal project OR they're a member of its
  team. Existing callers (issue handlers, project page,
  workload chips) get the team-shared access automatically.
- `peisear_storage::projects::list_for_user` returns
  personal projects + team-shared projects. Behaviour-
  preserving for users with no team memberships.

### Deferred (future)

See ROADMAP "Privacy & access control evolution" for the
full list. Highlights:

- **Sub-teams**: one-level parent. Linear added this in
  2025; we'd add it after observing operator usage.
- **Custom roles**: per-team named roles with selectable
  capability sets. Currently nothing in the V2.1 brief
  drives this; opening it up adds complexity we're not yet
  paying for.
- **Per-team privacy policy**: stricter postures
  (admin-can-see-burnout would actually be *less* strict
  than the floor, and we've decided not to allow it; *more*
  strict postures like "all sustainability silent across
  the team" are reasonable Phase 2 work).
- **Per-user privacy controls**: a user opts to hide their
  workload chip from team members.
- **Anonymous aggregation**: project workload as
  median/p90 instead of named per-user chips, for larger
  teams where individual chips create pressure.
- **Cross-team project moves**: today, an admin can detach
  a project to personal and the original creator can
  reassign it. A future "transfer to another team" flow
  would skip the personal-project intermediate state.
- **Email-based invitation flow**: today the invitee must
  already have a peisear account. Email invite tokens are
  a Phase 2 improvement.

## [0.13.0] — 2026-04-28

### Added

- **Notifications subsystem.** Warnings now reach the user
  through an inbox + multi-channel dispatch pipeline, instead of
  living only on whichever page surfaces the warning. V2.1 §1.4
  ("warnings should reach you") is the design driver.
- **Edge-triggered events with cooldown.** Notifications fire
  when a tracked signal *transitions* across a threshold (a user
  goes from `< 8` consecutive over-capacity snapshots to `≥ 8`,
  or from `< 14` stalled-days to `≥ 14`), not on every tick that
  the signal stays elevated. A 24-hour cooldown on
  `(user_id, kind)` further dampens flapping. The combination
  produces "one notice per real change", not noise.
- **Three channels: in-app, email (stub), webhook (stub).**
  - **in-app**: persists a row in the new `notifications`
    table; the inbox at `/notifications` reads and renders
    these.
  - **email**: stub today (logs the dispatch intent at info
    level). Will be wired to `wasm-smtp` once that crate
    releases. No code change to users when that happens — just
    a swap of the `send_via_channel(channel_id::EMAIL)` body.
  - **webhook**: stub today (logs the intent). Real impl will
    POST a small JSON envelope to a per-user URL configured in
    a future settings extension.
- **Inbox at `/notifications`** with read/unread distinction,
  per-row "Mark read", header "Mark all read", filter-free
  newest-first ordering, severity-coloured left border (warning
  for `Watch`, info for `Info`), and contextual "View context →"
  links (burnout kinds → `/me`, project trend → `/projects`).
- **Preferences at `/settings/notifications`** with smart
  defaults and minimal first-time UX:
  - First-login email banner: a single Yes/No prompt
    ("Yes, send me email" / "Just in-app, thanks"). Either
    answer dismisses the banner permanently. The choice is
    recorded as a `_global` row in `notification_preferences`.
  - Per-kind preferences live in a folded `<details>` —
    closed by default; users who want defaults never see it.
    Each row has channel checkboxes and a min-severity
    selector ("All" or "Watch only").
  - "Silence all" link in the header sets every user-facing
    kind's channels to empty. Reachable but not bait —
    notification fatigue is a legitimate concern.
- **Top-nav bell icon with unread badge.** The bell links to
  `/notifications`; the badge shows the unread count (capped
  display at "99+" for sanity). Initially threaded through
  the layout shell so all authenticated pages see it; the
  count is fetched per page render, so it's accurate as the
  user navigates.
- New core types in `peisear-core::notifications`:
  - `Severity` enum (`Info`, `Watch`) with `from_storage_str`,
    `as_str`, `meets_minimum`.
  - `Notification` and `Preference` domain structs.
  - `kind`, `channel` submodules with constants
    (`BURNOUT_OVERLOAD`, `BURNOUT_STALLED`,
    `PROJECT_TREND_DECLINE`, `IN_APP`, `EMAIL`, `WEBHOOK`,
    `GLOBAL`) and `human_name` helpers.
  - Edge detection helpers: `is_edge_into_watch_burnout_overload`,
    `is_edge_into_watch_burnout_stalled`.
  - `DEFAULT_CHANNELS` (`[IN_APP]`), `DEFAULT_MIN_SEVERITY`
    (`Info`), `COOLDOWN_HOURS` (24).
  - `OVERLOAD_STREAK_WATCH` (8) and `STALLED_WATCH_DAYS` (14)
    promoted from inline magic numbers in
    `classify_overload_streak` / `classify_stalled` to public
    constants so the notification edge logic doesn't keep its
    own copy of the threshold.
- New storage module `peisear-storage::notifications`:
  `insert`, `recent_for_user`, `unread_count_for_user`,
  `mark_read`, `mark_all_read`, `last_dispatched_at_for_user_kind`
  (cooldown query), `preferences_for_user`,
  `preference_for_user_kind`, `upsert_preference`,
  `global_acknowledged`, `set_global_acknowledged`,
  `global_preference`.
- New web module `peisear-web::notifications` with
  `dispatch_loop`, `DispatchEvent`, edge detection wrapping,
  channel-specific `send_via_channel` (no-op for in-app, log
  stub for email/webhook).

### Design

- **Two-task architecture (Rust idiomatic).** The
  `snapshot_loop` and `dispatch_loop` are independent tokio
  tasks connected by an `mpsc::channel` of `DispatchEvent`.
  The snapshot loop detects edges and `try_send`s events; the
  dispatch loop drains events, applies preferences and
  cooldown, fans out to channels, and persists audit rows.
  Discussed in design notes:
  - Responsibility separation (snapshot ≠ dispatch).
  - Slow-channel isolation: a 30-second webhook timeout
    blocks dispatch, not the next snapshot tick.
  - Foundation for the planned digest mode (Phase 2):
    receiving events into a different routine is a smaller
    change than refactoring an inline dispatch path.
  - Cooperative shutdown: dropping the snapshot loop's
    `mpsc::Sender` closes the channel, which lets the
    dispatch loop drain and exit naturally. No oneshot
    needed for the dispatcher.
- **Smart defaults and one-question first-login.** Q3=A in
  the design discussion. The default channel list
  (`[IN_APP]`) means a fresh user has working notifications
  immediately. The first-login banner asks one question
  (email yes/no), no more. Per-kind details are folded in a
  `<details>` element. Together: a user who never opens the
  preferences page still gets sensible behaviour, and the
  user who wants more control never finds the UI hostile.
- **Edge-trigger primary, cooldown safety net.** A naive
  every-tick send would generate a notification per snapshot
  for as long as the user is over capacity. Edge-trigger
  reduces this to one notification per actual transition.
  Cooldown then guards against threshold flapping (rapid
  cross-and-back).
- **Audit row always.** A successful dispatch produces a
  `notifications` row with `dispatched_via` listing the
  channels that worked. A failed dispatch (no channels
  succeeded — webhook 500, email TODO) produces a row with
  `dispatched_via = ""`. The user's inbox view is the same
  whether or not external channels worked: the in-app
  artefact is the row itself.
- **Severity ceiling at "Watch", same as the rest of the
  project.** The notifications subsystem inherits the
  `HealthIndicator::Watch` ceiling — no `Concern` palette,
  no escalation. The whole posture is "here is a thing to
  glance at; you decide what to do".
- **In-app row IS the artefact, not a marker for one.** The
  in-app channel doesn't *also* have a side-effect; the
  notifications row is itself the in-app delivery. Saves a
  table and keeps audit and inbox in lockstep.
- **String-typed kind / channel vocabulary, but enums for
  severity.** Kinds and channels are TEXT in storage so new
  ones don't need a migration. Constants in
  `peisear-core::notifications::kind` / `::channel` keep the
  spellings consistent. Severity is an enum because the set
  is small and exhaustive matching is useful.
- **Per-kind row absence = system defaults**, not "delete
  the row". `preferences_for_user` returns only configured
  rows; the web layer merges with `DEFAULT_CHANNELS` /
  `DEFAULT_MIN_SEVERITY`. New kinds shipping in future
  releases work for existing users without backfill.

### Changed

- All five workspace crates bumped to `0.13.0`.
- `AppShell` now takes an `unread_count` prop (defaulting to
  `0`); the navbar renders a 🔔 icon and an unread badge based
  on it. Existing handlers continue to compile (default keeps
  the old behaviour); the notifications/preferences pages and
  any future page can pass the live count.
- `peisear_web::jobs::spawn_all` returns the same shape it did
  before (the `Vec<oneshot::Sender<()>>` for graceful shutdown)
  but internally now spawns two tasks instead of one. The
  dispatch loop's lifetime is tied to the snapshot loop's via
  the mpsc sender drop.
- `capture_one_user` was extended to compute the prior burnout
  state, write the snapshot, recompute the new burnout state,
  and emit edge events. The snapshot persistence path is
  unchanged for callers who don't care about notifications.
- `classify_overload_streak` / `classify_stalled` now reference
  the new public constants (`OVERLOAD_STREAK_WATCH = 8`,
  `STALLED_WATCH_DAYS = 14`) instead of inline literals.
  Behaviour identical.

### Deferred (future)

- **Digest mode**, where a user can opt to receive an end-of-day
  summary instead of per-event notifications. The dispatch
  pipeline is shaped to support this — a digest mode is "drain
  events into a per-user accumulator, flush at a scheduled
  cadence" — but Phase 1 ships per-event only. (Q2=A in the
  design discussion.)
- **wasm-smtp integration for email.** Today the email channel
  is a log-emitting stub. Once `wasm-smtp` releases, we replace
  the body of `send_via_channel(EMAIL)` with the real send.
  No interface change needed.
- **Webhook configuration UI.** Webhook channel exists but
  there's no per-user webhook URL field yet — the stub logs the
  intent without a destination. The UI lands alongside the
  real send implementation.
- **Project-trend decline detection.** The `project_trend_decline`
  kind exists in the vocabulary and the inbox renders it
  correctly when a row is inserted, but the snapshot loop does
  not yet detect the edge. The detection would compare a
  median of the past 7 days' composite scores to the prior 7
  days; ships in a follow-up.
- **Per-page nav unread badge on every page**, not just on
  pages that already pass `unread_count`. The default of `0`
  means existing pages don't show the badge, only the bell.
  This keeps the 0.13.0 diff bounded; full coverage is a later
  pass over each handler.

## [0.12.0] — 2026-04-28

### Added

- **Period-scoped capacity table.** New `user_capacities` table
  with `period_start` / `period_end` (both nullable) lets a user
  describe how much they can carry at a given time:
  - Open-ended row (both NULL) is the default.
  - Bounded row (both set) covers a sprint, a leave window, a
    quarter, etc.
  - Half-open rows (one bound NULL) cover "from a date forwards"
    or "up to a date".
- **CRUD UI on `/settings`.** The Capacity section renders:
  - "Effective today" status banner — shows the points value
    that's currently in effect, or "no capacity set for today"
    if no row covers today.
  - Capacity rows table with inline edit (dropdown form),
    "Close on date" helper for open-ended rows, and remove
    action with `confirm()`.
  - "Add a capacity row" form below the table, with explicit
    points / period_start / period_end / note fields.
- **"(this period)" hint on the Load chip** at `/me`. When the
  row that's currently effective has any period bound, the chip
  surfaces a small italic "(this period)" annotation so the user
  knows their displayed capacity isn't a permanent default.
- New core / storage / web symbols:
  - `peisear-storage::user_capacities` module with
    `effective_for_user`, `effective_for_user_on_date`,
    `effective_row_for_user`, `list_for_user`, `find`,
    `overlaps_existing`, `insert`, `update`, `delete`,
    `close_at`, plus `CapacityRow` and `ConflictInfo` types.
  - `StorageError::Conflict(String)` and
    `StorageError::Validation(String)` for application-level
    rejections that need a useful message.
  - Settings handlers: `insert_capacity`, `update_capacity`,
    `delete_capacity`, `close_capacity`.

### Design

- **Single source of truth.** An earlier draft kept
  `users.capacity_points` as the default and added
  `user_capacities` as overrides. Per design discussion, this
  was rejected: two sources meant "why is my capacity 15?"
  required consulting both, and migration semantics ("the user
  with `users.capacity_points = 10` — is that a default or a
  no-bounds capacity?") were ambiguous. We chose the breaking
  change: drop `users.capacity_points` entirely, make
  `user_capacities` the only source. After this migration, the
  answer is always "look at the row whose period covers the
  date in question".
- **Non-overlapping periods (application-layer enforcement).**
  Periods may not overlap for a single user. SQLite cannot
  express a multi-row constraint at the schema level (CHECK is
  row-local), and triggers are easy to bypass and harder to
  test, so the check lives in
  `user_capacities::overlaps_existing`. Every `insert` and
  `update` calls it first and returns
  `StorageError::Conflict` on overlap, which the web layer
  surfaces via redirect with `?error=...` and a `role="alert"`
  message on the next page render. The narrow race between the
  check and the write is zero in practice for SQLite/WAL; a
  future PostgreSQL backend will use either a transaction or
  an exclusion constraint.
- **NULL bounds mean "infinity".** A row with
  `period_start = NULL` is effective from the dawn of time;
  `period_end = NULL` means "until further notice". This makes
  the migration row (both NULL) trivially compatible with the
  pre-0.12.0 data model, where a single capacity value was
  effective forever. Operators reading a fresh database see
  "10 pt: — to —" and understand it without docs.
- **`close_at` as a UI-shaped helper.** The common workflow is
  "I have an open-ended row; I want to add a new row starting
  next month, but they would overlap." Rather than make the
  user issue a manual `update` to set `period_end` first, we
  ship `close_at(row, date)` as a dedicated endpoint at
  `/settings/capacity/{id}/close`. The UI exposes it as a
  "Close on date" dropdown only on rows that need it
  (open-ended ones).
- **No schema-level CHECK for overlaps; explicit comment in the
  migration explains why.** The migration file documents the
  schema-vs-application boundary so a future operator (or
  future maintainer) reads the rationale before reaching for
  triggers.
- **Snapshot honesty preserved.** `user_metrics_snapshots`
  (0.10.0) records `capacity_points` at write-time, so any
  past snapshot continues to reflect what the user's capacity
  *was* on that date. The 0.12.0 schema migration doesn't
  touch existing snapshot rows; future writes continue to
  resolve through `effective_for_user_on_date`.

### Changed

- **Breaking: `users.capacity_points` removed.** Migration
  `0009_user_capacities.sql` migrates each user's existing
  value into a single open-ended `user_capacities` row, then
  drops the column. SQLite ≥ 3.35 supports this; we depend on
  3.40+.
- **`User` core type still has `capacity_points: Option<i64>`,**
  but it's now populated by callers (handlers, components)
  through `user_capacities::effective_for_user` rather than
  read off the row. Pre-existing code that reads
  `User.capacity_points` continues to compile but always gets
  `None` from `users.rs`; the right call is to consult
  `personal_metrics::for_user_*` (which handles resolution
  internally).
- **`personal_metrics::for_user_in_project` and `for_user_global`**
  now resolve capacity through `user_capacities`. The
  `PersonalMetrics::capacity_points` field semantics are
  unchanged for callers.
- **`issues::project_workload`** SQL now joins against
  `user_capacities` via correlated subselect (`WHERE
  period_start IS NULL OR period_start <= date('now')` AND
  symmetric for end). Result shape (`Vec<UserLoad>`) unchanged.
- **Settings page restructured.** The single capacity input is
  replaced by the rows table + add form. WIP-limit form is
  unchanged.
- **All five workspace crates bumped to 0.12.0.**

### Deferred (future)

- **Schema-level overlap constraint via trigger.** If
  application-layer enforcement turns out to leak in practice
  (multiple writers, complex flows), a `BEFORE INSERT/UPDATE`
  trigger raising on conflict is the natural fallback. Today's
  bet is that the application layer is reliable enough; we'll
  revisit if operator reports say otherwise.
- **Sprint table integration.** Adding a sprint should
  optionally auto-create matching capacity rows (period bounded
  by sprint dates). Lands with the planned sprint feature.
- **Per-day breakdown rendering.** A small visual showing
  "today is 4/28 — your row covers 4/15 → 4/30, with 12 days
  remaining" is a small addition that we don't ship today
  because the table already conveys the information.
- **Bulk import.** A future "paste a CSV of period rows" form
  would help operators set up sprints in bulk. Today's manual
  add-one-at-a-time is fine for low-frequency use.

## [0.11.0] — 2026-04-28

### Added

- **Estimation drift trend.** The `<Sustainability>` panel on
  `/me` now shows a "Pace drift" chip comparing the median
  dwell-time-per-point of the recent two weeks vs. the prior two
  weeks across a 28-day window. Reports `↑ longer per point`,
  `↓ shorter per point`, or `→ steady` (within ±25%). Both
  medians render visibly on the chip
  (`recent 0.45 vs. 0.20 d / pt`) so the user has the "by how
  much" answer at a glance.
- **Cognitive switching pattern.** A "Switching" chip in the
  same panel surfaces the median count of `-> in_progress`
  events per active day over the past 14 days, alongside the
  sample size (`5 / active day`, `42 events over 14 d`). The
  median plus sample period together let the user contextualise
  the number — "5 / day over 14 days" reads quite differently
  from "5 / day over 4 active days".
- **Insufficient-data chips.** When either signal cannot be
  computed (drift needs both halves of the window populated;
  switching needs ≥5 events), the chip renders explicitly as
  `— need more data` with a clear aria-label, rather than
  vanishing. The user can tell the difference between "this
  signal is steady" and "this signal isn't computable yet".
- New core types in `peisear-core::user_burnout`:
  `EstimationDriftTrend`, `DriftDirection` (Up / Down / Steady),
  `CognitiveSwitchingPattern`. New constants
  `DRIFT_WINDOW_DAYS = 28`,
  `DRIFT_STEADY_THRESHOLD_RATIO = 0.25`,
  `SWITCHING_WINDOW_DAYS = 14`, `SWITCHING_MIN_EVENTS = 5`.
- New storage functions in `peisear-storage::user_burnout`:
  `estimation_drift_for_user` (event-time-bucketed median per
  half), `cognitive_switching_for_user` (per-day grouped count
  with active-day median).
- New "Patterns" subsection in the `<Sustainability>` panel,
  visually separated from the warning chips, with its own
  framing footnote: "These are descriptions of recent rhythm,
  not evaluations. Many patterns have legitimate reasons behind
  them."

### Design

- **Patterns are not warnings.** The two new chips deliberately
  use the neutral `badge-ghost` palette. Drift up does not
  warrant a `Watch` palette and the type system reflects this:
  `classify_drift` returns `DriftDirection`, not
  `HealthIndicator`. Switching has no classifier at all because
  there is no threshold that meaningfully separates "good"
  switching (debugging, coordination) from "bad" switching.
- **Drift uses the same median-vs-median comparison shape as
  the project trend chip** (`peisear-core::project_health::
  classify_trend`), but applied within a single user's
  completed work. Symmetry across the codebase: when we compare
  past to present, we use medians and explicit windows, not
  means or single points.
- **Why median over active days for switching, not calendar
  days.** A user who works in bursts (intensive 3-day pushes
  then quiet days) should see their rhythm represented by the
  rhythm of those 3 days, not diluted by the quiet ones. The
  question we're trying to answer is "what does a typical
  active day look like for you?", and active days are the
  right unit.
- **Insufficient data is shown, not hidden.** Both new chips
  render an explicit `— need more data` state when their
  evidence is below threshold. Earlier prototypes hid the chip
  on `None`; we changed to explicit rendering because per V2.1
  §4.4 (説明可能性), telling the user "we don't have enough
  yet" is information; silent absence is not. The distinction
  between "this signal is steady" and "we can't tell yet" is a
  user-meaningful one and the UI should preserve it.
- **Visible context, not tooltip-only.** Both chips show the
  raw numbers behind the headline (drift's two medians,
  switching's sample period) on the chip face, not in a
  tooltip. A tooltip would hide the "by how much" answer
  behind a hover; visible numbers let the user agree or
  disagree with the headline at a glance.
- **`summarize()` is unchanged.** Drift / switching are not
  added to the panel's natural-language summary. The summary is
  for warnings ("here's what to glance at"); pattern facts are
  separate and clearly labelled as descriptive. Mixing them
  into the warning summary would conflate "act on this" with
  "notice this".

### Changed

- Workspace and all five crates bumped to `0.11.0`.
- `peisear-core::user_burnout::UserBurnoutSignals` gained two
  optional fields: `estimation_drift: Option<EstimationDriftTrend>`
  and `cognitive_switching: Option<CognitiveSwitchingPattern>`.
- `peisear-storage::user_burnout::for_user` now also computes the
  two new signals as part of its single call. Same return type
  (`Option<UserBurnoutSignals>`) so callers don't change.
- `<PersonalDashboard>`'s sustainability panel now renders a
  Patterns subsection alongside any streak chips. The panel's
  hide-condition is unchanged (panel hidden only when no streak
  signals AND no computable patterns); the addition is that an
  insufficient-data chip is treated as "showable" — the
  Patterns block always renders when the panel is open at all.

### Deferred (future)

- **Per-project drift / switching scope.** Today these are
  computed across all of a user's assigned work. A future
  variation might offer per-project breakdowns when the user is
  in many projects of distinct character. Probably waits until
  the planned Team / organisation feature lands; the per-project
  story makes more sense in that context.
- **Drift direction's UI semantics.** Right now both `↑` and
  `↓` use the same neutral palette. If user testing reveals
  that a small visual differentiation (still no warning palette,
  just clearer affordance) helps comprehension, we'll add it.
  Today's choice is "let the arrow do the talking".

## [0.10.1] — 2026-04-28

### Added

- **Five new operations docs** under `docs/operations/`:
  - [`background-jobs.md`](docs/operations/background-jobs.md) —
    what the snapshot loop does, what it costs, how to observe
    it, what to expect after restarts.
  - [`data-retention.md`](docs/operations/data-retention.md) —
    growth rates of `issue_events`, `metrics_snapshots`, and
    `user_metrics_snapshots`, the privacy posture around each,
    retention-policy options the operator can choose between,
    and the SQL to actually delete old rows.
  - [`upgrade-runbook.md`](docs/operations/upgrade-runbook.md) —
    pre-upgrade checklist, how to dry-run a migration, the
    forward-only stance and rollback boundary, per-version
    notes for 0.7.0+.
  - [`observability.md`](docs/operations/observability.md) —
    `tracing` log levels, structured fields, alerting threshold
    suggestions including the SQL-based "no successful tick in
    24 hours" check, debugging checklist for common symptoms.
  - [`scaling.md`](docs/operations/scaling.md) — when SQLite is
    enough, the signs it isn't, the easy levers before
    PostgreSQL, what the background task does under load, the
    "just one binary" promise's boundary.
- The operations README now organises docs into "Day-one" and
  "Day-two" sections so first-time deployers and longtime
  operators can find what's relevant.

### Design

- **The new docs do not change the binary.** This is a
  documentation-only release. The runbook and observability
  posture they describe match what 0.10.0 already does; the
  effect is that operators of existing 0.10.0 installations now
  have written guidance instead of reading source.
- **Honest about limits.** The docs spell out what peisear does
  *not* yet do (no `/healthz` endpoint, no Prometheus metrics, no
  built-in retention cleanup, no PostgreSQL backend) and what
  the workarounds are. The intent is to set realistic operator
  expectations, not to oversell the small-binary story.
- **Cross-linked.** Each new doc links to its siblings so an
  operator chasing a question can navigate without going back to
  the README. Same convention as the existing `architecture/`
  and `getting-started/` directories.

### Changed

- Workspace and all five crates bumped to `0.10.1` for
  consistency with the doc release. No code changes.

## [0.10.0] — 2026-04-27

### Added

- **Per-user sustainability signals.** A new `Sustainability`
  panel on `/me` shows two streak-style indicators: the
  consecutive-snapshot over-capacity streak, and the days since
  the user's oldest in-flight issue last had a status change.
  The panel is muted by default and opens itself only when at
  least one indicator reaches `Watch`. The framing is question-
  form ("consider whether some work can wait"), not diagnosis.
- **`user_metrics_snapshots` table** — per-user point-in-time
  history written by the background task on the same tick as
  `metrics_snapshots`. Stores current WIP, in-flight points,
  capacity, the over-capacity boolean (denormalised), the
  effective WIP limit, and the over-WIP boolean (denormalised).
- **`peisear-storage::user_burnout`** module with `for_user`
  computing the streak signals from snapshots + events.
- **`peisear-storage::user_metrics_snapshots`** module with
  `insert`, `recent_for_user`, `users_with_active_assignments`.
- **`peisear-core::user_burnout`** module with the
  `UserBurnoutSignals` type, classification functions
  (`classify_overload_streak`, `classify_stalled` — both
  deliberately ceiling at `Watch`, never `Concern`), and the
  pure-function `summarize` that produces the natural-language
  prompt.
- The background snapshot task now also runs `capture_all_users`
  on each tick, writing one `user_metrics_snapshots` row per
  user with at least one in-flight assigned issue.

### Design

- **A separate per-user table, not new columns on
  `metrics_snapshots`.** Privacy boundaries differ: project-
  level metrics are visible to anyone who can see the project;
  per-user metrics are visible only to that user (and,
  eventually, manager / neutral-third-party roles). Putting
  them in the same table would either bleed access boundaries
  (rows from one user visible alongside aggregate data) or
  push the privacy logic to the row level. Schema-level
  separation keeps the access control tractable; this
  matches V2.1 brief §2.5 ("集計と個別を混同しない").
- **`Watch` is the ceiling for burnout indicators.** No
  `Concern` palette appears in the sustainability panel under
  any condition. "Concern" framing on burnout territory
  crosses into "you are burnt out" telling, which is the
  user's call to make about themselves. The classification
  functions in `peisear-core::user_burnout` have no `Concern`
  branch at all, so the type system enforces this design.
- **Question-form summary text.** `summarize()` returns
  prompts like "you've been over capacity for X recent
  snapshots — consider whether some work can wait or move",
  not "you are over capacity". Same posture in the panel
  footnote ("they exist so you can pace yourself"). Per V2.1
  §5.3, alarming language is avoided even when the signal is
  real.
- **Honest unit labelling.** The over-capacity streak is
  expressed as "snapshots" rather than "days". The snapshot
  tick rate is configurable (`SNAPSHOT_INTERVAL`) and the
  count is mechanical; calling it "days" would imply daily
  resolution that the data doesn't have. The window for the
  streak ("of last 14") is the calendar number where
  resolution doesn't matter.
- **Phase 1 of user burnout ships only two indicators.** Two
  more were considered (estimation drift trend; cognitive
  switching frequency) and explicitly deferred. They both
  require more SQL and, more importantly, their interpretation
  is more context-dependent — drift can mean estimation is
  off, but it can also mean recent work was unusually hard;
  switching frequency is high during legitimate debugging
  sessions. Shipping interpretive signals without surrounding
  context would hand the user a number they can't act on.
  Deferring keeps the panel's claims tight.

### Changed

- Workspace and all five crates bumped to `0.10.0`.
- The background snapshot task (`peisear-web::jobs`) now does
  two passes per tick: project snapshots and user snapshots.
  Both fail-tolerantly per row; the loop does not exit on
  errors.
- `peisear-web::handlers::me::page` now also fetches
  `user_burnout::for_user` and threads `Option<UserBurnoutSignals>`
  into the dashboard render.
- `<PersonalDashboard>` gained a `burnout` parameter and a new
  `<Sustainability>` section. The "What do these mean?"
  collapsible was extended with the new section's explanation.
- `<PersonalDashboard>`'s "Pace" description was corrected to
  reflect the 0.8.0 reality (event-based dwell time, not
  calendar approximation). The description-update is a
  documentation-only change.

### Deferred (future)

- **Estimation drift trend** — week-over-week median of
  `Pace`, with `Trend::Up { delta }` semantics on the personal
  side. Builds on what's here; drops in additively.
- **Cognitive switching frequency** — same-day status_changed
  event count per user, surfaced as a streak / rolling
  average. Defers until we have a UX answer for "high
  switching is sometimes correct".
- **Manager / neutral-third-party scopes.** The V2.1 brief
  calls for view scopes beyond `self`; arrives with the
  planned Team feature, since it requires a permission model
  and a UI for granting / revoking the access. Until then,
  `/me` is the only path to per-user data.

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

[Unreleased]: https://github.com/nabbisen/peisear/compare/v0.16.0...HEAD
[0.16.0]: https://github.com/nabbisen/peisear/releases/tag/v0.16.0
[0.15.0]: https://github.com/nabbisen/peisear/releases/tag/v0.15.0
[0.14.0]: https://github.com/nabbisen/peisear/releases/tag/v0.14.0
[0.13.0]: https://github.com/nabbisen/peisear/releases/tag/v0.13.0
[0.12.0]: https://github.com/nabbisen/peisear/releases/tag/v0.12.0
[0.11.0]: https://github.com/nabbisen/peisear/releases/tag/v0.11.0
[0.10.1]: https://github.com/nabbisen/peisear/releases/tag/v0.10.1
[0.10.0]: https://github.com/nabbisen/peisear/releases/tag/v0.10.0
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
