# GATE-005 — what should the browser gate cover?

**An investigation. The deliverable is a proposal, not tests.**
**Requirements**: `NFR-A11Y-008` (P1). **Register**: `§10.15`, `§10.17`,
`§10.35`, `DEC-048`. **Target**: 0.47.0.

**Two items have now been deferred into the same question** and it is time to
answer it once:

- **`NFR-A11Y-008`'s three untested live-region surfaces** (`§10.35`'s gap).
  Its regions are **updated by script**, so a Rust test can assert a region
  *exists* and **cannot** assert that anything was announced.
- **The real-drag-gesture test**, deferred at `DM-TEST-001` under `§10.15`.

**The reason both were held**: writing a Rust test that asserts a
script-updated region *exists* and then recording `NFR-A11Y-008` as covered
**would build `§10.35`'s own defect one release after naming it** — an
acceptance narrower than the requirement, cited as if it were not.

---

## §0 — what exists, so you are not re-deriving it

`browser-checks/` holds **two gates in one CI job** (`DM-TEST-002` §2):

- **`overflow-gate.mjs`** — 24 pages × 5 widths, **one assertion**
  (`scrollWidth <= clientWidth`). `DEC-048` condition 3 says *one assertion
  only, do not widen this gate*, and that governs **this script's own
  assertion set**, not what else runs in the job.
- **`undo-mousedown-trap.mjs`** — real `Input.dispatchMouseEvent` sequences
  across the three draggable surfaces; the one property no source scan can
  observe, which is **the browser's own drag-versus-click decision**.

`cdp.mjs` is the harness: one tab over CDP, Node's built-in `WebSocket` and
`fetch`, no npm dependency, **three known limits in its own doc comment** —
one page at a time, waits on `document.readyState` only, no notion of
animation completion. **Read those three before proposing anything**; they
bound what is buildable.

**And `RFC 011` declined to buy a JavaScript test harness.** That decision
stands and this investigation does not reopen it. The question is **what the
gate we already own should cover**, not whether to acquire a different one.

## §1 — the question

> **Which properties of the shipped JavaScript are worth a browser gate, and
> which are honestly out of reach?**

`§10.15` records the standing position: **1,909 lines in five files executed
by almost no test**, the gap declared permanent, its *size* not. Two
compensating evidence runs are documented in
`docs/static-js-verification.md` and **each has caught one defect no gate
could**.

So this is not *how do we test the JavaScript*. It is: **given one CDP
harness, two existing gates, and a documented refusal to buy more, what is
the next most valuable thing a browser gate could assert — and what should be
left to the evidence runs?**

## §2 — what to produce

**A ranked proposal, three to five candidates**, each with:

1. **The property**, stated as an assertion — what would be true or false.
2. **Why no Rust test can reach it.** If a Rust test *can*, it is not a
   candidate for this gate; say so and move it.
3. **What it would catch that nothing catches today**, concretely. `§10.15`'s
   table does this for the two evidence runs — *"a row marker that never
   flipped"*, *"a block's height collapsing from 59.875 px to 20 px"* — and
   that is the standard.
4. **Cost**: one script or an extension of an existing one; whether it needs a
   capability `cdp.mjs` lacks; roughly how long.
5. **Whether it fits the one job**, and whether its server can coexist — the
   two current scripts use ports `4173` and `4174`, each tearing down its own,
   and `browser-checks/README.md` records that **`PEISEAR_PORT` is read by
   both**, so parallelising them collides.

**`NFR-A11Y-008` must be one of the candidates**, because it is the item that
forced this question. For it specifically: **can a CDP gate observe that a
live region received text after an action** — not that the region exists, but
that something was put in it? If `cdp.mjs` cannot, say so plainly; that is a
finding, and it decides whether `NFR-A11Y-008`'s acceptance can ever cite a
mechanism matching its text.

**The real-drag-gesture test must be another**, with an honest estimate.
`DM-TEST-001` called it *medium-high* and I deferred it saying the structural
findings had taken most of its value. **Re-derive that judgement rather than
inheriting it** — `CAL-004` added a form path since, which may change what a
drag test would still prove.

## §3 — what this must not become

- **Not a recommendation to buy a harness.** `RFC 011` settled that. If you
  believe the answer requires one, that is an **escalation to the owner**, not
  a proposal in this report.
- **Not a widening of `overflow-gate.mjs`.** `DEC-048` condition 3 holds.
  A new property is a **new script in the same job**, which is the precedent
  `undo-mousedown-trap.mjs` set.
- **Not a plan to cover 1,909 lines.** The gap is permanent by decision. The
  question is which **named property** earns a gate next.
- **Not tests.** Write none. If a candidate is so cheap that building it
  seems faster than describing it, **describe it anyway** — the ranking is the
  deliverable and I need it before any of them is built.

## §4 — the honest-negative outcome

**If the answer is that nothing here earns a gate beyond what exists, that is
a complete result** and it closes `NFR-A11Y-008`'s question differently: its
acceptance would then be amended to **state its own limit** — the region's
presence is asserted, the announcement is not, and `§10.15` owns the
remainder. `FR-DM-006` already carries exactly that shape, naming two
instruments and what neither reaches.

**`REQ-003` is the precedent**: its deliverable was *no*, and it was the more
useful answer. **Do not manufacture a candidate to justify the
investigation.**

## §5 — gates

Nothing is built, so: `DEC-007` confirmed once at its current figure,
unchanged, and an empty diff. **If you run a CDP experiment to establish
whether something is observable, keep the script in the review package rather
than in `browser-checks/`** — an unwired script beside gated ones reads as
coverage, which is the reasoning that wired `undo-mousedown-trap.mjs` in the
first place.

## §6 — exit condition

A ranked proposal of three to five candidates with the five fields above;
`NFR-A11Y-008` and the drag gesture both among them; a plain answer on whether
`cdp.mjs` can observe a live-region update; and **the stated limits of your
own assessment**, which in this thread has twice been worth more than the
findings.

**`§10.35` closes when this and its two siblings land** — `PER-001` and
`HLT-003` write tests; this one decides whether `NFR-A11Y-008` gets a
mechanism or an honest limit.
