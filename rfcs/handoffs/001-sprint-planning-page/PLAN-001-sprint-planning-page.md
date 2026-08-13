# PLAN-001 — The sprint planning page

**Issued by**: Architect
**Date**: 2026-08-13
**Priority**: P1 — the feature work of 0.22.0
**Governing RFC**: [001](../../accepted/001-sprint-planning-page.md)
**Depends on**: TEAM-001 (landed). RFC 001's assignee filter was meaningless
before it.

---

## 1. Read this first: the RFC is fifteen weeks old

RFC 001 was written 2026-05-04, before the i18n architecture, before the
compliance pass, before RFC 009, and before `viewer` roles were enforced
anywhere. Most of it holds. **Five things in it are now wrong**, and they are
listed in §2 rather than left for you to trip over.

Where this handoff and RFC 001 disagree, **this handoff wins** and the RFC has
been amended to match. If you find a sixth disagreement, that is a finding —
report it rather than picking one, the way TEAM-001 did with §D1's sample SQL.

## 2. Five corrections to RFC 001

### 2.1 Non-members get **404**, not 403

RFC 001's test 6 is `non_team_member_gets_403`. Wrong. `resolve_team_membership`
(`handlers/sprints.rs:32`) returns **404**, deliberately: external design §9
uses 404 for team membership so a non-member cannot confirm a team exists. 403
is for personal data, where the URL already implies existence.

Use the existing helper. Do not write a second one.

### 2.2 `viewer` may read the page and may not move anything

RFC 001 says the page is "accessible to any team member (admin or otherwise)"
and is "not admin-only". Both still true, and both now under-specified:
`TeamRole::Viewer` exists and is read-only on team projects
(`0011_teams.sql:71`).

- **GET** — any member, including `viewer`. A viewer sees the plan.
- **POST** `/add` and `/remove` — `TeamRole::can_write()` only, i.e.
  `Admin | Member`. A viewer gets **403** here, not 404: they are a member, so
  the team's existence is not what is being concealed. This mirrors
  `handlers/sprints.rs:93`'s `can_manage_team()` pattern exactly.

`can_write()` is the same predicate `CANDIDATE_SET_CTE` filters on. Use it;
do not re-derive the role set.

A viewer's page renders without move buttons — the same read-only shape RFC 001
already specifies for completed sprints, so this is a second reason to reach
that branch, not a second branch.

### 2.3 The capacity hint is deferred — it is an `NFR-PRIV-007` problem

**Do not implement `team_capacity_hint`. Do not render a capacity hint.**

RFC 001 §Security claims *"The capacity hint is a sum-only number — no
per-person breakdown — so a member can't infer another's capacity from the
hint."* That is false, and the RFC's own formula is why: the hint sums
`effective_for_user(today)` across members **who have at least one open issue
in the team's projects**. A viewer of that page can see who those members are —
assignees are on the rows.

- One participating member: the hint **is** that person's capacity.
- Two: subtract your own, get theirs exactly.

`NFR-PRIV-001` makes capacity self-only and is P0. `NFR-PRIV-007` — aggregates
must not be reversible to individuals — is recorded in the baseline as
unimplemented, with the note *"No genuine aggregate currently exists that could
resolve to one person; when one is built —"*. **This would be the first one**,
in a product designed for teams of about five.

That needs an owner decision and a design that survives a two-person team, and
neither should be made under a release's schedule pressure. Everything else in
RFC 001 is unblocked, so the hint leaves this handoff and gets its own.

**Ship the committed total** (`committed: N pts`). That is a sum of effort on
issues everyone on the page can already see, not an aggregate of private data.

If a stakeholder asks where the hint went while you are working: it is
deferred, not dropped, and the reason is in this section.

### 2.4 All copy goes through `peisear-i18n`

RFC 001's HTML sketch hardcodes `"Backlog"`, `"Sprint Items"`,
`"committed: 24 pts"`. It predates RFC 006. Every user-visible string is a
`MessageKey`, `prose_scan` will fail on literals in `components/` and
`handlers/`, and **§D6 rule 7 applies**: do not assemble a sentence with
`format!` or select wording in a `match` returning `String` — one key, one
`en.rs` arm.

The `aria-label` triple RFC 001 specifies per row (title, points, column) is a
composed sentence and therefore one key taking typed parameters, following
`IndicatorAriaLabel`.

