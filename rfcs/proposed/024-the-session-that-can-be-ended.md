# RFC 024: The session that can be ended — logout, and what a credential means after it

**Status**: **Proposed** 2026-11-26, with `0.71.0`'s row (`ROADMAP.md`,
authorized the same day — **the row, not this document**). *Numbered 024, not
023: `RFC 023` was used once — the branching policy, proposed `60cc0ff`
**2026-11-11** and withdrawn into `DEC-060` the same day, `bf4893c` — and its
file was **deleted rather than moved to `archive/`**, so the number is spent
and no folder records that it ever existed. Noted in `§6`.*
**Author**: the architect. **Supersedes nothing.** Opens `§10.66`.
**Related requirements**: `FR-AUTH-001`, `FR-AUTH-002`, `FR-AUTH-003` (P0),
`NFR-SEC-*` (read in full; none governs session lifetime — see `§7`).
**Why it exists as an RFC and not only a register entry**: `§6`.

## Summary

**`FR-AUTH-003` is a P0 requirement whose normative text the shipped product
does not satisfy, and whose status reads `Implemented`.** It says a user MUST
be able to end their session, *"after which protected routes are inaccessible
with the prior credentials"*. `logout` removes a cookie from the client's jar
and does nothing else. The credential keeps working.

This RFC states the one rule that makes the sentence true, picks the cheapest
mechanism that makes it true **by construction**, and says plainly what that
mechanism costs a user who expected otherwise.

## §1 — the measurement this comes from

**Measured 2026-11-26** against a debug build of `cfa45d9` (the `0.70.0`
release commit), served on `127.0.0.1`, with a test account created for the
purpose.

| step | observed |
|---|---|
| `POST /register` | `303`, `Set-Cookie: it_session=<231 chars>` |
| `GET /projects` with that token | **200** |
| `POST /logout` carrying that token | `303`, `Set-Cookie: it_session=; Max-Age=0` |
| `GET /projects` — **same token replayed** | **200** |
| `GET /today` — same token | **200** |
| `GET /api/search` — same token | **200** |

**There is nothing server-side to end.** No `jti` claim, no session table, no
denylist: searched across `crates/*/src`, the only `blocklist` matches are two
comments in `config.rs` about a configuration value. `logout`
(`handlers/auth.rs:121`) is four lines and touches no storage.

**So the exposure is the whole token lifetime**, which `jwt.rs:28` fixes at
`SESSION_TTL_SECS = 60 * 60 * 24 * 7` — **seven days** — and neither the user
nor the operator can shorten it. **There is also no password change or reset
anywhere in the product**: zero matches for `password` in `app.rs`'s route
table and zero in `handlers/settings.rs`. A user who believes their password
is known to someone else has no action available at all.

## §2 — what is actually wrong, which is not "logout is buggy"

`logout` does exactly what its acceptance check asks of it, and the check is
the defect's hiding place.

`logout_clears_session_and_redirects` (`smoke.rs:158`) drives a cookie-jar
client: it logs out, then requests `/projects` **with no cookie**, and asserts
a redirect. **It verifies that the client forgot the credential. It never
asks whether the credential still works.**

That is `§10.36`'s class in its narrower half — the half `§10.36` instance 9
named at the `0.70.0` cut, *a check whose reach is narrower than its name* —
and this is the fourth instance of it inside two days. **It is the first with
a P0 product defect behind it rather than a tooling gap**, which is why it
earns an RFC and not a line in the register.

**The same family has a second, milder instance**, found by reading the four
entries rather than by looking for it. `FR-AUTH-001`'s acceptance has two
clauses; **nothing in the repository names the second** (*"no plaintext
password is persisted or logged"* — zero matches for `plaintext` across
`tests/` and `peisear-auth`). **Measured, not assumed: the product is
clean.** Registering with a distinctive marker password under `RUST_LOG=debug`
leaves that string **0 times** in the SQLite file, **0 times** in the log and
**0 times** anywhere under the data directory; the ≥ 8-character rule holds (7
characters → `400`, 8 → `303`). **True, and evidenced by nobody** — a test,
not a fix.

## §3 — the rule

> **A credential is valid only if it was issued after the subject's sessions
> were last ended. Ending a session is a write, not a deletion of the
> client's copy.**

Two consequences, both deliberate:

- **Logout is server-side.** Removing the cookie stays (the client should not
  keep a dead credential), but it is no longer the mechanism.
- **The check is on the request path, not on logout's path.** A rule enforced
  only where the state changes is a rule a replayed credential walks past.

## §4 — the mechanism, and what it costs

**One column on `users`** — a *sessions-valid-from* instant — written to
`now` by `logout`. `jwt::verify` surrenders the token's `iat`, which `Claims`
already carries and `issue` already sets (`jwt.rs:65`, `:71`). A token whose
`iat` precedes the column is refused.

**Why this and not a `sessions` table keyed by a `jti`:**

| | one column | `jti` + session table |
|---|---|---|
| migrations | 1 | 1 + a table |
| continuous maintenance | **none** | expired rows swept forever |
| granularity | all of that user's sessions | one device |
| UI the product has to express it | **the one Log out button** | a device list it does not have |

The owner's stated axis is *"I don't care initial cost so much but do care
about maintenance cost (continuous) and serious risk"*. A table that must be
pruned for the life of the product, bought to express a precision **the
product has no screen for**, is the wrong side of that axis. **And the cheap
mechanism does not foreclose the precise one**: `iat`-based refusal and a
later `jti` compose, so this is not a decision against per-device revocation.

