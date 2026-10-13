# REL-0.52.0 — release candidate

**Contents**: `GRD-001`, `GRD-002`, `GRD-003`. **Every handoff reviewed
and closed.** Schedule authorized by the owner 2026-10-14
(`ROADMAP.md`, *What ships next*).

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop. The four `DEC-056` legs are the architect's, on the
owner's approval.

---

## §0 — already done this cycle; reconcile, do not redo

Four commits landed after `GRD-003` and before this handoff. The
`DEC-028` sweep should find them already correct and say so:

| Commit | What |
|---|---|
| `a68b6d9` | `NFR-A11Y-007` states `alignment_scan`'s own coverage — 37 of 41 `grow_text` sites |
| `be3332f` | `§10.39`'s residual (nine, not four) and out-of-scope figure (45/21); `docs_scan.rs`'s suffix-list doc comment |
| `9efc00d` | `components.rs:146` 136 → **212**; `NFR-MNT-005` amended — the guard-location inconsistency **declined**, with the plant that shows why |
| `1132f3e` | `Appendix C`'s 0.51.0 row, figures corrected |

**The guard-location question is closed and is not open for
reinterpretation in this round.** `GRD-002 §5` recommended preferring
`src/` placement workspace-wide; the architect agreed in review, then
found the hazard it rested on is already closed by `dec_007_fs_scan` and
reversed. `NFR-MNT-005` carries the reasoning and the plant.

## §1 — the ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only. **`0.51.0` →
`0.52.0`.**

**No migration** — `0021` remains the most recent, **the third release
running**.

**Expected: 451**, from 442. Nine tests: `scan_support`'s four
(`GRD-001`) and the widened needle's five (`GRD-003`). **`GRD-002` adds
none** — moving a test between files neither creates nor destroys one,
and a different figure there means one was lost in the split. **State
the arithmetic yourself; mine is from the review packages and from two
runs of my own, not from yours.**

## §2 — the claim that makes this release unusual, and how to falsify it

**Nothing this release ships is different.** Measured across
`0.51.0..HEAD`: every changed line under `crates/` is a doc comment, a
`#[cfg(test)]` module, or a test file. The only change to a line that
compiles into a shipped binary is `peisear-web/src/lib.rs` gaining
`#[cfg(test)] mod scan_support;`, which is itself `#[cfg(test)]`.

**Reproduce this yourself and report what you got** — do not take the
sentence above. For every `.rs` file in the range's name list, count the
changed lines that are not blank, `//`, `///` or `//!`, and name every
file where that count is non-zero. My result: thirty-three files, all of
them a `#[cfg(test)]` scan module or its `tests.rs`, plus `lib.rs`'s two
lines.

**This is what the browser gates are for in this release.** They are not
a formality here: they are the falsification. A zero-line production
diff predicts they cannot move. **If one moves, the diff method is
wrong and that is the finding** — stop and report it rather than
chasing the gate.

## §3 — `DEC-028`

Both specifications to `0.52.0` — a baseline row in `requirements.md`'s
Appendix C and in `external-design.md`'s own table — **and the five-pass
sweep reported pass by pass.**

- **`external-design.md`'s row is the short one, and should say so
  plainly**: no screen, no route, no status code, no copy string, no
  state. A release that changes nothing the document describes is a row
  that says exactly that, not a row padded to look like the others.
- **`requirements.md`'s row** carries the three rounds, the closed
  `§10.37` and `§10.38`, the new `§10.39`, `NFR-MNT-005` reaching
  `Implemented`, and the test inventory 442 → 451.
- **Expect the sweep to find the amendments in §0 already applied.**
  Report that it found them; a sweep that reports "nothing to do"
  without naming what it checked is `§10.36`.

## §4 — the changelog

**Lead with the fact that there is nothing to see, and then say why
that was worth a release.** A reader who upgrades and notices no
difference should find the notes telling them that first.

1. **Nothing you can see changed, and nothing you run differently.**
   No screen, no route, no copy, no migration — the third release with
   none. Every line that ships is what `0.51.0` shipped.
2. **What changed is the checking.** This project decides what is true
   about itself by running guards. Two of them were not answering the
   question they were named for, and both were found by looking at the
   guards rather than at the product.
