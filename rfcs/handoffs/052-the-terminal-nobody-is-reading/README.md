# Handoffs — the terminal nobody is reading

**0.58.1.** `RFC 020` §7, amended 2026-10-28; authorized by the owner
the same day. One subject: **a first run that is started the way
servers are actually started must serve, not stop.**

`0.58.0` shipped the wizard and shipped a defect with it. The predicate
that decides whether to prompt asks *is a terminal attached*; the
question is *can anyone answer*. A background job is a terminal nobody
is reading, and a first run there stops dead — `SIGTTIN`, nothing
listening, no `.env` written, the only trace a question in a redirected
log. The same cause cost `0.58.0` its Windows archive, because
`Start-Process` creates a console nobody will ever type into, so the
release walkthrough timed out and the packaged `.zip` was never
attached.

**The evidence is `§10.49`** in `docs/specification/requirements.md` —
measured twice on Linux at tag `0.58.0`, plus the failing Windows job.
The handoffs do not repeat it; read it first.

Two work rounds, ordered by dependency rather than by risk: the
predicate cannot be proven on a Windows runner before it exists.
**Nothing in the `0.58.0` design is reversed** — a foreground first run
still asks its two questions and makes its three statements. One of
`RFC 020` §1's properties is being implemented as the property instead
of as one of its examples.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| TTY-001 | [the predicate](./TTY-001-the-predicate.md) | *Can we actually prompt* replaces *is a terminal attached*. On Unix the kernel answers it exactly — the foreground process group, the same test it uses to raise `SIGTTIN`. On Windows, where no such answer exists, `is_terminal` stays and `PEISEAR_SETUP=never` lets automation declare it. Unable to prompt means **start**, on the path a container already takes | — | **Closed** — 2 rounds. The predicate is *can we actually prompt*: `PEISEAR_SETUP` first, then a terminal, then — on Unix — the kernel's own foreground-process-group test. Round 2 closed the error branch, where a failed `tcgetpgrp` resolved to *yes*: `tcgetpgrp` also fails on a terminal with no controlling relationship (`setsid`), and there the wizard blocked forever with no `SIGTTIN` to even show `Stopped`. Both the attended and unattended cases proven against real binaries, in both directions |
| TTY-002 | [the archive, and the two walks](./TTY-002-the-archive-and-the-two-walks.md) | Restores the Windows archive `0.58.0` dropped, and divides the two release walk steps by the property each proves. They have been testing different things under one name: the Unix step's `stdin` has never been a tty, so it has never met the wizard, while the Windows step always did and nobody meant it to | TTY-001 | **Round 2 owed** — the archive is back, proven by a real five-platform `workflow_dispatch`; the walks are divided by property; the unhosted Windows case is named with five diagnosed attempts rather than faked. **One case is missing and it is this series' own**: nothing in the repository exercises *a terminal attached, process not its foreground job*, and the Windows walk that once caught that class by accident now declares `PEISEAR_SETUP=never`. `test.yml` case 7, seen red against the planted predicate. `§10.49` is **fixed, not yet guarded** until it exists |
| REL-0.58.1 | *not yet written* | The cut: `DEC-028`, `DEC-056`, `§10.49`'s own closure. The architect writes it when both rounds close | TTY-001, TTY-002 | **Not yet written** |

---

## What this release is not

**Not a rewrite of the wizard.** Two questions, three statements, the
`.env` it describes — all unchanged. If a round finds itself editing
`run_wizard`'s questions, something has gone wrong; escalate.

**Not a CI fix.** `-NoNewWindow` on the Windows step, or redirecting
`stdin` from `NUL`, would restore the fifth archive within the hour and
leave the stopped server shipped. The archive comes back because the
product is fixed, in that order, which is why `TTY-002` is blocked
rather than parallel.

**Not a third `PEISEAR_SETUP` value.** `auto` and `never`. The reasons
`always` is excluded are in `TTY-001` §2, written down so it is not
rediscovered.

## The standing exception still applies

A defect found in the shipped product while this series is open earns a
round of its own rather than being folded into whichever round noticed
it. `0.58.0` closed five rounds against three scoped that way, and
both additions were real.
