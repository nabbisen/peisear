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

**And a second measurement, because the first invites a bad fix**:
**fourteen `<details>` across six views**, **one** open by default, and
**exactly one requirement governs any of it** — `FR-PER-007`, for
`/today` alone, which says in terms that the rhythm panel MUST be
collapsed and the sustainability panel MUST self-expand only at a watch
threshold. Counting what is *governed or guarded* — those two panels,
plus the project screen's health and workload blocks since `VIEW-001`
and its round 2 — leaves **ten** that are neither. So *"collapse
things"* is not a rule this project has; it is a thing it has done ten
times without one.

> *Corrected 2026-11-11.* This paragraph first said **eighteen**, two
> open by default, thirteen ungoverned, and `VIEW-001 r2`'s own handoff
> inherited the arithmetic as *"the other eleven"*. **The grep behind
> those numbers counted `<details>` mentioned in comments as if it were
> markup** — six of them, including three in `issues.rs`'s own doc
> comments about disclosures. Re-measured with line comments stripped
> first, the way `css_scan` already does it for exactly this reason, and
> counting opening tokens rather than single-line tags so a tag split
> across lines is not missed. **A census is a measurement and its method
> is part of the number**; this one was taken by the architect and
> stated without one.

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
| VIEW-001 r2 | [the property itself](./VIEW-001-round-2-the-property-itself.md) | Pin *collapsed by default* for the health block, in the shape `/today`'s own guard already uses — and apply the same rule, with its own guard, to `WorkloadStrip`, which round 1 measured at a further 90 px / **124 px on a phone** and correctly declined to build | VIEW-001 | **Closed 2026-11-11, no findings.** Both guards pin the property round 1 left open, and the architect verified each by its own plant rather than the round's: `open=true` on the health block's outer `<details>` (line 321) fails `health_block_is_collapsed_by_default` **by name**, `open=true` on the workload block's (line 640) fails `workload_strip_is_collapsed_by_default`, and **each plant fails exactly one test** — 12 passed, 1 failed, both times. **The round's own §1 is the best thing in it**: its first guard copied `/today`'s backward scan verbatim, which is safe there and wrong here because the health block's first child *is* a second `<details>` — found **by planting**, then confirmed against the raw response body to rule out a Leptos serialization bug before rewriting the anchor. `§3b` is the property the review asked for: **44 px collapsed, constant across two fixtures and both widths, for both blocks**, against 536–913 px (health) and 126–160 px (workload) open. `DEC-007` **676 → 678** and `MessageKey::all()` **714 → 715** with `page_title_scan`'s pin updated in the same commit, both re-derived; `CI`/`Docs` `success` at `83e0cc7`. **One note, not a defect**: *“Workload across the team.”* renders on a personal project, where there is no team — the round's own guard creates exactly that case. One string with a condition already at the call site; held for this series' closing round rather than sent back for a word the round was not asked to choose |
| VIEW-003 | [one outcome, one place](./VIEW-003-one-outcome-one-place.md) | The undo moves beside the control it undid and **the corner toast stops existing** — removing the duplicated sentence, the ~700 px distance and the two-toast overlap by construction rather than managing them. **All four surfaces in one round**: the mechanism is one shape in four files, and a mechanism half-moved is the defect it is fixing | — | **Closed 2026-11-11, no findings.** Reviewed on the PR branch before the merge — `gh pr merge` was denied in the architect's own session by the same classifier that blocked the dev team, so the review happened where it could: `pull/49/head` built in an isolated worktree. **Both undo gates exit 0 on that binary** — stored effect confirmed, live region text correct, **Tab reaches Undo** on both surfaces. **The architect's own plant, aimed at the one thing the round is about**: deleting `region.appendChild(undoButton)` from `plan.js` fails with *“plan: Tab from the acted-on row did not land on the undo button”* **after** the stored-effect and live-region assertions pass, so the failure is specific to the Undo's presence rather than a broken surface. **The timer and the toast are genuinely gone**: `setTimeout` is **0** across all four files, every remaining `toast` token is in a comment explaining what was replaced, and `undo_toast_attachment_scan` is **4 tests** down from 12 — the eight deleted with the reason stated in the file's own doc, *“a single, relocatable region cannot suffer the two-toasts-collide defect the timer's pause/resume existed to manage”*, which is the right reason in the right place. `peisear-web --lib` **246** unchanged, `DEC-007` **678 → 670** re-derived. `CI`/`Docs` `success` read directly at the merge commit `6d2ad33`. **Two notes, neither a defect.** The `CHANGELOG`'s *“right next to what it undoes”* is generous at desktop width: measured after a real drop at 1280×900, the acted-on row at `(685, 413)` and the Undo at `(1069, 436)` — **dy 23 px, dx 384 px**, inside the row it belongs to and at the far end of it. Against a corner toast ~700 px away diagonally that is the change the round claims, and the Tab-order property is now structural rather than maintained; but a 1280 reader will experience it as *the same line, at its other end*. **One width was measured** — on a phone the row is ~358 px wide, so `dx` can only be smaller, and the reader this series is for is in the better case, which is stated rather than claimed. Worth one honest word in the changelog, not a round. And the scan is still named `undo_toast_attachment_scan` for a thing it no longer scans: a rename is six coordinated edits plus a citation check, so it is **named as debt, not ordered** |
| NAME-001 | [the destination that is about you](./NAME-001-the-destination-that-is-about-you.md) | The nav word for `/today` becomes **`Me`**: the page is about the reader, in the published design's own words, and the word said otherwise. Closes `NFR-A11Y-012`'s pinned exception and gives `nav_destination_naming` its fifth case. The route does not move | — | **Closed 2026-11-11, no findings.** The word is `Me` in the nav, in `/today`'s own title and `<h1>`, and — for free, because it was always one `MessageKey` and never a second copy — in the breadcrumb root of every detail screen. The pending tripwire is **deleted, not edited**, which is what it was written to require, and `nav_destination_naming` gains the fifth live-rendered case it was built to hold. **Two plants from opposite sides**: the round reverted the *heading* to `My dashboard`, the architect reverted the *nav word* to `Today`, and the same test caught both by name — neither plant alone shows that it fails on either half of the pair diverging. Counts re-derived independently and matching: `--lib` **246 → 245**, `nav_destination_naming` **4 → 5**, `DEC-007` **670 → 670**, `MessageKey::all()` unchanged at 715. **The acceptance measurement was a measurement**: at the 768 px threshold all seven header controls sit in one row with no overflow, and substituting `Today` back into the same render pushes the search field 20 px right — the shorter word buys headroom rather than spending it. **One incidental finding, pre-existing and halved by this round**, filed as `§10.55`: at 768–800 px the account menu's box covers the search field's right edge by 18/13 px, invisible because the control is `btn-ghost` and undetected because `overflow-gate` measures overflow, never occlusion. **And the round read `main`'s own CI at the merge commit rather than the PR's green, and found it red** — one job, a lost runner: the step was cancelled 47 seconds in, the job was closed out 45 minutes later and no log was ever uploaded. Their conclusion (not a regression) was right; their evidence (*"a 46-minute hang"*) was the run's elapsed time, not what ran. `§10.56`, not re-run |
| VIEW-002 | — | Issue detail, the other screen in the authorized row. **Measured clean** (table above): its own `<h1>` is a quarter of a screen down at both widths, behind four controls, which is a breadcrumb and a back link. **No round is proposed**; the owner may still want one, and the measurement is here so that is a decision rather than a silence | — | **Not proposed** |

