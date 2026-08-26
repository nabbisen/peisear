# RFC 0008: Explainability — the basis behind an indicator

**Status**: **Accepted 2026-08-27**
**Target**: 0.29.0
**Related spec sections**: `SPEC §28.3` (indicator basis), `SPEC §31.1`
(chart equivalence), `SPEC §41.3` (Definition of Done)
**Related requirements**: `FR-HLT-007` (P2), `NFR-A11Y-003` (P1); collides
with `NFR-PRIV-002` and `NFR-PRIV-007` — see §2 and §4
**Governing decisions**: `DEC-019` (management, not oversight)
**Closes**: baseline `§10.4`, and **Definition of Done item 3**
**Last updated**: 2026-08-27 — reconciled against the code before drafting

## Summary

Health indicators explain themselves in a sentence and offer no way to check
the sentence. Charts carry a label and no equivalent. This RFC adds the route
from an indicator to what produced it, and the tabular equivalent for the two
charts.

**Two of its four parts collide with privacy requirements**, and the collisions
are the substance of the design rather than an obstacle to it.

## Background

`§10.4` has been open since 0.19.1 and named `FR-HLT-007` and `NFR-A11Y-003`.
It was targeted at 0.25.0 and slipped three times. **Definition of Done item 3
cannot be met while it is open**, which is the reason it is now scheduled
rather than deferred a fourth time.

### What already exists — reconciled 2026-08-27

**`NFR-A11Y-003` reads *"Not implemented"* and is partially satisfied.** It
requires three things per chart; the first exists for both:

| | Burndown | Completed-work bars |
|---|---|---|
| One-sentence summary label | **yes**, dynamic — range and max | **weak** — *"Bar chart of recent sprint outcomes"* |
| Two-to-three sentence textual summary | no | no |
| Tabular equivalent | no | no |

The bar chart's label names **what kind of chart it is**, not what it shows. It
is an accessible *name*, and the requirement asks for a *summary*.

**A caption is not a summary either.** The completed-work chart carries prose —
*"Each pair of bars: completed (filled) and carried over (light)"* — which
explains the **encoding**. `NFR-A11Y-003` wants the **finding**: what the data
says, not how to read it. That distinction governs §5.

**`FR-HLT-007` is genuinely absent.** Six indicators, six explanation
sentences, no route from any of them to anything.

## Design

### 1. The basis route, per indicator

The project issue list already accepts `view`, `status`, `assignee` and `sort`
as query parameters, so a basis link is a URL for some indicators and new work
for others:

| Indicator | Basis | Reachable today |
|---|---|---|
| Throughput | issues reaching Done vs all | **partly** — `?status=done` gives the numerator |
| Staleness | oldest in-flight issue | **probably** — status filter plus a sort |
| Bus factor | distribution across assignees | **partly** — `?assignee=` per person, but the basis is a distribution, not a list |
| Activity | issues created or finished in 14 days | **no** — needs a date-window filter |
| Long-stale | in-flight issues untouched 14 days | **no** — needs a staleness filter |
| WIP compliance | which assignees are over their limit | **must not** — see §2 |

**Two new filters, not six.** Adding `activity_since` and `stale_for` to the
existing query shape covers the two unreachable rows and nothing else needs
inventing.

**A basis link goes on the explanation row, not the chip.** The chip is a
status; the sentence is the claim; the link belongs to the claim.

### 2. `FR-HLT-007` collides with `NFR-PRIV-002`, and privacy wins

`FR-HLT-007` says **each** indicator MUST offer a route to its basis. WIP
compliance's basis is **which assignees are over their WIP limit** — and a WIP
limit is named in `NFR-PRIV-001`'s inventory as *"visible only to its
subject"*.

The indicator's own sentence is already the aggregate form —
*"{count} active assignees are over their WIP limit"* — a count, deliberately
not names. **A "what this is based on" route would have to name them.**

**Resolution: WIP compliance offers no basis link, and `FR-HLT-007` is amended
to say so.** A blanket MUST that cannot be satisfied for one of its six cases is
a requirement that will be quietly not-met; better to carve the exception, name
the reason, and have the exception be checkable.

**What it offers instead**: a route to the *calculation* (§3) — the threshold
and how the count is derived — without the membership. A reader learns what the
indicator means without learning who.

### 3. The calculation, shown

Each indicator gets its thresholds and derivation stated: what counts as
Good/Watch/Concern, over what window, and what the current inputs were.

**This is the part with no privacy question and the highest ratio of value to
risk**, and it is what `SPEC §28.3`'s *"not only a tooltip"* is chiefly about.
`FR-HLT-009` already separates computation from presentation, so the numbers
exist; they are simply never shown.

