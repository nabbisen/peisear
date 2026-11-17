# TYPE-002 — the first screen after signing in

**Series:** [062 — the smallest text on the screen](./README.md)
**Release:** `0.64.0`, authorized 2026-11-17 — the row, not the cut
**Blocked on:** `TYPE-001`, closed across two rounds 2026-11-17 and
accepted in full.
**Register entry:** `§10.60`, still **open**

`TYPE-001` read the six sites *below* the scale. **This round reads the
scale's own floor on one screen** — `/today`, the first thing a reader
sees after signing in, and the screen the owner's question of
2026-11-04 named first.

**Decide by class, not site by site.** Twenty-one sites and twenty-one
separate judgements is not a reading of a screen; it is a list. The
classes are in §4, and a decision is owed per class with its own
measured cost.

**This round does not adopt a type floor.** That is still `TYPE-003`'s,
after this one.

---

## §1 The population — 21 render sites, all in `me.rs`

```bash
grep -rn 'text-xs' crates/peisear-web/src/components/me.rs | grep -vE ':[[:space:]]*//'
```

**21.** The `grep -vE` is not optional: a plain `grep -c` returns 23,
because two comments in this file *mention* `text-xs` while rendering
nothing. That difference is how the architect's own first figures were
wrong, and it is written up in `§10.60` and in the series `README.md`.
**Re-derive with the command above and report your own number.**

| class | n | sites |
|---|---|---|
| **Section headings** — `text-xs uppercase tracking-wide` | 4 | 287, **332**, 518, 552 |
| **Chip labels** — the word beside a number | 11 | 295, 304, 340, 348, 358, 533, 543, 597, 635, 663, 696 |
| **Italic explanatory lines** | 3 | 307, 559, 563 |
| **Value / sample lines** | 2 | 640, 700 |
| **A container** — `<section class="text-xs …">` | 1 | **367** |

Two of these are marked, and §2 and §3 say why. **Read both before
touching anything.**

**Two chip labels are rendered from two code paths each.**
`PaceDriftChipLabel` at 597 and 635, `SwitchingChipLabel` at 663 and
696 — two branches of the same panel. A decision about the chip-label
class must reach **both** halves of each pair or neither; `GRAM-002`
found a duplicate path the same way and the handoff that missed it was
the architect's.

## §2 The constraint to read first — four 44 px controls are built on `text-xs`'s line-height

**This is the thing that makes this round more than a font-size
change, and neither `TYPE-001` round could have known it.**

Four `<summary>` controls in this codebase reach `NFR-A11Y-007`'s
**44 px** touch-target floor by an arithmetic that depends on
**`text-xs`'s own 16 px line-height**, and each one writes that sum into
a comment beside itself:

| site | the recorded sum | where `text-xs` comes from |
|---|---|---|
| `me.rs:332` | `py-4` (32) + 16 = **48** | the summary's own class — **in this round's population** |
| `me.rs:373` | `py-4` (32) + 16 = **48** | **ambient**, from `<section class="text-xs">` at **367** — in this round's population |
| `issues.rs:350` | `py-4` (32) = **48**, and *"`py-3` (24px) would total only 40px here"* | `<details class="text-xs">` at 342 |
| `sprints.rs:438` and `:985` | `py-4` (32) + `text-xs`'s 16 = **48** | `<details class="text-xs">` at 434 and 981 |

**Two of the four are inside this round's twenty-one.**

**The direction of risk is favourable and the record is the hazard.**
Raising the ambient size *raises* the line-height, so a control goes
**48 px → 52 px** and the floor is not breached — nothing breaks.
**But four comments then state a sum the tree no longer produces**, and
that is this series' recurring defect, twice paid for already:
`§10.58` shipped in `0.63.0` with a heading that disagreed with its own
body, and `§10.60` was published two days later saying three of four
sites moved when all four had. **A comment that derives a number is a
claim about the tree exactly like a register entry is.**

`issues.rs:350` is the one to respect: it records that the margin over
44 px is **4 px**, and that `py-3` would miss the floor outright. Any
future change that *lowers* an ambient size, or trims padding on the
belief that 48 px has room in it, is 4 px from a **P1** failure. That is
not this round's work — **it is why this round must leave the arithmetic
true rather than leave it stale.**

**What is owed**: if this round raises any site whose ambient size feeds
one of those sums, **update that site's comment to the sum the tree then
produces**, and say in the review request which comments were touched.
`touch_target_scan` and `disclosure-marker-gate.mjs` must both be run
and reported if `me.rs:332` or `me.rs:367` changes — `MARK-002` measured
that plain text in a 44 px control wraps to 104 px, so a summary's
height is not a thing to reason about from a stylesheet.

## §3 One of the twenty-one is not a leaf

`me.rs:367` is `<section class="text-xs text-base-content/70">`. It is
**a container**, and every one of its descendants that does not set its
own size inherits 12 px from it — including the `<summary>` at 373 whose
touch target §2 describes, and the glossary `<p>` elements under it.

**So "raise site 367" is not one site's worth of change.** Before
deciding it:

1. **Measure the subtree.** How many text nodes actually inherit from
   it, and what are they? Report the count and what they are.
