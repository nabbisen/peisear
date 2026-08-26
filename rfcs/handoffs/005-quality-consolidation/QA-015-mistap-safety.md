# QA-015 — Two controls whose mis-tap cannot be undone

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: **P0** for §3 — an irreversible action within a mis-tap of its
own escape hatch. P1 for §2.
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §6
**Depends on**: `QA-014`, closed. Its measurement is the input.

**This is not the touch-target pass.** That is a design pass, scheduled for
0.30.0 once a browser harness exists to verify a relayout. This handoff takes
the two pieces that should not wait for it, and the owner approved them on that
basis.

---

## 1. Why these two and not the other 137

The owner's decision on `NFR-A11Y-007` is **uniform 44 px, reached per surface
rather than by inflating every control**. `SPEC §33.2` stands unamended.

Two items are pulled ahead:

1. **Three checkboxes at 16 px** — below WCAG 2.2's **AA** floor (2.5.8,
   24 × 24), which every other control in the product clears. No trade-off to
   weigh.
2. **The confirmation screen's Cancel and Delete** — 32 px, adjacent, one of
   them irreversible.

Everything else waits, because everything else is a density question and these
two are not.

## 2. The checkboxes — delete one word, three times

`notification_preferences.rs:185`, `:190`, `:195`, all
`class="checkbox checkbox-xs"`.

`.checkbox-xs{height:1rem;width:1rem}` — **16 px** — and it appears later in
the pinned stylesheet than `.checkbox` at equal specificity, so it wins.
`QA-014` established this; do not re-derive it.

**Bare `.checkbox` is `1.5rem` — 24 px.** So the fix is removing `checkbox-xs`
from three class attributes. Not `checkbox-sm`, which is `1.25rem` / 20 px and
would still fail; not `checkbox-md`, which is the same 24 px as bare and adds a
class for nothing.

**Confirm the resolved sizes from the pinned CSS yourself** before deleting
anything. If bare `.checkbox` is not 1.5rem, stop.

This reaches the **AA** floor, not 44 px. Say so plainly in the package — these
three are still short of `SPEC §33.2` and are part of 0.30.0's pass like
everything else. What changes today is that they stop being the only controls
in the product below the AA minimum.

## 3. The confirmation screen — the item this handoff exists for

`components/confirmation.rs:53` and `:58`:

```rust
<div class="card-actions justify-end mt-4">
    <a href=cancel_href class="btn btn-ghost btn-sm">      // 32 px
    <button type="submit" class="btn btn-error btn-sm">    // 32 px
```

**RFC 010 built this screen because the plain path was more dangerous than the
enhanced one.** Then it put an irreversible action within a mis-tap of its own
escape hatch. That is the same defect in a dimension nobody measured, on the
one screen in the product whose entire purpose is to prevent an accident.

### 3.1 The fix, and why it is provably a fix

Add `min-h-11 min-w-11` to both. The project already has this exact pattern at
`issues.rs:661` from `DEV-002` — one precedent, one shape.

**`min-height` beats `height`; this is not a cascade gamble.** `btn-sm` sets
both `height:2rem` and `min-height:2rem`; `min-h-11` is `min-height:2.75rem`.
Per CSS 2.1 §10.7 the used height is clamped by `min-height` after `height` is
resolved, so 44 px wins regardless of source order or specificity between the
two `min-height` declarations — the larger minimum governs. Do not "verify" this
by reading rule order; it follows from the property.

### 3.2 Two things to report, not to fix

- **The gap between them.** They sit in `card-actions` with DaisyUI's own gap.
  Report the resolved value. Two 44 px targets touching is better than two
  32 px targets touching, and still not the same as two separated ones.
- **Delete is the rightmost control**, under `justify-end`. On a phone the
  rightmost position is the most thumb-reachable, which is the wrong place for
  the irreversible half of a pair. Say whether you agree; **do not reorder
  it** — button order is a convention question across the whole product, not a
  one-screen fix.

## 4. Not in scope, with the reason

**The four delete *triggers* stay as they are** — `sprints.rs:499`, `:520`,
`issues.rs:1630`, `projects.rs:234`. Every one is an `<a href>` that
**navigates to the confirmation screen**. Mis-tapping one costs a page load and
a Cancel. Mis-tapping the control in §3 destroys data.

That is the line: **navigate versus destroy**, not "delete-ish versus not".

No other control resized. No layout changed. The notification-preferences table
keeps its shape — that table is 0.30.0's problem and `QA-014` already described
why it needs a different layout rather than taller cells.

## 5. Guards

**Assert both**, since neither can silently regress afterwards:

- The confirmation screen's Cancel and Delete both carry `min-h-11` and
  `min-w-11` — in `confirmation`, planted **one class at a time**, four plants.
- `checkbox-xs` appears nowhere under `crates/peisear-web/src/`. A one-needle
  scan; fold it into `contrast_scan`'s file or add a sibling, and say which and
  why. Pin the resolved sizes and the DaisyUI version in the doc comment, as
  `contrast_scan` does.

**Do not extend the scan to the other sub-44 px classes.** They are still in
use by design until 0.30.0, and a guard that fails on the current tree would be
weakened until it passed — which is worse than no guard.

## 6. Escalate rather than deciding

- If bare `.checkbox` is not 24 px in the pinned CSS, stop.
- If adding `min-h-11 min-w-11` visibly breaks the card's layout in a way you
  can see from the markup — a fixed-height ancestor, an overflow constraint —
  stop and report rather than working around it.
- **If any *other* control in the product is below 24 px**, that is a second
  instance of §2's class and I want it before 0.30.0, not inside it. `QA-014`
  found only the checkboxes; say whether you agree after touching this code.

## 7. Acceptance

1. Resolved sizes confirmed from the pinned CSS before any deletion.
2. Three `checkbox-xs` removed; stated plainly that this reaches AA, not 44 px.
3. Both confirmation buttons carry `min-h-11 min-w-11`.
4. Both guards present, running in CI; four one-at-a-time plants demonstrated.
5. §3.2's two observations reported, neither acted on.
6. No other control resized; §4's four triggers untouched.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. §3.2 as prose. Each plant transcript separately.
