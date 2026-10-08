# RFC 0015: Rescheduling without a pointer — the one surface whose keyboard path leaves the screen

**Status**: **Accepted** (2026-10-08) — owner-approved, all recommendations
taken and none varied. **The clause (§5) is in force from 0.44.0; the control
(§4) is 0.45.0 work and §7.1 says why the implementer does not start it yet.**
**Target**: **the clause at 0.44.0, the control at 0.45.0** — see §7.1 for
why the split is the optimisation rather than a delay
**Related spec sections**: `SPEC §21.2`, `SPEC §32`, `SPEC §39`;
external design `SCR-20`
**Related requirements**: **`FR-DM-002`** (P0 — Partial, three of four
on-surface), `FR-DM-001` (Met, four surfaces), `FR-DM-006` (Met),
`NFR-A11Y-007` (touch targets), `NFR-A11Y-001`
**Related register entries**: `§10.34`, `§10.15`
**Governing decisions**: establishes **`DEC-059`**
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
"where does the control live when it cannot live in the block"**. The answer
is **§4's Option A**: the block's link leads to a control whose only job is
moving that block, with the server computing both timestamps so the duration
survives and the stored effect matches a drag by construction.

**Two things in this RFC are corrections of my own work, and both are kept
visible rather than tidied away.** §4 reverses my first recommendation — a
list below the grid — which renders every scheduled issue twice on one screen
and was never cheaper to reach by keyboard, two things I asserted without
checking. §5 is the **third** draft of a one-sentence clause for
`FR-DM-002`: the first two both measured *distance*, and distance is the
symptom. **The defect is that the user has to reconstruct the effect** — the
drag shifts both ends by one delta, the edit form makes the user compute the
second one.

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

## 4. Recommendation — **Option A**, revised

**This section replaced an earlier recommendation of Option B.** The owner
asked me to re-read my own reasoning against the project's philosophy, and it
does not survive it. The reversal and its reason are kept rather than
overwritten, because the reason is the useful part.

### What I recommended, and the one thing holding it up

I recommended **Option B** — a list below the grid — and the argument rested
on a single property: it is *"genuinely on-surface"*, which satisfied the
clause I had drafted the day before. **That is a rule selecting a design.**
The owner's instruction is explicit that it should run the other way: *"the
UI/UX must be familiar with users and be what users can understand easily.
Rules should follow it as next with revision or update added instead of the
top priority."* My own §5 had already caught the clause as too strong and I
went on recommending the design the clause demanded.

### And one claim in it was never measured

I presented B as the cheaper path for a keyboard user without counting it.
**It is not cheaper.** The measurement gives **21 Tab stops** from day-view
load to a given block's link, and those stops *are* the grid's block links. A
list rendered **below the grid** sits after every one of them in DOM order, so
reaching it costs **at least the same 21** — and then the row and its
control. B is cheaper only if a skip link is added to jump to it, which is
more machinery for a path that is supposed to be the simple one.

### Option A against the four words

- **Clean.** One control, one place, nothing duplicated. The grid stays
  exactly what it is. **B renders every scheduled issue twice on one
  screen** — once spatially, once as a row — and that *is* the D-5
  objection, which I waved at by saying the list "answers a different
  question". It does not. The *control* answers a different question; the
  **list** answers the same one the grid answers.
- **Not confused or misunderstood.** A is the pattern the rest of this
  product already uses: activate the thing, act on the thing. B asks a user
  to understand that the rows beneath the calendar are the same issues as the
  blocks above, and that the select's options change meaning when the view
  changes. On a month view with forty scheduled issues, B is forty rows and a
  wall of options under the grid — worse on a phone, which `RFC 0004`'s
  requirement 10 exists to protect.
- **Robust.** A is one rendering and one handler. **B is two renderings of
  the same data that must stay in agreement forever** — precisely the
  continuous maintenance cost the owner says to weigh over initial cost.
- **Sophisticated.** The sophisticated part was never B's: it is the control
  computing both timestamps **server-side** so the stored duration is
  preserved and the effect is identical to a drag by construction. That
  property belongs to A's control unchanged.
- **Safe and secure.** Identical in both: same endpoint, same lock, same
  `client_updated_at`.

**So: Option A.** The block's link leads to a `GET`-able reschedule control
for that issue, whose only job is to move it: a `<select>` of targets labelled
in words, each carrying server-computed `planned_start_at`/`planned_end_at`
that preserve the stored duration, a hidden `client_updated_at`, and an
explicit Save. No JavaScript, no arithmetic, no second rendering of the
calendar.

**Option B stays recorded, not deleted.** If a user ever asks to move several
blocks in one sitting, B's list is the right answer to *that* request and this
section is where its design already is. That is a different request from the
one `FR-DM-002` is about.

## 5. The clause I proposed, twice, and what it should say

One day before this RFC I ruled that `FR-DM-002` should gain the clause *the
keyboard equivalent MUST be reachable from the surface that offers the pointer
affordance, **without navigating to another screen.*** The block geometry
(§2) makes that admit exactly one design. **Then I proposed a replacement —
*"from the surface, in one step, by a control whose purpose is that action"* —
and that one is still not right.** Two faults:

- **"in one step" is ambiguous**, and the owner's standing preference is for
  rules that are simple *and* clean. One keypress? One page load? One
  activation? A clause that invites that argument will get it.
- **"from the surface" is the wrong anchor.** A surface is a screen; the
  thing a pointer user acts on is an **element**. Anchoring to the element is
  concrete and checkable by inspection, and it is what the board and the
  sprint plan actually do — the form is a sibling of the card, in the card.