**Every work round closed 2026-11-11; the release candidate opened
2026-11-12.** The four rounds specified here are closed and
reviewed: `VIEW-001` and its round 2, `VIEW-003`, `NAME-001`. `VIEW-002`
was decided as *not proposed*, with its measurement recorded so that is a
decision rather than a silence. The unit's own closing obligations are
discharged in the same cut — **`external-design §5.10`** states the
section-order rule once, **`FR-NAV-006`** cites it rather than restating
it, **`FR-NAV-001`** names `Me`, and **`NFR-A11Y-012`** reaches Met for
all five destinations. What this unit deliberately did not decide: the
ten remaining ungoverned `<details>`, which are a set to be decided once,
and the typography and colour floor, whose stated condition — *adopt
after the views are decided* — is now met.

## `FOCUS-001` — open, and it blocks the cut

**The architect's `DEC-028` pass over this series found a P1 regression
in this series' own work, caused by the architect's own instruction.**
`VIEW-003` deleted the undo's focus restoration because `VIEW-003`'s
handoff told it to; measured 2026-11-12 with real key events, activating
Undo now leaves `document.activeElement` on **`BODY`**, which is the
defect `NFR-A11Y-002` (P1) names and `A11Y-004`/`A11Y-005` closed in
0.41.0. **Not released** — it is on `main` only, which is why it is
found before a cut rather than after one.

