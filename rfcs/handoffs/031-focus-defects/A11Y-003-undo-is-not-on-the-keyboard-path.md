# A11Y-003 — undo is not reachable from the keyboard

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.41.0
**Related requirements**: **`FR-DM-002`** (keyboard parity), `NFR-A11Y-002`.
**Governing RFC**: `RFC 0004`, requirement 1. **Source**: `A11Y-001` §1.1,
measured. **Depends on**: nothing. **After `A11Y-002`.**

---

## 1. The defect

The undo toast is appended to the **end of `<body>`**, not beside the control
that was acted on, and lives **five seconds**. Tab presses from the pressed
button to reach *Undo*, measured:

| page | presses |
|---|---|
| issue detail | 2 |
| issue list | **13** |
| board, after a drop | **33** |

**Thirteen to thirty-three presses inside five seconds is not a path.** Focus
is not moved to the toast, and when the toast ends — pressed **or** expired —
focus drops to `body`.

**`FR-DM-002`**: *every direct-manipulation action MUST have a keyboard
equivalent producing the identical effect.* `RFC 0004b`'s open question 3
decided undo was worth building **because the inconsistency between surfaces
was visible to a user**. It was built on the pointer path only. **Five releases
have shipped with it and nobody counted the presses.**

## 2. What to do — and this is a design question, so bring the options

**The requirement is that a keyboard user can undo.** How is yours to propose
and mine to choose. **Do not implement a preference; measure two and report.**

Shapes worth considering, not a menu to pick from:

- **Move focus to the toast** when it appears. Solves reach; **takes focus away
  from someone typing**, and the toast is transient — where does focus go when
  it expires? The cure for §1's last row can be worse than the disease.
- **Put the toast next to the acted-on control** in DOM order, so it is the
  next Tab stop. Keeps focus where it was; constrains where the toast can be
  drawn, on three surfaces with different layouts.
- **A keyboard shortcut** — the usual answer, and this product has none and
  `DEC-021`'s posture makes adding a global one a decision, not a detail.
- **Extend or pause the timer on keyboard interaction.** Cheapest; does nothing
  about 33 presses.

**Report what each costs on the board**, which is the worst case and the one
that decides it.

## 3. What must stay true

- **`§10.15` applies**: this is `static/*.js`, executed by no test. **Both
  required evidence runs** — the no-reload sequence and, if the toast's markup
  or position changes, the before-and-after rendering check. `PLAN-002` round 1
  is why.
- **The five-second lifetime is `RFC 0004`'s**, not incidental. Changing it is
  a decision; propose, do not take.
- **The pointer path must not regress.** Its Tab count is 2 on issue detail
  today, and a fix that helps the board must not cost that.
- **`JS-003`'s outcome classification** and the announcements are untouched.

## 4. Verification

- **Tab counts, before and after, on all three surfaces** — the measurement
  that found this is the one that closes it.
- **Both toast endings**: pressed, and expired while focused. Where does focus
  go? It must not be `body`.
- A pointer user's experience is unchanged: report it.
- `DEC-007` reported; **the overflow gate** if the toast's markup changes.
- `fmt`, `clippy`, three consecutive workspace runs.

## 5. Escalate rather than deciding

- **Before writing the fix**, with §2's options and their costs. **This handoff
  expects a round trip**; implementing a shape and reporting it afterwards is
  not what it asks for.
- **If no shape keeps the pointer path at 2 presses** while fixing the board.
- **If the answer needs the toast's lifetime changed.**

## 6. Exit condition

First: a report of two shapes with their costs on the board. Then, on my
ruling, undo reachable from the keyboard on all three surfaces with the Tab
counts to show it, and focus never landing on `body` when a toast ends.
