# RFC 0005: Quality consolidation

**Status**: Proposed
**Target**: 0.26.0 (Phase E) — §9 shipped at 0.22.0, §10 pulled forward to 0.25.0
**Related spec sections**: §40 (Phase E plan), §11.5.5 (API
authorization QA), §21.4 (optimistic-lock conflict),
§30-34 (ABDD axes)
**Last updated**: 2026-08-16

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
| (Phase D mutations — fill in as they ship) | | | |

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
| (Phase D rows added as they ship) | | | |

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

#### 10.2 Project delete reports success to a non-owner

`handlers::projects::delete` calls
`projects::delete(&state.db, &project_id, &user.id)` with no ownership check of
its own, relying on the storage layer's `WHERE owner_id = ?2`. For a team member
who is not the owner that deletes zero rows, returns `Ok`, and redirects with
the *"project deleted"* flash.

**The project survives and the user is told it did not.** Not an access defect —
nothing is deleted — but a false report about a destructive action, which this
project's own standards on honest reporting do not permit.

`CONF-001` made it visible: its new `GET` is stricter than the `POST` it fronts,
which is the right direction and an odd state to leave.

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
