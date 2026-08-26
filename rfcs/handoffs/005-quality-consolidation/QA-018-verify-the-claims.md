# QA-018 — Verify the baseline's own claims

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P1 — the defect class this baseline exists to correct, at the
scale of 124 claims
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §8
**Depends on**: nothing.

**Audit and report. Change no requirement text.** The baseline is the
architect's document; you establish the facts and I correct it.

---

## 1. Read RFC 005 §8 first — it was rewritten today

The original said to grep for `TODO`/`FIXME`, `#[ignore]`, and
`unimplemented!()`/`todo!()`. **All three come back empty**, and the
`prose_scan` allowlist already has its own honesty guard.

**Reproduce that first**, in one pass, and report the three counts. If any is
non-zero, stop — the rest of this handoff assumes a clean tree and I would
rather know.

## 2. The input, and the one confirmed instance

The requirements baseline is
`.git-exclude/specs/peisear-0.27.0-requirements-en.md`. It carries **153**
requirement blocks. Of those, **124 are marked `Implemented`**: 40 cite an
acceptance test, 84 cite nothing.

**One citation is already known wrong.** It read that `board_keyboard`
verifies `NFR-A11Y-007` (44 × 44 touch targets). It does not — its six tests
assert routes, the lock token, reachable statuses, accessible names and
vocabulary, and **nothing in the suite asserts a touch-target dimension**.
Found by accident during `QA-014`, corrected there.

That is the class. One was found without looking; this handoff is the looking.

## 3. Part A — the 40 citations

For each `Implemented` requirement carrying an `*Acceptance*` clause:

1. Does the named test or suite **exist**? (I checked every distinct function
   name and all exist — confirm, do not re-derive.)
2. **Does it assert what the requirement says?** Read the assertions. Not the
   test name, not its doc comment — the assertions.
3. Verdict: **holds**, **partial** (asserts some of it), or **does not hold**.

**Report all 40, holds included.** An audit listing only failures cannot be
distinguished from one that stopped early — this project has said that four
times now and it is still true.

Where a verdict is `partial` or `does not hold`, say **what the test actually
asserts**, so I can either correct the citation or add the requirement to
§9.2. Do not propose new tests; that is a separate decision.

## 4. Part B — the 84 without a citation, sampled

`§9.2` lists exactly **four** requirements as "implemented but unverified".
Eighty-four carry no evidence. **Those two numbers cannot both be describing
this document honestly**, and there are only two explanations:

- Most of the 84 **do** have tests that were never cited, and §9.2 is right
  about the true count being small.
- Most of the 84 **do not**, and §9.2 is badly incomplete.

**Sample fifteen**, spread across families — take some `FR-PROJ`, `FR-ISS`,
`FR-TEAM`, `FR-SPR`, `FR-HLT`, `FR-PER`, `FR-NTF`, and at least three
`NFR-*`. For each: is there a test that would fail if the requirement were
violated? Name it, or say there is none.

**Report the ratio you find and stop.** Fifteen of eighty-four is a sample and
must be labelled one — I will size the remainder from what it shows. Do not
extrapolate the ratio into a claim about the other sixty-nine.

## 5. What I expect to be wrong about

I think Part B will come back mostly "a test exists, nobody cited it", because
this project writes tests before it writes citations. **If it comes back the
other way, that is the finding**, and it is much larger than this handoff — say
so plainly and do not soften it to fit the scope you were given.

Similarly, if Part A turns up several broken citations rather than one or two,
the conclusion is about how this baseline is maintained, not about those
requirements. Say that too.

## 6. Escalate rather than deciding

- If §1's greps are non-zero, stop.
- **If any Part A citation names a test that asserts something a requirement
  forbids** — not merely fails to assert it — stop immediately. That is a live
  defect, not a documentation gap.
- If a requirement's text is itself ambiguous enough that "does the test assert
  it" has no answer, say so and list it separately. That is a finding about the
  requirement, and it is mine to fix.
- Do not edit the baseline. It is not in the repository you commit to, and the
  corrections are the architect's.

## 7. Acceptance

1. §1's three counts reported.
2. All 40 Part A verdicts, holds included, with assertions named for anything
   not `holds`.
3. Fifteen Part B samples with a named test or an explicit "none", labelled as
   a sample.
4. §5 answered — which way it came back, without softening.
5. Nothing changed: `git status --short` empty.
6. fmt and clippy exit 0; three consecutive `cargo test --workspace` runs
   (unchanged count expected).

## 8. Required review-request format

Workflow §9.2. Part A as a table. Part B's ratio as a number with its sample
size beside it. §5 as prose.
