# `SIZE-002` — The team screens, and the gate that quotes a class

**`SIZE-001` is closed** (accepted over two rounds: five sites raised,
thirteen left as a decided set, both comment sums re-derived, census
`53 → 48`). Read `README.md` in this directory first.

**Scope: `teams.rs`'s 7 and `sprint_plan.rs`'s 2 remaining `text-xs`
sites — nine — plus two carried corrections and one coupling to delete.**

---

## 0. Two corrections carried from `SIZE-001`, before the new work

**0.1 — a precedent sentence in the tree is wrong, and it was the
architect's.** `sprints.rs:466–467` says `issues.rs` sizes four `<details>`
at `text-sm` and calls all four *"exactly this shape, unsized summary inside
a sized container."* **`issues.rs:649` is not that shape**: its container
holds the product's one **bare** `<summary>`, the site `alignment_scan`
excludes by its exact literal tag, and it is not one of the thirteen
`grow_summary()` summaries at all.

Correct the sentence to the measured breakdown — and **verify each member by
reading inside its container rather than copying this list**, which is the
whole point of the correction:

- four `issues.rs` `<details>` at `text-sm`: `:329`, `:374`, `:649`, `:726`;
- **three** of them wrap an unsized `grow_summary()` summary (`:338`,
  `:388`, `:731`); `:649` wraps the bare one;
- so of the nine unsized summaries, **five** sit in a sized container:
  `sprints.rs` ×2 and `issues.rs` ×3.

**A precedent is a population, and a population needs its members read, not
tallied.** The round-1 claim was wrong, the round-2 claim replaced it with a
better-measured one that still carried this member, and the error entered
through the handoff. One clause, one place — the burndown twin carries only
the arithmetic and needs nothing.

**0.2 — report your own gate line again.** `SIZE-001` round 2 reported
`CI: success, 50/50` in place of the acceptance list's own items. Reinstated
as standing: the **re-derived census**, the **`(289, 2)` confirmation**, and
the **gate line** — the `DEC-007` block's count read with real exit codes,
`fmt`, `clippy` both feature variants, `contrast_scan`,
`touch_target_scan`, `alignment_scan`, `overflow-gate.mjs`. *CI is green*
is a state of a service; *the block ran and the count is N* is this round's
own work, and `DEC-007` exists because the second cannot be inferred from
the first.

---

## 1. The nine sites

**`teams.rs` — 7:**

| site | what it is |
|---|---|
| `69` | a paragraph under the page's own intro |
| `229` | a paragraph in the team-creation area |
| `311` | the *by email* hint beside an invite control |
| `338` | a paragraph after a form block |
| `419` | an italic note, `mt-4` |
| `577` | the **(You)** suffix after a member's own name in the table — `text-xs opacity-70 ml-1` |
| `641` | a paragraph in the lower section |

**`sprint_plan.rs` — 2:**

| site | what it is |
|---|---|
| `423` | a backlog row's meta line — `flex items-center gap-2 … mt-0.5` |
| `532` | a row's own points figure — `tabular-nums … mt-0.5` |

**Read each by its neighbours, not by its role name.** That method is what
`SIZE-001` round 2 established and what caught `590`: name the element
above, the element below, and their sizes, and decide from that. Report the
neighbours whether the site moves or not.

**Two sites need more than a size decision:**

- **`577`** is the series' last in-scope **bare `opacity-*`** site. If you
  raise it, the spelling question comes with it: `text-base-content/70` is
  what `contrast_scan` can read and what the product spells 155 times
  against 16. **Apply `§10.60`'s own condition** — single level, no faded
  ancestor — or leave the spelling and say why. Leaving the *size* is also
  an outcome: it is a parenthetical marker beside a name, in a table cell
  whose sibling cell renders at `text-sm`.
- **`532`** is a figure, not prose. `tabular-nums` exists to keep digits in
  column; measure whether a step up breaks the column's alignment with the
  rows above and below it, not only whether the row grows.

---

## 2. The coupling to delete

`browser-checks/overflow-gate.mjs:538` validates that the board fixture
rendered a workload chip by quoting the literal `text-sm font-medium`:

```js
const stripChips = [...boardHtml.matchAll(new RegExp(`<span[^>]*text-sm font-medium[^>]*>\\s*${nameRe}\\s*</span>`, 'g'))].length;
```

That literal **already broke once**, in `0.64.0`, on a round that never
touched the gate — and the class it now quotes matches **seven** sites in
`issues.rs`, so it is both coupled and less specific than when written.

**Delete the coupling rather than checking it**: give the chip
(`issues.rs:717`) a **named hook** and have the gate match that. `css_scan`
already tracks named JavaScript hooks, so the hook itself is checkable, and
no size round can break it again. **No new scan, and no retry logic.**

**Editing a gate means running that gate** — `overflow-gate.mjs` is 120
cells and it is this round's own evidence, not an optional extra. Say what
it reported.

---

## 3. Out of bounds

- `settings.rs`'s 2 and `auth.rs`'s 3 remaining sites — `SIZE-003`'s.
- `calendar.rs`, both notification surfaces, `search.rs`, `layout.rs` — the
  21 the row left out, a later decision.
- The thirteen `sprints.rs` captions — **decided**: they stay at 12 px, and
  the condition that would reopen them is another screen growing the same
  pattern at `text-sm`. Do not revisit them site by site.
- `btn-xs`/`select-xs` — the control tier, a recorded candidate.
- The other fifteen bare-`opacity` sites outside this round's own population.
- Any rule, floor or scale. `§10.60` stays closed.

---

## 4. Acceptance

1. **Nine sites accounted for**, each with its neighbours named and their
   sizes, changed with measured before/after at **390 px and 1280 px** or
   left with a reason that survives those neighbours.
2. **`577`'s spelling** decided explicitly, with `§10.60`'s condition
   checked if it converts.
3. **`532`'s column alignment** measured, not reasoned about.
4. **The gate coupling deleted**, the hook named, and
   `overflow-gate.mjs` run with its cell count reported.
5. **`sprints.rs:466–467` corrected**, with each member verified by reading
   inside its container.
6. **Census re-derived** — **48** now, less whatever is raised.
7. **`(289, 2)`** confirmed, or, if a token leaves `components/` (a named
   hook may add one), updated **with the reason** as `LBL-001` did.
8. **The gate line**, per §0.2, including the `DEC-007` count (**687** as it
   stands) with real exit codes.
9. **Privacy floor per artefact**, per `README.md §8.7`.
10. **Review request**:
    `.git-exclude/review-request/SIZE-002-the-team-screens-and-the-gate-that-quotes-a-class/README.md`.

**If a measurement contradicts this file, the measurement wins.** Three of
this series' recorded reasons have been wrong so far and two of them were
the architect's; the rounds caught them by re-deriving rather than by
trusting.
