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

**No `### 10.N` heading was ever written for them.** Checked against
history: `git log -S'### 10.N '` over `requirements.md` returns
nothing for all four, and a broader search for any heading form
returns nothing either. So this is **not** `§10.41`'s shape (a
heading deleted and its body absorbed).

**Corrected 2026-10-22, by round 1 — the entries themselves do
exist.** This section first concluded *"citations to register numbers
that were never written."* `requirements.md:4355`–`:4376` carries an
older compact register (`| §10.N description | status |`) and all
four are in it, dated, with their closing handoff named. The
architect searched for headings, found none, and then wrote a
conclusion about *entries* — an absence established by a search
narrower than the claim. Round 1 looked in the document and found it.

**Measured, so the shape is not in doubt**: 45 heading entries, 28
compact rows, **24 in both forms**, **4 table-only** (these four),
21 heading-only. This is a migration that is **24 of 28 complete**,
and these four are the stragglers.

## §1a — the fix, decided after round 1: promote the four

**Write the four `### 10.N` entries from the content already in their
table rows and their citing sentences. Rule 1 stays strict — a
heading or nothing.**

This is not §2's forbidden invention. That prohibition is against
minting an entry so a citation resolves; here the entry exists,
dated, naming its closing handoff, and what is missing is the form
the register has used since `§10.29`. Their 24 neighbours were
promoted the same way. **Promoting finishes a migration; it does not
manufacture a record.**

The alternative — teaching rule 1 that a compact row also defines a
number — was considered and rejected: it blesses two forms of
register entry permanently, and a reader following a citation to a
one-line row gets less than one following it to an entry.

**Budget: each table row plus what its own citing sentences say, and
nothing more.** If a promotion tempts you into reconstructing a
defect's history from the handoff it names, stop — §2's invention
clause still applies.

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

Four-digit `RFC 0NNN` references: **45 occurrences over tracked
files** at scoping time, `RFC 0001` through `RFC 0005`.

**Corrected 2026-10-22.** This first said *"43 occurrences, every one
inside `docs/specification/history/`."* Both halves were wrong: the
count was taken over the working tree rather than tracked files, and
**two of the occurrences were in this handoff's own `§4`**, where the
architect wrote *"`RFC 0001` through `RFC 0005`"* — a census-taking
document instantiating the pattern it counts. Round 1 hit the
identical problem from the other side, in its own `rfcs/` count, and
reported it.

**The population matters more than the number.** Scoped to
maintained `docs/` plus published doc comments, rule 2 is green. It
must **not** widen to all of `crates/**/*.rs`: `spec_scan/tests.rs`
legitimately carries five planted `RFC 0017` references, and a rule
that flags its own fixtures is worse than no rule.

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
