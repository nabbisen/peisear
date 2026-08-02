# DEV-001 — Repair the optimistic-lock contract on the kanban status endpoint

**Issued by**: Architect (high-capability model)
**Date**: 2026-07-31
**Priority**: P0 — data integrity
**Depends on**: nothing. Start immediately.
**Blocks**: DEV-002 (kanban keyboard parity) — awaiting owner decision, do not anticipate it.

---

## 1. Purpose

`POST /projects/{id}/issues/{issue_id}/status` currently accepts a mutation
with **no optimistic-lock value and applies it silently**. The shipped kanban
client never sends one, so *every* drag-and-drop status change in production
bypasses the lock. Close the hole, make the client send the value, and handle
the conflict response the client will now start receiving.

## 2. Background

`NFR-CONC-001` and `NFR-CONC-005` are **P0** requirements recorded in the
0.19.1 baseline as *Implemented*. They are not implemented on this path.

The form-based paths are genuinely safe: `check_optimistic_lock`
(`crates/peisear-web/src/error.rs:212`) parses the client value as RFC3339, so
an empty string fails the parse and returns `AppError::Validation` → 400. That
is why `issue_update_with_missing_client_updated_at_is_rejected` passes.

The JSON status path never reaches that function when the value is absent. It
short-circuits at `crates/peisear-web/src/handlers/issues.rs:742-747`:

```rust
if body.client_updated_at.is_empty() {
    tracing::debug!(%issue_id, "kanban status change without client_updated_at \
                                (Phase A rollout: tracked, allowed)");
} else { /* lock is checked here */ }
```

The "Phase A rollout window" this refers to closed at 0.17.0, three minor
releases ago.

**Note the stale comment.** The doc comment at `issues.rs:720-722` claims "The
kanban JS reads it from the card's `data-updated-at` attribute and includes it
in the JSON body." **None of that is true.** The attribute is not rendered
(`components/issues.rs:526-530` emits only `data-issue-id`) and `board.js`
never reads or sends it. Do not trust that comment; correct it.

**Consequence you must design for.** Once the client sends the value, `409`
becomes reachable on this endpoint for the first time. `board.js` currently has
no conflict handling at all. Shipping step 1 without step 3 would turn a silent
data-integrity bug into a visible broken interaction.

## 3. Applicable requirements

| ID | Requirement | Priority |
|---|---|---|
| `NFR-CONC-001` | Every owned-entity mutation carries the observed `updated_at`; mismatch → 409 | P0 |
| `NFR-CONC-005` | A mutation omitting the lock value MUST be rejected, not processed unguarded | P0 |
| `NFR-CONC-004` | No force-overwrite path may exist | P0 |
| `NFR-CONC-006` | 409 body carries entity type, entity id, current `updated_at` | P2 |
| `NFR-LANG-001` | No prohibited vocabulary in any user-visible string (§1.7) | P0 |
| `FR-DM-004` | On conflict: revert, notify neutrally, re-render current state, **no automatic retry** | P1 |
| `FR-DM-005` | Conflict wording states another member changed it first; no failure vocabulary, no danger colour | P1 |

Baseline references: requirements §5.2, §1.7, §7.2; external design §15.

## 4. Change scope

Only these files:

- `crates/peisear-web/src/handlers/issues.rs` — `StatusChange`, `change_status`
- `crates/peisear-web/src/components/issues.rs` — board card render (~L505-542)
- `static/board.js`
- `crates/peisear-web/tests/optimistic_lock.rs` — new tests
- `CHANGELOG.md` — entry with rationale

## 5. Non-change scope — do not touch

- Any other handler, or `check_optimistic_lock` itself.
- The `Form`-based issue/project/sprint/capacity lock paths. They are correct.
- Keyboard accessibility of the kanban. That is DEV-002 and needs an owner
  decision first. **Do not add keyboard handlers in this task.**
- `static/search.js`. Reviewed and clean.
- Any migration. No schema change is required.
- The `Concern` / score-badge health divergence. Separate work.

## 6. Required implementation

### 6.1 Server — reject a missing lock value

In `change_status`, delete the empty-value bypass. Every request must go
through `check_optimistic_lock`.

Reject an absent or empty value with `AppError::Validation` so it renders as
**400**, consistent with the form paths and §7.2. Do not remove
`#[serde(default)]` to achieve this — that yields a 422 from the extractor and
an uncontrolled body.

The rejection message is user-visible. It MUST NOT contain "Failed", "Error",
or failure framing (§1.7). Use:

> `This page is showing an earlier version of the board. Reload to see the current state.`

Correct the stale doc comment on `StatusChange` to describe what the code
actually does.

**Separately**: `check_optimistic_lock`'s parse-failure message
(`error.rs:220-222`) interpolates the raw client value and uses developer
vocabulary. It is user-visible. Replace it with the same neutral sentence.
This is in scope because you are already establishing the wording.

### 6.2 Render the lock value on the card

In `components/issues.rs`, add `data-updated-at` to the `issue-card` anchor
(~L527). `issue.updated_at` is already in scope at L508. Serialise as RFC3339
so it round-trips through `chrono::DateTime::parse_from_rfc3339` unchanged.

### 6.3 Client — send it, and handle the outcomes

In `static/board.js`:

1. Read `data-updated-at` from the dragged card; include it as
   `client_updated_at` in the JSON body.
