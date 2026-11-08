# Handoffs — what the browser actually draws

**No release number yet.** This is **unit two** of the sequence the owner
authorized on 2026-11-04 — *"one by one with time we need: App base
layout(s), user workflows, and each view."* Unit one (`058`, the base
layout) closed 2026-11-07. This series is the **user workflows**.

**The row is authorized; three rounds have closed.** `CSS-002` closed
2026-11-08 with no findings against the work. `FLOW-001` closed the
same day with **one finding** — its two pause sources shared one piece
of state, so releasing either resumed the countdown while the other
still held — and `FLOW-001 r2` closed it the same day with no
findings, verified across the whole eleven-state truth table.
**`FLOW-002` is the last unstarted round in the `0.61.0` scope.** The owner authorized
`0.61.0` on **2026-11-07**, covering `FLOW-001` and `FLOW-002`;
`CSS-002` carries no release number and landed under `[Unreleased]`
(`254599c`).
**Authorizing the row is not approving the cut** — that is a separate
decision at Release cycle exit criterion 7, and `DEC-056`'s four legs
run only after it.

---

## How this unit was read, and why it found what it found

Unit one was read in **source**. This one was read in a **browser**: a
scratch walk drove the real binary through the operations a reader
actually performs — arrive, sign in, find a project, capture a piece of
work, move it along, look at today, look at the inbox, leave — and
recorded at each step, at a desktop and a phone width, where the reader
is, what the application said after they acted, where keyboard focus
landed, and every visible control in `<main>` and `<header>` in DOM
order. Screens were then looked at, not only parsed.

**The instrument is not committed.** It reuses `browser-checks/cdp.mjs`
and `overflow-gate.mjs`'s own `createFixtures`, and it asserts nothing —
it is a reading instrument, not a gate, and `browser-checks/README.md`'s
own title is *gates, not tests*. If unit three needs it a second time,
that is the moment to decide whether it earns a place in the repository;
deciding now would be widening a gate reactively, which `§10.32`'s
closing note forbids.

