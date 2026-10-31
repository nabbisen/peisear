# Handoffs — absence is not error

**0.60.0.** `RFC 022`, accepted 2026-10-31, authorized by the owner the
same day. **Read `RFC 022` §3 before any round** — it is one sentence
and every round is an application of it.

**One rule**: absence degrades; a value the operator supplied and the
product cannot use **refuses startup, by name, naming what would have
been accepted.**

Thirteen operator-facing variables. **Three already refuse**
(`PEISEAR_SETUP`, `JWT_SECRET`, `BIND_ADDR`). **Four discard a supplied
value in silence** — `COOKIE_SECURE`, `SMTP_PORT`, `SMTP_TLS_MODE`,
`RUST_LOG` — and the first two of those decide cookie security and
transport security. `§10.51` is this series' entry and closes with it.

**The defect is not four bad parses.** It is one rule that was never
written down and therefore never applied, so a new variable lands
wherever its author's habit puts it. `RFC 022` writes it down;
`CFG-003` makes it enforceable.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| CFG-001 | [the two that already have an error type](./CFG-001-the-two-that-already-have-an-error-type.md) | `COOKIE_SECURE` and `RUST_LOG`, both reachable from an existing `ConfigError`/`Result` path. The security-shaped one first | — | **Open** |
| CFG-002 | [the crate with no error channel](./CFG-002-the-crate-with-no-error-channel.md) | `SMTP_PORT` and `SMTP_TLS_MODE` — and `SmtpConfig::from_env() -> Option<Self>` becoming fallible, which is this release's real engineering content | CFG-001 | **Open** |
| CFG-003 | [the rule, enforced](./CFG-003-the-rule-enforced.md) | A guard so a fifth variable cannot be added in the silent shape. Until it lands, `RFC 022` is prose | CFG-002 | **Open** |
| REL-0.60.0 | *not yet written* | The cut. **It carries `054`'s work too** — the front door landed under `[Unreleased]` with no release of its own, and this is the release that picks it up | CFG-001–CFG-003 | **Not yet written** — the architect writes it when the work rounds close |

**`CFG-001` before `CFG-002`** because the first works inside an error
type that already exists and the second has to build one; getting the
error *wording* settled on the easy path first means `CFG-002` copies a
shape rather than inventing one.

## What this series is not

**Not a change to graceful degradation.** An unconfigured email channel
still never blocks a boot. `RFC 022` §3's two halves exist precisely so
this is not in tension: **absence is not error.** A round that makes an
*unset* variable refuse has misread the rule.

**Not a widening of what is accepted.** `COOKIE_SECURE` must keep
accepting `true`, which works today — narrowing it would break working
deployments in the refusing direction, a worse failure than the one
being fixed (`RFC 022` §5).

**Not a rewrite of the three that are already right.** `PEISEAR_SETUP`,
`JWT_SECRET` and `BIND_ADDR` are the model. Read their error messages
and match them; do not improve them in passing.

## The trap this series will walk into, named once here

**`docs_scan` reads the code `CFG-002` changes.** `smtp_port_default`
(`docs_scan.rs:347`) finds the literal `env::var("SMTP_PORT")` and then
`.unwrap_or(` within the next 200 characters. `CFG-002` moves the
default out of `.unwrap_or(`, **so this lookup will stop matching** —
by design.

**The right outcome is that the guard goes red and is then updated.**
The wrong outcome — and the one that has happened here before — is that
it silently returns `None` and the rule examines nothing.
**`§10.36` instance 6 is this exact defect in this exact function**, and
that entry records the remedy: rule 4 now asserts its own lookup
succeeded rather than trusting it. **Verify that assertion fires. Do not
assume it does because the entry says it was added.**

## The standing exception applies

A defect found in the shipped product while this series is open earns a
round of its own rather than being folded into whichever round noticed
it. 0.58.0 closed five rounds against three scoped that way, 0.58.1 two
against two, and 0.59.0 four against four; every addition was real.
