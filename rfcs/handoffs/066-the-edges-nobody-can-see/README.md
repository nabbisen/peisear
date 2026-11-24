# 066 — The edges nobody can see

**Series index.** `0.68.0`'s scope, authorized by the owner 2026-11-24 —
**the row, not the cut**. Opened the same day.

**Three releases taught `contrast_scan` to read text colour.** The muted
floor, the semantic families, the colours the stylesheets apply — and **all
three are text-only by their own documented scope.** The layer below is
**WCAG 1.4.11 non-text contrast**, which asks **3:1** for the visual
information needed to identify a control and its states. This product has
never measured it, and it fails.

---

## 1. What was measured

Tokens resolved from the pinned sheets (`ASSET-001`, `DEC-051`), composited
over the product's own backgrounds, OKLCh → sRGB → relative luminance —
the method the last three releases used:

| what the sheet draws | resolved | on `base-100` | on `base-200` |
|---|---|---|---|
| `.input-bordered`/`.select-bordered`/`.textarea-bordered`/`.checkbox` border — `base-content/20` | `#d1d1d4` | **1.52:1** | **1.50:1** |
| `.input:focus`/`:focus-within`, `.select:focus`, `.textarea:focus` **outline** — the same `/20` | `#d1d1d4` | **1.52:1** | **1.50:1** |
| `base-content/30`, for comparison | `#bababf` | 1.93:1 | 1.89:1 |
| `base-content/50` — **not** the first tier that clears 3:1; see the correction below | `#8c8c94` | 3.34:1 | **2.73:1 — fails** |
| **`base-content/70`** — the first tier that actually clears 3:1 everywhere measured | `#5d5f6a` | **6.42:1** | **5.24:1** |
| `.checkbox:focus-visible` outline — **full `base-content`**, which daisyUI already uses | `#181a2a` | **17.21:1** | 14.01:1 |

**Corrected 2026-11-24, by the round that measured it.** This index said
`/50` was the first tier clearing 3:1 on both backgrounds. It is not: a
semi-transparent border composites over **the element's own background**
(`background-clip: border-box` is the default), and these controls paint
their own white fill — so `/50` resolves to `#8c8c94` and reads **2.73:1**
against the page body, below the requirement. The architect's own figure
composited the border over the page instead, which is not what a browser
does. **`/70` shipped**, and `EDG-002` chose it as the only valid candidate
rather than as a preference.

**And the fill does not rescue the boundary.** These controls paint
`base-100`, so a field on a white card is **1.00:1** against its own
surround and **1.23:1** on the grey page body. **On a card, the 1.52:1
border is the only thing that says a field is there.**

**Re-derive before relying on any of it.** The command that matters is not a
grep: resolve the token, composite it over the background the element
actually sits on, and compare. If your measurement disagrees with this
table, **yours wins** — in the last three series the rounds corrected the
architect eleven times that way.

---

## 2. The populations, counted

| round | family | sites |
|---|---|---|
| `EDG-001` | focus rings — every `.input` (35), `.select` (23) and `.textarea` (9) — **closed** 2026-11-24 (`5878974`): full `base-content`, **17.21:1** on a card and 14.05:1 on the page body, the losing `/70` candidate measured (6.42/5.84) and its figures recorded at the rule | **67** |
| `EDG-002` | boundaries — the same 67 plus `.checkbox` (3) — **closed** 2026-11-24 (`07b2151`): `/70`, after measuring that the architect's recommended `/50` fails at **2.73:1** on the page body | **70** |
| `EDG-003` | the guard, the specification work, one register entry, and the `rfcs/` counted-population exclusion — **handed 2026-11-24** | — |

**Every `.input`/`.select`/`.textarea` site in the product carries
`-bordered`** — 35/35, 23/23, 9/9, checked rather than assumed — so the two
families are the same elements seen twice: once for the line around them,
once for the ring that appears when a reader tabs to them.

---

## 3. Method

