# `LBL-001` — Three classes for one size

**Round 1 of series `063`** (`0.65.0`, authorized 2026-11-18 — the row, not
the cut). Read `README.md` in this directory first: it carries the census
command, the method, the stylesheet facts and the privacy floor, and this
file does not repeat them.

**Scope: 15 sites.** Both label conventions, product-wide, decided once —
because a label's size is one decision and deciding it per screen is how the
product came to have two.

---

## 1. The 15 sites

**Eight form labels** — `label-text text-xs`:

| site | what it labels | its control's own text size |
|---|---|---|
| `sprint_plan.rs:215` | backlog filter — *Project* | `select-sm` → **14 px** |
| `sprint_plan.rs:230` | backlog filter — *Priority* | `select-sm` → **14 px** |
| `sprint_plan.rs:247` | backlog filter — *Assignee* | `select-sm` → **14 px** |
| `settings.rs:296` | *Close on*, inside a `w-64 card-compact` dropdown | `input-xs` → **12 px** |
| `settings.rs:330` | *Points*, inside a `w-80 card-compact` dropdown | `input-xs` → **12 px** |
| `settings.rs:338` | *From*, same dropdown | `input-xs` → **12 px** |
| `settings.rs:346` | *To*, same dropdown | `input-xs` → **12 px** |
| `settings.rs:354` | *Note*, same dropdown | `input-xs` → **12 px** |

**Seven field hints** — `label-text-alt text-xs`:

| site | the hint | the label beside it, same row |
|---|---|---|
| `auth.rs:92` | password minimum length | `label-text text-sm` → **14 px** |
| `settings.rs:205` | in-progress issues | `label-text text-sm` → **14 px** |
| `sprints.rs:531` | *optional* (sprint goal) | `label-text text-sm` → **14 px** |
| `sprints.rs:1163` | *optional* | `label-text text-sm` → **14 px** |
| `teams.rs:169` | optional, auto-derived | `label-text text-sm` → **14 px** |
| `teams.rs:184` | *optional* | `label-text text-sm` → **14 px** |
| `teams.rs:176` | slug helper text (`text-base-content/70`, not `opacity-70`) | its own row |

Six of the seven carry a bare `opacity-70`; `teams.rs:176` carries
`text-base-content/70`.

---

## 2. What the stylesheets make of that — read before you change anything

`README.md §5` has the table; the two facts that decide this round:

- **`.label-text` already is 14 px/20 px** — identical to `.text-sm`. So on
  those eight sites the `text-xs` token is an **override downward** of a
  component class that already says the larger size. **Deleting `text-xs`
  raises the label**; nothing needs adding.
- **`.label-text-alt` already is 12 px/16 px** — identical to `.text-xs`. So
  on those seven the `text-xs` token is **redundant**, and the five sites
  `0.64.0` raised were raised by **adding** `text-sm` to override the
  component class upward.

`tailwind.css` is served last, so at equal specificity a `text-*` token wins
(`layout.rs:68–70`, and `layout.rs:34` already depends on that order).
**Derived from the files; confirm the rendered `font-size` in a browser
before and after, per `README.md §5`.**

---

## 3. The finding this round exists for, and why it is not one answer

**The census says one thing and the sites say three.** The same 15-site
population splits by what each site's *neighbours* are:

1. **`sprint_plan.rs`'s three filter labels are smaller than their own
   controls' text** — a 12 px label over a `select-sm` whose options render
   at 14 px. There is no compactness argument here: the label is the only
   12 px thing in its row.
2. **`settings.rs`'s five labels sit in a coherent 12 px cluster** —
   `card-compact` dropdowns whose `input-xs` controls render the user's own
   typed value at 12 px. Raising the label alone would make the label larger
   than the value it labels. **This is a decision about the cluster, not
   about the label.**
3. **The seven hints are the only 12 px thing in a 14 px row** — label at
   14 px, control at 14 px, hint at 12 px, all three in the same `label`
   row. This is exactly the shape `0.64.0` fixed on the five screens it read.

**Say this plainly in the report, because it matters beyond this round:**
this is the third time in two releases that a class census and the right
answer have disagreed. That disagreement is the argument `§10.60` closed on —
**it is not a reason to write a rule, and not permission to reopen it.**

---

## 4. The three questions, with the architect's recommendation on each

