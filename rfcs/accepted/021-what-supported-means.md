# RFC 021: What "supported" means — and the backend that is documented without being installable

**Status**: accepted 2026-10-29, with `0.59.0` (`ROADMAP.md`).
**Author**: the architect. **Owner's direction**: make PostgreSQL real.

## Summary

peisear has a second database backend that its operations
documentation explains how to back up and how to scale, and that its
getting-started documentation never mentions. It has an email channel
whose seven environment variables appear in no operator-facing
document. Neither is a missing feature. Both are **a configuration
the product can be run in and does not say it can be run in**, which
is the same defect wearing two coats.

This RFC does two things: it states what makes a configuration
*supported*, as a rule short enough to apply without interpretation,
and it applies that rule to the two configurations that currently
fail it.

## §1 — the measurement this comes from

Measured 2026-10-29, at `af83fde`:

| | PostgreSQL |
|---|---|
| `docs/operations/` pages documenting it | **3** — `backup.md` (a full section, every command proven against a real `postgres:17`), `scaling.md`, `background-jobs.md` |
| `docs/getting-started/` pages mentioning it | **0** — `install.md`, `configuration.md`, `first-run.md` are SQLite only |
| `peisear-web` integration binaries run against it in CI | **3 of 33** |
| declared version range | **none** |
| `§10.48` | **open** — the `postgres` feature gates the `cfg`, not the driver, and the entry says it *"becomes a defect at P5"* |

And:

| | email |
|---|---|
| `SMTP_*` variables the code reads | **7** |
| named in the startup warning | **4** |
| documented operator-facing | **0** |

The three undocumented variables are `SMTP_PORT`, `SMTP_FROM_NAME`
and **`SMTP_TLS_MODE`**. The last is not a prose gap: a transport
security mode an operator cannot read about is one they cannot set
deliberately.

## §2 — what is actually wrong, which is not "tests are missing"

The P5 work has been described in this repository as *the full
`peisear-web` suite on both backends*. That is the **means**. Stating
it as the goal is what let this sit: unrun tests are an internal
matter, easy to rank below whatever shipped most recently, and I did
rank them there as late as this morning.

**The defect is a contradiction between two documents a single
operator reads in one sitting.** `backup.md` carries working
PostgreSQL commands. `install.md` offers three install paths and
every one of them is SQLite. A reader who finds the first concludes
the backend is supported and goes looking for how to start it; there
is nothing to find, and the specification says it is not supported.

Silence would be better than this. An undocumented capability is
invisible; a half-documented one is a promise. By the owner's own
standard — *"APIs and UI/UX for users not to be confused or
misunderstand"* — a promise the product cannot keep is the more
expensive defect, and it is the reason this outranks the email gap,
which is merely silent.

## §3 — the rule

**A configuration is *supported* when all four of these are true. Any
fewer and it is not supported, and the documentation must not imply
it is.**

1. **It can be reached from the getting-started documentation.** A
   reader who has read nothing can get from `install.md` to a running
   instance in that configuration.
2. **Its own variables are documented where configuration is
   documented**, not only in a crate's rustdoc and not only in a
   startup warning.
3. **CI exercises it on the same population it exercises the default
   on.** Not a subset chosen by hand — `§10.46` is this register's
   own entry for why a hand-picked list cannot tell *green* from
   *examined nothing*.
4. **Its version range is declared**, so "supported" names something
   falsifiable rather than a feeling.

**And the negative half, which is the part that would have caught
this**: documentation for *operating* a configuration — backup,
scaling, jobs — **may not exist for a configuration that fails this
rule.** An operations page is a stronger promise than an install
page, because the reader meets it already believing they are running
the thing.

This is deliberately four conditions and not a checklist that grows.
If a fifth is ever needed, that is a change to this RFC, not an
addition someone makes in passing.

## §4 — applied to PostgreSQL: make it real

The owner's direction, recorded in `ROADMAP.md`'s own row. Each of
the four conditions becomes work:

- **(3) first**, because it is the one that can falsify the rest: the
  30 `peisear-web` integration binaries that have never run against
  PostgreSQL. If running them finds the backend does not actually
  work, the other three conditions are premature and this RFC's own
  §5 applies instead.
- **`§10.48` with it**: a `--features postgres` build still links
  SQLite's bundled C library, so the feature's name promises a lean
  PostgreSQL binary and delivers the opposite. Closing it is part of
  (3), not a separate nicety — a build that carries a driver it will
  never open is not a clean answer to *which backend am I running*.
- **(4)**: the version range comes out of what CI actually runs
  against, not from a guess. `backup.md` has been proven against
  `postgres:17`; one tested version is a range of one, and saying so
  is honest.
- **(1) and (2)**: the install path and the variables, written last,
  because a documented install path for something (3) has not yet
  proven is how this started.

## §5 — the alternative, named so it is not rediscovered

**End it**: remove the `postgres` feature, the three operations
sections, and the dialect seam. Not chosen, and the reason is worth
recording — `CON-002`–`CON-006` already built that seam, widened
every integer column, replaced fourteen SQLite-only transaction
sites, and proved `backup.md`'s commands against a real server.
Removing it would discard finished, working work to resolve a
documentation contradiction that can also be resolved by finishing.

**But it stays the answer if §4's step (3) fails.** If running the
suite on PostgreSQL finds defects whose repair is open-ended, the
honest move is this section, not a longer series. A backend that
takes four more releases to support is not supported, and the
operations pages would still be lying in the meantime.

## §6 — applied to email: the seven variables

Condition (2), and nothing else — the email channel is reachable from
the default install and needs no install path of its own. Document
all seven where configuration is documented, including the three the
startup warning does not name, and say what `SMTP_TLS_MODE`'s values
mean.

**The startup warning is also wrong by omission** and is part of this:
it names four variables as though they were the set. A warning that
lists a subset teaches the subset.

## §7 — what this RFC does not claim

- **It does not claim the rule is new.** `NFR-REL-011` already
  requires every documented install path to be exercised end to end;
  this RFC generalises that from *install paths* to *configurations*
  and adds the negative half.
- **It does not grade past releases.** `0.53.0` shipped three install
  paths that satisfy the rule for SQLite. The contradiction this RFC
  names arrived with the PostgreSQL operations documentation, which
  was itself good work (`CON-004`) landing one condition ahead of the
  other three.
- **It does not settle `§10.50`** — no CI job compiles for any
  platform but Linux outside the release tag. That is the same shape
  as condition (3) applied to platforms rather than backends, and it
  is open and unscheduled on purpose.
