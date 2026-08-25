# RFC 0005: Quality consolidation

**Status**: Proposed
**Target**: 0.27.0 (Phase E) — §9 shipped at 0.22.0, §10 pulled forward to 0.25.0,
§12-13 added from `REL-0.26.0`'s review
**Related spec sections**: §40 (Phase E plan), §11.5.5 (API
authorization QA), §21.4 (optimistic-lock conflict),
§30-34 (ABDD axes)
**Last updated**: 2026-08-25

## Summary

A QA pass over everything Phase A–D shipped: keyboard,
screen reader, mobile, language consistency, colour contrast,
and security (authorization boundaries + optimistic-lock
behaviour). No new feature work; the goal is that every
shipped surface satisfies the six ABDD axes (§30) plus the
two security axes (§40.1.6).

This is the Phase that closes the few `#[ignore]`d tests
left in 0.19.0 and audits the surfaces that grew piecemeal
across PRs.

## Background

Phases A–D each shipped surfaces with their own feature
requirements; ABDD acceptance was checked at the PR level.
Phase E is the "step back and look at the whole" pass. Two
specific drivers:

1. The spec §40 explicitly schedules QA work as its own
   phase rather than baking it into every feature PR. Phase
   E is the slot where keyboard navigation (`j/k`), WCAG AA
   contrast, and mobile completion get systematically
   addressed instead of opportunistically.
2. The security axes (§40.1.6) were added in v2.1
   specifically because the §11.5 boundary and §21.4
   optimistic-lock contract are easy to *almost* implement
   correctly — the "team admin reads member burnout"
   edge case in B-PR2 was a textbook example. Phase E
   inventories every entry point and confirms the
   boundaries hold.

## Requirements

### Six ABDD axes (§30, every surface)

The six axes restated for clarity:

1. **Keyboard** — every action reachable.
2. **Focus** — visible focus ring; predictable post-action
   focus location.
3. **Screen reader** — meaningful announcements; no
   dead-end "graphic" labels.
4. **Color** — information conveyed by more than colour
   (icon, text, position).
5. **Mobile** — the four key flows (Today / Inbox / Issue
   detail / Calendar today-view) complete on a phone.
6. **Live update** — dynamic changes (status flip,
   notification arrival) announce via aria-live.

### Two security axes (§40.1.6)

7. **Authorization** — every surface that handles personal
   data refuses cross-user reads (other user's auth token
   → 403; admin reading member personal data → 403; no
   auth → 401 JSON for `/api/*`, 303 to login for HTML).
8. **Concurrency** — every mutation that owns an entity
   honours the §21.4 optimistic-lock contract; conflicts
   surface as 409 with the structured body and the UI
   rolls back without celebratory language.

### Must-haves

1. Every surface listed in §40.2 receipt 1–6 passes.
2. Authorization audit table populated (see Design §1).
3. Optimistic-lock audit table populated.
4. The single ignored test from 0.19.0
   (`cross_user_settings_post_returns_403`) is activated
   or deleted with cause.
5. Keyboard navigation `j/k` works on issue lists and
   kanban (long-promised in §32).
6. Locale audit complete: English UI strings only (the
   mixed-language drift the spec calls out in §40.1.4 is
   resolved).
7. Colour contrast audit run against every page: WCAG AA
   4.5:1 minimum.

### Nice-to-haves

- Snapshot-test contrast values per theme so future Tailwind
  upgrades don't silently regress them.
- Lighthouse score ≥ 95 for the four key flows (proxy
  metric; not a hard requirement).
- Bundle size: `static/dm.js` (RFC 0004) under 8 KB
  uncompressed.

### Explicitly out

- New features. If a new feature is found *necessary* during
  the audit, it gets its own RFC and ships in a later
  version.
- Cross-team aggregation surfaces. The spec is unambiguous
  about not adding them; an audit isn't license to invent
  them.
- Refactoring for the sake of refactoring. Phase E touches
  code only where the audit found a defect.

## Design

### 1. Authorization audit

Build a table — owned by this RFC and updated as the audit
proceeds — listing every endpoint that carries personal
data (§11.5.1) or per-user mutation:

