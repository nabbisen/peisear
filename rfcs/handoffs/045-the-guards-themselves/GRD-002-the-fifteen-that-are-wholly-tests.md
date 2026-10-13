# GRD-002 — the fifteen that are wholly tests

**Target**: 0.52.0, after `GRD-001`. **The owner settled `NFR-MNT-005`'s
kind 2 on 2026-10-13: move them.** `§10.37` closes with this round.

**Do `GRD-001` first.** It edits the helper half of five of these files;
splitting them first would mean editing each twice.

---

## §1 — what was decided, and the argument that was wrong

`NFR-MNT-005` says tests are separated from implementation files and
in-file `#[test]` modules inside implementation source are not used. It
restates the owner's own `project-instructions-rust.md`, which gives the
shape: `src/some_mod.rs` + `src/some_mod/tests.rs`.

Fifteen files break the literal text. `§10.37` argued the rule should
perhaps carve them out, because *"a module that **is** a test has nothing
to separate tests from"*, and because a scan needs
`env!("CARGO_MANIFEST_DIR")` and a unit test of a private helper cannot
live under `tests/`.

**Measured, that argument does not hold.** Twelve of the fifteen import
nothing from their own crate at all. `optimistic_lock_guards.rs` imports
only public items (`pub mod pool`, `pub mod projects`, `pub type Pool`,
`pub enum Guarded`). Two — `dec_007_ci_scan`, `dec_007_fs_scan` — share
`pub(crate)` helpers with `dec_007_scan`, and that sharing is *between
the scans*, not between a scan and the code under test.

**And every one of the fifteen has helper functions**, from 1 to 29:

| File | helpers | tests |
|---|---|---|
| `peisear-web/src/docs_scan.rs` | 29 | 21 |
| `peisear-web/src/changelog_scan.rs` | 16 | 8 |
| `peisear-web/src/prose_scan.rs` | 14 | 4 |
| `peisear-web/src/touch_target_scan.rs` | 13 | 4 |
| `peisear-web/src/static_js_scan.rs` | 12 | 3 |
| `peisear-web/src/alignment_scan.rs` | 8 | 2 |
| `peisear-web/src/dec_007_ci_scan.rs` | 6 | 2 |
| `peisear-storage/src/optimistic_lock_guards.rs` | 5 | 5 |
| `peisear-web/src/dec_007_scan.rs` | 4 | 1 |
| `peisear-web/src/dm_fallback_boundary_scan.rs` | 4 | 3 |
| `peisear-web/src/test_harness_scan.rs` | 4 | 1 |
| `peisear-web/src/dec_007_fs_scan.rs` | 3 | 1 |
| `peisear-web/src/updated_at_authority_scan.rs` | 2 | 3 |
| `peisear-web/src/contrast_scan.rs` | 1 | 1 |
| `peisear-web/src/one_encoder_scan.rs` | 1 | 1 |

So each one **does** have something to separate the tests from. The
premise that there was nothing was never checked against the files.
`GRD-001`'s own counts will shift `alignment_scan`'s and the naive five's
helper figures slightly; re-measure rather than reusing this table.

## §2 — the shape: the owner's own, not `tests/`

For each of the fifteen, `<name>.rs` keeps the module documentation and
the helpers; `<name>/tests.rs` takes the `#[test]` functions. The parent
gains, at its foot:

```rust
#[cfg(test)]
mod tests;
```

and `tests.rs` opens with `use super::*;`. **This is exactly what the
nine kind-1 files did at `0.51.0`** — `src/config.rs:102-103` +
`src/config/tests.rs:1` is the template; copy its shape, not its
content.

**Every one of these modules is already declared `#[cfg(test)] mod
<name>;` in its crate's `lib.rs`**, so the helpers compile only under
test and nothing becomes dead code. Verified for all fifteen.

**Build cost: none.** They stay in the same lib binary. All 72 of
`peisear-web`'s scan tests run in 0.05 s today and will after. This is
**not** the `tests/` move whose cost the architect quoted at ~2 s of link
each — that destination was the architect's error, found before the
handoff was written; the owner's own prescribed shape is both compliant
and free.

## §3 — the three that need more than a split

