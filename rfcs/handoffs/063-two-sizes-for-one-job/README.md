# 063 — Two sizes for one job

**Series index.** `0.65.0`'s scope, authorized by the owner 2026-11-18 —
**the row, not the cut**. Opened the same day.

`0.64.0` read the smallest text on three screens, raised sixty of its
sixty-one sites, and **declined to write a type rule** (`§10.60`, closed by
a recorded decision that no requirement should govern text size, because the
size answer converged). This series reads the five screens that release did
not reach, and it writes no rule either.

---

## 1. What this series is about, in one measurement

The product now has **two sizes for the same job**, and the line between
them is not a design decision — **it is which screens have been read.**
Measured against the `0.63.0` tag rather than remembered:

| class | at `0.63.0` | today |
|---|---|---|
| `label-text-alt text-xs` (a field's hint) | 12 | **7** |
| `label-text-alt text-sm` | 0 | **5** |
| `label-text text-xs` (a form label) | 11 | **8** |
| `label-text text-sm` | 51 | **54** |

*(the `0.63.0` figures are `git grep -c` at the tag, which counts comment
lines too, so they are an upper bound; today's are render sites only, by the
command in §2.)*

**Before `0.64.0` the hint class was uniform.** That release raised the five
hints on the screens it read and left the seven it did not, and the markup is
otherwise identical on both sides of that line — six of the seven small
hints carry `opacity-70` and one carries `text-base-content/70`, mirroring
four and one among the five raised. **Nothing but the read boundary separates
them.** This series finishes that edge; it is `0.64.0`'s own unfinished work
and is described that way deliberately.

---

## 2. The census, and the command that measures it

A handoff that names a population owes the command that measures it —
`0.64.0` published five inflated figures taken with a plain `grep -c`, which
counts a comment *mentioning* a class alongside a site that *renders* it:

```bash
grep -rn 'text-xs' crates/ --include='*.rs' | grep -vE ':[[:space:]]*//' | wc -l
```

**68 render sites remain**, across ten component files.

| in scope (**47**) | sites | out of scope (**21**) | sites |
|---|---|---|---|
| `sprints.rs` | 20 | `calendar.rs` | 11 |
| `teams.rs` | 10 | `notification_preferences.rs` | 4 |
| `settings.rs` | 8 | `notifications.rs` | 3 |
| `sprint_plan.rs` | 5 | `search.rs` | 2 |
| `auth.rs` | 4 | `layout.rs` | 1 |

**Re-measure before each round and say what you measured.** If a figure here
disagrees with the tree, **the tree is right and this file is wrong** — say
so in the round's report rather than working around it.

---

## 3. The rounds

| round | what it reads | sites |
|---|---|---|
| `LBL-001` | both label conventions, product-wide, decided once — **closed**, two rounds | 15 |
| `SIZE-001` | `sprints.rs`'s remaining body and meta text, and its two stale comment sums | 18 |
| `SIZE-002` | `teams.rs`'s remaining 7 and `sprint_plan.rs`'s remaining 2, and the gate coupling in §6 | 9 |
| `SIZE-003` | `settings.rs`'s remaining 2 and `auth.rs`'s remaining 3 — including the sign-in page's own flash message, the front door at 12 px | 5 |

15 + 18 + 9 + 5 = **47**, the in-scope census above.

**Rounds closed so far.** `LBL-001` **accepted** 2026-11-18 (`bb79b15`): all
15 sites changed — the eight labels by *deleting* the overriding `text-xs`,
the seven hints by raising to `text-sm` — plus the two `settings.rs`
dropdowns raised as clusters, `input-xs` leaving the tree, and
`css_scan`'s population moved `290 → 289` with its reason. **The census is
now 53.** The round **corrected this series' own handoff**: the
`24px → 32px` control-height cost it was told to measure cannot happen,
because `grow()`'s `min-h-11` (44 px) already dominates both classes' own
heights. **Round 2 accepted** 2026-11-18 (`924b88f`), and **`LBL-001` is closed**:
the six hints converted to `text-base-content/70` after a per-site
single-level check, zero exceptions, re-measured at **6.42:1** each (from
`6.51:1`, within the same pixel noise `§10.60` recorded for the two
spellings); the button tier **stated and not raised** in both cards; and
`css_scan/tests.rs`'s clause corrected to name that scan's own scope.
Measured in `components/`: bare `opacity-70` **22 → 16**, guarded
`text-base-content/70` **149 → 155** — *the round's own report said 150,
which is the one figure it got wrong, and nothing in the repository carried
it.* What the conversion buys is bounded and worth stating: `contrast_scan`
is a **ban** on `/10`–`/60`, not a coverage counter, so a later dimming of
those six is now refusable where before it was invisible.

**Round 3 is open** —
[`SIZE-001-the-sprint-screens-own-small-print.md`](./SIZE-001-the-sprint-screens-own-small-print.md):
`sprints.rs`'s 18 remaining sites, and the **last two `text-xs`-keyed
comment sums in the product**, which its own Group A moves.

**One round at a time**,
each handed separately and reviewed before the next opens. A round may be
followed by a second round on the same scope; that is normal and happened
twice in `062`.

---

## 4. Method — inherited from `062`, unchanged

1. **Measure the cost, never assume it.** Every size change is measured at
   **390 px and 1280 px**: the element's own rendered box, and whether
   anything it sits in grew, wrapped or overflowed. *"It should be fine"* is
   not a measurement.
2. **Removal is on the table in every round**, and in both directions — a
   site may be left as it is, raised, or have its text removed, and *"this
   label should not exist"* is a legitimate finding. `0.64.0` raised sixty of
   sixty-one and left one deliberately.
3. **Measure contrast where a size meets an opacity.** `NFR-A11Y-005`'s own
   entry states its two limits in terms: `contrast_scan` **does not cover
   bare `opacity` at all**, and **does not know which background a passing
   modifier sits over**. In scope, `text-xs` meets a bare `opacity-*` at
   **eight** sites — `sprints.rs:531`, `sprints.rs:1163`, `teams.rs:169`,
   `teams.rs:184`, `teams.rs:538`, `settings.rs:81`, `settings.rs:205`,
   `auth.rs:92`. Measure the rendered pair in a browser. `0.64.0` measured
   four such sites and all four passed; that is a reason to measure, not a
   reason to assume.
4. **No new type rule, and the one condition that would reopen the
   question.** `§10.60` is closed on the decision that no requirement should
   govern text size. Apply the majority the product already has; **do not
   write a floor, a scale or a rule**, and do not reopen that entry. **If a
   round finds the right answer differs site by site on these screens**, that
   is evidence the other way: say so in the report, with the sites, and let
   the architect decide. A round does not adopt a rule quietly and does not
   reopen `§10.60` on its own authority.
5. **A new class token must have a rule in one of the three stylesheets the
   binary serves.** The CSS is **vendored and pinned** (`ASSET-001`,
   `DEC-051`), not compiled, so an invented arbitrary value resolves to
   nothing at all. `css_scan` enforces this; `text-sm` and `text-xs` both
   exist, `min-h-11` exists, a new `text-[13px]` does not and will not.
6. **The stylesheet is the authority on what a class does, not its name.**
   `§10.52` is the entry for reasoning about CSS from Rust: three review
   rounds, a required prop and a compile-error plant all passed while the
   shipped navigation computed `display: none` at every width.

---

## 5. What the stylesheets actually say (read, not assumed)

| class | file | font-size / line-height |
|---|---|---|
| `.text-xs` | `static/tailwind.css` | `.75rem` / `1rem` (12 px / 16 px) |
| `.text-sm` | `static/tailwind.css` | `.875rem` / `1.25rem` (14 px / 20 px) |
| `.label-text` | `static/daisyui.min.css` | `.875rem` / `1.25rem` — **identical to `.text-sm`** |
| `.label-text-alt` | `static/daisyui.min.css` | `.75rem` / `1rem` — **identical to `.text-xs`** |

`layout.rs:68–70` serves `daisyui.min.css`, then `app.css`, then
`tailwind.css` — **tailwind last**, so at equal specificity (one class each)
a `text-*` token wins over a `label-text*` token. That ordering is already
load-bearing and is documented at `layout.rs:34`.

**Consequence, and it changes the shape of `LBL-001`:** the eight small form
labels carry a `text-xs` that **overrides a component class already saying
14 px**, so the fix there is a **deletion**. The seven small hints carry a
`text-xs` that is **redundant** with `.label-text-alt`'s own 12 px, so the
decision there is genuine — 14 px on all twelve, or 12 px on all twelve.
**Derived from the files above; still measure the rendered `font-size` in a
browser before and after.** A read of a stylesheet is a better claim than a
guess and a worse one than a measurement.

---

## 6. Two traps in this series' own path

**`sprints.rs:436` and `sprints.rs:983`** each state, in a comment nothing
reads, `py-4 = 32px + text-xs's own 16px line-height = 48px`. `SIZE-001`
raises that screen, which invalidates both. **The 44 px floor itself is not
at risk**: `grow_summary()` composes `TOUCH_TARGET` (`min-h-11 min-w-11` =
`2.75rem` = 44 px) and `alignment_scan` enforces that every real `<summary>`
reaches that one helper — one bare `<summary>` excepted by its exact literal
tag (`alignment_scan.rs`'s `NAMED_ESCALATION_EXCLUSION_TAG`, `issues.rs:650`,
outside this series). So **correct the arithmetic in the comment; do not
invent a mechanism to hold a property that is already held.**

**`browser-checks/overflow-gate.mjs:538`** validates that the board fixture
rendered a workload chip by quoting the literal `text-sm font-medium`. That
literal **already broke once**, during `0.64.0`, on a round that never
touched the gate — and the class it now quotes matches **seven** sites in
`issues.rs` rather than one. **`SIZE-002` deletes the coupling rather than
checking it**: give the chip (`issues.rs:717`) a named hook and have the gate
match that. `css_scan` already tracks named JavaScript hooks. **No new scan.**
And per the standing rule: **editing a gate means running that gate**, not
reading its other call sites.

---

## 7. Deliberately not in this series

- **The 54 `label-text text-sm` sites.** That token is redundant with
  `.label-text`'s own size and renders identically with or without it.
  Deleting it is a 54-site diff with nothing a reader can see, across screens
  this series does not read. **Recorded as a candidate in `ROADMAP.md`, not
  scheduled. No round takes it on its own initiative.**
- **The 21 out-of-scope sites** (`calendar.rs`, both notification surfaces,
  `search.rs`, `layout.rs`) — calendar density and navigation chrome, a
  different population, decided after this series shows what the read costs.
- **`/today`'s content** — the owner's open question, not a defect, and not
  this series'.
- **New fixture branches for the browser gates.** `§10.32` has its own
  stopping rule: an unrendered branch is recorded in
  `browser-checks/README.md` and **not chased** unless a defect turns up
  behind it. Measure a closed-by-default surface **in a browser by opening
  it**; do not grow the fixture to reach it.

---

## 8. What each round owes when it reports

1. **The command and the number**, for every population it names.
2. **Before and after, measured**: the rendered `font-size`/`line-height` per
   changed site, and the layout cost at **390 px and 1280 px**.
3. **Every site it did not change, with the reason** — a site left small is a
   finding, not an omission.
4. **Contrast ratios** for any site where a size meets an `opacity-*` or a
   `/NN` modifier, with the background it was measured over.
5. **The gates it ran**, with the real exit code — `cmd | tail -1; echo $?`
   reports `tail`'s status, which has hidden a missing tool here for days.
   The `DEC-007` block is 42 commands per `.github/CONTRIBUTING.md`; the count
   stands at **687** and a round says what it became.
6. **`css_scan`'s population assertion** (`css_scan/tests.rs`, currently
   `(290, 2)`) is **re-measured, not assumed**. Both `text-xs` and `text-sm`
   stay in use elsewhere, so it probably does not move — and if it does, the
   assertion is updated **with the reason**, which its own failure message
   demands.
7. **The privacy floor, per artefact.** Any screenshot, trace, log or HTML
   dump attached to a report is **inspected before it is attached** and must
   carry no local paths, hostnames, account names, session cookies or other
   local-environment detail. Non-text artefacts are not scanned by anything —
   this is a named exit condition for every round, not a reminder.

---

## 9. Where things go

- **This series' handoffs** are published here, one file per round.
- **A round's review request** is its own package with its own entry point:
  `.git-exclude/review-request/<round>-<slug>/README.md`. Reviews stay
  private and are never published.
- **Release numbers live in `ROADMAP.md` and nowhere else.** This file states
  rounds and their order; it does not assign versions.
- **No round tags, publishes or creates a release.** `DEC-056`'s four legs
  are the architect's, on the owner's approval at **exit criterion 7**, which
  is a separate decision from the authorization this series was opened on.
