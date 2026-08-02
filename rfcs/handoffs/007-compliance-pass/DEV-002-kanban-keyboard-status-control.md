# DEV-002 — Keyboard-operable status control on the kanban board

**Issued by**: Architect
**Date**: 2026-07-31
**Priority**: P0 — accessibility (`FR-DM-002`)
**Governing decision**: `DEC-018`
**Depends on**: **DEV-001 must be merged first.** Do not start until it is.

---

## 1. Purpose

The board's only status-change path is a mouse drag. `FR-DM-002` (**P0**)
states every direct-manipulation action MUST have a keyboard equivalent
producing the identical effect, and that a mouse-only action MUST NOT exist.
Add that equivalent.

## 2. Background

`components/issues.rs:526-541` renders each card as an `<a href>` with
`draggable="true"`. The anchor is keyboard-focusable, but activating it
navigates to the issue — it does not change status. There is no keyboard route
to the status change the drag performs.

`DEC-018` chose to keep the drag and add a form-POST control rather than remove
the board or pull RFC 0004 forward. The reasoning: RFC 0004's value is the
*drag* contract — optimistic update, undo, rollback — which a form POST does
not touch, so Phase D is not preempted.

## 3. Applicable requirements

| ID | Requirement | Priority |
|---|---|---|
| `FR-DM-002` | Keyboard equivalent with identical effect; no mouse-only action | P0 |
| `NFR-A11Y-001` | Every primary flow completable by keyboard alone | P0 |
| `NFR-A11Y-007` | Interactive elements ≥ 44 × 44 px | P1 |
| `NFR-A11Y-004` | Meaning not carried by colour alone | P1 |
| `NFR-CONC-001/005` | Mutation carries the lock value; omission rejected | P0 |
| `DEC-021` | No JavaScript by default | — |
| `FR-ISS-007` | State transitions result from explicit user action | P0 |

## 4. Change scope

- `crates/peisear-web/src/components/issues.rs` — board card render
- `crates/peisear-web/src/handlers/issues.rs` — a form-encoded status handler
- `crates/peisear-web/src/app.rs` — route registration if a new route is needed
- `crates/peisear-web/tests/status_segment.rs` — or a new `board_keyboard.rs`
- `CHANGELOG.md`

## 5. Non-change scope

- **Do not touch `board.js`.** DEV-001 owns it. The drag path stays as DEV-001
  leaves it.
- Do not add drag, undo, or optimistic update to the keyboard path. Those are
  RFC 0004.
- Do not alter the `SCR-11` issue-detail status segment. `FR-ISS-006` keeps it
  display-only until Phase D — this task is the *board*, not the detail screen.
- No schema change.

## 6. Required implementation

1. **Each card gains a keyboard-operable status control** inside the board
   column — a small `<form method="post">` per card with a submit control per
   reachable target status, or a `<select>` plus submit. Your choice; justify
   it in the review request.

2. **It must work with JavaScript disabled** (`DEC-021`). Plain form POST,
   Post/Redirect/Get, no `fetch`.

3. **It carries the lock token.** Hidden field `client_updated_at`, populated
   from the same `issue.updated_at` DEV-001 renders into `data-updated-at`
   (`issue.updated_at` is in scope at `components/issues.rs:508`). A missing or
   stale value is rejected exactly as DEV-001 established — reuse that path;
   do not write a second lock check.

4. **A form-encoded handler.** `change_status` takes `Json`, which a plain form
   cannot submit. Add a `Form`-based sibling, or refactor so both share one
   inner function. **The shared inner function must contain the lock check** so
   the two entry points cannot diverge — that divergence is exactly what caused
   the DEV-001 defect.

5. **Redirect back to the board** preserving filter and sort context
   (`FR-NAV-005`).

6. **Accessible naming.** Each control names the issue and the target status —
   an unlabelled "Done" button repeated twenty times down a column is unusable
   with a screen reader. Use the issue title in an accessible name or
   `aria-label`. Targets ≥ 44 × 44 px. Do not rely on colour alone.

7. **Vocabulary.** All new strings comply with §1.7 — no "Failed", no "Error",
   no achievement or celebration framing (`FR-DM-007`).

## 7. Required tests

New or extended integration tests:

1. Board HTML contains a form-based status control for each rendered card.
2. POSTing that form with a valid token changes the status and redirects.
3. POSTing with a **missing** token is rejected; status unchanged.
4. POSTing with a **stale** token returns 409; status unchanged.
5. Each control has a non-empty accessible name that distinguishes it from the
   controls on other cards.
6. The rendered board contains no user-visible prohibited vocabulary.

## 8. Acceptance criteria

1. A keyboard-only user can change an issue's status from the board.
2. The flow works with JavaScript fully disabled.
3. Both the keyboard path and the drag path pass through **one** lock check.
4. No mouse-only status action remains on the board.
5. fmt, clippy `-D warnings`, and the full web test suite are clean.

## 9. Prohibited shortcuts

- **Do not** implement the keyboard path in JavaScript. That defeats the point
  and violates `DEC-021`.
- **Do not** duplicate the lock check. One implementation, two entry points.
- **Do not** make the keyboard path skip the lock "because forms are safer".
- **Do not** use `tabindex` on non-interactive elements to fake focusability.
  Use real buttons.
- **Do not** remove or weaken the drag path to simplify this task.

## 10. Known risks

| Risk | Mitigation |
|---|---|
| Per-card forms make the board markup considerably heavier | Acceptable at the 5–30 user / project-sized board scale (`NFR-PERF-001`). Raise it if a board renders hundreds of cards |
| Two mutation paths could drift | Mitigated by the shared inner function in step 4 — this is the point of that requirement |
| Nested interactive elements: a form inside a card that is itself an `<a>` | **This is invalid HTML.** Restructure the card so the link and the form are siblings, not nested. Flag it in the review request — it is the one structural change this task requires |

## 11. Required evidence

- Changed-file list.
- fmt and clippy output.
- Full test output for the affected test crates.
- A keyboard-only walkthrough: focus a card, change its status, land back on
  the board — described step by step, stating which keys were used.
- Confirmation the flow works with JS disabled, and how you verified it.

## 12. Required review-request format

Per workflow §9.2, into `.git-exclude/review-request/`. Request focused review
on: the card restructuring in risk 3, and the shared-lock-check refactor.

**Escalate rather than deciding** if: the card restructure turns out to affect
the list or detail views; or if sharing the lock check requires changing the
existing `Json` handler's signature in a way DEV-001 did not anticipate.
