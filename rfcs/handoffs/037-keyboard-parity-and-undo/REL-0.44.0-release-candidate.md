# REL-0.44.0 — release candidate

**Contents**: `DEP-001` (`7383dd0`), `DM-TEST-001` (`c7de37f`), `DM-TEST-002`
(`558be78`, `09dded1`), the `FR-DM` measurement and its amendments
(`df2fa9a`, `596fc51`), **`DEC-059`** with RFC 0015 accepted (`85d2d7a`,
`1c860ab`, `c582287`), and the specification amendment with its sweep
(`f227ad8`). **Every handoff reviewed and closed.**

**`DEC-028` is already done** — both specifications cover `0.44.0`
(`f227ad8`), including §3's sweep. **Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration** — `0020` remains the most recent, the **fifth** release
running.

**Expected: 376**, up from 367. Nine new tests, all from `DM-TEST-001`; the
count did not move in `DM-TEST-002`, which renamed and wired only. **Two
things to say in the same breath as the figure**, because last release's
number fell and this one rises for an unusual reason:

- **Four of the nine are a source scan, not a test of running code**
  (`undo_toast_attachment_scan`). They pin what `static/*.js` is *written* to
  do. Do not let the suite's growth read as new runtime coverage.
- **One is not a Rust test at all.** `undo-mousedown-trap.mjs` is a browser
  gate and is outside the 376; it runs in the `browser-overflow-gate` job.

**No production source changed in this release.** Not one file under
`crates/*/src/` or `static/`. Every change is a test, a CI job, a lockfile
bump, documentation, or the specification. **Check that and say so** — it is
the clearest single fact about this release and it is cheap to verify with
`git diff --stat 0.43.0..HEAD -- crates/*/src static`.

**The overflow gate is 120 cells** and **its job now runs two scripts.**
`browser-overflow-gate` gained `undo-mousedown-trap.mjs` as a second step,
and its displayed name changed to match. Run **both** from the extracted
tree, in the job's own order.

## 2. The changelog

**This release ships no behaviour, and that is the thing to be honest about
rather than to dress up.** It is a rule, a lockfile bump, nine tests and a
correction to the project's own record. Six things.

1. **Lead with `DEC-059`, because it is the only thing here that changes what
   the product must do.** `FR-DM-002` is a **P0** that read `Partial` on the
   architect's judgement; it now has a clause, so the calendar fails a written
   test instead of an opinion: *the keyboard equivalent MUST be offered by the
   element that carries the pointer affordance — on it, or one activation
   away — and MUST perform the same action rather than require the user to
   reconstruct its effect.* State what the calendar costs a keyboard user
   today: **77 Tab and Enter keystrokes across three pages**, plus computing
   the new end time by hand, against one drag. **And state that the surfaces
   are not inaccessible** — the effect is reachable and both write paths are
   locked. What is wrong is the cost and the shape.
2. **The remedy is scheduled, not pending**: RFC 0015 Option A at **0.45.0**.
   A reader should not have to wonder whether a named P0 failure is being
   left alone.
3. **`FR-DM-006` reaches Met on all four surfaces — reversing a correction
   made one release earlier.** Write this plainly, including why: the entry
   was corrected *because* it was stale, its own three named places were
   counted, and `calendar.js` was never opened. **A changelog that records a
   correction to a correction is worth more than one that quietly lands the
   right answer**, and the procedure changed as a result — *amend from the
   code, never from the entry's own words.*
4. **`spin 0.9.8` and `chacha20 0.10.0` were yanked**; both moved within the
   same minor. Say which is shipped (`spin`, via `sqlx-sqlite`) and which is
   test-only (`chacha20`, via `axum-test`), and that **neither is known to be
   vulnerable** — *yanked* means withdrawn, which can be a security
   withdrawal or a mistaken publish, and we do not know which. **Do not imply
   a vulnerability was fixed.** Found by the release procedure's own
   `--dry-run`, which is the system working.
5. **Nine tests, and what they catch.** The one worth a sentence is
   `plan_add_then_remove_leave_issues_updated_at_untouched`: a plan move
   beginning to write `issues.updated_at` would silently invalidate every open
   tab's optimistic-lock stamp for that issue, with no error anywhere.
6. **`§10.15`'s title changed after eighteen releases** — *almost* no test
   rather than no test, because one gate now executes the shipped scripts for
   one property on three surfaces, out of **1,909 lines in five files**. Say
   that **RFC 011's refusal to buy a JavaScript harness is unchanged**, or a
   reader will conclude otherwise.

**Do not write a `### Highlights` heading unless you want one.** There is no
such rule.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`f227ad8`). Confirm both specifications read
  `Covers release: 0.44.0` and that each carries a `0.44.0` baseline row, and
  say so.
- **The sweep**: run for this release by the architect; its four findings are
  in `f227ad8`. **Confirm rather than re-run** — that the entries this
  release touched carry `0.44.0` amendments — and report *swept, nothing
  further found* **or** what you found. A silent pass is indistinguishable
  from no pass.
- `DEC-007` **376**, three consecutive runs. **Extract the command block from
  `.github/CONTRIBUTING.md` rather than reusing a saved copy** — the block
  changed twice this release, and the architect's review measured 372 from a
  stale copy before catching it.
- **Both browser scripts on the extracted tree**, from the job's own command
  lines in order: the overflow gate at **120 cells**, then the trap script
  clean on all three draggable surfaces. **Confirm the second starts its own
  server after the first has torn its own down** — different default ports,
  and `browser-checks/README.md` records why that matters.
- **`mdbook build` from the extracted tarball** — fourth release with the
  site.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `rustdoc-links`
  with **seven** `Documenting` lines.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.43.0** binary and confirm it comes up at 20 with nothing
  applied.
- `cargo publish --workspace --dry-run` **from a detached worktree of the
  candidate commit** — seven crates, and confirm **neither** yanked-crate
  advisory appears, since removing them is half of what this release is.
- **No behavioural check is asked for this time, and that is deliberate**: no
  production source changed. If you believe one is warranted anyway, say why
  rather than running it silently.

## 4. Escalate rather than deciding

- **If the count is not 376**, or the gate is not 120.
- **If any file under `crates/*/src/` or `static/` turns out to differ from
  `0.43.0`** — that contradicts §1's central claim and it is mine to explain,
  not yours to reconcile.
- **If either advisory still appears** in the dry run.
- **If the two browser scripts cannot run in sequence** in the one job.
- **If the sweep turns up a fifth finding** — report it; do not amend the
  specification.

## 5. Exit condition

A candidate commit on `main`; a changelog that says plainly this release
ships no behaviour and says what it does ship; 376 green three times from a
freshly extracted command block; the gate at 120 and the trap script clean,
both from the one job's own commands; the book building from the tarball; the
0.43.0 database coming up clean; and a dry run with seven crates and neither
advisory. Then stop.

---
**One thing is the owner's and it is not a blocker**: `FR-DM-002` ships
`Partial` against a clause it measurably fails, with the remedy at 0.45.0.
That was decided when `DEC-059` was taken; it is restated here only so the
candidate report does not present it as an open question.
