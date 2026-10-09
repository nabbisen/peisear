# REQ-005 — the four left unresolved

**An investigation, and the shortest one in this thread.** `REQ-004` audited
48 entries carrying an `*Acceptance*` and resolved 44. **It named four it
triaged by text and test *name* only, without opening the code** — and named
them as *unresolved* rather than scoring them `Covers`. **That paragraph is
why this handoff exists**, and it was the most valuable thing in that report.

**Register**: `§10.35`, which does not close until these four are resolved and
`REQ-004`'s findings are scheduled. **Target**: 0.47.0, and **first** in it.

**Why first rather than after the citation fixes.** Two reasons, both
ordering-relevant: anything you find here **joins the same specification
amendment**, and amending the same entries in two consecutive releases is
`§10.34`'s churn, recorded one release ago; and **`NFR-PRIV-002` is a P1
privacy requirement currently sitting at *plausible under-coverage, not
confirmed***, which outranks a trigger gap whose triggers SQLite enforces
whether a test asks or not.

**Method**: `REQ-004`'s own, unchanged — requirement's normative sentence,
then the mechanism, then **the code the mechanism touches**. The third step is
the one the four did not get.

---

## §0 — do not trust my counts, and here is the evidence for that instruction

Three of my measurements failed in this thread alone, each by matching a
token rather than a meaning: `<select>` counted five times inside `//`
comments; the entry denominator undercounted 48 as 45 because my split
truncated long entries; and a live-region population of eight where five was
right, because I counted static warning banners as dynamic regions.

**`TT-007` and `REQ-004` both corrected me by measuring.** Do the same here.
Where I give a figure below, treat it as a claim to check.

## §1 — `NFR-PRIV-002` (P1, privacy) — lead with this one

**Normative text, two parts.** A permission list of shareable categories, and
then a prohibition: *"workload distribution" means each member's volume of
in-flight work. It MUST NOT include another member's capacity value, WIP
limit, or any state derived from either — **including badge colour, glyph,
tooltip, or annotation.***

**Cited mechanism**: `workload_privacy` test crate, 4 tests.

Three questions, in this order:

1. **How many categories does the permission list actually name?**
   `REQ-004` says six; **I read five**, depending on whether *"project health
   indicators and trends"* is one category or two. **Settle it from the text
   and say which reading you took** — a prohibition's scope depends on the
   permission's scope, and this is exactly the *indeterminate* verdict
   `REQ-004`'s own method allows for.
2. **Does anything assert the prohibition's four named vectors** — badge
   colour, glyph, tooltip, annotation? The clause enumerates them, which means
   the requirement itself defines the population. **Four named vectors is a
   checkable list**, and a mechanism that covers *capacity value* but not
   *badge colour* is narrower than a text that names both.
3. **`NFR-PRIV-001` governs on overlap**, by this entry's own words. If a
   vector is covered by `NFR-PRIV-001`'s mechanism instead, **that is
   coverage, and the finding is a citation one** — say so rather than counting
   it as a gap.

## §2 — `FR-API-002` (P0) — the citation may point at the wrong half

**Normative text**: *"These endpoints MUST return data only when the path
`user_id` equals the authenticated user's id."* That is a **prohibition**.

**Cited mechanism**: `self_can_read_own_*` return **200**.

**A test that self can read its own data does not test that another cannot.**
The negative is very likely covered — `FR-API-003` carries a wildcard
`/api/*` note and `auth_boundary` exists — but **the citation names the
positive case for a requirement whose force is negative.** On a **P0** that
is worth settling precisely:

- Does a test assert a **non-self** `user_id` is refused, on **each** of these
  endpoints? Name the endpoints from the entry's own list, then the tests.
- If the negative is covered elsewhere, this is a **citation** defect and the
  fix is to name what covers it.
- **If no test asserts the refusal on any of them**, that is a coverage gap on
  a P0 and it stops being a documentation question. **Escalate immediately
  rather than finishing the round** — do not fold a P0 finding into a report I
  read tomorrow.

## §3 — `FR-PER-006` (P1) — a three-tier chain cited by its empty case

**Normative text**: at most one callout, chosen by a **strict precedence
chain** — sustained burnout, then WIP over limit, then long-stale assigned
work; **if none applies, none is rendered.**

**Cited mechanism**: `today_renders_no_callout_for_fresh_user` — **the "none
applies" case.**

So the citation covers the last sentence and nothing about the chain. Check:

- Is each tier asserted **in isolation**?
- **Is any tie-break asserted** — burnout *and* WIP both applying, and only
  burnout rendering? **That is the requirement's actual content**: a
  precedence chain is only a chain where two tiers compete. A test per tier
  with no overlap tests three independent conditions, not a precedence.
- **At most one** — is that asserted at all?

`REQ-004` found the neighbouring `FR-PER-007` had a clause tested but uncited;
**the same may be true here**, so look for tests before concluding a gap.

## §4 — `FR-HLT-007` (P2) — "each indicator", with an exception already carved

**Normative text**: *each* indicator MUST offer a route to its basis — issue
list, calculation, and recent history. **Status says two of three limbs ship;
history is deferred**, and **WIP compliance is excepted** by owner-approved
amendment with *a test asserting the absence*.

**Cited mechanism**: `basis_route`, 5 tests.

The population is *indicators × shipped limbs*. So:

- **How many indicators are there?** The entry's own amendment says *"one of
  six cases"*. Confirm from the code, not from that phrase.
- Six indicators × two shipped limbs, minus the WIP exception — against **5
  tests**. Does each indicator have a route asserted, or do five tests sample
  twelve cells?
- **The exception's absence-test is cited as existing.** Confirm it does. An
  exception whose guard is claimed and missing is `FR-HLT-006`'s shape, which
  this document has already recorded once.

This is the lowest-risk of the four and the most likely to be a legitimate
sample. **Say so if it is** — `REQ-004`'s `FR-SUB-006` near-miss is the
precedent for declining to score a structural guarantee as a gap.

## §5 — the deliverable

Per entry: **the verdict** (covers / narrower / indeterminate), **the two
populations with counts** where narrower, and **whether it is a citation
defect or a coverage defect** — `REQ-004` established that distinction and it
determines who the work goes to.

Plus:

- **Your reading of `NFR-PRIV-002`'s category count**, with the reasoning.
- **Anything that changes a `REQ-004` verdict.** If resolving one of these
  reveals that an entry scored `Covers` was scored on a wrong assumption, say
  so. Re-opening a verdict is not a failure of the first audit.
- **Limits again.** If a fifth entry needs the same treatment, name it rather
  than absorbing it.

**No specification edits.** Findings come to me; the citation fixes are mine
and they land as one amendment with yours included.

## §6 — gates

Nothing is built, so: `DEC-007` confirmed once at **384**, unchanged. `fmt`
clean. If you open a test file to read it and change nothing, say so — a
report that touched no code should be able to say its diff is empty.

## §7 — exit condition

Four verdicts with their populations and their kind, `NFR-PRIV-002`'s
category count settled from the text, and **`FR-API-002`'s negative case
established either way** — escalated the same day if it turns out no test
asserts the refusal.

**`§10.35` closes when this lands and `REQ-004`'s three coverage gaps are
scheduled.** `NFR-A11Y-008` is deliberately **not** among them for 0.47.0:
asserting a script-updated region's *existence* in a Rust test and calling
the requirement covered would build the very defect `§10.35` names, one
release after recording it. It is going with the deferred drag-gesture test,
as one scoping of what the browser gate should cover.
