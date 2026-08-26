# QA-001 — The test harness collides with itself

**Issued by**: Architect
**Date**: 2026-08-11
**Priority**: P1 — first item of 0.22.0
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §9
**Governing gap**: baseline `§10.13`
**Depends on**: nothing

---

## 1. The defect

`TestApp::spawn` (`crates/peisear-web/tests/common/server.rs:35`) names its
temporary database directory from `SystemTime::now().as_nanos()` and nothing
else. Two tests entering `spawn` within the same clock tick — across threads
in one target, or across targets under a parallel run — get the **same
directory and the same `test.db`**. `create_dir_all` succeeds on an existing
directory, so nothing signals the collision.

Reproduce:

```
cargo test --workspace
```

Roughly one run in two, on a different test each time:

```
thread 'cannot_assign_sprint_directly_to_sub_issue' panicked at
crates/peisear-web/tests/common/server.rs:44:44:
connect test pool: Database(Database(SqliteError { code: 5, message: "database is locked" }))
```

Also seen at `:45` (`migrate`) and on
`issue_status_change_with_empty_client_updated_at_is_rejected`.

**Not new.** Reproduced at tag `0.20.1` on the same command. Nothing in 0.21.0
caused it and no shipped code is involved.

## 2. Why no gate log ever showed it

`DEC-007` mandates per-crate runs and every `peisear-web` integration target
individually, for isolation. That procedure never triggers the collision. Every
gate log this project has captured is honest, green, and blind to this.

Say that plainly in the review request. It is the finding.

## 3. Item 1 — make the name unique

Prefer a crate that guarantees a unique temporary directory (`tempfile` is the
obvious one, already a common transitive dependency — check before adding).

If you hand-roll it instead, say why, and combine at minimum the process id and
an atomic counter with the clock. **A hand-rolled unique-name scheme is what
failed here**, so the bar for another one is that it cannot collide by
construction rather than that collision is unlikely.

Two things to check while you are in that file:

- **Cleanup.** These directories are created under `/tmp` and, as far as I can
  tell, never removed. Confirm, and if so fix it in the same change — a
  `tempfile::TempDir` handle held by `TestApp` does both jobs at once.
- **`expect` messages.** `"connect test pool"` gave no clue which test or which
  path collided. If the failure recurs in some other form, the message should
  name the directory.

## 4. Item 2 — the gate set, which is the real deliverable

Add a **repeated full-workspace run** to the gate set: `cargo test --workspace`,
at least three times, all passing.

Three because one pass proves nothing at a ~50% failure rate, and because the
number should be justified by the observed rate rather than chosen for
roundness. If you measure a different rate, pick a count from it and show the
arithmetic.

Where this lives — `DEC-007`'s written procedure, CI, or both — is yours to
propose; you know the CI wiring better than I do. What matters is that it runs
under the conditions a contributor actually uses, not only under the isolation
the procedure chose.

**Prove the guard.** Before fixing item 1, run the new repeated gate and show it
failing. A gate added after the defect it would have caught is a gate nobody
has seen work. Same discipline as `I18N-007` §3, and the same reason.

## 5. Escalate rather than deciding

- If the collision turns out **not** to be the nanosecond suffix — if it
  reproduces with genuinely unique directories — stop. That would mean shared
  state somewhere else, and the diagnosis above would be wrong.
- If the repeated gate surfaces **other** flaky tests, report them separately
  and do not fix them here. A flaky-test sweep is its own work with its own
  scope; folding it in would hide how many there are.
- If `tempfile` (or whatever you choose) is not already in the tree and adding
  a dev-dependency is contentious, say so rather than hand-rolling around it.

## 6. Acceptance

1. `cargo test --workspace` passes three consecutive times.
2. The repeated gate was demonstrated **failing** before item 1 landed.
3. Temp directories are cleaned up, or their retention is explained.
4. The gate set records the new run and where it executes.
5. fmt and clippy exit 0; test counts unchanged — this changes no assertions.

## 7. Prohibited

Do not change any test's assertions or skip a test to make the run green. Do not
serialise the suite to hide the collision — `--test-threads=1` would pass and
would be a worse product. Do not fix unrelated flakes here.

## 8. Required review-request format

Workflow §9.2. Include the before/after transcripts for §4's proof, and the
observed failure rate you measured rather than the one I quoted.
