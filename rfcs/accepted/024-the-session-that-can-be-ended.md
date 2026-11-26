# RFC 024: The session that can be ended — logout, and what a credential means after it

**Status**: **Accepted** 2026-11-26 by the owner, after the amendment below.
*Proposed the same day, with `0.71.0`'s row (`ROADMAP.md`, authorized then
too — the row, the amendment and this acceptance were three separate
decisions, taken in that order.)* **This is release-cycle entry criterion 1,
and `SES-001` may now start.** *Numbered 024, not
023: `RFC 023` was used once — the branching policy, proposed `60cc0ff`
**2026-11-11** and withdrawn into `DEC-060` the same day, `bf4893c` — and its
file was **deleted rather than moved to `archive/`**, so the number is spent
and no folder records that it ever existed. Noted in `§6`.*
**Amended 2026-11-26**, the same day, **before acceptance**: the owner asked
for the three open decisions to be reviewed against his fundamental
philosophy — *"finally clean, safe and secure, and robust and sophisticated
design"* and *"APIs and UI/UX for users not to be confused or misunderstand
are also very important"* — and that review **changed this document's
mechanism**. `§3`'s rule is restated as a generation rather than an instant,
`§4` is rewritten, and the mechanism first recommended here is recorded as a
rejected alternative in `§4.4` rather than deleted. The defect, the rule's
intent, the scope and the priority are unchanged.
**Author**: the architect. **Supersedes nothing.** Opens `§10.66`.
**Related requirements**: `FR-AUTH-001`, `FR-AUTH-002`, `FR-AUTH-003` (P0),
`NFR-SEC-*` (read in full; none governs session lifetime — see `§7`).
**Why it exists as an RFC and not only a register entry**: `§6` — and since
2026-11-26 that is no longer a choice, by the owner's ruling recorded there.

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

> **A credential is valid only if it was issued under the subject's current
> session generation. Ending a session advances the generation. Ending a
> session is a write, not a deletion of the client's copy.**

**This rule was first written as a statement about time** — *"valid only if it
was issued **after** the subject's sessions were last ended"* — and that is
still exactly what it means. It is stated as a **generation** because a rule
worded in time invites an implementation that compares clocks, and `§4.1`
measures why a clock is the wrong thing to compare here. The intent did not
change; the thing the code compares did.

Three consequences, all deliberate:

- **Logout is server-side.** Removing the cookie stays (the client should not
  keep a dead credential), but it is no longer the mechanism.
- **The check is on the request path, not on logout's path.** A rule enforced
  only where the state changes is a rule a replayed credential walks past.
- **The check is in one place, and it is a place nothing can go around.**
  `§4.2`. A rule enforced in two extractors is a rule the third one skips.

## §4 — the mechanism, and what it costs

**One integer column on `users`** — `sessions_epoch`, a **session
generation**, `INTEGER NOT NULL DEFAULT 0`. **`logout` advances it**
(`sessions_epoch + 1`) and still clears the cookie. **`Claims` carries the
generation its token was issued under**, and `verify` surrenders that value —
**a value, not the struct**, so `DEC-058`'s shape and its plant survive
unchanged. **A request whose token's generation differs from the row's is
refused.** Nothing is compared but two integers.

`login_submit` (`auth.rs:90`) **already loads the user row** to verify the
password, and `register_submit` has just written it, so **stamping the
generation into a new token costs no read on either path**.

### §4.1 — why a counter, and not the timestamp the token already carries

**This project has already paid a migration for exactly this comparison, and
the migration is on disk.** `0021_updated_at_millisecond_precision.sql` opens
with its own reason: SQLite's `CURRENT_TIMESTAMP` is whole-second, `updated_at`
doubles as the optimistic-lock token, *"so two writes to the same row inside
one second leave the token unchanged, and a client holding a stale token
compares equal and is admitted"* — demonstrated through the handlers in
`optimistic_lock.rs`'s `issue_edited_twice_in_the_same_second_with_the_same_token_must_409`,
red before the migration and green after.

**`iat` is whole seconds too** — `jwt.rs:71` sets it from
`chrono::Utc::now().timestamp()`, written into the claim at `:75`, and that is the JWT claim's defined
resolution, not an implementation choice this project can tighten. Comparing it
against a stored instant reproduces the `0021` window three ways, and **which
one you get depends on a decision the implementer would otherwise make
silently**:

- **Compare at second resolution** and a credential replayed inside the
  logout's own second is admitted. Small, but it is the exact property the
  requirement's words forbid, and it would be the one case the new test does
  not cover.
