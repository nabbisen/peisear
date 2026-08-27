# RFC 0011: Browser verification — deciding what it buys before buying it

**Status**: Proposed
**Target**: 0.29.0 for step 1 (an inventory); steps 2-4 across 0.30.0-0.32.0
**Related spec sections**: `SPEC §30` (ABDD axes), `SPEC §33` (mobile)
**Related requirements**: `NFR-A11Y-001` (focus visibility residue),
`NFR-A11Y-006`, `NFR-A11Y-007`
**Closes if adopted**: baseline `§10.15` — by shrinking it, then re-asking
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

**(a) Adopt.** Close `§10.15`, unblock three items, accept a
non-deterministic gate and a dependency.

**(b) Decline, and say so properly.** `§10.15` stays open **by decision rather
than by default**, with `DEC-021`'s degradation named as the mitigation it
actually is.

**(c) Adopt narrowly.** One job, the three scripts' happy paths and the
fallback boundary.

**(d) Shrink what cannot be verified, then decide about what is left.**

## Recommendation: (d), and the first three were the wrong question

*Revised 2026-08-27 after the owner rejected a cost-shaped recommendation.*

The first three options all ask **"how much browser should we buy?"** That
takes the 756 lines as given and shops for a tool to point at them. The
question worth asking is **what belongs in JavaScript at all.**

### The measurement that changes the answer

`dm.js` is **36 decision points against 24 DOM operations.** It is more policy
than mechanics — and the policy is the part that matters:

- fall back to a native submit, or announce and stop?
- `409` conflict, or non-`409` unavailable?
- did the change land before the failure, or after?
- which announcement, in which live region?

**Every one of those is a rule, not a manipulation.** None of them needs a
browser to be true or false. They need a browser only because they are
currently written where nothing else can reach them.

### The precedent is this project's own

**RFC 006 moved copy out of Rust into a message table so it could be
checked.** Before it, every string was correct by review; after it, a violation
is unconstructible. The strings did not become easier to test — they moved to
where testing was possible.

**`QA-019` did the same for `updated_at`**: one authority, in the layer that
can enforce it. **`HLT-001` did it again**: return the set rather than
re-derive it, so a count and its basis cannot disagree.

**The pattern this project keeps arriving at is *move the fact to where it can
be checked*, never *add a checker where the fact is*.** A browser harness is
the second of those, and it is the first time this project would have chosen
it.

### The shape

**Policy becomes data.** The `board-copy` JSON island already carries copy from
Rust to the scripts. It carries **decisions** too: a table of
outcome → action, computed and tested in Rust, that the scripts look up rather
than encode.

The scripts keep what genuinely needs a browser — reading a `fetch` result,
moving a node, starting a timer — and stop holding the rules about *when*.

**What that leaves uncovered is DOM mechanics**, where a failure is visible
immediately and locally rather than subtle and conditional. That is a
categorically better residue than the fallback boundary, which
`STATUS-002`'s review caught by reading and which reading catches once.

### Then, and only then, the browser question

After the shrink the question is no longer "do we need a browser for 756 lines
of policy and mechanics" but "do we need one for N lines of mechanics" — a
smaller question with a cheaper answer, and one that can be answered against a
real number rather than a fear.

**I am not pre-deciding it.** It may still be yes.

## Schedule

Controlled, with a review point between each step and nothing committed past
the next one.

| | Release | What | Exit condition |
|---|---|---|---|
| **1** | 0.29.0 | **Inventory.** Classify every decision in the three scripts as *movable policy* or *irreducible mechanics*, with the count. No code moves. | A table the owner can read, and a number for what would remain |
| **2** | 0.30.0 | **Move `dm.js`'s policy**, the highest-risk file and the one whose boundary has already been got wrong once. Rust-side tests for every rule moved. | `§10.15`'s entry updated with the new residue |
| **3** | 0.31.0 | **Move `board.js`'s and `search.js`'s.** | The residue is mechanics only |
| **4** | 0.32.0 | **Re-ask the browser question** against the measured residue. | A decision, recorded either way |

**Step 1 is the only thing being asked for now.** It is an audit, it costs one
handoff, and its output is the input to a decision that is currently being made
on an estimate.

**If step 1 finds the policy is not movable** — that the decisions are entangled
with DOM state in ways that do not survive extraction — that is a finding, and
it returns the browser question immediately with the estimate replaced by
evidence.

## Out of scope

- No harness is built by this RFC.
- No change to `DEC-021`.
- No change to the four `NFR-A11Y-006` flows or the touch-target target.

## References

- Baseline `§10.15`, `§10.13`, `§10.16`
- RFC 005 §5, §6 — the audits that hit this limit
- `QA-011` §0 — the markup-assessment labelling this RFC would make unnecessary
