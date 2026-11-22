# `PLC-002` — Your own address, and every table header

**Round 2 of series `065`.** Read `README.md` in this directory first.

**Sequencing**: this round and `PLC-001` both edit `app.css`. **Start after
`PLC-001`'s commit is on `main`** and rebase onto it; if you begin sooner,
do the `layout.rs` site first (it needs no stylesheet change) and leave the
table-header rule until that commit lands. Say which you did.

**Scope: one line in the account menu, and eight table headers.**

---

## 1. The account menu's own line — the worst text in the product

`layout.rs:383`:

```rust
<li class="menu-title w-full min-w-0 overflow-hidden">
  <span class="text-xs opacity-70 truncate">{user.email}</span></li>
```

**Two fades, multiplied.** daisyUI's `.menu-title` sets
`color: oklch(var(--bc)/.4)` (read from the pinned sheet, together with its
padding and `font-weight:700`), and the span inside adds a bare
`opacity-70` on top: **0.40 × 0.70**, resolving to `#bebfc3` — **1.84:1**
against the white menu surface. **It is the reader's own e-mail address**,
and it is the lowest-contrast text in the product.

**Recommended repair: give the span an explicit `text-base-content/70` and
drop the `opacity-70`.** That is a component class, so `contrast_scan`
already governs it, and the span's own colour beats the `<li>`'s inherited
`/40` — no stylesheet rule needed for this site. Expected **6.34:1**.

**The alternative is on the table and must be measured, not dismissed**:
drop `menu-title` entirely. It carries padding and `font-weight:700` as well
as the colour, so losing it changes the row's shape — measure what moves at
390 px and 1280 px, in both the closed and open states of that dropdown,
before preferring it. `TT-004`'s own comment two lines below warns that
daisyUI's `.menu` selector is doing layout work here.

**Do not change what the line shows.** Whether the account menu should
display a full e-mail address at all is a different question from its
contrast; if you think it should not, say so as a finding.

---

## 2. Eight table headers at a tier this requirement already rejected

`.table :where(thead, tfoot)` in `daisyui.min.css` sets
`color: oklch(var(--bc)/.6)` — **4.53:1** on `base-100`, **4.25:1** on
`base-200`. `NFR-A11Y-005`'s own entry rejected exactly this tier when it
chose the floor: *"`/60` measured 4.54:1 on `base-100` — a pass by 0.04 —
while failing at 4.23:1 on `base-200`"*. **On the page body it fails.**

The eight sites: `notification_preferences.rs:141`, `teams.rs:457`,
`settings.rs:154`, `sprints.rs:494`, `sprints.rs:1096`, `sprints.rs:1167`,
`issues.rs:1321`, `issues.rs:2519`.

**Recommended repair: one rule in `app.css`**, at the project's own floor:

```css
.table :where(thead, tfoot) { color: oklch(var(--bc) / 0.7); }
```

**The cascade favours you here, and for a different reason than
`PLC-001`'s.** That rule lives in `daisyui.min.css`, which is served
**first** (`layout.rs:68–70`), so an equal-specificity rule in `app.css`
wins on order — the opposite of the placeholder case, where `tailwind.css`
is served last and had to be out-specified. **Verify it in a browser
anyway**; the two cases differ, and `§10.52` is why neither is believed
from the file.

Expected after: `base-content/70` — **6.36:1** on `base-100`, **5.76:1** on
`base-200**, **5.15:1** on `base-300`, the figures `NFR-A11Y-005` already
records for its own floor. Report what you measure, per background.

**Not in scope, and worth naming in the report**: that same rule sets
`font-size: .75rem; font-weight: 700`, so every table header is **12 px
bold**. That is a type question, and the 21 remaining smallest-text sites
are a recorded candidate for a later row — **do not change the size here**,
and if you think a 12 px bold header deserves its own look, say so as a
finding.

---

## 3. What to measure

- **Rendered colour before and after**, per site family, on the background
  it actually sits on: the account menu's surface, a card table
  (`settings.rs:154`) and a table on the page body (`sprints.rs:494` or
  `issues.rs:1321`) — check which background each one really has rather
  than assuming, as `CLR-001` had to.
- **Both widths (390 px, 1280 px)** for the account-menu line, in the
  dropdown's closed and open states, since `truncate` and `overflow-hidden`
  are doing work there.
- **That the `<li>`'s own `/40` no longer reaches any text**: with the span
  carrying its own colour, nothing else in that element renders text —
  confirm by rendering, not by reading.
- **`css_scan`'s population**: the span gains `text-base-content/70`, a
  token the product already uses in many places, so `(287, 3)` should hold.
  **Verify.** The `app.css` rule adds no class token at all.
- **The eight headers' own `aria-label`led tables still read correctly** —
  a colour change cannot break that, so this is a one-line confirmation,
  not an investigation.

---

## 4. Acceptance

1. **Nine sites accounted for** (one menu line, eight headers), each with
   before/after ratios and the background named.
2. **The account-menu repair chosen between the two options with a
   measurement**, not a preference — and if `menu-title` is dropped, the
   layout cost measured at both widths and both dropdown states.
3. **The cascade verified in a browser** for the `app.css` rule: the
   computed header colour is the project's and not daisyUI's. **Plant it if
   you want certainty** — comment the rule out in a scratch copy, confirm
   `/60` returns, restore.
4. **`(287, 3)`** verified, not assumed.
5. **The gate line in full** per `README.md §6`, including the `DEC-007`
   block with real exit codes, and the browser gates — **a served
   stylesheet changes again**, so `overflow-gate.mjs` runs and reports its
   cell count; `disclosure-marker-gate.mjs` too if any `<summary>` ancestor
   moves.
6. **`CHANGELOG.md`** under `[Unreleased] → Fixed`, in the same entry family
   `PLC-001` started.
7. **Privacy floor per artefact, and this round needs it most**: the site
   in §1 renders an e-mail address. **Any screenshot must carry the
   fixture's address, and the report must say which.** Scratch under
   `.git-exclude/tmp/`, deleted when the round closes.
8. **Review request** at
   `.git-exclude/review-request/PLC-002-your-own-address-and-every-table-header/README.md`.

**If a measurement contradicts this file, the measurement wins** — including
the two cascade claims, which differ from each other on purpose.
