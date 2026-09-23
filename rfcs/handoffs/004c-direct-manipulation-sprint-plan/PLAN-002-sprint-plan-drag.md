# PLAN-002 — drag between the backlog and the sprint

**Target release**: 0.35.0. **Governing RFC**:
[RFC 0004c](../../accepted/004c-direct-manipulation-sprint-plan.md) (D-4),
under [RFC 0004](../../proposed/004-direct-manipulation.md)'s cross-cutting
requirements 0–10. **Depends on**: nothing.

## 1. What ships

On `/teams/{slug}/sprints/{id}/plan`, a backlog row can be dragged into the
sprint column and a sprint row back to the backlog. The move applies on the
client immediately and the POST goes out behind it. **The move buttons stay**:
they are the no-JavaScript path, the keyboard path and the touch path, and
this substep adds an affordance rather than replacing one.

**Three things are already decided and are not yours to reopen** — all settled
when the RFC was accepted:

- **No keyboard drag.** The buttons already satisfy umbrella requirement 1.
  A second keyboard idiom on one screen is a permanent cost for a user who is
  already served.
- **One toast at a time, replaced rather than stacked.** On a bulk screen a
  user may make eight moves in twenty seconds, and eight stacked toasts bury
  the screen being worked on. Each new move replaces the toast; the replaced
  move is no longer undoable through that path, which is the same bargain the
  five-second timeout already makes.
- **No optimistic lock.** `sprint_issues` carries `issue_id`, `sprint_id`,
  `assigned_at` and no `updated_at`, so `DEC-013`'s triggers do not reach it.
  No `client_updated_at`, no comparison, no 409 branch. Not "a lock that
  always passes" — none at all.

## 2. What I found reading the code, and what each fact changes

Reconciled 2026-09-23, per the practice RFC 003 established. **Five facts, and
every one of them moves something.** Confirm each rather than inheriting it.

**a. Both endpoints return `303`, not JSON.** `plan_add` and `plan_remove`
return `Redirect::to(".../plan{qs}")`. D-1's precedent looks like the opposite
— `/status` takes `Json<StatusChange>` and returns `Json<StatusChangeResponse>`
beside three form routes — **but that JSON route exists to carry the new lock
value back to the client**, which is umbrella requirement 6. This substep has
no lock, so the response carries nothing the client needs beyond success or
failure. §3.1 decides what to do instead.

**b. Every failure path already returns a real status.** `AppError::status()`
maps `Forbidden` → 403, `NotFound` → 404, `Validation` → 400. No failure
renders a 200 page. That is what makes §3.1 safe.

**c. The remove form omits three fields its own handler accepts.**
`PlanRemoveForm` declares `project`, `priority` and `assignee` with
`#[serde(default)]`, and `plan_remove` feeds them to `plan_query_string`.
`render_sprint_items`'s form sends only `issue_id`. So **a button-driven
remove drops the backlog filter and a button-driven add preserves it.**
Pre-existing and small; it matters here because the drag's fallback is a
native submit of that same form. Fixed in §4.6.

**d. The two columns render different row content.** A backlog row carries the
project name and a priority badge; a sprint row carries points only — and
`render_sprint_items` receives `(issue_id, project_id, title, effort, status)`
with **no project name at all**, so the columns cannot be made to match
without changing the query. §3.4 decides.

**e. The plan page has no script and no live region.** `dm.js`'s
`#status-announcements` and `#status-announcements-assertive` are rendered per
page in `issues.rs`, not in `AppShell`. This page needs its own pair.

## 3. Decisions — made here so they are not made in the code

### 3.1 The redirect *is* the success signal

POST the same body the form sends, with **`redirect: "manual"`**, and treat
**`res.type === "opaqueredirect"`** as success. Everything else is a failure
and reaches the fallback.

- **Not a new JSON route.** D-1 needed one to return a value; this needs none,
  and a second route per mutation is server surface, two more handlers and two
  more tests bought for nothing.
- **Not `redirect: "follow"` with `res.ok`.** A followed `303` re-renders the
  whole plan page server-side and throws it away — eight discarded renders for
  eight drags on exactly the screen where a user makes many moves quickly.
