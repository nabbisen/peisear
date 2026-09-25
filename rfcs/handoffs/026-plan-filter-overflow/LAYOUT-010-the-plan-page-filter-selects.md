# LAYOUT-010 — the sprint plan's filter selects take their longest option's width

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none — defect fix.
**Source**: found by `GATE-001`'s fixture; escalated red rather than pushed
(`DEC-048` condition 3). Ruled in `.git-exclude/reviewed/GATE-001-review.md`.
**Depends on**: nothing. **`GATE-001`'s patch lands in the same round as this
fix**, so `main` never carries a red gate — the way `LAYOUT-006` was held for
`BROWSER-002`.

---

## 1. The defect

`components/sprint_plan.rs:192` and its assignee twin:

```rust
<select name="project" class=grow("select select-sm select-bordered")>
```

in a `flex flex-wrap` row, with **no `min-w-0`**. A `<select>` sizes to its
longest `<option>`, and an option is unbreakable text, so the control's
content-based minimum wins over the flex container. Measured at 320 px with
`GATE-001`'s fixture: **project select 729 px, assignee select 626 px, page
`scrollWidth` 745 against a 320 `clientWidth`** — 425 px of overflow, and 355
at 390 px.

**This is `§10.25`'s shape A** — `LAYOUT-001`, `LAYOUT-003` and `LAYOUT-006`'s
mechanism — on an **input element** rather than a text node. The remedy that
worked there is the starting point, not necessarily the finish: a `<select>`'s
intrinsic width comes from its options, so confirm that `min-w-0` alone
actually constrains it before assuming it does.

**Why the gate never caught it**: both option lists are filled from the team's
projects, and the fixture's team had none, so the selects held only *All
projects* and *Anyone*. **The gate has swept that page since `BROWSER-001`
shipped and could not have failed on it.** An empty fixture made a covered
surface look covered.

## 2. What to do

- **Both selects, and look for a third.** The priority select is bounded by its
  own fixed copy today; if it is the same shape it gets the same treatment,
  because a translated locale could lengthen it. Check the other filter bars in
  the product for the same construction and **report what you find** — do not
  fix them here.
- **The remedy is measured, not assumed.** `min-w-0` on the flex item is the
  shape-A fix; a `<select>` may also need a width constraint or truncation for
  its own intrinsic sizing. **Say which combination you used and what each one
  contributed** — `LAYOUT-008` found a third shape by not assuming the first
  two applied.
- **The selected option must stay readable.** A control narrowed until nobody
  can tell which project is filtered has traded an overflow for a worse
  problem. State what the narrowest width shows.
- **Do not change what the filter does**, its parameters, or its round trip —
  `filter_round_trip_narrows_backlog_and_survives_move` passes unmodified.

## 3. Verification

- **The gate, with `GATE-001`'s patch applied: 95 cells, 0 failing.** That is
  the exit condition for both pieces of work and the reason they land together.
- **Report the box** of each select at 320, 360, 390, 414 — as the `LAYOUT-*`
  handoffs did — and what the selected option renders as at the narrowest.
- **768 and 1280 unchanged**: the filter bar is pixel-identical with ordinary
  names before and after.
- **The touch-target floor holds** — `select-sm` is already at the boundary and
  `touch_target_scan` must stay green.
- `DEC-007` unchanged at **346** unless you add a test; a rendered assertion on
  the select's class is worth one if it is cheap.
- `fmt`, `clippy`.

## 4. Escalate rather than deciding

- **If `min-w-0` does not constrain a `<select>`** on its own. That is a fourth
  shape for `§10.25` and it is worth recording as one.
- **If the same construction exists on filter bars elsewhere** (§2) — report,
  do not fix.
- **If narrowing makes the selected option unreadable** at 320 and you cannot
  see a remedy that keeps both.
- **If the gate finds a second failing surface** once the fixture lands.

## 5. Exit condition

The gate is 95 cells and 0 failing **with `GATE-001`'s fixture applied**; both
selects fit at 320 px with the selected option still legible; the filter's
behaviour and the desktop rendering are unchanged.