[`FOCUS-001-where-undo-leaves-you.md`](./FOCUS-001-where-undo-leaves-you.md)
restores the ladder that entry already names, measures **all four**
surfaces (the architect measured one), and adds the assertion whose
absence is the real finding: `drag-outcome-gate` proves Undo can be
**reached** and never presses it. `§10.57` records that class.

## The release candidate — `0.62.0`

**Started 2026-11-12 on the owner's instruction, and split.**
[`REL-0.62.0-release-candidate.md`](./REL-0.62.0-release-candidate.md)
carries the whole cut: which exit criterion is answered where, the claim,
what this release does not fix, and the four legs that run **only** after
the owner approves at criterion 7.

**The cut is not the dev team's role.** Three preparation rounds are, and
they are specified in that file:

| ID | What | Why it is not assumed |
|---|---|---|
| `PREP-001` | Regenerate the **seven** published screenshots and **look at every one** | Two of the seven were opened by the architect and are stale in three ways — the nav word, the breadcrumb root, and a health block rendered fully expanded, which is the arrangement this release replaced. **Five were not opened**, and that bound is the round's first job |
| `PREP-002` | Criterion 2 on `main`'s own tip, with captured logs and real exit codes | The release cycle names this *"the one that has historically failed"*. `DEC-007`'s total is **670** by the architect's count; a disagreement is to be reported, not reconciled |
| `PREP-003` | Walk the upgrade with real binaries, `0.61.0` → candidate, on one database | No migration is added — but that is a diff, and the claim is a walk |

**Not the dev team's, in that file and in this one**: the version bump,
the dated heading, the tag, the publish, the Release. Not as a step, not
in passing, not as a dry run.

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

## The three questions, answered 2026-11-11 — and accepted the same day

The owner returned all three with *"review them carefully by yourself with
my fundamental philosophy."* Each is answered below. **Two of the three
reverse what this file said**, and the third keeps its verdict while
discarding the reasoning that produced it.

