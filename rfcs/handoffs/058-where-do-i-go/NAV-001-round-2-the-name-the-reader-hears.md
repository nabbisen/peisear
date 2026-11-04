# NAV-001 round 2 — the name the reader hears

**Round 1 gave the bell its word, and in doing so made its visible label
and its accessible name disagree.**

- visible text: `MessageKey::NavLinkInbox` → **"Inbox"**
- accessible name: `bell_aria` → `NavBellLabelNone`/`NavBellLabelUnread`
  → **"Notifications"** / **"Notifications (3 unread)"**

**WCAG 2.5.3 Label in Name, Level A**: a control's accessible name must
*contain* the text presented visually. "Notifications" does not contain
"Inbox". **While the bell was icon-only this could not arise** — there was
no visible label. Round 1 added the word, which was right, and the
mismatch is the consequence.

**It is not a design question, it is a leftover.** `FR-NAV-002`'s own
acceptance records the rename `/notifications` → `/inbox`. The rename
reached the route, the page and the nav entry; **the accessible name is
the last place the old name survives as the destination's name.**
`FR-NAV-001` names the entry **Inbox**.

## §0 — what this round is

`NavBellLabelNone` → **"Inbox"**; `NavBellLabelUnread { count }` →
**"Inbox ({count} unread)"**. Two strings in `peisear-i18n`, reached from
**two** call sites — `layout.rs:212` (primary row) and `:277` (the `Menu`
duplicate) — so verify both render the new name.

**Do not touch the other five `Notification*` strings.** *Notification
preferences*, *Notification kinds*, *Notification list*, *Notifications
silenced* and the `NotificationsSectionName` all name **the items**, not
the destination, and that distinction is correct: the page is Inbox, the
things in it are notifications. **Collapsing that would be a worse defect
than the one being fixed.**

## §1 — the fixture locale is part of this

`peisear-i18n` has an enforced-exhaustiveness arrangement and a **fixture
locale** (`tests/common/fixture_locale.rs`) whose whole purpose is to
prove the rendering mechanism is not English-shaped. Two message values
change, so **run `peisear-i18n`'s own suite**, not only `peisear-web`'s —
`guard.rs`, `rendering.rs`, `enumeration_guard.rs`, `language_guard.rs`.
If any of them pins the old string, that pin is part of the change and
its own reason should be read before it is edited.

## §2 — seen red

**Assert the agreement, do not just change the strings.** A test that the
bell's `aria-label` **contains** its visible label text, rendered from
the keys on both sides (`REQ-005`'s lesson). Then plant: set
`NavBellLabelNone` back to `"Notifications"` and watch it fail naming the
mismatch; revert; green.

**Put it in `navbar.rs`**, beside round 1's two tests, and **scope it to
the primary `<nav>`** the same way — round 1's own finding was that an
unscoped assertion stays green on the `Menu` duplicate.

## §3 — what this round does not decide

**Whether a requirement should govern accessible-name/visible-label
agreement is not settled here, and must not be invented in passing.**
The architect's bounded measurement: roughly **twenty** controls or
containers in `components/` carry both an `aria-label` and visible
`t(MessageKey…)` text. **That figure is approximate and not a defect
count** — a `<nav>`'s or a banner's own label is not a 2.5.3 case at all,
and most of the remainder look like they match (`MarkAllReadAriaLabel`
against `MarkAllReadButton`). **The bell is the one confirmed mismatch,
and the architect did not verify the other pairs' strings.**

So: **report whether, while you were in there, any other pair plainly
disagrees.** Do not audit twenty. One instance is not a class, and
`REQ-004`'s own 117 unaudited entries are this project's precedent for
naming a denominator rather than quietly adopting it.

## §4 — gates

`fmt`; `clippy --workspace --all-targets -- -D warnings`;
**`peisear-i18n`'s full suite** and `peisear-web`'s `navbar` binary at
minimum, and the whole integration set if any rendered string outside the
navbar moves; `contrast_scan` and `touch_target_scan` (unchanged by a
string change, so state that you re-ran them rather than assuming);
`DEC-007` from the runner, **stated from and to** — this round adds a
test, and remember `test.yml`'s own block lines define the count, where a
bare `-p peisear-i18n` includes that crate's `tests/*.rs`.

## §5 — escalate rather than deciding

- If a `peisear-i18n` test pins `"Notifications"` **as the bell's name**
  for a reason the handoff has not seen, **report it and stop.**
- If "Inbox (3 unread)" reads worse in context than the architect expects
  — a badge already shows the count beside it — **say so with what you
  saw.** The count in the accessible name exists for readers who cannot
  see the badge, so removing it is not obviously right; that is a copy
  decision and it is the architect's.

## §6 — exit conditions

1. The bell's accessible name contains its visible label at both call
   sites.
2. The other five `Notification*` strings untouched.
3. A scoped test asserting the containment, rendered from keys, **seen
   red on a plant**.
4. `peisear-i18n`'s own suite run and reported.
5. Any other plainly-disagreeing pair reported, with no audit undertaken.
6. `§4`'s gates after the last edit, `DEC-007` from and to.
