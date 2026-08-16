# RFC 0003: Inbox refinements

**Status**: Implemented (0.24.0), on the rewrite. The 2026-05-04 text was
returned from Accepted on 2026-08-16 and is superseded in full

> **Implemented by `INBOX-001`**, one handoff, two review rounds, no schema
> migration.
>
> **This is the only RFC in the project so far to have been rewritten rather
> than amended**, and the record of what that bought belongs here rather than
> only in the reconciliation note. Against the May text, the rewrite removed a
> schema migration, dropped an item that was already built *before* the RFC was
> accepted, avoided giving two facts a second home, and corrected a banner
> trigger that read `global_acknowledged` — "has been prompted for the email
> opt-in" — as though it meant "has silenced notifications".
>
> That last one is why the round trip was worth a release's delay. An
> implementer following the May text would have used that function, and a test
> written from the same text would have used it too. They would have agreed
> with each other and both been wrong, and no amount of care downstream catches
> that. `inbox_refinements`'s test 2 now fails if anyone reaches for it again.
**Target**: 0.24.0
**Related spec sections**: §19 (Inbox), §6 (notifications), §38.1 task 4
**Supersedes**: the accepted-2026-05-04 version of this RFC, in full
**Last updated**: 2026-08-16

> **Why this was rewritten rather than amended.** The previous text was
> accepted 2026-05-04 and never dispatched. Reconciled against the shipped code
> before any handoff was written
> (`.git-exclude/tasks/architect/008-rfc-003-reconciliation.md`), three of its
> four design items did not survive:
>
> - **Mark-all-read** was already built — hide-when-zero included — at
>   0.9.0–0.16.0, *before* the RFC was accepted. It was wrong on the day it was
>   written, not stale now.
> - **The silence-resume banner** triggered on `global_acknowledged`, which
>   means "has been prompted for the email opt-in". The `silence_all` handler
>   carries a comment saying exactly that: *"Don't touch the global pref row —
>   that's only the first-login email opt-in record, conceptually different
>   from per-kind silencing."* The RFC contradicted a comment in the code it
>   was describing.
> - **Migration `0017`** would have added `users.email_opt_in` and
>   `email_opt_in_prompted_at`, a second home for facts the
>   `notification_preferences` global row already holds.
>
> Amending those in place would have produced a document whose reasoning no
> reader could follow. This version is written from the code.

## Summary

Three refinements to `/inbox`, all small, and **no schema migration**:

1. A silence-resume banner, triggered on the condition that actually
   represents silence.
2. The email opt-in prompt moved to the inbox, after a first notification —
   using the record that already exists for it.
3. Sub-issue search results showing their parent, so they read in context.

## Design

### D1 — Silence-resume banner

**The trigger.** A user is silenced when every kind in
`kind::all_user_facing()` has an empty channel set. That is precisely what
`silence_all` writes (`handlers/notification_preferences.rs:136`), and it is
the only state that means "silenced".

Add one storage predicate rather than deriving it at two call sites:

```rust
/// True when every user-facing kind has an empty channel set —
/// the state `silence_all` produces, and the only state that
/// means "silenced". Not `global_acknowledged`, which records
/// whether the email opt-in has been answered.
pub async fn all_kinds_silenced(pool: &Pool, user_id: &str) -> StorageResult<bool>;
```

The doc comment carries the distinction because the previous version of this
RFC got it wrong, and the next reader will be looking at two similarly-named
functions.

**Resume deletes rows; it does not write defaults.** `dispatch.rs:242` reads
`None => DEFAULT_CHANNELS.to_vec()` — an absent per-kind row already means "the
default". So resume is the exact inverse of silence: remove the rows
`silence_all` created and let absence mean what it already means.

Writing `DEFAULT_CHANNELS` into the rows instead would give the default a
second home, and a later change to `DEFAULT_CHANNELS` would reach users who
never silenced anything while missing every user who had resumed. This project
has recorded that defect shape four times; it is avoidable here by deleting.

