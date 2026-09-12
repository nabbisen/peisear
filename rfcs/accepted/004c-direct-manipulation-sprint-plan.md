# RFC 0004c: Direct manipulation — the sprint plan (D-4)

**Status**: **Accepted** (2026-09-13) — implementation may begin
**Target**: 0.35.0
**Umbrella**: [RFC 0004](../proposed/004-direct-manipulation.md) — substep D-4
**Governing decisions**: `DEC-021`, `DEC-013`
**Related requirements**: `FR-SPR-*`, `FR-DM-002/005`, `NFR-LANG-001`,
`NFR-A11Y-001/006/007`
**Last updated**: 2026-09-13 — accepted; both open questions settled below

## Summary

Drag a backlog issue into the sprint, or out of it, on
`/teams/{slug}/sprints/{id}/plan`.

**The no-JS path already ships in full** — `PLAN-001` (0.22.0) gave the screen
two columns and button-driven moves through `POST /plan/add` and
`POST /plan/remove`. So requirement 0 is satisfied before this substep starts,
which is the opposite of D-1's position and the same as D-2's.

**Three things make this substep unlike its predecessors, and each one removes
work rather than adding it:**

1. **There is no optimistic lock to carry.** `sprint_issues` is a join table
   with `issue_id`, `sprint_id`, `assigned_at` and no `updated_at`, so
   `DEC-013`'s triggers do not reach it. Umbrella requirements 5 and 6 — carry
   `client_updated_at`, return the new lock value — **do not apply here**, and
   this is the first substep where that is true. Nothing to compare means no
   409 path and no stale-value bug.
2. **Keyboard parity already exists** and is not the sketch's binding. Every
   move is a `<button>` in a form: Tab reaches it, Enter activates it. The
   umbrella's requirement 1 is met by the screen as it stands.
3. **The authorisation gating already exists and must be reused, not
   re-derived.** `can_move` and `show_backlog` are two independent flags for
   three shapes.

## Background — reconciled against the code, 2026-09-13

Per the practice RFC 003 established and RFC 004b followed: checked before
writing.

**What the sketch says and what is true:**

| D-4 sketch | State |
|---|---|
| "Endpoints: `/plan/add` and `/plan/remove` — shipped at 0.22.0, not pending" | **Correct.** Both routes exist and both are exercised by the plain-form path |
| "No optimistic-lock — `sprint_issues` is a join table" | **Correct**, and verified again here |
| "`can_write()` gates both POSTs; a viewer sees the plan read-only; move controls exist only on a `Planned` sprint" | **Correct**, and the module doc records the three shapes as a table |
| "Keyboard: Tab to issue, Space to move (cycles through destination columns; Enter confirms)" | **Questioned.** See open question 1 — the buttons are already the keyboard path |

**One fact neither the sketch nor the umbrella records, and it changes what
this substep can claim.** `static/board.js` — the only drag this product ships
— contains **no touch or pointer event handling at all**: zero
`touchstart`, `touchmove` or `pointerdown` handlers. HTML5 drag-and-drop does
not fire for touch input, so **the board's drag does not exist on a phone**,
and `NFR-A11Y-006`'s mobile verification passed through the *form* path rather
than the drag.

A sprint-plan drag built the same way will be mouse-only for the same reason.
That is acceptable — the buttons are the touch path and they work — but it must
be stated rather than discovered, and **no substep may describe a drag as the
way a phone user performs an action.** This RFC adds that as a cross-cutting
observation for D-3 and D-5 to inherit.

## Requirements

1. **No user-visible string is authored inside the new script.** Every
   string — the toast, the announcement, any conflict notice — comes from
   `peisear-i18n` through the JSON-island pattern `dm.js` established at
   0.25.0 and `JS-003` consolidated at 0.30.0. This is the item that mattered
   most in D-2 and it is the item that matters most here.
2. **The drag is derived from `can_move`**, the same flag the move buttons
   read. A drag handle that appears where a button does not is a second,
   divergent answer to "may this user move this issue" — the shape RFC 009 §D1
   exists to prevent.
3. **The buttons stay.** They are the no-JS path, the keyboard path and the
   touch path. This substep adds an affordance; it removes none.
4. **Undo toast, 5 seconds, inverse mutation** (umbrella requirement 4): the
   inverse of add is remove and of remove is add. No celebratory language
   (requirement 7).
5. **Optimistic update with rollback** (requirements 2, 2a, 3): the card moves
   on the client, the POST goes out behind it, and a failure **before the
   mutation lands** reverts the card and falls back to the plain form. The
   fallback catch ends where the mutation is confirmed — the corrected form
   `STATUS-002`'s review established, which this substep inherits rather than
   rediscovers.
