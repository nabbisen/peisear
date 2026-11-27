# RFC 025: The password a user can change — and the reset that cannot ship with it

**Status**: **Proposed** 2026-11-27 by the architect. Not accepted. Until the
owner accepts it, release-cycle **entry criterion 1 is unmet** for `0.72.0`
and no handoff may start.
*Numbered 025, re-derived from the folders rather than trusted: `000`–`013`
and `015` are in `done/`, `014`, `016`–`022` and `024` in `accepted/`, and
`023` is spent with no file anywhere (the withdrawn branching policy).*
**Author**: the architect. **Supersedes nothing.**
**Proposes two new requirements** — `FR-AUTH-006` and `FR-AUTH-007`, the
first two numbers free in that family — **and the mechanism for only one of
them**, for the reason in `§4.3`.
**Related requirements**: `FR-AUTH-001` (registration, and the no-plaintext
clause), `FR-AUTH-002` (login), `FR-AUTH-003` (logout, P0, met at `0.71.0`),
`NFR-SEC-003` (password storage), **`FR-NTF-008` — Graceful degradation without
SMTP** (P1, `Implemented`), which is the requirement that makes `§4.3`'s
first prerequisite a consequence rather than a judgement call.
*Corrected after writing: this line first cited `FR-NTF-007`, the deferred
email opt-in, which is about **when** the opt-in appears and not about a
config-dependent surface at all — the same mis-citation failure this
architect made with `NFR-REL-011` earlier the same day, caught here by
reading the entry instead of remembering it.*

---

## Summary

**`0.71.0` shipped half a remedy and said so in its own release notes.** A
user who believes their password has been seen can now end every session on
every device, immediately — and then has no way to stop the person who knows
that password from signing straight back in.

**This RFC proposes the other half, and proposes shipping it in two pieces
rather than one.** A password **change**, for a user who can still sign in,
is small, complete and depends on nothing that does not already exist. A
password **reset**, for a user who cannot, depends on three things this
product does not have — and one of them means a reset could silently fail on
a self-hosted instance, which is worse than not offering it.

**So: `FR-AUTH-006` (change) `Specified` and built; `FR-AUTH-007` (reset)
accepted as a requirement and recorded `Deferred`, with its three
prerequisites named so the next round starts from them rather than
rediscovering them.**

---

## §1 — the measurement this comes from

Taken 2026-11-27, against `main` at `45d6a88`:

- **`requirements.md` specifies password change or reset nowhere.** Not
  `Implemented`, not `Divergent`, not `Deferred`, in neither `§11` nor any
  phase. The word *password* appears **13** times in that document and never
  once as a requirement to change or reset one.
- **The product agrees.** `0` hits for `password` in `app.rs`'s **68**
  `.route(` registrations; `0` in `handlers/settings.rs`.
- **`SCR-22` has four sections** — Profile, WIP limit, Capacity,
  Notifications — and none of them is a credential.

**So this is not a defect and nothing is `Divergent`.** No document claims
the product can do this. **It is an absence**, which is why it needs a
requirement before it can need a mechanism, and why the owner's decision
precedes the architect's.

**And it is the owner's second philosophy sentence in its hardest form.**
*APIs and UI/UX for users not to be confused or misunderstand* is usually
about a screen that misleads. Here there is no screen. A user who needs to
change their password opens Settings, finds Profile, WIP limit, Capacity and
Notifications, and has to conclude that this product does not think the
question is one they would ask.

---

## §2 — what already exists, so neither half is re-litigated on feasibility

Measured rather than assumed:

- **Hashing**: `peisear-auth/src/password.rs` — `Argon2::default()`, hash and
  verify. `NFR-SEC-003` governs it and `FR-AUTH-001`'s second clause (no
  plaintext persisted or logged) gained its first real test at `0.71.0`.
  **Counted rather than asserted, and the count is useful**: `password_hash`
  has exactly **one** production write site, the `INSERT` at
  `peisear-storage/src/users.rs:111`, and **zero `UPDATE` sites anywhere**.
  So `FR-AUTH-006` adds the first path that ever changes a stored credential,
  which is why `§4.1` step 3 insists the write and the generation bump share
  one transaction. *(Three test fixtures also insert a `password_hash`
  directly, with non-Argon2 values; an earlier draft of this line called
  Argon2 “the only route”, which those fixtures contradict.)*
