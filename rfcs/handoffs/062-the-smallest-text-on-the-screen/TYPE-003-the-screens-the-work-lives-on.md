# TYPE-003 — the screens the work lives on

**Series:** [062 — the smallest text on the screen](./README.md)
**Release:** `0.64.0`, authorized 2026-11-17 — the row, not the cut
**Blocked on:** `TYPE-002`, closed across two rounds 2026-11-17 and
accepted.
**Register entry:** `§10.60`, still **open** — and this round does not
close it either. See §8.

The last read. `TYPE-001` took the six sites below the scale;
`TYPE-002` took one screen's 21 at the floor. **This round takes the
36 on the screens where the actual work happens** — and it is where the
form labels, a person's display name and the table cells live, which is
why the `0.64.0` row named it as the round that needed the first two to
inform it.

**Decide by class, as `TYPE-002` did.** Thirty-six separate judgements
is a list, not a reading.

**This round does not write the type requirement.** That decision comes
after it and it is the architect's — *rule after design*, since
2026-11-04. §8 says what this round owes toward it.

---

## §1 The population — 36 render sites

```bash
grep -rn 'text-xs' crates/peisear-web/src/components/issues.rs crates/peisear-web/src/components/projects.rs \
  | grep -vE ':[[:space:]]*//'
```

**31 in `issues.rs`, 5 in `projects.rs`.** The `grep -vE` is not
optional — a plain `grep -c` returns 33 and 6, because comments mention
the class. **Re-derive and report your own number.**

| class | n | sites |
|---|---|---|
| **Containers** (set the size for a subtree) | 4 | `issues.rs:333`, **`:342`**, **`:588`**, `:663` |
| **Form field labels** — `label-text text-xs` | 3 | `issues.rs:1093` (Status), `:1118` (Assignee), `:1140` (Sort by) |
| **Form hints** — `label-text-alt text-xs` | 5 | `issues.rs:1367`, `:1520`, `:1929` (all `StoryPointsHint`), `projects.rs:146`, `:155` |
| **Table cells** — `<td class="text-xs">` | 3 | `issues.rs:1269`, `:1270`, `:1271` |
| **A person's display name** | 1 | `issues.rs:646` |
| **Monospace textareas** — `font-mono text-xs` | 3 | `issues.rs:1333`, `:1480`, `:1894` |
| **Section headings** — `text-xs uppercase tracking-wide` | 2 | `issues.rs:335`, `:665` |
| **Italic / explanatory prose** | 3 | `issues.rs:257`, `:782`, `:1724` |
| **Card text** | 3 | **`projects.rs:70`**, `:83`, `:239` |
| **Remaining spans and divs** | 9 | `issues.rs:477`, `:506`, `:583`, `:668`, `:725`, `:1034`, `:1655`, `:1789`, `:2108` |

**Three `StoryPointsHint` sites are the same hint from three code
paths** (`:1367`, `:1520`, `:1929`). `GRAM-002` found a duplicate path
the hard way; reach all three or none.

