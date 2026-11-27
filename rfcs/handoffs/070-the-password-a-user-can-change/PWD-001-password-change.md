# `PWD-001` — Password change

**Series**: `070`. **Governing RFC**: `RFC 025` (`rfcs/accepted/`), accepted
2026-11-27. **Requirement**: `FR-AUTH-006`, `Specified`, P1.
**Do not start until the `0.72.0` row is authorized** — the series README's
gate note says why.

---

## §1 — what is wrong, measured

Nothing is `Divergent`. **This is an absence**, which is the unusual part: no
document claimed the product could do this, so there is no contradiction to
repair — there is a requirement with no implementation.

Measured 2026-11-27 at `9ef2231`:

```
app.rs            68 .route( registrations, 0 containing "password"
handlers/settings.rs                        0 hits for "password"
requirements.md   "password" × 13, never as a requirement to change one
SCR-22            4 sections: Profile, WIP limit, Capacity, Notifications
password_hash     1 production write site (users.rs:111, an INSERT)
                  0 UPDATE sites anywhere
```

**That last pair is the design fact of this unit.** You are adding the first
code path in this product's history that changes a stored credential.

## §2 — the rule

`FR-AUTH-006`, in its own words: a signed-in user MUST be able to change their
own password; the current password MUST be required and verified first; **on
success every *other* session of that account MUST become invalid and the
session that made the change MUST remain valid**; and the new password MUST
meet the constraints registration applies.

**The emphasis is the amendment.** `RFC 025`'s first draft said *every*
session including the acting one. Withdrawn — `§6` of the RFC has the
reasoning. The short form: step 3 below has just proved knowledge of the
current password, which an attacker holding a stolen cookie cannot do, so the
acting session is the legitimate user's and is the one that must survive.

## §3 — the mechanism, in the order it must happen

1. **`SCR-22` gains a section** with three fields — current password, new
   password, confirm — and a submit. It is a form post, not a script: `§5.8`'s
   no-JavaScript obligation applies to it like every other form here.
2. **Validate the new password** against the same rule registration enforces.
   Find that rule rather than restating it; `FR-AUTH-001` names it and
   `register_submit` applies it. **If the two could drift, say so** — a second
   copy of a constraint is a finding, not a detail.
3. **Verify the current password** with `peisear_auth::password`'s existing
   verify. On failure, reject with `FR-AUTH-002`'s neutral message and
   **preserve nothing in the fields** (`§5.3`: errors preserve input, and a
   password field is the one place that rule must not apply).
4. **One transaction**: write the new Argon2 hash and
   `session_generation = session_generation + 1`. Both or neither. A change
   that writes the hash and fails to bump leaves every stolen session alive
   against a password that no longer opens them.
5. **Re-issue this session** on the same response, under the **new**
   generation: `jwt::issue(&id, &email, <the new generation>, &secret)` then
   `jar.add(build_session_cookie(…))`. Read `login_submit` first — these are
   its own two lines. **Take the generation from the value you just wrote, not
   from a re-read**, and not from the stale row you loaded in step 3.
6. **`303` back to `SCR-22`** with a success flash. **Not** `/login`, and
   **not** `§5.3`'s session-notice kind — that flash means *a credential you
   held was invalidated somewhere else*, and reusing it here would give it two
   meanings one release after it was added.

## §4 — acceptance

`FR-AUTH-006` has no acceptance citation yet. These are it, and each needs a
plant that fails first:

1. **The change works and the session survives.** Sign in, change the
   password, and assert the *same* cookie still reaches `/projects` with
   `200` — this is the amended rule, so it is the test that would have caught
   the withdrawn design.
2. **Every other session dies.** Two sessions for one account; change the
   password in the first; the second gets the `SessionRevoked` landing —
   `303 → /login?flash=…` on HTML and `401` on `/api/*`, which is `§9`'s own
   pair from `0.71.0`.
3. **The old password no longer signs in**, and the new one does.
4. **A wrong current password is refused** and changes nothing: assert the
   stored hash is byte-identical afterwards and `session_generation` has not
   moved. A rejection that still bumped the generation would log the user out
   for a typo.
5. **The new password is never persisted or logged in plaintext.**
   `FR-AUTH-001`'s second clause now has a second path to cover.
   **Give this one a population floor**: `0.71.0`'s own
   `a_distinctive_password_never_reaches_storage_or_logs` asserts absence from
   a captured log buffer and **never asserts the buffer is non-empty**, so it
   would pass having captured nothing. Do not copy that shape — assert the
   capture is non-empty, and while you are there, fix the original. The
   architect's probe measured **854 bytes** there, so the floor is real.
6. **Argon2, not something else**: the new hash starts `$argon2` and differs
   from the old one.

`DEC-007` is **703** today; say what it becomes and why.

## §5 — escalate rather than deciding

- **If `FR-AUTH-006` appears to need a migration.** It should not. If it does,
  that is the architect's call, not an implementation detail — and `0025` is
  reserved for `FR-AUTH-007`.
- **If the registration constraint cannot be shared** without duplicating it.
- **If `jwt::issue`'s signature makes step 5 awkward.** `DEC-058` constrains
  what may leave that module; widening it is a design decision.
- **If the copy in `§5` of the RFC reads wrong to you.** Propose; the
  architect reviews. You are closer to the screen than he is.
- **Anything touching `FR-AUTH-007`.** Out of scope here, deliberately.

## §6 — what this unit does not do

No password reset. No e-mail. No rate limiter. No device or session list. No
operator command. `RFC 025` `§4.3` measured why each of those is a separate
round, and `§4.2` holds the reset's shape so it is not redesigned from
scratch later.
