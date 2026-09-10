# LAYOUT-003 — the board's columns overflow at 320 px

**Target release**: 0.33.0. **Governing RFC**: none — defect fix.
**Source**: found 2026-09-10 while reviewing `A11Y-006`; `§10.24`.

## 1. The defect

**Project detail's board columns each overflow the viewport by 33.4 px at
320 px** when an issue title contains a long unbroken run of characters.

| | 320 | 360 | 390 | 414 |
|---|---|---|---|---|
| project detail (board) | **33.4** | 0 | 0 | 0 |

Fixture: one issue titled *"Fixture issue whose title is long and contains an
unbroken run AAAA…"*. With a short title, zero at every width.

## 2. The cause is one you have already fixed once

**This is `LAYOUT-001`'s mechanism on a different element.**

`#board` is `grid gap-3 md:grid-cols-3` — below 768 px it is a single column.
**A grid item's default `min-width` is `auto`, meaning its content's minimum**,
and an unbreakable word's minimum is the whole word. `line-clamp-2` truncates
*lines*; it does not introduce a break opportunity. So the title sets the
column's minimum width and the column leaves the grid — exactly as `menu-title`
left the account menu at `LAYOUT-001`.

**The fix there was `min-w-0` on the flex item plus `overflow-hidden`, and
`truncate` on the text.** Whether the same shape is right here is yours to
determine — a board card's title legitimately wraps to two lines, so `truncate`
is probably wrong and the break behaviour needs thought.

## 3. Why nothing caught it

- **`BROWSER-001`'s gate starts at 390 px.** This is only visible below that.
- **`A11Y-006`'s audit never measured project detail** — the requirement's four
  flows do not include it.
- **No test observes rendered layout** — `§17.8`.

**Do not widen `BROWSER-001` to 320 as part of this.** Whether the gate's width
set should change is a separate decision with its own cost, and bundling it into
a defect fix would pre-empt it. **Report whether you think it should**; that is
useful and it is not this handoff's to settle.

## 4. Verification

`browser-checks/cdp.mjs`.

- **`scrollWidth - clientWidth` is 0 on project detail at 320, 360, 390, 414**,
  with **an issue whose title contains a long unbroken run** — a short-title
  fixture cannot see this defect and is not sufficient evidence.
- **The four pages `BROWSER-001` already covers stay at 0** at all four widths.
- **The board still works**: the columns are readable, cards are draggable
  targets, and `TT-004`'s 44 px targets are intact.
- **The title still conveys the issue.** A fix that hides the title to stop the
  overflow trades one defect for a worse one.

## 5. Escalate rather than deciding

- **If the fix changes how titles wrap on wider viewports.** That is a visual
  change to the primary content of a card and it is a design question.
- **If the same shape appears elsewhere** — any grid or flex container holding
  user-supplied text is a candidate, and `LAYOUT-001` plus this makes two. **A
  third would suggest the pattern deserves a rule rather than a third fix.**

## 6. Exit condition

Zero overflow at four widths with a long-title fixture, no regression on the
existing pages, `DEC-007` clean at **256**, three consecutive workspace runs.

---

**Who holds what**: dev team — the fix. **What's blocked**: nothing.
