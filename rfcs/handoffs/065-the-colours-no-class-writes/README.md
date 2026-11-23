# 065 — The colours no class writes

**Series index.** `0.67.0`'s scope, authorized by the owner 2026-11-23 —
**the row, not the cut**. Opened the same day.

**`0.66.0` taught `contrast_scan` to read all four of the theme's colour
families. It reads only the classes a component writes.** Three kinds of
text get their colour from the served stylesheets instead — by element
selector, or baked into a component class.

**Corrected 2026-11-23, by the round that measured it.** This index first
said all three were below `NFR-A11Y-005`'s 4.5:1. **Two are** — the
seventeen placeholders at **2.54:1** and the account menu's own line at
**1.85:1**. The eight table headers read **4.53:1**, which *passes by
0.03*: all eight sit inside a `card bg-base-100`, including the two this
index offered as "page body" candidates. They are repaired anyway, because
**4.53:1 is the exact `/60` tier `NFR-A11Y-005` already rejected as a
floor** and it fails the moment a table renders on the page body — but that
is policy consistency on a fragile tier, not an active failure, and the
difference is the architect's to have stated correctly the first time.

---

## 1. What was measured, and how

Tokens resolved from the pinned sheets (`ASSET-001`, `DEC-051`), composited
over the product's own backgrounds, OKLCh → sRGB → relative luminance, with
the conversion checked against daisyUI's published hexes — the method
`0.66.0` used and this series inherits:

| what | where the colour comes from | resolved | `base-100` | `base-200` |
|---|---|---|---|---|
| form placeholders | `input::placeholder` in **both** served sheets | `#9ca3af` | **2.54:1** | *n/a — see below* |
| the account menu's email line | `.menu-title` = `base-content/40`, **plus** that site's own `opacity-70` | `#bebfc3` | **1.84:1** | — |
| every table header | `.table :where(thead, tfoot)` = `base-content/60` | `#74767f` | **4.53:1** | **4.25:1** |
| **the floor this project chose** | `base-content/70` | `#5d5f6a` | **6.34:1** | — |

**AA is 4.5:1 for normal text; 3:1 for large text** (≥24 px, or ≥18.66 px
bold). None of the three is large text.

**And a placeholder has no page background at all** — `PLC-001`'s own
finding, measured rather than argued: `.input`/`.textarea` set their own
`background-color` from `--b1` unconditionally, so the control paints white
whatever it sits on. Proved by sampling the lightest pixel inside a search
box that sits directly on the grey page body and getting pure white. The
`base-200` column above does not apply to placeholders. **After the repair
they read 6.42:1** — not the 6.34:1 this index first predicted; three
methods differ in the last decimal (composite 6.34, the requirement's own
record 6.36, the rendered pixels 6.42) and **the rendered pixels are what a
reader sees**.

Two facts about that table worth carrying rather than re-deriving:

- **The `/60` tier is the one `NFR-A11Y-005` already rejected.** Its own
  entry says: *"`/60` measured 4.54:1 on `base-100` — a pass by 0.04 —
  while failing at 4.23:1 on `base-200`"*. That is exactly what the sheet
  applies to every `thead`, and on `base-200` it fails by measurement.
- **The account menu's line is the worst text in the product**, and it is
  the reader's own email address.

**Re-derive before relying on any of this.** The command that matters is not
a grep: it is resolving the token and compositing it over the background the
element actually sits on. If your measurement disagrees, **yours wins** and
the disagreement is the finding — `0.66.0`'s rounds corrected five of the
architect's figures that way.

---

## 2. The populations, counted

| round | family | sites |
|---|---|---|
| `PLC-001` | form placeholders — `issues.rs` 4, `teams.rs` 4, `settings.rs` 3, `projects.rs` 2, `sprints.rs` 2, `search.rs` 1, `layout.rs` 1 | **17** |
| `PLC-002` | the account menu's email line (`layout.rs:383`) and every `<thead>` (8 sites across 11 tables) — **handed 2026-11-23**, sequenced after `PLC-001` since both edit `app.css` | **9** |
| `PLC-001` | **closed** 2026-11-23 (`2be1002`) — one `app.css` rule, all 17 reached, cascade planted; **6.42:1** after | 17 |
| `PLC-002` | **closed** 2026-11-23 (`4d4ed28`) — the account line to **6.42:1** from 1.85:1, the eight headers to 6.42:1 from 4.53:1, the `menu-title` alternative measured and rejected on hover semantics rather than on box size (which does not move) | 9 |
| `PLC-003` | the guard, `NFR-A11Y-005`'s second amendment in two releases, one register entry, and `app.css`'s own stale comment — **handed 2026-11-23** | — |

