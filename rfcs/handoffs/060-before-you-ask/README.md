# Handoffs — before you ask

**`0.62.0`, authorized by the owner 2026-11-10** — the row, not the cut.
This is **unit three** of the accessibility sequence the owner set on
2026-11-04: *"one by one with time we need: App base layout(s), user
workflows, and each view."* Units one (`058`) and two (`059`) shipped in
`0.61.0`.

**The question this unit asks, in the owner's own words**: *what are
visible by default, and what are invisible by default.*

---

## What was measured before anything was proposed

Every number below is from the shipped build, at a desktop and a phone
width, with the gate's own fixtures. "First work" means the first thing
on the screen that is the reader's own work rather than analysis about
it, navigation to it, or a control that acts on it.

| Screen | 1280×900 | 390×844 |
|---|---|---|
| **Project board** — first column | y=**508** (0.56 screens), **17** controls above it | y=**616** (0.73 screens), 11 above |
| **Project list** — first row | y=**576** (0.64 screens), **22** controls above it | y=**752** — **0.89 of the viewport** — 19 above |
| Issue detail — its own `<h1>` | y=204 (0.23 screens), 4 above | y=204 (0.24 screens), 4 above |
| Issue detail — the description card | y=348 (0.39 screens), 9 above | y=440 (0.52 screens), 9 above |

**The project screens are the finding. Issue detail is not.** On a
phone, a reader opening a project's list view sees **no work at all**
without scrolling — the first row begins at 89 % of the viewport, behind
nineteen controls. On the board the first column begins three-quarters
of the way down. What occupies that space is the breadcrumb and back
link, the title row with its four buttons, **the health block** — a
heading, a sentence, an `Indicators` disclosure and six indicator chips
each with its own expander — and the workload strip.

**And a second measurement, because the first invites a bad fix**: there
are **18 `<details>` across six views**, two open by default, and
**exactly one requirement governs any of it** — `FR-PER-007`, for
`/today` alone, which says in terms that the rhythm panel MUST be
collapsed and the sustainability panel MUST self-expand only at a watch
threshold. The other thirteen are ad-hoc. So *"collapse things"* is not
a rule this project has; it is a thing it has done thirteen times
without one.

## What governs this today — checked in both documents, not assumed

**Nothing.** `external-design §5.9` states the order of items **within**
a list, once, for every list-bearing screen. `FR-HLT-008` governs the
composite indicator's weight **within** the health block. `FR-HLT-007`
requires each indicator to offer a route to its basis. None of them
reaches the vertical order of a screen's own sections.

