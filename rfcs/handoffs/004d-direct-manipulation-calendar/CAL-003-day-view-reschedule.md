# CAL-003 — drag a day-view block to reschedule it

**Target release**: 0.36.0. **Governing RFC**:
[RFC 0004d](../../done/004d-direct-manipulation-calendar.md) (D-3), under
[RFC 0004](../../proposed/004-direct-manipulation.md)'s cross-cutting
requirements 0–10. **Depends on**: nothing.

## 1. What ships, and what was already cut from it

Drag an issue block up or down the **day view** to reschedule it. Both planned
timestamps move by one delta, so the appointment keeps its duration and the
block keeps its height.

**The RFC was accepted carrying one of the D-3 sketch's three actions**, and
the two that are gone are gone for reasons the project's own rules give rather
than for scope. Do not reintroduce them:

- **Empty-cell drag → a pre-filled new-issue dialog** has no plain-form path
  (the new-issue form takes no planned-date prefill), so umbrella requirement 0
  forbids the enhancement being the first implementation of the action.
- **Edge drag → resize** needs an edge affordance, and an edge affordance
  cannot carry `NFR-A11Y-007`'s 44 px floor inside a fifteen-minute block
  without breaking exactly the proportionality `DEC-050`'s exclusion exists to
  protect.
- **Week and month view** are out: the day view is the only view whose geometry
  *is* time, so a drag distance maps to a duration. In the others it maps to a
  date at best.

## 2. What I found reading the code, and what each fact changes

Reconciled 2026-09-23. **Six facts.** Confirm each rather than inheriting it.

**a. `update()` returns a redirect and takes the whole edit form.** Ten fields,
`Form<IssueForm>`, `AppResult<Redirect>`. **Umbrella requirement 6 — the
endpoint must return the new lock value — cannot be met by it**, and the
calendar has no way to reconstruct the other eight fields anyway. §3.1 decides.

**b. D-1 already solved this exact shape.** `/status` takes
`Json<StatusChange>` and returns `Json<StatusChangeResponse> { updated_at }`,
beside three form routes. That is the pattern to follow — **and the reason
D-4 did not follow it was that D-4 had no lock.** This substep does.

**c. There is no ordering rule on the planned dates anywhere.**
`parse_planned_datetime` parses a `datetime-local` string and that is all;
`IssueForm::validate` has nothing about start versus end. **Do not invent
one.** A reschedule moves both timestamps by one delta, so an ordering that
was valid before stays valid by construction.

> **Corrected 2026-09-23, by the implementer, and left visible.** *"Anywhere"*
> was wrong: migration `0016` carries
> `issues_planned_range_check_insert`/`_update`, which `RAISE(ABORT, …)` when
> both dates are set and the end precedes the start. I searched the web
> handlers and `IssueForm::validate` and wrote a conclusion one scope wider
> than the search. **The instruction below still stands and for the same
> reason** — a delta applied to both timestamps cannot invalidate an ordering
> that was valid, so the trigger never fires for a real drag, and no
> application-layer check was added. `update_schedule` routes its error
> through `translate_trigger_error`, so the defensive path is correctly worded
> for free.

**d. The day container is a fixed `height: 960px` for 24 hours.** Not a
percentage of the viewport — `render_day_view` renders
`style="height: 960px;"` and positions blocks by percentage inside it. So
**1 px is 90 seconds and fifteen minutes is 10 px**, exactly, at every screen
size. §3.4.

**e. The block is a bare `<a>` with an inline `style`, and no data attributes
at all** — no issue id, no lock value. The board card solved the same problem
with a wrapper `<div>` carrying the identity and `draggable`, and an inner
`<a draggable="false">`. §3.3.

**f. The calendar has no form on it, and no live region.** Every other
enhanced surface has a plain control to fall back to — the board's per-card
form, the issue list's status form, the sprint plan's move buttons. **The
calendar has only links.** The plain path is to click through to the issue and
use its edit form, which is a navigation, not a submit. §3.5 decides what
failure means here. The announcement regions are rendered per page in
`issues.rs`, so this page needs its own pair.

## 3. Decisions — made here so they are not made in the code

### 3.1 A new JSON endpoint, modelled on `/status`

`POST /projects/{project_id}/issues/{issue_id}/schedule`, taking
`{ planned_start_at, planned_end_at, client_updated_at }` and returning
`{ updated_at, time_label }`.

