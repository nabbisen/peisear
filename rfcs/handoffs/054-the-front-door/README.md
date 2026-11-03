# Handoffs — the front door

**No release number.** `README.md` on `main` is what GitHub serves the
moment it merges, so this work does not wait for a cut and does not
get one of its own. It lands under `## [Unreleased]` in
`CHANGELOG.md` the normal way, and whichever release is cut next picks
it up in its own `DEC-028` sweep and Appendix C row.

**Raised by the owner, 2026-10-31:** *"Is there any plan to enhance
'Quick Start'? The current guide in `README.md` is far from 'quick'.
Also, not kind and not easy to users at all. Most users do not have
`cargo` client. Some of them do not have even `git`."*

There was no plan. The gap is real and it is narrower than a style
complaint: **we already fixed this in `install.md` and never carried
it to `README.md`.**

## What is actually wrong

Measured 2026-10-31 at `293df27`.

`docs/getting-started/install.md` offers three paths, ordered by what
a reader needs to have:

| | needs | commands |
|---|---|---|
| **Option A** — download a prebuilt binary | nothing | extract, run |
| **Option B** — `docker run` | Docker | **one** |
| **Option C** — `cargo install`, *"if you already have Rust"* | the Rust toolchain | two, plus a build |

`README.md`'s Quick Start offers **two paths, and both require
Rust** — the second also requires `git`:

```bash
cargo install peisear          # needs cargo
git clone … && cd peisear      # needs git and cargo
cargo run --release -p peisear
```

**So the product has shipped a no-toolchain path since `0.53.0` —
seven releases — and the front door never learned about it.** The
quickest start peisear has is one command
(`docker run -p 3000:3000 -v ./data:/data ghcr.io/nabbisen/peisear`)
and the section called *Quick Start* does not mention it.

**The cost of the path it does lead with**, measured rather than
asserted: CI's own `cargo install walk (NFR-REL-011)` job ran
`23:14:37`→`23:19:31` — **4m54s** — on a runner that already had the
toolchain. A reader without Rust installs Rust first. *Stated
bounded*: that figure is the whole job (checkout, toolchain, install,
walk), so the install alone is less; the point is the order of
magnitude against Option B's single pull-and-run.

## Why it survived seven releases

- **`FIV-004` (0.53.0) wrote `install.md`'s three options and did not
  carry them to the README.** Good work landing in one of two places,
  the same shape as `CON-004` writing operations pages one condition
  ahead of the others.
- **`NFR-REL-007` governs the README's *ordering*** — hero, overview,
  why/when, quick start, features or design notes, links — **and says
  nothing about what the quick start must contain.** So the five-pass
  sweep had nothing to catch, every release, correctly.
- **`§10.41` already quoted this exact block** as *"the instruction a
  first-time reader is most likely to follow"* and was closed on a
  different defect inside the same four lines. Nobody asked whether
  those were the right four lines.

**The lapse is the architect's** — `FIV-004`'s review and every sweep
since.

