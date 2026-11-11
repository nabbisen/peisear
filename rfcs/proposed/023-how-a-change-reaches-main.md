# RFC 023: How a change reaches `main` — the policy this project never wrote

**Status**: proposed 2026-11-11. **Decision required from the human
owner** — this depends on policy and governance, not on technical
judgement alone. **Author**: the architect. **Supersedes nothing.**

## Summary

peisear has **no written branching or version-control policy**. What it
has instead is a contributor document addressed to people outside the
repository, which the dev team correctly read as the only existing
convention, and an architect who has pushed straight to `main` for
**18 of `main`'s last 25 commits** while the dev team opened a pull
request for the other **7**.

The owner met the consequence on 2026-11-11: a branch in their own
checkout they had not created, and a merge step that reached them for
approval. Neither is a defect in a round. Both follow from the missing
policy, and this RFC asks for it to be decided.

## §1 — what was measured, 2026-11-11 at `6d2ad33`

**Verified facts**, each re-checked for this RFC rather than recalled:

- **No branching or version-control policy exists** — not in this
  repository, and not in the project's private working rules, which say
  only that both models are authorized to commit and push and name no
  branch. The project's governance baseline requires each project to
  supplement it with a branching and version-control policy of its own.
  peisear never did.
- **`.github/CONTRIBUTING.md` § *The pull request process* is the only
  written git workflow the repository has.** Its first step is *"Fork
  the repository and create a topic branch from `main`"* — written for a
  contributor outside the repository, and nowhere does it say so.
- **`main` is not protected.** `GET /repos/nabbisen/peisear/branches/main/protection`
  answers *Branch not protected*. The branch was convention, never a
  constraint.
- **Of `main`'s last 25 commits, 7 arrived through a pull request and 18
  were direct pushes.** All 7 are the dev team's; all 18 are the
  architect's.
- **The practice has already leaked into documentation as if it were a
  rule**: `docs/development/changelog-and-releases.md` calls it *"this
  project's own squash-per-round convention"* and records a
  contradiction it could not resolve — the series-archive move is
  specified to land *"in its own commit"*, which a squash cannot honour,
  with the resolution left open for a future series boundary.
- **`origin` carries two branches**: `main`, and `release-0.30.0`, which
  is an ancestor of `main` and therefore holds nothing `main` does not.

**Inference, not fact**: the dev team's standing instruction is to use
existing project conventions unless a handoff explicitly changes them.
The pull request is what they found written down. **The absence of a
policy is the defect; their reading of it was correct.**

## §2 — the divergence this RFC exists to surface

The lifecycle this project is run under ends an implementation round at
*submit a review request* — not at integration — and makes the
architect's review the following phase, whose outcomes include
**corrections required** and **design reconsideration required**.
Review precedes acceptance.

**peisear has not worked that way for some time, and never wrote it
down.** PR #47 and PR #48 were merged by the dev team before the
architect reviewed them, and the review of #47 produced a finding that
became an entire second round — after the change was already on `main`.
PR #49 is the only round reviewed before its merge, and only because a
classifier blocked the merge.

So **the pull request has never been the review gate here.** The gate
that protects a user of this product is the release cut: the candidate
pass, and the owner's approval at the release cycle's own exit
criterion. Whichever option below is chosen, this divergence from the
baseline lifecycle must be **written down and owner-approved** rather
than left implicit — that requirement is the governance baseline's own,
and it has been unmet for several releases.

## §3 — the options

### Option A — `main` only

Two sentences, and nothing else:

1. **Both models commit and push to `main`.** No topic branch and no
   pull request for work either of them does. Contributions from outside
   the repository keep fork-and-pull-request, which is what
   `CONTRIBUTING.md` is for.
2. **Whoever pushes watches `main`'s own CI and Docs to completion.** A
   red `main` is that person's only work until it is green — forward fix
   or revert.

**Benefits.** Six mechanical steps per round disappear (create a branch,
push it, open a pull request, write its description, wait on its CI,
merge and delete the branch) and none is added. No merge step can reach
the owner again, so the blocking class that stopped this round cannot
recur. The two roles become symmetric. *"Squash-per-round"* stops
existing, and with it the open archive-move contradiction in §1: the
move simply **is** its own commit. It matches the owner's own stated
model, and the project's private rule — *authorized to commit and
push* — becomes true as written.

