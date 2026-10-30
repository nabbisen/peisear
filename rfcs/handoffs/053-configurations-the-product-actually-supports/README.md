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
| SUP-001 | [the feature that gates the `cfg`](./SUP-001-the-feature-that-gates-the-cfg.md) | `§10.48`: `sqlite = []` gates the `cfg` and not the driver, so a `--features postgres` build still links SQLite's bundled C library. Forward each driver from the feature that names it, and prove it with symbol counts both ways | — | **Closed** — `§10.48` closed on its own stated condition. Each feature now forwards its own `sqlx` driver; `cargo tree` finds `libsqlite3-sys` gone from the graph under `--features postgres` and present by default. **The round's own finding outweighs its fix**: the entry's 387-symbol figure had already decayed to 0 before any change, through newer linker dead-code elimination, so the evidence had stopped being able to show a defect that was still there — re-measured because the entry told it to |
| SUP-002 | [the suite on both backends](./SUP-002-the-suite-on-both-backends.md) | The 30 integration binaries that have never run against PostgreSQL, unfiltered rather than by a hand-picked list (`§10.46`), and the version range that comes out of what CI actually exercised. **This round decides whether the rest of the release happens** | SUP-001 | **Closed** — and the release is **not** falsified. 33 of 33 `peisear-web` integration binaries run against a real `postgres:17`, unfiltered, every one green, with nine real defects found and each seen red from a captured error first. `RFC 021` condition (3) is met, so `RFC 021` §5 — end it — is off the table. Postgres job cost: **+6m09s**, stated plainly. One review finding, carried to `SUP-003`: CI's single service container now runs all 33 steps with no restart at PostgreSQL's default 64 MB of shared memory, which is the accumulation that exhausted it locally — green today, headroom unmeasured |
| SUP-003 | [the install path, and what may say so](./SUP-003-the-install-path-and-what-may-say-so.md) | Conditions (1) and (2) for PostgreSQL, written **last** on purpose: the install path, `DATABASE_URL`'s PostgreSQL form, the declared range, and the three operations pages reconciled against what now actually exists | SUP-002 | **Closed** — one install path supports PostgreSQL (`cargo install`), measured rather than assumed, and `--no-default-features` is **required**: Cargo features being additive, `--features postgres` alone silently rebuilds the dual-driver binary `SUP-001` eliminated. Walked in CI from outside the checkout, register → authenticated `/projects`. The three operations pages reconciled, none added. One finding carried to `REL-0.59.0`: `NFR-CMP-003` reserved a PostgreSQL clause **for the owner** on exactly this trigger, and this round rewrote the `scaling.md` sentence that entry cites as its basis |
| SUP-004 | [the seven variables](./SUP-004-the-seven-variables.md) | All seven `SMTP_*` documented where configuration is documented, `SMTP_TLS_MODE`'s values explained, and a startup warning that stops teaching a subset | — | **Closed** — all seven `SMTP_*` documented where configuration is documented, `SMTP_TLS_MODE`'s values and port-derived fallback explained, and the startup warning no longer teaches a subset of four: it names none and points at the page. `docs_scan` rule 4's reach over these seven measured by **planting a wrong default and watching the suite stay green**, not by reading the rule. The round's own plainly-documented fallback is what let the architect find `§10.51` |
| REL-0.59.0 | *not yet written* | The cut: `DEC-028`, `DEC-056`, and `RFC 021`'s own four conditions answered one at a time for both configurations. The architect writes it when the work rounds close | SUP-001–SUP-004 | **Not yet written** — and it carries one question for the owner, raised by `SUP-003`'s review: `NFR-CMP-003`'s 0.56.0 amendment reserved a PostgreSQL persistence clause *"for the owner to decide, not assumed"* on **the day a PostgreSQL install path is documented**. That day is 2026-10-30. Its own *"silence on PostgreSQL until P5 is deliberate"* is now false about a live state, and the `scaling.md` sentence it cites as its basis was rewritten by `SUP-003`. **And `§10.51`**, newly opened by `SUP-004`'s review: `SMTP_PORT` and `SMTP_TLS_MODE` each discard a malformed value silently, where `PEISEAR_SETUP` refuses one by name — two rules for one input class, the permissive half governing transport security. Unscheduled; the cut should name it rather than leave it to be found |

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