**This is `RFC 021`'s own class, one level closer to the reader.**
There, operations documentation promised a configuration
getting-started could not reach. Here, the front door offers the
hardest path while the product ships two easy ones.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| QS-001 | [the first command](./QS-001-the-first-command.md) | The Quick Start leads with what needs nothing, and `NFR-REL-007` gains the clause that would have caught this | — | **Closed 2026-11-01 — written by the architect, not the dev team, and the reason is a failure of mine.** This series was authorized 2026-10-31 and never relayed; `055` ran five rounds while it sat, and **the owner asked the same question a second time** before anything moved. Scheduling authorized work is my job, and letting it stall behind an internal-tooling series is the ranking error the owner corrected before `0.59.0`, this time by inattention rather than judgement. Since the round is documentation plus one specification clause — both things the architect writes — waiting for a third relay would have cost more than doing it. The Quick Start now leads with the path that needs nothing installed, then `docker run` as one command, then `cargo install` marked *if you already have Rust* in `install.md`'s own wording, then the clone, which says it is for reading or changing the source. **The verified trap was not walked into**: the container block does not claim a prompt, because `can_prompt` needs a terminal and the documented `docker run` has no `-it`. `NFR-REL-007` gained the clause whose absence let this sit, with `*Status*` unmoved and the missing guard named |
| QS-002 | [the rule that keeps them together](./QS-002-the-rule-that-keeps-them-together.md) | A `docs_scan` rule: every command in the README's Quick Start names a path `install.md` offers. Seen red on a plant before it is trusted | QS-001 | **Closed 2026-11-03, both rounds.** `docs_scan` rule 9 is live: every command in `README.md`'s `## Quick Start` appears verbatim in a ` ```bash ` fence in `install.md`, with an asserted population census and a lookup that reports its own failure by name. **The round came back blocked once and blocking was right** — `§5`'s second bullet fired as written. The architect's decision is `§7` of the handoff, and it was **planted on the real tree before it was issued**: the README's heading structure was what disagreed with the README, whose own closing sentence already said `install.md` *"covers all three paths in full"* while the heading span held four. So the source block took its own `##` heading (`##`, not `###` — the terminator is `"\n## "`), `install.md` adopted the README's copy-pasteable glob because `peisear-<version>-…` pasted literally fails, and the census became `5`. **Option (a)'s rule half was declined and its document half became `QS-003`.** Round 2 applied it with **no change to the rule's matching logic** — census and doc comments only. **Architect's verification, by mutation rather than by the report**: finding 2 was that a plant can stop planting while still passing, so rule 9 was broken three ways — matching disabled, the anchor arm silenced, the census comparison disabled — and **each mutation failed exactly one test and no other**. Three plants, three failures, one-to-one: none passes by coincidence. A command `install.md` does not offer, planted inside the live Quick Start, is caught by name with the census firing beside it. `DEC-007` **632** (297 per-crate + 335 integration, the latter re-verified as touching no file this round changed); `fmt` and `clippy --workspace --all-targets -- -D warnings` clean, `mdbook` with linkcheck exit 0, rustdoc 6 unchanged, and `CI`/`Docs` both `success` read directly at `a1f8e40`. **The round's three findings were all incorporated, and the third bound is credited in the code to the review rather than the round.** *Twice in this review the architect questioned one of the round's figures and the figure was right* — rustdoc 6 (mine swept up a summary line and a dependency notice) and `DEC-007` 632 (mine re-sliced the count without reading that `test.yml`'s own block lines define it: `-p peisear-storage --lib` excludes that crate's doctest, bare `-p peisear` includes its own). **The residual, named so this closing is not read as wider than it is**: rule 9 does not guard the `## Build from source` block's two commands against anything, and `QS-003` closes only the document half The round came back **blocked, and blocking was right**: `§5`'s second bullet fired exactly as written, the rule being red on the README `QS-001` produced. **The rule is correct and was verified by the architect's own plants on the real tree**, not taken from the report — the anchor renamed reports the lookup failing *by name* and the three ordinary violations vanish with it (`§10.36` instance 6's class, handled from the start); a deleted command line moves the census; and **the decision itself, planted before it was issued, takes `the_repository_satisfies_the_live_rules` green with no change to the rule's logic.** Every measurable claim in the report held on re-running — `fmt`, `clippy`, rustdoc **6** unchanged, `DEC-007` **222** with the runner, and the aggregate gate failing *only* for its `§1` reason. Two of the report's claims were checked against the files rather than its summary and both held; **the architect's own first grep said `install.md` does not link `build-from-source.md`, and that was the wrong needle** — the link is bare-relative and it is there, at `:57`. **The decision**: the README's heading structure is what disagrees with the README, since its own closing sentence already says `install.md` *"covers all three paths in full"* while the heading span holds four — so the source block takes its own `##` heading (**`##`, not `###`**: the terminator is `"\n## "`), `install.md` adopts the README's copy-pasteable glob, and the census becomes `5`. Option (a)'s rule half is declined — following every link out of `install.md` is a far larger haystack, following one named link is option (c) renamed, and the round was right to rule (c) out. **Three findings, none blocking**: a third bound is unstated and is the one that matters (the rule tests against `install.md` *as a whole*, not the corresponding path's block); plant 1 anchors on a neighbouring heading, so it stops planting the moment the boundary moves; and the fourth test — unlisted in the request's own `§0`, which named three — pins the live red and **should not survive the fix**, because the aggregate gate already carries that signal. **Three of the four new tests are written against the live documents, so the fix invalidates all three**: *done but for a decision* is not nearly done |
| QS-003 | [the page that never clones](./QS-003-the-page-that-never-clones.md) | `install.md:57` tells the reader that `build-from-source.md` *"clones the repository and builds from a checkout"*. **That page never clones anything** — it installs `rustc` two ways, verifies the version, and its `## Next` skips the step its own title promises. One published document asserting a second teaches a step the second does not contain, and **the same shape one layer down**: `.github/CONTRIBUTING.md:23` says *"Clone the workspace"* over a fence that never clones | — independent of `QS-002`'s files; take it after, so one round is in flight at a time | **Closed 2026-11-03.** Found by `QS-002` while measuring its rule against the real tree, and given a round of its own rather than folded into the round that noticed it, **on the standing exception this project has applied through three releases**. `NFR-REL-007`'s class: a reader following written instructions does not reach a working instance. The decision about *where the command lives* is made in the handoff's `§1` so the round does not re-open it — `build-from-source.md` owns the step, because `install.md:57` already says it does, and `CONTRIBUTING.md` drops its clone claim rather than becoming a third copy. **`env-example-walk.yml` already walks this path**, so the commands could agree with a job that has actually run it — with two bounds stated rather than implied: CI does not prove the clone line, and **`NFR-REL-011` must not be widened to cover this reader**, a decision that entry already made in `0.53.0` round 2. **Delivered**: `## Get the code and run it` added after `## Verify`, with the clone, `cp .env.example .env` and `cargo run --release -p peisear`; `## Next` reaches it instead of jumping past it; `CONTRIBUTING.md` step 2 drops the clone claim its own fence never showed, one fewer copy rather than a third. **The round reused `CONTRIBUTING.md`'s existing command pair verbatim rather than minting a fourth variant** — the handoff's `§3` trap, avoided by choosing deliberately and saying so. **Falsified with a real clone of the published repository**, reaching a listening server, not read. **And it respected a denial instead of working around it**: `.env.example` is covered by the owner's own permission rule, so the round proved the `cargo run` claim with equivalent environment variables and stated plainly that `cp .env.example .env` itself is not independently re-proven — the right call, and the bound named rather than blurred. Architect's verification: `DEC-007` **632 unchanged** recounted with the CI block's own slices, `fmt` and `clippy --workspace --all-targets -- -D warnings` clean, `mdbook` with linkcheck exit 0, `CI`/`Docs` both `success` read directly at `55a1dc8`, and the diff exactly two documents. **One correction against the architect, found while verifying**: this row and the handoff's `§2` both said the documented commands would agree with *a job CI runs* — `on every push`, in the handoff's words. **`env-example-walk.yml` is `workflow_dispatch:` only and has run three times, last on 2026-10-16**, four releases ago. `§10.40` had it right all along as *"a one-off `workflow_dispatch`-only job"*; the overstatement was mine. **The round did not propagate it** — its report wrote *"on every manual dispatch"*, quietly correct where the handoff was wrong, though naming it would have been better still, on `CFG-001`'s own precedent for catching an architect's false premise rather than writing around it. **Two residuals, neither this round's to fix**: the page's pre-existing distro-package build line still reads, in page order, as assuming a checkout not yet shown — flagged by the round, and moving it is a second placement decision; and **`cp .env.example .env` is now documented on two pages with nothing exercising it automatically**, carried to the candidate list |

