# REL-0.30.0 — release candidate

**Governing RFC**: [011](../../accepted/011-browser-verification.md), step 2
**Contents**: `JS-003` only — both rounds.
**Depends on**: nothing outstanding. `JS-003` is reviewed and approved.

**Do not tag. Do not publish.** Produce the candidate and stop. The owner
approves; the architect executes the tag, the merge, and `cargo publish`.

## 1. Read this section before running any git command

**This release does not branch from `main`'s HEAD, and that is the whole
difficulty.**

`main` currently carries `JS-003` **and** all of RFC 012 (the touch-target work,
`TT-001`–`TT-003`). The owner has split them: **0.30.0 is `JS-003` alone; RFC
012 becomes 0.31.0.** So the 0.30.0 tree must **not** contain RFC 012.

The boundary is exact:

| | Commit |
|---|---|
| Previous release | `de521d2` — "Release 0.29.0" |
| **Last `JS-003` commit — branch from here** | **`7845751`** — "JS-003 round 2: the reload flags checked…" |
| First RFC 012 commit (must be excluded) | `da13420` |

**What to do:**

1. `git switch -c release-0.30.0 7845751`
2. Bump the workspace version to `0.30.0` and write `CHANGELOG.md`'s `[0.30.0]`
   entry (§3).
3. Commit — **version bump and changelog only**, nothing under `crates/` or
   `static/`.
4. Push the branch. **Do not tag. Do not merge to `main`.**

**One commit is deliberately excluded, and the reason matters**: `2bf34a9`
("RFC 011: step 2 shipped at 0.29.0…") sits between `7845751` and `da13420`. It
was **wrong** — it claimed `JS-003` had shipped in 0.29.0 when its commits
post-date that tag — and `main` already carries the correction. Branching from
`7845751` skips it, which is the right outcome; the RFC on this branch will read
with its original, correct 0.30.0 target.

**If the branch point looks wrong to you, stop and say so** rather than adjusting
it. Getting this boundary wrong ships RFC 012 inside 0.30.0, and a published
crate cannot be unpublished.

## 2. Gates, on the branch

Full `DEC-007` at `release-0.30.0`, cold cache:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- The per-crate/per-target loop
- **Three consecutive `cargo test --workspace` runs**

**Expected: 245.** That is 240 at the 0.29.0 tag plus `JS-003`'s five. If you
see 254 you have branched from the wrong commit — 254 is `main`, which includes
RFC 012's nine.

**On the repeated-run gate**: `§10.13` now records that this gate can fail with a
`SIGSEGV` in `peisear-notify` under memory pressure, that this is not a code
defect, and that it is also **not noise to retry past**. The distinguishing
evidence is whether the *same binary* passes once memory frees — **not** whether
it passes under `cargo test -p`, which builds a different binary. If you hit it,
report it the way you did at `JS-003`: stop, evidence, ask.

## 3. The changelog

`[0.30.0]`. `JS-003` is mostly deduplication, and **the entry must not inflate
it**. One authority replacing three copies is worth stating plainly; it is not a
feature.

Four things to cover:

1. **The board no longer passes a failed status change over in silence.** This
   is the only user-visible behaviour change and it should lead. Previously a
   `2xx` response with an unusable body left the card sitting in its new column
   with a stale lock value and no announcement — so the *next* drag of that card
   produced an unexplained "someone else changed this". It now announces and
   refreshes. **State it as the fix it is**; it was latent, never reported, and
   saying so is more useful than implying users were hitting it.
2. **New copy for the case where the outcome is unknown.** *"This status change
   may not have completed…"* replaces text that said *"could not be
   completed"* — which asserted two things the code could not support, since the
   server had returned `2xx` and the refresh could show the card in its new
   column moments later. Say what the copy now does: **it says only what is
   known.**
3. **`409` is no longer written in JavaScript at all.** The classification is
   authored in Rust and read as data. Internal, one sentence, no adjectives.
4. **What a reader should not conclude.** `§10.15` is still open. The shipped
   JavaScript is still executed by no test; this shrank the untested surface, it
   did not remove it. Fold this into the entry rather than appending a
   disclaimer — `0.26.0`'s precedent.

**Run `find_violations` over the finished `[0.30.0]` section** and report the
character count, as every release since 0.26.0 has.

## 4. The tarball

`git archive` at the release commit, into `evidence/`, with a **package-relative
checksum** — `sha256sum -c` must pass from **inside** `evidence/tarball/`.

This has been the correction on more than one release: 0.23.0 got it wrong,
0.24.0–0.28.0 got it right, **0.29.0 regressed to a repository-root-relative
path**. It is the single most-repeated defect in this project's release history.

Verify by extraction, not by assertion: file list identical to
`git ls-tree -r --name-only <release-commit>`, no `.git-exclude/`, extracted
`Cargo.toml` reads `0.30.0`, the extracted tree builds clean, and a
representative sample runs inside it. **`response_outcomes` and `status_control`
are the right sample this time** — the suite that is new and the surface that
changed.

## 5. Post-publication check — state as pending

Not yours to run. State the three steps as pending: the tag on the remote, all
seven crates at `max_version` `0.30.0` (`DEC-047`), and any crate that did not
land named before the release is called done.

**No `gh release create`.** It has never been part of this project's release.

## 6. What is not in this release, and must not be added

- **RFC 012** — the entire touch-target body of work. 0.31.0.
- **The baseline and external design amendments.** Mine, and they are written
  against 0.31.0 for RFC 012's material; `DEC-028`'s criterion is satisfied at
  the release by my own work, not this package's.

## 7. Escalate rather than deciding

- **If the test count is not 245.**
- **If the branch point produces a diff you did not expect** — `git diff
  0.29.0 7845751 --stat` should show `JS-003` and RFC 011 documents only.
- **If `find_violations` flags the changelog.** Copy is not yours to rewrite.

---

**Who holds what**: dev team — the candidate on `release-0.30.0`. **What's
blocked**: the tag, the merge to `main`, and publication, all architect-side
after the owner approves. **What's next**: review request.
