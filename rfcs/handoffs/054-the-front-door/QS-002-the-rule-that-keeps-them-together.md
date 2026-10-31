# QS-002 — the rule that keeps them together

**A `docs_scan` rule: every install command in `README.md`'s Quick
Start names a path `docs/getting-started/install.md` offers.**

`QS-001` fixes the drift. This round is why it cannot happen a fourth
time — and *fourth* is measured, not rhetorical: `install.md` and the
README have disagreed about install paths since `0.53.0`, through
`FIV-004`, `FIV-005`, `FIV-006` and `WIZ-003`, each of which touched
one of the two documents.

**Blocked on `QS-001`**, because the rule reads what that round
writes.

## §0 — what this round is

**Code.** A rule in `crates/peisear-web/src/docs_scan.rs` plus its
own test, following that file's existing shape: a pure function over
an `Input`, so the rule is **plantable** — which is the point, and
`§1` is about nothing else.

## §1 — seen red before it is trusted

**`DEC-048`'s own lesson, and this project's most expensive repeated
mistake when it is skipped**: a guard that has never failed is a guard
nobody has tested.

- **Plant a violation and watch the rule fail**: add a command to the
  README's Quick Start naming a path `install.md` does not offer, run
  the rule, capture the failure text, revert, run again green.
- **Plant the other direction too, and this is the half that gets
  skipped**: make the rule's own *input lookup* fail — rename or move
  what it anchors on — and confirm it reports **the lookup failing**
  rather than reporting zero violations. `§10.36` instance 6 is
  exactly this defect in `docs_scan` rule 4: a literal moved, the
  `if let` did not match, and the rule passed having examined
  nothing. **Rule 4 now asserts its own lookup succeeded; this rule
  must do the same from the start.**
- **Report both plants with their captured output.** A claim that a
  rule works, without the red, is not evidence.

## §2 — what the rule must and must not be

**It is a predicate, not a list.** `§10.36`'s own decided shape:
*"every `§10.N`-shaped citation must have a matching heading"* was
chosen over a numbering check because a predicate does not drift.
Apply the same here — **do not hardcode the three install paths**, or
the rule becomes a fourth place that can disagree with the other two.
Derive the offered paths from `install.md` itself.

**State its own bound in the doc comment**, as the other rules do.
Two bounds I can already name; find any others and name them too:

1. It checks that a command in the README **corresponds to a path
   `install.md` offers**. It cannot check that the command *works* —
   `NFR-REL-011`'s CI walks do that, for two of the three paths.
2. It cannot check **order**, which is what `QS-001` is actually
   about. A README that listed the toolchain-free path last would pass
   this rule. **Say so**; a rule that silently covers less than the
   clause it serves is `§10.36`'s own class, and `NFR-REL-007`'s new
   clause has two halves of which this is one.

**If order turns out to be cheaply checkable, report that rather than
building it** — it changes what the clause can promise, which is a
specification question.

## §3 — the population, asserted rather than assumed

`docs_scan`'s rules each walk their own population and `spec_scan`
rule (b) asserts the size of its census. **Do the same**: assert how
many commands the rule examined in the Quick Start, so that a README
restructure which hides the section from the rule shows up as a
changed count rather than as silence. This is the single assertion
that separates *nothing wrong* from *nothing examined*, and it is
cheap here.

## §4 — gates, and the one this round is itself

- **This round edits a gate's source, so run that gate** — not the
  other call sites, not a reasoning-about-it. `cargo test -p
  peisear-web --lib`, with the number stated.
- **`DEC-007` moves this round** — the rule's tests are new. Count
  with the runner, give the before and after, and name which crate
  the delta is in.
- `fmt`; `clippy --workspace --all-targets -D warnings`; rustdoc with
  `.github/CONTRIBUTING.md`'s own documented flag — the new rule has
  a doc comment and rustdoc reads it.
- `mdbook build docs` plus the link check.
- **CI on the branch**, and quote the run's own conclusion and its
  non-success count rather than summing jobs.

## §5 — escalate rather than deciding

- If deriving the offered paths from `install.md` needs an anchor
  fragile enough that `§10.36` instance 6 would repeat — **say so and
  propose the anchor you would use**, rather than hardcoding the list
  to get the round finished.
- If the rule as specified would be red on the README `QS-001`
  produced. That means one of the two rounds is wrong, and which one
  is a design question.

## §6 — exit conditions

1. The rule exists in `docs_scan.rs`, a pure function over an
   `Input`, with its own test.
2. **Seen red both ways** — a planted README violation, and a broken
   input lookup reported as a broken lookup rather than as zero
   violations — each with captured output, each reverted, green after.
3. The rule derives `install.md`'s offered paths rather than
   hardcoding them.
4. Its doc comment states its bounds, including that it does not check
   order.
5. The population it examined is asserted, not assumed.
6. `§4`'s gates run after the last edit, `DEC-007`'s delta stated with
   the runner, CI's own conclusion quoted.
