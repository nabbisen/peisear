# `PLC-001` — The grey every form asks you to read

**Round 1 of series `065`** (`0.67.0`, authorized 2026-11-23 — the row, not
the cut). Read `README.md` in this directory first: it carries the measured
tokens, the cascade, the method and the privacy floor.

**Scope: the seventeen form placeholders. Nothing else.**

---

## 1. The seventeen

Every one carries `.input` or `.textarea`, which is what makes the repair in
§2 reach it (checked at each site, not assumed from the pattern):

| site | field | element |
|---|---|---|
| `layout.rs:290` | the header's own search box | `input input-bordered input-sm` |
| `search.rs:88` | the search page's query field | `input` |
| `projects.rs:165` | new project — name | `input` |
| `projects.rs:171` | new project — description | `textarea` |
| `settings.rs:186` | capacity — points | `input` |
| `settings.rs:208` | capacity — note | `input` |
| `settings.rs:261` | WIP limit (its placeholder is a **number**, `DEFAULT_WIP_LIMIT`) | `input` |
| `issues.rs:1493` | new issue — title | `input` |
| `issues.rs:1519` | new issue — description | `textarea` |
| `issues.rs:1671` | new sub-issue — title | `input` |
| `issues.rs:1684` | new sub-issue — description | `textarea` |
| `sprints.rs:560` | new sprint — name | `input` |
| `sprints.rs:604` | new sprint — goal | `textarea` |
| `teams.rs:171` | new team — name | `input` |
| `teams.rs:197` | new team — slug | `input` |
| `teams.rs:234` | new team — description | `textarea` |
| `teams.rs:373` | invite — e-mail | `input` |

All seventeen render `#9ca3af`: **2.54:1** on a white card, **2.07:1** on
the page body. AA wants **4.5:1**, and placeholder text is text.

**Two of them matter more than their count suggests.** `layout.rs:290` is in
the header of **every authenticated page**, and `teams.rs:373`'s placeholder
is an example e-mail address — the hint a reader needs most when the field
will reject a malformed value.

---

## 2. The repair, and why it cannot simply go where you would put it

**One rule, in `app.css`**, at the floor this project already chose:

```css
.input::placeholder,
.textarea::placeholder { color: oklch(var(--bc) / 0.7); }
```

**Not** `input::placeholder`. `tailwind.css` is served **last**
(`layout.rs:68–70`) and carries
`input::placeholder,textarea::placeholder{color:#9ca3af;opacity:1}`, so an
element-selector rule in `app.css` **loses on sheet order at equal
specificity**. A class plus pseudo-element out-specifies it and wins
wherever it sits. All seventeen sites carry the class.

**Check the opacity declaration too.** That same tailwind rule sets
`opacity:1` on the placeholder; if your rule changes the colour but
something else re-asserts an opacity, the composite is not what the
declaration says. **Measure, do not reason** — `§10.52`.

**Expected after**: `#5d5f6a`, **6.34:1** on `base-100`. Report what you
actually get, per background, and if it differs say so rather than
reconciling it to this figure.

---

## 3. What to measure

- **Rendered placeholder colour before and after**, read from the painted
  pixels (the method this project has used since `TYPE-001`) or from
  `getComputedStyle` on the pseudo-element — **say which**, and if the two
  disagree, that disagreement is the finding.
- **At least one site per background**: a white card (`projects.rs:165`) and
  the page body or header (`layout.rs:290`), since the ratio belongs to the
  site and not the token — `0.66.0`'s own lesson, learned twice.
- **The search box in the header at 390 px and 1280 px**: it is the one
  placeholder inside a flex row that collapses behind the `Menu` control
  below `md`. Confirm the repair is visible at both and that nothing moved.
- **That `css_scan` still passes.** The new rule adds **no class token to a
  component**, so the population should hold at `(287, 3)` — verify.
- **Whether any placeholder is doing work the label should do.** Removal is
  on the table in this series too: `settings.rs:261`'s placeholder is a bare
  number and `teams.rs:373`'s is an example address — if either reads as the
  *value* rather than a hint, say so as a finding. **Do not change copy in
  this round**; name it.

---

## 4. Out of scope

- **The account menu's email line and the eight table headers** —
  `PLC-002`'s.
- **The guard, the amendment, the entry and `app.css`'s stale header
  comment** — `PLC-003`'s. **Do not fix that comment here**, even though
  you will be editing the file it sits in: one round, one reason, so the
  review line stays readable.
- **The disabled placeholder tiers** (`/40`) — exempt under WCAG 1.4.3,
  named in `PLC-003`.
- Anything in the four recorded candidates (`README.md §5`).

---

## 5. Acceptance

1. **Seventeen sites accounted for** — the rule reaches each one, confirmed
   by rendering, not by the class list.
2. **Before/after ratios per background measured**, with the background
   named; the expected after is **6.34:1**.
3. **The cascade verified in a browser**: the computed placeholder colour is
   the project's rule and not tailwind's `#9ca3af`. **Plant it if you want
   certainty**: comment the rule out in a scratch copy, confirm the colour
   returns to `#9ca3af`, restore. A rule believed is worth less than a rule
   watched winning.
4. **`css_scan` `(287, 3)`** — verify, do not assume.
5. **The gate line in full** per `README.md §6`, including the `DEC-007`
   block (**689** as it stands) with real exit codes, and the browser gates
   — **a served stylesheet changed**, so `overflow-gate.mjs` runs and its
   cell count is reported.
6. **`CHANGELOG.md`** under `[Unreleased] → Fixed`: what a reader could not
   read, in a sentence that does not require knowing what a placeholder is.
7. **Privacy floor per artefact**, and if a screenshot includes
   `teams.rs:373` or the account menu, **it must carry the fixture's
   address, not a real one** — say which address is in the image.
8. **Review request** at
   `.git-exclude/review-request/PLC-001-the-grey-every-form-asks-you-to-read/README.md`.

**If a measurement contradicts this file, the measurement wins.** In the
last series five of the architect's own figures were corrected that way, and
every correction improved the result.
