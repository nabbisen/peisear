# RFC 0015: Rescheduling without a pointer — the one surface whose keyboard path leaves the screen

**Status**: **Proposed** (2026-10-08) — open for review; implementer should
not start
**Target**: not yet scheduled; the owner asked for it to be scheduled, which
is what this RFC is for
**Related spec sections**: `SPEC §21.2`, `SPEC §32`, `SPEC §39`;
external design `SCR-20`
**Related requirements**: **`FR-DM-002`** (P0 — Partial, three of four
on-surface), `FR-DM-001` (Met, four surfaces), `FR-DM-006` (Met),
`NFR-A11Y-007` (touch targets), `NFR-A11Y-001`
**Related register entries**: `§10.34`, `§10.15`
**Governing decisions**: none yet; §7 requests two
**Last updated**: 2026-10-08 — written from `FR-DM-002`'s measurement
(`.git-exclude/reviewed/FR-DM-002-measurement-review.md`) and from the block
geometry in `components/calendar.rs`, both measured rather than described

## Summary

Three of the four direct-manipulation surfaces offer a keyboard path **on the
surface itself**: the board card carries a sibling `<form>` with per-status
buttons, and the sprint plan row carries a move button beside its drag
handle. **The calendar does not.** A keyboard user reschedules a block by
tabbing to its link, opening the issue, opening its edit form, and typing two
absolute timestamps — **77 Tab and Enter keystrokes across three pages**,
plus computing the new end time by hand, against one drag.

**This RFC does not propose the obvious fix, because the obvious fix does not
fit.** A per-block control cannot live inside a calendar block: in the
day and week views **a block's height *is* its duration**, so a 30-minute
block is roughly 2% of a 24-hour column, and in the month view a cell is
`h-24` (96 px) holding a day number and several blocks. There is no room for
a 44 × 44 px control per block, and `NFR-A11Y-007` is not negotiable.

So the question this RFC exists to answer is **not "which control" but
"where does the control live when it cannot live in the block"** — and the
answer bears on a clause I proposed one day earlier, which the geometry
contradicts.

## 1. Background — what is true today, measured

From the `FR-DM-002` measurement, all verified against the tree:

- **The drag**: `calendar.js:96-108` POSTs JSON to
  `/projects/{pid}/issues/{iid}/schedule` with `planned_start_at`,
  `planned_end_at` and `client_updated_at`. It always shifts **both**
  timestamps by the identical delta (`:313-319`), so **a drag cannot change a
  block's length**.
- **The keyboard route**: `calendar.rs:183-184` makes the block a
  `<div draggable="true">` wrapping an `<a href>` to the issue. The only
  interactive element on the view is that link. `calendar.rs` renders **no
  form, button or select at all** — confirmed by grep, zero rendered
  elements.
- **The destination**: the issue edit form's two `datetime-local` inputs
  (`issues.rs:1782-1795`), posting to the issue's own URL.
- **Both write paths are locked.** `update` (`issues.rs:752`) and
  `apply_schedule_change` (`:1130-1175`) each call `check_optimistic_lock`
  against the same `client_updated_at`. **There is no lock asymmetry** — this
  was raised as a suspicion and measured away.
- **The two paths agree by convention, not by construction.** They are two
  independently-written call sites that both call `parse_planned_datetime`
  and write the same two columns. The comment at `calendar.rs:107` claiming
  the drag *reuses* the parse helper is *true in effect, false in mechanism.*
- **The form is an edit, not a move.** Two absolute fields with an ordering
  guard (a database CHECK, surfaced at
  `peisear-storage/src/issues.rs:409-412`) and **no duration guard**:
  changing only the start to a validly-ordered later time silently shortens
  the block. That is not a defect — a pair of absolute fields cannot carry
  "preserve the length" — but it is why the form **is not the drag's
  equivalent in substance**, whatever the stored rows can be made to match.

### The precedent, and why it does not transfer