1. **No new colour.** Every repair lands on a tier the product already
   uses: `base-content/50` for a boundary (the first that passes), and for a
   ring **the full `base-content` daisyUI already draws on a checkbox**.
   The vendored sheet is internally inconsistent — a checkbox's ring is
   17.21:1 and a text field's is 1.52:1 — so the fix is the product's own
   stronger pattern applied where it is missing.
2. **No change to the theme.** `--bc` is shared with every passing surface.
3. **Measure the rendered edge, not the declaration.** A border and an
   outline are different properties with different adjacent colours, and
   `§10.52` is why neither is believed from a file. **Screenshot and sample
   the pixels of the edge itself**, at both widths, resting and focused.
4. **The cascade is `PLC-002`'s direction**: these rules live in
   `daisyui.min.css`, served **first**, so an equal-specificity rule in
   `app.css` wins on order. But `.input:focus` carries a pseudo-class — an
   override must match that specificity, not merely the selector. **Plant
   it**: comment the override out, confirm `/20` returns, restore.
5. **A focus ring is not a decoration.** Where a judgement is needed,
   prefer the reader who cannot see where they are over the one who finds a
   strong ring unattractive, and say which you chose.
6. **The guard grows once, in `EDG-003`.**

---

## 4. Deliberately not in this series

- **Card and alert borders** — `border-base-300` (~1.3:1), `border-error/30`,
  `hover:border-primary`. Measured and out: each is decoration beside a
  shadow or a filled surface, **not the sole identifier of a control**,
  which is the test 1.4.11 actually applies. If a round disagrees for a
  specific site, say so as a finding rather than widening the scope.
- **The 21 remaining smallest-text sites**, the **nine** `link link-hover`
  sites, the **fifteen** bare-`opacity` sites, the **`xs` control tier** —
  recorded candidates, all text or type questions.
- **`.radio`/`.toggle`** — zero sites in the product today, confirmed by
  grep. The guard in `EDG-003` should fail the day one appears rather than
  pre-empting it here.
- **`/today`'s content** — the owner's own open question.

---

## 5. What each round owes

1. **Per family**: the rendered edge colour before and after, the adjacent
   colour it was measured against, and the ratio — **sampled, not computed**.
2. **Both widths (390 px, 1280 px)** and both states (resting, focused),
   since a ring only exists in one of them.
3. **The cascade planted**, not asserted.
4. **Gates in full**, with real exit codes (`cmd | tail -1; echo $?` reports
   `tail`'s status): the `DEC-007` block (**691** as it stands — say what it
   became), `fmt`, `clippy` both feature variants, `contrast_scan`,
   `touch_target_scan`, `alignment_scan`, `css_scan`'s population
   (`(287, 3)` — re-measure, never assume), and the browser gates whenever a
   served stylesheet changes.
5. **`CHANGELOG.md`** under `[Unreleased] → Fixed`: this is a defect, and
   the entry should say what a reader could not see — *where they were on
   the page* — rather than name a ratio.
6. **The privacy floor, per artefact**: a screenshot of a focused field may
   contain whatever the fixture typed into it. Use the fixture's own data,
   say which, and delete the image when the round closes. Scratch under
   `.git-exclude/tmp/`, never `/tmp`.
7. **Check the citation count before pushing.** `spec_scan` counts citations
   of four dangling register numbers across `rfcs/`, **including handoffs
   and reports** — the architect reddened `main` three times on it at
   `0.67.0`. `EDG-003` ends that; until it lands, re-derive the count after
   your last edit and before the push.
8. **Review request** at
   `.git-exclude/review-request/<round>-<slug>/README.md`.

---

## 6. Where things go

- **Handoffs** are published here, one file per round; **reviews stay
  private**.
- **Release numbers live in `ROADMAP.md` and nowhere else.**
- **No round tags, publishes or creates a release.** `DEC-056`'s four legs
  are the architect's, on the owner's approval at **exit criterion 7**.