**Placement.** Inside `<main>` on `/inbox` only. Not the app shell — a banner
on every page reads as an alarm, and the user came to the inbox to look at
notifications.

### D2 — Email opt-in at the inbox, with no new columns

The facts already exist:

- `global_acknowledged(user_id)` — "has this user been prompted?"
- `set_global_acknowledged(user_id, email_opt_in: bool)` — records the answer,
  writing `channels = "in_app,email"` or `"in_app"` on the global row.

So this item is a surface, not a schema change. On `GET /inbox`, when
`!global_acknowledged(user_id)` **and** the user has at least one notification,
render a prompt above the list. Both answers POST to `/inbox/email-opt-in` and
call `set_global_acknowledged`; the prompt does not reappear.

**Nothing is removed from registration** — there is no prompt there. The
previous version's step 2 had nothing to act on.

**Existing users see the prompt** at their next inbox visit with a
notification, which is the same affordance new users get. Not grandfathered
out — the previous version's default-if-no-decision, and still right.

### D3 — Sub-issue parent in search results

Unchanged from the previous version, and the one item that reconciled cleanly:
`components/search.rs` has no parent handling at all today.

When a result is an issue with a non-NULL `parent_issue_id`, render
`Project / Parent title / Sub-issue title`. Gather the parent ids for the page
and fetch them in **one** query, not one per row.

## Requirements

1. The banner appears exactly when every user-facing kind is silenced, and not
   otherwise.
2. Resume restores the default by deleting the rows, and a resumed user is
   indistinguishable from a user who never silenced.
3. The opt-in prompt appears once, for a user who has never been prompted and
   has at least one notification; either answer stops it permanently.
4. No schema migration.
5. Sub-issue search results name their parent; the parent lookup is one query
   per page.
6. All copy through `peisear-i18n` (RFC 006 §D6, including rule 7).

## Test plan

| # | Check |
|---|---|
| 1 | Banner absent by default; present after `silence_all`; absent again after resume |
| 2 | **The trigger is not `global_acknowledged`** — a user who has answered the email prompt and silenced nothing sees no banner |
| 3 | Resume deletes the rows: after resume, `preference_for_user_kind` returns `None` for every user-facing kind |
| 4 | A resumed user receives a dispatch that a silenced user does not |
| 5 | Prompt shown for a never-prompted user with ≥1 notification; absent with 0 |
| 6 | Either answer sets the global row and the prompt does not return |
| 7 | Sub-issue result renders its parent's title; a top-level result does not |
| 8 | The parent lookup issues one query for a page with several sub-issue results |

Test 2 is the regression guard for this RFC's own history. Write it so it fails
if someone reaches for `global_acknowledged` again.

## Security and privacy considerations

- The banner and prompt disclose nothing about anyone else.
- `/inbox` is already self-only; no new authorisation path.
- The search breadcrumb shows a parent title from a project the viewer can
  already read — `find_accessible` governs the result set and is unchanged.
- No new personal data is stored. D2 records a yes/no the user just gave.

## Out of scope

Mark-all-read (built). Any schema change. Per-kind resume — resume is
all-or-nothing, matching silence. Email delivery behaviour itself. A global
banner outside `/inbox`.

## Open questions

1. **Does a read notification count for D2's "at least one"?** The spec's
   wording is "you have received one notification", which reads as *received*,
   not *unread*. *Default-if-no-decision: any notification, read or unread.*
2. **Should resume be confirmable?** It is not destructive, so probably not —
   but it is a mutation with no undo beyond re-silencing. *Default-if-no-
   decision: no confirmation.* Note this interacts with external design §17.4's
   open question on the confirmation pattern; if that lands first, revisit.

## References

- `.git-exclude/tasks/architect/008-rfc-003-reconciliation.md` — the findings
  that produced this rewrite
- `handlers/notification_preferences.rs:136` — `silence_all`
- `storage/notifications.rs:326, 348` — `global_acknowledged`,
  `set_global_acknowledged`
- `peisear-notify/src/dispatch.rs:242` — absent row means `DEFAULT_CHANNELS`
