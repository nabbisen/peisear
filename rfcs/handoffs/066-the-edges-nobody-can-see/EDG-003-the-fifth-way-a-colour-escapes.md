# `EDG-003` — The fifth way a colour escapes

**Round 3 of series `066`, and the last.** `EDG-001` and `EDG-002` are
**accepted**: all 67 focus rings draw at full `base-content` (**17.21:1**),
and all 70 boundaries at `/70` (**6.42:1** on a card, **5.24:1** on the page
body). Read `README.md` in this directory first.

**What the two rounds corrected, which this round inherits rather than
repeats:**

- **The architect's recommended `/50` would have shipped failing.** A
  semi-transparent border composites over **the element's own background**
  (`background-clip: border-box` is the default), and these controls paint
  their own white fill — so `/50` resolves to `#8c8c94` and reads
  **2.73:1** against the page body, below 1.4.11's 3:1. The index's own
  3.16:1 came from compositing the border over the page instead, which is
  not what a browser does. **`EDG-002` measured both candidates and chose
  `/70` as the only valid one.**
- **`contrast_scan` was never in reach of either defect**: `outline-color`
  and `border-color` are not `color:` properties.

---

## 1. The guard, and the one thing it must not try to do

Four mechanisms now exist, and **all four were built because a colour
escaped a guard whose requirement's words covered it**:

| # | how the colour escaped | release |
|---|---|---|
| 1 | a muted class below the floor | `0.28.0` |
| 2 | a semantic family used as text | `0.66.0` |
| 3 | a colour the stylesheet applies, no class involved | `0.67.0` |
| 4 | a **non-text** property — `outline-color`, `border-color` | this release |

**The pattern is the finding, not any instance**, and that list belongs in
the guard's own doc comment and in `§10.63`.

**Recommended shape: `PLC-003`'s acknowledgement machinery, extended to
boundary and focus properties** — enumerate `border-color` and
`outline-color` declarations in the served sheets whose colour resolves
below **3:1**, restricted to selectors a component can reach (the same
`written_class_tokens()` reachability `PLC-003` already computes), and
require each to be **acknowledged with what beats it** or **exempt with its
reason**.

**And the thing it must not do: compute the ratio.** `EDG-002` proved that
the arithmetic needs `background-clip` semantics, the element's own fill,
and which side of the border the adjacent colour sits on — three facts a
source scan does not have and the architect got wrong while holding all
three in his head. **So the measured figure is a human's, recorded in the
acknowledgement beside the selector, and the test asserts the
acknowledgement exists and its override is still present — never that a
number is right.** Say that in the code, so nobody later "improves" the
guard into computing it.

**Plant it twice**, as `PLC-003` did:

- **a newly reachable below-3:1 rule**: `.toggle:focus-visible` is `/20` in
  the pinned sheet and the product writes **zero** `.toggle` sites today
  (confirmed by grep) — add one in a scratch worktree and watch the guard
  fail by name;
- **an acknowledgement losing its proof**: remove one of this series' own
  `app.css` rules and watch the entry that names it fail.

Quote both failures, remove both plants.

---

## 2. The specification: a new sibling, not a fourth clause

`NFR-A11Y-005` says *"Text and background combinations MUST meet WCAG AA
(4.5:1)"*. **1.4.11 is a different criterion with a different threshold for
different things** — a control's boundary and its focus state at 3:1 — and
three releases of stretching one entry's *Acceptance* line is enough.

**Recommendation: add `NFR-A11Y-013 — Non-text contrast`**, the next free
number in that family (001–012 exist), stating: the visual information
needed to identify a control and its state MUST meet 3:1; its *Acceptance*
is this round's new mechanism plus the measured figures; *Status* **Met at
0.68.0** (`EDG-001`–`EDG-003`); *Priority* **P1**, matching `NFR-A11Y-005`.
`NFR-A11Y-005` then **cites** it rather than absorbing it — the convention
this document already uses (`FR-NAV-006` cites `external-design §5.10`
rather than restating it).

**This moves counts.** `Appendix A`'s own Accessibility row and the
non-functional totals are derived from each entry's `*Status*` line, and
`spec_scan` asserts the entry population (**171** at the last count it
recorded). **Re-derive all of them from the document rather than
incrementing**, and expect `spec_scan`'s own population assertion to go red
first — which is that assertion working, exactly as `0.59.0` recorded when
it added `NFR-CMP-007`.

