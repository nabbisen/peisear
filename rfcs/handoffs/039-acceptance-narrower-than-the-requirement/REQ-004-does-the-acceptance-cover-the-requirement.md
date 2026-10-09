# REQ-004 — does the acceptance cover the requirement?

**An investigation. The deliverable is a count and a list, not a fix.**
**Register**: `§10.35`. **Precedent**: `REQ-001` (the status audit),
`REQ-003` (whose deliverable was *no*). **Target**: 0.46.0.

`REQ-001` audited every requirement whose **status** was a claim and found
**sixteen stale**. **Nothing has ever audited an *acceptance*.**

`§10.35` is one instance, found by accident: `NFR-A11Y-007` says *every
interactive element* and cites a guard that reads **three of six** relevant
tag names — 161 elements covered, 107 not. The requirement reads `Met`, the
guard is green, and both statements are true. **The question this
commissions**: how many other entries cite a mechanism narrower than their own
normative text?

---

## §1 — the question, stated so it can be answered

For every requirement entry with an `*Acceptance*` field:

> **Does the cited mechanism exercise what the requirement's own normative
> text says, over the population the text names?**

**Three outcomes per entry**, and the third is the valuable one:

- **Covers** — the mechanism's scope is the requirement's scope.
- **Narrower** — the mechanism covers a strict subset. **Quantify it**:
  what is in scope, what is out, and how many of each. `§10.35`'s table is
  the shape.
- **Cannot be determined from the text** — the requirement's own words do not
  define a population precisely enough to check. **This is a finding about the
  requirement, not a failure of the audit**, and `REQ-001` found six such
  entries for statuses; expect siblings here.

**Report the denominator.** How many entries carry an `*Acceptance*` at all,
and how many do not. An entry with no acceptance cannot have a narrow one —
but *a requirement with no stated mechanism* is its own observation and
`NFR-REL-007` is the precedent for what that costs.

## §2 — how to read, and the trap that matters most

**Read the requirement's normative sentence, then the mechanism, then the
code the mechanism touches.** All three, in that order. Not the status, and
**not the prose beneath the status** — that prose often argues the requirement
is satisfied, which is the thing under examination.

**The trap, from `§10.34`'s eighth instance:** the architect corrected
`FR-DM-006` by reading the entry's own words, counting the three places they
named, and never opening `calendar.js`. **An acceptance's adequacy cannot be
read off the entry.** If the mechanism is a test crate, look at what it
asserts. If it is a scan, look at its population — `TAG_NAMES` is literally
one line and it is the whole of `§10.35`.

**And beware the reverse error**: a mechanism may look narrow and be
sufficient because the requirement's population is genuinely small, or
because another mechanism covers the remainder. `NFR-A11Y-007` cites one
guard; `FR-DM-006` cites a scan **and** a browser script, each named with its
limit. **Narrower than the text** is the finding; *narrower than I expected*
is not.

## §3 — what is not being asked

- **Not whether the requirement is satisfied.** That is `REQ-001`'s question
  and it is not this one. An entry can be genuinely Met with an acceptance
  that happens to under-cover it — `NFR-A11Y-007` may well be exactly that.
  **Say so when it is**, and do not escalate a coverage gap into a compliance
  claim.
- **Not a fix.** Findings are scheduled afterwards, by me.
- **Not a new scan.** `REQ-003` tested four text rules for status staleness
  and **none reached the bar**; the shape here is harder, because judging
  coverage needs reading the code the mechanism touches. **If you think a
  mechanical pre-filter would help you read 100+ entries, propose it and say
  what it would miss** — do not build it as the deliverable.

## §4 — the deliverable

1. **A table**: entry, its acceptance, the verdict (covers / narrower /
   indeterminate), and for *narrower* the two populations with counts.
2. **The denominator**: entries with an acceptance, entries without.
3. **The three worst**, with the reasoning for each, so the ranking is
   arguable rather than asserted.
4. **Your own assessment of the audit's limits** — which entries you could not
   judge and why. `REQ-001` stated its blind spot in its own words and
   `NFR-REL-007` then fell into exactly it; **that sentence was worth more
   than several of its findings.**

**If the answer is that `§10.35` is the only instance, that is a complete and
valuable result** — it would mean the acceptance fields are in better shape
than the status fields were, and it closes `§10.35`. **Do not manufacture
findings to make the audit look worthwhile.**

## §5 — scope control

**Read every entry, but do not read every entry's code.** Triage: an
acceptance naming a test crate whose subject is obviously co-extensive with
the requirement (`FR-SCH-001..004` ↔ `search`) can be judged in a line. Spend
the time where the requirement's text names a **population** — *every*,
*all*, *any*, *each* — because that is where a narrow mechanism hides, and it
is how `§10.35` reads.

**Report the time it took.** If this is a two-day job the next audit gets
commissioned sooner; if it is a week, that is worth knowing before the next
one is scoped.

## §6 — gates

Nothing to build, so: `DEC-007` unchanged at its current figure, confirmed
once. **No specification edits** — findings come to me. If you need to record
working notes, they belong in the review-request package, not in
`requirements.md`.

## §7 — exit condition

The table, the denominator, the three worst with reasoning, and the stated
limits. **`§10.35` closes on this report plus my scheduling of whatever it
finds** — not on `TT-007`, which fixes the one instance we already know
about.
