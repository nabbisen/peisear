# CAL-004 — a move control for the keyboard

**RFC**: [0015](../../accepted/015-rescheduling-without-a-pointer.md),
**`DEC-059`**. **Target**: 0.45.0. `DM-TEST-001`/`DM-TEST-002` are closed, so
the tests that pinned this surface have settled and the behaviour may move.

**What this closes**: `FR-DM-002` (**P0**) currently fails its own clause on
the calendar — the equivalent is three activations away and makes the user
**reconstruct the effect** by computing a new end time. This handoff gives
the keyboard a control that *moves* a block, with the server computing both
timestamps so the duration survives.

**Read first**: `FR-DM-002`'s entry (the clause and how the calendar fails
it), RFC 0015 §2 (the geometry), §4 (why Option A), §5 (the clause), §8 (the
open questions this handoff must answer).

---

## §0 — one variation on `DEC-059`, stated before anything else

**RFC 0015 §3 wrote Option A as the block gaining "a second link (or its
existing link's target changes) to a `GET`-able reschedule page". This
handoff specifies neither.** The control goes on the **issue detail page**,
in its schedule area. The owner has been told; if they prefer the RFC's
literal shape, §2 is the only section that changes.

Why the literal shape does not survive contact with the code:

- **A second affordance in the block is the geometry problem again.** RFC
  0015 §2 established that no 44 × 44 px control fits — a 30-minute block is
  ~2% of a 24-hour column and a month cell is 96 px shared. `NFR-A11Y-007`
  allows an expanded hit area, but it must *participate in layout* and must
  not overlap a neighbour, and there is no room for either.
- **Changing the block's link target takes something away from pointer
  users.** Today `calendar.rs:92` and `:165` both link a block to
  `/projects/{pid}/issues/{iid}`. Pointing it at a reschedule page instead
  would serve the keyboard path by removing the pointer path to the issue.
  **Trading one user's journey for another's is not what `DEC-059` bought.**
- **The issue page is one activation from the block**, which is what the
  clause asks for, and the clause asks for *one activation*, not *a page of
  its own*.

**And the two-affordances objection is answered by what the product already
teaches.** The issue page will then offer both the edit form's two absolute
`datetime-local` fields and a move control. That is not two answers to one
question: the calendar itself already distinguishes **drag = move** from
**open and edit = set**, and this mirrors it. Keep them visually distinct and
label the move control for what it preserves (§2.3).

## §1 — the pattern to follow, which already exists twice

**Do not invent a shape.** `/status` already has exactly this: a JSON
endpoint for the script and a **form sibling** for the keyboard, both calling
one shared function.

| | JSON (script) | Form (keyboard) | Shared |
|---|---|---|---|
| status | `change_status` | `change_status_form` (`issues.rs:990`), `Form<StatusChangeForm>` → `Redirect` | `apply_status_change` (`:884`) |
| schedule | `change_schedule` (`:1222`) | **this handoff** | `apply_schedule_change` (`:1130`) — already factored out, already shared-shaped |

`apply_schedule_change` already takes start, end and `client_updated_at` as
strings and already carries the lock. **The form sibling computes the target
and calls it.** Nothing in the drag path changes.

**Two comments become false when you land this and must be corrected in the
same commit**, not left for a sweep to find:

- `app.rs:242-244` — *"No form sibling; see `change_schedule`'s own doc
  comment for why."*
- `issues.rs:1219-1221` — *"No form sibling: the day view has no plain
  control to fall back to (§2f)."*

Both are accurate records of a gap this handoff closes. Rewrite them to say
what is true; do not delete the history of why the gap existed.

## §2 — what to build

### 2.1 The form sibling

A new route and handler beside `/schedule`, form-encoded, returning a
redirect (Post/Redirect/Get), mirroring `change_status_form`. The path is
yours — `/schedule/move` reads well; say what you chose.

It receives: the issue, a **target** (§2.2), `client_updated_at`, and the
return context (§2.4). It computes both timestamps server-side and calls
`apply_schedule_change`.

**`#[serde(default)]` on `client_updated_at`**, for the reason
`StatusChangeForm` gives in its own doc comment: *a form stripped of its
hidden input, or replayed from a stale page, must fail the same way a
hand-built body does.*

### 2.2 The target, and duration preserved by construction

**Granularity: *same time, different day*, and nothing else in this
handoff.** RFC 0015 §8 named it as the drag's commonest gesture and the
smallest honest first step. A `<select>` of days, an explicit **Move**
button.

**The server computes the new end from the stored start and end** — shift
both by the same whole-day delta. The user supplies a *day*, never a
timestamp, so **there is nothing to reconstruct**, which is the half of the
clause today's route fails. Do not accept a `planned_end_at` from this form
at all; if the request carries one, that is a bug in the form, not an input
to honour.

