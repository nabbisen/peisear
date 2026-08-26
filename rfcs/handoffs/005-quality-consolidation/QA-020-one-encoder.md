# QA-020 — One encoder, every redirect through it

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P2 — nothing is broken today, and the reason it is not is a
property of the copy that nothing enforces
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §11
**Depends on**: nothing. **The last section of Phase E.**

---

## 1. Read RFC 005 §11 first — it was reconciled today

The section names three raw-interpolation sites and two copies of an encoder.
Both counts are right. **It does not mention the largest group at all:**

| Strategy | Sites |
|---|---|
| `percent_encode_query` — two identical copies | 6 call sites |
| **`.replace(' ', "+")`, hand-rolled** | **23**, across 7 files |
| Raw interpolation | 3 |

**Reproduce all three counts** before changing anything. If any differs, stop.

## 2. Why this is P2 and not P1

**`.replace(' ', "+")` is correct for every string it currently encodes.** I
checked all twenty flash keys: plain ASCII words. None contains `&`, `=`, `#`,
`%`, `+`, `?`, or a non-ASCII byte.

**That is a property of the copy, not of the code.** `find_violations`
constrains tone, not character set, and this project's copy uses `—` freely
elsewhere. A flash message reading *"Sprint started & backlog updated"*
truncates its parameter; one containing an em dash puts a raw multi-byte
sequence in a `Location` header.

Neither crashes — axum returns a 500 rather than panicking, and `HeaderValue`
accepts those bytes. **The failure mode is a message silently arriving wrong.**

**Confirm my check rather than trusting it.** `MarkedAsReadFlash` is not a
simple literal — it takes a parameter — and I did not resolve what that
parameter can contain. If it can carry anything but a number, say so; that
would move this from latent to live and change the priority.

## 3. What to build

**One `percent_encode_query`, in one place.** The two copies are identical;
pick a home — `handlers/mod.rs`, or a small module beside it — and say why.

**Every one of the 29 sites through it.** Including the 23 `.replace` sites
and the 3 raw ones. `plan_query_string` and `change_status_form_list` are the
raw ones §11 names.

**Check the two copies are actually identical before collapsing them.** They
were written separately. If they differ in any character's treatment, that
difference is a finding and I want it before either is deleted.

## 4. The guard, and what it should assert

A scan, same family as the other six.

**Assert `.replace(' ', "+")` appears nowhere** under
`crates/peisear-web/src/`. After §3 there should be none, and it is the exact
phrase that reintroduces the class.

**Do not try to assert "every redirect is encoded"** — that needs to know which
`Redirect::to` arguments carry user-or-copy-derived text and which are static
paths, which a text scan cannot tell. 48 sites, most of them static. **Say so
in the module doc**: the guard forbids the known-wrong idiom rather than
proving the right one, and that limit is the honest boundary.

## 5. What I am not asking for, and why

**No change to the flash mechanism itself.** Messages travel in a query
parameter; that is the existing design and this handoff is about encoding, not
transport.

**No new copy, no message-key changes.**

**No attempt to make the encoder handle non-ASCII "better" than percent
encoding.** Percent encoding is the answer; the point is that one function does
it everywhere.

## 6. Escalate rather than deciding

- If §1's counts do not reproduce, stop.
- **If the two `percent_encode_query` copies differ**, stop and report the
  difference before deleting either.
- **If `MarkedAsReadFlash`'s parameter can carry non-numeric text**, stop —
  §2's "latent" framing would be wrong.
- If any of the 23 `.replace` sites turns out to encode something that is *not*
  a flash message — a path segment, an id — report it separately. That is a
  different sink with different rules.

## 7. Acceptance

1. §1's three counts reproduced.
2. The two copies compared before collapsing; any difference reported.
3. One encoder; all 29 sites through it.
4. §2's `MarkedAsReadFlash` question answered.
5. The §4 guard present, running in CI, planted, with its limit in the doc
   comment.
6. A test that a flash message containing `&` and a non-ASCII character
   survives the redirect intact — **the case that is currently impossible only
   because no such message exists**.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. §6's comparison of the two copies as prose. The plant
transcript.
