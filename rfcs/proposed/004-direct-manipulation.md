# RFC 0004: Direct manipulation

**Status**: Proposed
**Target**: 0.25.0 (Phase D, full)
**Related spec sections**: §22-27 (direct manipulation
scenarios), §32 (keyboard alternatives), §39 (Phase D plan)
**Governing decisions**: `DEC-021` (JavaScript posture),
`DEC-018` (board keyboard control)
**Last updated**: 2026-08-16 — reconciled against the shipped code

> **Reconciliation note (2026-08-16).** Checked against the code before any
> substep RFC was written, per the practice RFC 003 established. **The shape
> survives; two substep sketches drifted.** D-2's view shipped without its
> drag, and its "List / Calendar / Kanban selector" contradicts RFC 0002,
> which made the calendar its own screen. D-4's endpoints now exist and carry
> three constraints from `PLAN-001` that this document predates. Both
> corrected in place, each marked.
>
> D-1's preconditions were verified intact — it still describes the exact
> inert-segment markup that ships. D-3's optimistic-lock claim was already
> annotated from CAL-001's review. D-5 is unaffected.
>
> **This document is not May-vintage**, unlike RFC 003: it was revised
> 2026-08-01 for `DEC-021`. That is the likeliest reason its shape held while
> RFC 003's did not, and it is an argument for the revision itself rather than
> for the reconciliation gate.
>
> Full findings: `.git-exclude/tasks/architect/009-rfc-004-reconciliation.md`.

> **Revision note (2026-08-01).** Revised to carry `DEC-021`, and
> to correct three defects found on review: normative text that
> used prohibited failure vocabulary (§1.7 / `FR-DM-005`); an
> internal contradiction over `aria-grabbed`; and an undo path
> that cannot work as specified because the mutation endpoint
> returns no new lock value. Target shifted 0.23.0 → 0.25.0 with
> the 2026-07-31 roadmap change. Substep sketches otherwise
> unchanged.

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

### JavaScript posture (`DEC-021`)

`DEC-021` settles a question this RFC previously left open:

> No JavaScript by default. JavaScript is permitted only as a
> **named progressive enhancement** that guarantees keyboard
> parity and full degradation without it.

This is a stronger constraint than the original draft assumed.
Keyboard parity alone is not sufficient — a keyboard path
implemented *in JavaScript* still fails with scripting disabled.
Every substep must work without JavaScript at all, and the
pointer path enhances that working baseline.

In practice this is already the direction of travel. `DEC-018`
gives the board a form-POST status control ahead of this RFC
(DEV-002), and DEV-001 puts the optimistic-lock contract on the
status endpoint. D-2 therefore no longer introduces the board's
*only* status path — it layers drag onto one that already works.
The same shape applies to the other four substeps.

Consequently the earlier framing — that Phase D is "where Leptos
hydration starts paying for itself" — is withdrawn. Hydration
would make JavaScript the primary render path, which `DEC-021`
excludes. See open question 1, now resolved.

## Requirements

### Cross-cutting (apply to every substep)

0. **A working no-JS path ships first.** Every action reachable
   by drag must already be reachable by plain form POST, with
   scripting disabled, *before* the pointer affordance is added.
   The enhancement may not be the first implementation of the
   action. A substep whose no-JS path does not yet exist must
   ship that path as its own step.
1. **Keyboard parity**: every action achievable by mouse drag
   is achievable by keyboard. No mouse-only action exists.
2. **Optimistic update**: the UI moves immediately on the
   client; the request goes out behind it.
3. **Rollback on failure**: if the server returns 4xx/5xx,
   the UI reverts and shows an inline message. The message is
   text + icon, not just colour (§5.3 / §31).

   The earlier draft's example wording — *"status couldn't
   change: stale data — refresh and retry"* — is **withdrawn**:
   it is failure framing, which §1.7 prohibits and `FR-DM-005`
   forbids for conflicts specifically. Use the vocabulary in
   the lock-conflict section below, aligned with the wording
   DEV-001 establishes.
4. **Undo toast for 5 seconds**: every successful mutation
   shows a toast with an Undo button. Clicking Undo issues
   the inverse mutation. After 5 seconds the toast
   disappears; the action is no longer un-doable through
   this path (the user can still issue the inverse action
   manually).