- **The failure mode under later change is benign, and that is the deciding
  argument.** If someone later makes these handlers return `200` instead of a
  redirect, this client reads success as failure and falls back to a native
  submit — which re-sends a mutation that already applied. Adding an issue
  already in the sprint, or removing one already out, is idempotent in effect
  (RFC 0004c §D1), so the user sees a page reload and the correct state. The
  inverse arrangement — `res.ok` with `follow` — would read a failure as
  success if an error path ever rendered `200`, and that failure is silent.

**Comment the `opaqueredirect` test where it is written.** It is a correct and
unusual idiom, and a reader meeting it once should not have to look it up
twice.

### 3.2 Drag the row. No handle

The drag source is the `<li>`, not a new grip control. This is the same answer
RFC 0004d settled for the calendar and for the same reason: a handle would be
a new interactive element, `NFR-A11Y-007` would give it a 44 px floor, and the
question of where a 44 px grip fits in a dense list row would have to be
answered for no gain.

**`draggable="false"` goes on the row's inner `<a>`.** An `<a href>` is
draggable by browser default, and `board.js` hit exactly this: without it the
drag becomes a link drag carrying the href in `dataTransfer`, because two
nested drag sources exist (`DEV-002-005-review.md` §1.3). That trap is already
paid for once; do not pay for it again.

### 3.3 The attachment point is a server-rendered attribute, never a client decision

The script attaches only where the server says it may. Render a data attribute
from the **same `can_move` flag the move buttons read** — not a second
expression that happens to agree with it today. A drag that appears where a
button does not is a second, divergent answer to *"may this user move this
issue"*, which is the shape RFC 009 §D1 exists to prevent. The four shapes:

| Sprint status | Role | Backlog | Move buttons | Drag |
|---|---|---|---|---|
| Planned | admin / member | shown | shown | **attached** |
| Planned | viewer | shown | hidden | **not attached** |
| Active | any | shown | hidden | **not attached** |
| Completed | any | hidden | hidden | **not attached** |

### 3.4 The moved row keeps its own markup, and that is a named limit

Per §2d the columns render different content, so a dragged row will show the
metadata of the column it came from until the next load. **Move the `<li>` as
it is.** Do not have the script rebuild the row: that is markup authorship in
JavaScript, and the priority label inside it is copy.

**This is a limit, not a solution**, and §6 asks you to photograph it in both
directions so I can judge it. If it reads badly, the fix is to make the sprint
column carry the project name and priority too — a query change and a design
change, and therefore its own step rather than something folded in here.

### 3.5 The island carries no `outcomes` block

`dm.js` and `board.js` islands carry an `outcomes` object classifying a
response into conflict / unavailable / unconfirmed (`JS-003`), and
`response_outcomes.rs` asserts that shape for both. **This island is
deliberately different**, because requirement 6 forbids a conflict path and
there is no lock to conflict over. Do not add an `outcomes` block for symmetry
with its siblings; a future reader should find this sentence rather than a
missing-looking key.

The island needs: the two "moved to" announcements, the undo label, the two
empty-state messages (§4.4), and one message for an undo that did not apply.
**Reuse `UndoButtonLabel`, `NoBacklogIssuesMessage` and
`NoSprintItemsInPlanMessage`** — they exist. Report which keys you added.

## 4. What to build

1. **Row and column attributes** in `sprint_plan.rs`, rendered from `can_move`
   (§3.3): a per-row marker carrying the issue id and which direction it moves,
   and a per-column marker naming the drop target. `draggable` on the row,
   `draggable="false"` on its inner `<a>` (§3.2).
2. **The live regions** — a polite and an assertive one on this page, matching
   `issues.rs`'s pair (§2e). A success announcement is polite; a failed undo
   is assertive (`QA-011` §2, `NFR-A11Y-008`).
3. **The copy island**, server-rendered JSON, read once at load, validated for
   shape before any listener attaches — `dm.js`'s opening twenty lines are the
   model, including the feature-detect-and-return-early shape.
4. **`static/plan.js`**, loaded with `defer` from this page only. It must:
   - move the row optimistically, then POST, then confirm per §3.1;
   - on any failure **before** the mutation lands, put the row back and fall
     back to a native submit of that row's own form. **One funnel**, the way
     `dm.js` has exactly one `fallback()` call in its chain — checkable top to
     bottom. Use the `dmBypass`-style dataset flag so the form's own listener
     steps aside for the native submission;
   - past the confirmation point, never resubmit: announce, show the toast,
     and if the DOM work throws, announce and reload;
   - show one page-level toast, replaced not stacked (§1), with Undo issuing
     the inverse POST to the opposite endpoint;
   - on a failed **undo**, announce assertively and reload — an undo has no
     form to fall back to, which is `dm.js`'s rule and the reason it exists;
   - **handle the empty and no-longer-empty cases.** Both columns render their
     `<ul>` and their empty-state paragraph conditionally, so moving the last
     row out of a column or the first row into an empty one is real DOM work.
     The empty-state strings come from the island (§3.5), never from the
     script.
