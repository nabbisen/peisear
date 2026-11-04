# Handoffs — a gate that is not watching

**No release number.** Accepted by the owner 2026-11-03, on the proposal
recorded in `ROADMAP.md`. `main` is what a reader gets; these land under
`[Unreleased]` and whichever release is cut next picks them up in its own
`DEC-028` sweep.

**No RFC, and the distinction is worth stating once** because the owner's
acceptance used the word. The RFC numbering is at **`022`**; `057` is a
**handoff series**, numbered after `056`. An RFC states a rule; a series
applies one. **There is no new rule here** — `§10.50` and `§10.36` already
name the class, and `NFR-REL-009` already promises the thing a user cares
about (*"Every published GitHub Release MUST carry the five per-platform
archives… **the check is the artefact being there**"*). **This series
amends no requirement**, deliberately: `NFR-REL-009` governs whether the
artefact exists, and what is wrong here is *when we find out it cannot be
built* — a cost and a risk, not a promise to a user. Folding that into an
entry whose own text draws the opposite distinction would blur the reason
it was written.

**One rule, and it is a question rather than a prohibition**: *can this
gate tell "nothing is wrong" from "nothing was examined"?* Both rounds are
gates that currently cannot. One never compiles the platform it is read as
covering; the other asserts correctly and has run three times in its life.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| PLT-001 | [the platform nothing compiles](./PLT-001-the-platform-nothing-compiles.md) | **All 47 of `test.yml`'s jobs run on `ubuntu-latest`; not one runs on Windows or macOS.** Every other platform is first compiled when the release archive is produced — the latest point a finding can still be acted on without re-cutting. `0.58.0` lost a published platform to this class, and in one series `release.yml` found two defects `test.yml` structurally could not. One `windows-latest` job running `cargo clippy -p peisear -- -D warnings`, the remedy `§10.50` names itself | — first in the series | **Closed 2026-11-04, and the evidence is the strongest this register entry has.** `clippy-windows` (`cargo clippy -p peisear -- -D warnings`, `windows-latest`, not advisory, mirroring its Linux sibling's own shape). **Proven on a real runner, not read from the YAML**: the round re-created `TTY-002`'s own historical defect rather than inventing one — `#[cfg(unix)]` removed from `resolve_is_foreground`, live on Unix and dead on Windows — and pushed it. **Architect's verification, read from the runs themselves**: in run `41659208632` **exactly one job of the whole run failed**, `cargo clippy (windows)`, while `cargo clippy` and all 46 other Linux jobs passed. The handoff asked for the asymmetry because two red jobs prove nothing; what came back is better than asked — **the defect that cost `0.58.0` a published platform is invisible to every job this file had before and visible to the one it now has.** Three pushes on the branch, all three verified independently: `56cacfd8` clean, `820738f7` the plant (CI `failure`), `baebf92c` reverted (CI `success`). Every figure re-measured and exact: **48** jobs defined (47 `ubuntu-latest`, 1 `windows-latest`); **49** checks success plus the usual `Deploy to GitHub Pages` `skipping`, which reconciles as 48 CI jobs + 2 Docs jobs; the job's own duration **3m06s** cold and **1m23s** warm, read from `gh run view` as they were; `DEC-007` **637** unchanged; `fmt` and the Linux clippy clean; `CI`/`Docs` `success` at `9454edb`; diff one file. **`§5` did not fire and the round said so**: the job needed no C-toolchain step, since `sqlx`'s bundled SQLite compiles natively on `windows-latest`. **`§10.50` is narrowed, not closed** — of the five platforms `release.yml` publishes, **two are now compiled before a tag and three are not** (`aarch64-unknown-linux-gnu`, both Darwin targets), and the Windows coverage is a compile-and-lint of the binary crate rather than *Windows is tested*. Both bounds are stated in the entry so the narrowing is not read as wider than it is. **One imprecision in the report, not in the work**: it says `-D warnings` is *"inherited from the job's own `RUSTFLAGS`"*; the run line passes `-- -D warnings` explicitly, which is the stronger arrangement and the one the handoff asked for |
| WLK-001 | [the gate nobody triggers](./WLK-001-the-gate-nobody-triggers.md) | `env-example-walk.yml` asserts the documented copy-and-run path and is **`workflow_dispatch:` only** — three runs in its life, the last 2026-10-16, before four releases. `QS-003` just made `cp .env.example .env` documented on a second page. A `push` trigger scoped to the paths that can break it | PLT-001, by order rather than dependency | **Round 1 closed 2026-11-04; one finding, and it earns round 2.** The walk fires on `push` scoped to six paths, and **`main`'s own `Env Example Walk` ran for the first time in this gate's life** at `9a59269` — `success`, event `push`, verified directly. **Every one of the six runs the round cited was checked by the architect for event and conclusion and all six are exact**: the pre-work dispatch on `main` (`41735711119`, `success` — `§5`'s escalation condition checked *first*, before touching the YAML, and correctly), a push firing it, a dispatch still working, and the plant sequence `success` → **`failure`** → `success` with every event reading `push`, not a dispatch. **Two things better than asked.** The path list gained `crates/peisear/src/main.rs` beyond the handoff's own suggestion — `build_env_filter` lives there, not in `config.rs` — and the round **proved that addition by breaking exactly that function**. And its first plant *passed*: a bare string with no `=` is accepted by `EnvFilter` as a lenient target name, the same leniency `CFG-001` found in `RUST_LOG=dbug` — reported as a methodology finding rather than quietly replaced, then corrected to an unrecognised *level*, which the project's own existing test already names as the real failure shape. It also **did not wait out a flake**: `main`'s `CI` first read `failure` on an unrelated Chrome CDP launch, re-run to `success` and said so. `.env.example` was never read, copied or printed locally. `DEC-007` **637** unchanged; `fmt`/`clippy` clean; the walk costs **~7 minutes** (7m00s, 7m22s). **The finding**: the trigger is a bare `push:` with no `pull_request:`, where `test.yml` — this repository's own convention — uses `push: branches: [main]` plus `pull_request:`. A `push` for a fork's branch happens in the fork, so **in this repository the walk never fires for the contribution flow `.github/CONTRIBUTING.md:235` documents** — *fork the repository… push to your fork and open a pull request* — and that file is itself in the walk's path filter. **The one guard whose subject is the contributor path is blind to the outside contributor**, and in-repo work fires it twice for one change, branch push and squash, at seven minutes each. Round 1 named its departure from `test.yml` and gave the upside; it did not name this cost. `TTY-002`'s shape: the subject achieved, a property of the project's own standard not met |
| WLK-001 r2 | [the contributor it cannot see](./WLK-001-round-2-the-contributor-it-cannot-see.md) | `pull_request:` with the same six paths, `push` narrowed to `main`, `workflow_dispatch` untouched — the arrangement `test.yml` already uses. Three things measurable and required (a `pull_request`-triggered run, **no** run from a filtered push to a non-`main` branch, the dispatch path still working); the fork case **stated as documented behaviour and labelled as such**, since no fork is available to push from | WLK-001 round 1 | **Open, the dev team's** |

**One round in flight at a time**, in that order. `PLT-001` is first
because it is the one with a recorded cost: a published platform already
lost.

## What this series is not

**Not a platform matrix.** `PLT-001` adds **Windows and not macOS**, on
`§10.50`'s own reasoning: macOS costs runner minutes Linux does not, and
the recorded incident was Windows. A second platform is a later decision
with its own evidence, not a thing to slip in because the job is being
edited anyway.

**Not a widening of `NFR-REL-011`.** Its `0.53.0` round 2 decision
excluded the contributor path deliberately. `WLK-001` makes an existing
job run; it does not put that job under a requirement's authority.

**Not a repair of anything already repaired.** This is the distinction the
owner's review drew on 2026-11-03 when it removed three items from a
four-item proposal: a check written for a defect already fixed by hand is
not worth a round. **Both of these are live gaps on shipped artefacts** —
a platform axis nothing compiles, and an assertion that is not running.

## The trap this series will walk into, named once here

**Both rounds can only be proven on a real runner, and both are therefore
tempting to report from the YAML.** `PLT-001` has no Windows here;
`WLK-001`'s trigger cannot be observed locally at all. **A workflow read
is not a run.** Each round's `§2` requires a push, a planted failure, and
a quoted run — and for `WLK-001`, confirmation from the run page that the
event was **`push`** and not the author's own dispatch.

**The second half of the same trap**: a path filter that names a path
wrongly is **silent** — the job simply never fires again. That is
`§10.36` instance 6's shape (a lookup that misses and passes), moved into
YAML, and it is why `WLK-001`'s plant must live in a file the filter
names.

## The standing exception applies

A defect found in the shipped product while this series is open earns a
round of its own rather than being folded into the round that noticed it.
`054` closed three rounds against one scoped that way; `056` closed three
against three and each of them corrected a measurement of the
architect's.
