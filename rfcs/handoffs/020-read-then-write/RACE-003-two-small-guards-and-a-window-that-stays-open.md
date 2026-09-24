# RACE-003 — two small guards, and one window that stays open by decision

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0
**Governing RFC**: none — defect fix.
**Source**: the dev team's fifth-instance survey in `RACE-001`'s review request
§5, after the three ranked sites were fixed. Rulings in
`.git-exclude/reviewed/RACE-001-review.md`.
**Depends on**: `RACE-001` and `RACE-002`, both shipped.

**This is the small one.** Two defects, neither losing data, and one thing that
is *not* a defect and needs writing down so it stops being rediscovered.

---

## 1. `plan_add` versus `complete` — an issue joins a sprint that has finished

`handlers/sprints.rs:582` (and the `add_issue` path at `:794`) reads the
sprint's status, refuses a completed sprint, then calls `sprints::add_issue` on
the pool. A `complete` landing in between leaves an issue added to a finished
sprint.

**Nothing else closes it.** `RACE-002` shut the sibling case — `delete_sprint`
versus `start` — because the delete carries a lock and `start` moves the stamp.
**These routes carry no lock at all** (`D-4` shipped without one, deliberately),
so there is no stamp to move.

**Severity: a planning inconsistency, not lost data**, and it is in this handoff
rather than the last for that reason.

**The fix is `CAP-001`'s pattern**: the status read and the insert under one
`BEGIN IMMEDIATE`, inside a storage function, with the refusal returned as the
existing `Conflict`. **Do not add a lock to these routes** — that is a
different decision, it changes two forms and their failure copy, and it is not
this handoff's.

## 2. `teams::add_member` — the right state, the wrong words

Reads `role_for`, then `INSERT`s. `team_memberships` has
`PRIMARY KEY (team_id, user_id)`, so a concurrent duplicate **is** refused by
the schema — the state is correct without any change. What the second caller
gets is a raw database error where `UserAlreadyTeamMemberMessage` is owed.

**So this is copy, not integrity**, and the fix follows from that: map the
primary-key violation onto the existing message rather than wrapping the whole
thing in a transaction it does not need. If distinguishing that violation from
any other write error is not clean at the sqlx layer, the transaction is the
fallback — **say which you did and why.**

## 3. The actor-authority window — ruled **not a defect**, and to be written down

Every handler reads the actor's own role (`role_for` on the pool), checks
`can_manage_team()`, then writes. An admin demoted in the millisecond between
those two still acts.

**This stays open, deliberately.** Every authorisation check in every system
has a window between deciding and acting; closing this one means holding the
database write lock across each handler's body, which buys a millisecond of
freshness on a permission change that takes effect on the next request anyway.
The dev team's own reading was right: *"a class, not a site — the fix is not a
transaction in each handler."*

**What to do is write it down**, once, where the check lives — a short comment
on the shared authority helper (or on `role_for`, if there is no single helper)
saying: the role read is a point-in-time check; a permission change concurrent
with an action in flight may not be seen by it; this is accepted, and the
reason. **Not five copies.** It is the third time this window has been found
and reported; the comment is what makes the fourth time unnecessary.

`§10.30` is the warning to heed while writing it: **do not claim a guarantee
the code does not provide.** Say the window exists and is accepted — not that
it is small, or that some property closes it.

## 4. Verification

- **§1**: a concurrent test failing before and passing after, both runs
  reported with a failure rate, in the shape `RACE-001` used — N pairs of
  (`plan_add`, `complete`) released together; after, an issue is never in a
  completed sprint. Sequential behaviour and copy unchanged.
- **§2**: two concurrent `add_member` calls for one user — one `Ok`, one
  carrying **`UserAlreadyTeamMemberMessage`**, one membership row. Assert the
  *message*, since the state was already right and the message is the defect.
  The sequential duplicate case keeps its existing behaviour; name the test.
- **§3**: no test — it is a comment. Confirm the wording claims nothing.
- `DEC-007`: report the new count, last recorded **321**. A new test file means
  the `CONTRIBUTING.md` block **and** the `test.yml` job, both authorised,
  travelling together.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched — no markup, class or string changes.

## 5. Escalate rather than deciding

- **If §2's primary-key violation cannot be told from another write error**
  cleanly (§2).
- **If `BEGIN IMMEDIATE` on §1 measurably delays unrelated sprint writes.**
  `§10.31` is the context for reading any such number: on a path that was a
  deferred read-then-write, the faster baseline was failing.
- **If a sixth instance appears.** Three surveys have now found this shape
  nine times. Report it; do not fix it here.
- **If a user-visible string changes** other than §2's, which is the point.

## 6. Exit condition

An issue cannot join a completed sprint under a race; a concurrent duplicate
membership gets the message it is owed; the actor-authority window is written
down once, accurately, where the check lives.
