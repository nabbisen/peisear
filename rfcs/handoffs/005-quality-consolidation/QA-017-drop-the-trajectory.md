# QA-017 — Keep the aggregate, drop the trajectory

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P2 — `NFR-PRIV-007` is a **SHOULD**; this is a deliberate design
change, not a defect fix
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §7
**Depends on**: `QA-016`, closed. Its audit is the input; do not re-derive it.

**Owner-approved.** Of three options — suppress the charts, leave them, or keep
the aggregate and drop the trajectory — the owner chose the third.

---

## 1. What was decided, and the reasoning it rests on

Below two distinct contributors:

- **The sprint burndown** does not render. The sprint-end totals stay.
- **The velocity chart** renders its bars without the **median reference
  line**.

The reasoning, from `QA-016`: the *totals* are already assemblable elsewhere;
what is not is the **day-by-day trajectory** and the **computed median**. The
requirement is about what is disclosed, not about whether a component appears.

**The predicate is distinct contributors, not team size**, and the difference
is the point. In a genuinely one-person team the only viewer of the burndown is
its subject, and hiding it protects nobody. The case that engages
`NFR-PRIV-007` is a five-person team where one person did all of a sprint's
work and their daily pattern is shown to the other four — including anyone
holding `viewer`, which `QA-016` confirmed sees both charts.

## 2. §1 is nearly free on the burndown, and that is not an accident

`sprints.rs:528` already reads:

```rust
let burndown_card = (!matches!(sprint_status, SprintStatus::Planned) && !burndown.is_empty())
    .then(|| render_burndown(burndown));
let summary_card = render_summary_card(sprint_status, summary);
```

The card is **already conditional**, and `summary_card` is **already
independent of it**. So "keep the aggregate, drop the trajectory" is a third
term in an existing condition, not a new layout. Confirm that before writing
anything — if the totals turn out to depend on the burndown card, this handoff
is aimed wrong.

## 3. The contributor count — new work, and three questions inside it

`BurndownIssueRow` is `{ id, effort, status, assigned_at, updated_at }`; the
query selects no `assignee_id`. `SprintSummary` is all sums and counts. Neither
path can express this predicate today.

**Shape**: I lean to a separate small query — `distinct_contributors(sprint_id)`
— rather than widening `burndown()`'s return type, because velocity needs the
same count over a *set* of sprints and a reusable function serves both. Say
which you chose and why.

Three questions the implementation has to answer, and none has an obvious
answer:

**3.1 — Contributor to what?** Every issue in the sprint, or only completed
ones? The disclosure is about who *did the work*, which argues for completed.
But a sprint where Alice completed everything and Bob has one open issue is
still a sprint whose trajectory is Alice's. Pick one, state it, and say what
the other would have changed.

**3.2 — Unassigned issues.** An issue with no assignee has no contributor. A
sprint with one issue assigned to Alice and four unassigned has one *known*
contributor and an unknown number of real ones. **Counting distinct assignees
treats unknown as solo.**

For a privacy requirement the safe direction is to suppress when the answer is
unknown — but that means a sprint with mostly unassigned work loses its
burndown, which may be common and may be wrong. **Report what the data looks
like** before choosing: how do real sprints in this schema distribute
assignment? If you cannot tell, say so and take the safe direction.

**3.3 — Velocity's window.** The median spans up to five sprints
(`VELOCITY_MEDIAN_WINDOW`). The predicate is **distinct contributors across the
whole window**, not per sprint: five solo sprints by the same person produce a
median about that person, while five solo sprints by five different people
produce a genuine team aggregate. Get this one right; the per-sprint reading is
the tempting wrong answer.

## 4. The copy trap — read this twice

The reflex in this project is to explain everything. **Here, explaining
discloses the thing being protected.**

*"Burndown hidden because only one person contributed to this sprint"* tells
every viewer exactly what the suppression exists to withhold. So does any
wording that lets a reader infer the predicate from the absence.

**Render nothing.** No placeholder, no tooltip, no explanatory note. A sprint
without a burndown already looks like a sprint that is Planned or has no data —
`sprints.rs:528`'s existing condition produces exactly that today, and the
absence is unremarkable.

The same applies to the median line: it goes, without a legend entry saying it
went.

**If you find yourself adding a `MessageKey` for this, stop and report.** That
is the signal the design has drifted into disclosing by explanation. I would
rather hear that the silence is unacceptable for some reason I have not seen
than have it quietly explained away.

## 5. Tests

| # | Check |
|---|---|
| 1 | Two contributors: burndown renders, median line renders |
| 2 | One contributor: burndown absent, **summary totals still present** |
| 3 | One contributor: velocity bars present, median line absent |
| 4 | One contributor: the page contains **no text explaining the absence** |
| 5 | `viewer` role: same behaviour as member (the audience `QA-016` found) |

**Test 4 is the one that matters most and is easiest to skip.** It is the guard
against §4 being undone later by someone adding a helpful note.

Each demonstrated against a plant — for 2 and 3, remove the predicate and watch
the chart come back.

## 6. Not in scope

- **No change to the workload chip.** Settled twice; `QA-016` §1 confirmed it.
- **No suppression of the summary card, the issues table, or anything else.**
  Totals stay, by design.
- **No change to `NFR-PRIV-007`.** It stands as a `SHOULD` at P2.
- **No exposure of `issue_events`.** It stays out of `peisear-web`.

## 7. Escalate rather than deciding

- If §2's structure is not as described, stop.
- **If §3.2's data check shows most sprints carry mostly unassigned issues**,
  stop and report before choosing a direction — that changes whether this
  suppression fires on the common case rather than the rare one.
- If dropping the median line leaves the velocity chart visually broken — an
  axis scaled to it, a legend that no longer balances — report rather than
  reworking the chart.
- If §4's silence turns out to be impossible for an accessibility reason (a
  live region announcing a change, an `aria-label` naming what is not there),
  **that outranks the privacy framing** — report it and stop.

## 8. Acceptance

1. §2's existing structure confirmed.
2. Contributor count implemented; §3.1, §3.2 and §3.3 each answered explicitly
   with what the alternative would have changed.
3. Both suppressions in place, keyed on distinct contributors.
4. Five tests, each planted.
5. **No new `MessageKey`, no explanatory copy anywhere.**
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 9. Required review-request format

Workflow §9.2. §3's three answers as prose. §3.2's data check reported as what
you actually found, including "could not tell".
