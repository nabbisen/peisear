# `CLR-003` — The guard that reads one family

**Round 3 of series `064`, and the last.** `CLR-001` and `CLR-002` are
**accepted**: thirteen of the fifteen failing sites are repaired, every
figure re-measured against the background and the ambient opacity each site
actually renders with. Read `README.md` in this directory first.

**Two corrections the rounds made to my own handoffs, which this round
inherits rather than repeats:**

- **A ratio belongs to a site, not to a token.** Three of `CLR-001`'s seven
  sat on `base-200` (the page body, `layout.rs:72`), not on a card, so their
  *before* figures were **2.50:1**, worse than the 3.08:1 I wrote. And
  `CLR-002`'s anchors carry no colour of their own, so after the repair they
  inherit an ancestor's `/70` and read **6.42:1**, not the 17.21:1 I
  predicted. **Both the background and the ambient opacity are part of the
  measurement.**
- **This environment cannot produce `:hover`.** Its headless browser reports
  `(hover: hover): false` and `(pointer: none): true` — verified
  independently by the architect through this repository's own `cdp.mjs` —
  and daisyUI gates every hover rule behind `@media (hover: hover)`. The
  method that works: read the declaration from the pinned sheet and, where
  it changes a paintable value, apply that exact value inline and screenshot
  the composite.

---

## 1. The last two sites

| site | what it is | class today | background | ratio |
|---|---|---|---|---|
| `confirmation.rs:56` | the delete interstitial's own heading | `text-lg font-semibold text-error` | `base-100` card | **3.08:1** |
| `projects.rs:286` | the *Delete project* heading in the danger card | `font-medium text-error` (16 px) | `base-100` card | **3.08:1** |

Neither reaches the large-text route: 18 px semibold is below WCAG's 18.66 px
bold threshold, and 16 px medium is not close.

**Recommendation: drop the colour on both**, as the previous two rounds did.
Both headings sit inside a card that already carries
`border border-error/30`, and `confirmation.rs`'s own solid `btn-error`
confirm (**6.83:1**) remains the loud element of that page — the warning is
structural, not tonal. **The alternative is `confirmation.rs:56` at
`text-xl font-bold`**, which reaches 20 px bold, qualifies as large text and
would pass at 3.08:1; measure it and say how it reads next to the
consequence line before rejecting it, because a bigger heading there is not
absurd the way a 24 px status sentence was.

**`projects.rs:286` keeps its size** — `TYPE-003` set it, and its comment
records the measurement. Colour only.

---

## 2. The guard — this is the round's substance

`contrast_scan` bans `text-base-content/{10..60}` and sees nothing else.
**Extend it so a semantic colour cannot reach a text position again.**

**The rule** (stated as the ban this project's own precedent prefers, not as
a per-site computation):

- Refuse, in `src/components/**`: `text-{success,error,warning,info,primary,secondary,accent}`
  including any `/N` variant; the same set spelled `link-*`; and the pair
  `btn-outline` **with** `btn-{that set}` on one element — the spelling
  `CLR-001` found that a `text-*` grep misses.