**Q1 — the three `sprint_plan.rs` filter labels.** *Recommendation: delete
the `text-xs` token*, which renders them at `.label-text`'s own 14 px.
**Measure**: the filter row is `flex flex-wrap items-end gap-2`, with
`min-w-0` on each label for a defect already fixed at 320–414 px
(`sprint_plan.rs:205–208`'s own comment) — so measure the row at **390 px**
for a new wrap or a changed row height, and at **1280 px** for anything that
moved. Do not remove `min-w-0`.

**Q2 — the five `settings.rs` labels.** *No recommendation on the outcome;
there is one on the order of work.* Three outcomes, all legitimate:

- **raise the cluster** — labels to 14 px *and* the `input-xs` controls to
  `input-sm`; note that changes control height **24 px → 32 px** inside
  fixed-width `w-80` (320 px) and `w-64` (256 px) cards, which is the cost to
  measure;
- **leave the cluster at 12 px**, recorded with the reason — a deliberately
  compact editing surface whose label matches its own control;
- **raise the label only** — measure it, but say how it reads: a label larger
  than the value beneath it.

*Measure the cluster raise first*, because it is the one with a real cost to
find. **These five labels live inside `<details>` dropdowns that are closed
by default**, so the browser gates have never rendered them open: **open them
in a browser and measure.** Do not grow a gate fixture to reach them
(`§10.32`'s own stopping rule, and `README.md §7`).

**Q3 — the seven hints.** *Recommendation: match the five `0.64.0` raised* —
replace `text-xs` with `text-sm`, giving label, hint and control one size per
row. The alternative is live and must be rejected on evidence rather than
ignored: **12 px on all twelve**, deleting the redundant token here and
reverting those five. **Measure** each hint at **390 px and 1280 px** — these
are short strings (*optional*, a minimum length, a slug helper) and the
`0.64.0` sites measured *no wrap at either width*, which is a reason to
expect the same and not a reason to skip measuring. **Measure contrast** for
the six `opacity-70` sites and for `teams.rs:176`'s `text-base-content/70`,
over the background each actually sits on: `contrast_scan` covers neither
case (`README.md §4.3`).

---

## 5. Out of bounds for this round

- **The 54 `label-text text-sm` sites.** Redundant token, no rendered change,
  screens this series does not read. **Do not touch them**, and do not
  "tidy" them while passing. Recorded as a candidate in `ROADMAP.md`.
- **Non-label text on these screens** — that is `SIZE-001`–`SIZE-003`. This
  round changes label and hint sites only, even where a neighbouring
  `text-xs` site is obviously next.
- **Any new rule, floor or scale.** `§10.60` is closed.
- **`sprints.rs:436`/`983`'s comment arithmetic** — `SIZE-001`'s, not this
  round's, and untouched by a label change.

---

## 6. Acceptance

1. **The 15 sites accounted for, one by one**: changed with its measured
   before/after, or unchanged with its reason. A site left at 12 px is a
   finding; a site silently skipped is a defect.
2. **Rendered `font-size` measured in a browser**, not inferred from the
   class name, at least once per distinct change shape.
3. **Costs at 390 px and 1280 px** for each of the three groups, including
   the two fixed-width dropdown cards opened.
4. **Contrast ratios** for the seven modifier-bearing hint sites, each with
   the background it was measured over.
5. **`css_scan` population re-measured, not assumed** — `css_scan/tests.rs`
   asserts `(290, 2)`. Both tokens stay in use elsewhere, so it probably does
   not move; if it does, update it **with the reason**, as its own failure
   message demands.
6. **The `DEC-007` block** per `.github/CONTRIBUTING.md` — 42 commands, real
   exit codes read (`cmd | tail -1; echo $?` reports `tail`'s status). The
   count stands at **687**; say what it became.
7. **Every gate this round's changes can reach**, including the browser
   checks if any markup moved — and if a gate's own source is edited, that
   gate is run.
8. **The privacy floor per artefact** (`README.md §8.7`): inspected before
   attaching, no local paths, hostnames or account names.
9. **The review request** goes to
   `.git-exclude/review-request/LBL-001-three-classes-for-one-size/README.md`.

**If the measurements contradict anything in this file, the measurements
win** — report it as a finding rather than working around it. Three of
`062`'s five rounds corrected the architect, and that is the expected rate.