In the two served sheets, **109 selectors** set a text colour below the
`/70` floor. The product reaches **three** families today, which is why this
series is three families and not 109.

---

## 3. The cascade, which decides how every repair is written

**The sheets are served in this order** (`layout.rs:68–70`):
`daisyui.min.css` → `app.css` → **`tailwind.css` last**. And
`tailwind.css` carries `input::placeholder,textarea::placeholder{color:#9ca3af;opacity:1}`.

**So a rule added to `app.css` with the same selector loses** — later sheet,
equal specificity. An override must out-specify it. All 17 placeholder sites
carry `.input` or `.textarea` (35 and 9 class sites product-wide, checked),
so `.input::placeholder` / `.textarea::placeholder` — class plus
pseudo-element — wins regardless of sheet order.

**This is `§10.52`'s shape**, and that entry is why the rule here is
absolute: **measure the rendered colour in a browser.** Three review rounds,
a required prop and a compile-error plant all passed while the shipped
navigation computed `display: none` at every width, because every one of
them read Rust and none could see a stylesheet.

---

## 4. Method

1. **No new colour.** Every repair lands on `base-content/70`, the floor
   this project already measured and chose, so `contrast_scan`'s existing
   rule and this work agree by construction.
2. **No change to the theme.** `--bc` and the rest are shared with every
   passing surface; the repairs change what the product *does* with them.
3. **A class token must have a rule in one of the three served sheets** —
   the CSS is vendored and pinned, so an invented class resolves to nothing.
   `css_scan` enforces it. **A rule added to `app.css` is the exception
   this series creates deliberately**, and it is the project's own sheet:
   16 lines, two rules, no colours of its own until now. Say what you added
   and why, at the rule.
4. **Measure the rendered colour, not the declaration** (§3). Before and
   after, on the background the element actually sits on.
5. **Removal is on the table**, as always: dropping `.menu-title` from the
   one site that uses it is a legitimate repair and may be the better one.
6. **The guard grows once, in `PLC-003`**, and no round widens it on its own
   initiative.

---

## 5. Deliberately not in this series

- **The 21 remaining smallest-text sites**, the **nine** `link link-hover`
  sites, the **fifteen** bare-`opacity` sites, the **`xs` control tier** —
  recorded candidates, none of them a contrast failure.
- **The disabled tiers** — `.input:disabled` and friends at `/40`,
  `.menu li.disabled` at `/30`. **WCAG 1.4.3 exempts text that is part of an
  inactive user-interface component**, so they are named in `PLC-003`'s own
  exclusions rather than silently skipped.
- **`.stat-title`/`.stat-desc`** (`/60`): the product uses neither — **zero**
  sites, checked. Banned by the guard anyway, so the day a component writes
  one it fails rather than ships.
- **`/today`'s content** — the owner's own open question.

---

## 6. What each round owes

1. **Per site**: the rendered colour before and after, the background it
   sits on, and the ratio — measured in a browser.
2. **Every site in its family accounted for** — repaired, or left with a
   reason that survives its own measurement.
3. **Gates in full**, with real exit codes (`cmd | tail -1; echo $?` reports
   `tail`'s status): the `DEC-007` block (**689** as it stands — say what it
   became), `fmt`, `clippy` both feature variants, `contrast_scan`,
   `touch_target_scan`, `alignment_scan`, `css_scan`'s population
   (`(287, 3)` — re-measure, never assume), and the browser gates when
   markup or a served sheet changes.
4. **`CHANGELOG.md`** under `[Unreleased] → Fixed`: this is a defect, and
   the entry says what a reader could not read.
5. **The privacy floor, per artefact** — any screenshot inspected before it
   is attached, no local paths, hostnames or account names. **The account
   menu's own line is an e-mail address**: a screenshot of that repair must
   not carry a real one, so use the fixture's address and say which.
6. **Review request** at
   `.git-exclude/review-request/<round>-<slug>/README.md`.

---

## 7. Where things go

- **Handoffs** are published here, one file per round; **reviews stay
  private**.
- **Release numbers live in `ROADMAP.md` and nowhere else.**
- **No round tags, publishes or creates a release.** `DEC-056`'s four legs
  are the architect's, on the owner's approval at **exit criterion 7**.
