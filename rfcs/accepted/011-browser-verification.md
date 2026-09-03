# RFC 0011: Browser verification — deciding what it buys before buying it

**Status**: Accepted
**Target**: steps 1 and 1b shipped in 0.29.0; step 2 is **done but unreleased**, shipping in 0.30.0; steps 3-4 at 0.31.0 and 0.32.0
**Related spec sections**: `SPEC §30` (ABDD axes), `SPEC §33` (mobile)
**Related requirements**: `NFR-A11Y-001` (focus visibility residue),
`NFR-A11Y-006`, `NFR-A11Y-007`
**Closes if adopted**: baseline `§10.15` — by shrinking it, then re-asking
**Governing decisions**: `DEC-021` (JavaScript as progressive enhancement);
introduces `DEC-048` (conditional acceptance of a non-deterministic gate)
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

### The measurement — corrected 2026-08-27 by `JS-001`

**The figure this section originally carried was wrong.** It said `dm.js` was
*"36 decision points against 24 DOM operations — more policy than mechanics."*

The 24 reproduces exactly. **The 36 does not.** It came from a grep matching
any line containing `if`, `else`, `return` or `throw`, and `return`/`throw` are
not decisions. The real figure is **23 branch points**, and they divide:

| | movable policy | mechanics | guard clause |
|---|---|---|---|
| `dm.js` | 6 sites / **4 rules** | 5 | **12** |
| `board.js` | 9 sites / **4 rules** | 1 | 12 |
| `search.js` | 2 | 16 | 4 |

**Guard clauses are the largest bucket in both files that matter** — 52% and
55%. Across all three scripts there are roughly **ten distinct movable rules**,
and three of `board.js`'s four are the same rules `dm.js` enforces against the
same endpoint.

That is a small, concrete number. It is not "more policy than mechanics."

**Ten rules is still worth moving** — a `409`-vs-other classification written
twice in two scripts is two homes for one fact, and this project has closed
that shape six times. But the size is what it is, and this section previously
overstated it by a factor the owner was asked to schedule against.

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

### The rule I staked this on cannot move — and that is the finding

`dm.js`'s fallback boundary — *"falling back to a native submit is correct
before the server has applied the change and wrong after"* — was named in the
step-1 handoff as the one rule most worth moving.

**`JS-001` traced it and it does not survive extraction.** `applyChange` holds
its own `try { … } catch { announce; reload }`. The guarantee is that an
exception raised inside it unwinds into *that* handler and therefore never
reaches the outer `.catch()` that calls `fallback()`. **There is no boolean
anywhere.** The property is a fact about which handler is nested where.

A `mutationConfirmed` flag would make the rule *legible* — one branch instead
of two nested handlers — and would not make it testable: the branch stays in
JavaScript, and a Rust test would assert something no shipped code consults.
**Legibility is not testability.**

**So moving policy does not protect the thing this RFC most wanted protected.**
Steps 2–3 buy deduplication and about ten Rust-testable rules. That is worth
doing and it is not what was claimed for it.

### What the trace surfaced instead — step 1b

The regression `JS-001` names is *"someone flattening the two `catch` blocks
back into one, or removing `applyChange`'s inner `try`"*. **That is a shape,
not a behaviour.**

This project has seven guards that assert shapes in source text, and
`static_js_scan` already reads these files. A guard asserting that
`applyChange` contains its own `try`/`catch`, and that `fallback(` is called
**outside** it, is deterministic, needs no browser, costs one test, and catches
exactly the regression that `STATUS-002`'s review caught by reading — which
reading catches once.

**It does not test that the boundary works. It pins the structure the boundary
is made of** — the same claim `test_harness_scan` makes about clock-derived
temp paths: not that the harness is correct, but that the shape which made it
incorrect cannot return.

**Step 1b comes before any policy moves**, because it is the cheapest item here
and it protects the one thing the rest of the plan turned out not to.

### Then the browser question

After 1b and the shrink, what remains uncovered is DOM mechanics plus one
control-flow guarantee now pinned by a guard. **The question at step 4 is
narrower and better posed than the one this RFC opened with.**

**I am not pre-deciding it.** It may still be yes.

## Schedule

Controlled, with a review point between each step and nothing committed past
the next one.

