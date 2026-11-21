# 064 — The colours the guard cannot see

**Series index.** `0.66.0`'s scope, authorized by the owner 2026-11-21 —
**the row, not the cut**. Opened the same day.

**A P1 requirement recorded `Met` is not met.** `NFR-A11Y-005` says *"Text
and background combinations MUST meet WCAG AA (4.5:1)"*, and **fifteen
shipped sites render text below it** — every destructive action in the
product among them. The reason is structural: that requirement's acceptance
is `contrast_scan`, which reads **one** token family
(`text-base-content/{10..60}`), and the entry generalised from it in so many
words — *"the theme's own tokens were never the problem"*.

---

## 1. What was measured, and how

The product serves **one** theme — `data-theme="corporate"`, hardcoded at
`layout.rs:63` — so these are the product's own numbers, not one
configuration among many. Tokens resolved from the pinned
`static/daisyui.min.css` (`ASSET-001`), converted OKLCh → sRGB → relative
luminance, **and the conversion checked against daisyUI's own published
hexes** before any of it was used:

| foreground | hex | `base-100` | `base-200` | `base-300` |
|---|---|---|---|---|
| `success` | `#00a96e` | **3.03:1** | 2.47:1 | 1.98:1 |
| `error` | `#ff5861` | **3.08:1** | 2.50:1 | 2.01:1 |
| `primary` | `#4d6eff` | **4.21:1** | 3.42:1 | 2.75:1 |
| `base-content` | `#181a2a` | 17.21:1 | 14.01:1 | 11.25:1 |

**AA: 4.5:1 for normal text; 3:1 for large text** (≥24 px, or ≥18.66 px
bold). Solid variants pass properly — `error-content` on `error`
**6.83:1**, `success-content` on `success` **6.93:1** — which is why the
repairs in §3 use patterns the product already has.

**Re-derive these yourself before relying on them.** The command that
matters is not a grep: it is resolving the token and computing the ratio.
If a figure here disagrees with your own measurement, **yours wins** and the
disagreement is the finding.

---

## 2. The fifteen failing sites

| round | family | sites | ratio |
|---|---|---|---|
| `CLR-001` | destructive affordances — `btn-ghost … text-error` ×6 (`layout.rs:417`, `teams.rs:619`, `settings.rs:469`, `issues.rs:2256`, `sprints.rs:719`, `sprints.rs:747`) and `btn-error btn-outline` ×1 (`projects.rs:295`) | **7** | 3.08:1 |
| `CLR-002` | `link-primary` ×5 (`auth.rs:72`, `auth.rs:146`, `notifications.rs:172`, `notifications.rs:257`, `issues.rs:463`) and `text-xs text-success` ×1 (`notification_preferences.rs:67`) | **6** | 4.21:1 / 3.03:1 |
| `CLR-003` | `text-error` headings ×2 (`projects.rs:286`, `confirmation.rs:56`), plus the guard, the amendment and the register entry | **2** | 3.08:1 |

**Three sites use these colours and pass. Do not "fix" them**:
`error_page.rs:18` (48 px status number — large text, 3:1), and the two
`text-primary` brand bullets (`auth.rs:15`, `layout.rs:211` — decorative
glyphs, not text). If a round changes one of these, it has broken something
that was correct.

**Two narrow passes, recorded so a theme change has something to
re-measure against**: `btn-primary`, the product's main action at **37
sites**, measures **4.83:1** — a pass by 0.33, on exactly the reasoning
`NFR-A11Y-005` used to choose a `/70` floor over `/60`. And
`confirmation.rs:64`'s solid `btn-error` is **6.83:1**: the pattern the
repairs borrow.

**Rounds closed so far.** `CLR-001` **accepted** 2026-11-21 (`bf02b24`):
seven destructive affordances repaired, and it **corrected this series' own
headline table** — three of the seven sit on `base-200` (the page body), not
on a card, so their *before* figures were **2.50:1**, worse than the 3.08:1
the handoff asserted. It also found that **this environment cannot produce
`:hover`** (`(hover: hover): false`, verified independently by the
architect), so daisyUI's `@media (hover: hover)`-gated rules cannot be
triggered here by any technique; the working method is to read the
declaration from the pinned sheet and, where it changes a paintable value,
apply that value inline and screenshot the composite. `CLR-002`
**accepted** 2026-11-21 (`56ed15b`): six sites repaired, and it corrected
the handoff again — the anchors carry no colour of their own, so after the
repair they inherit an ancestor's `/70` and read **6.42:1**, not the
17.21:1 predicted. **A ratio belongs to a site, not to a token: the
background and the ambient opacity are both part of it.** `css_scan`
population `289 → 287`; `link-primary` and `text-success` have left
`components/` entirely.

