# TYPE-001 — below the floor there is no floor

**Series:** [062 — the smallest text on the screen](./README.md)
**Release:** `0.64.0`, authorized 2026-11-17 — the row, not the cut
**Blocked on:** nothing. This is the series' first round.
**Register entry:** `§10.60` (opened 2026-11-17, **open**)

Read the series `README.md` first, in particular *Removal is on the
table in every round* and *What this series does not promise*. **This
round does not adopt a type floor.** If you finish it believing a floor
is obvious, write that down as a finding — do not implement it.

---

## §1 The population — six sites

Every rendered site in the workspace whose text size is written as an
arbitrary value below `text-xs` (12 px), measured 2026-11-17 against
`0.63.0`'s tree:

| # | Site | Size | Colour | Renders on |
|---|---|---|---|---|
| 1 | `crates/peisear-web/src/components/calendar.rs:110` | `text-[10px]` | **bare `opacity-70`** | `/calendar` month and week blocks — project-name badge |
| 2 | `crates/peisear-web/src/components/calendar.rs:185` | `text-[10px]` | **bare `opacity-70`** | `/calendar` day view — project-name badge |
| 3 | `crates/peisear-web/src/components/calendar.rs:216` | `text-[10px]` | **bare `opacity-70`** | `/calendar` day view — the 24 hour labels |
| 4 | `crates/peisear-web/src/components/calendar.rs:310` | `text-[10px]` | **bare `opacity-70`** | `/calendar` month view — the *"+N more"* line |
| 5 | `crates/peisear-web/src/components/projects.rs:77` | `text-[11px]` | `text-base-content/70` | every project card — *"Updated &lt;date&gt;"* |
| 6 | `crates/peisear-web/src/components/issues.rs:1028` | `text-[11px]` | `text-base-content/70` | every board card — the whole metadata row (priority, effort, assignee, date) |

**Re-derive this before you start.** The command that produced it:

```bash
grep -rn 'text-\[[^]]*\]' crates/ --include='*.rs'
```

**That command returns seven hits, and the seventh is not a site.**
`crates/peisear-web/src/css_scan/tests.rs:87` contains
`escape_css_token("text-[11px]")` — a test asserting how an arbitrary
value is escaped. It renders nothing. **If your count is seven, this is
why**; if it is any other number, the tree has moved since this handoff
and your number is the right one — say so.

**Reachability, checked rather than assumed.** Sites 1 and 2 render only
when `project_badge_for` returns `Some(name)`, which is **the personal
axis only** (`/calendar`); on a project's own calendar the badge is
`None` by design (`calendar.rs:83–91`, `RFC 002` must-have 6). They are
reachable and real — but if you build a fixture, build it on the
personal axis or you will be looking at nothing.

## §2 What these six actually carry

Two of the six are not decoration:

- **Site 6 is an entire metadata row**, not one label — priority badge,
  effort, assignee and date, on every card on the board. It is the
  densest 11 px in the product and it is on one of the two screens a
  user looks at most.
- **Site 5 is the only recency signal on a project card.** *"Updated
  &lt;date&gt;"* is how a reader tells a live project from a dormant
  one, and it is the smallest text on that card by a step.

Sites 3 and 4 are navigational scaffolding (hour labels, an overflow
count). Sites 1 and 2 answer *which project is this block* — the
deliberately-textual answer to a question this project refuses to answer
with colour alone.

**None of the six is a candidate for silent enlargement.** Each one is
in a box whose size was decided by something else: a `h-24` month cell,
a `line-clamp-2` card, a percentage-positioned absolute block. Raising
the text without measuring the box is how this round produces an
overflow defect instead of a reading.

## §3 The contrast nobody has measured — sites 1–4

**This is the half of the round that is a measurement rather than a
decision, and it is the reason this round is first.**

`NFR-A11Y-005` is **Met**, `/70` is the muted-tier floor, and
`contrast_scan` guards it. **The requirement's own entry states two
limits of that guard in terms** — read it at source
(`docs/specification/requirements.md`, `NFR-A11Y-005`) rather than
trusting this paragraph:

