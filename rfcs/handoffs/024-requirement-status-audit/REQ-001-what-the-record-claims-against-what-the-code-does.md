# REQ-001 — what the record claims, checked against what the code does

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none — a record defect.
**Depends on**: nothing. **Do this first** of the three 0.40.0 handoffs: what it
finds may outrank the other two.

**Report, do not amend.** Every correction to `requirements.md` is mine. Your
output is a list of discrepancies with evidence; mine is the wording.

---

## 1. Why this is worth a release slot

**Four times in three days a requirement's recorded status was wrong, and twice
it cost real work.**

- `FR-DM-001` read *"two of five"* while four had shipped — and its **own
  correction note** says the earlier stale status *"is what allowed
  `FR-DM-002` to be violated unnoticed — a requirement believed dormant is not
  checked."* It then went stale again, in the same requirement.
- `FR-SPR-002` read *"Implemented (untested)"*. It was **not correctly
  implemented**: the rule was a read on one connection and a write on another,
  and twelve simultaneous starts left six active sprints.
- `FR-SPR-004`'s text and status sent me to write `SPRINT-001` and
  `SPRINT-002`. One shipped to `main` and was half reverted 47 minutes later;
  the other was withdrawn unbuilt.

**And it is not confined to those.** Three `Specified` requirements checked at
random before writing this are all shipped: `FR-CAL-001` (`/today/calendar` is
routed to `calendar::personal_page`), `FR-NTF-005`
(`POST /inbox/mark-all-read`), and the calendar privacy footnote
(`ProjectCalendarPrivacyFootnote`). **Three for three.**

A requirement nobody believes is live is a requirement nobody checks. That is
the mechanism, stated by the document about itself, twice.

## 2. Scope — 162 requirements, and you are not reading all of them

**2.1 — The three `Implemented (untested)`.** Highest risk of the three
groups: *untested* is the document confessing it does not know, and one of the
three (`FR-SPR-002`) turned out to be false in exactly that gap. For each: does
the behaviour hold, and does any test pin it?

**2.2 — The nine `Specified` (including the two qualified `Specified (RFC …)`).**
For each: is it shipped? Three are known already — confirm and move on.

**2.3 — The three `Partial`.** For each: which part is missing, and is the
recorded description of the missing part still accurate?

**2.4 — A mechanical sweep over all 162, in the dangerous direction.** *Every
`*Acceptance*` citation names a test target — crate, file or test name. Check
each named target exists.* A citation naming a test that no longer exists is
`§10.17`'s shape and means the requirement's evidence is imaginary. **Script
this**; do not read 162 entries by hand.

**Not in scope**: judging whether an implemented requirement is *well*
implemented, or re-reading the 89 plain `Implemented` entries by hand. §2.4's
sweep is the affordable proxy for that direction, and it is deliberately
weaker than a real audit — **say so in the report** rather than letting a green
sweep read as *"the other 89 are fine."*

## 3. What a finding looks like

For each discrepancy, one row: **the requirement id, what the status says, what
the code does, and the evidence** — a route, a function, a test name, a
migration, or a measurement. **A page rendered or a test run beats a grep**,
and where you can cheaply show the behaviour, show it.

Where a requirement is **ambiguous rather than wrong** — its text admits two
readings and the code satisfies one — that is a separate list, and it is the
more valuable one. `FR-SPR-004` was in that class and nobody noticed until two
handoffs had been written from the wrong reading.

## 4. Verification

- **No code changes are expected.** If §2.1 turns up a requirement that is
  false rather than merely unrecorded — the `FR-SPR-002` shape — **stop and
  report it before writing any fix**; that is a defect and it gets its own
  handoff and its own release slot.
- **Any script you write for §2.4 is scratch** unless it is small enough to
  belong beside the other scan modules, in which case say so and I will decide
  whether it becomes one. **Do not add a scan module on your own reading** —
  that is a standing check and a maintenance cost.
- `DEC-007` unchanged at **346** unless §4's first bullet fires.
- `fmt`, `clippy` if anything is written. The overflow gate is untouched.

## 5. Escalate rather than deciding

- **A requirement that is false, not just stale** (§4).
- **A requirement whose text cannot be checked at all** because it says
  something no code could satisfy or violate. That is worth knowing and it is
  mine to rewrite.
- **If §2.4's sweep finds more than a handful**, stop at ten and report the
  rate rather than enumerating all of them; the number matters more than the
  list at that point.

## 6. Exit condition

A report naming every discrepancy in §2.1–2.3 with evidence, §2.4's sweep
result with its rate, and the separate ambiguity list. **No amendment to
`requirements.md`** — that is the architect's, from your report.
