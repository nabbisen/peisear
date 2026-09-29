# REL-0.41.0 — release candidate

**Contents**: `REQ-002` (`edea18c`), `A11Y-002` (`d2cc17e`), `NTF-001`
(`bf4fbd3`), `A11Y-004` (`86ed166`), `DOCS-001` (`940a605`), `A11Y-005`
(`d04d881`), and the record work — `DEC-055`, `DEC-056`, `DEC-057`, the
`A11Y-001` rulings, `NFR-REL-007`'s correction, and 0.40.0's four malformed
entries repaired. All reviewed and closed.
**`DEC-028` is already done** — both specifications cover 0.41.0 (`9b47164`).
**Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration** — `0020` remains the most recent, the second release running.

**Expected: 369**, up from 0.40.0's 352. **No new test file**, so `DEC-007`'s
block is unchanged; **`test.yml` did gain a job** — `docs.yml` is a *second
workflow*, not a change to `test.yml`, and `dec_007_ci_scan` reads the latter.
**Confirm that scan passes and say so**, because this is the first release that
adds a workflow.

**The overflow gate is 120 cells.** Unchanged since 0.40.0.

## 2. This release is the first under `DEC-056` — four legs

**The release procedure changed and this candidate is where it first applies.**
`docs/development/changelog-and-releases.md` is normative: tag · push the tag ·
`cargo publish --workspace` · **create the GitHub Release with the `[0.41.0]`
section as its body**.

**Nothing in that is yours** — all four legs are the architect's, on the
owner's approval. **It is here so the candidate is prepared for it**: the
changelog section you write *is* the Release body, so it has to read as a
standalone announcement and not as a diff against the previous section.

## 3. The changelog

**Almost nothing a user can see changed — again — and this time the reason is
worth stating rather than apologising for.** Six things.

1. **Lead with the two defects**, because they are concrete and both were
   reachable by an ordinary person:
   - **The team role control committed on an arrow key.** A keyboard user
     changing someone Admin → Viewer **demoted them to Member on the way**.
     It now has a Save button — **and that is also the first time the role can
     be changed without JavaScript**, because the form had no submit control
     at all.
   - **Undo was not reachable from the keyboard**: 15 Tab presses on the issue
     list, inside a five-second window. The toast now sits beside the control
     it belongs to — three presses — and returns focus when it ends.

2. **`/settings/notifications` loses a row**, and say why it is a fix and not a
   removal: it offered preferences for a notification **that can never fire**,
   and its presence **held back the *everything is silenced* banner** until a
   user silenced something that did not exist.

3. **The documentation is now a published site** (`DEC-057`), and **each
   release now carries its notes on its GitHub Release page** (`DEC-056`).
   Both are new practice, not retrofits — **no back-fill of earlier releases.**

4. **Two P1 requirements stopped being unfalsifiable**, in one sentence for a
   reader who does not follow the register: they recorded *"Partial."* and
   nothing else, so nothing could be checked and nothing could ever be wrong —
   and auditing them is what found the two defects in §3.1.

5. **The honest paragraph.** `NFR-REL-007` was recorded **Implemented** and
   neither half of it was true. **Second false `Implemented` in three
   releases**, and **the first confirmed instance of the blind spot `REQ-001`
   named in its own words** — its sweep covered entries claiming
   incompleteness, and said in terms that it proved nothing about the 89
   claiming completeness. **Do not soften this into "documentation improved".**

6. **0.40.0's four malformed requirement entries are repaired**, and the
   reason they are mentioned at all: they were **this project's own mistake**,
   published, and the practice is to name them rather than fix them quietly.

**What a reader should not conclude**, folded in:
- **`§10.15`, `§10.17` and `§10.32` are open.** `§10.32` is open **by
  decision**.
- **The charts still separate their series by lightness alone** (1.77 : 1) with
  no patterning — recorded in `NFR-A11Y-004`, not fixed here.
- **Undo is still absent from the keyboard route on the board, plan and
  calendar**, where the keyboard path is a native form POST that shows no
  toast. Recorded against `FR-DM-005`; **not** an `FR-DM-002` violation, and
  say so, because an earlier draft of mine claimed it was.
- No WCAG conformance claim.

**Run `find_violations`** and report the character count. **"velocity" is still
prohibited.**

## 4. Verification before the candidate commit

- **`DEC-028`: already done** (`9b47164`). **Confirm and say so.**
- `DEC-007` **369**, three consecutive; **`dec_007_ci_scan` green with the new
  `docs.yml` present** (§1).
- **The overflow gate on the extracted tree: 120 cells.**
- **`mdbook build` from the extracted tarball** — the site is new and the
  tarball is what a packager gets. Report whether it builds outside the working
  tree.
- `fmt`, `clippy --workspace --all-targets -D warnings`.
- **No migration to exercise**, but start the candidate binary on a database
  built by the released **0.40.0** binary and confirm it comes up at migration
  20 with nothing applied.
- `cargo publish --workspace --dry-run` from a detached worktree.
- Tarball sha256 and the `git ls-tree` comparison.

## 5. Escalate rather than deciding

- **If the count is not 369**, or the gate is not 120, or `dec_007_ci_scan`
  objects to `docs.yml`.
- **If `mdbook build` fails from the extracted tarball.**
- **If `find_violations` flags anything** — report the phrase.
- **If §3.1's two claims are not true of the built artefact.** They are the
  only user-visible claims in the section, and both are keyboard behaviour —
  drive them.

## 6. Exit condition

A candidate commit on `main`, the changelog written as a standalone
announcement, 369 green three times, the gate at 120, the book building from
the tarball, the 0.40.0 database coming up with no migration applied, and the
dry run clean. Then stop and hand it back.