**If you think a clause on `NFR-A11Y-005` is the better design**, say so as
a finding with the reason and stop — that is a decision, and the architect's
recommendation is not the last word on it.

---

## 3. `§10.63`, opened and closed in this round

Record, because a later reader needs these and not a summary:

- **The class and its four-instance table** from §1.
- **The defect itself**: 67 focus rings and 70 boundaries at **1.52:1**
  against 3:1 — *a keyboard reader could not see where they were, and on a
  card the only thing saying a field existed was a line at one and a half
  to one.*
- **The architect's own error, with its cause**: the recommended `/50`
  failing at 2.73:1 because a border composites over the control's own
  fill. **This is the fifth time in three series that a figure he computed
  was wrong where a measurement was available**, and the first where the
  wrong figure was the recommendation itself.
- **What the rounds did that the handoffs did not ask**: real `Tab` presses
  rather than `.focus()`; validating the sampling point with
  `elementFromPoint` after a fixed direction landed on a neighbouring
  button; reading both checkbox states off the page's own defaults instead
  of scripting a click; and settling an appearance question by looking at a
  screenshot.
- That the class was found by **scoping a release**, like `§10.60`–`§10.62`.

---

## 4. The `rfcs/` count, and the tax it has been charging

`spec_scan`'s `rfcs_citations_to_the_four_numbers_are_counted_not_enforced`
counts citations of four dangling register numbers across `rfcs/`,
**including `rfcs/handoffs/`** — which `docs_scan` rule 5 excludes **by
name** as *execution records, not documentation*. **The architect reddened
`main` three times on it at `0.67.0`**, once unwatched.

**Make this count share rule 5's own exclusion**, re-derive the constant
from the remaining population by hand, and record in the test's own doc
comment that the two scans now agree about what `rfcs/handoffs/` is. If the
remaining count is zero, say so plainly rather than deleting the test — it
is a historical record, and a record of zero is still a record.

---

## 5. The series' closing, which this round also owes

1. **The failing population re-derived to zero**, with the command, for both
   families — boundaries and rings.
2. **A reconciled rounds table** in this directory's `README.md`.
3. **What the series leaves behind, named**: card and alert borders
   (decoration beside a shadow, out by 1.4.11's own test), `.radio`/`.toggle`
   with zero sites today, the four recorded text candidates, and the
   browser-rendered sweep — **named for the third time now**, which is
   itself worth recording.
4. **No new rule beyond the ban and the acknowledgements.**

---

## 6. Acceptance

1. The guard added, **planted twice**, both failures quoted, both plants
   removed; and **the code says in terms that it does not compute ratios and
   why**.
2. `NFR-A11Y-013` added (or a reasoned finding against doing so), with
   `NFR-A11Y-005` citing rather than absorbing it.
3. **Appendix A's counts and `spec_scan`'s entry population re-derived from
   the document**, not incremented.
4. `§10.63` opened and closed with §3's contents, including the architect's
   own error and its cause.
5. The `rfcs/` count sharing rule 5's exclusion, its constant re-derived.
6. The failing population at **zero**, stated with its command.
7. The full gate line per `README.md §5` with real exit codes, the
   `DEC-007` count (**691** as it stands — expect it to move, since this
   round adds tests), and `contrast_scan` run **after** the new mechanism
   lands.
8. `CHANGELOG.md` under `[Unreleased] → Fixed` for the series' close, plus
   one line for the guard; and **`Changed` is wrong for it** — this was a
   defect.
9. Privacy floor per artefact; scratch under `.git-exclude/tmp/`.
10. **Re-derive the `rfcs/` citation count after your last edit and before
    pushing** — until §4 lands, it still bites.
11. **Review request** at
    `.git-exclude/review-request/EDG-003-the-fifth-way-a-colour-escapes/README.md`.

**After this round, series `066` is closed** and `0.68.0` goes to a release
candidate against exit criteria 1–6. **If a measurement contradicts this
file, the measurement wins** — twelve of the architect's figures or
instructions have been corrected that way across four series, and the last
one would have shipped a failing fix.
