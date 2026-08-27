# RFC 0011: Browser verification — deciding what it buys before buying it

**Status**: Proposed
**Target**: 0.29.0 for the decision; implementation 0.30.0 at the earliest
**Related spec sections**: `SPEC §30` (ABDD axes), `SPEC §33` (mobile)
**Related requirements**: `NFR-A11Y-001` (focus visibility residue),
`NFR-A11Y-006`, `NFR-A11Y-007`
**Closes if adopted**: baseline `§10.15`
**Governing decisions**: `DEC-021` (JavaScript as progressive enhancement)
**Last updated**: 2026-08-27 — reconciled against the code before drafting

## Summary

Three open items share one blocker: **this project cannot execute a browser.**
`§10.15` records 756 lines of shipped JavaScript that no test runs; RFC 005 §6's
mobile flows were assessed from markup and labelled as such; `NFR-A11Y-001`'s
audit could not confirm a focus ring is visible.

**This RFC is a decision, not a design.** Its purpose is to state what a
harness would and would not buy, at what cost, so the owner can decide — and to
record the answer either way, because "not scheduled" has been the answer for
three releases by default rather than by choice.

## Background — what is actually uncovered

| | Lines | What no test executes |
|---|---|---|
| `dm.js` | 262 | in-place status update, 5-second undo, `409` path, fallback boundary |
| `board.js` | 236 | drag, drop, undo, conflict revert, reload-on-stale |
| `search.js` | 258 | typeahead fetch, dropdown, keyboard interaction |

**What *is* covered, and it is more than it sounds.** Every endpoint these
scripts call; the response shape they depend on; every string they render, held
in the message table and checked by the vocabulary guard; the `<script>` tags
that load them (`QA-003`); and — the load-bearing one — **the server-rendered
path each enhances**, which `DEC-021` requires to work without them.

So a total failure of all three files degrades to tested behaviour. **The
uncovered surface is the enhancement, not the function.**

## What a harness would buy

1. **`§10.15` closes properly.** The three scripts' own behaviour becomes
   assertable — including the fallback boundary `STATUS-002` had to correct in
   review, which is currently held by reading.
2. **`NFR-A11Y-006` becomes testable.** Four flows at a narrow viewport, which
   RFC 005 §6 could only assess from markup.
3. **`NFR-A11Y-001`'s residue closes** — whether a focus ring is actually
   visible, which markup cannot establish.
4. **0.30.0's touch-target pass gets a way to verify a relayout**, rather than
   asserting class names and hoping.

## What it would not buy, stated first because it is usually stated last

- **It is not a device.** A simulated viewport is not a phone: no real touch
  target, no on-screen keyboard, no thumb reach. `NFR-A11Y-006`'s four flows
  would be verified *at a width*, not *on a device*, and the requirement's
  wording — *"completable on a phone"* — would still be one inference away.
- **It is not a screen reader.** `NFR-A11Y-003` and `-008` would remain verified
  by markup: that a live region exists and carries the right politeness, not
  that anything announces.
- **It does not make the JavaScript correct.** It makes it *assertable*. The
  fallback boundary, the undo window, the conflict revert — each still needs a
  test someone thought to write.
- **It adds a failure mode this project does not have.** Every guard here is
  deterministic. A browser harness is the first thing in this tree that can
  fail for reasons unrelated to the code, and `§10.13` is on record about what
  a flaky gate does to a team: it trains people to re-run rather than read.

## Cost

- **A dependency and a runtime**, in a project whose whole shape is one binary
  and a SQLite file, and whose `Cargo.toml` deliberately carries no cloud SDK
  (`NFR-CMP-002`).
- **CI time.** 32 jobs today, all fast. A browser job is not.
- **Maintenance.** The harness needs its own fixtures, and it is the only part
  of the suite that would need updating when a class name changes.

## The options

**(a) Adopt.** Close `§10.15`, unblock three items, accept a non-deterministic
gate and a dependency.

**(b) Decline, and say so properly.** `§10.15` stays open **by decision rather
than by default**, with `DEC-021`'s degradation named as the mitigation it
actually is. The three items stay open with their reasons recorded. **This is
not the same as the current state**, which is three releases of "not
scheduled".

**(c) Adopt narrowly.** One job, the three scripts' happy paths and the fallback
boundary — not the mobile flows, not accessibility. Buys the largest single
item; leaves the ones a browser is worst at.

## Recommendation

**(c), and I hold it loosely.**

`§10.15`'s real exposure is the fallback boundary: *"falling back to a native
form submit is correct before the server has applied the change and wrong
after"* — a distinction `STATUS-002`'s review caught by reading, and the kind
of thing reading catches once. That is worth a browser.

The mobile and accessibility items are where a browser flatters itself most: it
answers *"does this work at 375px in a headless Chromium"*, and the requirement
asks about a phone. **Buying a browser to answer those would let us mark them
verified on evidence that is one inference from what was asked** — and this
project has spent twenty-one handoffs finding claims of exactly that shape.

## Open questions

1. **(a), (b) or (c)?** The decision this RFC exists for.
2. **If (b), does `§10.15` get a recorded review date** rather than staying
   open indefinitely?
3. **If (a) or (c), is a non-deterministic gate acceptable at all**, given
   `§10.13`? A browser job that fails one run in fifty is worse than no job if
   the response is to re-run it.

## Out of scope

- No harness is built by this RFC.
- No change to `DEC-021`.
- No change to the four `NFR-A11Y-006` flows or the touch-target target.

## References

- Baseline `§10.15`, `§10.13`, `§10.16`
- RFC 005 §5, §6 — the audits that hit this limit
- `QA-011` §0 — the markup-assessment labelling this RFC would make unnecessary
