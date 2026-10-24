# SPEC-005 — the counts a step can check

**0.57.0, added after `SPEC-003`. Small, and not blocked on
anything** — `SPEC-004` waits on the owner; this does not.

`SPEC-003` built the extractor and stopped before wiring it to a
runner, which was the right place to stop. This round wires it —
**as a CI step, not a `cargo test` test.**

---

## §1 — why it is worth doing, in one number

`SPEC-003` checked all eight `` `name` (N tests) `` citations against
`cargo test --test <name> -- --list`. **Three were wrong**:

| Citation | Said | Was |
|---|---|---|
| `view_state` | 5 | 9 |
| `aggregate_privacy` | 12 | 13 |
| `optimistic_lock` | 21 | 22 |

Re-verified independently in review; all three figures confirmed, and
the five others confirmed already correct.

**Three of eight, in a population that is small, uniform, and already
the document's own house style.** The alternative — re-checking by
hand at each release sweep — is a recurring manual checklist, and
this project's standing position is that a property worth
guaranteeing belongs in a guard that runs itself, never in a ritual
someone performs.

## §2 — a CI step, and why not a test

**Do not spawn `cargo` from inside `cargo test`.** `SPEC-003` balked
at this and was right: nested invocations contend for the
package-cache lock, and it would rebuild six test binaries inside a
job that has already built them.

**The idiom already exists.** `0.56.0` added two CI steps that run
processes rather than pretending to be tests — the boot smoke that
starts `peisear` against a real PostgreSQL server, and the step that
proves `backup.md`'s own `pg_dump` commands against it. Neither is a
`cargo test` test, for exactly this reason.

**Put the step in a job that has already built the binaries**, so the
marginal cost is a `--list` per target rather than a build. Say which
job and why in the step's own comment.

## §3 — the shape

1. Extract the citations with **`spec_scan::test_count_citations`** —
   the function `SPEC-003` already built and tested. **Do not write a
   second parser in shell.** Expose whatever is needed (a tiny
   binary, an existing test printing them, or a `--list`-style
   helper) rather than duplicating the pattern, so there is one
   definition of what a citation looks like.
2. For each citation, run `cargo test --test <name> -- --list`, count
   lines ending `: test`, compare.
3. **Fail naming the citation, the cited number and the real one.**
   A failure that says only *"mismatch"* costs a reader the same
   search the step was built to save.

## §4 — what must not happen

- **Never count tests by grepping `#[test]`.** `NFR-A11Y`'s
  digit-bearing segment and `#[tokio::test(flavor = …)]` have both
  already defeated a pattern in this series. The runner is the only
  authority.
- **Do not widen `test_count_citations`' pattern** to pick up
  `optimistic_lock_atomicity`'s list-shaped `14`. `SPEC-003`
  deliberately left the population at eight and wrote why in the
  function's doc comment; a parser widened for one occurrence of a
  different shape is how a rule starts keying on strings.
- **Do not touch `§9.1`'s dated inventory table.** *"Measured per
  target on 2026-08-27 at `0.28.0`, not carried forward"* — those
  numbers are correct **because** they are dated, and `view_state`
  legitimately reads `5` there and `9` at `:658`. A round that
  "fixes" that table has misread the whole of `SPEC-003`.

## §4a — round 2: the step must know how many citations it expected

Round 1's step compares each citation to the runner in a `for` loop
over the extracted list. **An empty list skips the loop and reaches
the success message.** Proven in review by running the step's own
logic with a filter that matches nothing:

```
view_state: requirements.md own citation () matches the runner (9)
STEP EXIT=0
```

Reachable three ways, all realistic: the print test renamed or
deleted (`cargo test --lib <filter matching nothing>` **exits 0** —
the property `CON-002` measured and `§10.46` exists for); the
citation removed from `requirements.md`; or the document reflowed,
since these citations span line breaks.

**This is `§10.46`'s class one layer up, in the release built to
close it.**

**Assert the population before comparing it**, per job, by exact
count rather than merely non-empty:

| Job | Citations expected |
|---|---|
| `undo_toast_attachment_scan` | **2** — cited twice |
| the other six | **1** each |

An exact count also catches a citation being *added* for a name
without anyone noticing, which is the same class one step on.
`SPEC-003`'s real-tree test already fixes the total at eight; this
makes each job's share equally explicit.

**Plant the absence, not another wrong number.** Round 1's proof
exercised the mismatch path, which is why it could not catch this.
Rename the filter, watch a step go red, restore.

## §5 — gates

- **The step seen red**: plant a wrong number in a citation, watch
  the step name it, remove it. A check never seen to fail is not
  evidence, and this one is the round's only deliverable.
- The step green on the real tree, with its output quoted.
- `DEC-007` unchanged unless a test is added; state which.
- **`test.yml` is edited, so `test.yml`'s own gate runs** — the
  standing rule.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  `publish-dry-run`, CI.

## §6 — escalate rather than deciding

- **If no existing job has the binaries built**, so the step would
  add a full build. Bring the measured cost; a cheap check that
  becomes an expensive one is a different decision.
- **If exposing the extractor to a shell step needs a new binary or
  a published API**, say what it costs before adding it. A
  `#[test]` that prints the citations and a step that reads its
  output is acceptable if it is simpler.
- **If a ninth citation appears** that `SPEC-003`'s real-tree
  population test did not expect. That test exists to notice; report
  it rather than absorbing it.

## §7 — exit condition

The step wired into a job that already builds the targets, using
`spec_scan::test_count_citations` as the single definition of a
citation rather than a second parser; seen red on a planted wrong
number and green on the real tree with its output quoted; failures
naming citation, cited number and real number; the population still
eight, `§9.1`'s dated table untouched, and no count obtained by
grepping `#[test]`; `test.yml`'s own gate re-run; `DEC-007`
reconciled; all gates re-run after the last edit.
