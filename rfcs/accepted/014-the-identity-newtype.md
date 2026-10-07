# RFC 0014: The identity newtype — a convention held by a variable's name, made a type

**Status**: **Accepted** (2026-10-07) — implementation may begin
**Target**: 0.43.0 if accepted; the work is one substep and does not need
splitting
**Related spec sections**: `SPEC §11.5.4`, `SPEC Appendix E.2`
**Related requirements**: **`NFR-PRIV-005`** (Partial — 36 of 40),
`NFR-PRIV-006`, `NFR-PRIV-008`
**Related register entry**: **`§10.3`**, which argued this direction at
`QA-021` and left it unscheduled
**Governing decisions**: proposes `DEC-058`
**Last updated**: 2026-10-07 — written from `§10.3`'s own reconciliation and
`REQ-002`'s count, both of which measured the code rather than describing it

## Summary

Every identity in this codebase is a `String`. **Identity-ness lives in a
variable's name**, so a storage function cannot tell a session's user id from a
path parameter, and `for_user(pool, user_id)` returns whatever user it is
handed.

**This RFC proposes a newtype that only an authenticated session can
construct**, so that passing a caller-supplied id to personal-data storage
stops being a mistake someone has to avoid and becomes a thing that does not
compile.

**It is not a security fix. Read the next section before deciding.**

## Background — what is true today, measured twice

**The boundary holds.** `§10.3`'s reconciliation at `QA-021` found the 0.19.1
assessment *"wrong in the direction that matters"*:

- **36 of 40** personal-data storage functions take the subject's identity and
  scope on it (`REQ-002`'s independent recount: `notifications` 14/14,
  `user_capacities` 13/13, `view_states` 3/3, `personal_metrics` 2/2,
  `user_burnout` 1/1, `user_metrics_snapshots` 2/3, `users` 1/4 — the
  remainder being the authentication path, where no caller identity exists
  yet, and one job-side aggregate).
- **No handler passes a caller-supplied identity to personal-data storage.**
  The three `/api/users/{user_id}/*` endpoints call `require_self` and then
  pass the **session** identity; the path value is validated and discarded.

So there are **two independent barriers**, and `SPEC §11.5.4`'s outcome is
achieved by a different route than the requirement described.

**What is missing is a guarantee, not a behaviour.** *Scoping is not
verification.* Nothing stops a future handler from passing the wrong string,
and what guards against it today is **two assertions over one file**: that
every `Path(` extraction in `api_users.rs` binds the name `user_id`, and that
the name reaches nothing but `require_self`.

**That guard is the subject of this RFC.** It is name-based, it covers one
file of a seven-crate workspace, and it would not notice a new endpoint in a
new file doing the wrong thing.

## What the newtype buys

1. **Unconfusable.** A storage signature taking `SubjectId` cannot be handed a
   project id, an issue id, or a path parameter — at compile time, in every
   crate, forever.
2. **Findable structurally.** Today, auditing identity flow means reading
   variable names; `REQ-002` did exactly that and said so. With a type, the
   compiler and `grep` agree.
3. **One guard replaces a file-scoped pair.** The two `api_users.rs`
   assertions become unnecessary, because the property they approximate is
   held by construction.
4. **`NFR-PRIV-005`'s stronger reading becomes satisfiable.** It is *0 of 40
   and cannot be otherwise* with `&str`. That is the only requirement in this
   document whose status says a thing is impossible.

## What it costs

- **Every handler signature that carries an identity**, and the storage
  functions they call — on the order of the 40 in `NFR-PRIV-005`'s count plus
  their callers.
- **`peisear-core`'s or `peisear-auth`'s public API gains a type**, and
  `peisear-storage`'s signatures change. **Breaking for a published crate**,
  carried by a minor bump under 0.x, and it must be named in the changelog
  the way `Issue.position`'s removal was.
- **A construction discipline**: the type must be constructible *only* from an
  authenticated session, or it is a `String` with a longer name. That is the
  design's whole load-bearing element and §"Open questions" asks about it.
- **Churn in a release that fixes no user-visible defect.**

## The options

**A — nothing.** The boundary holds and is tested. Rejected as the default
only because the guard that keeps it holding is one file's worth of
name-matching, and `§10.3` has carried the direction unscheduled for nine
releases precisely because nobody costed it.

**B — widen the name-based guard** to every file that extracts a `Path`
identity. Cheaper, and it scales the wrong way: a list of names to maintain,
and `§10.28`'s shape — a guard whose description outruns its reach — is
already this project's third-commonest defect.