3. **A guard reporting clean on files it could not read.**
   `strip_line_comments` existed in six copies; five truncated any line
   holding `xmlns="http://…"`, which corrupted quote-pairing for the
   rest of five component files. `touch_target_scan` is one of those
   five, and `NFR-A11Y-007` rests on it. **One copy now**, the one that
   was already correct — which reverses `QA-002`'s duplication call for
   this helper and nothing else. Demonstrated by plant: a violation
   planted in a corrupted region passes at `0.51.0` and fails now.
4. **Fifteen files that were wholly tests** now use the shape
   `project-instructions-rust.md` gives. `NFR-MNT-005` is `Implemented`
   with a measured live count of zero; `§10.37` closes.
5. **A rule that enforced the spelling of what it banned.**
   `docs_scan` rule 5 forbids citing the project's private area and
   matched only the literal path; the house style is the bare file
   name. **64 such citations** sat in published doc comments and in the
   specification itself while the rule reported clean. Cleared by
   stating the substance — never by publishing the private file — and
   the needle widened to a shape that reads nothing under the
   gitignored path, so it works in a published crate and an extracted
   tree alike.
6. **What this release does not fix.** The widened needle has a
   residual of nine privately-named files it cannot match, stated with
   the number in `§10.39`; 45 occurrences in `//` comments and
   `crates/*/tests/` are out of scope by design and left; and the
   figures in this release's own register were corrected three times
   before they were right, which is recorded there because measurement
   discipline is what the release is about.

**Three things the notes must not claim**: that the guards now cover
what they name — beat 6 says where they do not; that this was a
refactor — it repaired a guard that was blind to a real violation; and
that `NFR-MNT-005` was a tidy-up — it was the owner's call on an open
question, taken 2026-10-13.

## §5 — verification before the candidate commit

- `DEC-007` **451**, three consecutive runs, from a block extracted
  **fresh** out of the extracted tree's own `.github/CONTRIBUTING.md`.
- `fmt --check`, `clippy --workspace --all-targets -D warnings`,
  `cargo doc` with broken-intra-doc-links denied — **8** private-link
  warnings (4 `peisear-storage`, 4 `peisear-web`), **0** broken. Count
  them by name, not by the summary line.
- **All three browser scripts on the extracted tree**, in the job's
  order — `overflow-gate.mjs`, `drag-outcome-gate.mjs`,
  `undo-mousedown-trap.mjs`. **Expect no movement at all**, per §2. The
  overflow gate's cell count is the one to state explicitly.
- **`mdbook build` from the extracted tarball**, four screenshots
  present in the output.
- **The database check**: build `0.51.0` from its own tag commit
  (`67d4524`), create a database, confirm **21**; boot the candidate
  against it and confirm **21 with nothing applied**.
- `cargo publish --workspace --dry-run` from a **detached worktree of
  the candidate commit** — seven crates, every `warning:` line read.
- **One spot-check on the artefact**: any project page, confirm *"(set
  your capacity)"* still sits beside `WORKLOAD`. `0.51.0` fixed that;
  this release must not have moved it.

## §6 — escalate rather than deciding

- **If the count is not 451.** A test lost in `GRD-002`'s split would
  surface here and nowhere else.
- **If any browser gate moves**, or the production-diff check in §2
  names a file I did not. Either means one of the two methods is wrong
  and the release's own headline is unsupported. Stop.
- **If the five-pass sweep turns up anything beyond §0's four
  commits.**
- **If a `DEC-007` run flakes** — say so with the disk state; the
  harness still writes its databases to `/tmp` by measured choice.

## §7 — exit condition

A candidate commit on `main` at **0.52.0**; both specifications amended
with their baseline rows and the sweep reported pass by pass, with §0's
four commits named as found-already-correct; a changelog that leads
with *nothing you can see changed* and ends with what is still not
fixed; **451** green three times with any flake named; three browser
gates clean and stated as unmoved, with the overflow gate's cell count;
the book building from the tarball with its four images; a `0.51.0`
database coming up at **21** with nothing applied; a dry run of seven
crates; and the §2 production-diff check reproduced with its own
result. Then stop.
