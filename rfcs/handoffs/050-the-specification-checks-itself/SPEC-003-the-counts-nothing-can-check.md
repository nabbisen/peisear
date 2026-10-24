# SPEC-003 — the counts nothing can check

**0.57.0, round 3 of 3.** Blocked on `SPEC-001` for the module.

`ROADMAP.md`'s row names a fourth property: *"counts stated in prose
checked against the workspace."* **As written it is not mechanically
decidable**, and this round's first job is to say so precisely rather
than build something that looks like it.

---

## §1 — why the rule as named cannot be written

`requirements.md` states numbers of two kinds, and **in prose they
are indistinguishable:**

- **A dated measurement**, correct forever: *"295 `?N` placeholder
  occurrences (measured 2026-10-18)"*. Re-checking it against today's
  workspace would make a true sentence fail.
- **A present-tense claim about the workspace**, which must match
  today: *"`171` call sites"*, *"`23` files"*.

A scan cannot tell which is which. **A rule that checks both breaks
the historical record; a rule that checks neither is the status
quo.** Either way the naive form is wrong, which is why this is a
round with a design question in it rather than three more rules.

## §2 — the two shapes that *are* checkable, and the one that is not

**Shape A — a predicate, not a count.** `NFR-MNT-005`'s status reads
*"live count zero, measured: every workspace `.rs` file under
`crates/*/src/` carrying a `#[test]` function either declares
`#[cfg(test)] mod tests;` … or is itself wholly tests."* **That is a
property, and a property is checkable at any time.** It has no date
on it because it needs none.

**Shape B — an opt-in annotation.** A present-tense count becomes
checkable if it says what it counts, in a form a rule can read. The
convention is the design question: an HTML comment beside the claim,
a trailing marker, something else. **Propose one; do not adopt one on
this handoff's word.**

**Shape C — a bare number in prose.** Not checkable, and the honest
answer is that it stays unchecked. **Say so in the rule's own doc
comment**, so the next reader knows the gap is known rather than
missed — the distinction `§10.36` is entirely about.

**Found in this very series, one round after this handoff was
written — `SPEC-002` §3b.** `SPEC-001` round 2's promoted `§10.20`
entry wrote *"one of **three** defects found in three days"*; the
same document's own Appendix C 0.32.0 row says **four**, and names
them. No marker on either number, no date on either number, and
nothing mechanical could have told which one was current — the
exact shape this section describes, arriving on schedule rather than
as a hypothetical. Fixed in `requirements.md` directly (`SPEC-002`);
recorded here as the evidence this section's own argument needed.

## §3 — what the architect recommends, to be argued with

1. **Prefer turning a count into a predicate** wherever the sentence
   allows it. *"Live count zero"* is stronger than *"23 files"*
   because it cannot go stale. Where a count exists only to assert
   "none" or "all", rewrite it.
2. **Adopt one annotation** for the counts that genuinely must be
   numbers, and keep the set small. A convention used three times is
   a convention; used thirty times it is a second language inside the
   document.
3. **Leave the rest unchecked and documented as unchecked.** This is
   the recommendation most likely to feel like giving up, and it is
   the one I am most confident of: a rule that silently checks a
   subset while claiming to check prose counts is exactly the defect
   `0.57.0` exists to close, reintroduced one layer up.

**None of the three is decided.** §5.

## §4 — gates

- Whatever is built is **planted and seen to fail**, per rule.
- **Shape C is written down**: the rule's doc comment, or
  `requirements.md` itself, states which numeric claims are checked
  and which are not. A reader must be able to tell without reading
  the implementation.
- If an annotation is adopted, **every use of it checked**, and the
  rule fails when an annotated claim stops matching — planted.
- If counts get rewritten as predicates, the entries changed are
  listed, and no entry's *meaning* changes in the process. **A
  rewrite that weakens a requirement to make it checkable is the
  failure mode here** — bring any case where the predicate would say
  less than the count did.
- `DEC-007` delta stated; `docs_scan`/`changelog_scan` green; `fmt`,
  `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`, CI.

## §5 — escalate rather than deciding

- **The annotation convention, if one is adopted**, is a change to
  how the specification is written. Architect's, and the owner's if
  it reaches the documents' own conventions.
- **Any count that resists all three shapes.** Report it; a named
  exception with a reason is an acceptable outcome of this round and
  an unexamined one is not.
- **If §3's third recommendation is what you land on for most
  claims** — that most prose counts stay unchecked — say so plainly
  and completely. That is a legitimate result, and `0.57.0` is still
  worth shipping without this rule: `SPEC-001` and `SPEC-002` are the
  two that close live defects.

## §6 — exit condition

The three shapes distinguished in writing; `NFR-MNT-005`-style
predicates identified and, where a count can become one without
weakening the requirement, rewritten with the list of entries
changed; an annotation convention either proposed and authorized or
explicitly declined; **what is checked and what is not stated where a
reader will find it**; everything built planted and seen to fail;
`DEC-007` reconciled; all gates re-run after the last edit.
