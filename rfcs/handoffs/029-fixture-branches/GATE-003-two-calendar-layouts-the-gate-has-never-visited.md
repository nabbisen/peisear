# GATE-003 — two calendar layouts the gate has never visited

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none. **Related**: `§10.32`.
**Source**: found by the dev team in `GATE-002` §4 — and it corrects my §1.2,
which said the gate sweeps all three calendar views. **It sweeps one.**
**Depends on**: `GATE-002`, landed — its planned issue is what makes these
views render anything.

---

## 1. Why this is not just five more cells

`?view=day` and `?view=month` are distinct `CalendarView` variants rendering
**distinct layouts**, and the gate visits neither: its two calendar URLs are
the defaults, both week.

**That is an unvisited layout, not an unrendered branch** — and the day view is
where `CAL-003`'s `h-full` silently collapsed a block from 59.875 px to 20 px.
That was found by a before-and-after rendering check *because the gate could
not see it*. **It still cannot.** The day view also carries the time axis and
the calendar drag; the month view is a grid of cells each holding user text,
which is the shape every `§10.25` defect has had.

## 2. What to do

**Four URLs**: `/today/calendar?view=day`, `?view=month`, and the same two on
`/projects/{id}/calendar`. **100 → 120 cells**; state it.

- **Assert each renders a block**, as `GATE-002` did — `GATE-002`'s planned
  issue is visible on both axes, so all four should show it. **A month cell
  that renders nothing is exactly the green-on-empty case `§10.32` is about.**
- **Use the anchor that shows the planned issue.** If a view defaults to a
  period that excludes it, say so and pick the anchor deliberately rather than
  hoping today's date cooperates — a fixture that passes in September and fails
  in October is worse than no coverage.
- **Report the run time** again. `GATE-002` measured +3% for five cells and
  found the machine's noise larger; twenty cells is the first change that might
  show. If it does, that is `§10.32`'s cost side getting its second number.

**Expect red.** Two of the four fixture changes this release turned the gate
red. **Escalate as before** — report the surface and hold, do not fix a page
and a fixture together (`DEC-048` condition 3).

## 3. Verification

- The gate, three runs, cell count stated with its reason.
- Each of the four views asserted to contain the planned issue.
- If red: the page, the width, the element, the measurement — and stop.
- `browser-checks/README.md`: the four views listed, and **the *known and not
  covered* list shortened accordingly** — it currently names these.
- `DEC-007` unchanged at **352** unless Rust is touched.

## 4. Escalate rather than deciding

- **Red cells.**
- **If a view cannot be anchored** so the block is reliably visible.
- **If the run time grows materially** — that is a decision about the gate's
  size, and mine.
- **If you find a sixth unrendered branch or unvisited layout.** Report only.
  `§10.32` is open precisely because this keeps happening.

## 5. Exit condition

The gate visits all three calendar views on both axes, each asserted to render
a block, with the cell count and run time stated.
