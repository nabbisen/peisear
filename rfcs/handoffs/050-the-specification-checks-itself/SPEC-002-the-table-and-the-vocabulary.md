# SPEC-002 — the table and the vocabulary

**0.57.0, round 2 of 3.** Blocked on `SPEC-001` only for the module
it adds the rules to.

Appendix A is correct today. It has been wrong before — *"wrong in two
compensating cells, so the total looked right"* is the first row of
`§10`'s own table of things a scan would have caught. This round makes
it stay correct, and closes the status vocabulary it depends on.

---

## §1 — measured 2026-10-22 at `7c52515`

**The totals agree with the entries**, which is worth knowing before
guarding them:

| | Entries counted | Appendix A says |
|---|---|---|
| `FR-*` | 99 | 99 |
| `NFR-*` | 69 | 69 |
| **total** | **168** | — |

Appendix A's own arithmetic is also consistent, recomputed from its
rows: functional `99 \| 77 \| 4 \| 16 \| 0 \| 2`, non-functional
`69 \| 57 \| 4 \| 0 \| 2 \| 6`.

**Read that as a starting state, not as a guarantee.** It is true at
this commit because it was recomputed twice this month by hand. The
point of the round is that nothing would notice when it stops being
true.

## §2 — two traps, both of which caught me while scoping

**Trap 1: `NFR-A11Y`.** An ID whose area segment contains digits. The
obvious pattern — `(FR|NFR)-[A-Z]+-[0-9]+` — matches **none** of the
eleven accessibility requirements, and I used exactly that pattern
while scoping this release. It gave 58 non-functional entries against
Appendix A's 69, and **I only noticed because the two disagreed.** A
scan with that pattern would have been green having examined 58 of
69 — `§10.36` inside the guard itself.

**Derive the ID pattern from the data.** The twenty-five area
segments in use today are: `API AUTH CAL DM HLT ISS NAV NTF PER PLAN
PROJ SCH SET SPR SUB TEAM` (functional) and `A11Y CMP CONC LANG MNT
PERF PRIV REL SEC` (non-functional). **The rule should assert the
count it examined**, so that a pattern which silently matches fewer
entries fails rather than passes.

**Trap 2: a status line is not one per entry.** There are **180**
`*Status*` lines and **168** entries. Every entry has at least one;
**five have more than one**, and `NFR-CMP-006` has **nine**:

```
FR-NAV-003: 2   NFR-A11Y-002: 2   NFR-REL-009: 2
NFR-REL-010: 2  NFR-CMP-006: 9
```

So counting `*Status*` lines gives 180, and the extras live inside
amendment blocks. **Which one governs is a design decision this round
must make and write down** — the first after the entry heading is the
obvious answer, and then an amendment that *changes* a status has to
say so in a way the rule can see, or the rule reads a stale one.
**Say what you chose and what it costs.**

## §3 — the vocabulary has already drifted

Leading tokens of all 180 status lines, measured:

| Value | Count |
|---|---|
| Implemented | 116 |
| Met | 19 |
| Met at | 12 |
| Partial | 4 |
| Specified | 2 |
| Not implemented | 2 |
| Not defined | 2 |
| **Partially implemented** | **1** |
| Deferred, Audited, Expired at, and six longer `Implemented …` forms | 1 each |

**Two findings in that table:**

1. **`Partial` (4) and `Partially implemented` (1) are the same
   status spelled two ways**, and Appendix A has a `Partial` column.
   One of the five is already outside the vocabulary the table
   assumes.
2. **`Met` (19 + 12 `Met at`) has no column in Appendix A at all.**
   The columns are Total, Implemented, Partial, Specified, Deferred,
   Divergent. So either `Met` is a synonym for `Implemented` and the
   table counts it there, or the table has been counting something
   the vocabulary does not name. **Settle it and write the mapping
   down**; it is the thing §4's second check depends on.