2. If the attribute is missing or empty, do not POST. Revert the card and show
   the reload message. A silent no-op is not acceptable.
3. **Capture the card's original column before the optimistic move** so it can
   be restored.
4. On **409**: revert the card to its original column, show the conflict
   message, and reload to re-render authoritative state. **No automatic
   retry** (`NFR-CONC-004`, `FR-DM-004`).
5. On any other non-OK response or network failure: revert the card and show a
   neutral message.
6. Remove `alert()`. Render messages into a `role="status"` region inside the
   board container; add one if absent. Keep it small — the full live-region
   treatment is Phase D (`NFR-A11Y-008`) and is **not** in this task.

Conflict wording (`FR-DM-005`) — use verbatim:

> `Another member changed this issue first. The board now shows the current state.`

Delete the string `"Failed to update status. Please refresh."` and the
`console.error("Failed to update status", err)` line. Both violate §1.7.

Do not add danger colouring to any of these messages.

## 7. Required tests

Add to `crates/peisear-web/tests/optimistic_lock.rs`:

1. `issue_status_change_with_missing_client_updated_at_is_rejected` — POST the
   JSON body **without** the field. Assert **400**. This is the regression test
   for the reported defect; it must fail against current `main`.
2. `issue_status_change_with_empty_client_updated_at_is_rejected` — field
   present, empty string. Assert 400.
3. Assert the stored status is **unchanged** after each rejection. A rejection
   that still mutates is the actual harm.
4. Assert no response body from this handler contains `"Failed"` or `"Error:"`.

Do not weaken or rewrite the existing six tests.

Existing `issue_status_change_with_stale_timestamp_returns_409` must still
pass unchanged.

## 8. Required documentation updates

- `CHANGELOG.md`: state that kanban status changes previously bypassed the
  optimistic lock, that they no longer do, and why (`NFR-MNT-009` requires the
  *why*, not just the what).
- No baseline edits. Requirements §5.2 already states the correct rule; the
  code was wrong, not the requirement. **Do not "fix" this by relaxing the
  requirement.**

## 9. Acceptance criteria

1. No request path reaches `issues::update_status` without a validated lock
   value.
2. A request with a missing or empty value returns 400 and leaves the row
   unchanged.
3. A request with a stale value returns 409 and leaves the row unchanged.
4. A drag on the board sends the value, and a conflict reverts the card
   visually.
5. No user-visible string on this path contains prohibited vocabulary.
6. `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`
   are clean.
7. All four new tests plus the existing six pass.

## 10. Prohibited shortcuts

- **Do not** keep the bypass behind a config flag, env var, or feature gate. A
  bypassable lock is not a lock.
- **Do not** add a force-overwrite or "apply anyway" path (`NFR-CONC-004`).
- **Do not** auto-retry on 409.
- **Do not** make the client fabricate a timestamp (`Date.now()`, re-fetching
  the row to obtain a fresh value, or similar). That defeats the entire
  mechanism. The value must be the one rendered with the page.
- **Do not** disable, skip, or `#[ignore]` any existing test to get green.
- **Do not** expand into keyboard parity, the health score badge, or the
  `Concern` palette. Report them if you touch them; do not fix them here.

## 11. Compatibility and security constraints

- No schema change. No migration. `updated_at` stays DB-trigger-maintained
  (`NFR-CONC-003`, `DEC-013`) — the application must never write it.
- The endpoint stays `AuthUser` (HTML session), not `ApiAuthUser`. It is
  browser-driven despite the JSON body. Do not switch extractors.
- Project access check at `issues.rs:736` stays first, before any lock logic.
  Authorisation precedes concurrency.
- The 409 body must not leak fields beyond entity type, id, and current
  `updated_at` (`NFR-CONC-006`).

## 12. Known risks

| Risk | Mitigation |
|---|---|
| Same-second `updated_at` collisions (SQLite whole-second precision) make a legitimate change look stale | Known, documented in ROADMAP. Out of scope. Do **not** attempt a sub-second migration here — raise it if you hit it in tests |
| A page open before deploy has no `data-updated-at` and its drags will 400 | Acceptable and correct. The message tells the user to reload |
| 409s become visible for the first time; may look like a regression to users | Expected. Wording and rollback are why they are in this task |

## 13. Required evidence

- Changed-file list.
- `cargo fmt --check` output.
- `cargo clippy --workspace --all-targets -- -D warnings` output.
- `cargo test -p peisear-web --test optimistic_lock -- --test-threads=1` output,
  full, including the new tests.
- Confirmation that test 1 **fails** against unmodified `main` (paste that run
  too). A regression test that passes before the fix is not testing the defect.
- Manual check: one drag succeeding, one drag conflicting. State how you
  produced the conflict.

## 14. Required review-request format

Per the organisation workflow §9.2:

1. Implementation summary
2. Addressed requirements (by ID)
3. Changed files
4. Important implementation decisions
5. Differences from this handoff, if any, with reasons
6. Executed tests and results
7. Build and static-analysis results
8. Unresolved issues
9. Known limitations
10. Requested review focus

Place the review request in `.git-exclude/review-request/`.

**Escalate to the architect rather than deciding yourself if**: the fix appears
to require touching another handler; a keyboard path seems necessary to make
the board usable; or any existing test fails in a way that suggests the lock
contract is broken elsewhere.
