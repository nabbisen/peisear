# LAYOUT-004 — unbreakable user text, three surfaces, two shapes

**Target release**: 0.33.0. **Governing RFC**: none — defect fix.
**Source**: `LAYOUT-003`'s own sweep (§6 of its package); `§10.25`.

## 1. The defects

With an issue title containing a long unbroken run, measured on a fresh build
of the `LAYOUT-003` tree:

| Surface | 320 | 360 | 390 | 414 | Element |
|---|---|---|---|---|---|
| **issue detail** | **694** | 654 | 624 | 600 | `h1.text-xl` — a flex item beside Edit/Delete |
| **issue delete interstitial** | **526** | 486 | 456 | 432 | `h1.text-lg.text-error` in the card body |
| **search results** | **380** | 340 | 310 | 286 | `div.font-medium` in the result row |

**Issue detail is worse than the board defect `LAYOUT-003` fixed** — it
overflows at 390 and 414, widths `BROWSER-001` already sweeps, and the gate is
green. **There is no safe width**: the overflow is `min-content − viewport`, so
a longer run overflows at any viewport.

## 2. Two shapes, and the remedies are not interchangeable

**This is the finding, and it is why this is one handoff and not three copies of
`LAYOUT-001`.** Measured by injecting each remedy at 320:

| | what is too wide | `break-words` alone | `min-w-0` alone | both |
|---|---|---|---|---|
| **A** — issue detail `h1` | **the box** — a flex item with `min-width: auto` | **no change** (694) | 550, still broken | **0** |
| **B** — issue delete `h1` | **the text** — the box is correct | **0** | **no change** (526) | 0 |
| **B** — search title | the text | **0** | no change (380) | 0 |

**Shape A** — a flex or grid item whose `min-width: auto` lets its content set
its size. `LAYOUT-001` (`menu-title`), `LAYOUT-003` (the board column), and issue
detail's `h1`. Needs **`min-w-0` *and* a break opportunity** — `min-w-0` alone
clears the box and cuts the text mid-word; `break-words` alone does nothing,
because `overflow-wrap: break-word` is defined not to affect min-content
intrinsic size.

**Shape B** — an ordinary block whose text overflows a correctly-sized box.
Delete interstitial, search results. Needs **`break-words` only**; `min-w-0` is
a no-op.

**Applying the wrong remedy is silent in both directions.** Do not fix these by
pattern-matching `LAYOUT-001`. Classify each site first; the classification is
the fix.

## 3. What to do

- **Issue detail `h1`**: shape A — `min-w-0` on the item, `break-words` on the
  text. **Confirm which element is the flex item** (the `h1` or its wrapper) by
  injection before editing, as `LAYOUT-003` did.
- **Delete interstitial `h1`**: shape B — `break-words`.
- **Search result title**: shape B — `break-words`.
- **Check the project delete interstitial and the breadcrumb** with a long
  project *name*, not just an issue title. Project delete measured 0 here because
  my fixture's project name was short. Different input, same mechanism.

**Record the two-shape rule in a comment at one site**, the way `LAYOUT-003`'s
comment records why both classes are needed, and reference it from the others.
A rule that lives in three copies of a comment is `RFC 006`'s problem again.

## 4. Verification

`browser-checks/cdp.mjs`, fixture with **≥ 64-character unbroken run** in the
issue title **and** the project name.

- All three surfaces at **0** at 320 / 360 / 390 / 414.
- **`BROWSER-001` still 48/48** — and note it will not see your fix, because its
  fixture cannot see the defect; `BROWSER-002` changes that.
- **Ordinary titles pixel-identical at 768 and 1280** — measure a short title
  before and after on each surface. `LAYOUT-003` showed the only change on wide
  viewports is a long-run title wrapping instead of being cut mid-word; the same
  must hold here.
- **The text still reads.** A remedy that hides the title trades one defect for
  a worse one.

## 5. Escalate rather than deciding

- **If a site is neither shape** — a fourth mechanism is a finding.
- **If `break-words` changes wrapping of ordinary text** anywhere. It should not
  (it only adds break opportunities inside words that would otherwise overflow),
  but measure it.
- **If the sweep finds more sites** — the three above are what one fixture
  exposed; a long project name or a long team name may expose others. Report
  them; do not silently widen.

## 6. Exit condition

Three surfaces at 0 at four widths with a long-run fixture, wide viewports
unchanged for ordinary text, `DEC-007` at **256**, three consecutive workspace
runs, the two-shape rule recorded once in the source.

---

**Who holds what**: dev team — the three fixes. **What's blocked**: nothing.
