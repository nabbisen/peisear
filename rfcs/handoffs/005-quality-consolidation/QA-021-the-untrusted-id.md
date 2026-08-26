# QA-021 — `§10.3` is narrower than recorded; guard what actually holds

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P2 — the gap is real, prospective, and one guard wide
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §1
(`§10.3` was scheduled with that audit and excluded from it)
**Depends on**: nothing.

---

## 1. Why this exists at all

Requirements baseline `§10.3` — **the longest-open gap in the register, since
0.19.1** — says it is *"scheduled with the Phase E authorisation audit."* RFC
005 has no section for it. `QA-007` was that audit and excluded it explicitly:
*"Do not add the storage layer in this handoff."*

**Phase E is complete and nothing did it.** That is a scheduling failure of
mine, not a defect, and it is why this arrives after the phase it belonged to.

## 2. The entry is wrong, and reconciling it shrinks the work

`§10.3` reads: *"The implementation verifies at the handler layer only."*
Measured 2026-08-26:

**Every storage function handling `NFR-PRIV-001`'s inventory takes the
subject's identity and scopes on it:**

| Module | Scoped |
|---|---|
| `user_capacities` | 10 / 10 |
| `notifications` | 14 / 14 |
| `view_states` | 3 / 3 |
| `personal_metrics` | 2 / 2 |
| `user_burnout` | 1 / 1 |
| `user_metrics_snapshots` | 2 / 3 |

The one exception is `users_with_active_assignments`, a job-side aggregate, not
a per-subject read. `users.rs`'s three unscoped functions are the auth path —
`find_by_email`, `find_by_id`, `insert` — where no caller identity exists yet.

**And no handler passes a caller-supplied identity to personal-data storage.**
The three endpoints that receive one — `/api/users/{user_id}/burnout`,
`/capacity`, `/notifications` — call `require_self(&user.id, &user_id)` and
then pass **`&user.id`**, the session identity. The path value is validated and
**discarded**; it never reaches storage and never reaches the response body.

**Reproduce both halves before building anything.** If a handler does pass a
path-supplied id to a personal-data storage call, stop and report — that is a
live finding and this handoff's scope is wrong.

## 3. So what is actually missing

Scoping is not verification. `for_user(pool, user_id)` returns whatever user's
data it is handed; it cannot tell a handler mistake from a legitimate call.
`§11.5.4`'s remedy is a storage layer that knows **who is asking** as well as
**whose data is wanted**.

**But the outcome that remedy exists for is already achieved, by a different
route.** Two independent barriers stand today: `require_self` rejects a
mismatch, and the path value is never used even if it did not.

**The residual risk is exact: nothing stops a future handler from passing a
caller-supplied id into a personal-data storage function.** Not a defect — an
unguarded invariant.

## 4. What to build

**Guard the invariant that holds**, rather than adding a requester parameter to
thirty functions to satisfy a sentence.

Assert that in `handlers/api_users.rs`, the `Path`-extracted `user_id` appears
**only** inside a `require_self` call. Today it appears at three extraction
sites and in `require_self`'s own definition and doc comment, and nowhere else
— every storage call and every response field uses `user.id`.

**Say in the module doc what this does not cover**: a *new* handler file taking
a caller-supplied id would be outside the scan's reach, and a broader rule —
"no `Path`-extracted identity reaches any storage call" — needs to distinguish
identity parameters from the many legitimate `Path` ids (`project_id`,
`issue_id`, `slug`). **Consider whether that broader rule is expressible
cheaply and report your reading**; if it is not, say so and keep the narrow
one. Do not extend it to something that would need to guess.

`teams.rs:278` and `:317` also take a caller-supplied `target_user_id`, for
role change and member removal. Those are **team-membership operations, not
personal data**, and `QA-007` established they are `can_manage_team`-gated.
**Confirm that reading and leave them alone** — but say so, so the next reader
knows they were considered rather than missed.

## 5. Not in scope

- **No requester parameter threaded through storage.** Thirty functions changed
  to satisfy a sentence, when the property is already held by two other
  mechanisms, is the wrong trade — and `§11.5.4` is a `should`, not a `MUST`.
- **No change to `require_self`.**
- **No change to the auth-path functions in `users.rs`.**

## 6. Escalate rather than deciding

- **If any handler passes a caller-supplied id to personal-data storage**, stop
  and report before writing the guard. That inverts this handoff.
- If §2's per-module counts do not reproduce, stop.
- If the broader §4 rule turns out to be cheaply expressible after all, say so
  before building the narrow one — I would rather have the wider guard.

## 7. Acceptance

1. §2's two halves reproduced, per module and per endpoint.
2. The guard present, running in CI, planted — reintroduce a storage call using
   the path variable and watch it fail.
3. §4's broader-rule question answered either way.
4. `teams.rs`'s two sites confirmed out of scope, in writing.
5. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. §4's answer as prose. The plant transcript.
