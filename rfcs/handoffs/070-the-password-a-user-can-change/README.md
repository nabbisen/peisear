# 070 — The password a user can change

**Governing RFC**: [`rfcs/accepted/025-the-password-a-user-can-change.md`](../../accepted/025-the-password-a-user-can-change.md),
**accepted 2026-11-27** by the owner.
**One unit**: [`PWD-001`](./PWD-001-password-change.md).
**Release**: `0.72.0` — *proposed, not yet authorized* (see below).

---

> **✓ Entry criterion 1 is met: `RFC 025` is in `rfcs/accepted/`, accepted
> 2026-11-27.** Criterion 2: the RFC leaves one thing to the implementer —
> the exact copy, which `§5` delegates to you to propose and the architect to
> review. Criterion 3: this directory. Criterion 4: `0.71.0`'s amendments were
> applied at its own cut.
>
> **✓ The `0.72.0` row was authorized by the owner 2026-11-28. `PWD-001` may
> start.** All four entry criteria are met: the governing RFC is in
> `rfcs/accepted/` (1), it leaves only the copy open (2), this directory
> exists (3), and `0.71.0`'s amendments were applied at its own cut (4).
>
> **What is *not* authorized is the cut.** In this project, accepting an RFC,
> authorizing a row and approving a cut are three decisions — taken here on
> two separate days — and the third has not been taken. `DEC-056`'s four legs
> run only after exit criterion 7, and **never by the dev team**. Build,
> test, and hand back; the release is not yours to make and not yet anyone's
> to make.
>
> **⚠ `RFC 025` was amended twice before acceptance, and the second amendment
> changed its normative rule.** Its first draft required a password change to
> end **every** session *including the one making the change*, sending the
> user to `/login`. That is withdrawn. **The rule is now: every *other*
> session dies, and the acting session survives, re-issued under the new
> generation.** `FR-AUTH-006`'s entry and `RFC 025` `§4.1` step 4 are the
> instruction; `§6` records the withdrawn version. **Nothing had been started,
> so nothing is to be redone** — but do not work from a cached reading of the
> RFC's first draft.

## What this series is

`0.71.0` made *Log out* genuinely end every session. Its own release notes had
to say that this is **a partial remedy and not the missing one**: whoever knows
the password signs straight back in. **There was no way to change a password
anywhere in the product**, and no requirement asking for one — measured at
acceptance: `0` hits for `password` in `app.rs`'s **68** `.route(`
registrations, `0` in `handlers/settings.rs`, and the word appearing 13 times
in `requirements.md` without ever being a requirement to change one.

`RFC 025` proposed two requirements and the owner accepted both.
**`FR-AUTH-006` — password change — is this series.** `FR-AUTH-007` — password
reset — is accepted and **`Deferred`**, with three prerequisites that are each
a round in their own right; `RFC 025` `§4.3` has them measured. **Do not build
any part of the reset here**, including a token column: `0025` is reserved for
it and this series adds no migration.

## Working rules for this series

These reach you only through this file, so they are stated rather than
assumed:

- **No migration, no new column, no new table.** `FR-AUTH-006` needs none.
  If you believe it does, that is an escalation, not an implementation
  decision.
- **The current password must be verified before the new one is accepted**,
  and the rejection must reuse `FR-AUTH-002`'s neutral framing — never
  disclosing which field was wrong. No new `§9` status row.
- **One transaction** for the hash write and the `session_generation` bump.
- **Re-issue the acting session on the same response.** `login_submit` already
  does exactly this: `jwt::issue(&id, &email, user.session_generation,
  &secret)` then `jar.add(build_session_cookie(…))`. Read it before writing
  anything.
- **Propose the copy; do not invent policy with it.** `§5` gives one site:
  `SCR-22`'s new section, stating that a change ends every *other* device and
  keeps this one.
- **A guard before a green run.** This project does not ship a rule it has not
  seen fail: plant each new assertion and watch it fail before trusting it.
- **Re-derive anything this document numbers.** The `SES-001` handoff told you
  to add `0023_*` and was wrong by one; you caught it. Do that again.

## What the architect owes this series

- The row's authorization is the owner's, and the architect brings it.
- `external-design.md`'s amendment at the cut, per `DEC-028`: a fifth row in
  `SCR-22`'s section table and a state on that screen. **Not** a new `§9`
  row and **not** a second use of `§5.3`'s session-notice flash — `RFC 025`
  `§5` says why.
- The review of your proposed copy.
