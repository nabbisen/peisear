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
| CSS-002 | [a class with no rule](./CSS-002-a-class-with-no-rule.md) | `CSS-001` fixed the stylesheet and both documented paths, and **nothing enforces the relationship**: a utility class added to a component still has no rule until a person remembers to run a binary that `cargo build` never invokes. A `css_scan` over the static `class="…"` tokens, with a named allowlist for the classes that are JavaScript hooks rather than CSS | — | **Not started** |
| FLOW-001 | [five seconds to find the undo](./FLOW-001-five-seconds-to-find-the-undo.md) | Every undo in the product is drawn in the corner furthest from the control that triggered it and is removed **5 000 ms** later, with no pause on hover and none on keyboard focus — measured in all four of `dm.js`, `board.js`, `calendar.js`, `plan.js`. For the reader the owner named, that is an affordance announced and withdrawn before it can be reached | — | **Not started** |
| FLOW-002 | [the word on the door and the name inside](./FLOW-002-the-word-on-the-door.md) | Two of the five destinations are called one thing in the navigation and another on arrival — *Inbox* → **“Notifications”**, *Today* → **“My dashboard”** — and of the page titles in `en.rs`, five end with *“— peisear”* and the rest do not, so three of the product's own screens are called nothing but **“Calendar”**, **“Notifications”** and **“My dashboard”** in a tab strip, a bookmark or a history list | — | **Not started** |

---

## The decisions these rounds implement, and why they are not the dev team's

**A class a component writes must have a rule the product serves.** Not
*should*; the relationship has to be checkable by something that runs,
because the one thing between it and silence today is a sentence in a
README that was itself wrong for three weeks. The guard reads Rust and
CSS, both already in the repository, and needs no browser — so it costs
one scan and no CI minutes (`CSS-002`).

**An undo must not expire while the reader is still looking for it.**
The recommendation is to **delete the timer**, not to tune it: a toast
that lives until it is replaced, dismissed, or the reader leaves the page
is both simpler to explain and simpler code than a timer that pauses on
hover and on focus and resumes correctly on both. *Less is more* applied
to the mechanism rather than to the screen. The alternative — pause on
hover and focus, floor raised — is recorded in the round because it is
the one a reviewer would otherwise propose (`FLOW-001`).

**The word on the door is the name inside.** `FR-NAV-001` fixes the five
words; the destinations must answer to them. A reader who clicks *Inbox*
and arrives at *Notifications* is being asked to hold two names for one
place, which is the precise cost `058` was authorized to remove
(`FLOW-002`).

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

**2. The home screen's substance is behind three closed disclosures.**
At 1280 px, `/today` occupies the top 290 px of a 900 px viewport and
the left 40 % of the width; its section labels are the smallest, faintest
text on the page, and under one of them (*RHYTHM*) nothing is visible at
all until it is opened. That is a **view** question, not a workflow one —
it belongs to unit three, recorded here so the reading is not lost.
