# PRIV-003 — the subject with no requester

**Issued by**: Architect
**Date**: 2026-10-08
**Target release**: 0.43.0
**Governing RFC**: [0014](../../accepted/014-the-identity-newtype.md),
**`DEC-058`** — **decision 2's own trigger has fired**, see §1.
**Source**: found by the dev team while implementing `PRIV-002`; ruled in
`.git-exclude/reviewed/PRIV-002-conversion-review.md` §1.
**Depends on**: `PRIV-002`, landed.

---

## 1. Why this is not a new decision

`DEC-058` decision 2: *"**One**, named for the requester. `SubjectId` and
`RequesterId` are the same value at every current call site; distinguishing
them would catch an administrator-acting-on-another case that does not
exist — **one, until a second is needed**."*

**The premise is falsified and the condition is met.** The snapshot job has a
**subject with no requester**: it iterates users on a schedule, so there is no
request and nothing to seal a `RequesterId` from. `PRIV-002` therefore left
**11 of 36** functions on `&str`.

**What that leaves, measured rather than taken from the report**: three of the
six personal-data modules — `personal_metrics`, `user_burnout`,
`user_metrics_snapshots` — contain **no `RequesterId` at all**, because every
function in them is job-reachable. Three of those functions are also reachable
from `/me`'s handler.

**And one of them lost a guard without gaining one.** `untrusted_id_scan`
covered `api_users.rs`, whose endpoints reach `user_burnout` and
`personal_metrics` — both in the untyped eleven. `require_self` still stands
in front of them and `auth_boundary`'s 17 tests still pass, **so nothing is
open** — but for those two the weak guarantee was traded for none rather than
for a strong one. **This handoff is where that closes.**

## 2. The shape

**A second sealed type, `SubjectId`** — *whose data this is*, as distinct from
*who is asking*.

**Two ways to obtain one, and no others:**

- **`From<&RequesterId>`** — a requester is always the subject of their own
  data. This is the handler path and it is total: every handler that has a
  `RequesterId` can get a `SubjectId` for free.
- **Minted by storage's own user enumeration.** The job's subjects come from
  `user_metrics_snapshots::users_with_active_assignments`; **that function
  returns `Vec<SubjectId>` instead of strings.** The job then hands them back
  to storage.

**Seal it the way `RequesterId` is sealed** — a private field in the module
that declares it, so nothing outside can write one. **Declare it in
`peisear-storage`**, not `peisear-auth`: its trusted origin is a database
enumeration, not a token, and putting it beside `RequesterId` would need
`peisear-auth` to know about rows. **If that placement does not work, say
why before building** — it is the one structural choice here and I have not
tried it.

**Then the eleven convert to `&SubjectId`**, and `&str` identity disappears
from personal-data storage entirely.

## 3. What this must not become

- **Not a public `SubjectId::new(&str)`.** That is the `String` again, and it
  would undo `PRIV-002` as well — a handler could mint a `SubjectId` from a
  path parameter and reach the converted functions through `From`'s inverse if
  one existed. **There is no inverse.**
- **Not a conversion of resource ids.** Project, team and issue ids stay as
  they are; RFC 0014's scope is identity.
- **Not a second extractor.** Handlers already have `RequesterId`; they use
  `From`.
- **`notifications::insert` and the dispatch-loop functions** are job-side and
  take `SubjectId` like the rest — but **check whether `peisear-notify` can
  obtain one** without a new constructor. If it cannot, that is the escalation
  and it may mean its subjects should come from the same enumeration.

## 4. Verification

- **The compiler is the test, and plant it twice**: a path-derived `String`
  must not compile against a converted function, **and** a `RequesterId` must
  convert cleanly where a handler needs one. Report both.
- **No public constructor exists**: show that a downstream crate cannot build a
  `SubjectId` by struct literal (`E0423`) or by any public function.
- **Every authorisation test passes unmodified in behaviour** — `auth_boundary`
  17, `optimistic_lock`'s cross-user three, and `/me`'s own.
- **The snapshot job still runs**: its integration test, and confirm the
  enumeration's new return type did not change which users it visits.
- **`&str` identity is gone from the six modules** — state it as a grep result.
- `DEC-007`: report the count, last recorded **367**. `fmt`, `clippy`, three
  consecutive workspace runs. The overflow gate is untouched.

## 5. Escalate rather than deciding

- **If `SubjectId` cannot live in `peisear-storage`** (§2).
- **If `peisear-notify` cannot obtain one** (§3).
- **If any function turns out to need a `RequesterId` *and* a `SubjectId`
  simultaneously** — that would be the administrator-acting-on-another case
  `DEC-058` said does not exist, and it would be a finding.
- **If any behaviour changes.** Nothing should.

## 6. Exit condition

No personal-data storage function takes an identity as `&str`; `SubjectId` is
obtainable only from a `RequesterId` or from storage's own enumeration, neither
of which a handler can forge; both plants reported; the snapshot job unchanged
in what it visits.