- **Session invalidation**: `session_generation` on `users`, advanced by one
  `UPDATE`, checked at the single choke point `verified_requester_id`
  (`RFC 024` §4.2). **A password change that leaves the old password's
  sessions alive is not a password change**, and `0.71.0` made that one
  statement rather than a design problem.
- **Mail**: `peisear-notify::email::send_email(cfg, to, subject, body)` over
  the `wasm-smtp` family with a production tokio/rustls transport. `§1.1` of
  the external design already carries an SMTP entity row.
- **Migration numbering**: the highest file is `0024` on **both** backends,
  so the first number free on both is **`0025`**. Re-derived here because the
  `SES-001` handoff got exactly this wrong by one and the dev team caught it.

---

## §3 — the two requirements this proposes

**`FR-AUTH-006` — Password change.** A signed-in user MUST be able to change
their own password. The current password MUST be required and verified before
the new one is accepted. On success, **every other session of that account
MUST become invalid, and the session that made the change MUST remain
valid** — the user is not signed out of the browser they are using. The new
password MUST be subject to the same constraints registration applies.
*Priority*: **P1**. *Source*: this RFC, on the owner's decision.

**Amended 2026-11-27, before acceptance, on the owner's philosophy review.**
This entry first read *"every session … **including the one that made the
change**, and the user MUST be returned to sign-in"*. **That was wrong, and it
was wrong in the normative sentence rather than in the prose around it** —
`§4.1` and `§6` carry the reasoning and the withdrawn version.

**`FR-AUTH-007` — Password reset.** A user who cannot sign in MUST be able to
regain access to their account without an operator's intervention, by proving
control of the address the account was registered with.
*Priority*: **P1**. *Status*: **`Deferred`**, and the status table's own
definition is *“accepted as a requirement, deliberately postponed to a named
phase”* — **so the phase is named here and it is not a release number**: a
release after the three prerequisites in `§4.3` are met, each of which is a
round in its own right. **Stated plainly because this register is full of
entries whose status claimed more than was true**: no number is assigned, and
pretending one is would be the drift `§10.36` collects.
*Source*: this RFC, on the owner's decision.

**Why the second is written now rather than when it is built**: so that the
specification records the goal rather than the architect's current capacity,
and so a reader of `FR-AUTH-006` can see that it is half of something.

---

## §4 — the mechanism, and why only one half of it

### §4.1 — the change, in full

Four things, and none of them is new machinery:

1. A section on `SCR-22` with three fields — current password, new password,
   confirm — and a submit.
2. The handler verifies the current password with `peisear_auth::password`'s
   existing verify, rejecting with the same neutral framing `FR-AUTH-002`
   uses, and **never** disclosing which field was wrong.
3. On success, in one transaction: hash the new password with the existing
   hasher, write it, and **advance `session_generation`**. Every credential
   issued under the old generation — every other device, every stolen cookie
   — is dead at the next request, by the mechanism `0.71.0` already proved.
4. **Then re-issue this session under the new generation and set the cookie on
   the same response**, which is the two lines `login_submit` already runs:
   `jwt::issue(&id, &email, user.session_generation, &secret)` followed by
   `jar.add(build_session_cookie(…))`. The user stays signed in, on the page
   they were on, with a success flash. **No `/login` detour and no new copy on
   that screen.**

**Step 2 is the half that is easy to get wrong.** A change form that does not
re-ask the current password turns an unattended browser into an account
takeover — the attacker does not need the password, only the open tab. This
is the one place in this RFC where a convenience and a security property
conflict, and the security property wins without argument.

**Step 4 is a correction to this RFC's own first draft, made on the owner's
philosophy review and kept visible rather than quietly fixed.** That draft
signed the user out of the browser they changed the password in, and argued:
*exempting the current session means the one session an attacker might be
holding is the one that survives.*

