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

---

## §7 — the architect's answer to `§5`, 2026-11-03

**`§5`'s second bullet fired exactly as written, and escalating was
right.** The rule is red on the README `QS-001` produced, three lines.
The answer is that **neither round is wrong — the README's heading
structure is**, and the decision below was planted on the real tree
before it was issued: with both halves applied,
`the_repository_satisfies_the_live_rules` goes **green** with no change
to the rule's logic.

### The decision

1. **The source block moves out of `## Quick Start` under its own `##`
   heading.** The README's own closing sentence already says `install.md`
   *"covers all three paths in full"* — **the prose says three and the
   heading span contains four.** So this is not a change to what the
   README promises; it is the structure catching up with the sentence
   above it, and `QS-001`'s ordering is untouched — the block stays where
   it is, in the same order, under a heading of its own.

   **Use `##`, not `###`.** `quick_start_section`'s terminator is
   `"\n## "`, so a `###` subheading does not end the section: the block
   would stay in the population and the fix would look like it had
   worked.

2. **`install.md` adopts the README's glob** —
   `peisear-*-x86_64-unknown-linux-gnu.tar.gz`, not
   `peisear-<version>-…`. The placeholder is the thing that is wrong:
   pasted literally it fails, while the glob pastes and runs, and a
   front-door command should be copy-pasteable. The README's form was
   authorized at `QS-001`.

3. **The census becomes `5`** (two fewer command lines in the section).
   That it has to change is the census working, not a defect.

**Option (a) is declined for its rule half and adopted for its document
half, as `QS-003`.** Following *every* link out of `install.md` pulls in
`deployment.md`, `scaling.md`, `configuration.md` and `first-run.md` with
their fences — a much larger haystack where a Quick Start command can
match in an unrelated page. Following *one named* link is option (c)
wearing a different hat, and the round was right to rule (c) out. The
predicate stays byte-exact against `install.md` alone: the simplest rule
that does the job.

### What this round still owes

**Three of the four new tests are written against the live documents, so
this fix invalidates all three.** Not a defect in the rule — it needs
nothing beyond the census const — but *"done but for a decision"* should
not be read as nearly done:

- **Plant 1 anchors on a neighbouring heading.** It inserts `brew install
  peisear` before `"## Features"`; once the section's boundary moves the
  plant lands outside the Quick Start and the test reports `got []`.
  **Build the fixture inside the section it tests**, not against whatever
  currently follows it.
- **The census plant hardcodes `examined 6`** alongside the const it also
  reads; under the decision the planted count is 4.
- **`rule_9_the_real_readme_has_exactly_these_three_violations_today`
  does not survive the fix, and should not.** Its reasoning is good and
  its doc comment states it plainly, but **the aggregate gate already
  carries that signal** — a fix silently resolving one of the three leaves
  the gate red at two, and the census catches structural moves. Delete it
  rather than inverting it; two tests asserting one red is one edit too
  many. *(It was also not listed in the review request's `§0`, which named
  three new tests where there are four. The `§3` count of +4 was honest;
  the list was not, and the unlisted one was the substantive one.)*

### One exit condition to extend, and it is `§6.4`

**The doc comment states two bounds and owes a third.** It names that the
rule cannot check a command *works* and cannot check *order*. It does not
name this: the rule tests the Quick Start's commands against the bash
lines of **`install.md` as a whole**, not against the corresponding path's
own block — so a command matching a line in the PostgreSQL section would
pass. That is weaker than *"names a path `install.md` offers"*, which is
how both this handoff and the rule's own message word it. **One sentence**,
and `§10.36`'s entire class is a check that reads wider than it is.

### The residual, to be recorded in the round's own closing

Once the source block is outside `## Quick Start`, **rule 9 does not guard
its two commands.** `QS-003` closes the document half; after it, nothing
checks that the README's source block and `build-from-source.md` agree.
State it in the closing so the closing is not read as wider than it is.
