# LAYOUT-001 — every page scrolls sideways by 17 px

**Target release**: 0.32.0
**Source**: `.git-exclude/tasks/architect/013-browser-inspection-findings.md` §0
**Governing RFC**: none. This is a defect fix, not RFC work.

## 1. The defect

**Every authenticated page has ~17 px of horizontal overflow at every viewport
width.** Measured on a running instance:

| Viewport | `clientWidth` | `scrollWidth` | Overflow |
|---|---|---|---|
| 390 | 390 | 407 | **+17** |
| 1280 | 1265 | 1282 | **+17** |
| 1920 | 1905 | 1922 | **+17** |

The user-visible symptom is a horizontal scrollbar on every page and a document
that drags 17 px sideways. **It has shipped in every release.**

## 2. The cause, as far as I traced it

`components/layout.rs:175` — the account dropdown's menu:

```
<div class="dropdown dropdown-end">
    <label tabindex="0" class=grow("btn btn-ghost btn-sm normal-case")> … </label>
    <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-48 border border-base-300">
```

Measured while **closed**, that `<ul>` computes `position: absolute`,
`visibility: hidden`, `display: flex` — and its right edge lands past the
viewport.

**`visibility: hidden` does not remove an element from layout.** It hides it
while it continues to occupy space and continues to contribute to the document's
scroll extent. `display: none` would; `visibility` does not.

**I have not established why it sits where it does.** `dropdown-end` should
right-align it, and the measurements suggest it may not be taking effect —
the menu appears to extend rightward from near the toggle rather than ending at
it. **Establish that before changing anything**: if the real fault is that
`dropdown-end` is inert here, the fix is different and better.

## 3. The trap, and it is the reason this is a handoff and not a one-line commit

**The menu opens on focus.** Both the toggle and the `<ul>` carry `tabindex="0"`,
and DaisyUI's dropdown reveals its content on `:focus` / `:focus-within`.

**An element with `display: none` cannot receive focus and its links cannot be
tabbed to.** So the obvious fix — hide the closed menu with `display: none` —
risks breaking keyboard operation of the account menu, which is `NFR-A11Y-001`
and one of the "Reach" conditions in the Definition of Done.

Whatever you change, **the menu must still open and be operable by keyboard
alone**, and the package must show that it does. A fix that removes a scrollbar and
takes the keyboard path with it is a worse defect than the one it fixes.

## 4. Verifying it — and you can, now

`.git-exclude/tools/cdp.mjs` drives headless Chromium over CDP from Node with
**no dependency**; its README has the exact invocation and the overflow check.
Chromium and Node are already installed. This is how the defect was found.

**Required evidence:**

1. **Overflow is gone** — `scrollWidth - clientWidth` is 0 at **390, 768, 1280
   and 1920**, on `/today`, `/inbox`, `/today/calendar`, a project detail page
   and an issue detail page. Report the numbers, not a summary.
2. **The menu still opens by keyboard** — tab to the toggle, activate it, and
   show that the menu's links are reachable and that focus enters them. Report
   how you drove it.
3. **The menu still opens by pointer.**
4. **No other page regressed** — the same overflow check on the pages above
   before and after.

**This is a one-off inspection, not a new gate.** Do not add a browser step to
CI, do not add a dependency to any `Cargo.toml`, and do not commit anything
under `.git-exclude/`. Whether any of this belongs in CI is RFC 011 step 4's
open question and is not yours to pre-empt.

## 5. Scope

**This defect only.** The same inspection found three other things — `join`
buttons overlapping by 1 px, sub-44 px `<summary>` toggles, and the named limit
covering more than we recorded. **None of them is in this handoff.** They are
requirement-level questions and they are mine.

## 6. Escalate rather than deciding

- **If the cause is not what §2 says.** I traced it far enough to be confident
  the closed dropdown is what overflows, and not far enough to be confident why
  it sits there. If `dropdown-end` turns out to be inert, or the real cause is
  elsewhere, that is a finding and it changes the fix.
- **If no fix exists that keeps both the scrollbar gone and the keyboard path
  working** without restructuring the navbar. That is a design question, not an
  implementation one.
- **If the fix would change how the menu looks or animates.** The transition is
  visible behaviour.

## 7. Exit condition

Overflow zero at four widths on five pages, keyboard and pointer operation
demonstrated, `DEC-007` clean, three consecutive `cargo test --workspace` runs.

**No new test is required.** Nothing in the suite can observe this, which is the
point of `§10.18` — recorded separately, and not this handoff's to close.

---

**Who holds what**: dev team — the fix. **What's blocked**: nothing. **What's
next**: review request.
