# A11Y-006 — eleven Tab presses to reach the content

**Issued by**: Architect
**Date**: 2026-10-07
**Target release**: 0.42.0
**Related requirement**: **`NFR-A11Y-011` — Not met**, measured `A11Y-001`.
**Depends on**: nothing. **First of the three.**

---

## 1. What is measured and not disputed

Every native form POST redirects, and on the resulting page focus is on
**`body`** with the page scrolled to the top. Reaching the first control inside
`<main>` then costs **11 Tab presses** (10 on a phone board) because the navbar
stops come first, and **no skip link exists**. On a phone this also discards
the reader's position — 605 px becomes 0 on a 1,697 px page.

**41 state-changing routes**, and `A11Y-001` established *why* it is uniform:
nothing in `static/*.js` calls `.focus()`, there is no `<main tabindex="-1">`,
and `autofocus` is on five form inputs only. **There is no code that could make
one route differ**, which is why one remedy fixes all of them.

## 2. The remedy — decided, and the reason is `DEC-021`

`NFR-A11Y-011` names three: a skip link, a focused `main` landmark, or a
focused status region. **Take the skip link.**

**It is the only one that works without JavaScript**, and a requirement about
*reaching content after a form POST* that only works when scripting is on
would be met for some readers and not others — `DEC-021`'s posture, and the
same objection `RFC 010` open question 1 raised against a dialog that appears
for some users and not others.

- **One link, in the layout component**, first in the DOM, before the navbar.
- **`href="#main"`, and `<main id="main" tabindex="-1">`.** The `tabindex` is
  load-bearing: a bare fragment jump moves the scroll but does not reliably
  move **focus**, which is the thing being fixed. **Measure that it does**,
  rather than trusting it.
- **Visually hidden until focused**, visible and legible when it is. **This is
  where it can go wrong**: a badly built visually-hidden style adds scroll
  width, which is exactly what `BROWSER-001` catches — expect the gate to tell
  you if you get it wrong, and do not use `display: none`, which removes it
  from the tab order and defeats the whole thing.
- **The copy goes through the message table** (`NFR-LANG-001`). One new key.

## 3. What this does and does not fix

**Fixes**: the reach, on every page, for every reader, with or without
scripting.

**Does not fix**: focus still begins on `body` after a navigation. **That is
correct and `NFR-A11Y-011` is written to allow it** — the browser places focus
at the document on every load, and `NFR-A11Y-002` explicitly excludes
navigation. **Do not add a script that focuses `main` on load**; it would take
focus from a reader who has already started moving, for a benefit the skip
link already provides.

**Also not in scope**: the lost scroll position on a phone. It is the same
mechanism and a different remedy, and `NFR-A11Y-011` does not require it.
**If you see a cheap way to preserve it, report it rather than doing it.**

## 4. Verification

- **Tab presses from a fresh load to the first control inside `main`**, on the
  pages `A11Y-001` measured at 11 — the interstitial, issue detail, the board,
  the sprint plan, sprint detail — and on a phone board at 10. **Report before
  and after.** Expect: one press to the skip link, activate, focus inside
  `main`.
- **Focus actually moves**, not just the scroll: `document.activeElement` after
  activating the link.
- **Scripting off**: the same, which is the reason for this shape.
- **Visible when focused**: report its box and what it reads at 320 px.
- **The five `autofocus` pages are unaffected** — on those, focus is already in
  the form and the skip link must not change that.
- **The overflow gate**: a new first element on **every** page at five widths.
  **Run it and report the cell count**; this is the change most likely to move
  it.
- **`touch_target`**: it is a link, and it is interactive when visible.
- `DEC-007` reported, last recorded **369**. `fmt`, `clippy`, three consecutive
  workspace runs. The i18n guards for the new key.

## 5. Escalate rather than deciding

- **If `tabindex="-1"` on `main` does not move focus** on the fragment jump —
  that is the assumption this shape rests on.
- **If the visually-hidden style moves the gate off its cell count** for a
  reason you cannot remove.
- **If a page has no `<main>`**, or more than one.
- **If the skip link interferes with the `autofocus` pages.**

## 6. Exit condition

One Tab press and one activation reaches `main` on every page, with scripting
on or off; focus is measurably inside `main` afterwards; the gate is green; the
`autofocus` pages are unchanged.