**Two of these sites were already raised once this release.**
`projects.rs:83` (*"Updated &lt;date&gt;"*) and `issues.rs:1034` (the
board card's metadata row) came up from 11 px to 12 px in `TYPE-001`.
If this round raises them again they will have moved twice in one
release. **That is a consequence of reading one screen at a time, not
churn** — but say so in the record, so it reads as intended.

## §2 The form labels already have an answer in their own file

`issues.rs` contains:

- **20** sites spelled `label-text text-sm`
- **3** sites spelled `label-text text-xs` — `:1093`, `:1118`, `:1140`

**The three 12 px form labels are the minority spelling of a pattern
that is already 14 px twenty times over, in the same file.** The new
issue form's own fields (`:1324` onward) label themselves `text-sm`
while the issue list's filter controls label themselves `text-xs`.

**This round does not need to invent a target for that class** — it
needs to decide whether the three are deliberate or accidental, and say
which. If accidental, raising them is making a file agree with itself,
which is the cheapest kind of change this series can buy.

**Measure it anyway.** *"The file already does this elsewhere"* is an
argument, not a measurement, and the filter row is a `flex` that can
wrap.

## §3 Two containers carry constraints — read before touching

**`issues.rs:342` — a touch-target sum with 4 px of margin.**
`<details class="text-xs">` sets the ambient for the `<summary>` at
`:350`, whose comment records: *"`py-3` (24px) would total only 40px
here; `py-4` (32px) reaches 48px, over the 44px floor rather than
exactly on it."* Raising the container to `text-sm` makes that
**32 + 20 = 52 px** — more margin, never less. **But the comment then
states a sum the tree no longer produces**, which is this series'
recurring defect, and `TYPE-002` set the standard for handling it:
update the sum, pin it from `static/tailwind.css`, and re-run
`touch_target_scan` and `disclosure-marker-gate.mjs` rather than
asserting them clean.

**`issues.rs:588` — the one bare `<summary>` in the codebase, and a
hardcoded anchor.** The `<summary>` at `:589` composes **no helper at
all**. That is legitimate: it is the single documented exception, and
`alignment_scan.rs:289` holds **its exact class string as a literal**,
with the rule that *"if a second bare `<summary>` ever appears, the rule
is wrong and goes."*

**So:**

- Raising the **container** at `:588` is safe — the summary's own class
  string does not change, and the anchor still matches.
- **Putting a size class on the `<summary>` at `:589` breaks
  `alignment_scan`'s anchor.** Do not. If a size on that element turns
  out to be the right answer, **stop and file a review request** — it
  makes the exception's anchor stale, which is a register-entry-shaped
  change, not a round's call.

This is the same hazard `spec_scan` rule 3 already carries in the
register: a check anchored on a quoted string from a file that people
edit.

## §4 `projects.rs:70` pays both costs at once

```
<div class="text-xs text-base-content/70 line-clamp-2 min-h-[2rem]">
```

`line-clamp-2` clips to two lines; `min-h-[2rem]` floors the box at
32 px, which is **exactly two lines at `text-xs`'s 16 px line-height.**

So raising this one site costs **both** things this series has learned
to measure separately:

1. **The card grows.** Two lines at `text-sm` is 40 px, above the 32 px
   floor, so the box gets taller — and every project card on the page
   with it.
2. **Fewer characters survive.** The clamp is by *lines*, so each line
   holds fewer characters at a larger size, and a description that fit
   in two lines may now be cut.

**This is `TYPE-001` round 2's own lesson on a different surface** —
there, the calendar badge on a `truncate` line, where the cost of larger
text was characters lost rather than pixels gained. **Measure both
here**, at 390 and 1280: the card's height, and whether a description
that previously fit now clips. Use a short, a typical and a long
description.

**`min-h-[2rem]` is itself keyed to `text-xs`'s line-height.** If the
size changes and the floor does not, the floor stops meaning *two
lines*. Decide what it should be and say so — do not leave an arbitrary
value whose reason has quietly expired.

## §5 The classes, and what each needs

A decision per class, each with the height delta measured at **390 and
1280** on the real page, and each from the three outcomes the series
`README.md` names: **keep, raise, remove.**

Four need more than a height:

- **Form field labels (3)** — §2. Measure the filter row's wrap at 390.
- **Table cells (3)** — `issues.rs:1269-1271`, holding effort, assignee
  and updated-at. A table's row height is driven by its tallest cell,
  and these three are a `table table-sm`. **Measure row height and
  whether the table starts to scroll horizontally at 390**, which is a
  different failure from a taller row.
- **Monospace textareas (3)** — `issues.rs:1333`, `:1480`, `:1894`.
  These are `rows="6" maxlength="10000"` description fields, and **their
  own labels are `text-sm` while the field the user types into is
  12 px monospace.** A field is read while being written. Raising it
  means fewer visible lines of what has been typed, so measure
  **characters and lines visible in the 6-row box**, not just height.
- **A person's display name (1)** — `issues.rs:646`,
  `text-xs font-medium wrap-anywhere`. It is a person's name rendered
  smaller than most of the furniture around it. `wrap-anywhere` means
  the cost of a larger size shows up as **wrapping inside the name**,
  which `§10.26`'s 21-character display name is the standing reminder
  about. Measure with a long name, not a short one.

## §6 Removal is on the table, and here is where to look

- **`issues.rs:1367`, `:1520`, `:1929`** — the same `StoryPointsHint`
  three times. If a hint has to be repeated at every entry point, ask
  whether the field is named well enough.
- **`issues.rs:257`, `:782`, `:1724`** — italic 12 px explanatory
  prose. `TYPE-002` kept all three of its italic lines and gave a
  specific reason for each; do the same, or remove.

Propose removal with the reason and measure the screen without it. **Do
not remove anything whose absence you have not looked at.**

## §7 What this round must not do

- **Must not write a type requirement, adopt a floor, or add a scan over
  text sizes.** §8.
- **Must not put a size class on `issues.rs:589`** (§3). Stop and file
  instead.
- **Must not touch `sprints.rs`, `teams.rs`, `settings.rs`,
  `sprint_plan.rs`, `notification_preferences.rs`, `auth.rs`,
  `notifications.rs`, `search.rs`, `layout.rs` or `calendar.rs`** — the
  68 deliberately excluded sites. `layout.rs:219` in particular records
  a deliberate decision *not* to use `text-xs` for primary navigation;
  leave it.
- **Must not revisit `TYPE-001`'s or `TYPE-002`'s decisions.** Both
  closed. Raising `projects.rs:83` or `issues.rs:1034` a second time is
  **not** revisiting — they are in this round's population now (§1).
- **Must not lower any ambient size** (§3's 4 px).
- **Must not reopen `NFR-A11Y-005`, `NFR-A11Y-007`, `MARK-002`'s
  helper, `contrast_scan`, `touch_target_scan` or `alignment_scan`.**

## §8 What this round owes toward the floor decision

**The rule is decided after this round, by the architect, on what the
three rounds found.** This round's job is to make that decision possible
on evidence rather than on taste. Alongside the per-class decisions,
answer three questions in the review request:

1. **Did a single rule fit all three screens, or did each screen need
   its own reading?** If the answer is that the readings converged, a
   requirement is cheap to write. If they diverged, say where.
2. **What would a floor have got wrong if it had been adopted on
   2026-11-04?** You now have three screens of evidence and the original
   proposal is on the record. This is the question that justifies — or
   retires — the *rule after design* condition itself.
3. **Which of this release's costs would a rule not have predicted?**
   The calendar badge's clipped line, `min-h-[2rem]`'s expired reason,
   the touch-target sums keyed to a line-height, a textarea's visible
   lines. A rule that says *"no text below 14 px"* predicts none of
   them.

**Answer these as findings, not as a recommendation.** The
recommendation is the architect's, and a round that arrives with one
has done `TYPE-003`'s work and then some of someone else's.

## §9 Acceptance

Each item names the record it comes from; **read the record, not this
list.**

1. **A decision per class (§5), each with its reason**, from keep /
   raise / remove.
2. **Height delta at 390 and 1280** wherever a size changes, plus the
   class-specific measurements §5 names: filter-row wrap, table row
   height **and horizontal scroll at 390**, textarea visible lines, and
   display-name wrapping **with a long name**.
3. **`projects.rs:70` measured both ways** (§4) — card height, and
   whether a description that fit now clips — and a decision about
   `min-h-[2rem]`'s own value.
4. **Every touch-target comment left true** (§3), pinned from
   `static/tailwind.css`, with the touched comments named.
5. **`touch_target_scan`, `disclosure-marker-gate.mjs` and
   `alignment_scan`** run and reported if any container in §3 changes.
6. **`node browser-checks/overflow-gate.mjs`** — rendered classes will
   change, so run it and report it.
7. **Tests per `.github/CONTRIBUTING.md` → *Before you open a pull
   request* → *Tests***, read there. Baseline: `DEC-007` **687**,
   `cargo test -p peisear-web --lib` **246**, both confirmed
   independently by the architect. Count the artefact with `wc -l`.
8. **Clippy both ways**, as CI runs it:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings
   ```
9. **The three questions in §8**, answered as findings.
10. **`§10.60` amended** with this round's measurements and decisions.
    **It stays open** until the floor decision is made — this round
    does not close it, and saying so in the entry is part of the
    amendment.
11. **`CHANGELOG.md`** under `[Unreleased]` for anything user-visible.
12. **Every non-text artefact checked for local information before it
    is committed** — a path under `/home`, a real display name, a
    hostname. Per artefact, because nothing scans them.

## §10 Where to file a question

`.git-exclude/review-request/TYPE-003-…/README.md` — the question, what
was tried, and what you are declining to do without a ruling. **A
blocking question goes in a file, never in chat.** `§3`'s
`alignment_scan` anchor is the most likely one to need it.
