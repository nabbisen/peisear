# A11Y-005 — the toast is now a child of a draggable element

**Issued by**: Architect
**Date**: 2026-09-29
**Target release**: 0.41.0
**Source**: `A11Y-004` §5 — flagged as untested by the dev team, ruled a
regression in `.git-exclude/reviewed/A11Y-004-review.md` §5.
**Depends on**: `A11Y-004`, landed. **Small.**

---

## 1. What changed underneath

`A11Y-004` moved the undo toast from the end of `<body>` into the element that
was acted on. On the board that element is **`.issue-card`, which carries
`draggable="true"`** (`components/issues.rs:880`); on the calendar it is the
block, which is also draggable.

**A mousedown on a child of a draggable element can start that element's
drag.** So pressing *Undo* with any pointer movement may begin dragging the
card instead of — or as well as — activating the button.

**The codebase already knows this shape.** Two lines below the `draggable`
attribute, `issues.rs:881` sets **`draggable="false"`** on an `<a>` inside the
card, with the comment *"an `<a href>` is draggable by default"*. The toast
arrived after that reasoning and did not inherit it.

## 2. What to do

- **Measure it first.** A real mouse gesture — mousedown on *Undo*, move a few
  pixels, mouseup — on the board and on the calendar. **Report what happens
  before any fix**: does the drag start, does Undo still fire, does the card
  move. If nothing goes wrong, say so and stop; **the attribute is not worth
  adding to prevent a problem that does not occur.**
- **If it does**: `draggable="false"` on the toast element, in the scripts that
  put it inside a draggable holder. Check whether `dm.js` and `plan.js` need it
  too — a row or a form may or may not be draggable; **establish it rather than
  applying the attribute everywhere.**
- **Do not change the holder's own `draggable`.** The card must stay draggable;
  that is `D-2`.

## 3. Verification

- **The gesture, before and after, on every surface where the holder is
  draggable.** Both outcomes stated.
- **The keyboard path is unchanged**: `A11Y-004`'s Tab counts and both endings,
  re-run — 3 presses on detail and list, focus never `body`.
- **The drag itself still works**: a card can still be dragged from its body,
  with a toast present and without one.
- `§10.15`: the **no-reload sequence** if any script changes. The
  before-and-after rendering check is **not** needed for an attribute that
  changes no computed style — **say so rather than skipping it silently**, and
  run it if the toast's markup changes in any other way.
- `DEC-007` unchanged at **369** unless a Rust file is touched; `fmt`,
  `clippy`; the overflow gate is untouched.

## 4. Escalate rather than deciding

- **If the gesture is harmless** — report and stop (§2).
- **If `draggable="false"` on the toast breaks the card's own drag** in any
  browser you can check.
- **If a holder other than the card and the block turns out to be draggable.**

## 5. Exit condition

Pressing Undo with a pointer cannot start a drag on any surface, the card's own
drag is unaffected, and `A11Y-004`'s keyboard results are unchanged — or a
measurement showing the problem never existed.
