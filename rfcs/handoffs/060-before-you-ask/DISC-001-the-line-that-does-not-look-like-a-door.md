# DISC-001 — the line that does not look like a door

**Series**: `060`, a repair of this series' own work, inside `0.62.0`'s
authorized scope. **Blocking**: the `0.62.0` cut, with `FOCUS-001`.
**Register**: `§10.58`.

## 1. What is wrong, measured

The one line this release leaves in place of the health block renders
**no disclosure marker**. Not hidden by a rule — never created.
`grow_text(base)` is `{base} min-h-11 min-w-11 inline-flex items-center`,
and a `::marker` box only exists on a `display: list-item` element.

Computed style, project screen, 1280:

| summary | `display` | marker |
|---|---|---|
| *"In-flight work is currently carried by one person."* | `inline-flex` | **none** |
| *"Workload across the team."* | `inline-flex` | **none** |
| `Indicators` | `inline-flex` | **none** |
| `How this is calculated` × 6 | `list-item` | rendered |

**Two conventions on one screen**, and the three without a marker are
the ones now hiding the most. A screen reader still announces a
collapsed disclosure and Tab still reaches it; a touch reader, and a
sighted reader who does not hover, sees a sentence.

## 2. The decision this implements — the architect's, not this round's

**A disclosure control must look like one, and it keeps its 44 × 44
target.** Both, not one. The marker returns to the three section-level
summaries; `grow_text`'s touch-target guarantee is not weakened for any
control that has it today.

**Not in scope**: changing the sentence, the precedence chain, what is
collapsed, or `058`'s rule that the control's word *is* the sentence.
A marker **beside** the word is what `058` argued for; a glyph
**instead of** it is what it argued against.

## 3. What to build

Make the three summaries list items again **without losing the target**.
The obvious shapes, in the order this architect would try them:

1. **Keep `display: list-item` and reach 44 px with padding or
   line-height** rather than `inline-flex`. Simplest, and it keeps the
   marker the browser already knows how to draw.
2. **A `grow_summary()` helper beside `grow_text()`** that carries the
   same `min-h-11 min-w-11` with a list-item display, so the next
   `<summary>` cannot inherit the defect by using the wrong helper.
3. Only if neither holds: an explicit marker in markup, and say why the
   first two failed.

**Report which you used and what the other two cost** — this is a
design decision with a measurement behind it, not a styling preference.

## 4. What to measure

Computed `display` and `::marker` for **every** `<summary>` on the
project board, the project list and `/today`, at **390 and 1280**, and
the rendered box of each control against **44 × 44**. Both properties in
one table: a marker that costs the touch target is not the fix, and a
target that costs the marker is the defect.

## 5. Acceptance

- Every disclosure control in the product renders a marker, **measured
  in a browser**, not inferred from a class list.
- `touch_target_scan`'s three assertions stay green, and each changed
  control still measures ≥ 44 × 44 at both widths.
- **A guard that would have caught this**, and it is the point of the
  round: assert that a `<summary>` is never rendered with a computed
  display that suppresses its marker. A source scan can check which
  helper a summary was given; a browser check can read the computed
  value. Pick the one that cannot pass while the defect is present, say
  why, and **plant the defect to show it fails by name**.
- `DEC-007` from → to, re-derived with the runner.
- The six documentation screenshots **regenerated afterwards** — they
  are committed as of `4f03045` and show the markerless state.

## 6. What this round must not do

- Do not change any summary's text, or what any disclosure contains.
- Do not open anything by default — `FR-PER-007`, `FR-NAV-006` and
  `external-design §5.10` all still hold.
- Do not weaken `grow_text` for controls that are not `<summary>`.
- Do not amend `requirements.md`; `§10.58` already records this.
