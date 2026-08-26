# QA-005 — The four structural guards have no CI job

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P1 — 0.27.0
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §14
**Depends on**: nothing. `QA-003` and `QA-004` are both closed; this needs
neither.

---

## 1. The defect

`.github/workflows/test.yml` runs a job for each of the twenty `peisear-web`
integration targets, and for `peisear-core`, `peisear-auth`,
`peisear-storage`, `peisear-i18n` and `peisear-notify`.

**It has no job running `cargo test -p peisear-web --lib`.** It has none
running `cargo test -p peisear` either.

Every structural guard this project has built lives in the first of those:

| Guard | Makes unconstructible |
|---|---|
| `prose_scan` | user-visible English authored in Rust (RFC 006) |
| `static_js_scan` | the same inside `static/*.js` (`BOARD-001`) |
| `test_harness_scan` | clock-derived temp paths (`QA-001`, baseline §10.13) |
| `dec_007_scan` | the `DEC-007` block drifting from the workspace (`QA-004`) |

`DEC-007`'s block in `.github/CONTRIBUTING.md` omits the same line, so a
contributor running the documented procedure does not run them either.

**No release has shipped without them** — `cargo test --workspace` runs three
times in the release gate and does include them. The exposure is
per-pull-request: a change that reintroduces any of those four defect classes
passes CI, and is caught at the next release candidate or not at all.

**Confirm this before fixing it.** Read the workflow file and count the `run:`
lines; do not take this handoff's word for it. If a job for
`peisear-web --lib` does exist under a name I missed, stop and report — the
rest of this handoff would be wrong.

## 2. What to add

**2.1 — A CI job**, in the shape the twenty existing ones use:

```yaml
  test-peisear-web-lib:
    name: peisear-web / lib (structural guards)
    ...
      - run: cargo test -p peisear-web --lib
```

Name it so a red check tells the reader what broke. `peisear-web / lib` alone
does not; the four guards are the reason the job exists.

**2.2 — A CI job for the facade**, `cargo test -p peisear` — bare, no `--lib`,
for the reason `QA-004` recorded: its single test is a doctest, and `--lib`
reports zero. It can share a job with 2.1 or stand alone; say which and why.

**2.3 — The line in `DEC-007`'s block**: `cargo test -p peisear-web --lib`.
Place it with the other `peisear-web` work, before or after the `--test` loop.

## 3. The part worth more than the three lines above

`dec_007_scan` passes today with the block missing that line, because it
asserts each member appears as `-p <name>` and `peisear-web` appears twenty
times via `--test` lines. RFC 005 §13 recorded that limit the day the guard
shipped. **This is its first live instance.**

Consider whether the guard should also assert that the block runs each crate's
**library** tests where that crate has any — and report your reasoning either
way rather than implementing silently. Two things to weigh:

- Knowing which crates have library tests statically means asking cargo or
  parsing sources, which was rejected once already as a parser for one line of
  a contributing guide.
- The cheap version — assert the block contains `-p peisear-web --lib`
  literally — special-cases one crate and would not have caught this class for
  `peisear-core` had the omission been there instead.

**A third option, if you see one, is what I actually want.** I do not think
either of the above is right, and I would rather have your reading of it than
my guess implemented.

## 4. Tests

| # | Check |
|---|---|
| 1 | The new job(s) run and pass on the current tree |
| 2 | Each of the four guards fails CI's new job when its own defect is planted — four plants, **one at a time** |

Check 2 is the point of the handoff. A job that runs the guards but is
mis-scoped — wrong crate, wrong flag, `continue-on-error` inherited from a
template — passes on a green tree and proves nothing. Plant `prose_scan`'s
defect (a bare English string in a Rust view), `static_js_scan`'s (a two-word
string literal in `static/dm.js`), `test_harness_scan`'s
(`SystemTime::now()` plus `create_dir_all` in a test file), and
`dec_007_scan`'s (remove a crate's line from the block).

You do not need CI itself to run four times: running the **exact command the
job runs** against each planted defect is the evidence. Say that is what you
did.

Counts do not move — no test is added.

## 5. Not in scope

- **No change to any guard's logic.** They work; they are not being run.
- **No new guard**, beyond what §3 asks you to *consider and report on*.
- **No change to `DEC-007`'s decision** or the per-crate shape.
- **No headless browser.** The JavaScript remains unexecuted by any test
  (baseline `§10.15`); that is recorded, deliberate, and not this handoff.

## 6. Escalate rather than deciding

- If a `peisear-web --lib` job already exists, stop — see §1.
- If adding the job pushes CI past a runner limit or a wall-clock budget you
  can see, report the numbers rather than trimming something to fit.
- If planting any of the four defects does **not** fail the new job, stop and
  report which. That would mean the guard has a second hole, not that the job
  is wrong.

## 7. Acceptance

1. §1 confirmed by reading the workflow file, and said so.
2. Job(s) added; the `DEC-007` block line added.
3. All four §4 plants demonstrated, one at a time, each reverted.
4. §3 answered with a recommendation and reasoning — implemented or not.
5. fmt and clippy exit 0; `DEC-007` gate set green **with the new line**;
   three consecutive `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. Include the four plant transcripts separately, and §3's
recommendation as prose rather than as a table row.
