# `SES-001` — The session that cannot be ended

**Series** [069](./README.md) · **Release `0.71.0`** · **Governing**
[`RFC 024`](../../accepted/024-the-session-that-can-be-ended.md)

> **Cleared to start: `RFC 024` was accepted by the owner 2026-11-26** and is
> in `rfcs/accepted/`, which is release-cycle entry criterion 1. See the
> series README.

**One round.** Make `FR-AUTH-003`'s own sentence true, and give the `FR-AUTH`
family acceptance evidence that matches its words.

> **Amended 2026-11-26, before the round opened.** The owner asked for
> `RFC 024` to be reviewed against his fundamental philosophy and **the
> mechanism changed**: a session **generation counter**, not a timestamp
> compared against the token's `iat`. `§2`, `§3`, `§4`, `§5` and `§7` below
> are the amended instructions; `RFC 024` `§4.1` and `§4.4` carry the
> measurement and the withdrawn alternative. **Nothing here was ever
> started**, so there is no work to redo.

---

## §1 — Step one: reproduce, do not trust

Before any change, reproduce the defect and record what you saw. `RFC 024`
`§1` has the architect's figures; **they are claims, and re-deriving them is
the first deliverable**.

A local instance is enough (`DATABASE_URL` to a scratch SQLite file,
`PEISEAR_SETUP=never`, `COOKIE_SECURE=false`, a `JWT_SECRET` you generate).
Capture the session cookie at register, confirm a protected route with it,
`POST /logout` carrying it, then **replay the same token**. Report the status
codes you observed, for at least one HTML route and one `/api/*` route.

**If it does not reproduce, stop and say so in a review-request file.** That
outcome is more important than the repair.

## §2 — The rule to implement

> **A credential is valid only if it was issued under the subject's current
> session generation. Ending a session advances the generation.**

The mechanism `RFC 024` `§4` settles. **Read `§4.1` before you reach for a
timestamp** — it is the one part of this design that was already got wrong
once, in this repository, by this project's own architect.

1. **One integer column on `users`** — a **session generation**,
   `INTEGER NOT NULL DEFAULT 0`. Name it so a reader of the schema knows what
   it does without the RFC (`sessions_epoch` in the RFC's prose; the name is
   yours to improve, not to shorten into something cryptic).
2. **`logout` advances it** — `sessions_epoch + 1` for that user — and still
   clears the cookie (a client should not keep a dead credential; it is
   simply no longer the mechanism). `logout` therefore needs the requester,
   which it does not take today (`handlers/auth.rs:121` takes only the jar).
3. **`Claims` carries the generation the token was issued under**, and
   `verify` **surrenders that value — a value, not the struct**: `DEC-058`'s
   shape is that nothing outside `jwt` can name `Claims`, and that must
   survive this change, plant included. `issue` reads it from the user row
   that `login_submit` (`auth.rs:90`) and `register_submit` already have, so
   **no new read on either path**.
   **Read it with `#[serde(default)]`.** A token minted by `0.70.0` has no
   such claim; without the default, upgrading logs every user out at once.
4. **The check goes in `verified_requester_id` (`extractors.rs:22`) and
   nowhere else** — not "in both extractors", which is what this handoff said
   before the amendment and would have been the defect rebuilt inside the
   fix. That function is already *"the only way either extractor (or anything
   else in the workspace) can obtain one"*, and all five extractors funnel
   through it: `AuthUser` and `Requester` directly, `ApiRequester` via
   `Requester`, `ApiAuthUser` and `MaybeAuthUser` via `AuthUser`. One site,
   and a sixth extractor written next year cannot go around it.
   **Both rejection shapes come for free**: `AppError::Unauthorized` is
   already a `303` to `/login` on HTML routes and a JSON `401` through
   `ApiAuthUser`/`ApiRequester`. Do not write a new rejection path.

**The migration is append-only.** Migrations that have shipped are not
edited — `0.53.0` shipped installable artefacts, so upgrade breakage is a real
cost. Add `0023_*`; do not touch an existing file. Both backends: the column
must exist for SQLite **and** PostgreSQL, and the postgres CI job runs the
whole `--lib` suite unfiltered, so a backend-specific type mistake will show
there rather than here. **An integer is the same integer in both** — which is
half the reason the mechanism changed.

## §3 — The reads, the ten handlers, and the comment that must change

`AuthUser` (`extractors.rs:60`) **already** reads the user row every request,
for this exact reason, stated in its own comment. `Requester` deliberately
does not.

