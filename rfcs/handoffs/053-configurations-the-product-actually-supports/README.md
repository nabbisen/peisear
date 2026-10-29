# Handoffs — configurations the product actually supports

**0.59.0.** `RFC 021`, accepted 2026-10-29, with `RFC 018`'s own
**P5**. Authorized by the owner the same day, **direction included:
make PostgreSQL real rather than end it.**

One subject: **the gap between what peisear can be run as and what it
tells an operator it can be run as.** Two surfaces fail that test
today, and they fail it differently.

**PostgreSQL is documented in the wrong half.** `docs/operations/`
explains how to back up and how to scale a PostgreSQL deployment on
three pages — `backup.md`'s every command proven against a real
server — while `docs/getting-started/` never mentions it and all
three install paths are SQLite. 3 of 33 `peisear-web` integration
binaries run against it in CI. No version range is declared.
`§10.48` is open and says the `postgres` feature *"becomes a defect at
P5"*. An operator who meets the backup page concludes it is
supported; the specification says it is not.

**Email is simply silent.** Seven `SMTP_*` variables, four named in a
startup warning, zero documented — and one of the three invisible
ones is `SMTP_TLS_MODE`.

Silence confuses less than contradiction, which is why PostgreSQL
leads and why the architect's earlier ranking of this list — which put
an internal CI gap first — was corrected in `ROADMAP.md` before this
series was scoped.

**`RFC 021` §3 is the rule the series is built on**, four conditions
with a negative half. Read it before any round.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| SUP-001 | [the feature that gates the `cfg`](./SUP-001-the-feature-that-gates-the-cfg.md) | `§10.48`: `sqlite = []` gates the `cfg` and not the driver, so a `--features postgres` build still links SQLite's bundled C library. Forward each driver from the feature that names it, and prove it with symbol counts both ways | — | **Next** |
| SUP-002 | [the suite on both backends](./SUP-002-the-suite-on-both-backends.md) | The 30 integration binaries that have never run against PostgreSQL, unfiltered rather than by a hand-picked list (`§10.46`), and the version range that comes out of what CI actually exercised. **This round decides whether the rest of the release happens** | SUP-001 | **Blocked** |
| SUP-003 | [the install path, and what may say so](./SUP-003-the-install-path-and-what-may-say-so.md) | Conditions (1) and (2) for PostgreSQL, written **last** on purpose: the install path, `DATABASE_URL`'s PostgreSQL form, the declared range, and the three operations pages reconciled against what now actually exists | SUP-002 | **Blocked** |
| SUP-004 | [the seven variables](./SUP-004-the-seven-variables.md) | All seven `SMTP_*` documented where configuration is documented, `SMTP_TLS_MODE`'s values explained, and a startup warning that stops teaching a subset | — | **Ready, parallel** |
| REL-0.59.0 | *not yet written* | The cut: `DEC-028`, `DEC-056`, and `RFC 021`'s own four conditions answered one at a time for both configurations. The architect writes it when the work rounds close | SUP-001–SUP-004 | **Not yet written** |

---

## The one thing that can end this series early

**`SUP-002` can falsify the release.** `RFC 021` §4 puts the suite
first among the four conditions for exactly that reason: if running 30
unexercised binaries against PostgreSQL finds defects whose repair is
open-ended, **the honest answer is `RFC 021` §5 — end it** — and that
is the owner's call, not a round's.

`SUP-003` does not start until `SUP-002` reports green. Writing an
install path for something the suite has not proven is how this defect
began; `CON-004` wrote the operations pages one condition ahead of the
other three, which was good work landing in the wrong order.

## What this series is not

**Not a performance or scaling exercise.** Nothing here claims
PostgreSQL is faster, or recommends it. The claim is only that it
either works and is documented, or does not exist.

**Not a second register entry for `§10.50`.** No CI job compiles for
any platform but Linux outside the release tag — the same shape as
`RFC 021` condition (3) applied to platforms instead of backends. It
is open, numbered, and deliberately unscheduled.

## The standing exception applies

A defect found in the shipped product while this series is open earns
a round of its own rather than being folded into whichever round
noticed it. 0.58.0 closed five rounds against three scoped that way,
and 0.58.1 two against two; both additions in each were real.
