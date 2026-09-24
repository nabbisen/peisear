# SPRINT-003 — reopen a completed sprint

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0
**Governing RFC**: none — `DEC-053`, owner-accepted 2026-09-24. Recorded in
`FR-SPR-001`'s amendment.
**Depends on**: **`SPRINT-002` must ship first or with it.** Reopen is the
answer to a gap `SPRINT-002` completes; shipping reopen while a completed
sprint is still editable through the upsert would add a door to a room with no
walls.

**This is a feature, not a defect fix** — the first in a while. It is a handoff
rather than an RFC because every design question it raises is already settled
by an existing pattern; §2 names each one and where it comes from. **If you
find one that is not, that is an escalation, not a judgement call.**

---

## 1. Why it exists

`SPRINT-001` and `SPRINT-002` close `FR-SPR-004` in all three directions. That
is right — a completed sprint's figures should not change under anyone — and it
leaves one problem: **a sprint completed by mistake can never be corrected.**
Its numbers are wrong and frozen.

**Reopen is the answer the owner accepted**, over the alternative of leaving
history editable. The distinction it rests on: **a correction becomes a
deliberate, visible state change someone performed, instead of a silent edit to
a finished record.**

**Do not read this as a loophole in `FR-SPR-004`.** Reopen makes that
requirement enforceable without trapping people; a later change that lets a
completed sprint be edited *without* reopening would undo both.

## 2. The design, and where each part comes from

| decision | what | why it is not an open question |
|---|---|---|
| **Target state** | `completed` → **`active`** | Reopen undoes `complete`, so it returns to the state `complete` left. `planned` would discard that the sprint ran |
| **`completed_at`** | set to **`NULL`** | The column is `Option<DateTime<Utc>>`; a sprint claiming a completion date while active is a lie. `recent_completed_for_team` filters `status = 'completed'`, so the sprint leaves the velocity list on status alone — clearing the column keeps the row honest rather than changing that behaviour |
| **Who** | `can_manage_team()` | `start` and `complete` both use it; `FR-SPR-001` says administrator action |
| **Refusal when another sprint is active** | reuse **`OtherSprintActiveInTeamMessage`** | `FR-SPR-002` binds reopen exactly as it binds `start`. Same rule, same words |
| **Atomicity** | one **`BEGIN IMMEDIATE`**: read status, check the team's active sprint, write | `RACE-001` §2.2. A non-atomic reopen reintroduces the defect that left six active sprints |
| **Optimistic lock** | carried, like `start` and `complete` | Those forms already carry it (`components/sprints.rs:572`); `RACE-002` made the comparison atomic. `reopen_guarded`, matching its siblings |
| **Presentation** | a POST form on sprint detail, beside Start and Complete | Neither of those uses a confirmation interstitial, and `RFC 010`'s rule keeps dialogs for the irreversible. **Reopen is reversible — complete it again** |
| **Refusal when not completed** | a new message | `start` and `complete` each have their own wrong-state message (`SprintAlreadyActiveMessage`, `SprintNotStartedYetMessage`). Reopen needs the same, worded for it |

**New message keys**: the action label, and the not-completed refusal. Through
the message table with a doc comment each, the enumeration the guards iterate,
`en.rs`, and the fixture — the shape `SPRINT-001` used. **Reuse
`OtherSprintActiveInTeamMessage`; do not add a second key that says the same
thing.**

## 3. Deliberately not in scope

- **No audit trail.** The sprint's own status and `updated_at` are the record;
  a reopen is visible because the sprint is visibly no longer completed. An
  events table for sprints is a larger thing and nobody has asked for it.
- **No time limit** on how old a sprint may be to reopen. An arbitrary window
  is a rule to maintain and explain, and the action is already restricted to
  administrators and visible in the sprint's state.
- **No bulk reopen**, no reopen from the sprint list.
- **No change to what reopening does to the issues.** They stay as they are;
  becoming editable again is the point.

## 4. Verification

- **The transition**: a completed sprint reopens to `active` with
  `completed_at` **NULL**; it leaves `recent_completed_for_team`'s result; its
  `summary` and `burndown` are computed as for any active sprint. Assert the
  velocity list before and after.
- **`FR-SPR-002` holds**, and concurrently: N simultaneous reopens of different
  completed sprints in one team leave **one** active, in the shape `RACE-001`
  used, with the before failure rate reported. **Write this test against the
  unfixed intermediate** — a non-atomic first draft is the thing it exists to
  catch, so if your first version is already atomic, say how you know the test
  would have failed.
- **Reopen is refused while another sprint is active**, sequentially, with
  `OtherSprintActiveInTeamMessage`.
- **Reopen is refused on a planned and on an active sprint**, with the new
  message.
- **Only an administrator can reopen** — a member with write access gets the
  same refusal `start` gives them. Name the existing test shape you followed.
- **The lock**: a stale `updated_at` gets `409`, as for `start`/`complete`.
  `optimistic_lock`'s per-route convention expects a test naming this route.
- **The round trip, which is the whole point**: complete a sprint, reopen it,
  **unassign an issue — which must now succeed** — complete it again, and
  assert the figures reflect the correction. This is the test that shows
  `SPRINT-001`/`SPRINT-002` and this handoff compose into a usable workflow
  rather than three separate rules.
- `DEC-007`: report the new count, last recorded **329**. A new test file means
  the `CONTRIBUTING.md` block **and** the `test.yml` job, both authorised,
  travelling together.
- Three consecutive workspace runs; `fmt`; `clippy`; **the overflow gate** — a
  new control on a page that already has two, at five widths.

## 5. Escalate rather than deciding

- **If any of §2's eight decisions turns out not to follow** from the pattern
  it cites. That would mean this needed an RFC and I misjudged it.
- **If reopening leaves any figure inconsistent** — a summary, a burndown
  point, a carry-over, a personal metric or a snapshot that assumed
  `completed_at` was set once and never cleared. **Look for readers of
  `completed_at` before you write**, not after; I checked
  `recent_completed_for_team` and the row mapping, and that is not a promise
  about the rest of the workspace.
- **If the plain form needs JavaScript** for anything. It does not.
- **If an existing user-visible string changes.** Two are added.

## 6. Exit condition

An administrator can return a completed sprint to active, once, visibly, and
only when no other sprint in the team is active; the round trip in §4 corrects
a mistakenly completed sprint's figures; `FR-SPR-004` still refuses every edit
to a sprint that is completed.
