# A11Y-008 — a focus ring drawn in transparent

**Issued by**: Architect
**Date**: 2026-10-07
**Target release**: 0.42.0
**Related requirements**: `NFR-A11Y-001`'s recorded residue; `NFR-A11Y-010`.
**Depends on**: nothing. **Smallest of the three.**

---

## 1. The defect

`A11Y-001` tabbed the board's first 14 stops and read the computed focus style:
**10 have a drawn ring; 4 do not.** The four are the **account menu's links** —
Today, Teams, Inbox, Settings — which carry `outline: solid 2px` with a
**transparent colour** and no shadow under `:focus-visible`. What a user sees
is a faint grey row background, judged by eye and **not contrast-measured**.

**The style comes from the vendored `static/daisyui.min.css`**, not from the
component — confirmed. **`DEC-051` vendored that file and it is not to be
edited**: the fix is a class on the element in the component, overriding the
vendored default.

## 2. What to do

- **Measure what is there first**, and this is the part `A11Y-001` left
  undone: the grey row background's contrast against its neighbours. **If it
  already reaches 3 : 1** (`NFR-A11Y-010`), the menu has a visible focus
  indicator that is merely unconventional, and **the right outcome may be to
  record that and change nothing.** Report the number before proposing a
  change.
- **If it does not**: give the four links a visible ring with a class in
  `components/layout.rs`, matching what the other ten stops already draw —
  **reuse the existing appearance rather than inventing a second focus style.**
  A product with two kinds of focus ring is worse than one with an
  unconventional one.
- **`:focus-visible`, not `:focus`** — a mouse click on a menu item must not
  draw it, which is why the other ten use the former.

## 3. Verification

- **The contrast number for the current state** (§2), which decides the rest.
- **All 14 board stops after the change**: 14 with a drawn ring, and **the
  other ten unchanged** — report their computed styles before and after.
- **Mouse click draws nothing**, keyboard focus draws the ring.
- **The ring reaches 3 : 1** against the menu's background (`NFR-A11Y-010`).
- **The dropdown still opens and closes** by keyboard and by mouse.
- `DEC-007` reported; **the overflow gate** if a class changes the box.
- `fmt`, `clippy`, three consecutive workspace runs.

## 4. Escalate rather than deciding

- **If the current grey already passes 3 : 1** — stop and report; the answer
  may be to record it (§2).
- **If the fix requires touching the vendored stylesheet.** It does not, as far
  as I can tell, and if it does that is a `DEC-051` question and mine.
- **If other components share the same transparent-outline default.** Four
  links were measured on one page; **say how many there are** rather than
  fixing the four.

## 5. Exit condition

Every keyboard stop in the account menu draws a focus indicator reaching
3 : 1, using the appearance the rest of the product already uses — or a
measurement showing the existing indicator already conforms, recorded.