**One read per request, wherever the check sits — and the arithmetic changed
with the amendment.** The check is in `verified_requester_id` (`§2.4`), so it
reads the row itself; **`AuthUser` must then stop doing its own second
`users::find_by_id` and use the row the choke point already fetched.** The 29
handlers that extract *both* run `verified_requester_id` **twice** per request
(they already do, decoding the JWT twice, which `Requester`'s comment calls
cheap because it is I/O-free) — so **memoise the row in `parts.extensions`**,
which also removes that double decode. Census by handler signature
(architect's, 2026-11-26 — re-derive it):

| extractor in the signature | handlers | reads/request, today → after |
|---|---|---|
| `AuthUser` only | 33 | 1 → **1** |
| both | 29 | 1 → **1** |
| `Requester` only | **10** | 0 → **1** |
| neither (public) | 10 | 0 → 0 |

**If your implementation makes any row of that table worse, say so in the
report rather than letting the measurement rot** — the earlier reading of
this cost (*"62 of 72 already make the read"*) was true only of a design that
put the check inside `AuthUser`, and it is corrected in `RFC 024` `§4.3`.

The ten: `notification_preferences::{save_preferences,silence_all}`,
`notifications::{resume,email_opt_in,mark_read,mark_all_read}`,
`settings::{insert_capacity,update_capacity,delete_capacity,close_capacity}`.
**Every one mutates the requester's own personal data** — precisely where a
revoked session must not be honoured.

**`Requester`'s doc comment says *"no DB round-trip, unlike `AuthUser`"*. That
stops being true. Correct the comment in the same change.** Leaving a doc
comment asserting a property the code no longer has is the failure class this
whole series is about.

## §4 — The acceptance evidence, which is the point of the round

**`FR-AUTH-003`.** Replace the citation. The new test must **replay a captured
credential after logout** and assert it is refused — not drive a cookie jar
and observe that the jar is empty. Cover an HTML route and an `/api/*` route,
since those take different rejection paths (`303` vs `401`).

**Plant it.** Write the test, watch it **fail** against the current behaviour,
then make it pass. A guard that has never been seen red has unknown reach.

**Three more tests the mechanism itself needs**, each cheap and each covering
something a reader would otherwise have to trust:

- **Log out, then log straight back in** — in the same test, with no sleep —
  and assert the new session works. This is the case a timestamp mechanism
  fails (`RFC 024` `§4.1`); with a counter it must simply pass, and the test
  is what keeps anyone from reintroducing a clock later.
- **A token minted before the column existed still verifies**: build one with
  no generation claim and assert it is accepted while the row reads the
  default. This is the `#[serde(default)]` property in `§2.3`, and without a
  test it is one attribute away from a silent mass logout at upgrade.
- **The other device, not just the other request**: two independent clients
  for the same account, one logs out, assert the *second* is refused. The
  one-sentence statement of this whole round.

**`FR-AUTH-001`, one test, not a second subject.** Its acceptance has two
clauses and **nothing in the repository names the second** — *"no plaintext
password is persisted or logged"*, zero matches for `plaintext` across
`tests/` and `peisear-auth`. The architect measured it and **the product is
clean**: a marker password appears 0 times in the SQLite file, 0 times in a
debug log and 0 times anywhere under the data directory; the ≥ 8-character
rule holds (7 → `400`, 8 → `303`). **So this is a test for a property that is
already true.** Make it a real one: a distinctive password, a registration,
then assert its absence from the database **and** from captured log output —
and plant it by checking that it fails if the hash step is bypassed, or say in
the report why that plant is not reachable.

## §5 — What the product says, in two places

`RFC 024` `§5`: the semantic is user-visible, so the product must say it.
Propose the exact copy for **both** sites; it goes in the i18n catalogue like
every other string, never in a comment. The architect reviews the wording —
*"APIs and UI/UX for users not to be confused or misunderstand are also very
important."*

1. **Where the action is taken**: pressing Log out ends that user's sessions
   everywhere.
2. **Where the consequence lands** — the other device, which today gets a
   bare `303` to `/login` with nothing to distinguish a revocation from an
   expiry or from never having signed in. Use the mechanism that already
   exists: `login_page` takes `?flash=` (`auth.rs:26`) and renders it, and
   dynamic text goes through `percent_encode_query`, which `one_encoder_scan`
   exists to keep. **The flash carries copy and nothing about the user** — no
   identifier, no email; a URL must not carry personal data.

**And pressing Log out when the session has already ended must still land
politely at `/login`**, which the existing `AppError::Unauthorized` path does
for free. The requirement is that nobody adds an error page for it.

## §6 — Register entries

- **`§10.66` opens**: the gap, measured, with `FR-AUTH-003` recorded
  `Divergent` until the fix lands (exit criterion 3 requires the entry for
  exactly that) and closed in the same release.
- **`§10.36` gains instance 10**: the cause, not the defect — the acceptance
  check narrower than its requirement's words. Instance 9 is two days old and
  names this half of the class; this is its fourth instance and the first with
  a P0 behind it.
- **No new `FR`/`NFR` number.** `FR-AUTH-003`'s acceptance widens.

## §7 — Exit conditions

1. The defect reproduced and recorded **before** the repair, with observed
   status codes.
2. The rule in `§2` implemented, with the migration append-only and both
   backends covered, **the check in one place** (`§2.4`) and the reads per
   request no worse than `§3`'s table.
3. The new `FR-AUTH-003` test **seen red** against the old behaviour, then
   green.
4. `FR-AUTH-001`'s second clause asserted by a real test, with its plant
   either done or its absence explained.
5. `Requester`'s doc comment corrected.
6. `§10.66` and `§10.36` instance 10 written; `FR-AUTH-003`'s acceptance
   field updated; Appendix A's arithmetic re-derived if any status moved.
7. **The three mechanism tests in `§4`** — immediate re-login, a
   pre-upgrade token, the second device — green, and the first of them
   reported as the reason a clock is not being compared.
8. The copy in `§5` proposed **for both sites**, in the catalogue.
9. `DEC-007`'s 42 commands run, **with the new count stated, measured by the
   runner**; `fmt`, both `clippy` variants and the release build clean.
10. **No published artefact carries local information** — a named exit
   condition, not a habit.
11. The report names the file it wrote.
