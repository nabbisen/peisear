# VIEW-001 — work first

**Series**: `060` (unit three, each view), `0.62.0`'s authorized scope.
**Blocked on**: nothing.

## 1. The defect, measured

On the project board and list — the two screens a user of a tracker
spends their time on — the reader's own work begins past the middle of
the viewport, and on a phone it is effectively off-screen:

| Screen | 1280×900 | 390×844 |
|---|---|---|
| Board — first column | y=**508** (0.56 screens), **17** controls above | y=**616** (0.73), 11 above |
| List — first row | y=**576** (0.64), **22** controls above | y=**752** — **0.89 of the viewport** — 19 above |

What fills that space, in render order (`components/issues.rs`, the
`ProjectDetailPage` view): breadcrumb, back link, the title row with its
four controls, **`HealthStrip`** — a section heading, a sentence, an
`Indicators` `<details>` and six indicator chips each carrying its own
expander — then **`WorkloadStrip`**, then the announcement regions, then
the board or the list.

Every one of those blocks is worth having. **The finding is their
order and their default state, not their existence.**

## 2. The decision

**A project screen's first content is its work. Analysis about the work
is one line until the reader asks for it.** The series README carries
the four clauses and the reason behind each; they are the round's
contract and are not re-litigated here.

In short: `HealthStrip` collapses to a **single line** — the
plain-language sentence of the highest-precedence non-healthy indicator,
or one neutral line when nothing needs attention — and expands, on the
reader's own action, to exactly the block that ships today.

**The precedence chain is `FR-PER-006`'s, transposed.** That entry
already defines one for `/today` — sustained burnout, then WIP over
limit, then long-stale work. The project screen's equivalent ranks
*indicators*, and the round proposes the ranking **in its report, with
the reason for each position**, for the architect to accept or change.
Do not invent a composite to sort by: `FR-HLT-008` forbids the
composite as a headline, and sorting by it would make it one.

## 3. What to build

- `HealthStrip` gains a collapsed form: one line, a disclosure control
  with a word (never a glyph alone — `058`'s own lesson), expanding to
  today's content unchanged.
- `WorkloadStrip`'s own place is a question this round answers **with a
  measurement, not a preference**: report where the first work lands
  with the health block collapsed and the strip left where it is, and
  with both collapsed. If the strip alone does not move the number
  meaningfully, leave it exactly where it is and say so.
- Nothing is removed, nothing is relocated below the work, and no
  indicator loses its basis route (`FR-HLT-007`).
- **No persistence.** The collapsed state does not survive a reload.
  Recommended rather than assumed, and the reason is the reader this
  sequence is for: a state remembered from one visit silently restores
  the defect for the reader who expanded it once and forgot.

## 4. Acceptance

- **The measurement above, re-run and reported, at both widths, for both
  views.** The round's own number is the evidence: first-work `y`,
  the fraction of the viewport, and the count of controls above it,
  before and after. A round that reports only "it is collapsed now" has
  not shown the thing it changed.
- A test that the collapsed summary **is a sentence and not a score** —
  pinned against `MessageKey`, the way `FR-HLT-006`'s own neutrality
  guard is, so a future edit cannot slip the composite into the slot.
- A test that a healthy project renders **one** neutral line and no
  callout, which is `FR-PER-006`'s rule in its new place.
- Every existing health test stays green and unchanged, including
  `basis_route`'s five.
- `browser-checks` 120 cells, and `touch_target_scan` on the new
  disclosure control.
- `DEC-007` from → to, measured with the runner.

## 5. What this round must not do

- **Do not touch `/today`.** Its panels are specified (`FR-PER-007`) and
  its own open question is the owner's.
- **Do not collapse anything else.** The thirteen other ungoverned
  `<details>` are a set, decided once, after this round shows what the
  rule should be.
- **Do not write `external-design §5.10`** or amend a requirement. The
  architect states the rule once, when this round closes.
- Do not change any indicator's computation, wording, or basis.
