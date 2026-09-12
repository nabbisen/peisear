# LAYOUT-008 — the sprint pages, the workload chips, and a third shape

**Target release**: 0.33.0. **Governing RFC**: none — defect fix.
**Source**: `LAYOUT-007-review.md` §4; `§10.25`. **`REL-0.33.0` waits on this.**

## 1. The defects

Measured on a release build of `8e5efe9` with a 64-character unbroken run in
the team name, sprint name, sprint goal and project name, and an 80-character
run as the display name:

| Page | 320 | 390 | 768 | 1280 | Element | Shape |
|---|---|---|---|---|---|---|
| sprints list (`/teams/{slug}/sprints`) | 383 | 313 | 0 | 0 | the row's `h3.font-medium` inside `div.flex-1.min-w-0` — `sprints.rs` | **B** — the wrapper already has `min-w-0`; the text has no break opportunity. The teams-list pattern from `LAYOUT-004` |
| sprint plan (`…/plan`) | 633 | 564 | 200 | 0 | `h1` as a direct item of `div.flex.items-center.gap-3` — `sprint_plan.rs:117` | **A** |
| **sprint detail** (`…/sprints/{id}`) | 781 | 711 | 348 | **68** | `h1` inside `div.flex.items-center.gap-3` inside a **bare wrapper `<div>`** that is the item of `div.flex.items-center.justify-between` — `sprints.rs:~670` | **A, nested twice**, plus the goal line (`p.italic`) and the dates line, shape **B** inside the same wrapper |
| issue new, issue edit (`…/issues/new`, `…/edit`) | 344 | 274 | 0 | 0 | the workload hint's chips: `span.text-base-content/70` holding a display name, inside `span.inline-flex.items-center.gap-1`, inside `span.inline-flex.items-center.gap-3.flex-wrap` — `issues.rs:568-579` | **C — new**, §2 |

Sprint detail overflows **on a 1280 px desktop**: the wrapper's min-content
(937 px for the fixture name) squeezes the buttons column beside it to 136 px
and its `flex-wrap` buttons spill. Not a phone defect.

None of the four pages is in the gate.

## 2. The third shape, measured

A **shrink-to-fit container** — `inline-flex` here; `inline-block`, a float or
a table cell would behave the same — sizes itself to its content's
**min-content**. `min-w-0` on the items does not change what they contribute
to that min-content, and `overflow-wrap: break-word` is defined not to affect
min-content at all. So the two remedies the rule knows are both inert:

| applied to the chips at 320 | overflow |
|---|---|
| nothing | 344 |
| name `min-w-0` + `break-words` | 344 |
| chip `min-w-0` + name `min-w-0` + `break-words` | 344 |
| outer `min-w-0` as well | 344 |
| **name `overflow-wrap: anywhere`** | **0** |
| outer as block-level `flex` + chip `min-w-0` + name `min-w-0 break-words` | 0 |
| name `word-break: break-all` | 0 — **rejected**: breaks ordinary words at every line end |

`overflow-wrap: anywhere` differs from `break-word` in exactly one respect:
its soft-wrap opportunities count toward min-content. Everything else about
where and when it breaks is the same, and on an ordinary title beside
`shrink-0` buttons the rendered geometry is identical to `break-word` at 320,
390 and 1280 (measured, `LAYOUT-007-review.md` §4).

**Decision: shape C takes `overflow-wrap: anywhere` on the text, and nothing
structural changes.** One class, no display change, no chain of `min-w-0`.

**Not a universal remedy, and this was tested so nobody re-derives it.** With
every existing `min-w-0`/`break-words` stripped, `anywhere` alone clears nine
of the thirteen known sites and fails two: the board's `line-clamp-2` title
(a `-webkit-box` does not shrink its min-content; the column's `min-w-0` is
what bounds it) and the list's `<select>` (a control, not text). The rule
keeps its two shapes and gains a third row; it does not collapse to one class.

## 3. What to do

- **Add the utility once.** `style/tailwindcss/input.css`, in
  `@layer utilities`: `.wrap-anywhere { overflow-wrap: anywhere; }` — named
  as Tailwind v4 names it, so a future upgrade renames nothing. Regenerate
  `static/tailwind.css` per that directory's README, binary checksum verified;
  report the class-level diff.
- **Sprints list**: `break-words` on the row link or its `min-w-0` wrapper
  (shape B; the wrapper carries the goal too if it is rendered there —
  check). Reference the rule.
- **Sprint plan**: `min-w-0 break-words` on the `h1` (shape A). Measured →
  0.
- **Sprint detail**: `min-w-0` on the bare wrapper **and** on the `h1`,
  `break-words` on the **wrapper** so the goal and dates lines inherit it.
  Measured, one step at a time at 320: wrapper alone 633, + `h1` `min-w-0`
  548, + `h1` `break-words` 234, + wrapper `break-words` **0**; and 0 at
  1280. Confirm by injection — two nested items is the first of its kind.
- **Workload chips**: `wrap-anywhere` on the name `<span>` (`issues.rs:568`).
  Both issue new and issue edit render the same component.
- **The rule** in `components.rs` gains shape C: what it is, why the two
  known remedies are inert there, the one that works, and the "not universal"
  paragraph above in two sentences.

## 4. The gate

- **The fixture gains a sprint**: created through the real form
  (`name`, `goal`, `starts_on`, `ends_on`) with the 64-character run in the
  name **and** the goal. That is the fifth deliberate fixture property;
  document it in `browser-checks/README.md`.
- **Four pages join the list**: `issue_new`, `sprints`, `sprint_detail`,
  `sprint_plan`. **Watch them go red first** — expect `sprint_detail` red at
  320, 390, 768 **and 1280**, the others at 320 and 390 — then fix, then
  green. Two commits.
- Coverage line: **18 pages × 5 widths = 90**. Update the README and the CI
  comment in `.github/workflows/test.yml`.

## 5. Verification

- Every site in §1 at **0** at 320 / 390 / 768 / 1280, with the fixture above
  and with a name that is one 80-character run.
- Gate **90/90**, exit 0, 0 non-local requests; the red run in the package.
- Ordinary text pixel-identical at 768 and 1280 on every touched surface with
  spaced names — including the chips, which is where `wrap-anywhere` is new.
- The text still reads: box and content sizes match, nothing clipped.
- `static/tailwind.css` regenerates byte-identical from the committed source.
- `DEC-007` **256**, three consecutive workspace runs, `fmt`, `clippy`.

## 6. Escalate rather than deciding

- **If any site is not the shape measured**, or a fourth appears.
- **If the new fixture turns a page red that §1 does not name.**
- **If `wrap-anywhere` changes ordinary text anywhere.** It should not; measure
  it.
- **If sprint detail needs more than the four classes in §3** — nested shape
  A may have a cleaner fix (flatten the wrapper) and that is a design change.

## 7. Exit condition

Five pages at 0 at four widths, the gate at 90/90 having been watched red on
four new pages first, the rule carrying shape C, `wrap-anywhere` defined once
and the vendored CSS regenerated, `DEC-007` at 256, three consecutive
workspace runs.

---

**Who holds what**: dev team. **Depends on**: nothing. **What's next**: review
request, then `REL-0.33.0`.
