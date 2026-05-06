# RFC 0004: Direct manipulation

**Status**: Draft
**Target**: 0.23.0 (Phase D, full)
**Related spec sections**: §22-27 (direct manipulation
scenarios), §32 (keyboard alternatives), §39 (Phase D plan)
**Last updated**: 2026-05-04

## Summary

Five surfaces gain drag-and-drop affordances that today
require form submissions or page navigation:

| Substep | Surface | Action |
|---|---|---|
| D-1 | Issue list, issue detail | Click status badge / segment to advance status |
| D-2 | Kanban (new view on project detail) | Drag issue between status columns |
| D-3 | Calendar | Drag issue blocks to reschedule |
| D-4 | Sprint plan | Drag between backlog and sprint |
| D-5 | Issue list | Drag rows to reorder |

Each substep ships with a keyboard equivalent that produces
the same effect; "discoverability through the mouse, parity
through the keyboard" is the contract. None of the substeps
ship without the keyboard path.

This RFC is the umbrella. Each substep gets its own follow-up
RFC (0004a, 0004b, …) once it's the next thing on the table.
The umbrella sets the shared contract — optimistic update,
undo, error rollback, accessibility — so each substep RFC can
focus on its specifics.

## Background

The existing surfaces ship with form-POST mutations as the
only way to change status, sprint membership, dates, or order.
Every change is a round-trip and a page reload. For users
who plan in bursts (a kanban shuffle session, a sprint
planning meeting, a calendar reschedule), the friction is
real.

The spec calls out (§5) that direct manipulation is a Phase D
investment, not an early-PR shortcut: "the keyboard path
must exist before the mouse path; the mouse path is icing,
the keyboard path is the contract." Until 0.19.0 we put the
affordance scaffolding in place (status segment in B-4,
sub-issues card in C-PR1) without the interaction; this RFC
is where the interactions land.

It's also where Leptos hydration starts paying for itself.
Until now Leptos has been server-only — every render is a
one-shot SSR. Phase D requires client-side state for in-
flight optimistic updates. Decision E-4 from Phase A planning
was "Leptos island for partial hydration"; D is where we
test whether that decision survives contact.

## Requirements

### Cross-cutting (apply to every substep)

1. **Keyboard parity**: every action achievable by mouse drag
   is achievable by keyboard. No mouse-only action exists.
2. **Optimistic update**: the UI moves immediately on the
   client; the request goes out behind it.