**Accepted by the owner 2026-11-11.** Two of the answers became work:
`VIEW-003` (the undo's place) and `NAME-001` (the word), both in the
rounds table above, and **`0.62.0`'s row was re-authorized the same day**
to carry them — the first use of this project's own rule that *a row
that changes after authorization is re-authorized, not quietly edited*.
`VIEW-002` stays unproposed, which was the third answer.

### 1. `VIEW-002` — not wanted, and the reason this file gave was not evidence

**The verdict stands: no layout round for issue detail.** What does not
stand is how it was reached. This file called the screen *"measured
clean"* on one number — where the first work sits — and that number was
invented for the **project** screens, whose defect was a wall of analysis
above the work. **Issue detail cannot fail that test**, so passing it
said nothing. A screen declared clean by a metric it cannot fail is
`§10.36`'s own shape, applied by the architect to his own unit.

Read instead against the question this unit actually asks — *what is
visible by default, and what is invisible* — at 390×844:

| y | size | what |
|---|---|---|
| 108 | 14 px | breadcrumb: Today › Projects › Website Launch › *the issue title* |
| 160 | 14 px | ← Back to issues |
| 204 | 20 px | the issue title, as `<h1>` |
| 272 | 14 px | Edit · Delete |
| 328 | 14 px | Open / In Progress / Done |
| 384 | **12 px** | High · 5 pt · Created 2026-11-10 23:31 |
| 408 | **12 px** | Updated 2026-11-10 23:31 |
| …  | | the description, then Sub-issues |

**Nothing on this screen is hidden by default** — it holds no
`<details>` at all, which is the one thing that made it worth reading
and the one thing the y-number could never have told us.

Two things that look like findings and are not, both checked rather
than assumed. **The doubled way back** — a breadcrumb whose penultimate
crumb is the parent *and* a "← Back to issues" link to the same place —
is `FR-NAV-003` in so many words: *"Detail screens MUST provide a
breadcrumb trail **and** a back link."* And **"+ Add sub-issue" is not
above its own heading**: the two sit in one `justify-between` row, and
the 12 px between their tops is vertical centring. The first read of
this page produced that as a finding; the source disproved it before it
reached anyone.

**One real item survives**: two lines of **12 px** metadata between the
status control and the description. That is the typography question,
not a layout one — see §4.

### 2. `/today`'s name — the recommendation is reversed

This file recommended **renaming the page's heading to *Today***. That
is the worst of the three available answers, and `FLOW-002` already had
the right one before this file talked itself out of it.

What a reader actually meets, in each case:

| | they click | they arrive at | what happens |
|---|---|---|---|
| (a) today | **Today** | *My dashboard* | a small dissonance, **resolved on arrival** — the page says what it is |
| (b) the rename | **Today** | *Today* | the dissonance is **confirmed and never resolved**: the page's own name agrees with an expectation it does not meet |
| (c) the word changes | **Me** | *My dashboard* | nothing to resolve |

**(c) is best, (a) is second, and (b) — the one recommended here — is
worst.** The published design already says which way this goes:
`external-design §3.2` gives this destination's own responsibility as
**"What should I look at right now, about myself?"** The page is right.
The word is wrong.

**What that costs, measured rather than feared**: one word in
`FR-NAV-001`, one `MessageKey`, and `NFR-A11Y-012`'s pending pair
resolves instead of staying pinned. The route does not move;
`FR-NAV-002`'s redirects are untouched.

**And the one thing against it, stated because it is real**: the product
renamed `/me` to `/today` deliberately at `0.17.0`, and **no published
document records why**. Changing one of five entry-point words that
`FR-NAV-001` fixes is product scope, so it is the owner's — but the
recommendation is now (c), and failing that **(a), the status quo**.
`NFR-A11Y-012`'s exception stays honest and pinned rather than being
closed by making the product worse.

### 3. Where an outcome and its undo belong — decided, not held a third time

Held at `059` for *"the unit-three reading of those surfaces"*. That
unit is now two rounds old and one of the four surfaces is the project
board, in this very row. **Holding it again would be deferral by
habit**, so it is decided here.

**One outcome, one place: the undo belongs beside the thing it undid,
and the corner toast stops existing.** The measurements are already in
`059`: the same sentence is printed twice, ~700 px apart on a 1280 px
screen; only the far copy carries **Undo**; and because each toast is
built per acted-on element in one shared fixed corner, two of them
overlap and cover each other's button. Moving the undo inline removes
all three **by construction** rather than managing them — and
`A11Y-004`'s own property, *Undo is the next Tab stop after the control*,
becomes trivially true instead of carefully maintained.

**This needs the row re-authorized before it starts.** The mechanism is
one shape in four files; two of its surfaces (board, issue detail's
status form) are in this row and two (`calendar.js`, `plan.js`) are not.
Splitting one mechanism across two releases would leave the product
saying the same thing two ways — which is the defect. So: all four, or
not yet.

### 4. What these three leave standing: typography

`VIEW-002`'s one real item is 12 px metadata. The project screen's
section labels are `text-xs` and the muted tier. `/today`'s are the
same. **The owner's original question listed typography and colour**,
and this architect demoted both on 2026-11-04 with a stated condition —
*adopt the floor after the views are decided, never before, rule after
design*. **Two views are now decided.** The condition is met, and the
rule is the natural next piece of work: one measurement across the
views this unit has read, one floor, written once.