Three rounds and no cut. `QS-002` did not start until `QS-001`'s shape
was settled, because the rule reads the thing `QS-001` writes. **`QS-003`
was not scoped when this series was authorized** — it is a defect `QS-002`
found in a published document while measuring, and it earns its own round
rather than widening the round that noticed it.

## Closed 2026-11-03 — all three rounds, and what the series actually was

**Scoped as one fix and closed as three, each found by the one before it.**
`QS-001` reordered the Quick Start. `QS-002`, building the guard for it,
found the README's own heading structure disagreed with the README's own
sentence. `QS-003`, named by `QS-002`'s measurement, found a page whose
title promised a step it never showed and a second page asserting it did.
**None of the last two was visible from the first**, which is the argument
for a guard over a fix: the rule did not just prevent the next drift, it
*reported* two that were already there.

**`QS-001` was absorbed by `0.60.0`** (it merged the day before that cut,
and the cut's own `DEC-028` sweep folded it in — the section says so).
`QS-002` and `QS-003` merged after the cut, so they sit under
`[Unreleased]` for whichever release is cut next, exactly as this series'
no-release-number disposition said they would.

**What the series does not leave guarded, stated so the closing is not
read as wider than it is:**

- Rule 9 covers the Quick Start against `install.md`. It does **not**
  cover the README's `## Build from source` block — moved outside that
  section by `QS-002` `§7` — against `build-from-source.md`. Nothing
  checks those two documents agree.
- The rule matches a command against `install.md` **as a whole**, not
  against the corresponding option's own block. Stated in its own doc
  comment, found by review rather than by the round.
- **`cp .env.example .env` is now documented on two pages and nothing
  exercises it automatically.** `env-example-walk.yml` asserts it, but it
  is `workflow_dispatch:` only and has run three times, last 2026-10-16.
  Carried to `ROADMAP.md`'s candidate list with no release number.
- **`NFR-REL-011` still excludes this reader by decision**, not by
  oversight (`0.53.0` round 2). The contributor path's guard stands on its
  own merit or not at all.

**Two of the architect's own claims were corrected during this series, both
found by measuring rather than by reading**: that `054`'s work had already
landed under `[Unreleased]` (`CFG-001`, when it had not), and that
`env-example-walk.yml` runs on every push (`QS-003`'s verification, when it
runs only on demand). The second is recorded in `QS-003`'s own `§2`.

## What this series is not

**Not a rewrite of the README.** `NFR-REL-007`'s ordering is met and
stays met; the hero, the overview, the why/when, the features and the
links are not this series' business. One section changes.

**Not a new install path.** All three already exist, are documented,
and are exercised — `NFR-REL-011` walks two of them in CI. Nothing
here builds an artefact; it changes which one a reader meets first.

**Not a claim that `cargo install` is wrong.** It is the right path
for someone who has Rust, it stays, and `install.md`'s own conditional
heading is the wording to mirror.