5. **Optimistic-lock compatibility**: mutations carry
   `client_updated_at`. On a 409 we do **not**
   retry-with-fresh-value silently — that would defeat §21.4.
   We rollback and tell the user the current state is now shown.

   As of DEV-001 this applies to *every* status mutation
   including the board's, and the lock check lives in one shared
   function with two entry points. Substeps use that function;
   none may introduce a second lock check.

6. **The endpoint must return the new lock value.** Requirement
   2 (optimistic update) implies a second action on the same
   entity without an intervening page load — and requirement 4's
   undo is exactly that. After a successful mutation the stored
   `updated_at` has advanced (trigger-maintained, `DEC-013`), so
   the value the client holds is stale the instant the mutation
   succeeds.

   **This is a blocker for D-1, not a detail.** `change_status`
   currently returns `204 No Content` with no body, so the client
   cannot learn the new value; undo as drafted would 409 every
   time. External design §7.3 already specifies the correct
   behaviour — "the server compares, then either updates and
   returns the new timestamp, or returns 409". The endpoint does
   not yet do the first half.

   D-1 must therefore include: change the success response to
   carry the new `updated_at`, and have the client update the
   element's `data-updated-at` from it. Deliberately **not**
   folded into DEV-001 — that is a P0 defect fix which is ready
   to dispatch and should not be widened.
7. **No celebratory language** (§30 / §31): no "✓ Done!",
   no confetti, no "great job". The toast for a status →
   Done move says "Moved to Done" and offers Undo. That's
   it.
8. **All keyboard alternatives announce via `aria-live`**:
   the same screen-reader announcement fires whether the
   action came from mouse or keyboard.
9. **All copy routes through the i18n table.** RFC 0006 lands
   at 0.21.0, four releases before this one, so every string
   introduced here — toasts, conflict notices, announcements —
   is table copy subject to the vocabulary guard. No inline
   user-visible literals.

### Substep contracts

Each substep RFC (0004a-e, future) commits to:

- **Its no-JS baseline path** (requirement 0): which form POST
  performs this action with scripting disabled, and whether that
  path already exists or must ship first.
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

When a mutation does not succeed, the toast above is replaced
by a notice with:

- `role="alert"` (assertive — interrupting screen-reader
  flow is appropriate here).
- Neutral text describing what happened and what the current
  state is. **Not** failure vocabulary: §1.7 prohibits
  "Failed to…" and "Error:…", and `FR-DM-005` forbids failure
  framing and danger colouring for conflicts specifically.
- A close button.

No retry button. Retrying without re-reading is what §21.4
explicitly forbids.

#### Lock-conflict handling

When the server returns 409, the notice carries the wording
established by DEV-001, so board drag, board keyboard, and
Phase D surfaces all say the same thing:

> Another member changed this issue first. The board now shows
> the current state.

Adapt the second sentence per surface ("The list now shows…"),
never the first. Do not auto-refresh — the user may have
unsaved input elsewhere on the page. Neutral colouring only.

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

#### D-2: Kanban drag

*Corrected 2026-08-16, from the reconciliation
(`.git-exclude/tasks/architect/009-rfc-004-reconciliation.md` §2.2). This
substep was "Kanban view + drag"; the view shipped without it.*

- **The board already exists.** `?view=board` on project detail, with a
  **List / Board** selector (`components/issues.rs:41–60`). This substep is
  the drag, not the view.
- **The calendar is not a third view mode.** RFC 0002 shipped it as its own
  screen — `/projects/{id}/calendar`, SCR-28 — reached by a link. The earlier
  text here described a "List / Calendar / Kanban selector"; implementing that
  would change two shipped URLs and undo RFC 0002's shape. Do not.
- Drag from one status column to another → POST
  status change.
- Keyboard: Tab to issue, Space to "pick up" (visual lift),
  Arrow keys move column, Enter to drop. State is exposed
  through a live region, **not** `aria-grabbed` — the earlier
  draft contradicted its own accessibility roll-up, which
  correctly names `aria-grabbed`/`aria-dropeffect` as
  deprecated.
- No-JS baseline: the per-card form control from `DEC-018` /
  DEV-002, already shipped at 0.20.0.
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
- Endpoints: from RFC 0001's `/plan/add` and `/plan/remove` — **shipped at
  0.22.0**, not pending.