**It found a defect in unit one within its first screen**, which is the
argument for the method: `CSS-001` (landed 2026-11-07, `838fb24`,
`§10.52`). The primary navigation `058` had just shipped computed
`display: none` at **every** width, because the committed stylesheet had
no rule for `md:flex` — three review rounds of render tests, a required
prop and a compile-error plant had all read Rust, and none of them can
see a stylesheet. That fix is already on `main`; the **guard** for it is
round 1 below.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| CSS-002 **(no release number)** | [a class with no rule](./CSS-002-a-class-with-no-rule.md) | `CSS-001` fixed the stylesheet and both documented paths, and **nothing enforces the relationship**: a utility class added to a component still has no rule until a person remembers to run a binary that `cargo build` never invokes. A `css_scan` over the static `class="…"` tokens, with a named allowlist for the classes that are JavaScript hooks rather than CSS | — | **Closed 2026-11-08, no findings against the work.** `css_scan` lands at the specified bound — `components/**/*.rs`, the three literal shapes, line-continuations joined, interpolated tokens skipped — and the architect re-measured the population with an independent extractor written before reading theirs: **their token set is a strict superset of mine**, 293 to 291, the two extras being `btn-active` and `join-item` from the real `grow(if … else …)` sites a plain regex cannot reach; **nothing missed**. Checked systematically rather than by spot check: of the 293 distinct tokens **exactly two have no boundary-valid match anywhere** — precisely the two allowlisted — and **none is satisfied only by a match inside a CSS comment**. **Two architect plants, neither of them the round's**: one *inside* `issues.rs`'s real `if`/`else` branch, which failed naming `zz-architect-plant` and the file; one disabling the `class=` extraction branch, which dropped the population **291 → 100 with `violations` still empty** — proving the hardcoded `(291, 2)` assertion is the only thing that sees a silent loss of reach, which was the one design doubt worth having here. `DEC-007` **642 → 651**, `--lib` 235 re-run; `test.yml` untouched and line 927 already runs it; `CI`/`Docs` `success` at `254599c`, checked directly. **The round's own best moment is its second plant**: renaming `js-status-form` left their allowlist assertion green, because `.js-status-form` is a prefix of `.js-status-form-renamed` — found by planting, named in the report as their own bug, fixed with the same boundary discipline `token_has_rule` already had. **Three notes, none a defect**: the `(291, 2)` assertion's continuous cost, now measured on both sides (10 of the last 395 commits touch `components/**`); bound 5's concrete reach gap — `grow_text(MENU_FOCUS_RING)` leaves four real `focus-visible:!outline*` classes outside the population, **all four verified to have rules today**; and `components/tests.rs` sitting inside the walk, harmless now at zero `class=` literals |
| FLOW-001 | [five seconds to find the undo](./FLOW-001-five-seconds-to-find-the-undo.md) | Every undo is removed **5 000 ms** after it appears, with no pause on hover and none on keyboard focus — all four of `dm.js`, `board.js`, `calendar.js`, `plan.js`, and **WCAG 2.2.1**. The round pauses the timer on hover and on focus and does nothing else. **Its first direction — *delete the timer* — was reversed before it started**: each surface builds its own fixed-position container in the same corner, and two of them, measured, print over each other and cover the earlier Undo | — | **Closed 2026-11-08, one finding — it earns round 2.** The reversal is implemented as reversed (timer kept, nothing deleted, no dismiss control, 5 000 ms untouched), the four source tests pin all four listeners per file, and the gate reads the **timer handle** rather than sleeping, which is the right instrument under `DEC-048` condition 2. **But the two pause sources share one piece of state**: measured on the real `plan` surface with the round's own scaffolding — focus on Undo, pointer crosses the toast and leaves — `focusInside=true pausedByFocus=null afterEnter=null afterLeave=2 stillFocused=true`: **the pointer leaving restarts the countdown under the reader's own keyboard focus**, which is the defect this round exists to remove. It was visible in their own instrument: the gate's first run failed with *“expected a running auto-remove timer before hovering, found none”*, and the `blur()` that fixed it steps over exactly this case. `DEC-007` **651 → 655** re-derived; `undo_toast_attachment_scan` 8 re-run; citations 8 at all three sites; `CI`/`Docs` `success` at `223a740`, checked directly |
| FLOW-001 r2 | [two hands on one latch](./FLOW-001-round-2-two-hands-on-one-latch.md) | Each handler checks **the other** source before resuming — `mouseleave` only resumes when focus is not inside the toast, `focusout` only when the pointer is not over it — and `resumeAutoRemove` clears a running timer before starting a new one. Two conditions and no new state: the DOM already keeps this truth correctly. Plus the browser assertion for the case round 1's `blur()` steps over | FLOW-001 | **Closed 2026-11-08, no findings.** Both guards land as specified — `mouseleave` resumes only when focus is not inside, `focusout` only when the pointer is not over, each clearing a running timer first, no new state. The architect verified it by walking the **whole truth table** on the real `plan` surface rather than re-running the sequence that found the bug: **eleven states, eleven correct**, with the previously-broken case (pointer away while focus holds) now `null` and its mirror too. **The two states worth the walk were 6 and 10** — both release orders resume — because the regression direction here is a toast that *never* resumes, which would quietly recreate the never-expiring toast this round's own handoff rejected. **Architect's plant, aimed where theirs was not**: removing only the `:hover` guard (theirs reverted the whole shape) failed the mirrored assertion by name while the first still passed — **the two guards are independently pinned**. `DEC-007` **655 → 659**, `undo_toast_attachment_scan` 12 re-run, citations 12 at all three sites, drag-outcome gate exit 0 on the restored tree, `CI`/`Docs` `success` at `8294403`. **Three notes, none a defect**: the citation check was run before the first push this time, which is round 1's own lesson applied; their new assertion **states its precondition and fails loudly when it is not met** — it caught the architect's own probe — where round 1 silently `blur()`ed the same precondition into existence; and the `CHANGELOG` entry is correctly left untouched, since it already claimed the behaviour round 1 did not deliver and round 2 makes true |
| FLOW-002 | [the word on the door and the name inside](./FLOW-002-the-word-on-the-door.md) | *Inbox* arrives at **“Notifications”**, and of the page titles in `en.rs`, five end with *“— peisear”* and the rest do not, so three of the product's own screens are called nothing but **“Calendar”**, **“Notifications”** and **“My dashboard”** in a tab strip, a bookmark or a history list. **Narrowed before it started**: *Today* → **“My dashboard”** is the same defect, but renaming that heading before deciding what the page shows trades a mismatch for a broken promise | open question 1 for the `Today` half only | **Not started** |

