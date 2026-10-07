# PRIV-004 — the identity type lives in a metrics module

**Issued by**: Architect
**Date**: 2026-10-08
**Target release**: 0.43.0 — **before the candidate**, see §1.
**Governing RFC**: [0014](../../accepted/014-the-identity-newtype.md).
**Source**: noticed on review of `PRIV-003`; ruled in
`.git-exclude/reviewed/PRIV-003-review.md` §4.
**Depends on**: `PRIV-003`, landed. **Small, and placement only — no logic.**

---

## 1. Why now and not after the release

`SubjectId`'s public path is
**`peisear_storage::user_metrics_snapshots::SubjectId`**, and **ten files
across three crates** import the identity type from a module named for metrics
snapshots.

The placement is **right for the seal** — the mint must live in the declaring
module, and the mint is `users_with_active_assignments` — and **wrong for the
name.**

**The path is a published API as of 0.43.0.** Moving it afterwards is a second
breaking change for a cosmetic reason; moving it now is free. **That is the
whole argument for doing this before the candidate rather than in 0.44.0.**

**This is my omission, not a defect in `PRIV-003`.** Its §2 told you to place
the type in `peisear-storage` and to report if that failed; it worked first
try. Which sub-module was mine to think about.

## 2. What to do

**A new module in `peisear-storage` named for what it holds** — `subjects` is
my suggestion and the name is yours if you prefer another; say which you chose.

Move into it, unchanged:

- `SubjectId` and its private field,
- `impl From<&RequesterId> for SubjectId`,
- **`users_with_active_assignments`** — it must move too, or the mint leaves
  the declaring module and the seal weakens from module-private to
  crate-private.

**Nothing else moves.** `insert` and `recent_for_user` stay in
`user_metrics_snapshots`; they consume a `SubjectId` and do not mint one.

**Do not add a `pub(crate)` constructor** as a way to keep the enumeration
where it is. That would let any module in `peisear-storage` mint an identity,
which is weaker than `RequesterId`'s module seal and weaker than what shipped
in `PRIV-003`. **If moving the enumeration turns out to break something, stop
and report it** rather than trading the seal for the placement.

## 3. Verification

- **The seal is unchanged in strength**: re-run `PRIV-003`'s forge plant from
  `peisear-web` — a struct literal must still fail with `E0423` — and
  `SubjectId::from(&rid)` must still round-trip. **Both, because this is the
  change most likely to weaken a seal by accident.**
- **`users_with_active_assignments`' SQL byte-identical again** — `git diff`
  on the query text, as `PRIV-003` did.
- **No behaviour changes**: `auth_boundary` 17, `optimistic_lock` 20, `smoke`
  12, `dispatch_integration` 3, all unmodified.
- **The public API change, stated**: the old path and the new one, so the
  changelog can say *moved* rather than *added*.
- `DEC-007` **367**, expected unchanged. `fmt`, `clippy`, three consecutive
  workspace runs. The overflow gate is untouched.

## 4. Escalate rather than deciding

- **If moving the enumeration breaks something** (§2) — report; do not weaken
  the seal to avoid it.
- **If a better module name presents itself**, use it and say so; `subjects`
  is a suggestion.
- **If anything other than placement has to change.** Nothing should.

## 5. Exit condition

The identity type's public path names identity rather than metrics; the mint
is in the same module as the type; the forge plant still fails and the
conversion still round-trips; no behaviour and no SQL changed.