The board solved this problem and its solution is worth copying in spirit:
`issues.rs:905-911` renders, as a **sibling** of the card's link inside the
same draggable div, a `<form method="post">` carrying a hidden
`client_updated_at` and one submit button per status. Sibling rather than
child because a `<form>` cannot nest inside an `<a>`. **No JavaScript is
required for it to work**, and `board_keyboard` has seven tests on it.

It transfers in two respects and fails in a third:

- **Transfers**: the sibling-form shape, the hidden lock stamp, and the rule
  that a `<select>` must never submit on change — `NFR-A11Y-002`'s own
  defect, where a navigation key committed a demotion, is why the team role
  control gained an explicit Save at 0.41.0.
- **Does not transfer**: **status is a finite set and a date is not.** Four
  buttons cover every status. The month view has 35 visible cells and the day
  view has a continuum. And the board card is a card with room; the calendar
  block, as above, is not.

## 2. The constraint that shapes every option

**A 44 × 44 px control cannot go inside a calendar block.** This is geometry,
not taste:

| View | Block | Room for a control |
|---|---|---|
| Day / week | `absolute left-12 right-1`, height proportional to duration, `text-xs px-1.5 py-0.5` | **none** — a 30-minute block is ~2% of a 24-hour column |
| Month | `<td class="h-24 p-1">` holding a day number plus every block for that day | **none** — 96 px total, shared |

Any option that puts a per-block control in the grid **changes what the
calendar is**, and the overflow gate's 120 cells would not be the half of it.
That is a redesign of `SCR-20`, not a parity fix, and this RFC does not
propose one.

## 3. Options

### Option A — a purpose-built reschedule control, one hop from the block