Statuses are written as prose — *"Implemented via snapshot tables"*,
*"Met at 0.44.0"* — so the rule closes the **leading token**, not the
whole string. A closed set of leading tokens with a documented
status→column mapping is the deliverable.

## §3a — one item carried in from `SPEC-001`

**24 register numbers exist in two forms at once.** Measured:
`requirements.md` has 45 `### 10.N` heading entries and 28 rows in an
older compact register, and 24 numbers appear in both. For each of
those 24, a status can change in one place and not the other —
`§10.34`'s class, two homes for one fact.

It is pre-existing and it is not a citation-rule concern, so
`SPEC-001` left it here. **Say whether this round's vocabulary work
can cheaply check that a row and its heading agree on status.** If it
can, do it. If it cannot, record the 24 as a known unchecked
population, in the rule's own doc comment, with a pointer to whoever
retires the duplication.

**Do not restructure the register.** The series README rules that
out and it is not lifted.

## §3b — a second item carried in: one wrong number

`SPEC-001` round 2's promoted `§10.20` says *"one of **three**
defects found in three days by a headless-browser inspection."* The
document says **four**, twice, and Appendix C's 0.32.0 row enumerates
them: `§10.18`, `§10.20`, `§10.19`, and the 314 px empty-anchor
overlap (`§10.21`).

`§10.21` was later closed as *"not a defect, my own measurement
artefact"*, so three survived as defects — which is probably the
reasoning. **Write what reconciles the two**: *"one of four reported,
one later withdrawn as a measurement artefact (`§10.21`)"*, or simply
four. One clause either way.

**Record it in `SPEC-003`'s own evidence while you are here.** A bare
number in prose, contradicting the same document's other statement of
itself, with no marker and no date for a rule to key on, is exactly
the case `SPEC-003` argues cannot be checked mechanically — found in
this series, one round after the handoff predicted the shape.

## §4 — two checks, and the second is the valuable one

**(a) The table's own arithmetic** — each total row equals the sum of
its own rows, per column. Cheap, and it is the check `§10`'s first
row describes: two compensating cell errors that left the total
looking right.

**(b) The table against the entries** — each area's `Total` equals
the number of entries with that prefix, and each status column equals
the number of entries whose governing status maps to it.

**(b) cannot be written before §3 is settled**, which is the real
sequencing in this round: vocabulary, then mapping, then the check.
If (b) turns out to disagree with the table somewhere, **that is a
finding about the specification, not a bug in the rule** — bring it
rather than adjusting the table to make the rule pass.

**Both get planted.** A compensating-cell error and a wrong per-area
count are different plants; do both.

## §5 — gates

- Rules (a), (b) and the vocabulary rule in `spec_scan`, each with a
  **planted test that fails without it**.
- The rule **states the population it examined** — entry count,
  status count — so a pattern matching fewer entries fails loudly.
  Trap 1 is the whole reason.
- Green on the real tree, which means §3's two findings are resolved.
- `DEC-007` delta stated.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`,
  CI.

## §6 — escalate rather than deciding

- **The `Met`/`Implemented` mapping is a specification question.**
  Propose it; nineteen entries use `Met` and the table has no column
  for it, so whichever answer is chosen is a statement about what
  those nineteen mean.
- **If check (b) disagrees with Appendix A anywhere.** Stop and
  report the disagreement with both numbers. Do not edit the table to
  match.
- **If an amendment's status cannot be bound to its entry
  unambiguously** — `NFR-CMP-006`'s nine are the hard case. Bring the
  shape rather than guessing at it.

## §7 — exit condition

The vocabulary closed to a documented set of leading tokens with a
status→column mapping written down; `Partial`/`Partially
implemented` resolved to one spelling; the `Met` question proposed to
the architect and answered before check (b) is relied on; checks (a)
and (b) in `spec_scan`, each planted and seen to fail; the rule
asserting the population it examined, with the ID pattern derived
from the twenty-five area segments in use rather than assumed;
the governing-status decision written down with its cost; all three
rules green on the real tree; `DEC-007` reconciled; all gates re-run.
