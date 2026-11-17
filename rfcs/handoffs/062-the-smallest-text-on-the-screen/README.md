# Handoffs — the smallest text on the screen

**`0.64.0`, authorized by the owner 2026-11-17** — the row, not the cut.
The **typography** half of the owner's own question of 2026-11-04 —
*could layout, typography and colour be kinder to elder people* — on the
condition the architect attached to it the same day:

> **Rule after design.** Adopt the floor after the views are decided,
> never before.

That condition has been met since unit three closed: `0.62.0` decided
two views. **Colour is not reopened** — `NFR-A11Y-005` is Met, `/70` is
the floor, `contrast_scan` guards the modifier range.

**This release is a read, not a rule.** Three rounds look at the real
screens, one population at a time, and the type requirement — if one
gets written at all — is decided **after** the third round on what the
three found. See *What this series does not promise*, below, and read it
before you start: a round that quietly adopts a floor has done the wrong
work, however good the floor.

---

## How work lands — read this before starting a round

**Commit and push to the shared `main`.** No topic branch and no pull
request: the owner coordinates the schedule so that no two workers are
ever on the same tree at once, so a branch and its merge step add a step
and remove nothing.

**Whoever pushes watches `main`'s own CI to completion.** A red `main` is
that person's only work until it is green again — fix forward or revert,
never leave it for someone else to find.

Exceptions exist and are narrow: parallel work the owner has
deliberately scheduled, a patch to an already-released version, a change
that cannot be made in steps that each leave the trunk working,
throwaway experiments, and any branch that is never pushed. A
contribution from outside the team is the public workflow, not this one.

---

## Why this is one release

**Two things a census of `text-xs` cannot see, and both are why this is
a read rather than a rule.**

**(1) 12 px is not reserved for metadata.** Measured 2026-11-17 at
source, `text-xs` appears at **120 render sites across `components/`**
— and reading the sites rather than counting them shows what it
carries:
every section heading (six of them `uppercase tracking-wide`, which is
harder again), every chip label that names a number, the form field
labels **Status**, **Assignee** and **Sort by**, **a person's display
name**, and **table cells** holding effort and assignee. The numbers on
these screens are large. The words that make them mean anything are the
smallest thing on the page.

**(2) There is text below 12 px, and that census cannot see it at all.**
**Six sites** — four at **10 px** and two at **11 px** — because they
are written as arbitrary values, not as a scale class. Four of the six
are in `calendar.rs`, and **all four use bare `opacity-70` rather than
`text-base-content/70`**. `NFR-A11Y-005`'s own entry states both limits
of its guard in terms: it does not cover bare `opacity` at all, and it
does not know which background a passing modifier sits over. **Two of
those four sit on `bg-primary/10` and `bg-primary/15` tints.** So the
smallest text in the product is also the text whose contrast nothing
checks — and the one previous hand check of that class found a site at
**2.64:1**, worse than a tier the rule bans outright.

**Nothing in either specification governs typography.** No scale, no
floor, no minimum: `font-size`, *typography*, *text size* and *type
scale* appear nowhere in `requirements.md` or `external-design.md`.
That is not an oversight to correct in passing — it is the same shape as
the ten ungoverned `<details>`, and it is part of the finding. It is
also why `§10.60` is opened **open** rather than written as a repair.

## Rounds

