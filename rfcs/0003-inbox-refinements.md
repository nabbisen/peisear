# RFC 0003: Inbox refinements

**Status**: Accepted
**Target**: 0.22.0 (Phase C PR4)
**Related spec sections**: §19 (Inbox), §6 (notifications),
§38.1 task 4
**Last updated**: 2026-05-04

## Summary

Tidy-up of the existing `/inbox` page: a fixed silence-resume
banner when notifications are globally silenced, a prominent
"mark all read" button, a deferred email-opt-in prompt that
appears after the user's first in-app notification rather
than at registration, and a small UX fix in search to make
sub-issue results legible (their parent's title in a
breadcrumb hint).

This is a small PR — the heavy lifting on notifications
already shipped through migrations and `peisear-notify`.

## Design

### Silence-resume banner

When `notifications::global_acknowledged(user_id)` returns
true (the user has hit "silence all"), render a fixed banner
at the top of the inbox view:

```
[!] Notifications are silenced.       [Resume notifications]
```

The Resume button POSTs to `/inbox/silence/resume` (new
endpoint) which clears the silence flag and 303-redirects
back to `/inbox`. The page hides the banner when silence is
not active — symmetric, no leftover banner.

Keep the banner inside the `<main>` of `/inbox` only. It is
not a global banner; pushing it to the app shell would visit
every page and read like an alarm. The user already came to
the inbox to look at notifications.

### Mark-all-read button

The storage helper `notifications::mark_all_read` already
exists. Surface it as a button at the top of the
notifications list:

```html
<form method="post" action="/inbox/mark-all-read">
  <button type="submit"
          class="btn btn-ghost btn-sm"
          aria-label="Mark all unread notifications as read">
    Mark all read
  </button>
</form>
```

Hide the button when `unread_count == 0`. Clicking POSTs to
`/inbox/mark-all-read` (already exists or added here, see
"Routes"), redirects back.

### Email opt-in deferral

Today, the email-opt-in prompt fires at registration. The
spec §19.4 wants it deferred until after the user has seen
the value of one notification:

> 「あなたに 1 つ通知が届きました。 これを email でも受け取りますか?」

Implementation:

1. Two boolean columns on `users`:
   - `email_opt_in`: the user's choice (NULL = unset).
   - `email_opt_in_prompted_at`: timestamp the prompt was
     last shown. NULL = never shown.

   New migration `0017_users_email_opt_in.sql`.
2. Remove the prompt from registration. Registered users
   simply have `email_opt_in IS NULL` until they've seen
   the prompt.
3. On `/inbox` GET, if all of:
   - `email_opt_in IS NULL`
   - `email_opt_in_prompted_at IS NULL`
   - the inbox has at least one in-app notification, read or
     unread
   are true, render an opt-in banner above the list:

   ```
   [?] You've received your first notification. Want these
   by email too?  [Yes, send email]  [No thanks]
   ```

   Both choices POST to `/inbox/email-opt-in` with `choice=yes`
   or `choice=no`, set `email_opt_in` accordingly, set
   `email_opt_in_prompted_at = now()`, and 303 back. After
   that, the banner does not reappear.
4. The user can still change their choice in `/settings`
   (existing notification preferences).

### Sub-issue parent breadcrumb in search

Phase C PR1 deferred this: when a search result is a
sub-issue, the result row should hint at its parent. Today
all results render as "Project / Issue Title". For sub-issues
we want "Project / Parent Title / Sub-issue Title".

Implementation in the search-result row component
(`components/search.rs`): when the result is an issue and
its `parent_issue_id` is non-NULL, fetch the parent's title
and prepend it. The fetch is one extra `IN (...)` query for
the page, batched: gather all sub-issue parent ids and fetch
them in one round-trip rather than one per row.

This keeps search-result rendering coherent with Phase C
PR1's principle that sub-issues "make sense in the context
of their parent" (§8.5).

### Routes

```
POST /inbox/mark-all-read       (already exists; verify)
POST /inbox/silence/resume      (new)
POST /inbox/email-opt-in        (new)
```