- **Compare finer** — store milliseconds, as `0021` taught this project to do —
  and a user who logs out and **immediately logs back in** is refused for the
  remainder of that second and lands back on the login page. That is the
  second sentence of the owner's philosophy broken by the fix itself, and the
  project's own smoke tests log in within milliseconds of logging out, so the
  pressure would be to paper over it with a sleep in the test.
- **Either way, a clock step** — an NTP correction backwards — refuses
  newly-issued credentials for the length of the step.

**A counter has none of these.** There is no resolution, no timezone, no
`DATETIME`-versus-`TIMESTAMPTZ` divergence between the two backends (`users`
already carries one of each: `created_at DATETIME` in
`migrations/0001_initial.sql`, `TIMESTAMPTZ` in `migrations-postgres/`), and no
boundary case to state, test, or get wrong. **The test is `!=` on an integer.**

**It is also the cheaper operator story**: ending every session of a user is
`UPDATE users SET sessions_epoch = sessions_epoch + 1 WHERE id = ?`, which
needs no clock and cannot be got wrong by a minute.

### §4.2 — one enforcement point, which the tree already has

**The check belongs in `verified_requester_id` (`extractors.rs:22`) and
nowhere else.** That function is already documented as *"the only way either
extractor (or anything else in the workspace) can obtain one"*, and all five
extractors funnel through it: `AuthUser` and `Requester` directly,
`ApiRequester` through `Requester`, `ApiAuthUser` and `MaybeAuthUser` through
`AuthUser`.

**So the revocation check inherits the property `PRIV-002`/`DEC-058` already
rests on**: a handler cannot obtain a `RequesterId` without it, including a
handler written next year with a sixth extractor. **A rule placed in "both
extractors" would be a rule the sixth one skips** — the same shape as the
defect this RFC exists for, rebuilt inside the fix.

**It also costs nothing to get the two rejection shapes right.** A refused
generation returns `AppError::Unauthorized`, which the HTML paths already turn
into a `303` to `/login` and `ApiAuthUser`/`ApiRequester` already translate
into a JSON `401`. **No new rejection path is written.**

### §4.3 — what the check costs, measured rather than estimated

**The earlier reading of this cost was wrong in a way worth recording.** It
said *62 of the 72 authenticated handlers already make the read the check
rides on* — true only if the check lives inside `AuthUser`'s existing
`users::find_by_id`, which is **not** where `§4.2` puts it. In the choke
point, the read is new for **all** of them unless the row fetched there is the
one `AuthUser` uses.

