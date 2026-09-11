# LAYOUT-005 — the two sites `LAYOUT-004` reported, and the gate that could not see them

**Target release**: 0.33.0. **Governing RFC**: none — defect fix.
**Source**: `LAYOUT-004`'s package §2; `§10.25`.

## 1. The defects

`LAYOUT-004`'s sweep classified seven sites and fixed five. The two it reported
rather than fixed, re-measured on the fixed tree with a 64-character run in the
project name and the team name:

| Surface | 320 | 360 | 390 | 414 | 768 | Shape | Element |
|---|---|---|---|---|---|---|---|
| **project calendar** | 548 | 507 | 478 | 453 | **114** | **B** | `h1.text-xl.mb-1`, `components/calendar.rs:451` |
| **team detail** | 643 | 603 | 574 | 549 | **210** | **A** | the `h1`'s wrapper `<div>`, `components/teams.rs:~309` — **the wrapper is the flex item, not the `h1`** |

Both overflow at 768, past the phone widths. Both were left unfixed because
neither handoff granted the scope — correctly. Both are **absent from the
gate's page list**, which is why the fixture that exposed the other five never
saw them.

## 2. The fixes — the rule already says what they are

`components.rs`'s module doc, "Rendering user-supplied text: two shapes, two
remedies". Do not re-derive it; classify against it.

- **Project calendar** — shape B. `break-words` on the `<h1>`. `LAYOUT-004`
  measured `break-words` → 0 and `min-w-0` → no change.
- **Team detail** — shape A. `min-w-0` on the wrapper `<div>` **and**
  `break-words` on the `<h1>`. `LAYOUT-004` measured `break-words` alone → 513,
  `min-w-0` alone → 418, both → 0. **Confirm by injection which element is
  the flex item before editing** — `LAYOUT-004` found the answer differs
  between the two shape-A sites, and assuming it half-fixes the other one.
  The wrapper carries the description `<p>` too, so `min-w-0` on the wrapper
  is what bounds both.

One-line reference to the rule at each site, as the other five do. **No copy
of the rule.**

## 3. The gate: add both pages, and watch them go red first

`LAYOUT-004`'s rule says the fixture, not a source guard, is what catches this
class. That is true only for pages the gate visits, and the two sites above
are exactly the two it does not. A fix here without a gate page is a fix that
can silently reopen.

In `browser-checks/overflow-gate.mjs`:

- Add `project_calendar` (`/projects/{id}/calendar`) and `team_detail`
  (`/teams/{id}`) to the page list. The fixture already creates a team with
  the 64-character run in its name (`BROWSER-002`); return its id from
  `createFixtures`.
- **Run the gate red before the fix**: both new pages failing at 390 and 768
  on the current tree, with the cell values in the package. Then fix, then
  green. The order is the evidence.
- Rename the `board` key to `list` — it points at `?view=list`. The board is
  `project_detail`, the default view. A reader of the coverage log should not
  believe a page is covered under a name that belongs to a different page.
- The coverage line becomes **14 pages × 4 widths = 56**. Update
  `browser-checks/README.md` where it counts.

## 4. Verification

- Both sites at **0** at 360 / 390 / 414 / 768 with the 64-character run.
- At **320**, every page carries `BROWSER-002` §4's navbar overflow of 24 until
  `LAYOUT-006` lands. Measure the two sites at 320 with the display name
  renamed to `"Al"` in place, and expect 0 — or land after `LAYOUT-006` and
  expect 0 outright. Say which.
- Gate **56/56**, exit 0, zero non-local requests; the red run in the package.
- Ordinary titles pixel-identical at 768 and 1280 on both pages, before and
  after — `LAYOUT-004` §4's method, not its assertion.
- The text still reads at 320: box and content size match, nothing clipped.
- `DEC-007` **256**, three consecutive workspace runs, `fmt`, `clippy`.

## 5. Escalate rather than deciding

- **If either site is not the shape measured.** The remedies are not
  interchangeable, and a site that needs neither is a fourth mechanism.
- **If the new fixture surfaces an overflow on a page not named here.**
  Report it; do not widen.
- **If adding a page to the gate needs anything the fixture does not already
  create.** A gate page that needs its own setup is a design question.

## 6. Exit condition

Two sites at 0 at four widths (and at 320 by the stated method), gate at 56/56
having been watched red on both new pages first, `board` renamed, `DEC-007` at
256, three consecutive workspace runs.

---

**Who holds what**: dev team — both fixes and the two gate pages. **Depends on**:
nothing to start; the 320 measurement's method depends on whether `LAYOUT-006`
has landed. **What's next**: review request.
