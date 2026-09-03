# REL-0.31.0 — release candidate

**Governing RFC**: [012](../../accepted/012-touch-target-conformance.md) — the
whole of it, `TT-001` through `TT-003`.
**Depends on**: nothing outstanding. All three handoffs are reviewed and closed.

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. This one is ordinary

0.30.0 branched from a historical commit because `main` already carried this
work. **0.31.0 does not.** Cut it the way every release before 0.30.0 was cut:
a version-bump-and-changelog commit on `main`'s tip. No branch, no boundary to
get wrong.

- Bump the workspace version to `0.31.0`.
- Write `CHANGELOG.md`'s `[0.31.0]` entry (§3).
- Commit — **`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.**

**No migration.** Nothing under `crates/*/migrations` changed; `0017` remains
the most recent.

## 2. Gates

Full `DEC-007`, cold cache. **Expected: 254.**

`peisear-web --lib` now carries **twelve** scan modules, up from eleven —
`dec_007_fs_scan` is new. If the loop or CI is missing a target, `dec_007_fs_scan`
itself will say so, which is the point of it.

`§10.13`'s note on the repeated-run gate still applies: a `SIGSEGV` in
`peisear-notify` under memory pressure is neither a code defect nor noise to
retry past, and the distinguishing evidence is whether the **same binary**
passes once memory frees — not whether it passes under `cargo test -p`, which
builds a different binary.

## 3. The changelog

This is the substance of the package. `[0.31.0]` covers five things.

1. **Every interactive control the guard covers now presents a 44 × 44 px touch
   target.** 139 of them: 136 grew, three checkboxes gained a `<label>` wrap
   that keeps the box itself at 24 px. Say what a user will notice — some
   controls are larger and some table rows are taller. That is the cost and it
   was accepted deliberately.

2. **What this does *not* claim, and it goes in the same breath, not a footnote.**
   The guard keys off DaisyUI sizing classes and the `checkbox` class. **Plain
   `<a>` links — breadcrumbs, whole-card links, inline text links — carry
   neither, so they are unassessed rather than passing.** WCAG's inline-link
   exception probably covers the text links; *probably* is the accurate word and
   the entry must use it or an equivalent. **Do not write that the product is
   WCAG conformant.** 0.28.0's entry stated plainly that it was not; this
   release improves the position and does not settle it.

3. **`NFR-A11Y-007` is met with a named limit, and Definition of Done item 5
   moves — for the first time since 0.19.1.** It is the oldest condition in that
   table. It moves to **"met, with mobile completion outstanding"**, not to
   "met": `NFR-A11Y-006` is still open. Item 3 drew the same distinction at
   0.29.0 and the reason is the same — the word is worth nothing if it is spent
   early.

4. **Two things found while doing this, both about the project's own gates.**
   These are not user-facing and they belong anyway, on `0.26.0`'s precedent:
   - **Eleven test assertions were passing while no longer testing what they
     name.** Every one was correct when written; none was broken by a commit.
     They decayed because the pages grew a second source of the string they
     checked — a CDN URL containing `5`, the navbar rendering the same link on
     every page, this release's own 139 identical class pairs. **No gate detects
     this and none plausibly could.** All eleven are fixed. Recorded as
     `§10.17`.
   - **A guard's scope was the gap, not its rule.** The guards that watch
     `DEC-007` checked that every target in the documented block has a CI job —
     and nothing checked that every test *file* is in the block. A new test file
     got no CI job silently. Closed by a twelfth scan.

5. **What a reader should not conclude**, folded into the entry rather than
   appended. The adjacency half of the rule — that two targets must not overlap
   — is structurally guaranteed for everything now in the tree, but verifying it
   in general wants rendered geometry this project does not have (`§10.15`,
   external design `§17.6`). And whether text stays vertically centred inside
   the taller inputs and selects is a rendering question for the same reason:
   **the box is provably 44 px; how it looks is not proven.** 65 of the 139 are
   inputs or selects. State it.

**Run `find_violations` over the finished section** and report the character
count.

**A caution specific to this entry**: it is the first release in a while with a
genuine milestone in it, and the temptation to spend the word "met" is real.
Points 2, 3 and 5 all exist to stop that. If the draft reads as an achievement
announcement, it is wrong.

## 4. The tarball

**The archive shape changed at 0.30.0 and the new shape is now the standard.**
Use `--prefix=peisear-0.31.0/` so the archive extracts into its own directory
rather than scattering ~290 files into the current one.

Consequently the file-list check is **prefix-stripped**, and **say so** — 0.30.0's
package described a prefix-stripped comparison as *"diffed directly, zero lines
of difference"*, which was the right check reported as a different one. Strip
the prefix, diff against `git ls-tree -r --name-only <release-commit>`, and
state that the strip happened.

Everything else as before, and **verify by extraction rather than assertion**:

- No `.git-exclude/` anywhere.
- Extracted `Cargo.toml` reads `0.31.0`.
- The extracted tree builds clean.
- **Package-relative checksum** — `sha256sum -c` passes from **inside** the
  tarball's own directory. 0.29.0 regressed on this; 0.30.0 got it right.
- Representative sample inside the extracted tree: **`touch_target`** (the new
  suite), **`confirmation`** and **`board_keyboard`** (the two whose own
  assertions this work rewrote).

## 5. Post-publication — state as pending

Not yours to run. The tag is bare **`0.31.0`** — no `v` prefix; 0.30.0's package
hedged on this and it is settled. All seven crates at `max_version` `0.31.0`
(`DEC-047`). **No `gh release create`.**

## 6. `DEC-028` is already satisfied

The baseline and external design amendments for this release are **written and
in place** — `NFR-A11Y-007`'s status, `§10.16`'s reopening and re-closure,
`§10.17`, Definition of Done item 5, external design `§5.7` and `§17.7`. That is
architect work and no part of it is this package's.

**This is the first release where the amendment preceded the cut** rather than
following it. Nothing for you to do; noted so the package does not report it as
outstanding.

## 7. Escalate rather than deciding

- **If the count is not 254.**
- **If `find_violations` flags the changelog** — copy is not yours to rewrite.
- **If the entry cannot be written without claiming more than §3.2 allows** —
  that is a real escalation, not a drafting problem.

---

**Who holds what**: dev team — the candidate. **What's blocked**: the tag and
publication, architect-side after the owner approves. **What's next**: review
request.
