# `SES-001` — The session that cannot be ended

**Series** [069](./README.md) · **Release `0.71.0`** · **Governing**
[`RFC 024`](../../proposed/024-the-session-that-can-be-ended.md)

> **Do not start until `RFC 024` is in `rfcs/accepted/`.** See the series
> README's entry-criterion note.

**One round.** Make `FR-AUTH-003`'s own sentence true, and give the `FR-AUTH`
family acceptance evidence that matches its words.

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

> **A credential is valid only if it was issued after the subject's sessions
> were last ended.**

The mechanism `RFC 024` `§4` settles:

1. **One column on `users`** — a *sessions-valid-from* instant. Name it so a
   reader of the schema knows what it does without the RFC.
2. **`logout` writes it** to now, and still clears the cookie (a client
   should not keep a dead credential; it is simply no longer the mechanism).
3. **`jwt::verify` surrenders the token's `iat`** — already in `Claims`,
   already set by `issue` (`jwt.rs:65`, `:71`). Return **a value, not the
   struct**: `DEC-058`'s shape is that nothing outside `jwt` can name
   `Claims`, and that must survive this change, plant included.
4. **The check is on the request path**, in both extractors. A rule enforced
   only where state changes is a rule a replayed credential walks past.

**The migration is append-only.** Migrations that have shipped are not
edited — `0.53.0` shipped installable artefacts, so upgrade breakage is a real
cost. Add `0023_*`; do not touch an existing file. Both backends: the column
must exist for SQLite **and** PostgreSQL, and the postgres CI job runs the
whole `--lib` suite unfiltered, so a backend-specific type mistake will show
there rather than here.

## §3 — The ten handlers that gain a read, and the comment that must change

`AuthUser` (`extractors.rs:60`) **already** reads the user row every request,
for this exact reason, stated in its own comment. `Requester` deliberately
does not. Census by handler signature (architect's, 2026-11-26 — re-derive
it):

| extractor in the signature | handlers |
|---|---|
| `AuthUser` only | 33 |
| both | 29 |
| `Requester` only | **10** |
| neither (public) | 10 |

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

## §5 — What the product says

`RFC 024` `§5`: the semantic is user-visible, so the product must say it.
**Pressing Log out ends that user's sessions everywhere.** Propose the exact
copy; it goes in the i18n catalogue like every other string, never in a
comment. The architect reviews the wording — *"APIs and UI/UX for users not to
be confused or misunderstand are also very important."*

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
   backends covered.
3. The new `FR-AUTH-003` test **seen red** against the old behaviour, then
   green.
4. `FR-AUTH-001`'s second clause asserted by a real test, with its plant
   either done or its absence explained.
5. `Requester`'s doc comment corrected.
6. `§10.66` and `§10.36` instance 10 written; `FR-AUTH-003`'s acceptance
   field updated; Appendix A's arithmetic re-derived if any status moved.
7. The copy in `§5` proposed, in the catalogue.
8. `DEC-007`'s 42 commands run, **with the new count stated, measured by the
   runner**; `fmt`, both `clippy` variants and the release build clean.
9. **No published artefact carries local information** — a named exit
   condition, not a habit.
10. The report names the file it wrote.