**Drawbacks.** `main`'s tip stops being guaranteed green. A bad change
is **reverted** rather than left unmerged. Review stays *after*
integration (§2), so a finding becomes a correction round on `main`,
which is already how the last two rounds with findings were handled.

### Option B — a branch, CI on the branch, no pull request

Push a topic branch; CI triggers on branch pushes (a workflow change —
today `test.yml` and `docs.yml` fire on `push` to `main` and on
`pull_request` only); fast-forward `main` once it is green.

**Benefits.** `main` stays green. No merge button, so no classifier.

**Drawbacks.** **Not simpler**: two pushes per round, a workflow trigger
change, and 52 jobs on *every* branch push rather than on `main`'s own
commits. Review is still after integration. Nothing records the round
but the commit, so the pull request's one real by-product is lost
without its cost being lost.

### Option C — the status quo, written down

Branch and pull request per round; add a permission rule so the merge
never needs the owner; optionally require the architect's review
*before* the merge, which would restore the lifecycle's own ordering.

**Benefits.** The **only** option that can restore
review-before-acceptance. `main`'s history stays green. CI runs on a
change before `main` carries it — including the checks nobody can run
locally: clippy on Windows, the MSRV build, `cargo publish --dry-run`,
the `cargo install` walk, and the mdbook link check.

**Drawbacks.** Keeps all six mechanical steps and the asymmetry between
the two roles. Needs a permission rule to stop reaching the owner. And
if the review genuinely precedes the merge, **every round gains a
blocking wait** on the architect between the dev team finishing and the
work landing.

## §4 — risks

- **A and B**: `main`'s tip may be red, so a release must **verify**
  green at the cut rather than assume it. That is already a release
  cycle criterion; it is restated here so it is never assumed. The
  precedent is `§10.54` — and it happened on a **direct push with nobody
  watching the run**, which is exactly what Option A's second sentence
  is for.
- **A**: a defect found in review is already on `main`. Bounded by the
  release gate (§2), and by the fact that every round's own gates run
  before the push.
- **C**: the merge needs a permission rule, and the one blocking step
  recurs every round. A permission rule is also a standing widening of
  what an agent may do unattended.
- **All three**: a policy nobody writes down reverts to whatever
  document is nearest. That is precisely how this happened, and it will
  happen again with the next unwritten convention.

## §5 — schedule impact

None on `0.62.0`'s authorized scope. The next round, `NAME-001`, is
specified and unstarted, so the decision changes how that round *ends*
and is worth making before it begins. No release leg depends on it. The
documents it touches are documentation only.

## §6 — recommendation

**Option A.**

1. It is the simplest rule that covers the case, and this project's
   stated preference is for simple and clean rules.
2. The gate that protects a user is the release cut, not the merge —
   and §2 shows the merge has not been that gate here for several
   rounds.
3. The asymmetry is the real defect. One rule for both roles removes
   the class of surprise the owner hit, rather than one instance of it.

**The honest argument against A, stated because it is real**: only
Option C can restore review-before-acceptance, which is the baseline's
own ordering. It is declined because the protection it would add is
already provided — later, and more thoroughly — by the candidate pass
and the owner's approval at the cut, and because it buys that protection
with a blocking wait in every round and a standing permission widening.

## §7 — if A is approved, what gets written

1. This RFC moves to `accepted/` and declares **`DEC-060`**: the policy
   (§3 A) **and** the §2 divergence, documented as the baseline requires.
2. `.github/CONTRIBUTING.md`: the pull-request section scoped to its
   real audience — contributions from outside the repository.
3. `docs/development/changelog-and-releases.md`: the *squash-per-round*
   paragraph and its open archive question replaced by one line.
4. One sentence in the project's private working rules, in the owner's
   own words, if the owner wants it recorded there as well.
5. `release-0.30.0` deleted from `origin`, on the owner's word, as
   residue of a policy this project no longer has.

No requirement changes and no `ROADMAP.md` row: this is governance and
documentation, not product scope.

## §8 — consequences of deferring

Nothing breaks. The next round ends the way this one did — a branch the
owner did not create, and a merge step that may reach them again — and
the §2 divergence stays undocumented, which is the part that compounds:
it is the kind of difference that is cheap to write down now and
expensive to reconstruct later.
