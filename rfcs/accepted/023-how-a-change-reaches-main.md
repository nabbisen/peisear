# RFC 023: How a change reaches `main`

**Status**: accepted 2026-11-11. **Declares `DEC-060`.**
**Author**: the architect. **Supersedes nothing.**

Decided by the owner in their own words: *"Just use the `main` branch,
shared. Of course, there can be exception(s)."*

## `DEC-060` — the policy

1. **Both models commit and push to the shared `main`.** No topic
   branch and no pull request for work either of them does.
2. **Whoever pushes watches `main`'s own `CI` and `Docs` to
   completion.** A red `main` is that person's only work until it is
   green.

**The exceptions, named so that "exception" does not become a
loophole:**

- **Contributions from outside the repository** keep
  fork-and-pull-request. That is what `.github/CONTRIBUTING.md`
  describes, and the only reason it mentions branches.
- **A shipped series that needs a patch after `main` has moved on**
  gets a release branch. `release-0.30.0` is the one precedent.
- **A local branch or worktree that is never pushed** — a plant, an
  isolated build, a measurement against an older tree. It touches
  nothing shared, and the architect's reviews depend on it.

Anything that is not one of those three is rule 1.

## §1 — why a one-line rule needed writing down

**No branching policy existed.** Not in this repository, and not in the
project's private working rules, which say only that both models are
authorized to commit and push and name no branch.

So the only written git workflow here was `.github/CONTRIBUTING.md`
§ *The pull request process*, whose first step is *"Fork the repository
and create a topic branch from `main`"* — addressed to someone outside
the repository, and nowhere saying so. The dev team's standing
instruction is to use existing project conventions unless a handoff
changes them, and that is the one they found written down. **The
absence was the defect.**

Measured 2026-11-11 at `6d2ad33`: of `main`'s last 25 commits, **18
were direct pushes (all the architect's) and 7 arrived through a pull
request (all the dev team's)**, and `main` carries **no branch
protection** — so the branch was convention, never a constraint. Two
rules for one repository, in which the two roles never work
concurrently.

## §2 — what the pull request was not: a review gate

Recorded once, here, because it is the one thing in this change that is
not about branches.

The lifecycle this project is run under ends an implementation round at
*submit a review request*, not at integration, and makes the
architect's review the next phase — with *corrections required* among
its outcomes. **peisear has not worked that way for some time and never
wrote it down**: PR #47 and PR #48 were merged by the dev team before
the architect reviewed them, and the review of #47 produced a finding
that became an entire second round, after the change was on `main`.
PR #49 is the only round reviewed before its merge, and only because a
classifier blocked the merge.

**The gate that protects a user of this product is the release cut** —
the candidate pass, and the owner's approval at the release cycle's own
exit criterion. `DEC-060` does not move that gate; it stops the pull
request from implying a gate it never was.

## §3 — the cost accepted with rule 1

`main`'s tip is no longer guaranteed green. A bad change is **reverted**
rather than left unmerged, and a review finding becomes a correction
round on `main` — which is already how both rounds with findings were
handled. Release approval therefore **verifies** green at the cut
rather than assuming it, as the release cycle already requires.

Rule 2 is what carries that cost, and it is not new: `§10.54` is a red
`main` that sat unnoticed for a day **after a direct push with nobody
watching the run**.

## §4 — two other shapes, declined

- **A topic branch with CI on branch pushes and no pull request**: not
  simpler — two pushes per round, a workflow trigger change, and 52
  jobs on every branch push.
- **The status quo plus a permission rule for the merge**: the only
  shape that could restore review-before-acceptance, declined because
  that protection is already provided, later and more thoroughly, by
  the candidate pass and the owner's approval — and it would cost a
  blocking wait on the architect in every round.

## §5 — what this changes in the repository

1. `.github/CONTRIBUTING.md`: the pull-request section scoped to its
   real audience, and one line on how the maintainers' own work lands.
2. `docs/development/changelog-and-releases.md`: the *squash-per-round*
   paragraph and the open series-archive question it could not resolve,
   replaced by one line — with `DEC-060` the move simply **is** its own
   commit.
3. This RFC, as the single home for the rule; `ROADMAP.md` gets no row,
   because this is governance and documentation rather than product
   scope.

Still open, pending the owner's word: whether `release-0.30.0` — an
ancestor of `main`, holding nothing `main` does not — stays on `origin`
as the precedent the second exception names, or is deleted.