2. **Decide it separately from the leaves**, and say so. A container
   and a `<span>` are the same grep hit and not the same decision.
3. If the class decision for leaves is *raise* and the container is
   left alone, **that is a defensible split** — but it has to be stated
   as a split, because the next reader will see one file with two
   answers.

## §4 The classes, and what each one needs

**A decision per class, each with the height delta measured at 390 and
1280 on the real page.** Density is the objection that correctly killed
the 2026-11-04 proposal; it is answered in pixels or not at all.

1. **Section headings (4).** `text-xs uppercase tracking-wide` — the
   smallest size, all capitals, and letter-spaced, which together
   remove the word-shape cues a reader uses most. These are the labels
   that say what a panel *is*. **If any single class on this screen is
   wrong for an elder reader, the architect's expectation is this one** —
   which is a reason to measure it carefully, not a conclusion to
   implement. One of the four is the `<summary>` at 332 (§2).
2. **Chip labels (11).** The word beside a number. The numbers render
   in `badge badge-sm`; the words naming them are the smallest text in
   the panel. Measure what the chip rows do at 390 — these sit in
   `flex flex-wrap`, so the cost of wider labels is **a new row**, not a
   taller line, and a wrapped chip row is a visible density change.
   **Report rows before and after at 390**, not just heights.
3. **Italic explanatory lines (3).** 12 px italic is the hardest
   combination on the screen after the uppercase headings. **Removal is
   genuinely open here** — ask what each line tells a reader that the
   chip beside it does not.
4. **Value / sample lines (2).** 640 and 700. Short, and they carry
   data rather than labels.
5. **The container (1).** §3.

## §5 Removal is on the table, and here is where to look

*Less is more* points at taking away before it points at enlarging, and
two candidates on this screen are worth asking about by name:

- **The glossary `<details>`** under 367 — *"What do these mean"*. A
  definition list for six indicators, behind a disclosure, in the
  smallest text on the page. Does a reader who needs it find it there?
- **The three italic lines.** If an italic 12 px aside is the only place
  something is said, it is being said quietly.

**Removal means removing information, and that is a design decision** —
so propose it with the reason and measure what the screen looks like
without it. Do not remove anything whose absence you have not looked at.

## §6 What this round must not do

- **Must not adopt a type floor, write a type requirement, or add a
  scan over text sizes.** Rule after design. `TYPE-003` decides.
- **Must not touch the project screens** (`issues.rs`, `projects.rs`) —
  36 sites, `TYPE-003`'s population. **Including `issues.rs:350`'s
  comment**, unless this round changes something that makes its sum
  wrong, which it should not.
- **Must not touch `sprints.rs`** — its two summary comments are in the
  65 deliberately excluded sites' territory.
- **Must not lower any ambient size** anywhere (§2's 4 px).
- **Must not reopen `NFR-A11Y-005`, `NFR-A11Y-007`, `MARK-002`'s
  helper, or `contrast_scan`.**
- **Must not revisit `TYPE-001`'s six sites.** Closed, both rounds.

## §7 Acceptance

Each item names the record it comes from; **read the record, not this
list** — three of the architect's criteria were stale in the previous
series and the rounds caught all three.

1. **A decision per class (§4), each with its own reason**, from the
   three outcomes the series `README.md` names: keep, raise, remove.
2. **Height delta measured at 390 and 1280 on the real page** wherever
   a size changes — and **chip-row counts at 390** for class 2 (§4.2).
3. **The container decided separately, with its subtree measured**
   (§3).
4. **Every touch-target comment left true** (§2), with the touched
   comments named in the review request.
5. **`touch_target_scan` and `disclosure-marker-gate.mjs` run and
   reported** if `me.rs:332` or `me.rs:367` changes.
6. **`node browser-checks/overflow-gate.mjs`** if any rendered class
   changes. If nothing rendered changes, say so and skip it.
7. **Tests per `.github/CONTRIBUTING.md` → *Before you open a pull
   request* → *Tests***, read there. Baseline after `TYPE-001`:
   `DEC-007` **687**, `cargo test -p peisear-web --lib` **246**, both
   confirmed independently by the architect. Count the artefact with
   `wc -l`; re-derive rather than assume.
8. **Clippy both ways**, as CI runs it:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings
   ```
9. **`§10.60` amended** with this round's measurements and decisions.
   **It stays open** — `TYPE-003` has not run.
10. **`CHANGELOG.md`** under `[Unreleased]` if anything user-visible
    changes.
11. **Every non-text artefact checked for local information before it
    is committed** — a path under `/home`, a real display name, a
    hostname. Per artefact, because nothing scans them.

## §8 Where to file a question

`.git-exclude/review-request/TYPE-002-…/README.md` — the question, what
was tried, and what you are declining to do without a ruling. **A
blocking question goes in a file, never in chat.** `GRAM-002` round 3
is the precedent and it was right to stop.

**And one thing learned twice in two days, worth carrying into this
round**: a listing that does not show what you expect is not a
statement about the world. It is equally consistent with *not yet*,
*not where I looked*, and *my query is wrong*. Round 1 read a four-hit
grep as two, round 2 read an in-flight CI run as an absent one, and the
architect read thirty minutes of a broken watch as *still running*.
**Look again a minute later before writing *confirmed*.**