**What it costs the reader, stated rather than discovered: pressing Log out
ends that user's sessions everywhere.** This is the one user-visible semantic
in this RFC and the one thing a reader could be surprised by, so it is named
in `§5` as copy, not left to be inferred. **The surprising behaviour is the
current one** — logging out leaves you logged in on the machine you walked
away from.

**What the check costs, measured rather than estimated.** `AuthUser`
(`extractors.rs:60`) **already reads the user row on every request**, and its
comment already gives this reason: *"so deleted or altered accounts are
immediately invalidated rather than waiting for the JWT to expire."* The
project bought this read for this purpose. Census by handler signature across
`handlers/*.rs`:

| extractor in the signature | handlers |
|---|---|
| `AuthUser` only | 33 |
| both | 29 |
| `Requester` only | **10** |
| neither (public) | 10 |

**62 of the 72 authenticated handlers already make the read the check rides
on.** The ten that would gain one are
`notification_preferences::{save_preferences,silence_all}`,
`notifications::{resume,email_opt_in,mark_read,mark_all_read}` and
`settings::{insert_capacity,update_capacity,delete_capacity,close_capacity}`
— **every one a mutation of the requester's own personal data**, which is
exactly where a revoked session must not be honoured. The ten are the argument
for the check, not against it.

**What is traded is `Requester`'s documented no-I/O property, not its safety
property.** `PRIV-002`/`DEC-058` makes `RequesterId` unforgeable because only
`jwt::verify` can construct one; a revocation read does not touch that. The
doc comment's *"no DB round-trip, unlike `AuthUser`"* is a performance note,
and it stops being true for those ten. **Said here so the implementer corrects
the comment rather than quietly invalidating it.**

**`jwt`'s module boundary moves by exactly one field.** `Claims` is private
and its doc comment records that nothing outside ever read anything but
`.sub`. Revocation needs `iat`, so `verify` returns one more value — **a
value, not the struct**, keeping `DEC-058`'s shape and its plant.

## §5 — what the product says

The semantic in `§4` is user-visible, so the product states it where the
action is taken rather than leaving a reader to find out. **The exact copy is
the implementer's to propose and the architect's to review**; what this RFC
fixes is that *some* words must say it, on the logout control or its
confirmation, and that they must say it in the i18n catalogue like every other
string, not in a comment.

## §6 — why this is an RFC, and a process finding that belongs to the architect

**Release-cycle entry criterion 1 is *"Governing RFC is in `rfcs/accepted/`"*.
Measured 2026-11-26: series `063` through `068` — six consecutive series, the
releases `0.65.0` through `0.70.0` — **cite no RFC anywhere in their
directories**, and none of their READMEs mentions the criterion or waives it.
Six rounds opened against a criterion nobody read, through six candidates and
six reviews, all of them the architect's.**

That is the same class as `§10.36` instance 9(a): **a rule whose only evidence
is a person following it.** Two documents also disagree about it — `ROADMAP.md`'s
*What ships next* table has a column headed **Governing document** and has been
filled with `requirements.md` entries, while the release cycle says **RFC**.
Whether a register entry may govern a series is **the owner's to settle**, and
this RFC does not settle it.

**A second, smaller finding in the same place, found while filing this
document.** `rfcs/README.md` states the layout — `proposed/`, `accepted/`,
`done/`, `archive/`, `handoffs/` — and that **the folder is the source of
truth for an RFC's state**. On 2026-11-26 neither `proposed/` nor `archive/`
existed: `proposed/` was created by this document, and `archive/` has never
existed, because `RFC 023` — the only RFC ever withdrawn — was **deleted**
rather than moved into it. So the one folder that records a withdrawal has no
instances and no directory, and the index's *Proposed* section read *"None
today"* truthfully only because the folder was gone. **Nothing checks any of
this**, which is this section's own class again, one level down.

**What this RFC does instead is make the criterion-1 question moot for this
round.** Those six
series were guard and accessibility work where the design *was* the
measurement. **This round has a genuine design choice in it** — which
mechanism, and what Log out means afterwards — which is precisely what
criterion 1 exists to have signed off before an implementer starts. So the
criterion is load-bearing here whether or not it was there, and this document
is written to meet it rather than to argue about it.

## §7 — the alternatives, named so they are not rediscovered

- **Shorten the TTL.** Does not end a session; it shortens every session for
  everyone, and a user who logs out still waits. **The 7 days stay**, and that
  is a decision: without revocation the TTL *was* the exposure, with
  revocation it is a policy, and moving both in one release would make neither
  measurable.
- **A denylist of tokens until expiry.** Needs storage anyway and grows with
  logouts rather than with users. Strictly worse than `§4` at the same job.
- **Delegate to an identity provider** (`sui-id`, OIDC). **R1 of `sui-id`'s own
  RP contract asks an embedder for exactly this revocation property**, so this
  work is not wasted if that question reopens — but that delegation is not
  scheduled, and a defect in peisear is not scoped as preparation for it.
- **A device/session list, an operator revocation command, password change,
  e-mail verification.** All new user-facing scope the owner has not asked
  for; the last two need an e-mail flow this release would have to invent.

## §8 — what this RFC does not claim

- **It does not claim the product is otherwise sound on authentication.** It
  read five `FR-AUTH` entries and measured two of them. `FR-AUTH-004` and
  `FR-AUTH-005` were audited at `0.46.0` and are not re-derived here;
  `FR-AUTH-002` is satisfied as written.
- **It does not claim `email` in the claims is right.** `Claims` carries one
  (`jwt.rs:61`), `sui-id`'s R4 raised the same shape upstream, and nothing in
  this RFC changes it. Named, not scoped.
- **It does not make the 7-day window safe for a leaked token.** Revocation
  requires somebody to know and act. What it removes is the state where
  knowing and acting achieve nothing.
