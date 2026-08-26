# QA-004 — The `DEC-007` block omits a crate

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P2 — 0.27.0
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §13
**Depends on**: nothing. Independent of `QA-003`; they touch different
files and can land in either order.

---

## 1. Where this came from

**The dev team found this in their own gate table while assembling
`REL-0.26.0`, and reported it instead of correcting the number.** That is
the behaviour the escalation rule exists to produce, and it is why the
scope below is larger than what they proposed — the finding was better
than the fix they suggested for it.

## 2. The defect

`.github/CONTRIBUTING.md`'s `DEC-007` command block runs six of the seven
workspace members. **`peisear`, the facade, is absent** — not present with
the wrong flags, absent.

Its one test is a doctest at `crates/peisear/src/lib.rs:28`:

```
$ cargo test -p peisear --lib
lib.rs:  0 passed        main.rs: 0 passed

$ cargo test -p peisear
lib.rs:  0 passed        main.rs: 0 passed
Doc-tests peisear: 1 passed
```

**No coverage was ever missing.** `cargo test --workspace` runs doctests,
and it runs three times before every release. What was wrong is
provenance: `REL-0.25.0`'s per-target table carried a `1` for that crate,
and `REL-0.25.0`'s own `cold-gate-tests.log` contains zero `Doc-tests
peisear` blocks. The number was correct. No command in the log produced
it.

That distinction matters for how this gets written up and it should
survive into the fix: this is a documentation defect with a reporting
consequence, not a hole in the suite.

## 3. What to change

**3.1 — Add the missing line** to the block, bare, no `--lib`:

```bash
cargo test -p peisear
```

Place it last, after the `peisear-web` loop, matching the dependency order
the rest of the block already follows.

**3.2 — Name the `--lib` trap** in the prose under the block, in one or two
sentences. `peisear-core`, `peisear-auth` and `peisear-storage` are invoked
with `--lib`, which skips doctests. There are none in those crates today —
the workspace contains exactly one doctest, the facade's — so nothing is
uncovered. It becomes a hole the day someone writes a documented example in
one of them.

Write it as the standing fact it is, not as an apology for the block. The
existing prose around `DEC-007` is a good model: it gives the live reason
for the per-crate shape (bounded linker memory), then a clearly-labelled
**History** paragraph for the defect that no longer applies. Do not disturb
either.

**3.3 — Guard the list against the next crate.** The block is seven crate
names maintained by hand against a workspace that has grown to seven. That
is the shape that produced this defect and it will produce it again at
eight.

Add a scan, in the established pattern — `crates/peisear-web/src/` already
has three (`prose_scan.rs`, `static_js_scan.rs`, `test_harness_scan.rs`),
and `static_js_scan.rs:188` shows how to reach the workspace root from
`CARGO_MANIFEST_DIR`:

- Read `members` from the workspace `Cargo.toml`.
- Read `.github/CONTRIBUTING.md`.
- Assert every member's crate name appears in the `DEC-007` block.

Failure message names the missing crate and says to add a line to the
block — the same posture `static_js_scan`'s message takes.

**Match on the crate name, not on a command string.** `peisear-i18n` and
`peisear-notify` are invoked bare, three crates take `--lib`, and
`peisear-web` is a shell loop over test targets. A guard that expects a
particular command shape would be wrong about four of the seven on the day
it was written.

**Watch the substring.** `peisear` is a substring of every other member's
name, so a naive `contains("peisear")` passes even with the facade line
absent — the exact defect this guard exists for would slip through it.
Match on a word boundary, and **prove that specific case**: with the
`cargo test -p peisear` line removed, the guard must fail.

## 4. Tests

| # | Check |
|---|---|
| 1 | Guard passes on the corrected block |
| 2 | Guard fails with the `cargo test -p peisear` line removed — the substring case from §3.3 |
| 3 | Guard fails with any other member's line removed |

Plant each removal **one at a time**. A compound plant proves less than it
looks like it does, which is how `STATUS-001`'s test 6 stayed green against
a defect for a full review round.

Expected counts: `peisear-web` lib **11 → 12**, workspace **178 → 179**
(**180 → 181** if `QA-003` lands first).

## 5. Not in scope

- **No retroactive correction of `REL-0.25.0`'s table.** It is a shipped
  record and the number in it is right.
- **No change to `DEC-007` itself** — the decision, the per-crate shape, and
  the three-consecutive-`--workspace`-runs procedure all stay exactly as
  they are. This handoff makes the documented block match the workspace it
  claims to cover; it does not revisit why the block exists.
- **No doctests added anywhere.** If a crate wants one, that is its own
  change.

## 6. Escalate rather than deciding

- If the `members` list cannot be read without a TOML dependency the crate
  does not already have, stop and report before adding one. A guard is not
  worth a new dependency; there are cheaper shapes.
- If the `DEC-007` block turns out to be duplicated elsewhere in the docs,
  report where. A guard pointed at one of two copies is worse than none,
  because it reads as coverage.

## 7. Acceptance

1. `cargo test -p peisear` in the block; the `--lib` trap named in prose.
2. Guard present, with all three §4 plants demonstrated one at a time —
   including the substring case.
3. fmt and clippy exit 0; `DEC-007` gate set green, **run with the new line
   included**; three consecutive `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. Include the three plant transcripts separately, and state
the workspace total with the new test, since it moves.
