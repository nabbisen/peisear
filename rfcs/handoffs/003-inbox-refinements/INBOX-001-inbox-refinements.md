# INBOX-001 — Inbox refinements

**Issued by**: Architect
**Date**: 2026-08-16
**Priority**: P1 — 0.24.0's feature work
**Governing RFC**: [003](../../accepted/003-inbox-refinements.md), rewritten
2026-08-16 and accepted the same day
**Depends on**: nothing

---

## 1. Scope

All three of RFC 003's items, in one handoff. No split: they are independent of
each other, there is **no schema migration**, and nothing here is irreversible
the way CAL-001's was.

1. The silence-resume banner on `/inbox`.
2. The email opt-in prompt, moved to `/inbox`.
3. Sub-issue search results naming their parent.

**If you find yourself writing a migration, stop and report.** RFC 003's
previous version had one; the rewrite exists partly because it was unnecessary,
and a migration appearing here would mean something in the reconciliation was
wrong.

## 2. Read the RFC's own history first

RFC 003 was accepted in May, returned to `proposed/` on 2026-08-16 after a
reconciliation against the code, rewritten, and re-accepted the same day. Its
header carries what did not survive.

**The most important of those, because it would have passed its own test**: the
old text triggered the banner on `notifications::global_acknowledged`, which
means *"has this user been prompted for the email opt-in"* — not *"has this
user silenced notifications"*. `silence_all` says so in a comment
(`handlers/notification_preferences.rs:141`).

An implementer following the old RFC would have used that function, and a test
written from the same RFC would have used it too, and both would have agreed
with each other and been wrong. **Test 2 below exists to make that
unrepeatable.**

## 3. Item 1 — the banner

### 3.1 The predicate

Add to `peisear-storage::notifications`:

```rust
/// True when every kind in `kind::all_user_facing()` has an
/// empty channel set — the state `silence_all` produces, and
/// the only state that means "silenced".
///
/// Not `global_acknowledged`, which records whether the
/// first-login email opt-in has been answered. RFC 003's
/// first version conflated the two.
pub async fn all_kinds_silenced(pool: &Pool, user_id: &str) -> StorageResult<bool>;
```

Keep that second paragraph. Two similarly-named functions sit next to each
other and one of them has already misled a document.

### 3.2 Resume deletes; it does not write defaults

`dispatch.rs:242` reads `None => DEFAULT_CHANNELS.to_vec()`, and
`DEFAULT_CHANNELS` is `[IN_APP]`. An absent row already means the default, so
resume removes the rows `silence_all` created and stops.

Writing `DEFAULT_CHANNELS` into the rows would give the default a second home:
a later change to it would reach users who never silenced anything and miss
every user who had resumed. That is the defect shape this project has recorded
four times, and here it costs nothing to avoid.

You will need a delete helper — `upsert_preference` has no inverse. Scope it to
the user's user-facing kinds, not to all rows for the user.

**A resumed user must be indistinguishable from a user who never silenced.**
Test 3 asserts exactly that.

### 3.3 The edge worth knowing about

`all_user_facing()` is `[BURNOUT_OVERLOAD, BURNOUT_STALLED,
PROJECT_TREND_DECLINE]`. If a fourth kind is ever added, a user who silenced
before that day has empty rows for three kinds and no row for the fourth — so
`all_kinds_silenced` returns false and **their banner disappears without them
resuming anything**.

That is the correct behaviour (they are, in fact, no longer silenced for
everything) but it is surprising, and it is the kind of thing found in
production. Note it in the predicate's doc comment. Do not build machinery for
it.

### 3.4 Placement

Inside `<main>` on `/inbox` only. Not the app shell.

## 4. Item 2 — the opt-in prompt

Both facts already exist: `global_acknowledged` reads "has been prompted",
`set_global_acknowledged(user_id, email_opt_in: bool)` records the answer on the
global preference row. **No columns, no migration.**

On `GET /inbox`: if `!global_acknowledged(user_id)` **and** the user has at
least one notification, render the prompt above the list. Read or unread both
count — RFC 003 open question 1's default, and the spec's wording is
"received".

Both answers POST to `/inbox/email-opt-in`, call `set_global_acknowledged`, and
303 back. The prompt never returns.

**Nothing is removed from registration.** There is no prompt there; the old
RFC's step 2 had nothing to act on, and looking for it will waste your time.

**No confirmation on either answer** (open question 2's default). Note that
external design §17.4's confirmation-pattern decision is open; if it lands
before this ships, say so rather than pre-empting it.

## 5. Item 3 — the parent in search results, in one query and no batch

**RFC 003 §D3 says to gather parent ids and fetch them in a second batched
query. Do not.** `open_issues_by_title` (`storage/search.rs:124`) is already a
single `SELECT` joining `projects`; a `LEFT JOIN issues parent ON parent.id =
i.parent_issue_id` adds the parent title to the same result set for nothing.

No second round trip, no N+1 to reason about, and no need to test a query
count — which is the part of RFC 003's test plan that would have been awkward
to assert honestly.

`SearchHit::Issue` gains `parent_title: Option<String>`. The component renders
`Project / Parent / Title` when it is `Some`, `Project / Title` when `None`.

The `LIMIT` and the access predicate are untouched. A `LEFT JOIN` cannot drop
rows, but confirm that rather than assume it — a top-level issue must still
appear.

## 6. Tests

| # | Check |
|---|---|
| 1 | Banner absent by default; present after `silence_all`; absent after resume |
| 2 | **Regression guard**: a user who has answered the email prompt and silenced nothing sees **no** banner |
| 3 | After resume, `preference_for_user_kind` returns `None` for every user-facing kind |
| 4 | A resumed user receives a dispatch a silenced user does not |
| 5 | Prompt shown for a never-prompted user with ≥1 notification; absent at 0 |
| 6 | Either answer records the choice; the prompt does not return |
| 7 | A sub-issue result renders its parent's title; a top-level result renders without one and is not dropped |

**Test 2 is written first and demonstrated failing** — implement the banner on
`global_acknowledged` deliberately, watch test 2 fail, then implement it on
`all_kinds_silenced` and watch it pass. That transcript is the evidence that
this RFC's own history cannot repeat.

Target: a new `crates/peisear-web/tests/inbox_refinements.rs`, with a CI job and
a `CONTRIBUTING.md` line.

## 7. Escalate rather than deciding

- If a migration seems necessary, stop — see §1.
- If deleting preference rows turns out to conflict with something reading them
  as "explicitly set to nothing", report it. That would mean absence and empty
  are not interchangeable, and §3.2's reasoning would need revisiting.
- If the `LEFT JOIN` changes the result set for top-level issues in any way,
  stop.
- If §17.4's confirmation decision lands mid-flight, report rather than adopt.

## 8. Acceptance

1. All seven §6 tests pass; test 2 demonstrated failing on a deliberate
   `global_acknowledged` implementation.
2. No migration; no new columns.
3. Resume deletes rows; a resumed user is indistinguishable from one who never
   silenced.
4. Search adds no query — one `SELECT`, with the parent joined.
5. All new copy through `peisear-i18n`; `prose_scan` passes with no new
   allowlist entries.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs; `test_harness_scan` passes.

## 9. Prohibited

No schema migration. No `users.email_opt_in` or `email_opt_in_prompted_at` —
those facts have a home. No mark-all-read work; it is built. No global banner
outside `/inbox`. No per-kind resume — resume mirrors silence, all or nothing.
No confirmation dialog on resume.

## 10. Required review-request format

Workflow §9.2. Include test 2's failing transcript as first-class evidence, and
state whether the `LEFT JOIN` changed anything about top-level results.
