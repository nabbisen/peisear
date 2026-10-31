# CFG-003 — the rule, enforced

**A guard, so a fifth variable cannot be added in the silent shape.**
Until this round lands, `RFC 022` is prose — and `§10.51` stays open on
exactly that ground.

Blocked on `CFG-002`, because the rule reads the shape those rounds
leave behind.

## §0 — what this round is, and the one thing it must not become

**A rule in `docs_scan.rs`**, following that file's shape: a pure
function over an `Input`, so it is **plantable**.

**It must not become a list of the thirteen variables.** A list is a
fourteenth place that can disagree with the code, and this project has
spent two releases on documents disagreeing with each other.
`§10.36`'s own decided shape is the precedent: *"every `§10.N`-shaped
citation must have a matching heading"* was chosen over a numbering
check **because a predicate does not drift.**

## §1 — the predicate, which is the hard part of this round

The property `RFC 022` §3 states is: **every `env::var` read of an
operator-facing variable either treats a parse failure as an error, or
has no parse at all.** The shape that violates it is recognisable —
`.ok().and_then(…parse…).ok()` then a fallback, or `.map(…).unwrap_or(…)`
over a present value, or `try_…().unwrap_or_else(…)`.

**Propose the predicate before building it, and say what it cannot
see.** I am not specifying its exact form, deliberately: a text-reading
guard over Rust expressions is the kind of rule that can be
over-fitted to today's four call sites, and an over-fitted rule is
worse than none because it reports green forever. **If you conclude a
reliable predicate is not achievable at this cost, say so and propose
what is** — for example, asserting that every `env::var` site in
`crates/*/src` appears in a known census *and* that the census's own
size is asserted, which is weaker but honest, and is the shape
`spec_scan` rule (b) already uses.

**That escalation is a legitimate outcome of this round, not a
failure.** `RFC 022` §7 already says a stated rule and an enforced rule
are different things.

## §2 — seen red both ways, before it is trusted

**`DEC-048`'s lesson, and this series' own entry is about guards that
cannot fail.**

1. **Plant a violation**: reintroduce one of the silent shapes
   `CFG-001`/`CFG-002` removed, run the rule, capture the failure
   text, revert, run again green.
2. **Break the rule's own input lookup** — rename or move what it
   anchors on — and confirm it reports **the lookup failing** rather
   than zero violations. **`§10.36` instance 6 happened in this very
   file**, and `CFG-002` will have just re-tested whether that
   entry's recorded remedy is actually in force. Whatever `CFG-002`
   found, this round's new rule asserts its own lookup from the start.
3. **Report both plants with captured output.** A guard claimed to
   work, without the red, is not evidence.

## §3 — the population, asserted rather than assumed

Assert **how many `env::var` sites the rule examined**, so that a
refactor which hides them from the rule shows up as a changed count
rather than as silence. This single assertion is what separates
*nothing wrong* from *nothing examined*, and it is the cheapest line in
the round.

## §4 — `§10.51` closes here, or it does not close

`§10.51`'s own text says the repair is *"a refusal, a warning, or a
documented acceptance"* and that the choice is a design call. `RFC 022`
made it: refusal. **This round is what lets the entry close on a
stated condition rather than on a feeling** — so the round's report
must say which of `§10.51`'s named instances are now impossible and
which are merely fixed-for-now.

**If `§1`'s predicate escalates to the weaker census shape**, say what
`§10.51` can and cannot claim on that basis. The entry closing is the
architect's call; the evidence for it is this round's.

## §5 — gates

- **This round edits a gate's source, so run that gate**, with the
  number.
- `DEC-007` moves. Count with the runner; before and after.
- `fmt`; `clippy --workspace --all-targets -D warnings`; rustdoc with
  `.github/CONTRIBUTING.md`'s documented flag — the new rule has a doc
  comment; `mdbook build docs` with the link check.
- CI on the branch: **quote the run's own conclusion and its
  non-success count**; do not sum the jobs.

## §6 — escalate rather than deciding

- **If a reliable predicate is not achievable** (`§1`) — the expected
  and acceptable escalation.
- If the rule would be red on code `CFG-001` or `CFG-002` produced.
  That means one of the three rounds is wrong, and which is a design
  question.
- If asserting the population requires an anchor fragile enough that
  `§10.36` instance 6 would repeat.

## §7 — exit conditions

1. A rule exists in `docs_scan.rs` as a pure function over an `Input`,
   with its own test — **or** `§1`'s escalation is reported with a
   concrete weaker proposal.
2. It is a predicate over the code, not a list of variable names.
3. **Seen red both ways** (`§2`), each with captured output, each
   reverted, green after.
4. Its doc comment states its bounds, including what shapes it cannot
   recognise.
5. The population it examined is asserted, not assumed.
6. `§4` answered: what `§10.51` can now claim.
7. `§5`'s gates after the last edit, `DEC-007`'s delta with the
   runner, CI's own conclusion quoted.
