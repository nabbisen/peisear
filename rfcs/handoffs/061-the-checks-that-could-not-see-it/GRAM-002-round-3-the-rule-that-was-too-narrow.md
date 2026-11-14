# GRAM-002 round 3 — the rule that was too narrow

**Series**: `061`, `0.63.0`'s authorized scope. **Register**: `§10.59`.
**Follows**: `GRAM-002` round 2 (`60cb2e4`), **accepted**.

**Round 2 is not being corrected.** It built the guard the row names, it
fixed all seventeen, it refused to widen its own regex past what it was
asked for, and it named both of the gaps that refusal left. Every one of
those is the right call.

**The defect is in the rule I wrote.** `§3` of round 2's handoff said *a
number **immediately** followed by the noun it counts*. Round 2
implemented that sentence exactly, and "immediately" is what lets
`"1 consecutive snapshots"` through. This round widens the rule and
closes what the widening cannot reach by a different route.

Both gaps were confirmed by plant on the merged tree, not taken from the
report: reverting `OverloadStreakAriaLabel` to `"1 consecutive
snapshots"` and `ReadFirstLongStaleBody` to its hand-written `if`
(`"1 issue haven't been touched"`) each leaves
`no_rendered_message_disagrees_with_the_number_one` **green**.
`OverloadStreakAriaLabel` is one of the four defects that render today,
so one of this release's own repairs is currently revertible in silence.

## 1. Widen the adjacency, with a closed stop set

The number and its noun may be separated by adjectives — *"1 consecutive
snapshots"*, *"1 total events"*, *"1 recent snapshots"*. They may not be
separated by a word that ends the noun phrase.

**Amended 2026-11-14 — the window is two words, not three. See `§7`.**
The paragraph below is left as it was written so the amendment can be
read against it.

**Scan forward from a bare `1` for up to three words, stopping at:**

- any token containing a digit (`"1 vs 14"`, `"1 / 12 …"`, `"… is 1.
  Pushing"`),
- any of the closed set **`of`**, **`or`**, **`/`** — the three that
  turn a count into a set description (`"1 of last 14"`,
  `"1 or more issues"`, `"1 / 12 of in-flight issues"`).

Flag the first plural-shaped word reached. The existing grammar-word
skip set (`is`, `as`, `vs`, `was`, `has`) keeps its present job: skipped,
not a stop — `"1 was over capacity"` must keep scanning and find
nothing.

**Measure it, do not assume it.** Run the widened guard over the real
748 renderings **before** changing anything else and report every hit
verbatim. Round 2's own §Gates discipline: the pre-change run is the
measurement. If it flags a string that is correct English, that is a
finding about this rule — report it and stop, exactly as round 2
reported the gap rather than patching around it.

`guard_does_not_reject_grammar_words_or_correctly_uninflected_shapes`
keeps every case it has and gains the three stop-set shapes at the new
window, so the negative space is pinned at the width that now matters.

## 2. Close the other gap at the mechanism, not the string

A verb or pronoun that disagrees sits too far from the number for any
adjacency rule, and round 2 is right that widening the window to reach
it would be a different kind of check. **So check the cause instead.**

`ReadFirstLongStaleBody`'s defect existed because an arm hand-rolled its
own inflection and got only the noun. Assert that cannot come back: a
source-level test over `en.rs` that **no `if … == 1` / `else` chooses
between two string literals outside `counted()`'s own body**, with
`ConfirmDeleteIssueCascadeNote`'s `referent` named as the single
exception and its reason quoted from the existing comment.

That is a smaller claim than "no sentence disagrees", and it is honest
about being smaller: it does not prove agreement, it proves the one
mechanism that produced an unguardable disagreement is gone. Put it
where it can see the source — a test in `peisear-i18n` reading
`en.rs` by path, in the shape `peisear-web`'s `prose_scan` already uses
for source reading.

## 3. What stays exactly as it is