| Endpoint | Auth check | Cross-user test | Status |
|---|---|---|---|
| `GET /today` | AuthUser → self via cookie | implicit (no user_id) | ✓ 0.18.0 |
| `GET /today/calendar` | AuthUser | implicit | (added in PR3) |
| `GET /inbox` | AuthUser | implicit | ✓ |
| `GET /api/users/{id}/burnout` | ApiAuthUser + require_self | `auth_boundary::burnout_endpoint_walls_off_other_users` | ✓ 0.18.0 |
| `GET /api/users/{id}/capacity` | same | `..._capacity_..._other_users` | ✓ |
| `GET /api/users/{id}/notifications` | same | `..._notifications_..._other_users` | ✓ |
| `GET /settings` | AuthUser | implicit | ✓ |
| `POST /settings/wip-limit` | AuthUser | **(needs test once user_id surface lands — see ignored test)** | partial |
| `POST /settings/capacity*` | AuthUser, lock-checked | implicit (no user_id in URL) | ✓ |
| `POST /inbox/resume` | AuthUser | implicit (no user_id) | ✓ 0.24.0 |
| `POST /inbox/email-opt-in` | AuthUser | implicit | ✓ 0.24.0 |
| `GET /projects/{id}/delete` | AuthUser + `find_accessible` + **owner check** | **unaudited** | 0.25.0 |
| `GET …/issues/{iid}/delete` | AuthUser + `find_accessible` | **unaudited** | 0.25.0 |
| `GET …/sprints/{sid}/delete` | AuthUser + `can_manage_team` | **unaudited** | 0.25.0 |
| `POST …/status/detail` | AuthUser + `find_accessible` | **unaudited** | 0.25.0 |
| `POST …/status/list` | AuthUser + `find_accessible` | **unaudited** | 0.25.0 |

*Reconciled 2026-08-25.* The "fill in as they ship" row was a promise to a
future that has now arrived twice — 0.24.0's two inbox routes and 0.25.0's
five. Filling it in was nobody's step, which is the same shape as a status
nothing checks.

**The three `GET` delete rows are new in kind, not only in number.** Before
0.25.0 no `GET` in this application rendered a page whose only purpose was to
authorise a mutation. `GET /projects/{id}/delete` carries an owner check that
`GET /projects/{id}` does not, so the interstitial is a **narrower** boundary
than the screen that links to it. Audit whether the `POST` half enforces the
same narrowing — a `GET` that refuses and a `POST` that does not is a boundary
that exists only in the user interface.

The audit fills in the right-hand column for every row. New
test entries land in `tests/auth_boundary.rs`. The ignored
test gets either a fresh body (if a user-scoped POST has
landed by then) or a `// removed: no user-scoped POST surface
exists; revisit if added` comment in the test file with the
test removed.

### 2. Optimistic-lock audit

Symmetric table:

| Mutation | Lock check | Conflict test | Rollback UI |
|---|---|---|---|
| `POST /projects/{id}/issues/{iid}` (update) | yes | `optimistic_lock::issue_update_with_stale_timestamp_returns_409` | (D-1 + D-3) |
| `POST /projects/{id}/issues/{iid}/status` | yes | `..._status_change_..._returns_409` | (D-1) |
| `POST /projects/{id}` | yes | `..._project_update_..._returns_409` | n/a (HTML form, page reload) |
| `POST /teams/{slug}/sprints/{sid}/edit` | yes | `..._sprint_start_..._returns_409` (analogous; covers update path) | n/a |
| `POST /teams/{slug}/sprints/{sid}/start|complete|delete` | yes | covered above | n/a |
| `POST /settings/capacity/{id}` | yes | `..._capacity_period_edit_..._returns_409` | n/a |
| `POST /teams/{slug}/sprints/{sid}/plan/add|remove` | **no** (join-table, intentional) | n/a | n/a |
| `POST …/status/detail` \| `/status/list` | yes — shared `apply_status_change` | `status_control`, `optimistic_lock` | D-1 (0.26.0) |
| `POST /teams/{slug}/sprints/{sid}/delete` | **yes** | — | n/a |
| `POST /settings/capacity/{id}/delete` | **yes** | — | n/a |
| `POST /projects/{id}/delete` | **no** | — | n/a |
| `POST /projects/{id}/issues/{iid}/delete` | **no** | — | n/a |

