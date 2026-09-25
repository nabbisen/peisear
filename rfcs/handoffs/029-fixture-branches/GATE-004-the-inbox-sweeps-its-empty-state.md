# GATE-004 — the inbox sweeps its empty state

> **WITHDRAWN 2026-09-25, not implemented.** The premise is wrong twice. **No
> route produces a notification** — the only emitters are the snapshot job's two
> burnout detectors, which fire on an edge between snapshots across a 14-day
> threshold, so *time* is missing rather than a second actor. And **the rows do
> not carry issue or project text**: both emitted kinds are fixed i18n copy with
> a count, `payload_json` is `None`. So no user-text run can be in an inbox row,
> and the `§10.25` shape this handoff invoked is not what the page holds.
>
> `§10.32`'s stopping rule applies on its first test: recorded, not chased.
> Kept for the reasoning and for the finding that produced it.

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none. **Related**: `§10.32`.
**Source**: `GATE-003` §4. **Depends on**: `GATE-003`, landed.

**This is the last branch added reactively.** `§10.32` gets a closing note
saying so: after this, an unrendered branch is recorded and not chased unless a
defect appears behind it. Four fixture changes in one release, each finding the
next, is the pattern that entry warns about.

---

## 1. The gap

`/inbox` is swept, and the fixture produces **no notifications**, so it renders
its empty state at all five widths. **Notification rows carry issue and project
text** — user text in a row with a timestamp and actions, which is the shape
every `§10.25` defect has had.

## 2. What to do

**Make the fixture generate at least two notifications**, at least one unread,
through whatever ordinary action produces them — do not insert rows. **One must
concern the long-titled issue**, so the run is in the row.

- **Assert the rows render**, as the last three handoffs did: the fixture
  throws unless the inbox contains the title. A green cell on an empty inbox is
  the thing being fixed.
- **Two notifications, not one**: `FR-NTF-005`'s *mark all read* control is
  present only when something is unread, and the row list's layout with more
  than one row is what the week's defects have been about.
- **Expect the cell count to stay 120** — content on a swept page, no page
  added. If it moves, say why.
- **If no route produces a notification with the fixture's single account**,
  say so rather than reaching for SQL: notifications may need a second actor,
  and that is a finding about the fixture's shape, not a reason to fake one.

**Expect red.** Three of the five fixture changes this release turned the gate
red. Escalate as before; `DEC-048` condition 3 still holds.

## 3. Verification

- The gate, three runs, cell count and **run time** stated — `§10.32`'s third
  number, and the first where content was added to an existing page rather than
  pages added.
- The rows asserted present, with the element found.
- `browser-checks/README.md`: the inbox off the *known and not covered* list,
  and **the closing note from §0** recorded there as well as in `§10.32`.
- `DEC-007` unchanged at **352** unless Rust is touched.

## 4. Escalate rather than deciding

- **Red cells.**
- **If notifications need a second account** (§2) — report the shape; the
  decision about a second actor in the fixture is mine.
- **If the run time jumps** rather than creeping.
- **A seventh branch**: record it in the README only. Do not bring it to me as
  work.

## 5. Exit condition

The gate sweeps an inbox with at least two notification rows, one unread and
one naming the long-titled issue; the cell count and run time are stated; the
README carries the closing note.
