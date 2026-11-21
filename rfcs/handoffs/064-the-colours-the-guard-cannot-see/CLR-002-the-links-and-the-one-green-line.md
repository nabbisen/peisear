# `CLR-002` — The links, and the one green line

**Round 2 of series `064`.** Read `README.md` in this directory first.

**Sequencing, because it matters here**: this round and `CLR-001` both touch
`issues.rs`. **Start after `CLR-001`'s commit is on `main`** and rebase onto
it; if you must begin sooner, do the five non-`issues.rs` sites first and
leave `issues.rs:463` until after that commit lands. Say which you did.

**Scope: five links at 4.21:1 and one status line at 3.03:1**, against
`NFR-A11Y-005`'s 4.5:1. Nothing else.

---

## 1. The six

| site | what it is | class today |
|---|---|---|
| `auth.rs:72` | *"Create one"* — the register link on the sign-in page | `link link-primary inline-flex items-center` |
| `auth.rs:146` | the sign-in link on the register page | `link link-primary inline-flex items-center` |
| `notifications.rs:172` | a link to the notification settings | `link link-primary` |
| `notifications.rs:257` | a notification's own target link | `link link-primary` |
| `issues.rs:463` | the health figure's *basis* link | `link link-primary inline-flex items-center` |
| `notification_preferences.rs:67` | *"✓ Email opt-in is on by default."* | `text-xs text-success` |

Links: `#4d6eff` on `base-100` = **4.21:1**. Status line: `#00a96e` on
`base-100` = **3.03:1**. Neither is large text, so neither has the 3:1
route. The two `auth.rs` sites are the **first links a new reader meets**.

---

## 2. Two things I measured that do *not* support a larger claim

Both are written down because a round should not have to re-derive a
negative, and because a handoff that only lists what supports its own case
is the thing this series exists to correct:

1. **The links are not a "colour alone" defect.** WCAG F73 asks that a link
   distinguished from surrounding text by colour alone reach **3:1 against
   that text**; `#4d6eff` against `base-content` `#181a2a` measures
   **4.09:1**, which passes. And `.link` itself sets
   `text-decoration-line: underline` (confirmed in the pinned sheet), so
   these five are underlined at rest anyway. **The only defect here is the
   ratio against the background.**
2. **The status line does not convey state by colour alone either.** Its
   string is *"✓ Email opt-in is on by default."* — the glyph and the
   sentence carry the state, and the off-state sibling two lines below is
   already `text-base-content/70` and passes. **The green is decoration on
   text that must still meet 4.5:1, and does not.**

---

## 3. Recommendations

- **The five links → drop `link-primary`, keep `link`.** The class
  underlines by itself and the text then inherits `base-content`:
  **17.21:1**, with the affordance untouched. No new colour, nothing added
  to `app.css`.
- **The status line → `text-base-content`** (17.21:1), keeping the ✓ and the
  sentence. **Its size stays `text-xs`** — that site is one of the 21
  small-print sites a later row will read as a set, and its *type* is not
  this series' business. A `badge badge-success` was considered and is not
  recommended: the string is a sentence, not a chip label.
- **Rejected, and measured rather than waved away**: keeping the green and
  making it large enough for the 3:1 route. `3.03:1` would scrape past at
  ≥24 px, and a 24 px status sentence on a settings page is not a repair.

---

## 4. What to measure

- **Per site, before and after**: rendered foreground, the background it
  actually sits on, and the ratio — in a browser, not from the stylesheet.
  Expect `4.21:1 → 17.21:1` and `3.03:1 → 17.21:1`.
- **Hover and focus**, reached by real dispatched events rather than forced
  pseudo-classes. `link-primary`'s own hover colour and `.link`'s hover
  behaviour are **not** assumed here: measure what the pair does now and
  what it does after, and report both. `§10.60`'s contrast work found
  hovered states lower than resting, which is why this is not optional.
- **That the underline survives** on all five at rest, after the change.
- **390 px and 1280 px**: the `inline-flex items-center` sites sit in rows
  with other content; confirm nothing reflows.
- **`notifications.rs:257` uses `grow_text(...)`**, so its 44 px touch
  target comes from `TOUCH_TARGET`; confirm the box is unchanged by a colour
  edit rather than assuming it.

---

## 5. Out of scope, including one thing found while measuring

- **`issues.rs:2256`, `sprints.rs:719`/`:747`, `projects.rs:295`,
  `layout.rs:417`, `teams.rs:619`, `settings.rs:469`** — `CLR-001`'s seven.
- **`confirmation.rs:56` and `projects.rs:286`** — `CLR-003`'s two
  headings, with the guard and the amendment.
- **The three sites that pass** (`error_page.rs:18`, `auth.rs:15`,
  `layout.rs:211`) — untouched, and confirm it.
- **Nine `link link-hover` sites, found while measuring this round and
  deliberately left**: `link-hover` *removes* the underline at rest and
  shows it on hover, so those links are distinguished from their
  surroundings by position alone until pointed at. That is a **use-of-colour
  and affordance** question rather than a contrast one — F73 only bites for
  a link *inside a block of text*, and some of these are standalone card
  titles and pagination controls where it does not apply. Recorded as a
  `ROADMAP.md` candidate; **do not change them here**, and do not convert
  any of this round's five into `link-hover`.

---

## 6. Acceptance

1. **Six sites accounted for**, before/after ratios with backgrounds named.
2. **Hover and focus measured** for every link changed, both states
   reported.
3. **The underline confirmed present at rest** on all five links after the
   change.
4. **The three passing sites untouched**, confirmed by grep and stated.
5. **Sizes untouched** — this round changes colour only.
6. **`CHANGELOG.md`** under `[Unreleased] → Fixed`, in the same entry family
   as `CLR-001`'s if that has landed: what a reader could not read, and what
   changed.
7. **Gates in full** per `README.md §5`, with `css_scan`'s population
   re-measured — removing `link-primary` from five sites and `text-success`
   from its only site may take both tokens out of `components/`, and if it
   does, the number moves and the reason goes in the assertion's comment.
8. **Privacy floor per artefact**; scratch under `.git-exclude/tmp/`.
9. **Review request** at
   `.git-exclude/review-request/CLR-002-the-links-and-the-one-green-line/README.md`.

**If a measurement contradicts this file, the measurement wins** — including
the two negatives in §2.