6. **No optimistic lock is introduced.** Not "a lock that always passes" — no
   `client_updated_at`, no comparison, no 409 branch. If a reviewer thinks one
   is needed, that is an escalation, because it would mean the join table's
   concurrency story is not what §1's point 1 says.
7. **`sprint_plan`'s twelve tests pass unchanged**, and the no-JS path is
   untouched.

## Design

### D1 — What moves, and what the server already guarantees

Two mutations, both existing: `POST /plan/add` with an `issue_id`, and
`POST /plan/remove` with an `issue_id`. Both gated by `can_write()`, both
refusing on a non-`Planned` sprint. The client sends the same body the form
sends.

**Concurrency without a lock.** Adding an issue already in the sprint, or
removing one already out, is idempotent in effect. Two people planning the
same sprint converge rather than conflict, which is why the join table carries
no `updated_at` and why requirement 6 forbids inventing one.

### D2 — The three shapes, from one flag

| Sprint status | Role | Backlog | Move buttons | **Drag** |
|---|---|---|---|---|
| Planned | admin / member | shown | shown | **attachable** |
| Planned | viewer | shown | hidden | **not attached** |
| Active | any | shown | hidden | **not attached** |
| Completed | any | hidden | hidden | **not attached** |

The script attaches only where `can_move` is true. The mechanism should be a
data attribute the server renders from that same flag, so the JavaScript never
recomputes an authorisation decision — `JS-001`'s distinction between movable
policy and irreducible mechanics, applied before the code is written rather
than audited afterwards.

### D3 — What must not change

- The move buttons' markup and behaviour.
- `PLAN-001`'s filter form, which is a `GET` and unrelated.
- The `<select>` in the backlog filter, which `LAYOUT-007` gave `min-w-0` for
  a reason recorded in `§10.25`.
- `NFR-A11Y-007`: any new drag handle is an interactive element and takes
  `grow()` like every other, or it is a finding.

## Test plan

- **Rust, no-JS**: `sprint_plan`'s existing twelve tests cover add, remove and
  the three shapes. They must pass unchanged. If this substep needs a new one,
  it is for the data attribute that carries `can_move` to the client — a
  markup assertion, which is the kind this suite can make.
- **The drag itself is not covered by `cargo test`**, and this RFC says so
  rather than implying otherwise: `§10.15` records that the shipped JavaScript
  is executed by no test, and it is open **permanently by decision**. What the
  suite can assert is the island's contents and the attribute's presence.
- **`BROWSER-001`** sweeps the sprint plan page as of `LAYOUT-008`. A drag
  handle that widens the row would turn a cell red, which is the gate doing its
  one job.

## Security and privacy considerations

`can_write()` is enforced server-side on both endpoints and this substep does
not touch that. The client-side flag governs whether an affordance is offered,
never whether a mutation is allowed. A `viewer` who forges a POST is refused by
the handler, as they are today.

## Out of scope

- Reordering within the sprint column. That is D-5 and it needs a `sort_order`
  column.
- Drag on touch. It does not work, for the reason in §Background, and the
  buttons are the touch path.
- Any change to the sprint lifecycle or to what a completed sprint shows.

## Open questions — both settled at acceptance, 2026-09-13

**1. The sketch's keyboard binding is not built.** "Tab to issue, Space to
move (cycles through destination columns; Enter confirms)" would add a second
keyboard idiom to a screen whose buttons already satisfy requirement 1. D-2
asked the same question and answered it the same way: the per-card form stays
the keyboard path and no keyboard drag is built. **The sketch's binding is
withdrawn**, and `RFC 004`'s D-4 sketch should be read with that line struck.

*The reason it is withdrawn rather than deferred*: a second idiom is a
permanent maintenance cost and a permanent thing to explain, bought for a user
who is already served. If someone later finds a keyboard user reaching for a
drag, that is evidence and this can be revisited on it.

**2. One toast at a time, replaced rather than stacked.** The umbrella
requires an undo toast of every substep, and on a bulk-planning screen a user
may make eight moves in twenty seconds. Eight stacked toasts would bury the
screen the user is working on. **Each new move replaces the toast and the
replaced move is no longer undoable through that path** — the user can still
issue the inverse move with the buttons, which is what requirement 4's own
last clause already says happens after five seconds.

*What this costs, stated rather than discovered*: a fast user loses the undo
for every move but the last. That is the same bargain the five-second timeout
already makes, applied to a second axis, and the alternative — a stack — trades
it for a screen the user cannot see past.

## References

- [RFC 0004](../proposed/004-direct-manipulation.md) — umbrella, cross-cutting
  requirements 0–9 and the substep contract
- [RFC 0004b](../done/004b-direct-manipulation-board.md) — the substep this one
  most resembles
- [RFC 0001](../done/001-sprint-planning-page.md) — the screen and its endpoints
- `.git-exclude/tasks/architect/018-0.34.0-scope-proposal.md` §3
