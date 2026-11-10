# NAME-001 — the destination that is about you

**Series**: `060` (unit three), `0.62.0`'s scope as re-authorized
2026-11-11. **Blocked on**: nothing. **Small, and it closes a pin.**

## 1. What is wrong

A reader clicks **Today** and arrives at **My dashboard**. The page is
not about the day; it is about them — `FR-PER-001` (**P0**) says it
shows *"the authenticated user's own current load, work rhythm, and
sustainability signals"*, and `external-design §3.2` gives the
destination's own responsibility as **"What should I look at right now,
about myself?"**

**The page is right and the word is wrong.** `FLOW-002` held the obvious
fix — renaming the heading to *Today* — and was right to: that would
make the product's own name agree with an expectation it does not meet,
and the reader would have nothing left to correct it. The architect then
recommended that rename anyway at `0.62.0`'s proposal and **reversed it
on 2026-11-11**; the three-way comparison is in this series' README.

## 2. The decision

**The nav word becomes `Me`.** Four reasons, in the order they
mattered:

1. It is what the destination is about, in the published design's own
   words.
2. `NFR-A11Y-012` then resolves instead of staying pinned: the page's
   own title becomes the same word, and `page_title_scan`'s
   `today_is_pending_the_content_decision_not_yet_fixed` — which asserts
   the two still **differ** — is deleted in the same change, which is
   what that test was written to require.
3. **It is the shortest of the candidates**, and length is not a
   quibble here: `NAV-001` chose the `md` (768 px) collapse threshold by
   measuring that five labelled words plus the bell and the brand fit
   there. *Dashboard* is four characters longer than *Today*; **Me is
   three shorter**. The round must confirm the row still fits rather
   than assume it.
4. It reads correctly beside its neighbours: **Me · Inbox · Projects ·
   Teams · Search** — the first is you, the fourth is everyone else.

**The route does not move.** `/today` stays, `FR-NAV-002`'s 308
redirects are untouched, and no bookmark breaks. This is a word, not a
rename of a destination.

**Recorded because it is real**: the product renamed `/me` to `/today`
deliberately at `0.17.0`, and **no published document says why**. This
round partly undoes that decision at the label while leaving its route
alone, with the owner's acceptance of 2026-11-11.

## 3. What to build

- `MessageKey::NavLinkToday` renders **`Me`**.
- `/today`'s own page title and `<h1>` become the same word, satisfying
  `NFR-A11Y-012`'s *destination answers to the navigation's own word*.
  The subtitle — *"Personal metrics for {name}. Visible only to you."* —
  is unchanged and is what carries the explanation.
- `page_title_scan`'s pending-pair test is **deleted**, not edited: the
  condition it was waiting on has happened.
- `nav_destination_naming` gains `/today` as its **fifth** live-render
  case, which is the pair that test file was built to hold and could not.
- Check every other place the old word is user-visible — the breadcrumb
  root on all four detail screens is `Today` (`FR-NAV-003`'s acceptance
  asserts *"the `/today` root"* by route, not by word; read it before
  changing anything) and `CalendarBreadcrumbWord`'s own parent.

## 4. Acceptance

- `nav_destination_naming` at **five** destinations, all live-rendered.
- The `md`-threshold fit **re-measured, not assumed**: the five words,
  the bell and the brand in one row at 768 px, with
  `browser-checks` 120 cells green.
- `peisear-i18n`'s own suite, including the fixture locale.
- The pending-pair test gone, and `NFR-A11Y-012`'s own *"the fifth is
  deliberately not met"* paragraph flagged in the report for the
  architect to amend — **do not amend requirements yourself**.
- `DEC-007` from → to; the citation check **before** pushing.

## 5. What this round must not do

- Do not move or alias the route, and do not touch `FR-NAV-002`.
- Do not change `/today`'s content, panels or subtitle.
- Do not change the other four nav words.