- Keyboard: Tab to issue, Space to move (cycles through
  destination columns; Enter confirms).
- No optimistic-lock — `sprint_issues` is a join table. Verified: the table
  carries `issue_id`, `sprint_id`, `assigned_at` and no `updated_at`, so
  `DEC-013`'s triggers do not reach it and there is nothing to compare.

*Added 2026-08-16, from the reconciliation §2.5.* `PLAN-001` shipped three
constraints this sketch predates, and the drag must honour all three:

- **`can_write()` gates both POSTs** — `admin`/`member` only.
- **A `viewer` sees the plan read-only**, with no move controls at all. The
  drag must not be attachable for them.
- **Move controls exist only on a `Planned` sprint.** Active is read-only;
  completed is read-only *and* hides the backlog column entirely.

A drag handle that appears where a button does not is a second, divergent
answer to "may this user move this issue" — the shape RFC 009 §D1 exists to
prevent. Derive from the same flag the buttons use.

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
3. **Vocabulary**: no separate `tests/dm_language.rs` regex.
   RFC 0006 ships the vocabulary guard at 0.21.0 and requirement
   9 puts all Phase D copy in the table, so the existing guard
   covers this surface. Extend the prohibited set with
   celebration terms (`great`, `yay`, `✓ Done!`) if RFC 0006 has
   not already; do not build a second checker.

Each substep adds (in its own RFC and its own test crate):

- **No-JS test**: the action completes with scripting disabled.
  This is requirement 0's acceptance and is not optional.
- Headless-browser test (e.g. via `axum-test` + a small JS
  evaluator if feasible, or fall back to template-output
  inspection) for the optimistic-update flow.
- Keyboard test (template renders the right `aria-` attrs
  and key bindings).
- **Undo round-trip test**: mutate, then undo within the window,
  and assert the entity returns to its prior state — which
  exercises requirement 6's returned lock value. If this test
  cannot be written, requirement 6 has not been implemented.

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

## Optimistic locking for the reschedule endpoint — added 2026-08-13

*From CAL-001's review, recorded here because this is where the risk first
becomes real.*

Phase D-3's drag-to-reschedule writes `planned_start_at` / `planned_end_at`
directly. **That endpoint must read `updated_at` before the write and compare
it, or route through `issues::update`.**

`issues` has no `updated_at` trigger — `DEC-013`'s machinery covers `sprints`,
`teams`, `team_memberships` and `user_capacities`. `issues::update` moves
`updated_at` only because its own `SET` clause does, and the whole-row edit form
touches it on every submission. A dedicated plan-date write path has neither
property.

CAL-001's implementer built exactly such a path as a throwaway, wrote a
planned date out of band, then submitted a normal edit still holding the
pre-write timestamp: **303 See Other.** Silent success, no error, no symptom —
`NFR-CONC-004` violated. The throwaway was deleted rather than shipped.

Today there is no legitimate call site that writes these columns outside the
lock-checked statement. This RFC creates the first one. A test asserting a stale
timestamp on a drag-reschedule returns 409 belongs in its handoff, and it is
demonstrable failing, unlike CAL-001's equivalent.

## Open questions

1. ~~**Vanilla JS vs. Leptos hydration for the toast module**~~
   — **Resolved by `DEC-021`: vanilla JS.** Hydration would make
   JavaScript the primary render path, which "no JS by default"
   excludes, and it would change how `peisear-web` ships assets
   (wasm target, client bundle). Adopting it would need its own
   RFC and owner sign-off. The enhancement layer stays vanilla
   and small. This closes the D-3 revisit clause too — if the
   calendar's interaction model turns out to need more than
   vanilla JS can carry, that is a signal to reduce D-3's scope,
   not to adopt hydration by the back door.
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
`rfcs/proposed/004a-direct-manipulation-status.md` etc. Use the
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
- RFC 0006 — i18n architecture and vocabulary guard
  (requirement 9; supersedes the planned `dm_language` test)
- `DEC-018`, `DEC-021` — approved decisions, 2026-07-31
- DEV-001 — optimistic-lock repair; establishes the shared lock
  check and the conflict wording this RFC adopts
- DEV-002 — board keyboard status control; D-2's no-JS baseline
- Requirements baseline §1.7; `FR-DM-002`, `FR-DM-005`,
  `NFR-CONC-004`, external design §7.3, §15