- **The lock check is the shared one.** `crate::error::check_optimistic_lock`,
  the same function the form paths and `/status` use. **Introduce no second
  lock check** — requirement 3 says so and `DEV-001` is why.
- **Parse with `parse_planned_datetime`**, not a second parser.
- **It is strictly narrower than the edit form**: two fields where the form has
  ten, the same access check, the same lock. It can do nothing the plain path
  cannot already do, which is the whole security argument for adding it.
- **`time_label` comes back from the server** — see §3.6.

### 3.2 The island carries an `outcomes` block — the opposite of D-4

`PLAN-002` was told **not** to carry one, and was right not to: there was no
lock and nothing to conflict over. **Here the lock applies in full**, a `409`
is reachable, and the three outcomes `JS-003` classifies — conflict,
unavailable, unconfirmed — are all distinguishable. The island carries them,
and `response_outcomes.rs` (which asserts that shape for `dm.js` and
`board.js`) gains this surface as a third.

**The difference between the two substeps is the lock, not a change of mind**,
and a future reader comparing the two islands should find that sentence.

### 3.3 The block becomes a wrapper and a link, exactly as the board card did

The wrapper `<div>` takes the absolute positioning and inline `style`, plus
`data-issue-id`, `data-project-id`, `data-updated-at`,
`data-planned-start-at`, `data-planned-end-at` and `draggable="true"`. The
`<a>` sits inside it, keeps the visual classes and takes
`draggable="false"` — an `<a href>` is draggable by browser default, and
`board.js` already paid for that lesson once.

**Two things must not move off the `<a>`:**

- **`bg-primary/15`.** `touch_target_scan`'s `is_named_escalation_exclusion`
  keys on `bg-primary/1` to exclude these blocks from the 44 px floor, because
  their height is duration-proportional. Move that class and the guard stops
  recognising the exclusion.
- **The `href`.** A click still navigates to the issue. That is the plain path.

**The wrapper is not an interactive element** by the guard's definition, the
same as the board card's own `<div>`, so `NFR-A11Y-007` acquires no new site.

**Verify the rendering is unchanged**, at 390 px and 1280 px, before and
after. The gate will not catch a visual regression, and proportionality is the
thing `DEC-050` protected.

### 3.4 The delta, and where the snap value lives

`deltaSeconds = deltaPixels / 960 * 86400`, snapped to fifteen minutes. **Both
timestamps move by the same delta** — that is what keeps the duration, and
therefore the height, unchanged (requirement 8). A block whose height changes
after a reschedule means the delta reached one timestamp and not the other.

**The snap interval comes from the island, not a literal in the script.** It is
a policy decision, and a `15` in a file no test executes is the shape `JS-001`
separates movable policy from irreducible mechanics to avoid. The pixel
arithmetic is mechanics and stays.

### 3.5 Failure means announce and reload — never resubmit

Per §2f there is no form on this page to fall back to, so umbrella requirement
2a's "fall back to a native submit" has no target. **This is `dm.js`'s *undo*
situation, and it takes `dm.js`'s undo rule**: revert the block, classify the
outcome through the island, announce, and reload where the outcome says to.
**Never resubmit.**

A `409` in particular must end in a reload, because requirement 3 says the
user is told *the current state is now shown* — and without a reload the page
is still showing theirs.

### 3.6 The block's own state must not go stale — `PLAN-002`'s lesson, applied in advance

After a **confirmed** move the script holds three facts that are now wrong:
`data-updated-at`, the two `data-planned-*` attributes, and the visible time
label. **All four must be brought into line before the move is considered
done**, or a second drag computes from stale values and an undo sends a stale
lock.

This is exactly what `PLAN-002` round 1 shipped and round 2 fixed: a moved row
kept its original marker, a second drag was silently rejected, and the stale
form action that failure masked would have diverged the client from the server.
**Do not rediscover it here.**

**The time label comes from the server**, in the response (§3.1), rather than
being formatted in JavaScript. Formatting a time in the script would be a
second authority for a format the server already owns, and this project's
pattern is to move the fact to where it can be checked.

## 4. What to build

1. **The endpoint** (§3.1): route, request and response types, handler. Access
   check, shared lock check, shared parser. Tests: success returns a new
   `updated_at` and the right `time_label`; a stale `client_updated_at`
   returns the conflict status through the same shared function; a forged
   `project_id`/`issue_id` is refused as the form path refuses it.
2. **The markup** (§3.3): the wrapper, the attributes, `draggable` on the
   wrapper and `draggable="false"` on the link. **Day view only** — the week
   and month views' `render_block` is untouched.