**C — the newtype.** Recommended. `§10.3` argued it and also argued what not
to do: *"a better answer than threading a requester parameter through thirty
storage functions, which is what this entry's original framing implied and
what nine releases of readers would have built."*

## Recommendation: C, and a warning about how it reads

**Take C.** It converts a convention into a property, which is the only one of
the three that stops the question recurring.

**But the changelog must not call it a security fix.** The boundary held
before and holds after; what changes is that it cannot be broken by
inattention. A release note implying a hole was closed would be false about
every release since 0.19.1 — the same error `§10.15`'s 0.37.0 entry had to
avoid, and the same one `A11Y-007` avoided by naming which reader it was
failing.

## Decisions taken 2026-10-07

**Accepted, with all three recommendations and none varied.** `DEC-058`:
*identity is a type that only an authenticated session can construct, and
personal-data storage accepts nothing else.*

1. ~~**Where does it live, what can construct it?**~~ — **`peisear-auth`,
   constructible only from the authenticated session.** A type in `core` that
   anything can build is the `String` again.
   **Corrected 2026-10-07 from `PRIV-002`'s investigation.** This decision, and
   the summary above it, described *"a private field plus a `From<&Session>`"*.
   **There is no `Session` type**, and what `jwt::verify` returns is `Claims`,
   **whose fields are all `pub` and must be for `serde::Deserialize`** — so a
   `From<&Claims>` would have added no guarantee whatever. The mechanism was
   specified against a type nobody had looked up.
   **What is achievable is stronger**: a struct's private field is private to
   its **module**, not its crate, so declaring the type inside
   `peisear_auth::jwt` with `verify` as the only function that can see the
   field makes *only a cryptographically-verified token produces one* a
   compiler-checked property. **Proven by plant** — a downstream crate's
   struct-literal attempt fails to compile (`E0423`), the legitimate path
   works, and a wrong-signature token yields nothing.
   **And `peisear-storage` does not depend on `peisear-auth`**; that edge is a
   first step, which this RFC did not say.
2. ~~**One type or two?**~~ — **One**, named for the requester. `SubjectId`
   and `RequesterId` are the same value at every current call site;
   distinguishing them would catch an administrator-acting-on-another case
   that does not exist.
3. ~~**Is the churn acceptable with no user-visible change?**~~ — **Yes.** The
   answer to *what does a user get* remains **nothing**, and the changelog
   must say so rather than implying a hole was closed.

## Open questions — as put, now settled above

1. **Where does the type live, and what can construct it?** `peisear-auth`
   beside the session, or `peisear-core` beside the domain types. The
   constructor must be unavailable to handlers — a private field with a
   `From<Session>`, or a sealed trait. **My recommendation: `peisear-auth`,
   constructible only from the authenticated session**, because a type in
   `core` that anything can build is the `String` again.
2. **One type or two?** `SubjectId` (whose data is being read) and
   `RequesterId` (who is asking) are the same value in every current call
   site, and distinguishing them would catch an administrator-acting-on-another
   case that does not exist yet. **My recommendation: one**, named for the
   requester, until a second is needed.
3. **Is the churn acceptable in a release with no user-visible change?**
   This is the question I would most like answered, because the honest answer
   to *"what does a user get"* is **nothing**.

## How tests obtain one — settled 2026-10-07, and there is no hole

`PRIV-002`'s investigation counted **40 direct storage call sites in tests
against 29 in production**, and proposed routing some through HTTP and
bounding the rest behind a scanned constructor.

**Neither is needed.** `jwt::issue` and `jwt::verify` are both `pub` and the
test harness fixes the secret, so a test mints and verifies its own token in
two lines and holds a **genuinely verified** identity — no HTTP, full timing
control for the atomicity tests, and **no constructor that bypasses
verification**.

**This is not a loophole.** The guarantee is *only a verified token produces
one*; a test that mints and verifies has satisfied it rather than side-stepped
it. **No feature flag, no test-only constructor, no scan.**

## Schedule

One substep, 0.43.0, after `REQ-003`. **It should not share a release with
anything user-visible**: a diff touching every handler signature is one a
reviewer must be able to read as exactly that.

## Out of scope

The three authentication-path functions in `users.rs`, which have no caller
identity. The job-side aggregate. Project, team and issue ids — this is about
identity, and widening it to all resource ids is a different and larger
proposal. `NFR-PRIV-006`'s concealment behaviour, which is independent and
already `Implemented`.

## References

- `§10.3`, the reconciliation at `QA-021` and the direction it recorded
- `NFR-PRIV-005`, and `REQ-002`'s three-tier recount
- `.git-exclude/reviewed/REQ-002-review.md` §3, on why *accept and scope* and
  *verify the requester* are different readings and only one is satisfiable