**Round 3 is open** —
[`CLR-003-the-guard-that-reads-one-family.md`](./CLR-003-the-guard-that-reads-one-family.md):
the last two headings, **the guard**, `NFR-A11Y-005`'s amendment, `§10.61`,
and the series' closing.

---

## 3. Method

1. **Measure before and after, per site**, in a browser: the rendered
   foreground, the background it actually sits on, and the ratio. A ratio
   computed from the stylesheet is a better claim than a guess and a worse
   one than a measurement of the rendered pixels.
2. **Add no new colour.** Every family has an AA-safe pattern already in the
   product (§2's table and `confirmation.rs:64`). `app.css` is 16 lines with
   two rules and no colours of its own; keep it that way unless a round can
   show nothing existing will do, and say so as a finding first.
3. **A class token must have a rule in one of the three served
   stylesheets** — the CSS is vendored and pinned (`ASSET-001`, `DEC-051`),
   so an invented class resolves to nothing. `css_scan` enforces it.
4. **Removal is on the table.** Dropping a colour is a legitimate repair,
   and on some of these sites it is the recommended one.
5. **A site's meaning is part of its measurement.** One of `CLR-001`'s seven
   is **Sign out**, which is not destructive at all; red there is wrong
   independent of contrast. Where a repair also changes what a colour
   *means*, say so rather than folding it into the ratio.
6. **The guard grows once, in `CLR-003`**, and no round widens it on its own
   initiative.

---

## 4. Deliberately not in this series

- **The 21 remaining smallest-text sites** (`calendar.rs` 11, the two
  notification surfaces 7, `search.rs` 2, `layout.rs` 1) — a type question,
  read as a set by a later row.
- **The `xs` control tier** (**18** `btn-xs`, **2** `select-xs` render
  sites). Two of `CLR-001`'s buttons are `btn-xs`: **their size is not this
  series' business, only their colour.**
- **The fifteen bare-`opacity` sites** and their two recorded conditions.
- **Semantic borders** — `border-error/30`, `border-info/40`,
  `hover:border-primary`. Measured and out of scope: each is card or alert
  decoration, or a hover affordance that also changes the shadow, never the
  sole indicator of a control's boundary, so WCAG 1.4.11's 3:1 is not
  implicated the way a form field's only border would be.
- **Any change to the theme itself.** The tokens are daisyUI's, pinned, and
  shared with every solid variant that currently passes; changing `--er`
  would repair fifteen sites and put 37 `btn-primary` sites and every badge
  at risk. The repairs change how the product *uses* the tokens.

---

## 5. What each round owes

1. **Per site**: the before and after ratio, measured on the rendered page,
   with the background named.
2. **Every site in its own list accounted for** — repaired, or left with a
   reason that survives its neighbours and its meaning.
3. **The three passing sites untouched**, confirmed.
4. **Gates in full**, with real exit codes (`cmd | tail -1; echo $?` reports
   `tail`'s status): the `DEC-007` block (**687** as it stands — say what it
   became), `fmt`, `clippy` both feature variants, `contrast_scan`,
   `touch_target_scan`, `alignment_scan`, `css_scan`'s population
   (`(289, 3)` — re-measure, never assume), and the browser gates when
   markup moves.
5. **`CHANGELOG.md`** under `[Unreleased] → Fixed`, since this is a defect:
   what it was, what a reader could not read, and what changed.
6. **The privacy floor, per artefact** — any screenshot or trace inspected
   before it is attached, no local paths, hostnames or account names. Write
   scratch under `.git-exclude/tmp/`, never `/tmp`, and delete it when the
   round closes.
7. **Review request** at
   `.git-exclude/review-request/<round>-<slug>/README.md`.

---

## 6. Where things go

- **Handoffs** are published here, one file per round. **Reviews stay
  private** and are never published.
- **Release numbers live in `ROADMAP.md` and nowhere else.**
- **No round tags, publishes or creates a release.** `DEC-056`'s four legs
  are the architect's, on the owner's approval at **exit criterion 7**,
  which is a separate decision from the authorization this series was opened
  on.
