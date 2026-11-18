# `SIZE-001` — The sprint screens' own small print

**Round 2 of series `063`.** `LBL-001` is **closed** (accepted over two
rounds). Read `README.md` in this directory first: it carries the census
command, the method, the stylesheet facts, the two traps and the privacy
floor, and this file does not repeat them.

**Scope: `sprints.rs`'s 18 remaining `text-xs` render sites, and the two
comment sums that go stale the moment two of them move.** No other file.

---

## 1. The 18 sites, in three groups

**Group A — two disclosure containers** (the trap this round owns):

| site | markup |
|---|---|
| `sprints.rs:434` | `<details class="text-xs mt-2">` — the velocity chart's data table |
| `sprints.rs:1000` | `<details class="text-xs mt-2">` — the same shape on the other surface |

**Group B — eight stat-block labels**, in a
`grid grid-cols-2 sm:grid-cols-4 gap-3` inside a `card bg-base-100`:
`798`, `802`, `807`, `811`, `816`, `820`, `826`, `830`. Each is a label
above a `text-lg` value — so the reader's eye goes to the number and the
word naming it is the smallest text on the card.

**Group C — eight paragraphs and one flex row**: `139`, `212`, `383`,
`395`, `556`, `948`, `953`, `988`. All `text-base-content/70`.

Every one of the 18 carries `text-base-content/70` **except** Group A's
two, which are bare `text-xs mt-2`. **No bare `opacity-*` anywhere in this
round's population** — so the contrast obligation here is lighter than
`LBL-001`'s, and what it needs is a measurement per *background*, not per
site (see §3).

---

## 2. Group A is this series' own named trap, and it fires this round

Both `<details>` are the **ambient size** for a `<summary>` whose own comment
states its touch-target arithmetic:

```
// MARK-002 (§10.58): grow_summary(), the only helper
// a <summary> may take -- py-4 = 32px + text-xs's own
// 16px line-height = 48px.
```

at `sprints.rs:436–438` and `sprints.rs:1002–1004`. Raising the container to
`text-sm` makes that sum **`32px + 20px = 52px`**, and the comments must be
re-derived with it — **the last two `text-xs`-keyed sums in the product**
(`§10.60`'s own finding, and `0.64.0` corrected the other three).

**What is not at risk, and must not be "fixed":** the 44 px floor itself.
`grow_summary()` composes `TOUCH_TARGET` (`min-h-11` = 44 px) and
`alignment_scan` enforces that every real `<summary>` reaches that one
helper. The arithmetic is explanatory; **do not add a mechanism to hold a
property that is already held**, and do not touch
`alignment_scan.rs`'s `NAMED_ESCALATION_EXCLUSION_TAG` or the one bare
`<summary>` it names (`issues.rs`, outside this round).

**If a container is left at `text-xs`**, say so and leave its comment
correct — that is a legitimate outcome, and `<details>` carrying a size for
its whole subtree is a fair reason to be careful: raising it moves the
summary **and** the table inside it.

---

## 3. What to measure, and where it is likely to cost something

- **Group A**: the `<summary>`'s own rendered box and line-height, the
  `table table-sm` inside each `<details>` **open**, and whether that table
  gains a horizontal scroll at 390 px (it already sits in
  `overflow-x-auto`, so the question is whether the scrollbar appears where
  it did not before). Both widths, open and closed.
- **Group B**: the stat grid is **2 columns at 390 px** and 4 at `sm` and
  above. Measure the card's height at both, and whether any label wraps to
  two lines — a wrapped stat label beside an unwrapped one is the visible
  cost to weigh, not the +4 px itself.
- **Group C**: a paragraph's own wrap count at 390 px, per site. `953`
  renders `{summary_text}` — **variable-length content**, so measure it with
  the longest string the fixture can produce rather than the default one,
  and say which you used.
- **Contrast**: all 18 are `text-base-content/70` or inherit from an
  untinted card, and `§10.60` measured that spelling at **6.32:1** untinted
  once raised. **Measure one representative per distinct background** (the
  `bg-base-100` card, and anything in Group C that sits on a tinted or
  `bg-base-200` surface — check rather than assume), and report which
  backgrounds you found.

---

## 4. Out of bounds

- **`teams.rs` and `sprint_plan.rs`** — `SIZE-002`'s, with the
  `overflow-gate.mjs:538` coupling.
- **The stat value's unit suffix** (`text-sm font-normal opacity-70`, beside
  each `text-lg` number): it is **not** in this round's `text-xs`
  population, and it is one of the sixteen bare-`opacity` sites now recorded
  as a candidate set in `ROADMAP.md`. **Leave it**, including its spelling.
- **`btn-xs` and `select-xs`** — the control tier, a recorded candidate,
  decided once and not by a size round (`LBL-001` round 2 settled how that
  is handled: state, do not raise).
- **Any new rule, floor or scale.** `§10.60` stays closed.

---

## 5. Acceptance

1. **All 18 sites accounted for** — changed with measured before/after, or
   unchanged with the reason.
2. **Both Group A comments re-derived** if their containers moved, and left
   exactly as they are if they did not. A comment that disagrees with its
   own element is the defect this round exists to avoid repeating.
3. **Costs at 390 px and 1280 px**, per group, including each `<details>`
   measured **open**.
4. **Contrast per distinct background**, with the background named.
5. **Census re-derived** with `README.md §2`'s command: it should fall from
   **53** by however many sites you raise, and your report says the new
   number.
6. **`css_scan`'s population**: `text-sm` and `text-xs` both stay in use, so
   `(289, 2)` should hold. If it moves, stop and report rather than updating
   the number.
7. **Gates**: the `DEC-007` block with real exit codes (count stands at
   **687**), `fmt`, `clippy` both feature variants, `contrast_scan`,
   `touch_target_scan`, `alignment_scan`, and the browser checks — markup
   will have moved, and `disclosure-marker-gate.mjs` reads `<summary>`
   markers, so run it rather than reasoning about it.
8. **Privacy floor per artefact**, per `README.md §8.7`.
9. **Review request**:
   `.git-exclude/review-request/SIZE-001-the-sprint-screens-own-small-print/README.md`.

**If a measurement contradicts this file, the measurement wins.** `LBL-001`
corrected its own handoff twice — once on a cost that could not happen, once
on line numbers that had shifted — and both corrections improved the result.