| | Release | What | Exit condition |
|---|---|---|---|
| **1** | 0.29.0 | **Inventory.** ✅ Done — `JS-001`. Corrected the count, found ~10 movable rules, and established that the fallback boundary is not one of them. | — |
| **1b** | 0.29.0 | **Pin the fallback boundary's shape.** ✅ Done — `JS-002`. Three assertions: the function exists by name, its body carries a `try` at its **own** depth, and `fallback(` is never called inside it. A nested-callback `try` was found to defeat the first version and was closed in review. | The two-catch structure cannot be flattened silently. **Residual**: a *narrowed* top-level `try` still passes — closing it would need a parser, or a rule that fails on the current tree |
| **2** | 0.30.0 | **Move `dm.js`'s four rules and `board.js`'s duplicates of them.** ✅ **Built, not yet released** — `JS-003`, merged after the 0.29.0 tag. The `409`/other-failure/malformed-body classification moved into the copy island both scripts read, built by one shared function, `conflictStatus` derived from a real `AppError::OptimisticLockConflict`. **Movable sites 15 → 3.** Settled the malformed-body asymmetry in `dm.js`'s favour and closed a latent stale-lock defect with it. Two review rounds: the reload flags were policy moved into Rust that nothing checked, and `unconfirmed` reusing `unavailable`'s copy asserted an outcome the code cannot support — both architect errors. | ✅ `§10.15` updated with the new residue |
| **3** | 0.31.0 | **`board.js`'s remaining rule** (the stale-card case). **`search.js` is excluded** — different shape, and its two "movable" rules fail the purpose: the server has no query-length floor, so moving `MIN_QUERY_LENGTH` would *invent* a second authority rather than remove one. | The residue is mechanics only |
| **4** | 0.32.0 | **Re-ask the browser question** against the measured residue. | A decision, recorded either way |

**Step 1 was the only thing asked for at the time of writing.** It was an audit,
it cost one handoff, and its output was the input to a decision then being made
on an estimate.

*Superseded 2026-08-27.* Steps 1, 1b and 2 are all built. **1 and 1b shipped in
0.29.0; step 2 did not** — `JS-003` merged after that tag and is unreleased.
**Step 3 is not yet authorised**; the review point between steps still stands.

*Correction, 2026-08-27.* An earlier amendment to this table (commit `2bf34a9`)
claimed step 2 shipped at 0.29.0 and retargeted the row from 0.30.0. **That was
wrong in the direction that matters**: the original 0.30.0 target was correct,
and the "correction" introduced the error. `JS-003`'s commits sit after
`de521d2`, the 0.29.0 release commit — visible from `git log 0.29.0..HEAD`,
which is the check that should have been run before rewriting a schedule.

**Step 1 found exactly the case it was told to look for.** The handoff said:
*"If the movable fraction is small — if `dm.js` is really mechanics with a few
guard clauses and my 36 was counting `if (!x) return;` — say that. It is the
most useful outcome this handoff can have, and it is the one I have staked a
recommendation against."* It was, and they did.

**One decision in `board.js` is left open by the inventory rather than settled
by it.** A malformed `2xx` body is announced as unavailable by `dm.js` and
passed over in silence by `board.js` — the same failure class, two behaviours.
Step 2 forces the choice. **`dm.js` is right**: a mutation that failed and says
nothing is the worse default, and `NFR-A11Y-008`'s assertive region exists so a
failure can be heard.

## Decisions taken 2026-08-27

**Step 1 is approved.** Nothing past it is committed.

### `DEC-048` — a non-deterministic gate is accepted, conditionally

The owner's words: *"the non-deterministic gate is accepted if there is a
concrete plan to solve it at good timing."*

**This is an acceptance with an obligation attached, and the obligation is the
architect's.** It is recorded as a decision rather than a note because
`§10.13` is on record about what a flaky gate does — *"a repeated run is not a
reliable detector for a deterministic property"* — and an acceptance without
the plan would be that lesson unlearned.

**The plan, due at step 4 and binding on it:**

1. **No external network.** The harness drives a locally-spawned instance, as
   `TestApp::spawn` already does. A gate that can fail because a CDN is slow is
   not acceptable at any timing.
2. **No wall-clock dependence.** The undo window is five seconds; a test that
   waits for it is a test that fails on a loaded runner. Time is controlled, or
   the behaviour is not tested this way.
3. **A flake is a defect with an owner, not a re-run.** Any browser test that
   fails and then passes unchanged is **quarantined the same day**, dated, and
   either converted into a deterministic assertion or deleted. **Silent retries
   are forbidden** — `§10.13`'s defect survived four releases because
   re-running was the response.
4. **A quarantined test is not coverage.** If the quarantine list is non-empty
   at a release, that release's candidate names it, and it is not counted in
   any total.

**If that plan cannot be met, step 4's answer is no** — and this decision is
what says so in advance, rather than after a gate has started costing
attention.

### `§10.15`'s review date

**0.32.0**, at step 4. The register entry carries that date, so it stays open
by decision rather than by inertia — which is what it had been for three
releases.

### Still open

**Nothing blocking.** Steps 2–4 are scheduled but not committed; each is
re-decided at its own release against step 1's evidence.

## Out of scope

- No harness is built by this RFC.
- No change to `DEC-021`.
- No change to the four `NFR-A11Y-006` flows or the touch-target target.

## References

- Baseline `§10.15`, `§10.13`, `§10.16`
- RFC 005 §5, §6 — the audits that hit this limit
- `QA-011` §0 — the markup-assessment labelling this RFC would make unnecessary
