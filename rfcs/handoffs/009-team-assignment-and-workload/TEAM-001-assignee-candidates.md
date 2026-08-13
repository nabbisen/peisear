# TEAM-001 — Assignee candidates for team-owned projects

**Issued by**: Architect
**Date**: 2026-08-13
**Priority**: P1 — first feature work of 0.22.0
**Governing RFC**: [009](../../accepted/009-team-assignment-and-workload.md),
requirements 1–4 and D1–D3
**Depends on**: nothing

---

## 1. Scope, and what is deliberately outside it

RFC 009 splits in two. **This handoff is the first half only.**

**In scope** — requirements 1, 2, 3, 4. The candidate set becomes the project's
team membership plus the owner, so a team member can be assigned an issue in
their own team's project. Today they cannot: `list_assignee_candidates` is also
the write-path validator, and it returns the owner alone.

**Out of scope** — requirement 5 and everything about who *sees* per-user
workload. `project_workload` is corrected here so it cannot disagree with the
candidate set (requirement 2), but **it gains no new consumers and no surface
changes to show more rows to more people.** RFC 009 open question 2 — whether a
per-user chip strip should be visible to all team members, to admins only, or
replaced by an aggregate — is the owner's and is unanswered.

If you find yourself editing a component to accommodate more rows, stop and
report. That is the signal that the split needs revisiting, and it is a finding,
not a blocker you should route around.

## 2. Settled since the RFC

**Open question 1 is settled**: a user removed from a team **keeps** any issues
already assigned to them. Requirement 2 is therefore *the candidate set is a
subset of the workload set*, not equal to it — the form describes policy going
forward, the report describes reality.

So `project_workload` returns the union of: the candidate set, and any user
holding an in-flight issue in that project. Someone removed mid-sprint still
appears in the report with their real load, and stops appearing in the dropdown.

## 3. One definition, not two corrected queries

RFC 009 §D1 is binding on this point. **Do not fix these as two independent
queries.** They diverged because they were written twice; a second correct
copy is the same defect with a longer fuse.

Derive both from one candidate expression — a `SELECT` over
`team_memberships` joined through `projects.team_id`, `UNION`ed with the
project owner. A personal project (`team_id IS NULL`) then yields exactly the
owner with no branch, because the `LEFT JOIN` contributes nothing.

`project_workload` joins its aggregates onto that set, plus §2's holders-of-work
term.

Where the shared definition lives — a private helper returning a query
fragment, a SQL view, or a `WITH` clause repeated in two `query_as!` calls — is
yours. State which you chose and why. A view is a migration and therefore a
bigger commitment than it looks; a repeated `WITH` clause is duplication that
the requirement-2 test would at least catch. Both are defensible.

## 4. Roles

`team_memberships.role` distinguishes `admin` from `member`. **Both are
candidates.** `0011_teams.sql`'s own comment says a member has "full project
participation". This handoff introduces no non-assignable role; if one is ever
wanted it belongs in the role vocabulary, not in an assignment query.

## 5. Assignment is not authorisation

Requirement 4, and the thing most likely to go wrong quietly.

Widening who may be *assigned* must not widen who may *read*. Project
visibility rules are untouched. A test asserts this directly: a user who is a
valid assignee but not authorised for the project still cannot read it.

If making the candidate query work seems to require relaxing an authorisation
check, that is not a requirement of this change — report it.

## 6. Tests

The regression guard matters more than the feature test. **Write it first, and
show it failing on today's code**, the way `I18N-007` and `QA-001` both
established in this project.

| # | Check |
|---|---|
| 1 | **Regression guard**: create a team, add a member, create a team-owned project, POST an issue assigned to that member — succeeds. Fails on today's code |
| 2 | Personal project (`team_id IS NULL`) yields exactly the owner |
| 3 | Owner is a candidate even when not in `team_memberships` for that team |
| 4 | A user with no relationship to the project is rejected — still a 400, not a silent unassign |
| 5 | **Requirement 2**: over a fixture set covering personal, team-owned, and removed-member cases, assert the candidate set ⊆ the workload set |
| 6 | §2's case: a removed member holding an in-flight issue appears in the workload set and not in the candidate set |
| 7 | **Requirement 4**: a valid assignee who is not authorised for the project still cannot read it |

Test 5 is the one that keeps the two queries honest after this handoff. Write
it so it fails if someone later edits one query and not the other.

## 7. The doc comment that hid this

`list_assignee_candidates`'s comment says team support is coming and this
function will then return the team's members. It has said that since before
teams shipped in 0.11.0, which is why the defect read as a known limitation
with a scheduled fix.

Rewrite it to describe what the function does. If a comment describes a future,
it should name the RFC that will deliver it or say nothing — and this one is
now the past.

## 8. Escalate rather than deciding

- If the candidate query needs `projects.team_id` to be non-null, or exposes a
  schema assumption I have not accounted for, stop and report.
- If test 1 **passes** on unmodified code, stop. That would mean the defect is
  not what RFC 009 says it is, and everything downstream of that diagnosis
  needs re-checking before any fix lands.
- If a component or template cannot render more than one candidate without
  redesign, report it — §1 says that is a finding.

## 9. Acceptance

1. All seven §6 tests present and passing; test 1 demonstrated failing on
   unmodified code, transcript in the evidence directory.
2. One shared candidate definition, with the choice of mechanism stated.
3. No authorisation or project-visibility change.
4. No new `project_workload` consumer, no surface showing more per-user rows.
5. `list_assignee_candidates`'s doc comment describes the present.
6. fmt and clippy exit 0; the `DEC-007` gate set green; `test_harness_scan` and
   `prose_scan` still pass.
7. Any new user-visible string goes through `peisear-i18n` — RFC 006 §D6 is in
   force, including rule 7.

## 10. Prohibited

No change to project visibility or authorisation. No new `project_workload`
consumers. No bulk reassignment on team removal. No new role. No relaxation of
`validate_assignee`'s reject-unknown-id behaviour — an unknown assignee stays a
400 rather than a silent fall back to unassigned, which is the correct
behaviour that was operating on a wrong set.

## 11. Required review-request format

Workflow §9.2. Include the test-1-failing transcript as first-class evidence,
and state which mechanism §3 landed on.
