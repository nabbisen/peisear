# `SIZE-001` round 2 — What a site is next to

**Round 1 is accepted.** Four sites raised, fourteen left **with a stated
reason each**, both comment sums re-derived to `32px + 20px = 52px`, census
`53 → 49`, `(289, 2)` held, CI 50/50, and a `CHANGELOG.md` entry that says
what was left as well as what moved. The two `<details>` were opened and
measured rather than reached by growing a fixture.

**Four items. One changes the tree; three correct reasons that are now
recorded in it.**

---

## 1. `sprints.rs:590` — read it where it sits

Left at 12 px as *"not stacked against a differently-sized sibling line"*.
**In place, it is.** It sits inside the same `<form>` as the Goal field,
directly below the hint `LBL-001` raised to 14 px and directly above the
submit button, so the form now renders **label 14 px, hint 14 px, notice
12 px, button**.

It is also the most consequential sentence on the screen — *the sprint will
be created in planned state; start it explicitly, the calendar dates don't
auto-promote* — and it is now smaller than the word *optional* three lines
above it. **That is the wrong text to leave smallest** against the standing
instruction that users must not be confused or misled.

**Raise it, measured at 390 px and 1280 px** (wrap count in the form card,
and whether the submit row moves), **or leave it with a reason that accounts
for its neighbours** — the second is still a legitimate outcome, but the
reason has to survive the markup around it.

**The generalizable point, which is why this is a finding and not a
preference**: *a site's role is decided by what it is next to.* That is a
reading of the markup, not a description of the role — and reading the
neighbours is exactly what got `139` and `212` right.

---

## 2. Re-read the other four Group C sites the same way

`404`, `416`, `982`, `987`, `1022` were grouped as "chart captions" by role.
**Re-read each in place** and report what is adjacent: the element above, the
element below, and their sizes. If one of them turns out to be stacked
against a `text-sm` sibling doing the same job — the `139`/`212` shape —
raise it. If all four are genuinely captions to a chart, say so with the
neighbours named and they stay. **No outcome is assumed here**; the method
is.

---

## 3. The precedent written into the tree is wrong — correct both comments

`sprints.rs:461–464` records the deciding reason as *"`settings.rs`, `me.rs`
set `text-sm` directly on their own `<summary>`; this file was the only one
sizing the ambient `<details>` instead"*, and its twin at `:1040` says the
same.

Measured across the tree: of **13** `grow_summary()` summaries, **4** set a
size directly (`settings.rs:146`, `settings.rs:299`, `me.rs:359`,
`me.rs:569`) and **9 set none**, inheriting their ancestor — and `issues.rs`
sizes **four** `<details>` containers at `text-sm` (`:329`, `:374`, `:649`,
`:726`), **which is exactly the shape this round produced.**

**The decision stands and is better supported than its stated reason.**
Correct both comments to the measured population: the product's dominant
pattern is an unsized `<summary>` inside a `text-sm` container, and
`sprints.rs` now matches `issues.rs` rather than being the only file of its
kind. Re-derive the figures yourself before writing them — this is the third
recorded reason in this series that was right in outcome and wrong in its
stated basis.

---

## 4. Two corrections to prose, one of which ships

- **`CHANGELOG.md`**: *"the stat-grid labels beneath each chart's numbers"* —
  those captions sit **above** their own numbers, in the sprint summary card,
  which is a sibling of the chart rather than beneath it. Released prose, so
  fix the sentence.
- **The `Docs` diagnosis in the next report**: run **`44281512009` (head
  `9202290`) is still `waiting`**, not cancelled, and is still holding
  `concurrency: group: "pages"`. `924b88f`'s run is cancelled by **ordinary
  supersession** — GitHub keeps only the newest *pending* run in a group, and
  `cancel-in-progress: false` does not change that. So this is **one
  anomaly, not two**, and the remedy is the single cancel the previous round
  already ranked first. **Take no action on the run**; the correction is to
  the reading, and it matters because "an undiagnosable second anomaly" and
  "the same wedged run" call for different things.

---

## 5. The question this round escalated, answered

*Should a chart caption move with the form hints, or stay with the stat
captions?* **Ruling: both sets stay at 12 px** — eight stat captions and five
chart captions, thirteen sites, all in `sprints.rs`.

The reason is the one this series was opened on rather than taste: **nothing
else in the product does either job at another size.** `me.rs`'s chips pair a
14 px label with a *badge*; these pair a caption with an 18 px
`tabular-nums` figure or with a chart. There is no second spelling of the
same job, so *two sizes for one job* does not reach them.

**Recorded as a decided set**, so no later round re-litigates it site by
site — and recorded with the condition that would reopen it: another screen
growing the same pattern at `text-sm`. **Group B's own framing should say
that too**: it is a first decision, stated, exactly as the chart captions
were — not a hierarchy fact settled elsewhere.

---

## 6. Acceptance

1. `sprints.rs:590` raised with measurements at both widths, or left with a
   neighbour-aware reason.
2. The four remaining Group C sites re-read in place, each with its
   neighbours and their sizes named.
3. Both `<details>` comments corrected to the measured precedent.
4. The `CHANGELOG.md` sentence fixed.
5. **Census re-derived**: **49** now, less whatever is raised.
6. `(289, 2)` holds; stop and report if it moves.
7. Gates: the `DEC-007` block with real exit codes (**687**), `fmt`,
   `clippy` both variants, `contrast_scan`, `touch_target_scan`,
   `alignment_scan`, `overflow-gate.mjs`, and
   `disclosure-marker-gate.mjs` **only if a `<summary>` or its ancestor
   moves again** — it passed this round and nothing here is expected to
   reach it.
8. Privacy floor per artefact, per `README.md §8.7`.
9. **Review request**:
   `.git-exclude/review-request/SIZE-001-round-2-what-a-site-is-next-to/README.md`.