3. **Rollback on failure**: if the server returns 4xx/5xx,
   the UI reverts and shows an inline message ("status
   couldn't change: stale data — refresh and retry"). The
   message is text + icon, not just colour (§5.3 / §31).
4. **Undo toast for 5 seconds**: every successful mutation
   shows a toast with an Undo button. Clicking Undo issues
   the inverse mutation. After 5 seconds the toast
   disappears; the action is no longer un-doable through
   this path (the user can still issue the inverse action
   manually).
5. **Optimistic-lock compatibility**: mutations that already
   carry `client_updated_at` (issue, project) keep doing so.
   On a 409, we do **not** retry-with-fresh-value silently
   — that would defeat §21.4. We rollback and tell the user
   to refresh.
6. **No celebratory language** (§30 / §31): no "✓ Done!",
   no confetti, no "great job". The toast for a status →
   Done move says "Moved to Done" and offers Undo. That's
   it.
7. **All keyboard alternatives announce via `aria-live`**:
   the same screen-reader announcement fires whether the
   action came from mouse or keyboard.

### Substep contracts

Each substep RFC (0004a-e, future) commits to:

- Concrete URL / endpoint for the underlying mutation, and
  whether it's an existing endpoint or new.
- The keyboard binding (e.g. D-2's "Tab to issue, Space to
  pick up, arrow keys to move column, Enter to drop").
- The undo direction (e.g. for D-3, undo a calendar
  reschedule = restore previous start/end).
- ABDD acceptance for that surface (§30 — six axes).
- Whether the substep needs new schema (only D-5 does, for
  `sort_order`).

## Design

### Shared infrastructure

#### `peisear-web::components::dm` (new module)

A small client-side helper module that handles the
cross-cutting requirements: optimistic move, rollback,
toast lifecycle. Each substep wires its specific verbs to
this module rather than re-implementing.

```rust
// Sketch — exact API to be refined per-substep.
pub struct OptimisticMove {
    pub url: String,             // POST endpoint
    pub form_payload: FormData,  // body
    pub optimistic_dom_op: ...,  // do-this-now in DOM
    pub rollback_dom_op: ...,    // undo-this-on-error
    pub undo_url: String,        // POST endpoint for undo
    pub undo_payload: FormData,
    pub success_announce: String,// for aria-live
}
```

The bulk of this is JavaScript (vanilla, in
`crates/peisear-web/static/dm.js`). Leptos hydration is the
preferred long-term path; for D-1 we use vanilla JS to
de-risk the deadline and migrate to Leptos when D-3 forces
the issue (calendar's interaction model is non-trivial).

#### Toast component

`<aside role="status" aria-live="polite">` element that
mounts and unmounts on each move. Position: bottom-right,
auto-dismiss after 5 s, contains:

- Inline text describing the move.
- Undo button (focusable; Tab moves into it from the page
  body).
- Close button (×).

A single toast at a time. New mutations replace the existing
toast; the previous undo is no longer available. This is a
deliberate simplification — multi-toast UX is hard to get
right and adds little for the action profiles peisear sees.

#### Error-display surface

When a mutation fails, the toast above is replaced by an
error toast with:

- `role="alert"` (assertive — interrupting screen-reader
  flow is appropriate here).
- Text describing the failure ("status couldn't change:
  stale data").
- Hint text ("Refresh and try again").
- A close button.

No retry button on the error toast. Retrying without a
refresh is what §21.4 explicitly forbids.

#### Lock-conflict handling

When the server returns 409, the toast carries the
spec-mandated language: "Someone else changed this just
now. Refresh to see the latest." Do not auto-refresh — the
user may have unsaved input on another part of the page.

#### Accessibility roll-up

Each substep ships:

- ARIA roles per the W3C drag-and-drop ARIA pattern (the
  newer "live region with `role=application`" guidance,
  not the deprecated "aria-grabbed/aria-dropeffect").
- Keyboard binding documented inline in the relevant
  template, *and* in `docs/src/keyboard.md` (single source
  for users to look up).
- Focus management: after a successful move, focus follows
  the moved item to its new location.

### Substep-by-substep

The detail of each goes in its own follow-up RFC. The
sketches below are enough to plan the umbrella shape.

#### D-1: Status click toggle

- Affected surfaces: issue list (status badge becomes
  clickable), issue detail (status segment becomes
  clickable — i.e. removes the `tabindex="-1"` /
  `cursor-default` we set in B-4).
- Click cycles Open → InProgress → Done → Open.
- Right-click / long-press (mobile) opens a dropdown to
  pick directly.
- Endpoint: existing `POST /projects/{id}/issues/{issue_id}`
  with the new status. Optimistic-lock applies.
- Keyboard: focus the segment, press Space to advance. Enter
  opens the dropdown.

#### D-2: Kanban view + drag

- New view on project detail: List / Calendar / Kanban
  selector at the top of `/projects/{id}`.
- Drag from one status column to another → POST
  status change.
- Keyboard: Tab to issue, Space to "pick up" (visual lift,
  `aria-grabbed=true`), Arrow keys move column, Enter to
  drop.
- Endpoint: same as D-1.
- This substep needs new view-state schema if column order
  is configurable; ship with fixed Open/InProgress/Done
  order in PR.

#### D-3: Calendar drag

- Block body drag → reschedule (move both
  `planned_start_at` and `planned_end_at` by the delta).
- Edge drag → resize (change `planned_end_at`).
- Empty-cell drag → new-issue dialog with the cell's
  date pre-filled.
- Endpoint: `POST /projects/{id}/issues/{issue_id}` with
  updated `planned_start_at`/`planned_end_at`.
  Optimistic-lock applies.
- Keyboard: Tab to block, Shift+Arrow to extend, Arrow to
  move.

#### D-4: Sprint plan drag

- Drag backlog issue to sprint column → POST add.
- Drag sprint issue to backlog → POST remove.
- Endpoints: from RFC 0001's `/plan/add` and `/plan/remove`.
- Keyboard: Tab to issue, Space to move (cycles through
  destination columns; Enter confirms).
- No optimistic-lock — `sprint_issues` is a join table.

#### D-5: Issue list reorder

- Drag rows in the issue list to reorder.
- Order is per-user (the user's reading order; doesn't
  globally reorder for everyone).
- Schema: new `issues.sort_order` column? Or a separate
  `user_issue_order` table indexed by `(user_id, project_id)`?
  *The substep RFC will decide.* Per-user ordering with a
  global column on `issues` doesn't work; the choice is
  between a JSON blob in `user_view_states` and a separate
  table.
- Endpoint: new `POST /projects/{id}/issues/reorder` with
  the full new order (or a delta — substep RFC).
- Keyboard: Tab to row, Space to pick up, Up/Down to move,
  Enter to drop.

### Migration order

Within Phase D, ship in this order:

1. **D-1** (no schema, builds the toast + rollback module).
2. **D-2** (uses D-1's module; no schema).
3. **D-4** (no schema; reuses D-2's column-style picker for
   keyboard parity).
4. **D-3** (uses calendar from RFC 0002; no schema).
5. **D-5** (introduces the per-user-order schema).

D-3 has the most complex interaction and benefits from D-1
& D-2 having validated the optimistic-lock + rollback path.
D-5 is last because the schema decision deserves separate
review.

## Test plan

The umbrella test plan is the cross-cutting one — each
substep gets its own (mostly headless-browser-flavoured)
suite.

For the umbrella:

1. **Unit tests for the toast and rollback module**:
   simulate a successful POST, an error POST, and a 409.
   Assert toast contents, lifecycle, and rollback DOM ops.
2. **Optimistic-lock conflict tests**: extend
   `tests/optimistic_lock.rs` with one scenario per
   relevant substep (D-1 status conflict, D-3 reschedule
   conflict). Assert the server still 409s; the client-side
   rollback path is unit-tested separately.
3. **No-celebratory-language guard**: scan substep
   templates for `✓`, `done!`, `great`, "yay" etc. as a
   regex test in `tests/dm_language.rs`. Maintenance
   guardrail.

Each substep adds (in its own RFC and its own test crate):

- Headless-browser test (e.g. via `axum-test` + a small JS
  evaluator if feasible, or fall back to template-output
  inspection) for the optimistic-update flow.
- Keyboard test (template renders the right `aria-` attrs
  and key bindings).

## Security & privacy considerations

- §11.5: nothing new. Each substep targets data that's
  already owned by the operating user (issues in projects
  they have access to). The mutations route through the
  existing handlers, which retain their access checks.
- §21.4: optimistic-lock compatibility is required (see
  cross-cutting requirement #5). The substep RFCs must
  re-confirm this for their endpoint.
- The undo path issues a real mutation with the *current*
  `client_updated_at` (read off the page when the toast
  was rendered). If 5 seconds have passed and another
  client has changed the row, undo will 409, show the
  conflict toast, and stop. This is the correct outcome.

## Out of scope

- Multi-select drag ("select 5 issues, drag them
  together"). Possible later; not in the Phase D umbrella.
- Cross-project / cross-team drag. Out of scope.
- Replacing form-POST endpoints with JSON APIs. The
  existing form handlers can serve both; `Content-Type:
  application/x-www-form-urlencoded` from JS works as well
  as JSON would. Keep the surface count low.
- Real-time multi-user updates (one user sees another's
  drop happen live). Possible Phase F or beyond — needs
  a transport story (websocket / SSE) that's not on the
  table.

## Open questions

1. **Vanilla JS vs. Leptos hydration for the toast module**.
   The cross-cutting infrastructure is small enough that
   either works. Vanilla JS ships faster and avoids a
   hydration-debugging discount window; Leptos sets us up
   for richer interactions in D-3. *Default-if-no-decision:
   vanilla JS for the umbrella module; revisit at D-3
   if calendar interaction wants more.*
2. **Toast position on mobile**. Bottom-right is desktop-
   sane but on mobile may sit under the system bar.
   *Default: top-right on mobile (one media query).*
3. **D-5 schema** (per-user ordering). Decide at the D-5
   substep RFC. The two candidates are (a) JSON blob in
   `user_view_states` (simple, hard to query), (b) new
   `user_issue_order(user_id, project_id, issue_id, position)`
   table (queryable, more migration weight).

## Per-substep RFCs

When each substep starts, open its RFC under
`rfcs/0004a-direct-manipulation-status.md` etc. Use the
detailed template (this scope warrants it).

The substep RFC inherits this umbrella's contract; it
specifies only what's new.

## References

- Spec §22-27 — direct manipulation scenarios
- Spec §31 — chart alt-text and language constraints
- Spec §32 — keyboard alternatives
- Spec §39 — Phase D plan (where the substeps are first
  named)
- RFC 0001 — sprint planning page (what D-4 wires drag onto)
- RFC 0002 — calendar surfaces (what D-3 wires drag onto)
