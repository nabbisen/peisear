# `EDG-001` — Where the keyboard is

**Round 1 of series `066`** (`0.68.0`, authorized 2026-11-24 — the row, not
the cut). Read `README.md` in this directory first: it carries the measured
tokens, the method, the cascade, the exclusions and the privacy floor.

**Scope: the focus ring on every text control — 67 sites. The boundary is
`EDG-002`'s; this round is only about what appears when a reader tabs.**

---

## 1. What is wrong

`daisyUI` draws the focus ring for a text control in `base-content/20`:

```css
.input:focus,.input:focus-within{ … outline-width:2px;outline-offset:2px;
                                  outline-color:oklch(var(--bc)/.2) }
.select:focus  { … same … }
.textarea:focus{ … same … }
```

Resolved and composited: **1.52:1** against a white card, **1.50:1** against
the grey page body. **WCAG 1.4.11 asks 3:1** for the visual information that
identifies a control's state, and a focus indicator is exactly that.

**So a keyboard reader cannot see where they are.** That is the whole
defect, and it is worth stating in those words rather than as a ratio: on
this product today, tabbing through a form moves an invisible cursor.

**The same sheet already knows better.** `.checkbox:focus-visible` draws its
ring in **full `base-content`** — `#181a2a`, **17.21:1** — and `.btn` and
`a` draw theirs in `currentColor`, which resolves to the text's own colour.
Three patterns in one stylesheet; the text controls got the weak one.

---

## 2. The 67 sites

Every `.input` (**35**), `.select` (**23**) and `.textarea` (**9**) site in
`components/**`. All of them also carry `-bordered`, so `EDG-002` will
revisit the same elements for the line around them — **do not touch the
border in this round**, even where it is the same declaration block.

Two sites are worth naming because they are the first and the most used:

- **`layout.rs:290`** — the header search box, on every authenticated page.
  It is hidden below `md` (the responsive design replaces it with the `Menu`
  dropdown), so measure it at **1280 px**; at 390 px there is nothing to
  see, which `PLC-001` already established and which is not a defect.
- **`auth.rs`'s sign-in fields** — the first controls a new reader ever
  focuses, on a page with no navigation to orient them.

---

## 3. The repair, and the one decision in it

**Recommended: one rule in `app.css`, the ring at full `base-content`** —
the tier daisyUI already uses for a checkbox:

```css
.input:focus,
.input:focus-within,
.select:focus,
.textarea:focus { outline-color: oklch(var(--bc) / 1); }
```

**The decision the round owns**: whether the ring should be full strength or
`/70` (**6.34:1**, the product's own text floor). Both clear 3:1 easily.
**Measure both and choose with the measurement in front of you**, then say
which and why. My recommendation is full strength, on the grounds in
`README.md §3.5` — a reader who cannot see where they are is worse served
than one who finds a strong ring heavy — and because it matches the
checkbox the product already ships.

**Do not change `outline-width` or `outline-offset`.** They are 2 px and
2 px, which is enough; this round changes a colour, and widening a ring is a
separate decision nobody has asked for.

**The cascade**: `daisyui.min.css` is served **first**, so an
equal-specificity rule in `app.css` wins on order — but `.input:focus` is a
class plus a pseudo-class, so the override must match that, not just the
class. **Plant it**: comment the rule out in a scratch worktree, confirm the
ring returns to `/20`, restore. A rule believed is worth less than a rule
watched winning.

---

## 4. What to measure

- **The ring itself, sampled from the rendered page**, not computed from the
  declaration: focus the control with a real `Tab` key event, screenshot,
  and read the ring's own pixels against what sits outside it. Before and
  after, at **1280 px**, for at least one `input`, one `select` and one
  `textarea`.
- **The adjacent colour is what the ring sits on, and `outline-offset: 2px`
  puts it outside the control** — so on a card it is measured against the
  card, and on the page body against the page. Name which for each site you
  sample.
- **`:focus-within` is in that selector list for a reason.** `.input` can
  wrap a child input in daisyUI's own markup; confirm whether any site in
  this product relies on it, and if none does, say so rather than leaving
  the selector unexplained.
- **That nothing else moved**: the control's own box, its border colour (that
  is `EDG-002`'s), and the page's layout at both widths.
- **This environment cannot produce `:hover`** (`CLR-001`'s finding,
  verified by the architect: `(hover: hover)` is false here). Focus is
  different — a real `Tab` keypress works — but if you find yourself needing
  a hover state, read the declaration and say so rather than forcing a
  pseudo-state.

---

## 5. Acceptance

1. **67 sites covered by the rule**, confirmed by rendering at least one of
   each kind rather than by the class list.
2. **Before/after ratios sampled**, with the adjacent colour named per site.
3. **The tier decided with a measurement** and the choice stated.
4. **The cascade planted**, with the failure quoted.
5. **`css_scan` `(287, 3)`** — verify; `app.css` is outside that scan's
   scope and no component class changes.
6. **The gate line in full** per `README.md §5`, including the `DEC-007`
   block (**691** as it stands) with real exit codes, and
   `overflow-gate.mjs` **because a served stylesheet changes**.
7. **`CHANGELOG.md`** under `[Unreleased] → Fixed`, saying what a reader
   could not see.
8. **Privacy floor per artefact**; fixture data only in any screenshot, and
   say which.
9. **Re-derive the `rfcs/` citation count after your last edit and before
   pushing** — `README.md §5.7`.
10. **Review request** at
    `.git-exclude/review-request/EDG-001-where-the-keyboard-is/README.md`.

**If a measurement contradicts this file, the measurement wins.** Eleven of
the architect's figures or instructions have been corrected that way across
the last three series, and every correction improved the result.
