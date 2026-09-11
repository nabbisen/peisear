# LAYOUT-006 — every page overflows a 320 px phone when the display name is 21 characters

**Target release**: 0.33.0. **Governing RFC**: none — defect fix.
**Source**: `BROWSER-002`'s package §4; `§10.26`. **Sequenced before
`BROWSER-002` round 2** (320 px), which lands green only once this is fixed.

## 1. The defect

At 320 px, with the signed-in user named `"Overflow Gate Fixture"` (21
characters, spaces at every point), **every page** has `scrollWidth` 344 against
`clientWidth` 320. Renaming the user to `"Al"` in place takes every page to 0
with nothing else touched. Reproduced by two people on two builds.

**This is a third mechanism, not `LAYOUT-004`'s.** Nothing here is unbreakable;
the account button's label is ordinary text. The navbar is a flex row —
brand, bell, account button — and at 320 the row's combined content minimum
is simply wider than the viewport. `overflow-wrap` cannot help, because
nothing needs breaking; the row needs to be allowed to give something up.

Real names of 21 characters are ordinary. Every page of the product scrolls
sideways on a 320 px phone for such a user, and 320 is the width `BROWSER-002`
was written to add.

## 2. The design decision — made here, so it is not made in the code

**The account button keeps showing the user's name at every width, and the
name gives way with an ellipsis only when the row cannot fit.** No breakpoint,
no fixed character limit, no hiding the name on phones.

Why this and not the two obvious alternatives:

- **Not "hide the name below `sm`".** A phone is where a user is least sure
  which account they are in, and it is a second rule for one row.
- **Not `max-w-[Nch] truncate`.** A number chosen for 320 truncates names on a
  1280 px screen that has room for them, and the number is a maintenance cost
  forever. The breadcrumb's `max-w-[24ch]` is a different case — a trail must
  stay one line at any width.

**The mechanism is `LAYOUT-001`'s, applied to the row instead of the menu**:
the flex item that carries the name must be allowed to shrink (`min-w-0` on
the item, so its content-based minimum stops setting its size), and the name
must be its own element that truncates (`truncate` on a `<span>` holding
`display_name`, `shrink-0` on the chevron). The brand keeps its content width.
`grow()`'s `min-w-11` already floors the button at 44 px, so the name can
shorten but the target cannot — `NFR-A11Y-007` holds without a new rule.

**Confirm the chain by injection before editing.** DaisyUI's `.navbar > *`
makes every child a flex container of its own, and the right-hand block is
`flex-none`; which of the nested items owns the content minimum is a
measurement, not a reading — `LAYOUT-004` found two shape-A sites with two
different answers. Inject `min-width: 0` one level at a time at 320 and record
which level takes 344 to 320.

## 3. What must not change

- **The gate's fixture display name.** `"Overflow Gate Fixture"` stays 21
  characters. `BROWSER-002` said why: shortening it turns 320 green by removing
  the input that produced the finding.
- **The account menu itself.** `LAYOUT-001`'s `menu-title` fix, `TT-004`'s
  targets inside the menu, and the menu's contents are untouched.
- **Wide viewports for ordinary names.** A 21-character name at 768 and 1280
  renders whole, in the same box, before and after — measure it.

## 4. Verification

- `/today`, `/inbox`, `/projects`, issue detail at **320**: overflow **0** with
  the 21-character name. Then with an **80-character** name — the form's own
  ceiling — also 0, and the button still ≥ 44 × 44.
- **What the button shows at 320 / 360 / 390** with the 21-character name:
  the rendered text and a screenshot. A user must be able to recognise their
  own name from what remains.
- The button at 768 and 1280 with the 21-character name: **not truncated**,
  pixel-identical before and after.
- `touch_target` tests and `DEC-007` at **256**; `fmt`; `clippy`; three
  consecutive workspace runs.
- `BROWSER-001` still **48/48** (or 56/56 if `LAYOUT-005` has landed), 0
  non-local requests.
- Then, as `BROWSER-002` round 2: `[320, 568, true]` added, **60/60** (or
  70/70) green, plants B and C still red under it.

## 5. Escalate rather than deciding

- **If brand + bell + a 44 px button do not fit 320 on their own.** Then the
  floor, not the name, is the problem, and that is a design question.
- **If DaisyUI's `.navbar` rules defeat `min-w-0`** and the fix needs an
  override in `input.css`. One override is a fourth rule; say so first.
- **If the name truncates on a viewport that had room for it.** That is the
  `max-w-[Nch]` failure by another route.

## 6. Exit condition

Four pages at 0 at 320 with the 21- and 80-character names, the button's
rendered text at three phone widths reported, wide viewports unchanged, the
touch target intact, `DEC-007` at 256, three consecutive workspace runs — and
`BROWSER-002` round 2 green on 60 cells with its plants still failing.

---

**Who holds what**: dev team — the fix, then `BROWSER-002` round 2. **Depends
on**: nothing. **What's next**: review request, with round 2's evidence in the
same package or the next.