- **`dec_007_scan` / `dec_007_ci_scan` / `dec_007_fs_scan`.** The
  `pub(crate)` helpers (`appears_at_word_boundary`, `dec_007_block`,
  `for_loop_targets`) stay in the parent modules, which is where they
  already are. The splits are independent. **Check the three together**
  afterwards: a `pub(crate)` item used only from a sibling's `tests.rs`
  is still `pub(crate)` and still resolves.
- **`optimistic_lock_guards.rs`** is `peisear-storage`, not
  `peisear-web`, and its five tests are `#[tokio::test]`. Same split;
  confirm `peisear-storage`'s CI job still runs them (`--lib`).
- **`contrast_scan` and `one_encoder_scan`** have a single helper and a
  single test each. Split them anyway. **A rule that holds except when
  the file is small is the rule with the carve-out the owner just
  declined.**

## §4 — the two already separated, and one inconsistency to settle

`peisear-core/src/notifications/enumeration_guard.rs` and
`peisear-web/src/components/me/read_first_tests.rs` are the other two of
`0.51.0`'s nine: test-only files under `src/`, already separated from the
implementation they came out of, but named for their subject rather than
`tests.rs`. **Leave both.** They satisfy the text — no implementation
file carries their tests — and renaming them would lose what the names
say.

**One inconsistency to report, not to fix**: `peisear-i18n` puts its
equivalent guard in `tests/enumeration_guard.rs` while `peisear-core`
puts its in `src/notifications/enumeration_guard.rs`. Both are
compliant. Say which you think is right and why, in the review request —
it is a decision for the architect, and `§10.37`'s closing text should
record whichever way it goes.

## §5 — `§10.37` closes, and `NFR-MNT-005`'s status with it

A `DEC-028` amendment, in `requirements.md`:

- **`NFR-MNT-005`'s status moves `Divergent` → `Implemented`**, with the
  live count at zero, measured, not assumed.
- **`§10.37` closes.** Its text currently states the impossibility
  argument as fact — *"a scan reading the source tree needs
  `env!("CARGO_MANIFEST_DIR")`, which only resolves inside the crate
  being scanned; a unit test of a private helper cannot live in
  `tests/`"*. **Correct it rather than deleting it**: the constraint is
  real and applied to none of these files, which is why the entry
  stayed open for a release. The project's own style for a superseded
  claim is a strike-through with a note (`~~§10.21~~`, `DEC-049`'s
  clause 4), not a rewrite.
- **Appendix A's Maintainability row** loses its `Divergent` count.
- The entry should record the general lesson in one line, because it is
  the second time this release family has met it: **an argument for why
  something cannot be done is a claim about the files, and is checked
  against the files.**

## §6 — gates

- `DEC-007`: block extracted **fresh** from `.github/CONTRIBUTING.md`,
  three consecutive runs. **The count must not change.** Moving a test
  between files does not add or remove one, so a different total means a
  test was lost or duplicated in the move — that is a stop, not a note.
  State `GRD-001`'s own post-round baseline and match it exactly.
- `fmt --check`, `clippy --workspace --all-targets -D warnings`,
  `cargo doc` with broken-intra-doc-links denied (8 private-link
  warnings, 0 broken).
- `mdbook build` with linkcheck, for the `requirements.md` amendment.
- No migration. Browser gates not expected to move — say you did not run
  them and why.

## §7 — escalation

- **A split breaks `pub(crate)` resolution** between the three
  `dec_007_*` modules. Report it with the error before working around
  it; the workaround may be the finding.
- **`DEC-007`'s count moves.** Stop. Find the test that was lost or
  gained before continuing.
- **A file turns out to have no helper at all** after `GRD-001` removes
  its copy of `strip_line_comments` — leaving a parent that is only
  `#[cfg(test)] mod tests;`. That is the one case where the owner's
  shape produces something silly, and it is the architect's call, not a
  judgement to make in-round. Report which file and stop on that file
  only.

## §8 — exit condition

Fifteen files split into `<name>.rs` + `<name>/tests.rs`, the kind-1
template followed, every crate's `lib.rs` declaration unchanged;
`DEC-007` matching `GRD-001`'s baseline **exactly**, three runs;
`NFR-MNT-005` at `Implemented` with a measured zero; `§10.37` closed
with its impossibility argument struck through rather than deleted and
the lesson stated in a line; Appendix A's row updated; the
`peisear-i18n`/`peisear-core` guard-location inconsistency reported with
a recommendation and left unfixed.
