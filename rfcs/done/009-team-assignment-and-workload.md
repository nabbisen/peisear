# RFC 0009: Team assignment and workload scope

**Status**: Implemented (0.22.0)

> **Implemented in full by `TEAM-001`.** The split this RFC proposed — ship
> assignment now, hold the workload report on an owner decision — turned out
> to be unnecessary and, as written, impossible: `WorkloadStrip` iterates
> whatever the query returns, so requirement 2 and "do not widen the
> consumers" could not both hold. `NFR-PRIV-002` permits workload
> distribution outright and `ISSUE-003` had already ruled on the member-count
> case, so there was no privacy gate to defer behind. Both corrected here
> before delivery.
**Target**: 0.22.0 — **before** RFC 001
**Related spec sections**: §9 (Team / Sprint), §11.5 (individual vs aggregate boundary), §17 (Sprint Plan)
**Related requirements**: `FR-TEAM-*`, `FR-ISS-004`, `FR-HLT-*`, `NFR-PRIV-007`
**Governing gap**: baseline `§10.11`
**Last updated**: 2026-08-13 (privacy section and §D1 sample corrected)

## Summary

Two storage queries decide who may be assigned work in a project and whose
workload is shown for it. Both answer that question with `projects.owner_id`
and ignore `projects.team_id` entirely. Teams shipped in 0.11.0.

The consequence is not a display bug. `list_assignee_candidates` is also the
**write-path validator**, so in a team-owned project **no team member can be
assigned an issue**: the form rejects them with a 400. Every downstream feature
that reasons about who is carrying what — workload chips, personal
sustainability signals, and RFC 001's per-assignee planning rollup — is
therefore built on a set that can never contain more than one person.

## Background

`crates/peisear-storage/src/issues.rs`:

```sql
-- list_assignee_candidates (:612)
SELECT u.id, u.display_name
FROM users u
JOIN projects p ON p.owner_id = u.id
WHERE p.id = ?1

-- project_workload (:649)
FROM users u
JOIN projects p ON p.owner_id = u.id
LEFT JOIN issues i ON i.assignee_id = u.id AND i.project_id = p.id
WHERE p.id = ?1
```

Each join yields at most one row: the owner. The schema has carried what is
needed since 0.11.0 — `projects.team_id` (nullable FK) and
`team_memberships(team_id, user_id, role)` — and `teams::members_of_team`
already exists.

`list_assignee_candidates`'s own doc comment says:

> Today's single-tenant model returns only the project owner, but callers
> should not assume that — when team / organisation support lands (Medium-term
> roadmap), this function will return all members of the project's team.

That comment is accurate about intent and two releases stale about fact. Team
support landed; the function did not follow. The comment is why nobody looked:
it reads as a known limitation with a scheduled fix rather than as a defect.

**How it reaches a user.** `validate_assignee`
(`handlers/issues.rs:377`) rejects any assignee outside the candidate list —
deliberately, so an unknown id is a 400 rather than a silent fall back to
unassigned. Correct behaviour on a wrong set. A team member picked from a
stale form gets a validation rejection for choosing a colleague.

**Why the personal surfaces look fine.** `/me` and `/today` key off
`issues.assignee_id`, not off these queries, so they work — and always show
nothing for a non-owner, because nothing can ever be assigned to one. The
feature the product leads with is unreachable for every user except project
owners, and it fails by being empty rather than by erroring.

## Requirements

1. **Assignee candidates for a team-owned project are its team's members.**
   For a personal project (`team_id IS NULL`), the owner alone, unchanged.
2. **`project_workload` covers the same set** as `list_assignee_candidates`
   for the same project, always. The write path and the report must not be
   able to disagree.
3. **The owner is always a candidate**, including where they are not a member
   of the project's team. An owner who cannot be assigned their own issue is a
   worse defect than the one being fixed.
4. **Assignment is not authorisation.** Being assignable does not grant access
   to a project. Existing project-visibility rules are unchanged, and this RFC
   adds no new read path.
5. **`NFR-PRIV-007` holds.** `project_workload` returns per-user in-flight
   points and capacity — individual-level data, per §11.5. Widening the row
   set widens who is disclosed to whom, and §"Security and privacy" below is
   the part of this RFC that needs the most scrutiny.
6. **No silent narrowing.** Where the candidate set is empty or degenerate,
   the surface says so rather than rendering an empty control that looks like
   a loading state.

## Design

### D1 — one query, one definition

Both queries derive from a single `SELECT` over
`team_memberships`, `UNION`ed with the project owner, so requirement 2 holds
by construction rather than by two authors remembering:

```sql
SELECT DISTINCT u.id, u.display_name FROM users u
JOIN projects p ON p.id = ?1
LEFT JOIN team_memberships tm
       ON tm.team_id = p.team_id AND tm.role IN ('admin', 'member')
WHERE u.id = p.owner_id OR u.id = tm.user_id
ORDER BY u.display_name ASC
```

*Corrected 2026-08-13, found by TEAM-001.* The first version of this sample had
no role filter, and `team_memberships.role` allows `viewer` — "read-only on
team projects", per `0011_teams.sql`. It would have made every viewer an
assignee. The implementer followed the handoff's §4 rather than this sample and
reported the discrepancy; the sample is now fixed so nobody copies it.

The filter belongs in the `ON` clause, not in `WHERE`: in `WHERE` it eliminates
the owner-without-membership row and silently breaks requirement 3.

Written this way a personal project (`team_id IS NULL`) yields exactly the
owner, so the two cases are one query rather than a branch. `project_workload`
joins its aggregates onto the same set.

