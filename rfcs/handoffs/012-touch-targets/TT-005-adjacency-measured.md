# TT-005 — measure the two `<summary>` overlaps, and settle `§10.19`

**Target release**: 0.34.0. **Governing RFC**: RFC 012 (`done/`), `DEC-049`,
`NFR-A11Y-007`'s adjacency clause. **Source**: `§10.19`, open since
2026-09-06 with the sentence nobody has acted on.

## 1. The question, and it is a question

`§10.19` records three overlapping pairs on project detail and the board. One
is settled: `join`-grouped buttons share a 1 px column because DaisyUI's
`join` collapses adjacent borders with a negative margin, and a 1 px seam
between visually contiguous segments harms nobody.

**Two are not settled**, and the register says so in as many words: *"The
`<summary>` pairs are larger and less obviously benign; they are worth a look
before anyone asserts they are harmless."*

| pair | overlap | where |
|---|---|---|
| `<summary>` ↔ `<a class="block">` | **34.7 × 16 px** | project detail, board |
| `<summary>` ↔ `button.btn-xs` | **20.2 × 5 px** | project detail, board |

**This handoff does not assume a defect and does not assume its absence.** It
asks for one measurement, reported either way.

## 2. What the mechanism probably is — verify, do not inherit

The `<summary>` is the health strip's "Indicators" disclosure toggle
(`issues.rs:251`). It is `grow()`'d, so `min-h-11` gives it a 44 px box around
about 16 px of visible text. The board cards below it carry the card link
(`issues.rs:879`, `grow("block")`) and the move-status buttons
(`issues.rs:849`, `grow("btn btn-ghost btn-xs px-2")`), both also grown.

So the likely reading is that **growing controls to 44 px is what produced
these overlaps** — the expanded hit area `NFR-A11Y-007` explicitly permits,
reaching into a neighbour that was itself expanded. If that is right, it is
worth saying plainly: the touch-target work created an adjacency question it
could not see, and `DEC-049` clause 4's gap argument never covered this shape
because these are not gap-separated siblings.

**Confirm or refute that by measurement.** It is a hypothesis in a handoff,
which is the weakest kind of evidence this project accepts.

## 3. What to measure

Use `browser-checks/cdp.mjs`. **`getBoundingClientRect` alone is not enough** —
it ignores ancestor clipping, which produced two of my own artefacts this
month. `elementFromPoint` is the instrument, and it answers the question that
matters.

For each of the two pairs, at **320 / 390 / 768 / 1280**:

- **The overlap rectangle**: its size, and both controls' boxes.
- **Who receives a tap in the overlap region.** Sample a grid of points across
  the overlap rectangle and record which element `elementFromPoint` returns at
  each. This is the whole question: if the region belongs to the card link,
  then part of the toggle's 44 px target is unreachable and a user aiming at
  the toggle activates the card; if it belongs to the toggle, part of the card
  is shadowed.
- **Whether either control is nested inside the other.** `§10.19` says neither
  pair is nested; confirm it rather than quote it.
- **What a real tap does**, not only what `elementFromPoint` predicts:
  dispatch a touch at the centre of the overlap region with
  `Input.dispatchTouchEvent` and record what happened — did the `<details>`
  toggle, did the page navigate, or neither. `NFR-A11Y-006`'s verification
  used exactly this instrument, and a prediction and a dispatched tap
  disagreeing would itself be the finding.
- **The `<details>` open state matters.** Measure with it closed *and* open. A
  closed `<details>` reports geometry for content it does not display, which
  is `§10.21`'s withdrawn entry and my own artefact; `checkVisibility()` is
  the guard against repeating it.

## 4. What to do with the answer

**Report it. Do not fix anything in this handoff.**

- If the overlap region reliably belongs to the control a user is aiming at,
  and a dispatched tap agrees, then say so with the numbers and `§10.19`
  closes as harmless-and-measured.
- If a tap in the region reaches the wrong control, that is a defect, and its
  remedy is a design decision about how a grown disclosure toggle sits above
  a grown card — mine, not yours. Report the size and which way it goes.

**Either outcome is a success for this handoff.** A measurement that finds
nothing is worth as much as one that finds something, and saying otherwise is
how a check gets pointed at the answer somebody wanted.

## 5. Escalate rather than deciding

- **If the pairs do not reproduce at all.** The measurement behind `§10.19` is
  mine, from 2026-09-06, and three of my measurements this month turned out to
  be artefacts of my own instrument. If these overlaps are a fourth, that is
  the finding and I would rather hear it than have it smoothed over.
- **If a third pair appears** that `§10.19` does not name.
- **If the answer differs between widths**, rather than being one answer.

## 6. Exit condition

Both pairs measured at four widths, closed and open, with the overlap
rectangle, the `elementFromPoint` grid, and a dispatched-tap result for each;
nesting confirmed; a stated conclusion for each pair. **No source change, no
new test, `DEC-007` stays at 256.**

---

**Who holds what**: dev team — the measurement. Architect — what `§10.19`
becomes, and any remedy. **What's next**: review request.