3. **`LOCK-001`'s guarantee, extended** (requirement 4): every day-view block
   renders a non-empty `data-updated-at`, asserted per block, in the shape
   `board_card_lock_value.rs` uses. The guarantee is what makes an optimistic
   update possible and it should be asserted where it is relied on.
4. **The live regions** (§2f): a polite and an assertive one on both calendar
   pages, matching `issues.rs`'s pair.
5. **The copy island**, server-rendered JSON, validated for shape before any
   listener attaches, carrying the `outcomes` block (§3.2), the "rescheduled
   to" announcement, the undo label and the snap interval (§3.4).
6. **`static/calendar.js`**, loaded with `defer` from the calendar pages only,
   doing §3.4's arithmetic, §3.5's failure handling and §3.6's state sync,
   with a five-second undo whose inverse mutation restores both original
   timestamps and carries the lock value the first mutation returned.
7. **The script tag**, asserted by a test — `§10.14` was a script tag guarded
   by nothing.

**Both calendar axes get the drag.** They share `render_day_view`, the server
decides permission through `find_accessible` as it already does, and the client
never computes an authorisation. Splitting them would need a flag with no
reason behind it.

## 5. What must not change

- `render_block` (week and month views), and the week/month layouts.
- The `<a>`'s `href`, its `bg-primary/15`, and the `DEC-050` exclusion.
- `update()` and the issue edit form — the plain path, reached by clicking a
  block.
- `calendar_surfaces`'s **ten** tests, which must pass unchanged.
- `FR-CAL-003`'s semantics and migration `0016`. **No new schema** — RFC 0004's
  substep contract says only D-5 needs one, and this substep confirms it.
- `FR-CAL-007`'s prohibitions: no occupancy rate, no efficiency figure, nothing
  derived from how full a day is.

## 6. Verification

- **The endpoint's three cases** asserted in Rust, including the conflict
  routed through the real shared function rather than a hand-written 409.
- **Every day-view block carries a non-empty `data-updated-at`** (§4.3).
- **The island's shape** asserted, including the `outcomes` block, and this
  surface added to `response_outcomes.rs`.
- **The script tag** asserted present on the calendar pages.
- **Rendering unchanged** by §3.3's wrapper: the block's box at 390 px and
  1280 px, before and after, pixel-identical for the same data.
- **A sequence, not only states** — the standing requirement `PLAN-002` earned:
  drag a block, drag **the same block** again, undo, **with no reload in the
  middle**, reporting `data-updated-at`, both `data-planned-*` values, the
  visible time label and the block's `top`/`height` at each step, then
  reloading once to show the server agrees. A run that reloads between drags
  verifies two first moves, not a round trip.
- **The height is unchanged by a reschedule** (requirement 8) — report it at
  each step of that sequence; if it moves, the delta reached one timestamp
  only.
- **The drag itself is executed by no test**, and say so — `§10.15`.
- **`BROWSER-001` 90/90**: both calendar pages are among its eighteen.
- **`static_js_scan`** passes over `calendar.js` with **no allowlist entry**.
- `fmt`, `clippy`, three consecutive `cargo test --workspace`, and report the
  new `DEC-007` count. A new test *file* would need the command block updated;
  say which you did.

## 7. Escalate rather than deciding

- **If the endpoint seems to need a validation rule** the edit form does not
  have (§2c). That would mean the plain path admits something this one refuses,
  which is a finding about the form, not about this endpoint.
- **If a second lock check seems necessary** anywhere.
- **If the wrapper changes the rendering** and the classes cannot be split
  without it.
- **If the snap makes the drag feel wrong** at fifteen minutes — report the
  number you would prefer and why; it is one island value, and it is mine to
  set.
- **If any user-visible string has no home in the message table.** There is no
  allowlist for this file.

## 8. Exit condition

A day-view block drags to a new time on both calendar axes, both timestamps
moving by one snapped delta with the height unchanged; the lock carried, the
new value returned and applied, a `409` reverting and reloading; a five-second
undo restoring both timestamps; no string authored in `calendar.js` and no
allowlist entry; the week and month views and the ten existing tests untouched;
`BROWSER-001` 90/90; the no-reload sequence reported; three consecutive
workspace runs with the new count stated.

---

**Who holds what**: dev team — all of §4. Architect — the snap interval if §7
raises it, and anything else §7 turns up. **What's next**: review request.
