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
source, `text-xs` appears at **126 sites across `components/`** — and
reading the sites rather than counting them shows what it carries:
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
| TYPE-001 | [below the floor there is no floor](./TYPE-001-below-the-floor-there-is-no-floor.md) | The six sub-12 px sites — the smallest population, the clearest question, and the four whose contrast nothing has ever measured | — | **Open, handed 2026-11-17** |
| TYPE-002 | — | `/today`'s own 23 `text-xs` sites, decided by class rather than site-by-site, **with the height delta measured at 390 and 1280** | TYPE-001 | Not written — waits on `TYPE-001`'s findings |
| TYPE-003 | — | The project screens' 35 (`issues.rs` 31, `projects.rs` 4), where the form labels, the display name and the table cells live | TYPE-002 | Not written — waits on `TYPE-002`'s findings |

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
- **The remaining 68 `text-xs` sites** on settings, teams, sprints,
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
is **126** — wrong by a third, and wrong in the direction that would
have made the work look smaller than it is.

Read each handoff's own §1 before starting, and **where it gives a
population, re-derive it rather than inheriting it**. This series'
predecessor was corrected five times by rounds that did exactly that.