**And both drafts missed the fault that matters most.** Today's route fails
`FR-DM-002` not mainly because it is far away but because **the user has to
reconstruct the effect**: the drag shifts both ends by one delta, and the
issue edit form makes the user compute the new end time by hand. A clause
about distance would be satisfied by a purpose-built control three screens
deep and violated by a perfect control two activations away. Distance is the
symptom. **Reconstruction is the defect.**

### §7.2's proposal, in its final form

> The keyboard equivalent MUST be offered **by the element that carries the
> pointer affordance** — on it, or one activation away — and MUST **perform
> the same action rather than require the user to reconstruct its effect**.

Checked against what exists, by inspection and nothing else:

| | On the element? | Same action? | Verdict |
|---|---|---|---|
| Board card | the form is a sibling **in** the card | per-status buttons | **passes** |
| Issue list / detail | in place | status control | **passes** |
| Sprint plan row | the move button is **in** the row | add / remove | **passes** |
| Calendar today | three activations away | **two absolute fields; the user computes the end** | **fails both halves** |
| Option A | the block's link, **one activation** | a move control; the server computes both ends | **passes** |
| Option B | a row in a list below the grid | same control | **passes** |

It admits both options, so it has stopped doing the architecture's job. It
excludes today's route on the substance rather than on a technicality. And
**"rather than require the user to reconstruct its effect"** is the sentence
that earns its place: it is what makes a two-field edit form not an
equivalent for a move, and it generalises past the calendar to every future
surface, which is the only reason to put a clause in a requirement at all.

## 6. What this RFC is not

- **Not a redesign of `SCR-20`.** The grid, its three views and its drag are
  unchanged by every option.
- **Not a fix for the duration difference on the issue edit form.** That was
  ruled not a defect; a pair of absolute fields means what it says.
- **Not a JavaScript change.** `calendar.js` is untouched by A and B.
- **Not an accessibility defect report.** The effect is reachable today and
  both write paths are locked. What is wrong is the cost and the shape, and
  `FR-DM-002`'s `Partial` already says so.

## 6a. `DEC-059`, as taken

**Accepted 2026-10-08, with all three recommendations and none varied.**

> **`DEC-059`**: *a keyboard equivalent is judged by the element it is offered
> from and by whether it performs the action — not by how far away it is.*

Three parts, in the form they were taken:

1. **Option A.** The block's link leads to a control whose only job is moving
   that block; the server computes both timestamps so the stored duration
   survives and the effect matches a drag by construction. **Option B is
   recorded, not rejected** — it is the answer to *move several blocks in one
   sitting*, which nobody has asked for.
2. **`FR-DM-002` gains §5's clause**, in its third and final drafting:
   *offered by the element that carries the pointer affordance — on it, or one
   activation away — and performs the same action rather than requiring the
   user to reconstruct its effect.* **This is the half of `DEC-059` that
   outlives the calendar**, and it is a test applied by inspection, needing no
   instrumentation.
3. **The clause at 0.44.0, the control at 0.45.0** (§7.1). The clause is
   free, rides an amendment already happening, and converts `FR-DM-002`'s
   `Partial` from the architect's judgement into a measurable failure.

**What `DEC-059` does not decide**, so nobody reads it as having done so: the
target granularity, how many `<select>` options are too many, and where the
user lands after Save. Those are §8's, for the handoff, and the third of them
is the backlog's lost-scroll-position item arriving in scope.

## 7. Decisions taken

All three accepted as recommended — **Option A**, **§5's clause**, and
**§7.1's split schedule** — and recorded as `DEC-059` in §6a above. The
argument that was most likely to be wrong, and was not challenged, is the
D-5 one: that B's list is a second rendering of the grid's own content rather
than an answer to a different question.

### 7.1 Schedule — the rule at 0.44.0, the control at 0.45.0

**Split across two releases, deliberately, and the split is the optimisation.**

**0.44.0 — the clause only.** One sentence into `FR-DM-002`, no code. It
costs nothing, it lands with `DEC-028`'s amendment that is happening anyway,
and it converts `FR-DM-002`'s `Partial` from *a judgement the architect is
making* into *a requirement the calendar measurably fails*. A P0 whose status
is a judgement is worth less than a P0 whose status is a measurement, and
until the clause exists the status rests on my reading alone.

**0.45.0 — the control.** One handler, one component, one test. Not 0.44.0,
for a reason that is about sequencing rather than capacity:

- **`DM-TEST-001` is in flight in 0.44.0 and it pins the current behaviour of
  all four surfaces** — including item 4, the undo control's DOM position on
  the calendar. Landing a new control on that surface mid-handoff makes those
  assertions a moving target and invites the dev team to write a test against
  markup that is about to change. **Tests that pin today's behaviour must
  settle before the behaviour moves.**
- **The clause will have been in force for a release** by the time the
  implementation starts, so the handoff can cite a requirement rather than an
  RFC recommendation. That is the difference between building to a rule and
  building to a proposal.

**What rides with it at 0.45.0, because it is the same code path and not
because a release wants filling:** §8's landing-and-scroll question is the
**lost phone scroll position after a POST**, already on the backlog
unscheduled. This control is a POST that returns the user to the calendar, so
it either answers that question or makes it worse on the one surface where
the user's place on the page is the information they came for. **Answering it
inside this work is cheaper than answering it twice**, and it is in scope here
without widening anything — §8 already owns it.

**What does not ride with it**, said so it is not assumed: `FR-DM-005`'s
keyboard undo is also a keyboard-path item on these surfaces and is **RFC-sized
on its own**. Folding it in would make 0.45.0 two designs in one release,
which is the shape the owner has asked me not to produce.

**If the owner wants it sooner**, the honest option is 0.44.0 for both halves
with `DM-TEST-001` **withdrawn and reissued after** the control lands — not
run in parallel with it. I do not recommend that: it discards a handoff that
is already with the dev team in order to save two weeks on a condition that
has held since 0.36.0.

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
