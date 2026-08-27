# JS-001 — What in these scripts is policy, and what is a browser's job

**Issued by**: Architect
**Date**: 2026-08-27
**Priority**: P2 — an inventory; it decides how a P1 gap gets closed
**Governing RFC**: [011](../../accepted/011-browser-verification.md), step 1
**Depends on**: nothing.

**No code moves. No test is added. Nothing is refactored.** This is a
classification whose output replaces an estimate in a decision currently being
made on one.

---

## 1. Why this exists

`§10.15` — 756 lines of shipped JavaScript that no test executes — has been
open since 0.26.0 and *"not scheduled"* for three releases. The obvious
closure is a headless browser in CI.

**The owner rejected buying a tool to point at the problem before asking what
belongs in the problem.** RFC 011's answer: this project's own pattern is
**move the fact to where it can be checked**, not **add a checker where the
fact is** — RFC 006 moved copy to a message table, `QA-019` moved `updated_at`
to one authority, `HLT-001` returned the set rather than re-deriving it.

**A rough measurement started this**: `dm.js` looks like 36 decision points
against 24 DOM operations. **That number is mine, it is a grep, and this
handoff exists partly because I do not trust it** — nine times in this project
a pattern of mine has answered a narrower question than the one I asked it.
**Reproduce it or contradict it.**

## 2. What to classify

Every decision in `static/dm.js`, `static/board.js`, `static/search.js`. For
each, one of:

- **Movable policy** — a rule whose truth does not depend on the DOM or on a
  live network result. *"A `409` is a conflict; a non-`409` failure after the
  change landed is unavailable, and must never re-submit."* Rules like that can
  be computed and tested in Rust and handed to the script as data.
- **Irreducible mechanics** — reading a `fetch` result, moving a node, starting
  a timer, matching a selector. A browser is the only thing that can check it.
- **Guard clause** — `if (!copyEl) return;`. Neither: defensive, untestable in
  any useful sense, and should not inflate either count.

**Report the three counts per file, and the movable ones individually** — one
line each, saying what the rule is. That list is the input to step 2 and the
reason this handoff exists.

## 3. The two answers that would change the plan

**3.1 — If the policy is entangled and does not survive extraction**, say so
plainly and show one example in detail. That returns the browser question
immediately, with the estimate replaced by evidence, and RFC 011's steps 2–4
are withdrawn rather than attempted.

**3.2 — If the movable fraction is small** — if `dm.js` is really mechanics
with a few guard clauses and my 36 was counting `if (!x) return;` — **say
that.** It is the most useful outcome this handoff can have, and it is the one
I have staked a recommendation against.

**I would rather be contradicted here than at step 2.**

## 4. What a "movable" rule has to satisfy

Before classifying something movable, check it against the shape that already
exists: the `board-copy` JSON island carries copy from Rust to the scripts.

- Can the rule be expressed as **data** — a mapping from an outcome the script
  can observe to an action it should take?
- Can the script **name the outcome** without knowing the rule? (`res.status
  === 409` is an observation; *"a 409 means revert and reload"* is a rule.)
- Would a Rust test of the rule assert something a reader would recognise as
  the requirement?

**If a rule needs the script to already know the policy in order to report the
outcome, it is not movable.** Say so and count it as mechanics.

## 5. The one rule I want traced individually, whatever the counts say

**`dm.js`'s fallback boundary.** *"Falling back to a native form submit is
correct before the server has applied the change and wrong after."*

That is `STATUS-002`'s cross-cutting requirement 2a, added in review after the
first round got it wrong. It is currently held by reading, and reading catches
that once.

**Trace it end to end**: where the script decides, what it observes to decide,
and whether that decision could be data. **If exactly one rule in these three
files is worth moving, my belief is that it is this one** — and if the trace
shows it cannot move, that is the single most important sentence this handoff
can produce.

## 6. Not in scope

- **No refactoring.** Not one line of the three scripts changes.
- **No harness, no dependency, no CI change.**
- **No test.** Adding one would require the very thing this inventory exists to
  decide about.
- **No design for step 2.** Classify; do not propose the mechanism.

## 7. Escalate rather than deciding

- If a script turns out to hold a rule that **contradicts** a server-side one,
  stop and report — that is a live defect, not an inventory finding.
- If `search.js` turns out to be a different shape from the other two (it
  predates `DEC-021`'s framing and is the only one not covered by `QA-003`'s
  reference tests), say so rather than forcing it into the same table.

## 8. Acceptance

1. Three counts per file — movable, mechanics, guard — with my `dm.js` figure
   reproduced or contradicted.
2. Every movable rule listed individually, one line each.
3. §5's fallback boundary traced end to end.
4. §3.1 and §3.2 each answered, even if the answer is "neither applies".
5. **Nothing changed**: `git status --short` empty.
6. fmt and clippy exit 0; three consecutive `cargo test --workspace` runs
   (unchanged count expected).

## 9. Required review-request format

Workflow §9.2. The movable list as prose, one line per rule. §5 as a trace,
not a verdict.
