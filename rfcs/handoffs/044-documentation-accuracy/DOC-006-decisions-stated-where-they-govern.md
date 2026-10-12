# DOC-006 — decisions stated where they govern

**The last content round of 0.51.0**, and the one that answers the question
this release started from: *why was the documentation not updated as it
should have been?*

**`RFC 0019 §3`, accepted.** **Target**: 0.51.0.

---

## §1 — the rule, and why it is not a register

**Owner, 2026-10-12**: *"It sounds strange to me if you asked me where to
collect the project decisions. Decisions should not be written isolated
from specification or requirement."*

> **A decision is stated once, in the document it governs, at a location
> the other mentions point to.**

**Not collected elsewhere** — a `decisions.md` would recreate the defect
one directory over. **And not only in passing**, which is the half
`DEC-049` proves is needed.

**The project's own worked examples:**

| | |
|---|---|
| `DEC-055` | its own heading in the document it governs — *"## Keeping the file a readable size — `DEC-055`"*. **Legible** |
| `DEC-056` | stated in place — *"A release is four steps: …"*. **Legible** |
| `DEC-013` | `requirements.md:2210`, *"Source: implementation decision `DEC-013`"*, **and nothing else anywhere**. **Not legible** |
| `DEC-049` | discussed **four** times in `requirements.md`, each about a different clause. **Not legible** — I reconstructed it from six citations |

**`DEC-055` costs one heading.** That is the whole shape.

## §2 — the population, measured

**34 decisions are cited from documentation itself** (`docs/`, and RFCs —
handoffs excluded, they are execution records). Citation counts run from
one file to eleven (`DEC-021`).

**That is the population. It is not necessarily one round's work**, and I
have mis-sized three counts in this release already. **Survey first,
report the split, and say if it needs two rounds** — I would rather read
that than receive a half-finished pass.

**The split to report:**

- **Already legible** — a document states it outright. `DEC-055`,
  `DEC-056`, and however many others turn out to be in that class.
- **Named only** — cited as a *Source* or in passing, stated nowhere.
  **This is the work.**
- **Scattered** — discussed in several places, no single statement.
  `DEC-049`'s shape; the fix is to state it once and let the others cite
  that place.

## §3 — what a statement is, and what it is not

**A few distilled lines where the decision takes effect.** Not the
review's prose, not the process that produced it, and not a history of
the alternatives unless one of them is why the decision reads as it does.

**It answers**: what was decided, and why that rather than the obvious
alternative. **`DEC-056`'s own entry is the length to aim for.**

**Where it goes** is the document that governs the subject — the
requirement for a requirement-level decision, `external-design.md` for a
presentation one, `docs/architecture/` for a structural one,
`docs/development/` for a process one. **If a decision has no governing
document, that is worth escalating**, not inventing a home for.

## §4 — two that are named specifically

**`DEC-013`** — `NFR-CONC-003`'s cited source. *The application MUST NOT
write `updated_at`; triggers maintain it, so the lock value has exactly
one authority.* Its payoff is on the record and nowhere stated: `QA-018`
found two authorities on three tables, `0017` closed them, and
`STORE-001`'s precision change was **seven trigger bodies in one
migration** rather than an audit of every write. **The requirement that
cites it is where it belongs.**

**`DEC-049`** — the 44px touch target, `Grow` as the default, and the
clause that matters: *"a `Grow` cluster inside a positive-`gap` container
is **presumed** to satisfy clause (2) with no further verification."*

**And it has just been amended, in a handoff.** `DOC-005 §3.1` ruled that
one helper cannot be correct for all three kinds and added `grow_text()`;
`DOC-005` then found six further sites the ruling mis-shaped. **That
amendment currently lives in an execution record**, which is exactly the
defect this round exists to close. **State `DEC-049` as amended, once,
where it governs** — and the presumption clause should be stated as
withdrawn, since the round that tested it found it false.

## §5 — the 24 with no committed trace

24 of the 59 assigned numbers appear in **no committed file at all**.

**Do nothing about them.** They are not cited, so nothing depends on them,
and listing them would be the register the owner declined. **Report the
count and move on** — a numbering gap is not a defect.

## §6 — gates

- **`DEC-007`** should not move; this is prose in existing documents.
  **If it moves, say why.**
- **`mdbook build`** with linkcheck — every document you touch is served.
- **Rule 5 stays green**: a decision stated in a document must not cite
  `.git-exclude/` for its reasoning. **If the only full account of a
  decision is in a review, that is the gap — write the decision, not the
  pointer.**
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken).
- **No migration.**

## §7 — escalate rather than deciding

- **If the survey says this is more than one round.** Report the split and
  stop; I will cut it.
- **If a decision has no governing document.**
- **If a decision's reasoning cannot be reconstructed** from what is
  committed. **Say so and leave it uncited rather than inventing one** —
  a plausible reconstruction presented as the record is worse than a gap.
- **If stating a decision contradicts the document that cites it.** That is
  a finding about the document, not about the decision.

## §8 — exit condition

The 34 surveyed and split three ways with counts; every *named only* and
*scattered* decision that documentation depends on stated once where it
governs, in `DEC-056`'s register and length; `DEC-013` stated in
`NFR-CONC-003`; `DEC-049` stated as amended with its presumption
withdrawn; the 24 reported and untouched; `mdbook build` clean and rule 5
still green.

**Then 0.51.0's content is complete** and the candidate is next.
