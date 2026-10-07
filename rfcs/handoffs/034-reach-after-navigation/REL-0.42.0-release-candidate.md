# REL-0.42.0 — release candidate

**Contents**: `A11Y-006` (`4f56154`), `A11Y-007` (`568fa4c`), `A11Y-008`
(`a539980`), `A11Y-009`, and the record work — `NFR-A11Y-002` and
`NFR-A11Y-004` to Met, **`NFR-A11Y-010` added**, and the vendoring README's
note on reading a regeneration diff. All four handoffs reviewed and closed.
**`DEC-028` is already done** — both specifications cover 0.42.0 (`cf61f5c`).
**Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration** — `0020` remains the most recent, the third release running.

**Expected: 369, unchanged.** Four handoffs and no net new tests: `A11Y-006`
added none and **rescoped one that had gone stale**, and the other three are
measurements rather than assertions. **That is unusual and §3.5 says what to
do about it** — do not present it as thin.

**`static/tailwind.css` was regenerated twice** (`A11Y-006`, `A11Y-008`).
**Both are additive, zero rules removed, same Tailwind version** — I checked
both diffs. `style/tailwindcss/README.md` now records why three of the added
rules correspond to no class anyone wrote.

**The overflow gate is 120 cells**, unchanged, and it was run for three of the
four handoffs with the reason given for the fourth.

## 2. The changelog

**This release is small in diff and finishes something**, which is the shape
to write to. Six things.

1. **Lead with the skip link**, because it is the only change a reader meets on
   every page: **eleven Tab presses to reach the content becomes one press and
   an activation**, hidden until focused, and **it works with scripting off** —
   which is why it was chosen over the two alternatives the requirement
   offered.

2. **The charts are legible now.** Their two series were **1.77 : 1** apart and
   one bar **1.86 : 1** against the page; both are 3 : 1 or better. **Say who
   this was failing**: not a colour-blind reader — both charts are single-hue
   and always were — but anyone with low contrast sensitivity, to whom two pale
   blues were one pale blue.

3. **A focus ring that was drawn in transparent.** Four links in the account
   menu, measured at **1.22 : 1**, now drawing what the rest of the page
   already draws.

4. **`NFR-A11Y-010` is a new requirement**, not a fix: WCAG 1.4.11's 3 : 1 for
   graphical objects, which this project had **only for text**. Say why it
   appeared — `NFR-A11Y-004`'s own rewording turned out not to cover the
   finding it had been given as a status.

5. **The honest paragraph, and it is about this document rather than the
   code.** Two requirement statuses were corrected because **I had written
   them wrong**: one contradicted its own body, one was written against a
   finding rather than the sentence it sat under. **Third and fourth instances
   of one habit** — amending an entry and leaving the `*Status*` field alone.
   **Do not soften it into "documentation updated".**

6. **The test count did not move**, and that is the thing most likely to be
   misread. **Say why**: the work was measurement — six hit-test points, two
   independent colour conversions, fourteen computed focus styles — and
   measurement closes a requirement without adding an assertion. `A11Y-007`
   explains in the record why a pinning test was declined rather than skipped.

**What a reader should not conclude**, folded in:
- **`§10.15`, `§10.17` and `§10.32` are open**; `§10.32` by decision.
- **Nothing graphical other than the two charts has been audited** for
  contrast — `NFR-A11Y-010` says so in its own status, and the changelog should
  not imply a sweep.
- **Undo is still absent from the keyboard route** on the board, plan and
  calendar (`FR-DM-005`), and the lost scroll position after a phone POST is
  still lost.
- Chrome only; no screen reader has been used.
- No WCAG conformance claim — **and this is the release where that sentence
  needs care**, because two WCAG criteria were just cited by number.

**Run `find_violations`** and report the character count. **"velocity" and
"failed to" are both prohibited** — the second caught the 0.41.0 draft.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`cf61f5c`). Confirm and say so.
- `DEC-007` **369**, three consecutive. Block and `test.yml` unchanged.
- **The overflow gate on the extracted tree: 120 cells.**
- **`mdbook build` from the extracted tarball** — second release with the site.
- `fmt`, `clippy --workspace --all-targets -D warnings`.
- **The skip link on the built artefact**: one Tab press and an activation
  reaching `main`, **with scripting disabled**. It is the release's headline
  claim and the one a reader can test in ten seconds.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.41.0** binary and confirm it comes up at 20 with nothing
  applied.
- `cargo publish --workspace --dry-run` from a detached worktree; tarball
  sha256 and the `git ls-tree` comparison.

## 4. Escalate rather than deciding

- **If the count is not 369**, or the gate is not 120.
- **If `find_violations` flags anything** — report the phrase.
- **If the skip link does not work on the artefact with scripting off.**
- **If `mdbook build` fails from the tarball.**

## 5. Exit condition

A candidate commit on `main`, the changelog written as a standalone
announcement, 369 green three times, the gate at 120, the book building from
the tarball, the skip link demonstrated on the artefact with scripting off,
the 0.41.0 database coming up clean, and the dry run clean. Then stop.