- **Three named exclusions, each with its measured reason in the code**, the
  `alignment_scan` precedent (exclude by exact literal, so a second
  occurrence anywhere else is still caught): `error_page.rs:18`'s 48 px
  status number (large text, 3.08:1 against the 3:1 threshold), and the two
  decorative brand bullets `auth.rs:15` / `layout.rs:211` (a glyph, not
  text; 4.21:1 against 1.4.11's 3:1).
- **Record the measured ratios in the module doc**, so a theme change has a
  baseline to re-measure rather than a rule to re-derive: `success`
  3.03/2.47/1.98, `error` 3.08/2.50/2.01, `primary` 4.21/3.42/2.75,
  `base-content` 17.21/14.01/11.25 on `base-100`/`base-200`/`base-300`, plus
  the two solid pairs that pass (`error-content` on `error` 6.83:1,
  `success-content` on `success` 6.93:1) and **`btn-primary`'s own 4.83:1 —
  a pass by 0.33 across 37 sites**, which is the figure most worth watching.
- **Plant it.** Add the ban, watch it fail on one of the sites `CLR-001`
  repaired (restore the class in a scratch copy, never on `main`), then
  remove the plant. A guard that has never failed on the defect it exists
  for is a claim, not a check.

**Do not** make the scan compute per-site ratios. Its hard half — *which
background is this element actually on* — is the limitation
`NFR-A11Y-005`'s own entry already records for the `/70` floor, and
`CLR-001` demonstrated that only the rendered tree answers it.

---

## 3. The amendment, and the entry

**`NFR-A11Y-005` is amended** — it is recorded `Met` with an acceptance that
reads one token family out of four:

- its *Acceptance* line names `contrast_scan`'s **new** reach as well as the
  `/70` floor;
- the sentence *"The theme's own tokens were never the problem"* is
  **corrected rather than deleted**: true of `base-content`, false of
  `success`, `error` and `primary`, and the measured figures say so;
- the status stays **Met** once this round lands, and says what makes it
  Met — thirteen repairs, three measured exclusions, and a guard that sees
  all four families.

**Open and close `§10.61`** in this round (the next free number — `§10.60`
is the last): *a guard measured one token family and the requirement
generalised to all of them.* It should record, because these are the parts a
later reader will need: that `QA-012`/`QA-013` measured `base-content`
honestly and the entry generalised past its own evidence; that the fifteen
sites included **every destructive action in the product**; that three sites
were worse than the headline because of the background they sit on; that
`btn-primary` passes by 0.33; and **that the class was found by scoping a
release rather than by any check** — the same way `§10.60` was.

---

## 4. The series' closing, which this round also owes

1. **The failing population, re-derived**: it should read **zero**. State
   the command and the count, per family, and name the three exclusions.
2. **A reconciled rounds table** in this directory's `README.md`:
   `CLR-001` (7, accepted), `CLR-002` (6, accepted), `CLR-003` (2 + the
   guard + the amendment + the entry).
3. **What the series leaves behind, named rather than implied**: the nine
   `link link-hover` sites (an affordance question, a recorded candidate);
   the fifteen bare-`opacity` sites and the compounding fade `CLR-002` found
   on the read-notification card — **add that its class is built by
   `format!`, which `css_scan`'s own doc records as a blind spot**, so that
   site is invisible to two scans rather than one; the `xs` control tier;
   and the 21 remaining smallest-text sites.
4. **No rule beyond the ban.** If this round believes the theme itself
   should change, it says so as a finding and stops: changing `--er` would
   repair nothing that is left and would put 37 `btn-primary` sites and
   every solid badge at risk.

---

## 5. Acceptance

1. Both headings accounted for, before/after measured **as rendered**, with
   the background named.
2. The guard added, **planted and watched failing**, then the plant removed;
   its exclusions named by exact literal with their reasons.
3. The module doc carrying the measured ratios.
4. `NFR-A11Y-005` amended; `§10.61` opened and closed.
5. The failing population re-derived to **zero**, per family.
6. `css_scan`'s population re-measured — dropping the last two `text-error`
   sites leaves that token at `error_page.rs:18` alone, so the count should
   hold at `(287, 3)`; **verify, do not assume**, and if it moves, the
   reason goes in the assertion's comment.
7. The full gate line per `README.md §5`, including the `DEC-007` count
   (**687** as it stands) with real exit codes, and **`contrast_scan` run
   after the ban is added, not before**.
8. `CHANGELOG.md` under `[Unreleased] → Fixed`, completing the entry family
   `CLR-001` and `CLR-002` started, plus one line for the guard.
9. Privacy floor per artefact; scratch under `.git-exclude/tmp/`.
10. **Review request** at
    `.git-exclude/review-request/CLR-003-the-guard-that-reads-one-family/README.md`.

**If a measurement contradicts this file, the measurement wins.** In this
series alone, two of my handoffs' own figures were wrong and the rounds
caught both.
