# REQ-006 — pin the absence claims

**Eleven times a requirement's status has claimed something about test
coverage that the suite contradicted. Nothing checks these, and a sweep
would not change that.**

`§10.34` is the entry; its count reached **eleven** on 2026-11-03, four of
them in two days, one a **P1**, one a **P0** that read as its own opposite,
and one (`FR-PLAN-005`) that **bought work already done** — the architect
had written its "missing" test into a proposed release round.

## §0 — what this round is, and what it deliberately is not

**It is a rule, not a census.** The first version of this work was *read
all 185 `*Status*` clauses and report*. That was rejected by the owner's
own philosophy: a one-time read is a snapshot that goes stale the next
time a status is written, and this project's doctrine is that **the defect
is not the instances, it is a rule that was never written down** (`RFC 022`
§3's framing, `§10.36`'s whole class).

**The shape to build is the one this codebase already trusts twice**:
`docs_scan`'s `EXPECTED_ENV_VAR_LITERAL_SITES` (rule 8) and
`EXPECTED_QUICK_START_COMMAND_COUNT` (rule 9) — **an asserted enumeration**.
A new absence claim changes the count and must be justified at the moment
it is written; an existing one cannot be forgotten, because the test names
it.

## §1 — why a rule is possible here when `REQ-003` concluded it was not

**Read `§10.34`'s `REQ-003` passage before building anything.** It ran four
candidate text rules against six historical instances and concluded **no
machine can catch this class**. That conclusion stands and this round does
not overturn it — because `REQ-003` was asking whether a rule could
**judge** whether a status is true.

**This rule judges nothing.** It asserts *which* status clauses make a
coverage-absence claim, and how many. A human decides each one's verdict.
**A rule that cannot be trusted to decide can still be trusted to
shortlist** — and the architect's own ten-minute probe over the 185 clauses
produced exactly three live candidates with **three different verdicts**:
one false (instance 11), one true (`FR-TEAM-005`, now `REQ-005`), one
accurate-but-misreading (`FR-HLT-006`, reworded). That is the evidence the
shortlist works.

**Three-valued output, and the entry now says so**: correct the status,
write the test, or reword. Only the first is a `§10.34` instance. **A rule
that treats all three the same would waste its own findings.**

## §2 — the calibration read, which is bounded and is not the deliverable

A pattern has recall you cannot assume. The architect's probe used
*no test*, *not pinned*, *nothing asserts/pins/checks*, *unpinned*,
*untested*, *no guard*, *nothing guards* — and **that set is a guess.**
Forms it would miss include *no scan*, *nobody checks*, *not checked*,
*by hand*, *no CI job*.

**So read the 185 clauses once, to calibrate the pattern — not to produce
corrections.** The deliverable of the read is *which phrasings exist*; the
deliverable of the round is the rule plus its pinned enumeration. If the
read finds a claim the pattern misses, the pattern gains that phrasing and
the enumeration gains that entry.

**Report any verdicts you form, but do not edit `requirements.md`.**
Statuses are the architect's (`CFG-001`'s division).

## §2a — the instrument `REQ-005` handed this round, and it is better than the pattern

**Amended 2026-11-03, after `REQ-005`.** That round found `FR-TEAM-005`'s
status false — instance **12** — and the architect's phrase pattern **could
not have found it**: the test renders a message key and compares against
that, so the status's literal words appear nowhere in the test's source. A
pattern over status prose shortlists claims; it cannot tell you a claim is
false when the evidence against it contains none of its words.

**What found it was the commit.** `edea18c` — *"`REQ-002`: pin the four
limbs `REQ-001` moved to Met on a probe"* — added exactly four tests, and
**all four of the requirements they pin kept a status denying them**:
`FR-PLAN-005` (8), `FR-CAL-005` (9), `FR-NTF-005` (11), `FR-TEAM-005` (12).
Four for four, twenty-two releases.

**So this round gains a second pass, and it runs first because it is both
cheaper and exact**: find the commits that added tests *in order to satisfy
requirements* — their own subject lines say so (`REQ-002`'s does) — and for
each, check every requirement it names against every test it added.
`git log --diff-filter=M -- docs/specification/requirements.md` and
`git log -S` on a test name are the tools; `REQ-005` used the second to date
its own finding to the day.

**This does not replace the enumeration rule**, which is still the
deliverable: the commit pass finds today's instances, the rule stops
tomorrow's. **But report the commit pass first**, because if it finds a
fifth and sixth the enumeration's pinned list must include them from the
start.

## §3 — the trap, named once

**The enumeration must fail loudly when its own lookup fails.** `§10.36`
instance 6 is `docs_scan` rule 4 passing having examined nothing after a
literal moved, and both rules 8 and 9 now assert their own lookup
succeeded. **This rule must do the same from the start**: if
`requirements.md` cannot be read, or the `*Status*` marker changes, it
reports *that* — never zero claims found.

**And it must be seen red both ways**, per `DEC-048`: plant a new
absence-claiming status and watch the count change; break the anchor and
watch it report the lookup failing rather than reporting nothing.

**One more, learned in `QS-002` this week**: build the plant's fixture
**inside** the structure it tests, not anchored on a neighbour, or the
plant silently stops planting the next time the document is restructured.
A plant that no longer plants still passes.

## §4 — where the rule lives

`spec_scan`, not `docs_scan` — `spec_scan` already owns the rules that read
`requirements.md` and already has `test_count_citations` reading the same
file. Reuse its block-reading helpers rather than writing a third copy;
`docs_scan` made its own `pub(crate)` for exactly this reason.

## §5 — gates

`cargo fmt --check`; `clippy --workspace --all-targets -- -D warnings`;
`mdbook build docs`; the full per-crate suite with `DEC-007` from the
runner, stated as a delta and remembering `test.yml`'s own block lines
define it. **Quote the enumeration your rule asserts** in the report — the
list is the round's most useful output for the next reader.

## §6 — escalate rather than deciding

- If the calibration read finds **more than a handful** of claims the
  pattern misses, the rule may be the wrong instrument. **Say so with the
  count** rather than widening the pattern until it fits.
- If a claim's verdict needs reading product code to settle, **report it
  unsettled.** `REQ-003` found two instances outside what any text rule
  reaches; an honest *unsettled* is worth more than a guess.

## §7 — exit conditions

1. A `spec_scan` rule asserting an enumeration of the coverage-absence
   claims in `requirements.md`, with its own lookup asserted.
2. **Seen red both ways**, each with captured output, reverted, green
   after — and the plant built inside the structure it tests.
3. The calibration read done and reported as *phrasings found*, with any
   verdicts offered for the architect rather than applied.
4. The asserted enumeration quoted in the report.
5. `§5`'s gates after the last edit, `DEC-007` delta from the runner.
