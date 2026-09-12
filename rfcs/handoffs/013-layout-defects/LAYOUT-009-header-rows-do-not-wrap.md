# LAYOUT-009 — page header rows squeeze the title instead of wrapping

**Target release**: 0.34.0. **Governing RFC**: none — defect fix.
**Source**: `LAYOUT-008`'s package §5, measured in `LAYOUT-008-review.md` §4;
`§10.27`. **Not in `REL-0.33.0`** — a different class from `§10.25`, and the
release does not wait on it.

## 1. The defect

A page header is a flex row: title on the left, actions on the right,
`justify-between`, no wrap. When the row cannot fit both, the actions column
keeps its content width and the title takes what is left. Measured with
**ordinary** names on a release build of `7b5301e`:

| Page | width | title column | `h1` lines | actions |
|---|---|---|---|---|
| **sprint detail** (`sprints.rs:~676`) | 320 | **54 px** | **8** | 136 px — Plan, Edit, Delete, Start sprint, already wrapping into two rows inside their own column |
| | 360 | 93 px | 5 | |
| | 390 | 110 px | 4 | |
| | 414 | 124 px | 3 | |
| **issue detail** (`issues.rs:~1828`) | 320 | 143 px | 4 (a 43-character title) | 2 controls, `shrink-0` |

Nothing overflows and nothing is clipped, so `BROWSER-001` is green on both.
This is not `§10.25`: the names have spaces. It is `LAYOUT-002`'s mechanism —
a row that cannot wrap — on the two headers that carry actions beside a
user-supplied title.

## 2. The design decision

**When a header row cannot fit the title and its actions side by side, the
actions drop below the title.** The title keeps the full width; the actions
row keeps its controls at 44 px. No breakpoint: the row wraps when its content
does not fit and not otherwise, which is how `LAYOUT-002` fixed the project
toolbar and how the navbar (`LAYOUT-006`) decides what yields.

- Not "shrink the buttons" — the touch target floor forbids it.
- Not "hide the actions behind a menu on phones" — a second rule, and a
  discoverability cost for the page's primary actions.
- Not a fixed `min-w` on the title — a number, and it does nothing at the
  width where the number is still too wide.

## 3. What to do

- Both header rows gain `flex-wrap`. Confirm by injection at 320 / 360 / 390
  / 414 that the actions drop below **only when the row cannot fit**, and
  that on a 768 px viewport with an ordinary title the row is
  pixel-identical to today.
- **Sprint detail** also carries a status badge beside the `h1` inside the
  wrapper. Decide by measurement whether the badge stays beside a wrapped
  title or belongs on the dates line; report what you chose and why.
- **Issue detail**: the `shrink-0` on the actions stays — with `flex-wrap`
  it is what makes the actions drop rather than squeeze.
- The `§10.25` classes on both headers (`min-w-0 break-words`) stay: a
  wrapped row still needs the title to break an unbreakable run.

## 4. Verification

- Both pages at 320 / 360 / 390 / 414: the `h1` spans the content width
  (report its box), no more than the lines an ordinary title needs, actions
  below at 44 px each.
- Both pages at 768 and 1280: pixel-identical before and after with an
  ordinary title.
- With the gate's fixture (an unbroken run): still 0 overflow; the run wraps
  across the full width.
- **Gate 90/90** — this changes nothing it observes, which is the point of
  `§10.27`'s entry: a header that squeezes its title to 54 px passes an
  overflow gate.
- `touch_target` unchanged; `DEC-007` **256**; three consecutive workspace
  runs; `fmt`, `clippy`.

## 5. Escalate rather than deciding

- **If a third header has the same shape.** Team detail's header was
  measured at 0 and reads at 320, but look.
- **If `flex-wrap` makes the actions drop on a width where they fit.**
- **If the badge has no good place.**

## 6. Exit condition

Two headers wrapping at phone widths and pixel-identical at desktop widths,
the `h1` box reported at four phone widths on both, gate 90/90, `DEC-007` at
256, three consecutive workspace runs.

---

**Who holds what**: dev team, after `REL-0.33.0`. **Depends on**: nothing.
**What's next**: review request.