**Edge case to decide and report, not to guess**: an issue whose
`planned_start_at` is set and `planned_end_at` is `NULL`. The calendar
renders such blocks (`calendar.rs:37` calls that case *"defensive, not
expected"*). Shifting a null end is a no-op; say what your implementation
does and why.

**`<select>` size**: the day view offers few options, the month view up to
35. Report the counts you render per view and whether any view makes the
control unusable. If one does, **say so rather than silently capping** — RFC
0015 §8 asks the question and this handoff is where it gets answered.

### 2.3 Labelling, which is the part most likely to be got wrong

**The control must say what it preserves.** A heading or label naming the
action — *move*, keeping the same length — beside the edit form's absolute
fields, which *set*. New copy goes through `peisear-i18n`'s message table
(`I18N-*`), never inline, and `static_js_scan`/the vocabulary guard apply as
usual.

**`NFR-LANG-002`'s ceiling and the prohibited-vocabulary guard apply.** If
any string you reach for is not in the table, that is a decision to report,
not a string to add quietly.

### 2.4 Where the user lands — `FR-NAV-005`, not a new invention

`FR-NAV-005` requires list filter and sort state to survive navigation, with
**the URL as the primary carrier**, and `change_status_form`'s own comment
reasons about exactly this before redirecting.

The calendar's state is `CalendarQuery { view, date }` on either
`/today/calendar` or `/projects/{id}/calendar`. **Carry `view`, `date` and
which of the two surfaces as typed form fields, and rebuild the URL
server-side.**

**Do not accept a return URL as a parameter.** A free-form redirect target
from a form is an open redirect, and nothing here needs one. Typed fields,
parsed with the same `parse_view` the calendar uses, rebuilt by the handler.
**If you find yourself wanting a URL field, stop and report.**

### 2.5 The scroll position — in scope, not optional

RFC 0015 §7.1 put the backlog's **lost phone scroll position after a POST**
in this handoff's scope, because this control is a POST that returns to the
calendar and the user's place on the page is the information they came for.

**Measure it first, then decide.** Report: on a phone-width viewport, after
a Move, where does the calendar land, and where was the user? If the answer
is "top of page, and they were 600 px down", say what it would take to fix
and whether it belongs here or is RFC-sized on its own. **An honest
measurement plus a scoping judgement closes this item; a fix is welcome but
not required.** What is *not* acceptable is landing the control and leaving
the question unmentioned.

## §3 — gates

- **`NFR-A11Y-007`** on every new control: 44 × 44 px, no overlap. This is
  the requirement that killed the in-block design; do not let the new
  control fail it on the issue page.
- **The overflow gate at five widths.** It is **120 cells** and this adds a
  control to a page the fixture already sweeps (`issue_detail`,
  `issue_detail_assigned`). A `<select>` is as wide as its longest option
  (`LAYOUT-005`, `§10.25`) and your options are **date labels**, which do not
  shrink. **Expect to have to think about this**; it is the most likely
  source of a defect in this handoff.
- **`DEC-007`** three consecutive runs, and the inventory will rise. Both
  halves if a new test file lands.
- `fmt`, `clippy -D warnings`, `rustdoc-links`.
- **The browser job's two scripts**, since a component changes.
- **`FR-DM-002`'s clause, asserted**: a test that the move control is
  reachable from the issue page and that a Move produces the **same stored
  row** as a drag of the same intent. That is the acceptance for a **P0**
  reaching Met, and it is the test that matters most here.
- **The lock**: a stale `client_updated_at` through the form must conflict
  the same way the JSON path does. `update_and_change_schedule_agree_on_a_stale_locks_status`
  (`DM-TEST-001` item 3) is the shape; extend it rather than writing a third.

## §4 — escalate rather than deciding

- **If `NFR-A11Y-007` or the overflow gate cannot be satisfied** with the
  control on the issue page. That returns the placement question to me and
  reopens §0.
- **If the move control and the edit form read as two answers to one
  question** when you see them rendered together. You will see that before I
  do. Say so.
- **If a month view's 35 options make the control unusable.**
- **If the scroll-position answer turns out to need a redesign** rather than
  a fix.
- **If any string you need is not in the message table.**

## §5 — exit condition

A move control on the issue detail page, reachable in one activation from a
calendar block, producing the **identical stored row** as the equivalent drag
with the duration preserved and the lock enforced; both false comments
corrected; `FR-DM-002`'s clause asserted by a test; `NFR-A11Y-007` and the
overflow gate green at 120; `DEC-007` three times at the new figure; and the
scroll-position question **answered**.

**Do not amend either specification.** `FR-DM-002` reaching Met is mine to
record, on your evidence.