**So it is**: `verified_requester_id` reads the user row once — it needs it for
the generation anyway — and hands it on, so **`AuthUser` stops doing its own
second `find_by_id`**. Census by handler signature across `handlers/*.rs`
(architect's, 2026-11-26 — re-derive it):

| extractor in the signature | handlers | reads per request, today → after |
|---|---|---|
| `AuthUser` only | 33 | 1 → **1** |
| both | 29 | 1 → **1** (one row, cached in `parts.extensions`) |
| `Requester` only | **10** | 0 → **1** |
| neither (public) | 10 | 0 → 0 |

**The 29 are the one place an implementer can make this worse**: they extract
both, so `verified_requester_id` runs twice per request — it already does
today, decoding the JWT twice, which `Requester`'s doc comment calls cheap
because it is I/O-free. With a read in it that stops being true, so **the row
is memoised in `parts.extensions`**, which also removes the double decode the
project accepted before. One read per request, whichever extractors a handler
takes.

**The ten that gain a read** are
`notification_preferences::{save_preferences,silence_all}`,
`notifications::{resume,email_opt_in,mark_read,mark_all_read}` and
`settings::{insert_capacity,update_capacity,delete_capacity,close_capacity}`
— **every one a mutation of the requester's own personal data**, which is
exactly where a revoked session must not be honoured. The ten are the argument
for the check, not against it.

**What is traded is `Requester`'s documented no-I/O property, not its safety
property.** `PRIV-002`/`DEC-058` makes `RequesterId` unforgeable because only
`jwt::verify` can construct it; a revocation read does not touch that. The doc
comment's *"no DB round-trip, unlike `AuthUser`"* is a performance note, and it
stops being true. **Said here so the implementer corrects the comment rather
than quietly invalidating it.**

**Tokens issued before the upgrade must keep working.** `ses` is read with
`#[serde(default)]`, so a token minted by `0.70.0` — which has no such claim —
deserialises as generation `0` and matches the column's default. **Without
that, upgrading to `0.71.0` logs every user out at once**, which is both a
surprise nobody asked for and an upgrade cost this project started paying
attention to when `0.53.0` shipped installable artefacts. It is one attribute,
and it is the difference between a silent mass logout and nothing visible at
all.

### §4.4 — the mechanism this replaced, recorded rather than deleted

**First recommended here, 2026-11-26**: one `users` column holding a
*sessions-valid-from* **instant**, written to `now` by `logout`, with
`jwt::verify` surrendering the token's `iat` and a token whose `iat` precedes
the instant refused. Its appeal was that it **adds no claim** — `iat` is
already in `Claims`, already set at issue — and on *"less is more"* that read
as the smaller change.

**It is not the smaller change.** It adds no claim and in exchange imports a
clock, a resolution, two backend time types and the boundary case in `§4.1`;
the counter adds one small integer claim and imports nothing. Counted in
*concepts that can be wrong*, the counter is the smaller of the two — and the
only argument for the timestamp was a count of *fields*.

**Withdrawn on the owner's philosophy review, before acceptance**, and
recorded because the reasoning that produced it (*reuse what the token already
carries*) is the reasoning that will produce it again at the next session
question.

### §4.5 — why not a `jti` and a session table

| | one generation column | `jti` + session table |
|---|---|---|
| migrations | 1 | 1 + a table |
| continuous maintenance | **none** | expired rows swept forever |
| granularity | all of that user's sessions | one device |
| UI the product has to express it | **the one Log out button** | a device list it does not have |
| what the check compares | two integers | a row's existence |

The owner's stated axis is *"I don't care initial cost so much but do care
about maintenance cost (continuous) and serious risk"*. A table that must be
pruned for the life of the product, bought to express a precision **the
product has no screen for**, is the wrong side of that axis. **And the cheap
mechanism does not foreclose the precise one**: a generation check and a later
per-device `jti` compose, so this is not a decision against per-device
revocation.

**What it costs the reader, stated rather than discovered: pressing Log out
ends that user's sessions everywhere.** This is the one user-visible semantic
in this RFC and the one thing a reader could be surprised by, so it is named
in `§5` as copy, not left to be inferred. **The surprising behaviour is the
current one** — logging out leaves you logged in on the machine you walked
away from.

## §5 — what the product says, in two places

The semantic in `§4` is user-visible, so the product states it where the
action is taken rather than leaving a reader to find out. **The exact copy is
the implementer's to propose and the architect's to review**; what this RFC
fixes is that *some* words must say it, and that they say it **in the i18n
catalogue like every other string, never in a comment**.

**Two sites, not one.** The first was named when this RFC was filed; the
second is the one a user actually meets, and the review that amended `§4`
found it missing.

1. **Where the action is taken** — on the logout control or its confirmation:
   *pressing Log out ends that user's sessions everywhere*.
2. **Where the consequence lands** — the **other** device. Today a refused
   credential produces a bare `303` to `/login` with no explanation,
   **indistinguishable from an expired session and from never having signed
   in at all**. A user whose laptop silently returns to a login screen has no
   way to tell a revocation from a bug. The product must say which it was.

**The mechanism for the second site already exists**, so this is copy and a
redirect target, not new machinery: `login_page` takes `?flash=`
(`auth.rs:26`) and renders it, and every handler that carries dynamic text
through a redirect sends it through `percent_encode_query`, which
`one_encoder_scan` exists to keep. **The flash carries copy, never anything
about the user** — no identifier, no email, nothing that makes a URL carry
personal data.

**One more behaviour to get right rather than discover.** `logout` currently
needs no session at all (`handlers/auth.rs:121` takes only the jar); once it
writes, it needs to know whose generation to advance. **Pressing Log out with
a session that has already ended must still land politely at `/login`** — the
existing `AppError::Unauthorized` path does exactly that, so the requirement
here is that nobody adds an error page for it.

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
**Settled by the owner 2026-11-26, in the same review that amended `§4`**:
**an RFC is required** — a register entry may not govern a series. The gate's
purpose is that the decision is approved *before* an implementer starts, and
criterion 7 is the owner's only other gate; a register entry is written by the
same hand that does the work, so admitting one would leave the architect his
own authority before the fact. **Three consequences, none of them this
series':** `ROADMAP.md`'s column header becomes **Governing RFC** (that header
was the whole surface of the disagreement), the criterion gains the sentence
*an RFC may be as short as its decision* — nothing requires the length of this
one — and **the criterion becomes machine-checked** rather than remembered,
floored at this series so that `063`–`068` stay on the record rather than being
papered over. The check is scheduled work, not this round's.

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
- **Reusing the token's `iat` against a stored instant.** Recommended by this
  document when it was filed and **withdrawn before acceptance**: `§4.4`, with
  the measurement in `§4.1`.
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
