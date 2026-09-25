# GATE-002 — make the fixture render what the pages can render

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none — closing known coverage gaps in `BROWSER-001`.
**Related register entry**: **`§10.32`**, opened by the three instances below.
**Depends on**: `LAYOUT-011`, landed.

**Expect to find defects.** Two of the three previous fixture changes turned
the gate red and each produced a handoff. **If that happens, escalate as
`GATE-001` and `LAYOUT-010` did** — report the surface, hold the fixture, do
not fix a page and a fixture in one change. `DEC-048` condition 3 still applies.

---

## 1. The three branches, and only these three

**1.1 — The issue-detail assignee badge.** `LAYOUT-011` fixed `:1820` and
unit-tested it; **the gate does not hold it**, because its issue-detail page is
the older unassigned issue and that handoff forbade changing it.

**Ruled: add a second issue-detail page rather than assign the existing one.**
Changing the existing issue alters a page the gate has swept for eleven
releases, and I would rather the fixture grow than a baseline move. Expect
**95 → 100** cells; say so.

**1.2 — Calendar blocks.** No fixture issue has planned dates, so all three
calendar views sweep empty. Plan the **long-titled** issue through the edit
route, as you probed. The project axis you already measured clean; **the
personal axis is unprobed** because it lists assigned issues only — the
assigned issue `LAYOUT-011` added is the one to plan, so both axes render.

**1.3 — A second assigned account on the board**, so the card badge and the
workload strip are swept with more than one name. `LAYOUT-011`'s measurement
found the team board's multi-card shape was where the 2 px y-offset showed;
the gate's personal board has one card and could not see it.

**Not in scope**: the team project's board, list and calendar as gate pages
(that is 95 → 105 and a separate question), any page not already swept, and
auditing the other sixteen pages for branches. **`§10.32` says why**: doing it
exhaustively is worth less than it costs, and this handoff closes what is
known, not what is possible.

## 2. How

- **Through the routes**, as `GATE-001` established.
- **Assert, do not hope.** `GATE-001`'s fixture throws when a title is absent;
  do the same for each branch here — the badge, a calendar block on each axis,
  the second card. **A green cell on a page that rendered nothing is the whole
  subject of `§10.32`**, and an assertion is the only thing that distinguishes
  them from outside.
- **Existing pages keep their content** wherever adding is possible instead of
  changing. Where it is not, say which page's baseline moved and by how much.
- `browser-checks/README.md`: what the fixture holds, why, and — per
  `§10.32` — **that this is not exhaustive**.

## 3. Verification

- **The gate, three runs.** State the cell count and why it is that number.
- **Each branch renders**, named, with the element found.
- **If red**: the surface, the element, the measurement, and stop (§0).
- **Run time.** The fixture is growing; report the gate's wall-clock before and
  after. If it has grown materially, say so — that is `§10.32`'s cost side and
  nobody is watching it.
- `DEC-007` unchanged at **352** unless a Rust test is touched.

## 4. Escalate rather than deciding

- **Red cells** (§0).
- **If a branch cannot be reached through the routes** — say which and why
  rather than writing SQL.
- **If the run time grows materially.**
- **If you find a fifth unrendered branch** while doing this. Report it; do not
  add it. `§10.32` exists because the list is open-ended.

## 5. Exit condition

The gate sweeps an assignee badge on issue detail, calendar blocks on both
axes, and a board with two assigned cards; each is asserted present by the
fixture rather than assumed; the cell count and the run time are both stated
with their reasons.