Round 2's structure, naming and reasoning. The `all()`-walking guard,
its failure format, both permanent tests, the 748 pin, every one of the
seventeen `counted()` call sites, the three exclusions
(`IndicatorExplanationLongStale`'s ratio,
`IndicatorCalculationActivity`'s *"or more"*,
`CognitiveSwitchingSignalMessage`'s float), `ConfirmDeleteIssueCascadeNote`'s
`referent`, and the `NFR-LANG-005` correction. **No new samples are
expected**; if the widened guard needs one, that is a finding to report
with the reason.

One stale name to fix while the file is open: the module doc cites
`guard_does_not_reject_grammar_words_ending_in_s`, and the test is
called `guard_does_not_reject_grammar_words_or_correctly_uninflected_shapes`.

## 4. Acceptance

1. **The widened guard's pre-change run over all 748, verbatim**, before
   any other edit — the hits, or none, read back from the runner.
2. **Both architect plants fail by name**, each with the rendered string
   in the message:
   - `OverloadStreakAriaLabel` → `"Overload streak: {n} consecutive snapshots over capacity ({})."`
   - and, for `§2`'s source test, `ReadFirstLongStaleBody` → its
     `ccbf67c` hand-written `if`.
   Plant by file copy, restore by file copy, `diff`-confirm, never
   `git checkout`.
3. The negative-space test still green at the new window, with the three
   stop-set shapes added.
4. `DEC-007` re-counted from `test.yml`'s own block; fmt and clippy
   (`--workspace --all-targets -D warnings`, and the `postgres`
   variant) exit 0; **`cargo test --workspace` once** — round 2 was
   right that `.github/CONTRIBUTING.md` retired the three-run
   recommendation on 2026-10-15, and round 2's own §5.6 asking for three
   was my stale boilerplate, not a rule.
5. `MessageKey::all()`'s population re-measured with the runner **only
   if it changed**; if it did not, say so and leave the pin alone.
6. One commit, pushed to `main`, CI watched to completion.

**Every number in the report read back from the runner that printed
it.** Round 2 caught its own `DEC-007` undercount that way and then
wrote three other figures from recall — *"fourteen distinct keys"* where
its own printed list has fifteen, *"sixteen of seventeen"* in the same
paragraph as two named absences, and *"9 → 12"* for a command that
yields 24. The conclusions were right every time; paste the figures.

## 5. Prohibited

No `regex` dependency — round 2's hand-written word split stays. No
widening of the stop set beyond `of`, `or`, `/` and digit-bearing
tokens; if a fourth is needed, that is a finding, not an edit. No
allowlist of strings anywhere. No new `MessageKey`, no re-wording
beyond agreement, no change to `en.rs`'s seventeen unless the widened
guard finds one, in which case report it first. `requirements.md` stays
untouched — `§10.59` records this round.

**This is the last grammar round in `061`.** `OCCL-001` is what remains
after it.

## 6. Required review-request format

Workflow §9.2. Lead with the widened guard's pre-change run.


## 7. Ruling on the blocking finding — the window, not the stop set

**The finding is correct and it is accepted.** The three hits were
reproduced here on the uncommitted file, and all three are correct
English: the three-word lookahead reads past the noun phrase the `1`
governs — across a full stop into *"This"*, across a comma into
*"completed points"*, across a semicolon into *"progress"*. Stopping at
the first acceptance step and putting the question in a file was the
right call, and `§1` asked for exactly this.

**The resolution is a narrower window: `WINDOW = 2`.** Not a fourth
stop condition, not an allowlist — `§5` stands unchanged, and this
takes machinery out rather than putting it in.

**Measured before being specified**, on the round's own uncommitted
implementation with nothing else changed and the file restored
byte-identical afterwards:

| window | `no_rendered_message_disagrees_with_the_number_one` over the real 748 | `"1 consecutive snapshots"` |
|---|---|---|
| 3 | **FAILED**, the three hits above | caught |
| **2** | **ok** | **caught** |

**The principle, and state it as the measurement it is**: the noun a
number counts is the head of its own noun phrase, and in this
interface's English that head is at most **one** modifier away. Every
counted phrase in `en.rs` is `1 <noun>` or `1 <adjective> <noun>` —
`"1 issue"`, `"1 day"`, `"1 consecutive snapshot"`, `"1 total event"`,
`"1 recent snapshot"`. A third word is already past the noun, which is
why it only ever found other clauses. Say so in the report as something
you re-derived, not as something this ruling asserts.

**Two things not to do**, both of which would make the three hits
disappear and both of which are the failure this series is about:

- **Do not add `progress` (or any word) to the grammar-word skip set.**
  It is a mass noun, the skip set is five grammar words, and growing it
  to fit a string that showed up is the allowlist `§5` forbids wearing
  a different hat.
- **Do not add punctuation stops.** At `WINDOW = 2` they are
  unnecessary, and an unnecessary stop is a rule nobody can test the
  need for.

**Keep the three strings as negative space.** Add all three *rendered*
sentences — `NotificationBurnoutOverloadBody`'s, `BarChartAriaLabel`'s
and `ReadFirstWipBody`'s, verbatim from the run above — to
`guard_does_not_reject_grammar_words_or_correctly_uninflected_shapes`.
They are test inputs that must not be flagged, not exemptions inside
the guard, and they pin at the new width the exact discovery this round
made. That is the round's own finding becoming part of the check.

**Then the rest of `§4` as written**, plus one correction to it: `§4.1`
has already been satisfied — the pre-change run above is the
measurement, it does not need repeating. Report the `WINDOW = 2` run
the same way, verbatim, and carry on into `§2` and `§3`.
