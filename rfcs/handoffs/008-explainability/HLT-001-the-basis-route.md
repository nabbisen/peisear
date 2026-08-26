# HLT-001 — The basis route (re-issued)

**Issued by**: Architect
**Date**: 2026-08-27 — **re-issued**; the first version's premise was wrong
**Priority**: P2 by `FR-HLT-007`; closes **Definition of Done item 3**
**Governing RFC**: [008](../../accepted/008-explainability.md) §1–§3
**Depends on**: nothing.

---

## 0. What changed, and why you are reading this twice

The first version said the issue list's existing `status`/`assignee`/`sort`
parameters reach most indicators' bases, and two new filters cover the rest.
**Three of the six rows in that table were wrong.** You found it by reproducing
the table before building, which is what §2 asked for and why it asked.

Confirmed here: `status` matches exactly one value, every sort is descending,
bus factor's query never selects the assignee's id, activity is an OR across
two columns, and staleness uses an event-aware clock rather than a column
compare.

**The error was mine** — I read the parameter names and inferred capability
without reading the filter or the indicator's own query.

**The design changed as a result**, and the owner accepted the larger scope
rather than a version that linked only the reachable third. RFC 008 §1 is
rewritten; read it before this.

## 1. The shape

**Do not build filters. Return the set.**

The query that computes an indicator already knows which rows produced it.
`ProjectHealthRaw` carries only counts today —
`long_stale_in_flight_issues`, `top_assignee_in_flight_issues`,
`recent_activity_count`, and the rest. **Each count that has a membership
gains it.**

A basis link goes to a route that renders exactly that set.

**One authority for the membership of a set.** If the health query and the
basis view can ever disagree about which issues are long-stale, the design is
wrong — that is `QA-019`'s `updated_at` lesson in a different place.

## 2. Where to be careful, in the order you will meet it

**2.1 — The membership must come from the same evaluation as the count, not a
second query with the same `WHERE`.** A second query is two homes again, one
`SELECT` apart. If the shape makes that hard — the counts are `SUM(CASE …)`
aggregates over one pass — **stop and report the shape before restructuring
it.** It may need the aggregate to become a fetch plus a fold, and that is a
design choice I want to see rather than assume.

**2.2 — Some indicators have no membership, and that is not a gap.**
Bus factor's basis is a *distribution*; "the most-loaded assignee's issues" is a
plausible basis but not the same thing as the number the indicator shows.
**Decide what bus factor's basis is, state it, and say what you rejected.**
Throughput's basis is two sets, not one.

**2.3 — WIP compliance returns no set, structurally.** Its basis is users, not
issues. **Do not add a users-shaped basis to make it symmetric.** RFC 008 §2
carved this on `NFR-PRIV-002` grounds and the owner approved it; the design now
makes it fall out rather than be carved, which is better and easier to undo by
accident.

**2.4 — The route renders issues the viewer can already see.** It is a
different view of the project's own issues, so it inherits the project's access
check and adds nothing. **Confirm that rather than assume it** — if the basis
set can contain an issue the viewer cannot otherwise reach, stop.

## 3. The test written first

**WIP compliance renders no basis link, and no assignee name appears in its
explanation area.** This is the one test here guarding a privacy boundary
rather than a feature. Write it before anything else, and plant it against a
link being added.

## 4. The calculation (§4 of the first version, unchanged)

Each indicator gets its **thresholds and derivation** — Good/Watch/Concern
boundaries and the window. **Thresholds only, not current inputs**: the inputs
are already on the page as the explanation sentence's numbers.

**Take them from `peisear-core`'s classify functions.** If a threshold cannot
be reached from the render site without retyping it, **stop and report** —
that is the same two-homes finding as everything above, and I would rather hear
it than have a constant duplicated quietly.

Your own round-1 §5 held this section pending the basis design. It is settled
now, so this can proceed alongside.

## 5. Not in scope

- **No history.** `FR-HLT-007`'s third limb needs `QA-017`'s contributor
  predicate. If you find yourself building a time series, stop.
- **No new filters on the issue list.** That was the wrong repair; see §0.
- **No per-user drill-down**, anywhere. §2.3 is the precedent inside this
  handoff.

## 6. Escalate rather than deciding

- **If the membership cannot come from the same evaluation as the count**
  (§2.1), stop and report the shape.
- **If a threshold cannot be read from `peisear-core`** (§4), stop.
- **If any basis set can contain an issue the viewer cannot otherwise reach**
  (§2.4), stop — that is a live privacy finding, not a design question.
- If bus factor's basis has no defensible definition, say so and leave it
  without a link rather than inventing one.

## 7. Acceptance

1. `ProjectHealthRaw` carries membership for each count that has one, from the
   same evaluation as the count.
2. A basis route rendering exactly that set, inheriting the project's access
   check.
3. Links on the indicators that have a membership; **§2.2's decisions stated**.
4. **WIP compliance renders none — test written first, planted.**
5. Thresholds from `peisear-core`, not retyped.
6. No time series, no new issue-list filters.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. §2.1's shape and §2.2's decisions as prose. Say plainly whether
§6's third case was found.