Each round is blocked on the one before it: `TYPE-002` reads screens
with whatever `TYPE-001` learned about measuring them, and `TYPE-003`
with both. **Do not start a later round early.**

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| TYPE-001 | [below the floor there is no floor](./TYPE-001-below-the-floor-there-is-no-floor.md) | The six sub-12 px sites — the smallest population, the clearest question, and the four whose contrast nothing has ever measured | — | **Closed 2026-11-17, accepted with one blocking correction (round 2).** All four previously-unmeasured ratios now have numbers and **all four clear 4.5:1** — `5.51`/`5.91`/`4.98`/`5.86` resting, measured as rendered against both tinted and untinted backgrounds, with hover reached by a real dispatched mouse move and a sampling rule that is a conservative floor rather than a point estimate. Five of six sites raised to the scale's own 12 px floor, each cost measured; one kept at 10 px **on a reason that is not about type** — its third line is clipped by the block's `overflow-hidden` under roughly an hour, at either size, which is a defect in its own right now named as found-and-not-fixed. Two removals considered and rejected with reasons. **The correction**: `§10.60` as written said three of four sites moved to the guarded colour spelling and that `calendar.rs:185` kept bare `opacity-70` — **the tree says all four moved**, and the site's own comment says so; `§10.58`'s class two days after it shipped. The code is right; the record was wrong, corrected in the entry by the architect. **It also found the architect's census method wrong**: every published population figure counted comments mentioning a class alongside sites rendering it. `DEC-007` **687** unmoved, `--lib` **246**, `overflow-gate` 120/120, `occlusion-gate` 120/120 |
| TYPE-001 r2 | [what the tree actually says](./TYPE-001-round-2-what-the-tree-actually-says.md) | Three measurements and one piece of reasoning — the badge's cost in the dimension it actually lives in, the hour label's 48 px clearance, and why a guarded spelling is right where the guard cannot verify | TYPE-001 | **Closed 2026-11-17, accepted. Documentation only, no `.rs` touched.** The badge's cost measured in the dimension it lives in: **at 390 px it paints zero pixels in both views, at both sizes, for all three name lengths** — the badge does not reach a phone at all, which is a `TYPE-003` question about whether the information belongs there. At 1280 the pixel budget barely moves, **and the round named the mechanism rather than just observing it**: the badge is the line's last child, so growing it grows the overflow by the same amount; what a fixed budget loses is *characters*, not pixels. The hour label measured at **30.7 px against a 48 px gap — 17.3 px clearance**, and measured rather than taken from the architect's own ~30 px estimate, which is the right relationship to one. **One correction, made in the entry rather than sent back**: the character figures are budgets that do not subtract the ellipsis glyph, which sits exactly where the badge is cut, so painted glyphs are fewer in both columns and the relative cost is larger than *modest*. **One anomaly reported as confirmed that did not happen**: the push CI run for `5404f45` existed and was created nine seconds after the commit — it was in flight, not absent, and a second run was dispatched on that premise. Harmless, same SHA, both green, and **it stayed out of every published document**. `--lib` **246**, `DEC-007` **687** |
| TYPE-002 | [the first screen after signing in](./TYPE-002-the-first-screen-after-signing-in.md) | `/today`'s own 21 sites, decided by class rather than site-by-site — and the four 44 px controls whose touch-target arithmetic is built on `text-xs`'s line-height | TYPE-001 r2 | **Open, handed 2026-11-17** |
| TYPE-003 | — | The project screens' **36** (`issues.rs` 31, `projects.rs` 5), where the form labels, the display name and the table cells live — and where the calendar badge's phone-width invisibility is read | TYPE-002 | Not written — waits on `TYPE-002`'s findings |

## Removal is on the table in every round

*Less is more* points at **taking away what a reader does not need**
before it points at making it bigger, and a rule cannot decide whether a
piece of information belongs on a screen — only a read of that screen
can. So in every round, for every site, three outcomes are equally
available and each needs its own recorded reason:

1. **Keep it at its current size**, with the reason it is right there.
2. **Raise it**, with what it cost measured, not asserted.
3. **Remove the information**, because the screen reads better without
   it.

A round that returns only outcome 2 has probably not considered the
other two, and will be asked why.

## What this series does not promise

- **The floor itself.** Whether a type requirement is written, and what
  it says, is decided **after** `TYPE-003`. Adopted or deliberately
  deferred — with the reason recorded either way.
- **Colour.** `NFR-A11Y-005` is Met and is **cited, not reopened**.
  `TYPE-001` measures the contrast of four specific sites because
  nothing ever has and they are the subject of its own round; that is a
  measurement inside this release, not a reopening of the requirement.
- **The remaining 65 `text-xs` sites** on settings, teams, sprints,
  notifications, search and auth. They wait for their own screens to be
  read.
- **Any global rule applied before the read.** A `text-xs` ban, a root
  `font-size` change, a scan over arbitrary text values — all of them
  are the rule arriving before the design, which is the thing the
  owner's own condition prohibits.

## Where the numbers in these handoffs come from

Every figure was measured on **2026-11-17** against `0.63.0`'s tree
(`922f976`). **One of them corrects the architect's own earlier note**:
that note of 2026-11-04 said *"ninety sites"*, and the census at source
is **120** — wrong by a third, and wrong in the direction that would
have made the work look smaller than it is.

**A population here is a count of render sites, and the command that
measures one excludes comments.** Corrected 2026-11-17 after
`TYPE-001`: the architect's first figures (126 / 23 / 35 / 68) were a
plain `grep -c`, which counts a comment that *mentions* a class
alongside a site that *renders* it. Six such comments pre-existed, and
`TYPE-001` wrote three more, so the naive census now reads 134 where
125 sites render. The figures above are render sites only:

```bash
grep -rn 'text-xs' crates/ --include='*.rs' | grep -vE ':[[:space:]]*//' | wc -l
```

Writing a class name into a comment is not a fault and nothing should
change about it. **The census method was what was wrong, and it was the
architect's to get right** — a handoff that names a population owes the
command that measures it, and the first version of this one did not.
`text-sm`, measured the same way, is **151**.

Read each handoff's own §1 before starting, and **where it gives a
population, re-derive it with the command above rather than inheriting
the number**. This series' predecessor was corrected five times by
rounds that did exactly that; this series' first round was handed a
number that re-deriving would have reproduced wrongly.
