# QA-002 — Three defects from CONF-001's review

**Issued by**: Architect
**Date**: 2026-08-16
**Priority**: P1 — 0.25.0, alongside `STATUS-001`
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §10
**Depends on**: CONF-001 landed

---

## 1. Scope

Three independent defects, all found while reviewing `CONF-001`, none a feature.
They share a handoff because they are small and unrelated — not because they
interact.

1. An **active sprint can be deleted**. Owner decision: it may not be.
2. **Project delete reports success to a non-owner** whose delete affected
   nothing.
3. **`prose_scan` scans comments as if they were code**, and the fix already
   exists in its sibling guard.

Do them in that order. Item 1 is the only one a user can currently be harmed by.

## 2. Item 1 — an active sprint may not be deleted

`handlers::sprints::delete_sprint` resolves membership, checks
`can_manage_team()` and the team match, verifies the lock, and deletes **any**
status.

The UI does not offer delete for an active sprint, which is why this read as a
dead path during `CONF-001`'s review. **It is not dead** — the route is live, and
`CONF-001`'s new confirmation `GET` will render "you are about to delete *X*" for
a team's running sprint and then let the `POST` do it.

**Owner decision, 2026-08-16: no.** At most one sprint per team is active, so
the live one is not equivalent to a planned one — deleting it discards the state
the team is working in right now.

### 2.1 Both halves refuse

- **`POST`** — reject an `Active` sprint before deleting. A **400** with a new
  `MessageKey`, not a 403: this is a state constraint, not an authorisation
  failure, and the caller may well be a team admin who is perfectly entitled to
  delete a *different* sprint.
- **`GET`** (the confirmation route) — refuse too. Rendering a confirmation for
  something that cannot happen is worse than refusing: it invites the user to
  press a button that will fail.

### 2.2 The message says the way out

There is one, and it already exists: complete the sprint, then delete it.

Say that. A refusal that only refuses leaves the user to guess whether the sprint
is undeletable forever. Follow the register of `SprintAlreadyCompletedMessage`
and `SprintCannotRestartCompletedMessage` — capitalised sentence, terminal
period, no failure framing.

### 2.3 What not to do

Do not hide the sprint-delete route behind a status check in the router, and do
not remove the route. The `GET`/`POST` pair is right; only the state check is
missing.

## 3. Item 2 — project delete must not report a success that did not happen

`handlers::projects::delete` calls `projects::delete(&state.db, &project_id,
&user.id)` and relies entirely on the storage layer's `WHERE owner_id = ?2`. A
team member who is not the owner deletes zero rows, gets `Ok`, and is redirected
with the *"project deleted"* flash.

**The project survives and the user is told it did not.**

**Fix at the handler**, matching what `delete_confirm` already does: resolve via
`find_accessible`, then `project.owner_id != user.id → NotFound`. That makes the
`POST` agree with the `GET` that fronts it, which is the state `CONF-001` left
half-done.

Do **not** change `projects::delete`'s signature or its `WHERE` clause. The
storage scoping is correct and is defence in depth; the defect is that the
handler treats "deleted nothing" as success.

**This is a behaviour change**: a non-owner who previously got a cheerful
redirect now gets a 404. That is the point, and the changelog should say so.

## 4. Item 3 — port `strip_line_comments` to `prose_scan`

A doc comment quoting attribute markup fails `prose_scan`. Reproduced in review:
putting `onsubmit="return confirm(...)"` into `confirmation.rs`'s module doc
yields `components/confirmation.rs:10 [attr:onsubmit]`.

`test_harness_scan` **already solved this.** Its first iteration false-positived
against its own doc comment; QA-001's round-1 correction was
`strip_line_comments`. `prose_scan` strips only `#[cfg(test)]` blocks.

Port the function across. Carry its documented limitation with it: splitting at
the first `//` truncates a literal containing `//`, which risks a shortened match
rather than a missed detection.

**Where the function lives is yours.** Duplicating a nine-line helper in two
files is defensible; so is a shared private module. If you share it, say so — a
shared helper between two guards means a change to it affects both, and that
should be a deliberate choice rather than a side effect of tidying.

**Prove it both ways**: the doc-comment case passes after the port, and a real
literal in real markup still fails. The second half matters more — a guard made
quieter is only an improvement if it stayed loud where it should.

## 5. Tests

Extend existing targets; no new one is needed.

| # | Check | Where |
|---|---|---|
| 1 | `POST` delete on an `Active` sprint → 400, sprint survives | `sprint_plan.rs` or a sprint target |
| 2 | `GET` the confirmation route for an `Active` sprint → refused, no interstitial rendered | `confirmation.rs` |
| 3 | Planned and completed sprints still delete, both halves | `confirmation.rs` |
| 4 | Non-owner `POST` project delete → 404, project survives | `confirmation.rs` |
| 5 | Owner `POST` project delete still works | existing coverage; confirm |
| 6 | A doc comment quoting attribute markup does not fail `prose_scan` | `prose_scan`'s own tests |
| 7 | A real literal in real markup still fails `prose_scan` | plant, per §4 |

Tests 1, 2 and 4 all fail on today's code without any planting — the defects are
the current behaviour. Run them first and show it.

## 6. Escalate rather than deciding

- If refusing an `Active` sprint at the `GET` needs the sprint loaded earlier
  than the route currently does, that is fine; if it needs the route
  restructured, report it.
- If the non-owner `POST` fix breaks an existing test, stop — that test encodes
  the defect and removing it is a decision, not a cleanup.
- If sharing `strip_line_comments` between the two guards means changing
  `test_harness_scan`'s behaviour in any way, report before doing it.

## 7. Acceptance

1. All seven §5 tests pass; tests 1, 2 and 4 shown failing on unmodified code.
2. An active sprint cannot be deleted through either half, and the refusal names
   completing it as the way out.
3. `projects::delete`'s signature and `WHERE` clause unchanged.
4. `prose_scan` no longer flags comments; still flags real markup.
5. All new copy through `peisear-i18n`; `prose_scan` and `test_harness_scan`
   pass.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Prohibited

No router-level status filtering and no route removal for item 1. No change to
`projects::delete`'s storage scoping for item 2. No weakening of either guard
for item 3 — the port makes `prose_scan` stop reading comments, nothing more.
No new test target. No feature work: RFC 005's "explicitly out" applies, and an
audit is not a licence to invent.

## 9. Required review-request format

Workflow §9.2. Include the failing-on-today's-code transcripts for tests 1, 2
and 4, and state where `strip_line_comments` ended up living and why.
