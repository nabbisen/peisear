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
| PLT-001 | [the platform nothing compiles](./PLT-001-the-platform-nothing-compiles.md) | **All 47 of `test.yml`'s jobs run on `ubuntu-latest`; not one runs on Windows or macOS.** Every other platform is first compiled when the release archive is produced — the latest point a finding can still be acted on without re-cutting. `0.58.0` lost a published platform to this class, and in one series `release.yml` found two defects `test.yml` structurally could not. One `windows-latest` job running `cargo clippy -p peisear -- -D warnings`, the remedy `§10.50` names itself | — first in the series | **Open, the dev team's** |
| WLK-001 | [the gate nobody triggers](./WLK-001-the-gate-nobody-triggers.md) | `env-example-walk.yml` asserts the documented copy-and-run path and is **`workflow_dispatch:` only** — three runs in its life, the last 2026-10-16, before four releases. `QS-003` just made `cp .env.example .env` documented on a second page. A `push` trigger scoped to the paths that can break it | PLT-001, by order rather than dependency | **Open, the dev team's** |

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
