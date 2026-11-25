# `STAT-001` — The rule that lived only in the checker

Series [067 — the gate that never ran](./README.md), **round 3 of 3, and the
last.** `GATE-001` and `IMG-001` are both **accepted**.

**This round exists because the round it replaces did not.** `ABS-001` was
scoped around *"eight of the 171 statuses assert an absence."* Measured
against **governing** statuses that population is **one**. The architect's
census had parsed the **first** `*Status*:` line of each entry; this
document's rule is that the **last** one governs. `IMG-001` found the same
error one entry at a time. **Read §1 before anything else: the premise you
are given is the thing this round is about.**

---

## 1. What is actually there, measured

```bash
# entries with more than one *Status*: line, and the governing value of each
# (governing = body.rfind("*Status*:"), which is spec_scan's own rule)
```

| entry | `*Status*:` lines | governing value |
|---|---|---|
| `FR-NAV-003` — Breadcrumb and back affordance | 2 | re-measure |
| `NFR-REL-009` — Released artefacts | 2 | `Implemented` (0.53.0) |
| `NFR-REL-010` — Published container image | 2 | `Implemented` (0.53.0) |
| `NFR-CMP-007` — Persistent state on the Postgres backend | **7** | re-measure |

**Four of 171 entries, and only one of the four marks its earlier status as
superseded near the line itself.** Re-derive this table yourself; it is the
architect's, and his last two tables of this kind were both wrong.

**`FR-HLT-005` is the one real absence claim** of the eight he listed:
governing status **`Implemented, unverified`** (*"was `Implemented` with a
citation that did not hold"*). The other seven were superseded openings the
document keeps on purpose.

**And the rule itself is stated nowhere.** `spec_scan` implements it in
Rust; the document never tells a reader that the last `*Status*:` wins. A
reader going top-down meets the superseded value first. **Verify that
claim** — search the register and its own front matter for any statement of
the rule before accepting it, because if it *is* stated somewhere, this
round's first item collapses to "the architect did not read it" and should
say so plainly instead.

---

## 2. What this round owes

### 2.1 Make the document state its own rule

**If and only if §1's last claim holds.** One sentence, where a reader of an
entry will meet it — the register's own reading preamble, beside whatever
already explains `*Status*`, `*Source*` and `*Priority*`. It must say which
line governs and why earlier ones are kept.

**Then mark the earlier statuses.** Three of the four entries leave their
superseded line looking current. The marking must not break
`governing_status_token`: it keys on the literal `*Status*:`, so a prefix
like *"superseded"* **outside** that marker is safe and a change **to** the
marker is not. **Prove that with a run, not a reading** — change one, run
`spec_scan`, and if `governing_status_token` stops resolving, that is the
answer and the mark goes elsewhere.

### 2.2 A guard, only if it earns itself

**Candidate**: a `spec_scan` test asserting that in any entry with more than
one `*Status*:` line, every line but the last is marked superseded.
Population four. **Decide whether it is worth existing** and say why either
way — *less is more* is the house rule, and a guard over four hand-written
lines may be ceremony. If you build it, **plant it**: an unmarked earlier
status must fail by name.

**What a guard cannot do** is stop a reader parsing the first field; only
the document saying so can. Be honest about which half of the defect each
measure closes.

### 2.3 `FR-HLT-005`

Governing status `Implemented, unverified`. **Either verify it or state what
verifying it would take** — a citation that holds, naming the surface and
what was read. If verification is out of this round's reach, say so and
leave the status alone rather than softening its wording; *unverified* is an
honest status and this series' whole subject is not writing claims you have
not measured.

### 2.4 Close the series

Rounds table reconciled, §1.2/§1.3's corrections carried into the closing
summary, and the series' own ledger stated plainly: **one of its three
rounds was real as scoped, one found its own premise false, and one exists
because a third premise was false.** Then the **`Amended` / `Covers
release`** header fields move to `0.69.0` — `IMG-001` correctly left them
for the close, and that is here.

### 2.5 The register entry

Open one for the class: **a document whose reading rule lived only in its
checker.** It is `§10.36`'s family — *cannot tell nothing wrong from nothing
examined* — but distinct, and worth its own number: here the checker was
**right and green**, and a human read the same file by a different rule and
believed the opposite. Record all three instances with their causes:
`NFR-REL-010`'s status (first line vs last), the absence census (eight vs
one), and the tag count (**`jq '.tags[-8:]'`, a slice the architect wrote
and then read as the whole list**). The third is not a document defect at
all — name it as the architect's tooling, in the same entry, because the
pattern is one pattern: **reading the shape of your own output as the shape
of the world.**

---

## 3. Gates, and what not to do

- `DEC-007`'s block in full — **696** at `IMG-001`; state the new count and
  what moved it.
- `spec_scan` **after** the last edit, not before, and say so.
- The usual: real exit codes, scratch under `.git-exclude/tmp/`, plants in a
  worktree with `git status` clean either side, `CHANGELOG.md` under
  `[Unreleased]`, no release leg, nothing published carrying local
  information.
- **Do not** touch `NFR-REL-010`'s status (`IMG-001` settled it), the
  `image.yml` check, or any component.
- **Disagree where the tree disagrees.** Two of this series' three premises
  came from the architect and were false. If §1's table is wrong too, that
  is the round's finding and it outranks everything above it.
