# DOCS-002 — nine broken links in the published API documentation

**Issued by**: Architect
**Date**: 2026-10-08
**Target release**: **0.44.0**, unless §4's count says otherwise.
**Source**: found while reviewing `PRIV-004`, whose move introduced the
twenty-third. Ruled in `.git-exclude/reviewed/PRIV-004-review.md` §3.
**Depends on**: nothing.

---

## 1. What is there

`cargo doc --workspace --no-deps` emits **23 warnings**, of which nine are
unresolved intra-doc links:

`UserLoad` (×2) · `UserBurnoutSignals` · `projects_in_team` ·
`move_issue_to_sprint` · `latest_status_change` ·
`HealthIndicator::badge_class` · `DEFAULT_PREFERENCES` ·
`users_with_active_assignments` (`PRIV-004`'s, fixed in that package)

The rest are public documentation linking **private** items — a different
class, and arguably fine, but each one renders as plain text rather than a
link for a reader who cannot see the target.

**These are published.** Seven crates go to crates.io and **docs.rs builds
their documentation**, so each broken link is live in the API docs of every
released version. **Nothing has ever looked**: `clippy --all-targets` does not
check doc links and `cargo doc` is in no gate.

## 2. What to do

**2.1 — Count first, and report before fixing.** Nine is the unresolved-link
count from one run. Establish:

- how many are a **renamed or moved** item that has a correct target,
- how many name something that **no longer exists**,
- how many are **public-links-private**, and for each whether the right answer
  is to make the target public, to unlink it, or to leave it.

**If the total is small, fix it and the gate in one package** and say so;
`§4` revisits the release target on your number.

**2.2 — The gate.** `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` in a
CI job, **and say which file it goes in.** `dec_007_ci_scan` reads `test.yml`
by name, so a new job there is in its scope and a separate workflow is not —
`docs.yml` already exists for the book and may or may not be the right home.
**Report which you chose and why.**

- **Do not deny `rustdoc::private_intra_doc_links`** until §2.1 has decided
  what to do about that class. Denying a lint before its backlog is cleared
  makes the build red on purpose, which is how a gate gets disabled.
- **Pin nothing new.** This needs no tool beyond the toolchain.

## 3. What this is not

**Not a documentation rewrite.** No prose changes, no new doc comments, no
restructuring. Every change is a link target or a visibility, and the
diff should read as exactly that.

**Not the mdbook site** (`DEC-057`), which publishes `docs/` and is already
gated on its own broken links. This is rustdoc, which nothing gates.

## 4. Verification

- **§2.1's counts, by class, reported before any fix.**
- `cargo doc --workspace --no-deps` with the lint denied: **exit 0**.
- **The gate seen to fail**: plant one broken link, confirm CI's command
  rejects it, revert byte-identical.
- **No public item's visibility changed without being named** — if a private
  item is made public to satisfy a link, that is a published-API change and it
  goes in the changelog like any other.
- `DEC-007` reported, last recorded **367** — expect unchanged; nothing here
  is a test. `fmt`, `clippy`, three consecutive workspace runs. The overflow
  gate is untouched.

## 5. Escalate rather than deciding

- **If the count is much larger than nine** once the other classes are
  included — that changes the release this belongs in and it is mine.
- **If a link's correct target does not exist**, because the item was removed.
  The answer may be to delete the sentence, which is a prose change this
  handoff otherwise forbids. **Report it; do not guess.**
- **If making a target public would widen the API** meaningfully.
- **If `dec_007_ci_scan` objects** to wherever the job lands.

## 6. Exit condition

`cargo doc --workspace --no-deps` is clean under
`-D rustdoc::broken_intra_doc_links`, a CI job holds it, that job has been
seen to fail, and any visibility change is named for the changelog.
