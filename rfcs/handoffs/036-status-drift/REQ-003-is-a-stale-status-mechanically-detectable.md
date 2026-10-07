# REQ-003 — is a stale requirement status mechanically detectable?

**Issued by**: Architect
**Date**: 2026-10-07
**Target release**: 0.43.0
**Governing RFC**: none. **Related**: `§10.17`, `FR-DM-001`'s obligation.
**Depends on**: nothing. **First of the release.**

**This is an investigation with a possible build at the end of it.** The
question is whether the defect below can be caught by a machine. **If the
answer is no, that is the deliverable** — say so with what you tried, and the
remedy becomes procedural instead.

---

## 1. The defect, six instances, all mine

A requirement's `*Status*` field says one thing and the release says another.

| # | entry | said | was |
|---|---|---|---|
| 1 | `FR-DM-001` | *two of five* | four had shipped |
| 2 | `FR-SPR-004` | `Specified`, blaming an unbuilt screen | three shipped routes reached it — **and this one cost two handoffs**, one shipped to `main` and half reverted, one written and withdrawn unbuilt |
| 3 | `FR-HLT-009` | `Partial`, with an exception its own correction had withdrawn | — |
| 4 | `NFR-A11Y-004` | `Partial`, against a finding that did not bear on its own sentence | — |
| 5 | `NFR-A11Y-011` | **Not met** | met by `A11Y-006` **in the release that shipped saying so** |
| 6 | `NFR-A11Y-010` | **Not met** | met by `A11Y-007`, same release |

**`REQ-001` corrected sixteen more of the same shape** in 0.40.0, and
`NFR-REL-007` was a false `Implemented` found by `DOCS-001`.

**The pattern, as precisely as I can state it**: statuses of entries I am
*already editing* get updated; statuses the *work* closed get missed. 5 and 6
were met by handoffs and never revisited, while 2 and 4 — which I was editing —
were corrected in the same pass.

**And the remedy recorded at instance 3 was a habit.** A habit has now failed
three more times.

## 2. What to find out

**Is there a rule a machine can apply?** Candidates, none of which I am
confident in — **test them against the six, which is the only evidence that
matters**:

- **(a)** A status matching `Not met|Partial|Specified` whose own entry body
  also contains a closure word (*met*, *closed*, *fixed*, *addresses*) near a
  handoff id. Catches 3, 4, 6. **Probably misses 5**, whose body named the
  audit, not a fix.
- **(b)** A status saying `Not met`/`Specified` that names a handoff id which
  **has shipped** — determinable from `CHANGELOG.md`, which is now
  machine-readable thanks to `changelog_scan`. Catches 6. Misses 5.
- **(c)** Something about the *release* rather than the entry: for each
  handoff cited in a shipped changelog section, the requirement it names must
  not be `Not met`. Needs a link from handoff to requirement that may not
  exist in a parseable form.
- **(d)** Your own, which is the one I most want.

**Run each candidate over the six and report its hit rate.** A rule that
catches two of six is not worth shipping; one that catches five might be.

**False positives are the thing that kills this.** A `Partial` whose body
legitimately says *"`X` addressed one limb"* is correct, not stale. **Count
them** over the whole document — 162 requirements — and report the number. A
guard that cries wolf on correct entries will be disabled within two releases,
which is worse than no guard.

## 3. If a rule works

Build it as a `--lib` scan beside the `dec_007_*` family — no new test file.
**Each rule seen to fail**, by planting one of the six historical states.

**And heed `§10.28`**: its doc must say what it cannot catch. If it misses
instance 5's shape, say so in the module rather than letting a green run imply
the class is closed.

## 4. If no rule works

**Say so, and say what you tried.** That is a real finding and it changes the
remedy to a procedural one, which I will then write into every release
candidate: *every requirement this release's handoffs claim to close has its
status checked.*

**There is evidence that the procedural version works**: you have caught three
of these yourself — `NFR-REL-007`, `NFR-A11Y-002`'s measured paragraph, and
`NFR-A11Y-002`'s status — **every one of them while reading the specification
to source changelog content.** That is the moment the entries get read against
what shipped, and it may be that a person at that moment beats any rule.

## 5. Verification

- The candidate rules, each run over the six instances, **hit rate reported**.
- **False-positive count over all 162 entries** for any rule you would ship.
- If built: `DEC-007` reported, last recorded **369**; each rule planted;
  `fmt`, `clippy`, three consecutive workspace runs.
- If not built: the attempts, and which of the six each would have caught.

## 6. Escalate rather than deciding

- **If a rule works but needs a data source that does not exist** — a
  machine-readable link from handoff to requirement, say. That is mine to
  create or decline.
- **If the false-positive count is high but the rule is otherwise good.**
  Do not tune it into uselessness; bring me the number.
- **If you find a seventh instance** while testing. Report it; I will fix it.

## 7. Exit condition

Either a scan that catches most of the six with a reported false-positive
count and a doc saying what it misses, **or** a statement that no rule works,
with the attempts — and in the second case the remedy moves to the release
procedure, which is mine.
