# REQ-002 — what `REQ-001` left behind

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.41.0
**Governing RFC**: none. **Depends on**: nothing. **After `A11Y-001`.**

**Three unrelated residues of the audit, none large.** Unlike `REQ-001` this
one **writes code** — four tests and possibly one deletion.

---

## 1. Four requirements are Met with nothing pinning the limb they name

`REQ-001` moved these to **Met** on the strength of a probe, and said so. A
probe is not a test, and the record now claims more than the suite holds:

| requirement | the limb nothing pins |
|---|---|
| `FR-PLAN-005` | a **personal** project's issue is excluded from the team backlog |
| `FR-NTF-005` | *mark all read* is **absent when nothing is unread** |
| `FR-CAL-005` | an unscheduled day carries **no comment and no fill** |
| `FR-TEAM-005` | the privacy footnote renders **on the team detail screen** (the string is byte-pinned in `peisear-i18n`; the screen is not) |

**Write one test each**, in the file where its neighbours live — no new test
file is expected. **Each must be seen to fail**: invert the behaviour by file
copy, confirm the test fails and nothing else does, restore byte-identical.
`FR-CAL-005`'s is the awkward one — asserting an **absence** — so say what
string or element you asserted absent and why that is the right one.

## 2. `project_trend_decline` — plumbing for a notification that cannot occur

A constant, a `NotificationKindLabel`, an `en.rs` string and a branch in
`components.rs`. **No emitter anywhere.** Found while establishing why the
inbox cannot be populated (`GATE-004`).

**Establish first whether a requirement expects it.** `FR-NTF-001`'s
edge-triggered dispatch names kinds; if one of them is this, it is an
**unimplemented requirement recorded as Implemented** — the dangerous
direction, and it stops being a cleanup. **Report that and stop.**

**If nothing expects it**, it is dead plumbing: say what removing it touches
and **do not remove it in this handoff**. A message key deletion crosses the
i18n guards and a removed enum variant is a public-API change in
`peisear-core`; both want their own change with its own before-and-after.

## 3. `NFR-PRIV-005` — a status that took the stronger reading silently

It reads *Not implemented — verification exists at the handler layer only*,
while `user_capacities::find(pool, user_id, id)` and
`projects::update(…, owner_id, …)` **do** scope by the caller's identity. The
requirement is a **SHOULD**: *storage functions should additionally accept and
verify the caller's identity.*

**Count it.** Of the storage functions that read or write a user-owned row, how
many take the caller's identity and scope by it, and how many rely on the
handler having checked? **A ratio and the two lists** is what turns *Not
implemented* into something a reader can act on. That is all — **no
refactoring**, and no opinion on whether the remainder should change.

## 4. Verification

- The four tests, each seen to fail, `DEC-007` reported (**360** before).
- §2 and §3 are reports; say which parts are measured and which read.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched.

## 5. Escalate rather than deciding

- **§2 turning out to be an unimplemented requirement** (§2).
- **A fifth requirement Met on a probe alone** that `REQ-001` and I both
  missed — the list in §1 is from that audit, not from a fresh sweep.
- **If `FR-CAL-005`'s absence cannot be asserted** without pinning a string the
  product might reasonably change.

## 6. Exit condition

Four limbs pinned by tests that were seen to fail; a statement of whether
anything expects `project_trend_decline`; and a count, with lists, of storage
functions that verify the caller against those that do not.