### 2.5 The assignee filter now filters something

RFC 001 §Must-have 3 lists an assignee filter. Until TEAM-001 shipped last
week, a team project's only possible assignee was its owner, so the control
would have had one option. Build it as specified; it now does work.

Candidate list comes from the same place the issue form's does. Do not write a
third query — RFC 009 §D1's whole point is that this set has one definition.

## 3. What stands unchanged

Requirements 1–8 otherwise, both routes, the 303-back-to-GET-with-filters
behaviour, top-level-only in both columns, read-only for active and completed
sprints, the `<li>`-sibling structure that RFC 004 will later attach drag and
drop to, and all three open-question defaults (team-scoped projects only;
`assigned_at` ascending).

**The optimistic-lock opt-out stands and is correct.** `sprint_issues`
(`0012_sprints.sql:101`) has `issue_id`, `sprint_id`, `assigned_at` — no
`updated_at`, so `DEC-013`'s trigger machinery does not reach it and there is
nothing to compare. Two planners moving the same issue converge on the same
row. RFC 001 asks for a handler comment saying so; write it, and say *why*
(no version column on a join row whose identity is its content), so the next
reviewer does not add a lock reflexively.

**The per-assignee rollup stays a nice-to-have, and is permitted.** It shows
each member's volume of committed work, which is `NFR-PRIV-002`'s "workload
distribution" — and `ISSUE-003` ruled that holds regardless of how many members
a surface lists. It carries no capacity value, so §2.3 does not touch it. Defer
it if time is tight, as RFC 001 says.

## 4. Tests

New target `crates/peisear-web/tests/sprint_plan.rs`, plus a CI job
`test-peisear-web-sprint-plan` and a line in `CONTRIBUTING.md`'s list —
`DEC-007`: a test crate without a CI job does not exist.

RFC 001's seven, with test 6 corrected and two added:

| # | Test |
|---|---|
| 1 | Planned sprint renders both columns, both headings, a form per movable row |
| 2 | `POST /plan/add` → 303; follow-up GET shows the issue under Sprint Items |
| 3 | `POST /plan/remove` → symmetric |
| 4 | Sub-issues appear in neither column |
| 5 | Completed sprint: 200, no `<form>` in either column |
| 6 | **Non-member gets 404** (§2.1) |
| 7 | Committed total is the sum of effort — two issues at 5 and 8 render 13 |
| 8 | **`viewer` GETs 200 with no move forms; `viewer` POSTing `/add` gets 403** (§2.2) |
| 9 | **Filter round-trip**: `?project=&priority=&assignee=` narrows the backlog and survives a move (the 303 preserves the query) |

Test 9 exists because the filter-preserving redirect is the kind of thing that
works when written and silently stops working later, and it is the difference
between a planner staying in context and losing their place on every move.

## 5. Escalate rather than deciding

- If the backlog query needs to reach personal projects to be useful, stop.
  Open question 1's default is team-scoped only, and widening it changes the
  auth boundary.
- If you find a surface where the committed total could be back-computed into
  someone's capacity, report it — §2.3 removed the obvious path, not
  necessarily every path.
- If `resolve_team_membership` does not fit the two POST routes, say so rather
  than copying it.

## 6. Acceptance

1. All nine §4 tests pass; the CI job and `CONTRIBUTING.md` line exist.
2. No capacity hint anywhere in the page, the storage layer, or the tests.
3. Every user-visible string in `peisear-i18n`; `prose_scan` passes.
4. `can_write()` gates both POSTs; `resolve_team_membership` gates all three
   routes.
5. The join-table lock opt-out is commented with its reason.
6. fmt and clippy exit 0; the `DEC-007` gate set green; `test_harness_scan`
   passes — the new test target must not derive a temp path from the clock,
   and `TestApp::spawn` already gives you the right thing.
7. Filter state survives both POSTs.

## 7. Prohibited

No capacity hint. No drag and drop, no multi-select, no in-backlog search
(RFC 001 §Explicitly out). No sprint state transitions from this page. No
change to `resolve_team_membership` or to any authorisation helper. No
optimistic-lock column added to `sprint_issues` — that is a schema change and a
different decision.

## 8. Required review-request format

Workflow §9.2. State which of RFC 001's open-question defaults you took, and
report anything that made you want to disagree with §2.