**That argument inverts the facts.** Step 2 has just required proof of the
**current password**. An attacker holding a stolen cookie cannot supply it —
that is the entire reason step 2 exists. **So the session performing the
change is the legitimate user's, with as much confidence as this product ever
has about anyone**, and it is the one session that must *not* be revoked.
Revoking it ends the user's own session to protect them from themselves,
drops them at a sign-in screen for doing the right thing, and buys **nothing**
— every session that could be an attacker's is already dead from step 3.

**And the corrected version is smaller**: one fewer screen transition, one
fewer copy string, and `§5`'s second site disappears. Where a design choice
and *less is more* point the same way, the first draft was usually reasoning
backwards from a mechanism it had already chosen.

### §4.2 — the reset's shape, specified but not built

Recorded so that `FR-AUTH-007`'s `Deferred` status has content:

- **A single-use, expiring token**, delivered to the registered address, with
  **only its hash stored** — the same posture `NFR-SEC-003` takes for
  passwords, so that a database read does not grant account access.
- **One outstanding reset per account**, which makes two columns on `users`
  sufficient and bounds abuse by construction. `RFC 024 §4.5` already
  rejected a side table for sessions on the same reasoning: a table is the
  right shape when the rows are many and independent, and these are neither.
- **A response identical whether or not the address exists.** Byte-identical,
  same status, same timing class — otherwise the screen is an
  account-enumeration oracle, and this product's own `§9` already reasons
  this way about 403-versus-404.
- **Session invalidation on completion**, same `UPDATE` as `§4.1`.

### §4.3 — why the reset cannot ship with the change, measured

**Three prerequisites, none of which exists today.**

**(a) SMTP is optional in this product, and a reset that cannot be delivered
is worse than none.** `peisear-notify`'s own config logs *"SMTP not
configured … the email channel will fail at send time; in-app delivery
continues to work"* and carries on. That is correct for a notification. **It
is not survivable for a credential recovery path**: a user told *"check your
email"* on an instance with no mail configured is left waiting for something
that will never arrive, with no way to learn that. **That is the philosophy's
second sentence failing worse than the absence this RFC is written to fix.**

**And this is not the architect's opinion about mail — it is already a P1
requirement.** `FR-NTF-008` reads: *with SMTP unconfigured, the system MUST
log a warning at startup and continue operating with in-app notifications.*
`§1.1` of the external design calls the relay **optional** in its own words.
**So the specification already requires this product to work without mail**,
and a credential-recovery path that only works with mail would be the first
feature to contradict that posture. That makes `§4.3(a)` a consequence of an
accepted requirement rather than a preference.

The remedy is not a better error message after the fact. **The affordance
must not appear where it cannot work**: no *"Forgot your password?"* link on
`SCR-02` when SMTP is unconfigured, and a direct visit to the route stating
plainly that this instance cannot send mail and naming who to ask.

**(b) Nothing in this product rate-limits anything.** Measured, and widened
after the first draft: **0** hits for `rate_limit`, `ratelimit`, `governor`,
`tower-governor`, `leaky bucket` or `throttl*` across **every** crate's
`.rs` **and** `.toml`, tests included — not a dependency, not a middleware,
not a helper. A
reset endpoint without one is a mail cannon that any stranger can point at
any address in the instance. **The reset half would be the first thing here
to need a rate limiter, and a rate limiter is its own design** — where the
counter lives, what it keys on, what it does to a legitimate retry — not a
line inside a password feature.

**(c) A mail-less instance leaves a locked-out user permanently locked out,
and this RFC does not solve that.** If the affordance is correctly hidden
when SMTP is unconfigured, then on those instances the answer is an operator
action, and **no operator action exists**. Measured: `crates/peisear` has one
binary target and `src/` holds only `lib.rs` and `main.rs`, with no
subcommand parsing of any kind — **there is no administrative CLI**.
*And the specification has no operator-account concept to appeal to either.
`§2.2`'s administrator boundary — which an earlier draft of this paragraph
cited as though it covered this — is about a **team** administrator's
authority over membership and sprint lifecycle and their lack of authority
over personal data. It does not speak to instance operators at all, so the
gap is an absence in the specification rather than a limit stated by it.*
**Named, not solved**: it is a third feature, and pretending otherwise is how
a release grows a scope nobody authorized.

