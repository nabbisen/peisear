# NTF-001 — a preference row for a notification that cannot fire

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.41.0
**Related requirements**: `FR-NTF-006` (the silence-all banner).
**Source**: `REQ-002` §2. **Depends on**: nothing.

---

## 1. This is a defect, not a cleanup

`peisear_core::notifications::kind::all_user_facing()` returns
**`[BURNOUT_OVERLOAD, BURNOUT_STALLED, PROJECT_TREND_DECLINE]`**. The third has
**no emitter** — no helper implements the trigger its own doc comment
describes, and `ROADMAP.md:444` lists it under deferred Phase 2 candidates.

Two consequences, both measured by `REQ-002`:

1. **`/settings/notifications` renders a row** — *Project health decline*, with
   In-app, Email, Webhook and minimum-severity controls — **for something that
   can never arrive.**
2. **`all_kinds_silenced` requires all three to be silenced**, and it gates
   `FR-NTF-006`'s pinned banner. **So a user must silence a phantom kind before
   the product will tell them everything is silenced.**

**The second is the defect.** A real user asking *have I turned everything
off?* gets the wrong answer until they act on a notification that does not
exist.

## 2. What to do — the narrow change

**Take `PROJECT_TREND_DECLINE` out of `all_user_facing()`, and nothing else.**

- The constant, the `NotificationKindLabel` variant, its English and fixture
  strings, the `components.rs` mapping and the inbox link **all stay.** They
  cost nothing, they are what the kind needs when someone builds it, and
  removing them crosses a published crate's public API and six i18n closed-set
  guards for no user-visible gain.
- **`REQ-002` listed the blast radius of a full removal.** That list is why
  this handoff is one function.

**What moves with it:**

- `every_notification_kind_except_global_appears_in_all_user_facing` — its
  expectation changes. **Read it first**: if it exists to catch a kind being
  forgotten, an exclusion needs a reason beside it, the shape
  `touch_target_scan`'s named exclusions use.
- `touch_target.rs:105` comments *"3 kinds"* and asserts a rendered count.
- `inbox_refinements.rs:202/215/232` iterate the kinds.
- `all_kinds_silenced` now needs two, which is the point.

## 3. Existing data

`0010_notifications.sql` records that orphan kinds *"display generically"*, and
installs may hold saved preference rows for this kind. **`REQ-002` did not run
that case; verify it before you finish**: a database with a stored
`project_trend_decline` preference row, opened by the new binary — the settings
page renders, nothing errors, and `all_kinds_silenced` answers on two kinds.
**No migration to delete the rows** — they are harmless and they are what the
kind needs if it is ever built.

## 4. Verification

- **The banner, before and after**: with both real kinds silenced and the
  phantom **not** silenced, `FR-NTF-006`'s banner is **absent before** and
  **present after**. That is the defect; assert it first.
- `/settings/notifications` renders two rows, named.
- The existing preference row case (§3).
- `banner_absent_by_default_present_after_silence_absent_after_resume` and the
  `inbox_refinements` kind loops pass, modified only where the count is.
- **The i18n guards pass** — nothing is removed from the message table, which
  is the reason this change is small.
- `DEC-007`: report the new count, last recorded **364**. `fmt`, `clippy`,
  three consecutive workspace runs. **The overflow gate** — the settings page
  loses a row, which it sweeps.

## 5. Escalate rather than deciding

- **If the guard test's exclusion cannot be given an honest reason** (§2).
- **If anything else iterates `all_user_facing()`** beyond what `REQ-002`
  listed.
- **If a stored preference row misbehaves** (§3). That would make this a
  migration question and it is mine.
- **If removing the row changes a user-visible string** other than its absence.

## 6. Exit condition

`/settings/notifications` offers preferences only for notifications that can
arrive; the silence-all banner appears when the two real kinds are silenced;
the kind's plumbing survives intact for whoever builds it.