This matters because the sibling case went the other way an hour
earlier: `/today`'s collapsed panels **are** specified (`FR-PER-007`),
and the obvious `0.62.0` proposal — *make `/today` show the day's work* —
died on `FR-PER-001`, a **P0** that says `/today` shows load, rhythm and
sustainability signals. **Read the entry before proposing the round.**

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| VIEW-001 | [work first](./VIEW-001-work-first.md) | A project screen's first content is its work; the analysis above it becomes **one line** until asked for, by the discipline `/today` already has | — | **Closed 2026-11-11, one finding — it earns round 2.** The collapse is built and the headline is right: the summary is the highest-precedence non-Good indicator's own sentence, or one neutral line; the precedence chain is **accepted as proposed** (its top three mirror `FR-PER-006`'s own, minus the project-level burnout that does not exist), and sorting by weight was correctly refused because that would make the composite a sort key. **Independently re-measured on the gate's fixture rather than theirs: −48 px at all four cells, reproducing their number exactly.** But −48 is not the property — measured on the shipped build the collapsed block is **44 px and constant**, where fully open it is **560 px at 1280 and 865 px at 390**; the report gave the delta from a near-healthy fixture without saying its block was ~92 px to begin with. **The finding: nothing pins *collapsed by default*.** Planting `open=true` on the new outer `<details>` — the exact defect this round removes — leaves lib 246, `health_explainability` 11 and `touch_target` 6 all green, because both new integration tests assert the headline's content and order, which hold open or closed. **The assertion already exists for the sibling screen**: `today_folds_rhythm_panel_by_default`, cited by `FR-PER-007`'s own acceptance. `DEC-007` **666 → 676** re-derived. **Two things done well and worth naming**: the `WorkloadStrip` question was *measured* rather than preferred (90 px desktop, 124 px phone) and reported as out of scope rather than built — the harder report to write, since the handoff's other branch would have let them say *leave it*; and their probe's own `<details>` discovery (Chromium keeps a closed subtree's last layout box) **explains an anomaly the architect's unit-two walk reported and did not pursue** |
| VIEW-001 r2 | [the property itself](./VIEW-001-round-2-the-property-itself.md) | Pin *collapsed by default* for the health block, in the shape `/today`'s own guard already uses — and apply the same rule, with its own guard, to `WorkloadStrip`, which round 1 measured at a further 90 px / **124 px on a phone** and correctly declined to build | VIEW-001 | **Not started** |
| VIEW-002 | — | Issue detail, the other screen in the authorized row. **Measured clean** (table above): its own `<h1>` is a quarter of a screen down at both widths, behind four controls, which is a breadcrumb and a back link. **No round is proposed**; the owner may still want one, and the measurement is here so that is a decision rather than a silence | — | **Not proposed** |

## The decision this round implements, and why it is not the dev team's

**A project screen's first content is its work. Analysis about the work
is one line until the reader asks for it.**

The clauses, each with the reason it exists:

1. **One line, and it is a sentence — never a score.** `FR-HLT-008`
   forbids a single aggregate as the headline representation of project
   health; a collapsed summary showing the composite would be exactly
   that under a different name. `FR-HLT-005` already requires a
   plain-language sentence for a non-healthy indicator, so the sentence
   exists — what is missing is choosing **one** of them.
2. **The choice is a precedence chain, and a healthy project gets a
   single neutral line.** This is `FR-PER-006`'s discipline, already
   written for `/today`: at most one callout, by strict precedence, and
   **no manufactured callout in a healthy state**, because *"a
   manufactured callout in a healthy state is noise that trains the user
   to ignore the slot."* The project screen is the one place that
   discipline was never applied.
3. **Expanded, the block is exactly what ships today.** Every indicator,
   every basis route (`FR-HLT-007`), every expander. This round moves
   nothing out of reach; it changes what a reader is shown *before*
   asking.
4. **Collapse, not relocation.** Moving the block below the columns
   would make a reader who wants health scroll past every issue. One
   line at the top is findable and costs nothing.

**Why not simply apply "collapse by default" everywhere** — the thirteen
ungoverned `<details>` are a **set**, and a set is decided once, after
two screens have shown what the rule should be. That census is named and
deliberately not in this row.

**The specification home comes after the round, not before.** The rule
will be stated **once**, in `external-design §5.10`, beside `§5.9` and
for the same reason that section gives: *"all four defects came from an
order being expressed in more than one place or in none."* A
requirements entry will **cite** it rather than restate it. The
architect writes both when the round closes, as with `FR-NAV-001` and
`NFR-A11Y-012`.

## Open questions — the owner's

**1. `/today` is reached by the word *Today* and titled *My dashboard*.**
`FLOW-002` held the rename back and `page_title_scan` pins it as
pending. The content half is now answered — `FR-PER-001` is a P0 and the
page is doing what it says — so the remaining choice is the name.
**Recommendation: rename the heading to *Today***, and let the word mean
*how your work is going today*, which is what `FR-PER-001`'s own three
nouns describe. It is a one-string change in whichever round takes it.

**2. Where an outcome and its undo belong**, carried from `059`: the
outcome is announced twice, and only the far copy carries **Undo**.
Still a view question, and it belongs to this unit rather than to a
timing fix.
