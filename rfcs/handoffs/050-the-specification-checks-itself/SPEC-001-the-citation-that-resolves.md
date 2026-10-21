# SPEC-001 — the citation that resolves

**0.57.0, round 1 of 3.** The two rules `ROADMAP.md`'s own row names,
and the first mechanical reader `requirements.md` has ever had.

`§10.42`'s own argument: this is the project's most-edited document and
nothing checks it. One of the two rules below is **already red, sixteen
times**.

---

## §1 — rule 1 is red, measured 2026-10-22 at `7c52515`

`### 10.N` headings that exist in `requirements.md` — **45**:
`10.0`–`10.19`, `10.21`, `10.25`–`10.48`.

**Four numbers are cited and have no heading:**

| Citation | in `docs/` | in `rfcs/` |
|---|---|---|
| `§10.20` | 5 | — |
| `§10.22` | 1 | — |
| `§10.23` | 1 | 1 |
| `§10.24` | 6 | 2 |
| **total** | **13** | **3** |

The thirteen in `docs/` are all in `requirements.md` and
`external-design.md` — **both live, normative documents.**

**And none of the four headings ever existed.** Checked against
history, not assumed: `git log -S'### 10.N '` over
`requirements.md` returns nothing for all four, and a broader search
for any heading form returns nothing either. So this is **not**
`§10.41`'s shape (a heading deleted and its body absorbed). These are
citations to register numbers that were never written.

## §2 — the fix is not mechanical, and must not be made so

Each of the four needs the citing sentence **read**, and there are
three honest outcomes:

1. **A typo for a neighbour** — the sentence means an entry that
   exists. Re-point the citation.
2. **A real defect that was described and never entered** — the
   sentence names something true. Then either write the register
   entry it refers to, or rewrite the sentence to stop citing.
3. **The sentence should not cite at all** — the reference is
   decoration. Remove it.

**Do not resolve this by minting register entries to match the
citations.** A guard made green by writing fiction into the register
is worse than the dangling citation, and `§10.N` is a record of
defects this project actually had.

**Report which outcome each of the four got, and why.** Four
judgements, four sentences.

## §3 — the `rfcs/` three: scope, with the reason

The rule covers **maintained documents** — `docs/` excluding
`docs/specification/history/` — and **not** `rfcs/`.

A closed handoff is a record of what was believed when it was
written. `[0.55.0]`'s own changelog section is left as shipped
history for the same reason, and `0.53.0` set that precedent. Editing
a closed handoff to satisfy a guard would be rewriting a record.

**But report the three.** A scope decision that hides its own
excluded population is `§10.36`'s shape. The rule's own doc comment
states the exclusion and its reason; the round's report states the
count.

## §4 — rule 2 is green today, which is the problem

Four-digit `RFC 0NNN` references: **43 occurrences**, `RFC 0001`
through `RFC 0005`, **every one inside
`docs/specification/history/`.** Measured repo-wide across `*.md` and
`*.rs`.

So rule 2 passes the day it is written. **A check never seen to fail
is not evidence** — plant it: put a four-digit reference into a live
document through the scan's own `Input`, watch the rule name it,
remove it. The same discipline `CON-006`'s predicate was held to.

## §5 — the shape, which already exists next door

Follow `docs_scan`, do not invent a second idiom:

- `crates/peisear-web/src/spec_scan.rs` with a sibling
  `spec_scan/tests.rs`, the layout all fifteen existing `*_scan`
  modules use.
- `pub(crate) struct Input<'a>` and `pub(crate) fn violations(input:
  &Input) -> Vec<String>` — `docs_scan.rs:431` and `:452`. **A pure
  function over an `Input` is what makes a rule plantable without
  touching a real file**, which is `§12`'s own lesson from `FIV-001`:
  a rule you can only exercise by editing the real document is a rule
  you cannot exercise at all.
- Plus one test that runs against the real tree via `repo_root()`
  (`docs_scan.rs:97`).
- Reuse `scan_support` where it fits. **I did not enumerate what it
  offers** — read it rather than duplicating it.

## §6 — gates

- Both rules in `spec_scan`, each with a planted test that **fails
  without the rule** and a real-tree test that passes.
- **Rule 1 green on the real tree**, which means §2's four judgements
  are made and `requirements.md`/`external-design.md` are clean.
- `DEC-007` moves by exactly the number of tests added; state the
  delta and the new total.
- `docs_scan` and `changelog_scan` still green — a new scan beside
  them must not change what they see.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  `publish-dry-run`, CI.

## §7 — escalate rather than deciding

- **If one of the four citations means a defect worth entering in the
  register.** Writing a new `§10.N` entry is a specification change
  and comes to the architect, with the sentence that prompted it.
- **If the `rfcs/` exclusion looks wrong** once you have read the
  three. Bring it; §3 is a judgement, not a measurement.
- **If a fifth dangling number appears** that this handoff did not
  name. That means my census was bounded where I thought it was
  total — report the full list.

## §8 — exit condition

`spec_scan` carrying both citation rules, each planted and seen to
fail; the four dangling numbers resolved by reading their citing
sentences, with the outcome and reason stated per number and no
register entry invented to satisfy a guard; the `rfcs/` exclusion
documented in the rule itself with its three citations counted in the
report; rule 2 green and **proven** rather than merely green;
`DEC-007` reconciled with its delta stated; all gates re-run after
the last edit.
