# Handoffs — the checks that could not see it

**`0.63.0`, authorized by the owner 2026-11-12** — the row, not the cut.
Three defects a reader meets, all shipped, **each invisible to every
gate this project owns for a structural reason**. The repairs are small.
**The guards are the release.**

---

## How work lands — read this before starting a round

**Commit and push to the shared `main`.** No topic branch and no pull
request: the owner coordinates the schedule so that no two workers are
ever on the same tree at once, so a branch and its merge step add a step
and remove nothing.

**Whoever pushes watches `main`'s own CI to completion.** A red `main` is
that person's only work until it is green again — fix forward or revert,
never leave it for someone else to find.

Exceptions exist and are narrow: parallel work the owner has
deliberately scheduled, a patch to an already-released version, a change
that cannot be made in steps that each leave the trunk working,
throwaway experiments, and any branch that is never pushed. A
contribution from outside the team is the public workflow, not this one.

---

## Why these three are one release

Each defect shipped because **the gate nearest to it asks a neighbouring
question**:

| the defect | the gate that was green | what it asks | what nobody asked |
|---|---|---|---|
| six disclosures render no marker | `touch_target_scan` | is the control **big enough**? | does a disclosure still **look like one**? |
| five sentences read *"1 issues"* | three tests quoting the sentence | does the **tail** match? | does the word before it **agree**? |
| two header controls overlap at 768–800 px | `overflow-gate`, green at 768 | does the page **overflow**? | do two controls **cover each other**? |

**So each round owes a guard that could not pass while its defect is
present**, and in each case that guard generalises past the instance: a
source scan that no `<summary>` reaches `grow_text(`; a population that
renders **every** counted message at one; an occlusion check over each
pair of controls sharing a row.

A round that fixes its defect and ships no such guard has done the
smaller half.

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| MARK-001 | [the other six](./MARK-001-the-other-six.md) | Six `<summary>` elements still take `grow_text(`, so their `::marker` is never created — `me.rs` ×2, `settings.rs` ×2, `sprints.rs` ×2, measured at source. Carries a decision: `alignment_scan`'s literal exclusion list stops being *"a handful"* at six | — | **Not started** |
| GRAM-002 | [every counted sentence](./GRAM-002-every-counted-sentence.md) | The five remaining strings that read wrong at one — and `MessageKey::all()` gains a singular sample per count-bearing variant, so the **population pin** catches the next one rather than a hand-kept list | — | **Not started** |
| OCCL-001 | [two controls, one row](./OCCL-001-two-controls-one-row.md) | The account menu covers the search field's right edge by 13–18 px at two widths, and nothing in this project can see occlusion | — | **Not started** |

Independent of each other; any order.

## What this series deliberately does not carry

- **The typography and colour floor.** Its condition — *adopt after the
  views are decided* — has been met since unit three closed, and it is
  the next **design** row rather than a repair. Putting a new rule
  beside three repairs would blur what this release claims.
- **The ten ungoverned `<details>` elements.** A set, decided once;
  `external-design §5.10` says so in terms.
- `§10.15`, `§10.32`, `§10.50` and PostgreSQL P3, all carried.
- The two call-site counts whose population was never stated —
  `ROADMAP.md`, *Unscheduled work*.

## Where the numbers in these handoffs come from

Every figure was measured on 2026-11-12 against `0.62.0`'s own tree, and
one of them **corrects a previous round's report**: `DISC-001` named
*"`/today`'s own three summaries"* as what remained, and a source-level
census across every shipped component found **six markerless of
fourteen**, seven already correct, plus one site that uses no helper and
is not a defect. Read each handoff's own §1 before starting; where it
gives a population, re-derive it rather than inheriting it.
