# Handoffs — what the browser actually draws

**No release number yet.** This is **unit two** of the sequence the owner
authorized on 2026-11-04 — *"one by one with time we need: App base
layout(s), user workflows, and each view."* Unit one (`058`, the base
layout) closed 2026-11-07. This series is the **user workflows**.

**Nothing here is started.** The rounds below are specified and ready to
authorize; `ROADMAP.md`'s *What ships next* decides which release pays
for them, and a row there is not a release cut (`DEC-056`, and the
two-approval rule above that table).

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
| CSS-002 **(no release number)** | [a class with no rule](./CSS-002-a-class-with-no-rule.md) | `CSS-001` fixed the stylesheet and both documented paths, and **nothing enforces the relationship**: a utility class added to a component still has no rule until a person remembers to run a binary that `cargo build` never invokes. A `css_scan` over the static `class="…"` tokens, with a named allowlist for the classes that are JavaScript hooks rather than CSS | — | **Not started** |
| FLOW-001 | [five seconds to find the undo](./FLOW-001-five-seconds-to-find-the-undo.md) | Every undo is removed **5 000 ms** after it appears, with no pause on hover and none on keyboard focus — all four of `dm.js`, `board.js`, `calendar.js`, `plan.js`, and **WCAG 2.2.1**. The round pauses the timer on hover and on focus and does nothing else. **Its first direction — *delete the timer* — was reversed before it started**: each surface builds its own fixed-position container in the same corner, and two of them, measured, print over each other and cover the earlier Undo | — | **Not started** |
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