5. **The script tag**, asserted by a test. `dm.js`'s and `board.js`'s tags are
   asserted by name in `status_control.rs` and `search.js`'s in `smoke.rs`,
   because `§10.14` was a script tag guarded by nothing. Put this page's in
   `sprint_plan.rs`.
6. **Fix §2c**: give the remove form the same three filter hidden inputs the
   add form has. Confirm by test that a button-driven remove preserves the
   filter, which is the assertion that would have failed before this change.

## 5. What must not change

- **The move buttons' markup and behaviour**, beyond §4.6's three added hidden
  inputs.
- **`PLAN-001`'s filter form** — a `GET`, unrelated.
- **The backlog filter's `<select>`**, which `LAYOUT-007` gave `min-w-0` for a
  reason recorded in `§10.25`.
- **`sprint_plan`'s twelve existing tests**, which must pass unchanged.
- **The no-JavaScript path**, end to end, with scripting disabled.
- **`NFR-A11Y-007`**: no new interactive element is introduced (§3.2). If you
  find yourself adding one, that is an escalation, not a `grow()` call.

## 6. Verification

- **The three shapes (§3.3) asserted in Rust**: the attribute present for an
  admin or member on a `Planned` sprint, and absent for a viewer, on an
  `Active` sprint and on a `Completed` one. This is the most important new
  test — it is the authorisation-derived attribute, and it is markup, which
  this suite can read.
- **The script tag** asserted (§4.5), and absent where it would have nothing
  to enhance.
- **The island's shape** asserted: every key the script validates is present
  and non-empty, the way `response_outcomes.rs` does for its two — and **no
  `outcomes` key**, per §3.5.
- **§4.6's filter preservation** asserted for remove as well as add.
- **The no-JavaScript path** re-verified with scripting disabled: add, remove,
  and the filter surviving both.
- **The drag itself is executed by no test**, and say so rather than implying
  otherwise — `§10.15`, open permanently by decision. What the suite asserts is
  the markup, the island and the server.
- **`BROWSER-001` 90/90**, exit 0, zero non-local requests. The plan page is
  one of its eighteen.
- **`static_js_scan`** must pass over `plan.js` with **no allowlist entry**.
  `search.js` is RFC 006's one named exclusion and this is not a second.
- **Evidence for §3.4**: the moved row photographed in both directions at
  390 px, before and after the drop.
- **Report the new `DEC-007` count.** It was 256 and it will move; no new test
  *file* is needed, so the command block itself should not change. If you find
  it does, that is an escalation.
- `fmt`, `clippy`, three consecutive `cargo test --workspace`.

## 7. Escalate rather than deciding

- **If `opaqueredirect` does not behave as §3.1 describes** on the target
  browser. The decision rests on it, and I would rather revisit the decision
  than have the code work around it.
- **If the optimistic move needs the script to author markup** beyond moving an
  element and swapping an empty-state paragraph (§3.4).
- **If a conflict path seems necessary** (§1's third bullet). That would mean
  the join table's concurrency story is not what the RFC says, which is a
  finding about the schema, not about this script.
- **If the fallback cannot be one funnel** — if a second `fallback()` call
  seems needed, say where, because `STATUS-002` shipped a wrapped-too-widely
  catch and the corrected shape is what this substep inherits.
- **If any user-visible string has no home in the message table.** There is no
  allowlist for this file.

## 8. Exit condition

Drag works in both directions on a `Planned` sprint for an admin or member and
is absent in the other three shapes, asserted in Rust; the buttons, the
keyboard path and the no-JavaScript path all unchanged and re-verified; one
toast with a working undo; no string authored in `plan.js` and no allowlist
entry; the filter preserved on remove as well as add; `BROWSER-001` 90/90;
`DEC-007`'s new count reported with its block unchanged; three consecutive
workspace runs.

---

**Who holds what**: dev team — all of §4. Architect — §3.4's limit once the
evidence is in, and anything §7 raises. **What's next**: review request.
