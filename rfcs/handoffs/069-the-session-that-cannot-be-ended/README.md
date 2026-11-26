# 069 — The session that cannot be ended

**Release `0.71.0`** · row proposed and authorized 2026-11-26 (**the row, not
the cut**: exit criterion 7 is a separate decision against a release
recommendation this series does not write).

**Governing document: `RFC 024` — The session that can be ended**
([`rfcs/proposed/024-the-session-that-can-be-ended.md`](../../proposed/024-the-session-that-can-be-ended.md)).

> **⚠ `RFC 024` was amended 2026-11-26, the same day it was filed and before
> this series opened.** The owner asked for it to be reviewed against his
> fundamental philosophy, and **the mechanism changed**: a session
> **generation counter** on `users`, not a timestamp compared against the
> token's `iat`, and the check in **one** place rather than in both
> extractors. `SES-001` `§2`–`§5` and `§7` are the amended instructions;
> `RFC 024` `§4.1` carries the measurement that forced the change and `§4.4`
> the withdrawn alternative. **Nothing had been started, so nothing is to be
> redone.**

> **⚠ Entry criterion 1 is not yet met, and this is the one thing that gates
> the work.** `RFC 024` is **Proposed**, and `rfcs/README.md` says of that
> folder: *"open for review; implementer should not start."* **Do not start
> `SES-001` until `RFC 024` is in `rfcs/accepted/`.** The architect found,
> while filing it, that series `063`–`068` opened with this criterion unmet
> and unmentioned six times running — all six his own lapse — so this series
> states its own entry state instead of inheriting the habit.

**One round.** One property, and the acceptance evidence for the family it
lives in.

---

## 1. What is wrong

`FR-AUTH-003` is **P0**. It says a user MUST be able to end their session,
*"after which protected routes are inaccessible with the prior credentials"*.
**Its status reads `Implemented`. The product does not do it.**

Measured 2026-11-26 against a debug build of `cfa45d9` (the `0.70.0` release
commit) on `127.0.0.1`:

| step | observed |
|---|---|
| `POST /register` | `303`, `Set-Cookie: it_session=<231 chars>` |
| `GET /projects` with that token | **200** |
| `POST /logout` carrying that token | `303`, `Set-Cookie: it_session=; Max-Age=0` |
| **same token replayed** → `/projects` | **200** |
| same token → `/today` | **200** |
| same token → `/api/search` | **200** |

`logout` (`handlers/auth.rs:121`) is four lines, touches no storage, and there
is nothing server-side to end: **no `jti`, no session table, no denylist**
anywhere in `crates/*/src`. The credential stays valid for the rest of
`SESSION_TTL_SECS` — **7 days** (`jwt.rs:28`) — and neither the user nor the
operator can shorten that.

**Reproduce it before changing anything.** That is round-1 step one, and the
figures above are to be re-derived rather than trusted: *the architect's
measurements are claims too.*

## 2. Why it survived, which is the part worth reading

The acceptance citation is `logout_clears_session_and_redirects`
(`smoke.rs:158`). It drives a **cookie-jar client**: it logs out, then asks
for `/projects` **with no cookie**, and asserts a redirect.

**It verifies that the client forgot the credential. It never asks whether the
credential still works.**

`§10.36` instance 9, written at the `0.70.0` cut two days ago, named this
half of the class — *a check whose reach is narrower than its name*. This is
its fourth instance in two days and **the first with a P0 product defect
behind it** rather than a tooling gap. When you write the replacement test,
the question to ask it is `§10.36`'s own: **what did it examine, and how would
I know if the answer were nothing?**

## 3. What this series must not do

- **Do not shorten the TTL.** It stays at 7 days, deliberately: without
  revocation it *was* the exposure, with revocation it is a policy, and moving
  both at once makes neither measurable (`RFC 024` `§7`).
- **Do not add** password change or reset, e-mail verification, a device or
  session list, or an operator revocation command. Each is new user-facing
  scope the owner has not authorized.
- **Do not weaken `PRIV-002`/`DEC-058`.** `RequesterId` must stay
  constructible only by `jwt::verify`. What this round trades is
  `Requester`'s documented *no-I/O* property — a performance note, not a
  safety one. The claim stops being true wherever `Requester` is used, even
  though the **net** reads per request rise only for the ten handlers that
  take it alone (`SES-001` `§3`), and the doc comment that states it must be
  corrected, not quietly invalidated.
- **Do not open a new `FR`/`NFR` number.** `FR-AUTH-003`'s own text already
  requires this; its **acceptance** widens.

## 4. The rounds

| Round | Unit | Subject |
|---|---|---|
| `SES-001` | [the session that cannot be ended](./SES-001-the-session-that-cannot-be-ended.md) | Make logout end the session server-side, and give the `FR-AUTH` family acceptance evidence that matches its own words |

**One round, not three.** `0.70.0`'s own scope was wrong when it was
authorized and the owner caught it by asking for a philosophy review; the flaw
was the architect's shape, not his measurement. This series is scoped as one
property with one test beside it for that reason.

## 5. Working rules for this series

- **Reproduce before repairing, and measure after.** Every figure in `SES-001`
  is to be re-derived at source. Three of the architect's own carried figures
  were wrong when he re-measured them while scoping this release.
- **Plant every guard and watch it fail.** A check that has not been seen red
  is a check whose reach is unknown — this project's most expensive recurring
  lesson.
- **`DEC-007`'s block is the test gate**, 42 commands, and the count moves
  when tests are added: say the new number, measured with the runner, never
  by grep or arithmetic.
- **Scratch builds go under `.git-exclude/tmp/` via `CARGO_TARGET_DIR`**, and
  the target directories are removed when the round closes.
- **Nothing published may carry local information.** Screenshots and other
  non-text artefacts are unscanned by any gate; the privacy floor is a named
  per-artefact exit condition, not a habit.
- **A blocking finding that needs the architect's decision goes in a
  review-request file**, not a question asked in passing.
- **The report names the path it wrote.** The architect is blocked until it
  can be relayed.