**Do not fix these as two independent queries.** They diverged because they
were written twice; the fix is to make that impossible, not to make it correct
once more.

### D2 — roles

`team_memberships.role` distinguishes `admin` from `member`. Both participate
in projects (0011's own comment: *"member: full project participation"*), so
both are candidates. This RFC does not introduce a non-assignable role; if one
is ever wanted, it belongs in the role vocabulary, not in an assignment query.

### D3 — the removed row problem

A user removed from a team may still hold assigned issues. Requirements 1–3
define who may be assigned **going forward**; they say nothing about existing
rows, and the two must not be conflated.

**Decision needed** (Open question 1). Doing nothing leaves an issue assigned
to a non-member, which is honest but makes the assignee list and the workload
report disagree — precisely what requirement 2 forbids. Unassigning on removal
silently discards a record of who was carrying what.

Recommended: keep the assignment, and have `project_workload` include any user
with in-flight issues in the project **whether or not** they are currently a
candidate. Then the report describes reality and the form describes policy,
and requirement 2 is restated as *the candidate set is a subset of the workload
set*, which is the true relationship.

### D4 — what this does not do

No new UI. The assignee dropdown, workload chip strip and validation path all
consume these functions already and need no change beyond what a wider result
implies. If a surface needs redesign to hold more than one person, that is a
finding — report it.

## Test plan

| Check | Mechanism |
|---|---|
| Team member is a valid assignee in a team project | Integration: create team, add member, POST an issue assigned to them, expect success |
| **Regression guard** — the defect itself | The above test, written to fail on today's code |
| Personal project still yields exactly the owner | Integration, `team_id IS NULL` |
| Owner is a candidate when not a team member | Integration: owner outside `team_memberships` |
| The two functions agree | Property-style: for a fixture set of projects, assert the candidate set ⊆ the workload set |
| Non-member is rejected | Integration: a user in no relationship to the project, expect 400 |
| Assignment grants no access | Integration: assignee who is not a project member still cannot read the project |

The regression guard matters more than the feature test. This defect survived
two releases, an external design document, a requirements baseline and a
compliance pass. What it never met was a test that tried to assign an issue to
a second person.

## Security and privacy considerations

**This is the part to review hardest.** The change widens who appears in a
per-user workload report from one person to a whole team.

**Corrected 2026-08-13, after TEAM-001.** This section originally treated a
multi-row workload strip as a new disclosure requiring an owner decision, and
told the implementer not to widen `project_workload`'s consumers. That was
wrong on the requirement and incoherent as an instruction.

**Wrong on the requirement**: `NFR-PRIV-002` explicitly permits sharing
"workload distribution" within a project or team, and defines it as *each
member's volume of in-flight work* — excluding capacity, WIP limit, and
anything derived from them, all of which `DEV-003` stripped from these two
components at 0.20.0. Nothing new is disclosed. `ISSUE-003` had already ruled
that this holds "regardless of how many members the strip lists", and that
ruling is quoted in `WorkloadStrip`'s own doc comment. I re-opened a question I
had already answered.

**Incoherent as an instruction**: `WorkloadStrip` and `WorkloadHint` iterate
whatever the query returns, so requirement 2 and "do not widen the consumers"
cannot both hold. The only way to have both is per-consumer filtering, which is
new UI logic and decides the question it was meant to defer.

What actually applies:

- `project_workload` returns display name, in-flight issue count, and in-flight
  points. It must not return `capacity_points` to a non-subject, and does not
  reach any surface that renders one (`NFR-PRIV-001`, `DEC-019`).
- `NFR-PRIV-002`'s caveat in the baseline — *"this requirement describes
  something that has never fully existed"* — is discharged by this RFC. The
  shipped `FR-TEAM-005` footnote already promises team members that "workload
  distribution is visible to all team members"; before this change that
  sentence was false in the user's favour.
- **Open question 2 is not a privacy gate.** What remains of it is whether the
  strip should show per-user rows or an aggregate — presentation, not
  disclosure. It blocks nothing.

## Out of scope

- Cross-project or global workload (`project_id = NULL`), already noted as
  future work in `project_workload`'s doc comment.
- Sprint capacity (`team_capacity_hint`), RFC 001's own concern.
- Any change to project visibility or authorisation.
- Bulk reassignment on team removal.

## Open questions

1. **Removed members holding assignments** — D3. **Settled 2026-08-13**: keep
   the assignment, and let the workload set be the superset of the candidate
   set. This is a design call and therefore mine; recording it here so the
   handoff does not re-open it. The relationship in requirement 2 is restated
   accordingly: *the candidate set is a subset of the workload set*, not equal
   to it.
2. **Who sees per-user workload rows** — **withdrawn as a privacy question,
   2026-08-13.** `NFR-PRIV-002` permits it and `ISSUE-003` already ruled on the
   member-count case; see the corrected privacy section. What survives is a
   presentation question — per-user rows versus an aggregate — which is design,
   not disclosure, and gates nothing. TEAM-001 ships as implemented.
3. **Does RFC 001 wait?** RFC 001 filters backlog by assignee and shows a
   per-assignee rollup. Both are single-valued today. It can be built on the
   current queries and will be correct once these are fixed, so the ordering
   requirement is only that **this lands first** — not that RFC 001 changes.

## References

- Baseline `§10.11`
- `crates/peisear-storage/src/issues.rs:612, :649`
- `crates/peisear-web/src/handlers/issues.rs:377`
- `crates/peisear-storage/migrations/0011_teams.sql`
- RFC 001 §3 (backlog filter, per-assignee rollup)