1. It bans a **modifier range** (`text-base-content/{10..60}`). It
   **does not cover bare `opacity` at all** — `opacity` is not a text
   property, and telling text from non-text needs rendering, which this
   project does not do (`§10.15`'s standing limit).
2. It **does not know which background a passing modifier sits over.**

**All four 10 px sites use bare `opacity-70`.** And two of them sit on a
tint, not a theme base colour:

- Site 1 is inside `<a class="… bg-primary/10 hover:bg-primary/20 …">`
- Site 2 is inside `<a class="… bg-primary/15 hover:bg-primary/25 …">`

That second fact is exactly the case `QA-013` had to check by hand —
its own note records *one site that sat on a `bg-primary`-tinted
background instead of a theme base colour*, and another at **2.64:1**,
worse than a tier the rule bans outright. **Those checks were done over
21 bare-opacity sites. Today there are 31 such lines under
`components/`.** Treat that as two populations that may not have been
defined identically rather than as a drift of ten — and state which one
you measured.

**What this round owes.** For each of sites 1–4, the **rendered**
contrast ratio, measured as composited by a browser, against:

- the resting background, and
- the `hover:` background where one exists (sites 1 and 2).

Report a ratio per site per background, with the method, and say
explicitly whether each clears **4.5:1**. **A pass is as much a result
as a failure here** — nothing has ever measured these, so the number is
the deliverable either way.

Two things to get right:

- **Measure as rendered, not by arithmetic.** The arithmetic is what got
  `QA-013`'s three nested sites wrong. `opacity` composites the
  element's whole rendering including an inherited colour that already
  carries alpha; `text-base-content/N` does not.
- **The `<a>` carries no `opacity`** at sites 1 and 2 — the `opacity-70`
  is single-level there, on a sibling of another `opacity-70` span. Do
  not report compounding you have not observed.

**If a site measures below 4.5:1, that is a defect in a Met P1
requirement** and it goes in `§10.60` with its number, not into a quiet
fix. Say so in your review request before repairing it.

## §4 The decision each site needs

For **each** of the six, return one of three outcomes with its own
recorded reason:

1. **Keep it at its current size.** Then say what makes 10 or 11 px
   right *here* when 12 px is the scale's own smallest step — and note
   that six sites are the only places in the product that go below it,
   which is itself an argument that wants answering.
2. **Raise it to `text-xs`** (12 px — the scale's existing floor, not a
   new value). Then **measure what it cost**: the containing box's
   height and whether anything overflows or re-wraps, at **390 and
   1280**, on the real pages. Density is the objection that correctly
   killed the 2026-11-04 proposal; it is answered in pixels or not at
   all.
3. **Remove the information.** Fully available, and for sites 3 and 4
   genuinely worth asking. If a reader does not need the 24th hour
   label or the *"+N more"* count, the kindest type change is deleting
   it.

**Do not invent a fourth size.** If the answer for some site is neither
10 px nor 12 px, that is a finding about the scale, and the scale is
`TYPE-003`'s business. Write it down; do not act on it.

**Replacing a bare `opacity-70` with `text-base-content/70` is in scope
where the site renders text**, and is the better spelling where it is
equivalent — it brings the site inside the guard's reach. **It is not
equivalent on a tinted background**, and sites 1 and 2 are on tints, so
that substitution is a change to measure, not a tidy-up. Sites 3 and 4
are the easier case. `calendar.rs:292` and `:328` are **not** in this
round: they are empty `<td>` cells dimming a border, explicitly excluded
as non-text by `QA-013 §2` and by `contrast_scan`'s own doc comment.

## §5 What this round must not do

- **Must not adopt a type floor, write a type requirement, or add a scan
  over text sizes.** Rule after design; the floor is decided after
  `TYPE-003`.
- **Must not change `text-xs` sites.** **120 render sites** exist: 55
  belong to the two later rounds (21 on `/today`, 34 on the project
  screens) and 65 are deliberately excluded. **This round's six sites
  are not among them** — none of the six is a `text-xs` site; that is
  the point of the round. (*Corrected 2026-11-17 after this round. The
  sentence first written here said "126 of them exist and 120 belong to
  the two later rounds", which was wrong twice: 126 was a `grep -c` that
  counted six comments mentioning the class, and the second figure
  subtracted this round's six sub-12 px sites from a `text-xs` census
  they were never in. See the series `README.md`, "Where the numbers in
  these handoffs come from".*)
- **Must not reopen `NFR-A11Y-005`.** Measuring four sites it never
  covered is not reopening it. Amending it is a separate decision and it
  is the architect's.
- **Must not change `contrast_scan`.** Widening it to bare `opacity`
  needs rendering, which is `§10.15`, which is carried and not this
  round.
- **Must not touch the two empty `<td>` cells** (§4, last paragraph).

## §6 Acceptance

**Do not take this list on trust — three criteria in this series'
predecessor were stale or wrong, and all three were caught by the
rounds.** Each item below names the record it comes from; read the
record.

1. **A measurement table for sites 1–4** — ratio per site per
   background, with method, and a clear pass/fail against 4.5:1 per
   cell. This is the round's headline deliverable.
2. **A decision per site for all six**, each with its reason, drawn from
   §4's three outcomes.
3. **Where outcome 2 was taken: the height delta measured at 390 and
   1280 on the real page**, not on a fixture that renders one card.
4. **Tests run per `.github/CONTRIBUTING.md` → *Before you open a pull
   request* → *Tests*** — its own current block, read there rather than
   copied from here. `DEC-007`'s baseline at `0.63.0` is **687**; report
   the figure from the runner, counted with `wc -l` on the artefact, not
   by arithmetic over the log.
5. **`cargo test -p peisear-web --lib`** — **246** at `0.63.0`;
   re-derive rather than assume.
6. **`node browser-checks/overflow-gate.mjs` must be run** if any class
   in a rendered component changed, and its result reported. A text-size
   change moves boxes; this gate is the one that reads boxes. The same
   applies to `occlusion-gate.mjs` if a change reaches the site header
   (it should not, but say so either way).
7. **Clippy as CI runs it**, both feature combinations:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings
   ```
   The second line's `--no-default-features` is not optional — without
   it the command exits 101 for a reason that has nothing to do with
   your change.
8. **`§10.60` amended with what was measured**, including a pass that
   found nothing. The entry stays **open** until `TYPE-003` — do not
   close it.
9. **Any screenshot or other non-text artefact must be checked for
   local information before it is committed** — a path under `/home`, a
   real display name, an address, a hostname. Named as a per-artefact
   exit condition because non-text artefacts are not scanned by
   anything.

**No `CHANGELOG.md` entry is required from this round unless a
user-visible size or string changes** — if one does, it goes under
`[Unreleased]`, and whichever release is cut next picks it up in its own
`DEC-028` sweep.

## §7 Where to file a question

**A blocking question goes in a file, not in chat.** Open
`.git-exclude/review-request/TYPE-001-…/README.md` with the question,
what you tried, and what you are declining to do without a ruling —
`GRAM-002` round 3 is the precedent, and that round was right to stop.
