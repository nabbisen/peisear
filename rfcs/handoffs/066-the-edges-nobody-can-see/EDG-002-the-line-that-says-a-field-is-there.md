# `EDG-002` — The line that says a field is there

**Round 2 of series `066`.** `EDG-001` is **accepted**: the focus ring on
all 67 text controls now draws at full `base-content` (**17.21:1** on a
card, 14.05:1 on the page body), the losing candidate was measured and its
figures recorded, and the border was left exactly alone — which is this
round's work.

**Verified by the architect in a browser, not taken on trust**: serving the
three real stylesheets with a one-element page and reading the computed
values gave `outline-color: oklch(0.223899 0.031305 278.072)` (the override
winning) and `border-color: oklch(… / 0.2)` — **still 1.52:1**, exactly what
is left to repair.

**Scope: the boundary on 70 controls. Nothing else.**

---

## 1. What is wrong

```css
.input-bordered   { border-color: oklch(var(--bc)/.2) }
.select-bordered  { border-color: oklch(var(--bc)/.2) }
.textarea-bordered{ border-color: oklch(var(--bc)/.2) }
.checkbox         { border-color: oklch(var(--bc)/.2) }   /* --tw-border-opacity: 0.2 */
```

Resolved: **1.52:1** on a white card, **1.50:1** on the page body, against
WCAG 1.4.11's **3:1** for the visual information that identifies a control.

**And the fill does not rescue it.** These controls paint `base-100`, so a
field on a white card is **1.00:1** against its own surround and **1.23:1**
on the grey page body. **On a card, that 1.52:1 line is the only thing that
says a field is there.**

Also true, and the reason this round is not simply "do what `EDG-001` did":
`.input:focus` *also* sets `border-color` to the same `/20`. **A focused
field now has a strong ring and a faint border**, which is coherent but
worth seeing before deciding whether the focused border should move too.

---

## 2. The 70

The same 67 `EDG-001` measured — `.input` 35, `.select` 23, `.textarea` 9,
every one carrying `-bordered` — **plus `.checkbox` ×3**
(`notification_preferences.rs:208`, `:215`, `:222`), whose border is the
same `/20` although its focus ring was already strong.

---

## 3. The repair, and the two decisions in it

**Recommended: one rule in `app.css` at `base-content/50`** — the first tier
that clears 3:1 on both backgrounds (**3.34:1** on `base-100`, **3.16:1** on
`base-200`), measured in this series' own index:

```css
.input-bordered,
.select-bordered,
.textarea-bordered,
.checkbox { border-color: oklch(var(--bc) / 0.5); }
```

**Decision one: the tier.** `/50` is the minimum that passes; `/70` (the
text floor, **6.34:1**) is the product's own other established tier.
`EDG-001` chose maximum strength for a *ring* because contrast did not
decide it and visibility did. **A border is not a ring** — it is present at
rest on every form in the product, so weight matters here in a way it did
not there. **Measure both and choose with the screenshots in front of
you**, and say what you saw, not just which you picked.

**Decision two: the focused border.** `.input:focus` sets `border-color`
to `/20` as well as the outline. Options: leave it (the ring now carries
focus, and the resting border rule above will cover the unfocused state),
or raise it with the rest. **Measure a focused field at both tiers** and
report whether a faint border inside a strong ring reads as deliberate or
as a bug.

**The cascade is the direction `EDG-001` proved**: `daisyui.min.css` is
served first, so an equal-specificity `app.css` rule wins on order —
`.input-bordered` is a single class, so a single-class override matches.
**`.checkbox` sets its colour through `--tw-border-opacity`**, so confirm
your override actually lands on it rather than being re-multiplied by that
variable; that is the one site in this round where the mechanism differs.
**Plant the rule either way**: comment it out, confirm `/20` returns,
restore.

---

## 4. What to measure

- **The border's rendered colour, sampled from the page**, before and after,
  at both widths, for at least one of each kind **and the checkbox**, with
  the adjacent colour named per site.
- **At least one site on the page body** — `search.rs`'s own inline search
  form is the one genuine non-card site this series has found twice.
- **A focused field**, for decision two.
- **A checkbox in both states** (checked uses `--chkbg`, full
  `base-content`, which already passes — confirm your change does not touch
  it).
- **That no layout moved**: a border colour cannot change geometry, so if
  something shifts, something else changed.

---

## 5. Out of scope

- **Card and alert borders** (`border-base-300` ~1.3:1, `border-error/30`,
  `hover:border-primary`) — decoration beside a shadow or a filled surface,
  not the sole identifier of a control. `README.md §4` has the reasoning; if
  you disagree for a specific site, say so as a finding rather than widening
  the round.
- **`outline-width`/`outline-offset`**, the ring's colour (`EDG-001`'s,
  closed), the guard (`EDG-003`'s), and everything in the four recorded
  candidates.

---

## 6. Acceptance

1. **70 sites covered**, confirmed by rendering one of each kind plus the
   checkbox, not by the class list.
2. **Both decisions made with measurements**, and what you saw stated.
3. **The `--tw-border-opacity` mechanism checked** for `.checkbox`.
4. **The cascade planted**, failure quoted.
5. **`css_scan` `(287, 3)`** verified.
6. **The gate line in full** per `README.md §5`, including the `DEC-007`
   block (**691**) with real exit codes and `overflow-gate.mjs` **because a
   served stylesheet changes**.
7. **`CHANGELOG.md`** under `[Unreleased] → Fixed`, in the same entry family
   `EDG-001` opened — and say what a reader could not see, not a ratio.
8. **Privacy floor per artefact**; fixture data only, and say which.
9. **Re-derive the `rfcs/` citation count after your last edit, before
   pushing** (**5** today).
10. **Review request** at
    `.git-exclude/review-request/EDG-002-the-line-that-says-a-field-is-there/README.md`.

**If a measurement contradicts this file, the measurement wins.**