All three return 303 to `/inbox`.

### Migration `0017_users_email_opt_in.sql`

```sql
ALTER TABLE users
    ADD COLUMN email_opt_in INTEGER;  -- 0/1/NULL

ALTER TABLE users
    ADD COLUMN email_opt_in_prompted_at TIMESTAMP;
```

Existing users have NULL for both, so they will see the
prompt at their next inbox visit (assuming they have at
least one notification). This is the desired migration
behaviour: existing users get the deferred prompt the spec
calls for, instead of being grandfathered into "never
prompted."

If we want grandfathered users to be implicitly opted out
(no email, no prompt), the migration should set
`email_opt_in = 0` and `email_opt_in_prompted_at = now()`
for all existing rows. **Default-if-no-decision: do not
grandfather. Show the prompt to existing users.** The
prompt is not noisy — a single banner — and giving existing
users the same affordance the spec gives new users is the
fairer choice.

## Test plan

Extend the existing `tests/smoke.rs` and add a small new
test crate `tests/inbox_refinements.rs`:

1. `silence_resume_banner_renders_when_silenced` — flip
   `global_acknowledged`, GET `/inbox`, expect the banner.
2. `silence_resume_banner_hidden_when_not_silenced` —
   negative.
3. `mark_all_read_button_hidden_when_unread_zero`.
4. `mark_all_read_button_marks_and_redirects` —
   create 3 unread notifications, POST mark-all-read, GET
   `/inbox`, expect 0 unread.
5. `email_opt_in_banner_appears_after_first_notification`
   — fresh user, no notifications: no banner. Insert one
   notification, GET `/inbox`: banner present.
6. `email_opt_in_yes_persists_choice_and_hides_banner` —
   POST `choice=yes`, second GET has no banner, settings
   page reflects opt-in.
7. `email_opt_in_no_persists_choice_and_hides_banner`.
8. `search_result_shows_parent_breadcrumb_for_sub_issue`
   — extend `tests/search.rs` (don't make a new crate) to
   create a sub-issue and confirm its result row shows the
   parent title.

CI: extend `tests/search.rs` job (already present) and add a
new job `test-peisear-web-inbox-refinements` mirroring the
others.

## Security & privacy considerations

- §11.5: nothing changes. Inbox content was already
  self-only; the new endpoints all act on the authenticated
  user's own data.
- §21.4: notifications are not subject to the optimistic-
  lock contract (high-frequency aggregate; last-write-wins
  is the user's mental model — they hit "mark all read" and
  expect everything to go to read regardless of races). No
  changes here.
- The deferred opt-in itself is a privacy improvement —
  asking after value has been demonstrated, instead of at
  registration, lets the user say no with information.

## Out of scope

- Snooze. The spec doesn't list it for PR4 and the storage
  shape isn't there yet (no per-notification snooze-until).
  Possible later PR.
- Notification grouping ("3 notifications about issue #45").
  Phase E candidate; a careful implementation needs more
  thought about how grouping interacts with mark-as-read.
- Push / web-push notifications. Out of scope.

## Open questions

1. **Existing users and the email opt-in prompt** — see
   migration discussion. *Default: show the prompt to
   existing users.*
2. **Banner placement when both banners apply** (silenced
   *and* first-notification-just-arrived): show the silence
   banner only — silencing implies the user doesn't want
   email either, so the email prompt is misplaced. *Default:
   silence wins.*
3. **What counts as "first notification"** — only in-app, or
   also dispatched emails (if email opt-in defaults change)?
   *Default: in-app only. The opt-in is *for* email, so we
   shouldn't gate on email events.*

## References

- Spec §19 — Inbox
- Spec §6 — notification subsystem
- Spec §38.1 task 4 — Phase C tasks
- CHANGELOG entry for 0.18.0 — `/me` → `/today`,
  `/notifications` → `/inbox` 308 redirect (this RFC builds
  on the renamed surface)