The block gains a **second link** (or its existing link's target changes) to
a `GET`-able reschedule page for that issue:
`/projects/{pid}/issues/{iid}/schedule`, which today is POST-only.

That page carries a **move** control, not an edit: a `<select>` of targets
drawn from the view the user came from, each option's label in words
(*"Thursday 9 October, same time"*), each carrying a **server-computed**
`planned_start_at`/`planned_end_at` pair that preserves the stored duration —
plus a hidden `client_updated_at` and an explicit Save. No arithmetic for the
user, no JavaScript, and the stored effect is **identical to a drag by
construction**, because the server computes the same shift the drag computes.

- **Cost**: one handler, one small component, one round trip. The `/schedule`
  route exists; it gains a `GET`.
- **Keystrokes**: roughly 22 Tab + Enter to the block, then a select and a
  Save — against 77 and two typed timestamps.
- **Against it**: it is still **a different screen**, so it does not satisfy
  the clause §7.2 was going to propose. See §5.

### Option B — a per-view reschedule section below the grid

The calendar page gains one form **below the grid**, listing the view's
scheduled issues with the same move `<select>` per row. One screen, no
navigation, and the grid is untouched.

- **Cost**: one component, one handler, no new route. Slightly more markup on
  a page the overflow gate already sweeps at five widths.
- **For it**: it is genuinely **on-surface** and it is the pattern RFC 0004's
  cross-cutting requirement 10 already blesses — *on a phone the affordance
  is per-row buttons rather than a drag.* A list of rows under the grid is
  that requirement's own answer, arrived at from the opposite direction.
- **Against it**: it duplicates, in a list, what the grid shows spatially.
  On a month view with many issues the list is long. And it is a second
  answer to *"where is this issue"* on one screen — the objection that
  retired D-5.

### Option C — selection state in the grid, acted on by a toolbar

Focus a block, and a toolbar above the grid acts on the focused block.

- **Rejected, and recorded so it is not re-proposed.** Selection state across
  a round trip means either JavaScript — which would make the only keyboard
  path depend on the thing every other surface's keyboard path works
  without — or a `?selected=` parameter and a re-render per keystroke. The
  board and D-1 both ship a no-JavaScript path **first**; this would be the
  one surface that inverts that.

### Option D — do nothing, amend `FR-DM-002` to permit an off-surface equivalent

Honest to record. The effect is already reachable, both paths are locked, and
nobody has reported this.

- **Against it**: 77 keystrokes and hand arithmetic is not an equivalent in
  any sense a user would recognise, and an amendment written to make the
  record green is the failure `§10.34` is about.

## 4. Recommendation

**Option B, with Option A's control**, in that order of preference — and the
two are not exclusive: B's per-row control is A's control, rendered on the
calendar page instead of on its own.

The reasoning, in the owner's order:

1. **The interface first.** B puts the keyboard path on the surface that
   offers the pointer affordance, which is what the other three surfaces do
   and the only reason this requirement reads `Partial`.
2. **It is the familiar shape, not a new one.** A list of rows with a control
   each is the sprint plan, the board's per-card form, and RFC 0004's own
   phone answer. A user who has used any other surface already knows it.
3. **Duration is preserved by construction**, because the server computes
   both timestamps. That removes the one genuine difference in *effect*
   between the two paths, which no amount of form-field polish on the issue
   page would.
4. **Maintenance cost is a component and a handler**, with no new route, no
   JavaScript, and no state carried across a request. The continuous cost the
   owner cares about is close to zero; the initial cost is a day.

**The D-5 objection must be answered, not waved at.** D-5 was retired because
a manual order would have been *a second answer to the same question with no
name in the UI*. This is not that: the list below the grid answers a
**different** question — *how do I move this without a pointer* — and it has
a name, because it carries a heading saying so. If the owner reads it as the
same objection, that is a reason to prefer A and I will not argue it as
settled.

## 5. The clause I proposed, and the geometry that contradicts it

One day before this RFC I ruled that `FR-DM-002` should gain one clause: *the
keyboard equivalent MUST be reachable from the surface that offers the pointer
affordance, **without navigating to another screen.*** I wrote that before
measuring the block geometry.

**§2 shows the grid cannot hold a control**, so the clause as drafted leaves
exactly one compliant design — Option B — and forbids Option A on a point of
wording rather than of substance. A clause that selects the design is a rule
doing the architecture's job.

So §7.2 asks for the clause in a weaker, truer form: the equivalent must be
reachable **from the surface, in one step, by a control whose purpose is that
action** — which admits both A and B, excludes today's three-page route, and
says what the board and the sprint plan actually do. **I am proposing the
revision of my own ruling, one day old, because the measurement came after
it.** Recording it rather than quietly re-drafting, since `§10.34` is about
exactly the gap between what a record says and what was checked.

## 6. What this RFC is not

- **Not a redesign of `SCR-20`.** The grid, its three views and its drag are
  unchanged by every option.
- **Not a fix for the duration difference on the issue edit form.** That was
  ruled not a defect; a pair of absolute fields means what it says.
- **Not a JavaScript change.** `calendar.js` is untouched by A and B.
- **Not an accessibility defect report.** The effect is reachable today and
  both write paths are locked. What is wrong is the cost and the shape, and
  `FR-DM-002`'s `Partial` already says so.

## 7. Decisions requested

1. **Option A, B, or B-with-A's-control** — or D, if the owner reads 77
   keystrokes as acceptable for a v0. The recommendation is **B with A's
   control**; §4's third paragraph is the argument most likely to be wrong.
2. **`FR-DM-002`'s clause, in the weaker form** of §5 rather than as I first
   drafted it: *reachable from the surface, in one step, by a control whose
   purpose is that action.* This is the decision that matters beyond the
   calendar, because it is the test every future surface is held to.
3. **Scheduling.** The work is one component, one handler and a test, and it
   does not need splitting. It does not block anything and nothing blocks it.

## 8. Open questions for the implementer, once accepted

Not decisions — things the handoff must answer rather than assume:

- **Target granularity.** The day view has times; the month view has dates.
  Does the `<select>` offer *"same time, different day"* only, or also time
  slots? *Same time, different day* is the drag's commonest gesture and the
  smallest honest first step.
- **How many options** before the `<select>` stops being usable — the month
  view's 35 cells is the upper bound to think about.
- **Where the user lands after Save**, and whether the calendar's scroll
  position survives it. The lost-scroll-position item is already on the
  backlog and this must not make it worse.
- **`NFR-A11Y-007`** on the new control, and the overflow gate at five
  widths with the list present.
