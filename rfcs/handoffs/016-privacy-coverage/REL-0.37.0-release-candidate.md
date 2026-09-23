# REL-0.37.0 — release candidate

**Contents**: `PRIV-001` (`ad9c941`) — five authorisation assertions and one
corrected comment. Reviewed and closed. **Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip. `Cargo.toml`,
`Cargo.lock`, `CHANGELOG.md` only.

**No migration.** `0017` remains the most recent.

**Expected: 272**, up from 0.36.0's 268 — four new test functions across two
existing files, so `DEC-007`'s command block is unchanged.

**No product behaviour changed at all this release.** Not a line. That is
unusual and §2 is mostly about saying so without making it sound like nothing
happened.

## 2. The changelog

**This release is a privacy milestone and a published correction, and the
danger is the first one.** Five things.

1. **Definition of Done item 1 — "Privacy maintained" — is Met**, for the
   first time. It has read *"Largely met"* since 0.19.1, which makes it the
   second-oldest condition in that table to move, after item 5 at 0.33.0.

2. **Bound it in the same breath, because this is the sentence a reader will
   over-read.** *The product did not become more private this release.* Every
   boundary this work touched already refused correctly; each was measured
   before a test was written for it. **What changed is that the behaviour is
   now asserted, and that the project's own record stopped disagreeing with
   itself.** An entry that lets a reader think a hole was closed would be
   claiming something untrue about the releases that came before it.

3. **The published correction, and it is the substantial half.** The 0.20.0
   entry withdrew an ignored test and said it *"asserted a boundary that cannot
   exist while settings mutations (`/settings/wip-limit`,
   `/settings/capacity/*`) are session-scoped rather than addressed by
   `user_id` in the path."*

   **Withdrawing the test was right** — an ignored test on a privacy boundary
   reads as coverage that does not exist, and that entry said so. **The reason
   was half wrong.** It holds for `/settings/wip-limit`, which takes no path
   parameter, so there is nothing to substitute. It does not hold for
   `/settings/capacity/{id}`, which names a **row**: asking for someone else's
   row is exactly the cross-user request, the boundary is real, and it refuses.
   Three tests now make that attempt. **Say the action was right and the reason
   was wrong** — that is the accurate pair and it is more useful than either
   half alone.

4. **What the five assertions are**, in one sentence rather than a list: the
   unauthenticated case for two of the three personal-data endpoints, and the
   cross-user attempt on the three capacity-row mutations.

5. **What a reader should not conclude**, folded in:
   - **No behaviour changed**, no schema changed, nothing a user can see is
     different.
   - **The suite grew by four**, and the value is in what they would catch
     later, not in anything they found now.
   - `§10.15` and `§10.17` remain the two open register entries, both open by
     decision.
   - The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

**The caution.** Previous entries risked reading as an achievement, an apology,
*"layout is now covered"*, paperwork, claiming reach, and a programme. **This
one risks the most consequential misreading of the set: that the product was
less private last week.** It was not. The guarantee is the same; the evidence
for it is new. Every sentence about the milestone needs the bound attached to
it rather than parked in a later paragraph — and equally, the entry should not
shrink into an apology for having found its own record wrong. **The useful
frame is that a claim this project had been making cautiously turns out to be
true, and is now tested.**

## 3. The tarball

`--prefix=peisear-0.37.0/`. **Package-relative checksum** — `sha256sum -c`
passes from inside the tarball's own directory. File list **prefix-stripped**
against `git ls-tree -r --name-only <commit>`, and **say that the strip
happened**.

**`static/tailwind.css` must be unchanged this release** — the check inverts
back from 0.36.0. No class changed, nothing was added to the markup, so a
difference would mean a stray regeneration. Confirm it matches 0.36.0's cut
(`9f4556b9…`) as well as `HEAD`.

**No new file under `static/`**, for the first time in three releases.

**Run the gate from the extracted tree**: `cargo build -p peisear` inside it,
then `node browser-checks/overflow-gate.mjs` — **90/90, exit 0, 0 non-local
requests**. `CARGO_TARGET_DIR` on disk, not under `/tmp`.

Representative tests from inside the extracted tree: **`auth_boundary`** (17),
**`optimistic_lock`** (19), `smoke`.

## 4. Post-publication — state as pending

Bare tag `0.37.0`. Seven crates at `max_version` `0.37.0` (`DEC-047`).
**No `gh release create`.**

## 5. `DEC-028` — architect work, already done

Both baselines carry `NFR-PRIV-008` Met, Definition of Done item 1 Met, the
corrected count of five, and the external design's status table distinguishing
a user id in the path from a resource id. The rebase to `0.37.0` is mine and
blocks the tag, not this candidate.

## 6. Escalate rather than deciding

- **If the count is not 272**, or `DEC-007`'s block needs an edit.
- **If `static/tailwind.css` differs from 0.36.0's cut.**
- **If the gate from the extracted tree is not 90/90.**
- **If `find_violations` flags the changelog.**
- **If §2.2's bound reads as a hedge rather than a fact.** It is the sentence I
  expect to be hardest to place, because it has to sit next to a milestone
  without deflating it, and saying so is a useful review result.

---

**Who holds what**: dev team — the candidate. Architect — the baseline rebase,
then tag and publication after the owner approves. **What's next**: review
request.
