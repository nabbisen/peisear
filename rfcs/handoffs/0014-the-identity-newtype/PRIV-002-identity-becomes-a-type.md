# PRIV-002 — identity becomes a type

**Issued by**: Architect
**Date**: 2026-10-07
**Target release**: 0.43.0
**Governing RFC**: [0014](../../accepted/014-the-identity-newtype.md), accepted
2026-10-07, **`DEC-058`**. **Read it first** — what this is and is not is there
and is not repeated here.
**Related requirement**: `NFR-PRIV-005`.
**Depends on**: nothing. **After `REQ-003`**, and **it should not share a
release with anything user-visible** — a diff touching every handler signature
is one a reviewer must be able to read as exactly that.

**This fixes no defect.** The boundary holds, with two independent barriers.
What changes is that it cannot be broken by inattention.

---

## 1. §7's first escalation, and do this before anything else

**Establish whether Rust can actually enforce "only an authenticated session
can construct it", and report what you find before writing the conversion.**

A private field plus a `From<&Session>` keeps handlers out. **Tests are the
hole**: `REQ-002` found storage called directly from dozens of call sites
across nine test files, and those have no session.

Options, and I do not know which survives contact:

- **A `#[cfg(test)]` constructor** — works inside the crate, and
  `peisear-web`'s integration tests are a different crate, so it does not
  reach them.
- **A feature-gated `test-util` constructor** — reaches them, and ships a
  public construction path in a published crate unless the feature is
  genuinely off by default and nothing in the dependency graph turns it on.
- **One documented constructor with a single asserted caller** — a scan over
  the workspace requiring `RequesterId::from_session` (or whatever it is
  called) to appear in exactly one place: the session extractor. **Weaker than
  the type system and stronger than today's name-based pair**, and honest
  about being a guard rather than a proof.
- **Integration tests go through HTTP**, where a session exists, and the
  direct-call sites change. **Count them before proposing this** — it may be
  the largest part of the work rather than a detail.

**Bring the options with what each costs.** If the answer is the third, say so
plainly: `DEC-058`'s wording says *only an authenticated session can
construct it*, and a guard that asserts one caller is not that. **I would
rather amend the decision than have it read as stronger than what shipped** —
which is `§10.28`'s shape and this project's third-commonest defect.

## 2. Then the change

- **The type in `peisear-auth`**, beside the session, per `DEC-058`.
- **One type, named for the requester.**
- **Personal-data storage takes it** — the 36 of 40 `NFR-PRIV-005` counts, and
  their callers.
- **Out of scope, per the RFC**: the three authentication-path functions in
  `users.rs`, the job-side aggregate, and every resource id. **Identity only.**
- **`api_users.rs`'s two assertions become unnecessary** — the property they
  approximate is held by construction. **Remove them and say why in the
  commit**, rather than leaving a guard that now guards nothing; `§10.17` is
  the entry about assertions that pass for a reason other than the one they
  name.

## 3. The published API

`peisear-auth` gains a public type and `peisear-storage`'s signatures change.
**Breaking, under 0.x, carried by the minor bump** — and **it goes in the
changelog explicitly**, the way `Issue.position`'s removal did, not left for a
downstream build to discover.

**Report every public item whose signature changed**, by crate. That list is
the changelog's raw material and I would rather have it from you than derive
it from a diff.

## 4. Verification

- **§1's finding, before the conversion.**
- **The compiler is the test for the main property**: a handler passing a
  path-derived string to personal-data storage must **not compile**. **Plant
  it** — write that line, confirm it fails to build, remove it. A type-level
  guarantee that has never been seen to refuse anything is not yet evidence.
- **Every existing authorisation test passes unmodified** — `auth_boundary`,
  `optimistic_lock`, and the three `/api/users/{user_id}/*` endpoints'
  cross-user refusals. **If any needed changing, say which and why**; this
  change should alter no behaviour at all.
- **`require_self` still refuses**, and the path value is still validated and
  discarded.
- `DEC-007`: report the count, last recorded **369**. **A net drop is
  expected** if `api_users.rs`'s two assertions go — say so rather than
  letting the number look like a regression.
- `fmt`, `clippy`, three consecutive workspace runs. The overflow gate is
  untouched — no markup, no copy.

## 5. Escalate rather than deciding

- **§1, before writing anything.**
- **If the direct-call test sites are a larger change than the production
  code.** That is a real possibility and it changes how this should be
  sequenced.
- **If a storage function turns out to need both a requester and a subject**
  — `DEC-058` chose one type on the grounds that they are the same value
  everywhere today. A counter-example reopens that.
- **If any behaviour changes.** Nothing should.

## 6. Exit condition

Personal-data storage accepts only the identity type; a handler passing a
path-derived string does not compile, demonstrated by a plant; every
authorisation test passes unmodified; the public-API changes listed by crate;
and `§1`'s construction question answered on the record rather than assumed.
