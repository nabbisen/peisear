# FLOW-002 — the word on the door and the name inside

**Series**: `059` (unit two, user workflows). **Blocked on**: nothing.

## 1. What was measured

Walking the product as a reader, two of the five destinations answer to
a different name than the one on the control that leads there:

| Navigation word | `<title>` and `<h1>` on arrival | Since |
|---|---|---|
| **Inbox** (`NavLinkInbox`, `NavBellLabel*`) | **Notifications** (`NotificationsSectionName`) | `NAV-001`, 2026-11-04 — the bell said *Notifications* before it said *Inbox* |
| **Today** (`NavLinkToday`) | **My dashboard** (`PersonalDashboardTitle`) | pre-existing |

`FR-NAV-002` already records the rename `/notifications` → `/inbox`;
the page's own heading is the last place the old name survives **as the
name of the destination**. `NAV-001` fixed the accessible-name half of
this (WCAG 2.5.3) and left the arrival half, which no criterion covers
and which the reader meets second.

**The page titles are a second shape of the same thing.** Of the page
titles in `crates/peisear-i18n/src/en.rs`, ~~five~~ **seven** end with
`— peisear` (`ErrorPageTitle`, `ProjectsListPageTitle`,
`ProjectNewPageTitle`, `LoginPageTitle`, `RegisterPageTitle`, and —
*corrected 2026-11-09, by the round, not by its author* —
`ProjectDetailPageTitle` and `ProjectEditPageTitle`, which carry the
suffix **inside a `format!`** and so were invisible to the literal-string
grep this paragraph's own count came from) and the rest do not. Three of the
product's own screens are therefore called nothing but **`Calendar`**,
**`Notifications`** and **`My dashboard`** in a tab strip, a bookmark,
a history list or a screen reader's window list — none of which says
which application they belong to, and two of which are words any other
open tab might also be using.

## 2. The decision

> **Scope narrowed 2026-11-07, before the round started, by the
> architect's own re-reading at the owner's request.** It covered both
> renames; it now covers **Inbox and the titles only**. Renaming
> `/today`'s heading to *Today* is held back, and §3 says why.

**The word on the door is the name inside.** `FR-NAV-001` fixes the five
words — Today, Inbox, Projects, Teams, Search — so the destinations
answer to them, in `<title>` and in `<h1>`.

**Every page title names the product.** One rule, no exceptions:
`<page name> — peisear`. Five already do; the rest follow. A title that
is built from user text (`New issue — {project_name}`,
`Sprint Plan — {sprint_name}`) keeps its own shape and gains the suffix
after it.

## 3. What to build

- `NotificationsSectionName`'s use **as the inbox page's own title and
  heading** becomes `Inbox`. **The five `Notification*` strings that name
  the items in it do not change** — the page is the Inbox, the things in
  it are notifications, and collapsing that distinction would be a worse
  defect than this one. `NAV-001 r2` made exactly this call for the
  bell's label; make it the same way, and check each call site rather
  than renaming the key globally.
- **`PersonalDashboardTitle` is *not* renamed in this round.** The first
  version of this handoff said it should become `Today`, noting that the
  rename "makes that sharper" and calling it right anyway. On re-reading,
  that is the wrong order. Today a reader clicks *Today* and arrives at
  *My dashboard*: two names for one place, which at least tells them what
  the page is. Rename it first and they click *Today*, arrive at *Today*,
  and find two metric chips and no tasks — **one name, now making a
  promise the page does not keep**, which is the worse of the two
  confusions and the harder one to notice. `components/me.rs` renders no
  issue at all; the day's planned block is on `/today/calendar`. So the
  content decision (this series' README, open question 1) comes first and
  the rename follows it, in whichever round answers it. The navigation
  word stays as `FR-NAV-001` fixes it either way.
- Every page title gains the ` — peisear` suffix. Build it **once**, not
  by editing twenty strings: a single helper that the shell applies, so
  the rule is in one place and a new page cannot forget it.

## 4. Acceptance

- A test over `en.rs` asserting **every** page-title string ends with
  `— peisear`, with the population count asserted — the same shape
  `spec_scan` rule 3 uses. If the suffix is applied by a helper rather
  than stored in the strings, assert it at the helper's own output for
  every variant, and say in the round report which of the two you built
  and why.
- A test asserting, for each of the five entries `FR-NAV-001` names,
  that the navigation word and the destination's own `<h1>` are the
  **same string**, by `MessageKey` — not two literals that happen to
  match today. This is the rule, and it is cheap: five pairs. **`Today`
  is the one pair that will not hold yet**: assert the other four, and
  record `Today` in the test by name as pending the content decision,
  with the entry failing the moment that decision lands and the rename
  does not. A rule with a silent exception is worse than four pairs and
  a named fifth.
- `peisear-i18n`'s own suite, including the fixture locale, which must
  stay deliberately distinct (`NAV-001 r2` read it correctly; read it
  the same way).
- `DEC-007` from → to, measured with the runner.

## 5. What this round must not do

- **Do not change what `/today` renders, and do not rename its heading.**
  Both wait on the owner's answer to open question 1.
- **Do not rename the five `Notification*` item strings.**
- **Do not touch `FR-NAV-001`'s own five words.** They are specified.
