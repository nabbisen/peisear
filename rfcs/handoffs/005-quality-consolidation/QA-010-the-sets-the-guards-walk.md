# QA-010 — The sets the guards walk

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P1
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §3, §14
**Depends on**: nothing. `QA-009` is closed.

---

## 1. Read this first: nothing here is broken today

`QA-009` found five variants **actually missing** from `MessageKey::all()`.
This handoff has no equivalent. I measured all three sets below and every one
of them is complete and correct as it stands.

That is why this is P1 and not P0, and it is the first thing your review
request should say back to me. **Do not go looking for a live defect here to
justify the work** — the work is a tripwire, not a repair, and a package that
implies otherwise misrepresents the tree.

The question all three items share is the one `QA-009` §2 opened and did not
finish: **every guard walks a set; what makes that set right, and what notices
when it stops being right?**

## 2. The fourteen label enums

`MessageKey::all()` is now provably complete over `MessageKey`'s 520 variants.
But 125 of those carry a label enum, and their entries are generated as
`X::all().into_iter().map(…)` — so the coverage of every message parameterised
by a label is exactly `X::all()`, and those are hand-maintained:

```rust
pub fn all() -> [IssueStatusLabel; 3] { [Open, InProgress, Done] }
```

**The array length is part of the type and moves with the literal**, so the
compiler has no opinion about whether the enum has a third variant. Planted:

```rust
- pub fn all() -> [IssueStatusLabel; 3] { [Open, InProgress, Done] }
+ pub fn all() -> [IssueStatusLabel; 2] { [Open, InProgress] }
```

`cargo test --workspace`: **202 passed, 0 failed.** With `Done` dropped,
`StatusChangedAnnouncement { status: Done }` renders *"Moved to Done."* and no
guard ever sees it.

**Reproduce that first.** Then, for all fourteen:

| | |
|---|---|
| `EntityKind` 6 · `Field` 17 · `IndicatorLabel` 6 · `NavSection` 3 | `IssueStatusLabel` 3 · `PriorityLabel` 4 · `SprintStatusLabel` 3 |
| `TeamRoleLabel` 3 · `DriftDirectionLabel` 3 · `HealthStateLabel` 3 | `NotificationKindLabel` 3 · `NotificationChannelLabel` 3 |
| `CalendarViewLabel` 3 · `TrendDirectionLabel` 2 | **all fourteen complete today — verified 2026-08-25** |

Assert, per enum: every declared variant is named in its `all()` body. Your
`enumeration_guard` already does this for one enum; fourteen more is a loop
over the same reading.

**Assert the declared length too** — `[X; N]` where `N` equals the variant
count. A guard that only checks membership passes on a list that names one
variant twice and omits another; the length catches that for free, and the
length is right there in the signature.

## 3. `peisear-core`'s kind and channel lists — the root of the chain

`components.rs`'s `every_declared_notification_kind_has_a_label` already guards
the i18n seam: a kind in `peisear-core` with no label fails a test. **That
guard iterates `kind::all_user_facing()`**, so its reach is that function's
completeness — the same shape one crate further out.

These are **`&str` constants, not enum variants**, so there is no type for the
compiler to be exhaustive over at all:

```rust
pub const GLOBAL: &str = "_global";
pub const BURNOUT_OVERLOAD: &str = "burnout_overload";
pub const BURNOUT_STALLED: &str = "burnout_stalled";
pub const PROJECT_TREND_DECLINE: &str = "project_trend_decline";

pub fn all_user_facing() -> &'static [&'static str] {
    &[BURNOUT_OVERLOAD, BURNOUT_STALLED, PROJECT_TREND_DECLINE]
}
```

Both lists are complete today. `channel` has three constants and
`ALL_CHANNELS` names three.

**`GLOBAL` is a deliberate exclusion**, documented in its own doc comment: a
sentinel preferences row recording whether the user has been prompted for the
email opt-in, never a real notification kind. So the rule is *every constant
except `GLOBAL`*, and that exclusion goes in the guard with its reason
attached, the way `static_js_scan` carries `search.js` — visible in the code,
not only in a document.

**This guard has to live in `peisear-core`**, since it is about that crate's
own source. Confirm `peisear-core` has a CI job that would run it — it appears
in `test-libs` as `cargo test -p peisear-core --lib`, so a `#[cfg(test)]`
module in `lib.rs` is covered, and a file under `tests/` would **not** be. Say
which you used and why.

## 4. `prose_scan`'s two directories

It walks `src/components/**` and `src/handlers/**`. The crate also has
`components.rs`, `handlers.rs`, `error.rs`, `app.rs`, `extractors.rs`,
`jobs.rs`, `state.rs` and `config.rs`, none of which are scanned.

**No live gap** — none of them contains a `SCOPED_ATTRS` literal today, which
`QA-009` reported and I re-confirmed. But planted into `components.rs`, one
level up from the directory it scans:

```rust
const QA010_PLANT: &str = r#"<span aria-label="This sprint has no planned work">"#;
```

`prose_scan`: **4 passed, 0 failed.** Invisible.

`QA-009` was right to leave this alone: the two-directory list is a
*documented deliberate scope choice*, not an oversight, and widening it
unilaterally would have been the wrong call. **The posture it proposed is what
I am rejecting** — "no gap today, revisit if a new directory appears" depends
on someone noticing the directory.

Two shapes, and **a third is what I would rather have**:

- **(a) Widen to all of `src/`.** Simple. Changes what the allowlist has to
  cover, and the module doc's calibration standard is explicit that more than
  a handful of allowlist entries means the heuristic is wrong. **Run it before
  choosing** and report the new hit count — if it is zero, this is nearly free
  and (b) is over-thinking.
- **(b) Keep the scope, guard the choice.** Assert that no `.rs` file outside
  those two directories contains a `SCOPED_ATTRS` literal. The narrow scope
  keeps its documented reason and gains a tripwire that fires the day a third
  location acquires markup — pointing the next person at this decision instead
  of silently not covering them.

I lean to (b) and would drop it for (a) if (a)'s hit count is zero, because a
scan that simply covers the files is easier to explain than a scan plus a
meta-scan. Report the number and then choose.

## 5. Escalate rather than deciding

- **If any set in §2 or §3 turns out to be incomplete, stop and report.** I
  measured all three as complete on 2026-08-25; if that is wrong, I want to
  know before any guard is written, because a live hole changes the priority
  and the order.
- If (a)'s hit count in §4 is more than a handful, say so and take (b) — do
  not start allowlisting to make (a) fit.
- If the `peisear-core` guard cannot live where its CI job would run it,
  report rather than placing it somewhere that does not run.

## 6. Acceptance

1. §1 acknowledged: the review request states plainly that nothing was broken.
2. §2: per-enum membership **and** declared-length assertions for all fourteen;
   the `IssueStatusLabel` plant reproduced and demonstrated caught.
3. §3: the kind/channel guard, with `GLOBAL`'s exclusion carrying its reason in
   the code; demonstrated by planting a fourth constant absent from the list.
4. §4: (a)'s hit count reported; a choice made and justified; demonstrated
   against the `components.rs` plant above.
5. Every plant one at a time, each reverted, `git diff` clean between.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. §4's hit count as a number before the choice that followed from
it. Each plant transcript separately.
