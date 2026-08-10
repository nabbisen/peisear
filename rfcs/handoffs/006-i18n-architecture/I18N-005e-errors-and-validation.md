# I18N-005e — Errors, validation, and auth

**Issued by**: Architect
**Date**: 2026-08-10
**Priority**: P1 — completes 0.21.0
**Depends on**: I18N-005a (pattern settled)
**Parallel with**: 005b, 005c, 005d

Pattern rules are in the queue README.

---

## 1. Scope

`src/error.rs` (~8), `components/auth.rs` (~11), and handler-level validation
messages across `handlers/*`.

Partly seeded already: I18N-001 put a handful of `AppError` and validation
messages in the table as its representative set. Finish the job.

## 2. Error copy is where §1.7 is hardest to satisfy

Every other group converts labels and headings. This one converts the sentences
users see when something has gone wrong — the exact place failure framing is
most natural to write and most prohibited.

§1.7 bans "Failed to…" and "Error:…". 0.20.0's own defect was
`"Failed to update status. Please refresh."`, and the guard's term list
generalises both to prefixes for that reason.

**Expect the guard to reject existing copy here.** When it does, that is a
finding under `FR-HLT-006`/`NFR-LANG-001` — report it with a proposed
replacement; do not reword unilaterally. Wording on a P0 vocabulary requirement
is an architect decision, as `ISSUE-006` established.

## 3. The authentication message is security-bearing

`FR-AUTH-002`: a failed login **must not disclose which field was wrong**. One
neutral message covers both cases.

Converting it must not split it into per-field keys, however tempting the
symmetry. If you find the current implementation already distinguishes them,
that is a **security finding** — escalate it rather than converting it
faithfully.

## 4. `AppError` versus `ApiAppError`

Two error surfaces (`DEC-008`): `AppError` renders HTML, `ApiAppError` renders
JSON.

`ApiAppError`'s JSON carries a stable machine-readable `error` code plus a
human `message` (`FR-API-004`, external design §8.4). **The code is not copy —
it is a contract**; clients branch on it. Only `message` goes through the
table.

DEV-001 fixed `AppError::public_message()`'s `Validation` arm leaking a
developer-facing prefix. Its sibling in `ApiAppError` was checked at the time
and found clean — the JSON path destructures the inner string rather than
routing through `Display`. Confirm that is still true; do not assume it from
this note.

## 5. Internal versus user-facing

`error.rs` contains both. `tracing::error!` text, `StorageError` variants and
`Display` impls used for logs are **not** copy and stay put.

The test: *does this string ever reach a rendered page or an API response?* If
only a log, leave it. Report anything ambiguous rather than guessing — a
converted log message is harmless, but a missed user-facing one is a hole in
the guard's coverage.

## 6. Watch for

- **Validation messages preserve user input** on failure (`GUI §9`, external
  design §5.3). Converting the message must not disturb the re-render.
- **The 409 conflict body** (`NFR-CONC-006`) carries entity type, id and
  timestamp alongside its message. Those are data, not copy.
- **The 403-not-404 rule** (`NFR-PRIV-006`, `FR-API-003`): refusals must not
  disclose existence. If two refusal paths render distinguishable messages
  after conversion, escalate.

## 7. Tests

Guard covers new entries; exhaustiveness holds; rendered output semantically
identical. `auth_boundary`, `optimistic_lock` and `smoke` exercise these paths.

Add an assertion that the login-failure message is identical for
unknown-account and wrong-password. It is a security property with no test
today, and this is the handoff that touches it.

## 8. Acceptance

1. No user-facing literal left in `error.rs`, `auth.rs`, or handler validation.
2. Internal-only strings untouched, and the distinction reported.
3. Login-failure indistinguishability asserted by test.
4. `ApiAppError`'s `error` code untouched; only `message` converted.
5. Guard passes; rendered output semantically identical.
6. fmt and clippy exit 0; suite counts unchanged.
7. Survey reported.

## 9. Prohibited

Do not reword guard-rejected copy unilaterally — report with a proposal. Do not
split the login message per field. Do not convert `error` codes. Do not convert
log-only strings.

## 10. Review focus to request

1. Every guard rejection, with your proposed replacement.
2. The internal/user-facing split — anything you found ambiguous.
3. Whether `ApiAppError`'s message path is still free of the `Display`
   fallthrough DEV-001 fixed on the HTML side.

---

## 11. On finishing 0.21.0

This is the last conversion group. When it lands, **every user-visible string in
the product goes through one table**, and `FR-HLT-006` and `NFR-LANG-001` — both
P0, both recorded for two releases as *"Implemented by convention; no automated
guard exists"* — become checkable for the first time.

Say so in the review request if it is true, and say what remains outside the
table if it is not. `search.js` is already known to be outside it (005d §5).
