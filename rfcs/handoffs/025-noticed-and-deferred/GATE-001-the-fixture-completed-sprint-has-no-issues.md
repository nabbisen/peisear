# GATE-001 — the gate's completed sprint has no issues in it

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none — a coverage gap in `BROWSER-001`.
**Source**: stated as a limitation by the dev team in `SPRINT-005` §4 and
recorded in its review.
**Depends on**: nothing. **After `REQ-001`.**

---

## 1. The gap

`SPRINT-005` added a completed sprint's detail page to the overflow gate's
fixture, taking it to nineteen pages and 95 cells, so the gate reaches the
*Reopen sprint* control and the *Summary at completion* / *Issues in this
sprint now* headings.

**That sprint has no members.** The fixture's project is personal, and only a
team project's issues can join a sprint — an attempt was refused `400`. So the
page is swept with **an empty issue list**.

**A completed sprint's issue list is exactly where a long issue title would
overflow**, and it is the one thing on that page the gate cannot currently see.
`§10.25`'s three shapes were all found on lists of user text.

## 2. What to do

**Give the fixture a team project with a completed sprint holding issues**, at
least one carrying the unbreakable-run title the fixture already uses for this
purpose, and at least one long enough to wrap. The existing personal project
stays as it is.

- **Build it through the routes**, as the rest of the fixture is built — create
  the team, the project, the issues, the sprint, add the issues, start,
  complete. A fixture written straight into SQL would not prove the flow it
  depends on still works.
- **The captured record must be real** — completed through the route, so
  `sprint_records` and `sprint_burndown_points` are populated by `complete`
  rather than by hand.
- **Two contributors**, so the burndown renders. `NFR-PRIV-007` suppresses the
  trajectory below two, and a one-contributor sprint would leave the chart out
  of the sweep — the trap `SPRINT-005` §3 already fell into once and caught.

## 3. Verification

- **The page the gate now sweeps actually contains the list**: report the issue
  titles present on it, not only that the page returns 200.
- **The cell count.** Adding issues to an existing page should leave it at
  **95**; adding a page would make it 100. **Say which happened and why** —
  `SPRINT-005` was asked the same and answered it, and the answer is the point.
- **The gate passes**: 0 failing, 0 non-local requests, exit 0. **If it now
  fails, that is the gap doing its job** — report the overflow, do not fix the
  page and the fixture in one change.
- `browser-checks/README.md` updated: what the fixture holds and why the team
  project exists.
- `DEC-007` unchanged at **346** unless a Rust test is touched.

## 4. Escalate rather than deciding

- **If the gate fails on the new content.** Report the surface and the
  overflow; a layout defect found this way gets its own handoff, as the nine
  `LAYOUT-*` ones did.
- **If a team project makes some other fixture page render differently** — the
  fixture is shared, and a new team is visible on more than one screen.
- **If the cell count moves for a reason you did not intend.**

## 5. Exit condition

The gate sweeps a completed sprint's page with a populated issue list
including user text that cannot break, at five widths, and
`browser-checks/README.md` says what the fixture holds.