*Reconciled 2026-08-25, and the last four rows are the finding.*

**Two of the four destructive deletes take a lock and two do not.** Verified by
reading the handlers: `sprints::delete_sprint` and `settings::delete_capacity`
call `check_optimistic_lock`; `projects::delete` and `issues::delete` take no
form at all — no `client_updated_at` reaches them.

**The mechanism for it exists and is used once.**
`render_delete_confirmation` takes a `hidden_fields: Vec<(String, String)>`
parameter. The sprint interstitial passes `client_updated_at` through it. The
project and issue interstitials pass `Vec::new()`.

**RFC 010 widened the window this protects.** A delete used to be one `POST`
from a page the user was looking at. It is now `GET` (render "Delete issue:
Fix login bug"), a pause while the user reads, then `POST`. Nothing binds the
`POST` to the state the `GET` displayed, so a user can confirm a deletion of an
entity that has changed underneath the sentence naming it — and for issues that
matters more than for most entities, because the delete **cascades**.

Whether a delete *should* lock is a real design question and this RFC does not
prejudge it: the row is gone either way, so a stale timestamp corrupts nothing.
What is not defensible is deciding it differently in four places without
recording a reason. The audit settles it once, in both directions.

**A copy finding from the same read, recorded here because it is the same
window.** `issues::delete_confirm` renders `ConfirmDeleteCannotBeUndoneNote` —
*"This cannot be undone."* — while `projects::delete_confirm` renders
*"All its issues will be deleted too. This cannot be undone."*
`issues.parent_issue_id` is declared `ON DELETE CASCADE` (`0015_sub_issues.sql`)
and the pool sets `foreign_keys(true)`, so **deleting a parent issue deletes its
sub-issues** and the screen naming what will be deleted does not say so. RFC 010
exists so a confirmation states its consequence; on the one route where the
consequence exceeds the entity named, it does not.

For mutations newly added in C-PR2/PR3/PR4 and Phase D, the
audit confirms each entry has a row.

### 3. Language audit

Run a script that scans templates and Rust string literals
for known Japanese characters (Hiragana, Katakana, common
Kanji). Output a list of every non-comment string that
contains them. Each gets converted to English; comments stay
in the language they were written in (most are already
English).

The CHANGELOG keeps both languages where needed. Spec stays
Japanese (it's a separate document, not user-visible).

### 4. Colour contrast audit

Use an off-the-shelf checker (e.g. `tailwindcss-contrast`,
or a manual pass with the WebAIM checker against the
documented colour pairs). Document the audit results in
`docs/src/accessibility.md` (new), with a table of
foreground/background pairs and their measured ratios.

If a Tailwind class fails AA, replace it with the next
darker/lighter variant. Do not add custom colours; the
DaisyUI theme tokens cover the cases we have.

### 5. Keyboard navigation

`j` moves selection down, `k` moves selection up. Apply
on:

- Issue list (selection is the row; Enter opens detail).
- Kanban (selection follows the card; Enter opens detail;
  with the D-2 work, Space picks up).
- Sprint plan (selection follows the row; Space moves it
  to the other column, mirroring D-4).

Hint footer: a small "Press `?` for keyboard shortcuts" link
at the bottom of pages with shortcuts. Pressing `?` opens
a modal with the binding list. Implementation: a small JS
file `static/keynav.js` (vanilla, no Leptos hydration).

### 6. Mobile completion

Manual QA against the four flows:

- Today: panels render at narrow width; the
  what-to-read-first callout doesn't truncate; rhythm
  details still expandable.
- Inbox: list scrolls; mark-read tap target ≥ 44 px.
- Issue detail: edit form uses native pickers; save button
  doesn't sit under a virtual keyboard.
- Calendar today-view: day view (the mobile default per
  RFC 0002) renders without horizontal scroll.

Document each flow's mobile screenshots in
`docs/src/mobile-checklist.md` for regression visibility.

### 7. Aggregate inferability check

§40.1.6 last bullet: "the workload chip with N=1 member
trivially leaks that member's individual data." Audit
aggregate surfaces:

- Workload chip on project detail. If only one member has
  open issues, the chip shows only that member's effort.
  Decision: render the chip at all, or suppress it when
  N=1?

  *Default: suppress when N < 2 with a tooltip "individual
  workloads are visible on each issue page." This matches
  §11.5 (aggregate vs individual), and the chip's value
  scales with team size.*

  Apply same logic to:
  - Capacity hint on the sprint plan page (RFC 0001) when
    N < 2 contributors.
  - Per-assignee mini-rollup if it lands in 0001 (likely
    the same suppression).

### 8. Phase A-D follow-up sweep

Final pass to confirm the original Phase A-D items still
satisfy ABDD/security after later PRs landed. Grep for:

- `// TODO` and `// FIXME` in handlers and components.
- `#[ignore]` in tests outside `tests/auth_boundary.rs`.
- `unimplemented!()` and `todo!()` in shipped code.

Each gets resolved: either fixed, ticketed for a future RFC,
or annotated with cause.

### 9. The test harness itself — pulled forward to 0.22.0

*Added 2026-08-11 from baseline `§10.13`, found while reviewing
`REL-0.21.0`.*

`TestApp::spawn` names its temporary database directory from
`SystemTime::now().as_nanos()` alone. Two tests entering it in
the same clock tick share a directory and a `test.db`;
`create_dir_all` succeeds on an existing directory, so nothing
signals the collision and the second arrival fails with
`SqliteError { code: 5, "database is locked" }`. Roughly one
`cargo test --workspace` run in two, on a different test each
time. Reproduced at `0.20.1` as well, so it is not new.

**Why it belongs to this RFC.** Phase E is where test debt is
paid, and this is test debt of the most consequential kind:
the harness that produces every gate result in this project.

**Why it does not wait for 0.24.0.** Every release from here
runs those gates. A suite that fails half the time on the
obvious command trains people to re-run rather than read, and
that habit is what a flaky test costs — not the minutes.

Two parts:

1. Make the suffix unique — process id and an atomic counter
   alongside the clock, or a crate that guarantees it. Prefer
   the crate: a hand-rolled unique-name scheme is what failed.
2. **Add a repeated full-workspace run to the gate set.**
   `DEC-007` mandates per-crate and per-target runs *for
   isolation*, and that procedure never triggers the
   collision. Every gate log this project has captured is
   honest and green, and the defect lived underneath all of
   them.

Item 2 is the finding, not item 1. **An isolation procedure
adopted to make results trustworthy hid a defect in the thing
producing them.** A gate set needs at least one run under the
conditions a contributor will actually use, or it measures
only the conditions it chose.

### 10. Three defects from CONF-001's review — pulled forward to 0.25.0

*Added 2026-08-16, from `.git-exclude/reviewed/CONF-001-review.md` §4–§6.*

Three independent defects surfaced while reviewing `CONF-001`. None is a
feature, none needs its own RFC, and all three are the kind of thing Phase E
exists to sweep up — so they arrive here rather than inventing a home, the same
way §9 did.

**They are pulled forward to 0.25.0** because two of them are reachable today
and the third makes a guard harder to use than it should be.

#### 10.1 An active sprint can be deleted

`handlers::sprints::delete_sprint` resolves membership, checks
`can_manage_team()` and the team match, verifies the optimistic lock, and
deletes — **for any status, `Active` included**.

The UI does not link delete for an active sprint, so this was read as a dead
path during `CONF-001`'s review. It is not: the route is live and destructive,
and `CONF-001`'s new confirmation `GET` will happily render "you are about to
delete *X*" for a team's running sprint.

**Owner decision, 2026-08-16: an active sprint may not be deleted.** At most one
sprint per team is active — `OtherSprintActiveInTeamMessage` exists because of
that — so the live one is not equivalent to a planned one, and deleting it
silently discards the state a team is currently working in.

The path out already exists: complete it, then delete it.

#### 10.2 ~~Project delete reports success to a non-owner~~ — **withdrawn, the defect does not exist**

*Withdrawn 2026-08-16, before implementation.* This entry claimed
`handlers::projects::delete` relied entirely on the storage layer's
`WHERE owner_id = ?2`, so that a non-owner deleted zero rows and was told the
project was deleted.

**False.** `peisear_storage::projects::delete` ends with
`if res.rows_affected() == 0 { return Err(StorageError::NotFound); }`, present
since v0.2. A non-owner's `POST` has always returned 404 with the project
intact.

The error was mine: I read that function through its `DELETE`, its `WHERE` and
its binds, and stopped three lines before the check that makes it correct. The
claim then travelled — a review, this section, and a dispatched handoff, each
citing the last. `QA-002`'s implementer checked empirically before implementing
and reported it.

**Kept rather than deleted**, because the register's own rule is that closed and
withdrawn items stay with their resolution. A section that quietly vanishes
teaches nothing; this one records that a defect can be manufactured by reading
most of a function.

Nothing to fix. The handler-level check `QA-002` added on this entry's
instruction is reverted — RFC 005's own "explicitly out" forbids refactoring
where the audit found no defect, and `rows_affected() == 0 → NotFound` **is**
the authorisation outcome, deliberately, not an implicit signal being abused.

#### 10.3 `prose_scan` scans comments as if they were code

A doc comment quoting attribute markup — `onsubmit="return confirm(...)"` in
prose — fails `prose_scan`. Reproduced in review.

**The sibling guard already fixed this.** `test_harness_scan`'s first iteration
false-positived against its own doc comment for exactly this reason, and
QA-001's round-1 correction was `strip_line_comments`. `prose_scan` strips only
`#[cfg(test)]` blocks and never received it.

Two guards, one false-positive class, one fixed and one not — the lesson stayed
local to the file that learned it, which is the more interesting half of this
entry.

### 11. Redirect construction — added 2026-08-16, from STATUS-001's review

Three handlers build a redirect by interpolating caller-supplied values straight
into a query string, unencoded:

- `handlers::sprints::plan_query_string` (`:594`) — PLAN-001, 0.22.0
- `handlers::issues::change_status_form_list` — STATUS-001, 0.25.0
- and the pattern is available to be repeated wherever a filtered view needs
  preserving across a POST

Meanwhile `percent_encode_query` exists in **two copies**
(`handlers/teams.rs:391`, `handlers/sprints.rs:829`), used for flash text and
not for these.

**Severity: low, and established rather than assumed.** axum 0.8.9's
`Redirect::into_response` does `HeaderValue::try_from` and returns a 500 with
the error string on failure — it does not panic and does not emit a split
header. A value containing `&` appends parameters to the redirect; the receiving
handlers read only known ones. There is no injection and no crash.

**What makes it audit material is the shape, not the risk.** Three construction
sites and two copies of the encoder they should be using is the
two-homes-for-one-fact pattern this project has now recorded five times, and a
redirect is a sink with different rules from the query parameter the value
arrived as — which is the reasoning STATUS-001's review request got slightly
wrong and is worth settling once.

§1's authorisation table has a natural sibling here: every place the application
constructs a redirect, and what encodes it.

### 12. Script tags nothing asserts

*Added 2026-08-25 from `REL-0.26.0`'s review, found by planting.*

`components/issues.rs:142` emits `<script src="/static/board.js" defer>`.
Delete that line and change nothing else, and **`cargo test --workspace`
still reports 178 passing**. The board ships with no drag-and-drop and no
undo, and every gate is green. Verified, not reasoned about.

`status_control.rs:485` states the opposite in a comment — that the board
"loads `board.js` instead — `boards_per_card_control_renders_unchanged`
above already pins that." That test asserts the board posts to
`/status/board` and does not pick up the two new routes. It never looks
for `board.js`.

Three files live in `static/` and exactly one of them is asserted
anywhere:

| File | Tag emitted at | Asserted by |
|---|---|---|
| `dm.js` | `components/issues.rs:564` | `status_control::dm_js_is_served_with_defer_on_both_surfaces` |
| `board.js` | `components/issues.rs:142` | **nothing** |
| `search.js` | `components/layout.rs:72` | **nothing** |

`search.js` is the worse of the two on reach — it sits in the app shell,
so it is on every page, and its tag disappearing takes search enhancement
with it everywhere at once.

**Why it belongs to this RFC rather than to a bug fix.** §8 is the Phase
A–D follow-up sweep, and this is precisely what that sweep is for: a
surface that grew across PRs and ended up with a dependency nothing
checks. It is also the second instance this project has found of *a
comment asserting what a test does* — the first was `RFC 003`'s
`global_acknowledged`, where a document and a test agreed with each other
and were both wrong. A comment is the one place with no guard.

**Not proposed: a scan.** A test that walks `static/` and asserts each
filename appears somewhere under `crates/peisear-web/src/` would extend
itself to a fourth file for free, but it would pass on a tag emitted in a
branch that never renders — which is most of what could go wrong here.
Three HTTP-level assertions for three files is complete today. The
residual gap is that a fourth file added later gets no assertion
automatically; that is named here rather than built for.

### 13. The `DEC-007` block omits a crate

*Added 2026-08-25 from `REL-0.26.0`'s review, found by the dev team in
their own gate table.*

`.github/CONTRIBUTING.md`'s `DEC-007` command block runs six of the seven
workspace members. `peisear`, the facade, is absent — not present with the
wrong flags, absent. Its single test is a doctest at
`crates/peisear/src/lib.rs:28`, so `cargo test -p peisear --lib` reports
`0` and only the bare `cargo test -p peisear` finds it.

**No coverage was ever missing.** `cargo test --workspace` runs doctests
and runs three times per release. What was wrong is provenance:
`REL-0.25.0`'s per-target table carried a `1` for that crate, and its own
`cold-gate-tests.log` contains zero `Doc-tests peisear` blocks. The number
was right; no command in the log produced it.

**The `--lib` shape is a trap, not a hole.** Three crates are invoked with
`--lib`, which would skip a doctest in any of them. There are none today —
the workspace contains exactly one doctest, the facade's — so nothing is
uncovered. It becomes a hole the day someone writes a documented example
in `peisear-core`.

**Why a guard and not just a line.** The block is a list of crate names
maintained by hand against a workspace that has grown to seven. That is
the shape that produced this, and it will produce it again at eight.

**What the guard does not catch, decided 2026-08-25 in `QA-004`'s round-2
review.** It asserts every member appears as `-p <name>` in the block. It
does **not** check that the flags on that line are right for the crate:

```bash
cargo test -p peisear --lib   # runs zero tests; the guard passes
```

That is this defect in a second shape — a facade line covering nothing.
Left open deliberately. Closing it means knowing which crates have
doctests, which means parsing fenced code blocks out of every crate's
source: a parser for one line of a contributing guide. And the block is
not the coverage boundary — `cargo test --workspace` runs three times
before every release and includes doctests, so flag drift here costs
developer feedback, not release coverage. The mitigation is prose under
the block saying why that line carries no `--lib`.

### 14. The four structural guards have no CI job

*Added 2026-08-25, found updating the requirements baseline to 0.26.0.*

The baseline's §9.1 has stated since 0.20.0: **a test crate without a CI job
does not exist.** `.github/workflows/test.yml` has a job for each of the twenty
`peisear-web` integration targets and for five of the seven crates. It has no
job running `cargo test -p peisear-web --lib`.

That is where every structural guard this project has built actually lives:

| Guard | Makes unconstructible | In CI |
|---|---|---|
| `prose_scan` | user-visible English authored in Rust (RFC 006) | **No** |
| `static_js_scan` | the same in `static/*.js` (`BOARD-001`) | **No** |
| `test_harness_scan` | §10.13's clock-derived temp paths (`QA-001`) | **No** |
| `dec_007_scan` | the `DEC-007` block drifting from the workspace (`QA-004`) | **No** |

`DEC-007`'s block in `.github/CONTRIBUTING.md` omits the same line, so a
contributor following the documented procedure does not run them either. They
execute only under `cargo test --workspace` — which **is** in the release gate,
three times, so no release has shipped without them. **The exposure is
per-pull-request, not per-release**: a change reintroducing any of those four
defect classes passes CI and is caught at the next release candidate, or by a
reviewer, or not at all.

**§13's limit, first live instance.** `dec_007_scan` asserts each member
appears as `-p <name>` but not that the flags are right for the crate.
`peisear-web` appears twenty times via `--test` lines, so the guard is
satisfied while the crate's library tests go unrun by the block. That was
recorded as a tolerable limit the same day; this is what it costs.

**Why this belongs to RFC 005 and not to a bug fix.** Phase E pays test debt,
and debt in the apparatus that enforces every other rule compounds faster than
debt in any single test. It is also the third entry in this RFC — after §12 and
§13 — where the project's verification reads as more complete than it is.

## Test plan

The Phase E test plan is largely the audit work itself; the
tests already exist in earlier crates and Phase E adds /
activates a few specific ones:

1. **Activate `cross_user_settings_post_returns_403`** when
   a user-scoped POST endpoint exists (or remove with cause).
2. **Add per-endpoint authorization tests** for any surface
   in the audit table that lacks one. Target: every row of
   the §40 audit table has a green test.
3. **Optimistic-lock conflict-rollback tests** in
   `tests/optimistic_lock.rs` for D-1/D-3 (add as those
   substeps land).
4. **Aggregate suppression tests**: in
   `tests/sprint_plan.rs` and `tests/projects.rs`, assert
   that workload chip / capacity hint render with N≥2 and
   suppress at N<2.
5. **Language audit script**: `scripts/audit-language.sh`
   shipped with the repo and run in CI as a non-blocking
   warning. (Blocking comes after the initial audit
   passes.)
6. **Contrast audit script**: same shape — `scripts/audit-
   contrast.sh` runs against documented colour pairs and
   warns on regressions.

## Security & privacy considerations

Phase E *is* the security pass. By construction:

- §11.5: the audit's purpose is to confirm the boundary
  holds across every endpoint. The audit table is the
  artefact; tests are the enforcement.
- §21.4: same — audit confirms every mutation's lock check
  and 409-rollback path.
- Audit log retention policy (deferred from V2.1): make a
  decision in this RFC's resolution. *Default: 30 days for
  audit log, 90 days for issue_events. Configurable via
  env var.*
- Aggregate inferability: section 7 above is the
  systematic answer.

## Out of scope

- Performance tuning. Phase E QA would surface a perf
  regression as a defect; perf *improvement* is its own
  separate work.
- Internationalisation infrastructure (i18n framework).
  Phase E unifies on English; future locale support gets a
  separate RFC.
- Replacing the audit doc format with a more structured
  tool (Excel, doc generator, etc.). Markdown tables are
  the format.

## Open questions

1. **Audit log retention** (raised in Sec & Privacy §). The
   spec leaves this open; this RFC's *Default* sets 30/90
   days. If a stakeholder wants different, raise here.
2. **`?` shortcut modal**: built with vanilla JS or Leptos
   island? *Default: vanilla — too small to justify
   hydration.*
3. **`j/k` collision with user input**: if focus is in a
   text input, `j`/`k` should type the letter, not navigate.
   *Default: implementation must check
   `document.activeElement` and bail when it's an
   editable element. Trivial in vanilla JS; mention in
   the implementation RFC if it gets one.*
4. **Mobile checklist screenshots**: where do they live?
   Repo (committed images) or external? Repo bloats the
   tarball; external introduces link rot. *Default: repo,
   under `docs/src/mobile-screenshots/`. Image size budget
   200 KB total.*

## References

- Spec §40 — Phase E plan
- Spec §11.5.5 — API authorization test suite
- Spec §21.4 — optimistic lock contract (the test surface
  this RFC sweeps over)
- Spec §30-34 — ABDD axes
- RFCs 0001-0004 — the surfaces this audit visits
