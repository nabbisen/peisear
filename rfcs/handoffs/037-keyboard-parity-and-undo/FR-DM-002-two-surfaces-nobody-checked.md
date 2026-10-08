# FR-DM-002 / FR-DM-006 — the two surfaces nobody checked

**This is a measurement, not a build.** Two requirements were recorded
complete over two of the four direct-manipulation surfaces that ship, and
0.43.0 corrected both to `Partial` without establishing what is actually
true. `FR-DM-002` is **P0**. Establish the facts; one design question at the
end is mine, not yours.

**Background**: `requirements.md`'s `FR-DM-002`, `FR-DM-006`, `FR-DM-001`
(four surfaces, Met), `§10.34`, and `docs/development/changelog-and-releases.md`'s
sweep procedure. The surfaces in question are the **sprint-planning drag**
(0.35.0, RFC 0004c) and the **calendar block drag** (0.36.0, RFC 0004d).

**Do not amend either specification.** `DEC-028` work is the architect's.
Report what you measure.

---

## §0 — what I already found, so you don't re-find it

This is evidence, not conclusions, and **two of these three readings point
the other way from 0.43.0's correction.** If you measure something that
contradicts any of it, your measurement wins — say so plainly.

**The sprint plan looks met by construction.** `sprint_plan.rs:368` and
`:476` render a native `<form method="post">` with a submit button **per
row**, with actions `/teams/{slug}/sprints/{id}/plan/add` and `.../remove`
(`:70-71`). `plan.js:474` POSTs to `column.dataset.planUrl`, a
server-rendered URL, and `syncRowForm` (`:366`, `:386`) re-points the row's
form after a drag. So the pointer path and the keyboard path appear to reach
**the same endpoint**, which is the D-1 pattern `FR-DM-002`'s status already
describes: the form path first, the drag layered over it.

**The calendar's keyboard path exists but leaves the surface.**
`calendar.rs:183-184` makes the drag source a wrapper `<div
draggable="true">` around an `<a href>` to the issue. `calendar.js:96-108`
POSTs **JSON** to `/projects/{pid}/issues/{iid}/schedule` with
`planned_start_at`, `planned_end_at` and a `client_updated_at`. The keyboard
route is Tab to the link, Enter, then the issue edit form's two
`datetime-local` inputs (`issues.rs:1782-1795`), which post to the issue's
own URL (`submit_action` = `issue_href`, `:1462`). **Two different
endpoints, two steps, another page.**

**`FR-DM-006` is probably Met on all four, and my 0.43.0 correction of it to
"three of four" is probably wrong in the understating direction.**
`calendar.js:200-203` builds a real `<button type="button">` carrying the
undo label and `:242-244` is a **5000 ms** timer. `plan.js` holds 45 `undo`
references and 28 `toast`; `calendar.js` holds 30 and 29. The status named
three places because that is what it named in 0.26.0 and nobody looked
again — **including me, two commits before this handoff, while correcting
that very entry.** That is the eighth instance of the same class and it is
recorded as such.

## §1 — `FR-DM-002`, the sprint-planning drag

Establish, for one representative move in each direction (backlog → sprint,
sprint → backlog):

1. **Do the drag and the row button reach the same endpoint with the same
   effect?** Compare the stored row after each, not the HTTP response.
   Identical issue–sprint membership, and identical `updated_at` handling.
2. **Does the drag carry anything the form does not?** `plan.js` sends a
   body; name every field, and say for each whether the form sends it, sends
   a default, or omits it. A field only the pointer path sends is the shape
   that makes *identical effect* false while both paths "work".
3. **Is the button reachable and operable by keyboard alone** on a row that
   can move — Tab to it, Enter, no pointer. And on a row that **cannot**
   move (`can_move` false at `:386`), confirm neither path offers the action,
   so parity holds in the negative case too.

## §2 — `FR-DM-002`, the calendar block drag

1. **Does the issue edit form produce the identical stored effect** as a
   drag that moves a block? Drag a block to a new day; separately, set the
   same `planned_start_at` and `planned_end_at` through the form. **Compare
   the stored rows.** `calendar.rs:107` says the drag reuses
   `parse_planned_datetime` and the same empty-means-unset convention, so
   the normalisation should match — **verify that, do not assume it from the
   comment.** A comment asserting a shared mechanism is `§10.30`'s exact
   shape.
2. **The duration question.** A drag may move a block without changing its
   length. Does the form path require re-entering both fields to achieve
   what one drag achieves? If a user must compute the new end time by hand,
   say so — that is a difference in *effect* for the user even when the
   stored rows can be made to match.
3. **`client_updated_at`.** The drag sends it (`calendar.js:113`) and
   handles a conflict status (`:389-391`). Does the form path carry the
   optimistic lock at all? **If one path can silently overwrite a concurrent
   edit and the other cannot, that is a finding in its own right**, separate
   from parity, and it is the more serious of the two.
4. **Is anything on the calendar itself keyboard-operable for
   rescheduling?** I found no form, button or select in `calendar.rs`.
   Confirm or correct that.

## §3 — `FR-DM-006`, both surfaces

For each of the four surfaces, state whether a completed manipulation offers
an undo affordance, **how long the window is**, and **whether the undo
control is reachable and operable by keyboard**. `calendar.js:223-225`
attaches a `mousedown` handler that focuses the button — check that this
does not leave the control pointer-only in practice, which is the trap
`A11Y-005` turned on.

If undo exists on all four, say so. The expected outcome is that
`FR-DM-006` moves to **Met**, not that it stays `Partial`.

## §4 — tests

**The suite holds exactly one keyboard test**, `board_keyboard`, and
`FR-DM-002`'s acceptance line names only it and `status_control`. Whatever
§1 and §2 establish, the gap in coverage is real.

Propose the tests, do not write them all: say for each surface what a test
would assert, at what layer, and what it would cost. **These are drag paths
in JavaScript, which `§10.15` records as the open class of code executed by
no test** — so be explicit about which of your proposals a Rust integration
test can actually reach and which would need the browser-checks harness.
I would rather have two assertions that hold than four that pass for the
wrong reason (`§10.17`).

If any proposal needs a new test file, remember `DEC-007` requires **both**
the `CONTRIBUTING.md` block edit and a matching `test.yml` job.

## §5 — escalate rather than deciding

- **If the stored effects differ** on either surface — report the
  difference; do not pick which path is correct.
- **If the optimistic lock is on one path and not the other.**
- **The design question is mine**: if the effect is identical but reaching
  it requires **leaving the surface** — Tab to a link, Enter, edit two
  fields on another page — is `FR-DM-002` met? Its words are *"a keyboard
  equivalent producing the identical effect"* and *"a mouse-only action MUST
  NOT exist"*, and neither requires the equivalent to live on the same
  screen. **Report the number of keystrokes and the pages traversed for one
  reschedule, keyboard-only, and let me rule.** Do not argue it either way
  in the report.
- **If `FR-DM-006` is Met on all four**, say so and stop — do not amend the
  entry.

## §6 — exit condition

A report that states, per surface and per requirement, **what is true, what
you measured to establish it, and what remains unknown**. Specifically: the
two stored-row comparisons, the field-by-field body comparison, the
optimistic-lock answer for both calendar paths, the keystroke count for a
keyboard-only reschedule, the undo window and keyboard-operability for all
four surfaces, and the test proposals with their layer and cost.

**No code changes are expected from this handoff.** If a defect falls out of
the measurement, report it; a fix is a separate handoff with its own scope.