---

## The decisions these rounds implement, and why they are not the dev team's

**A class a component writes must have a rule the product serves.** Not
*should*; the relationship has to be checkable by something that runs,
because the one thing between it and silence today is a sentence in a
README that was itself wrong for three weeks. The guard reads Rust and
CSS, both already in the repository, and needs no browser — so it costs
one scan and no CI minutes (`CSS-002`).

**An undo must not expire while the reader is still looking for it —
and the smallest change that achieves it is the right one.** The timer
pauses on hover and on keyboard focus. Nothing else changes.

*This reverses what this file said when it was written.* It said **delete
the timer**, on the argument that a toast living until it is replaced or
dismissed is simpler code than one that pauses. **Measured, that argument
is false**: each surface builds its own fixed-position container in the
same corner, nothing clears another element's toast, and two of them
overlap — both sentences printed over each other, the earlier Undo
covered and unreachable by pointer (`FLOW-001` §2 carries the rects).
**The 5-second timer is the only thing keeping that rare today**, so
deleting it promotes a collision into the resting state and then needs a
toast region, a cap and a dismiss control to undo the damage — more
mechanism, not less. *Less is more* is a claim about the result, and it
has to be measured like any other.

**The word on the door is the name inside.** `FR-NAV-001` fixes the five
words; the destinations must answer to them. A reader who clicks *Inbox*
and arrives at *Notifications* is being asked to hold two names for one
place, which is the precise cost `058` was authorized to remove
(`FLOW-002`).

**But a name is a promise, so it waits on what the page does.** *Today*
→ *My dashboard* is the same defect and is **held back**: rename the
heading while `components/me.rs` still renders no issue at all, and the
reader stops holding two names and starts being promised their day by a
page that shows two metric chips. The label follows the design, never the
other way round — the same ordering this project applied to the demoted
contrast rule.

**A guard nobody outside this repository can see does not need a release
number.** `CSS-002` lands under `[Unreleased]` like `054`'s own rounds
did; the owner's acceptance of that convention on 2026-11-03 is three
days older than this series, and `0.61.0`'s scope is the part a reader
would notice.

---

## Open questions — the owner's, not the architect's

**1. `/today` shows metrics about the work, never the work.**
`components/me.rs` renders no issue at all: two chips, a rhythm
disclosure, a sustainability disclosure. The fixture user has an issue
planned for **today**, and it appears on `/today/calendar` — one click
further, behind a button labelled *Calendar*. The destination the
navigation lists first, named *Today*, does not answer *what do I do
now*. Renaming the page (`FLOW-002`) makes the mismatch sharper, not
softer, so the question is worth asking before that round lands:
**should `/today` carry the day's planned work above its metrics?** The
data is already computed for the calendar. Recommendation: yes, and as
its own round, after `FLOW-002` — the naming is cheap and this is not.

**2. Where does an outcome — and its undo — belong?**
The outcome of a status change is announced **twice**: small grey text in
the `role="status"` region directly under the control, and a cyan toast
about 700 px away in the corner, which is the only copy carrying the
**Undo**. The overlap `FLOW-001` §2 measures is a symptom of a toast
built per acted-on element rather than per page; so is the distance. The
candidate answer — **the undo belongs beside the thing it undoes, and
the toast stops existing** — removes both by construction and is the
real *less is more* here. It touches four surfaces' visual language, so
it needs the unit-three reading of those surfaces and is **not** decided
inside a timing fix. Recommendation: hold it for unit three, and let
`FLOW-001` buy conformance in the meantime with two handler pairs.

**3. The home screen's substance is behind three closed disclosures.**
At 1280 px, `/today` occupies the top 290 px of a 900 px viewport and
the left 40 % of the width; its section labels are the smallest, faintest
text on the page, and under one of them (*RHYTHM*) nothing is visible at
all until it is opened. That is a **view** question, not a workflow one —
it belongs to unit three, recorded here so the reading is not lost.