**No new copy inventing an explanation.** The thresholds are in
`peisear-core`'s classify functions; the text states them, it does not
paraphrase them.

### 4. Recent history inherits `NFR-PRIV-007`'s predicate

`FR-HLT-007` also asks for *"recent history"*. **An indicator's history is a
time series, and for a project with one active contributor an indicator's
history is that person's history.**

That is precisely what `QA-016`/`QA-017` established for the sprint burndown
and suppressed at 0.28.0. Building indicator history without the same predicate
would reintroduce, on the project screen, what was just removed from the sprint
screen.

**Two options, and this RFC does not choose:**

- **(a) Defer history entirely.** `FR-HLT-007`'s other two limbs — basis and
  calculation — close `§10.4`'s substance, and history is the limb with a
  privacy predicate attached and the least demonstrated demand.
- **(b) Build it with `distinct_contributors < 2` suppression**, reusing
  `QA-017`'s function directly.

**(a) is my recommendation**, on the grounds that the other two limbs are worth
shipping sooner and history is the one that would need its own audit.
**Open question 1.**

### 5. Chart equivalence — the table, and a summary that says something

Per chart:

1. **The label stays**, and the bar chart's is rewritten to describe its data
   rather than its type.
2. **A two-to-three sentence textual summary** stating the **finding** — what
   the data shows — not the encoding. The existing caption stays; it does a
   different job.
3. **A tabular equivalent**: a real `<table>` of the plotted values, in a
   `<details>` beside the chart.

**The table is the part that must not be faked.** A chart's data is already in
hand at render time — `Vec<BurndownPoint>`, `Vec<(Sprint, SprintSummary)>` — so
the table is a second rendering of the same values, not a reconstruction.

**The burndown's table and summary inherit §4's suppression.** When the
trajectory is hidden below two contributors, its tabular equivalent is hidden
with it — a table of the same numbers would disclose exactly what suppressing
the chart withholds. **This is easy to get wrong and it is the first thing to
check in review.**

## Test plan

- A basis link per indicator that has one; its target actually filters to the
  claimed set.
- **WIP compliance renders no basis link** — asserted, not assumed.
- The two new filters return the sets their indicators name.
- Each chart renders a `<table>` whose values equal the plotted series.
- **Below two contributors, the burndown's table is absent along with the
  chart** — planted against the predicate's removal.
- The bar chart's label describes data, not chart type.

## Open questions — how the acceptance was read

*The owner accepted this RFC on 2026-08-27 without answering the three
questions individually. Recorded here is how the architect is reading that,
so a wrong reading is visible and correctable rather than silent.*

1. **History: deferred.** The RFC recommended (a) and acceptance is read as
   taking the recommendation. `FR-HLT-007`'s history limb does not ship in
   0.29.0; basis and calculation do. **If history was meant to be in scope,
   say so and it returns as its own handoff with `QA-017`'s predicate.**
2. **Calculation view: thresholds, not inputs.** No recommendation was given,
   so the architect decides: the current inputs are already on the page as the
   explanation sentence's own numbers — *"Throughput is 0 / 1 (0%)"* — and
   repeating them under a disclosure would be the same fact twice, which is
   the shape `§9.5` was just adopted to stop. **Thresholds and derivation
   only.**
3. **The `FR-HLT-007` amendment: this acceptance is the sign-off.** §2 narrows
   a MUST in a source requirement, which is the owner's to approve. Accepting
   the RFC that proposes the narrowing is being read as approving it. **If the
   amendment was not intended, it is one sentence to withdraw and the WIP
   indicator's exception goes with it.**

## Original open questions

1. **History: defer (a) or build with suppression (b)?** §4. My recommendation
   is (a).
2. **Does the calculation view show current inputs, or only thresholds?**
   Inputs are already on the page as the explanation sentence's numbers, so
   thresholds alone may be enough.
3. **Does `FR-HLT-007`'s amendment need the owner's sign-off as a requirement
   change?** §2 narrows a MUST. I think yes.

## Out of scope

- **No headline score.** `FR-HLT-008` forbids it and this RFC does not revisit
  it.
- **No new chart.**
- **No per-user drill-down anywhere.** `NFR-PRIV-002` governs, and §2 is the
  precedent within this RFC.
- **No browser-based verification.** `§10.15` stands; everything here is
  server-rendered and testable over HTTP.

## References

- Baseline `§10.4`, `§9.5` (what an acceptance clause must name)
- `QA-016`, `QA-017` — the aggregate-inferability predicate this RFC inherits
- RFC 005 §7 — the audit that produced it