### §4.4 — what this does not change

The `users` table gains **nothing** for `FR-AUTH-006`. No migration, no new
column, no new table. `0025` is reserved by `§2`'s derivation for the reset
half when it comes, not spent here.

---

## §5 — what the product says

Two copy sites, and the second is the one a reader will remember:

**One copy site, not two — and that is a consequence of `§4.1` step 4 rather
than a saving sought for its own sake.**

**`SCR-22`'s new section** states, beside the control and before it is used,
that changing the password signs out **every other device** and keeps this
one. Stated where the action is taken, which is the rule `RFC 024 §5`
established and `0.71.0` shipped — and it is now a sentence a user is glad to
read rather than a warning they must accept.

**There is no second site.** The first draft of this RFC needed one: a landing
on `/login` explaining why the user had just been signed out. `§4.1` step 4
removed the reason for it, so `§5.3`'s *session notice* kind is **not** reused
here and keeps exactly the one meaning `0.71.0` gave it — *a credential you
held was invalidated somewhere else*. Stretching it to cover *you changed your
own password a second ago, here* would have made the fourth flash kind mean
two different things within one release of its being added.

`external-design.md` gains: a fifth row in `SCR-22`'s section table, a state
on that screen, and **no new `§9` row** — the rejection reuses
`FR-AUTH-002`'s status and framing, which `§4.1` step 2 requires, and the
success is a `303` back to `SCR-22`, which `§9`'s *Success (mutation)* row
already covers.

---

## §6 — alternatives, named so they are not rediscovered

- **A change form that does not re-ask the current password.** Rejected in
  `§4.1`; an open tab becomes a takeover. This is the one rejection in this
  document that survived its own review unchanged.
- **Signing the acting session out along with the others.** **This RFC's own
  first draft, withdrawn on the owner's philosophy review** (`§4.1` step 4).
  It argued that exempting the current session preserves the session an
  attacker might hold — which inverts step 2, where that attacker has just
  failed to supply the current password. It cost a screen transition, a copy
  string and a stretched meaning for `§5.3`'s newest flash kind, and bought
  nothing. **Recorded rather than deleted**, the same way `RFC 024 §4.4`
  records the `iat` mechanism this architect also recommended first.
- **Both halves in one release.** Rejected in `§4.3`, and this is a change
  from the architect's own recommendation of 2026-11-27 earlier the same day,
  which was *both halves, with reset as the half that justifies the
  release*. **Measuring (a), (b) and (c) reversed it.** Recorded rather than
  quietly dropped.
- **A reset token in a side table.** Deferred with the half it belongs to;
  `§4.2` states why two columns are enough.
- **An operator CLI reset.** A third feature (`§4.3c`), named and excluded.

---

## §7 — what this RFC does not claim

- **It does not claim the change half is urgent.** No user is currently
  misled by a screen; they are not offered one. `FR-AUTH-003` is met, so the
  user who can sign in already has a real remedy against a stolen cookie —
  what they lack is one against a stolen password.
- **It does not claim a reset is more valuable than a change.** It claims the
  reset cannot be built responsibly yet, which is a different statement.
- **It does not size the work.** That is the handoff's job, and no handoff
  exists.
- **It proposes two requirements and the owner may accept one, both or
  neither.** Accepting `FR-AUTH-006` alone is a coherent outcome; accepting
  `FR-AUTH-007` alone is not, since `§4.1`'s machinery is what a reset would
  finish with.

---

## §8 — the decision this document asks for

**One decision, and it is not a mechanism review.** Are password change and
password reset requirements of this product?

- **Yes to both, as scoped here** — `FR-AUTH-006` built now,
  `FR-AUTH-007` recorded `Deferred` with `§4.3`'s three prerequisites. This
  is the architect's recommendation.
- **Yes to the change only** — `FR-AUTH-007` is not written, and `§4.2`
  becomes a note rather than a specification.
- **No** — this number is free, and the guard rounds under `ROADMAP.md`'s
  *Unscheduled work* are the honest alternative.

On a yes, the requirement entries are written, this document moves to
`rfcs/accepted/`, entry criterion 1 is met, and the handoff can be drafted.
