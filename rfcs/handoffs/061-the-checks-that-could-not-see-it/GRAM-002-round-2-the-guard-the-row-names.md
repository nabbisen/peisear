# GRAM-002 round 2 — the guard the row names

**Series**: `061`, `0.63.0`'s authorized scope. **Register**: `§10.59`.
**Follows**: `GRAM-002` (`ccbf67c`), accepted with this correction.

Round 1's mechanism is right and stays: `counted()` is private, prepends
the number, takes the plural at `0`, and is not a plural-rules
mechanism. Fourteen strings against the handoff's five, a duplicate code
path found by re-deriving from the enum, `CAL-002 §2.2` reversed with
the architect's ratification (`§10.59` now records it). **None of that
is reopened.**

This round is the other half, and the authorized row names it in these
words:

> a test that renders **every** count-bearing message at 1 and reads it

## 1. What round 1 built instead

`gram_002_counted_sentences_agree_with_their_own_number` and
`gram_002_burnout_sentences_agree_with_their_own_number` are lists of
`assert_eq!`, one per string the round had already decided to fix.

**That is the same shape as the three tests that could not see the
original defect.** `basis_route` and `health_explainability` ×2 each
quote the sentence they know about; a substring match cannot see a
grammar defect in the part it does not quote, and an `assert_eq!` list
cannot see a string that is not in the list. The round fixed its
instances and wrote a check that pins those instances — `MARK-001`'s
lesson, which this series was re-scoped on, one round later.

**Proof, and it is already running in CI.** `MessageKey::all()` has
carried `ReadFirstLongStaleBody { long_stale_count: 1 }` since before
this round. Every `page_title_scan` run renders it. It renders:

> **"1 issue haven't been touched in over two weeks. Closing or
> re-assigning them clears your queue."**

A verb that disagrees, a pronoun that disagrees, in `/today`'s
read-first callout, **rendered at 1 on every CI run of this
repository**, and nothing reads it. It is in neither of round 1's two
lists — not fixed, not reported — because it already had a hand-written
`if` for its *noun*, so a census looking for a number followed by a bare
plural passed straight over it.

## 2. Build the guard first, and watch it fail

**In this order. The guard is the deliverable; the fixes are what it
finds.**

1. Write the guard. Walk `MessageKey::all()`, render each entry, and
   assert no rendered string contains **the number 1 immediately
   followed by a plural-form word**. One regex over the rendered text:
   `\b1\s+([A-Za-z-]+s)\b`, with a closed-class skip set of exactly
   `is`, `as`, `vs`, `was`, `has` — grammar words, not an allowlist of
   strings. The number `1` only: `"1.0 issues"` is correct English for a
   float and the pattern must not match it.
2. Run it and record what it reports, verbatim, before changing one
   string. **That list is round 2's own population** — not §3 below,
   which is the architect's count and is there to be checked against
   yours.
3. Fix until green.

**`counted()` is the mechanism** for every one of them. Where agreement
reaches past the noun — a verb, a pronoun — put the whole predicate in
both arms, which is the shape `IndicatorExplanationWipCompliance`
already uses:

```rust
counted(
    long_stale_count,
    "issue hasn't been touched in over two weeks. Closing or re-assigning it clears your queue.",
    "issues haven't been touched in over two weeks. Closing or re-assigning them clears your queue.",
)
```

**`ReadFirstLongStaleBody`'s hand-written `if` goes.** It is the last
inflection branch in the file outside `counted()` itself; round 1
refactored `WipCompliance`'s away for exactly this reason and missed
this one. `ConfirmDeleteIssueCascadeNote`'s `referent` stays — it is a
variable clause with no number in it, documented, and round 1 was right
about it.

## 3. The seventeen, and the one rule that sorts them

Measured 2026-11-14 at source on `ccbf67c`, every string literal in
`en.rs`, mechanically. **Check this against your guard's own output and
report any difference** — one of us is wrong if they disagree, and the
guard is the one that will still be right in six months.

**The rule, which replaces round 1's own criterion:**

> **A number immediately followed by the noun it counts goes through
> `counted()`.** A unit abbreviation (`pt`, `d`, `%`) has no plural to
> get wrong. A noun after `of`, `or more`, or a ratio names a *set*, not
> the count, and agrees with the set.

Round 1's line — *a short standalone sentence versus a clause in a
longer explanation* — is a judgement about prose, and it sorted five
strings into the wrong bin. The rule above is a property of the string,
decidable by reading it, which is why the guard can decide it too.

| # | key | at 1 | reachable today |
|---|---|---|---|
| 1 | `ReadFirstLongStaleBody` | *"1 issue **haven't** been touched…"* | **yes — by design**: `compute_read_first` fires at `long_stale_count >= 1`, "deliberately permissive" in its own comment |
| 2 | `StalledAriaLabel` | *"Oldest stalled assigned issue: 1 **days** (steady)."* | **yes — ungated**, rendered from the raw signal whenever the panel renders |
| 3 | `OverloadStreakAriaLabel` | *"Overload streak: 1 consecutive **snapshots** over capacity (steady)."* | **yes — ungated**, same |
| 4 | `BurndownSummary` | *"Across the 1 **days** from … to …"* | **yes** — `burndown_card` needs only `!burndown.is_empty()`, so a sprint's first day renders it |
| 5 | `ReadFirstStalledBody` | *"…open for 1 **days**…"* | no — `STALLED_WATCH_DAYS = 14` |
| 6 | `ReadFirstWipBody` | *"You have 1 **issues** in progress…"* | no — `current_wip > effective_wip_limit`, and the limit is ≥ 1 (`parse_positive_int` rejects ≤ 0; `CHECK (wip_limit_default IS NULL OR wip_limit_default > 0)` in `0005_personal_limits.sql`; `DEFAULT_WIP_LIMIT = 3`) |
| 7 | `StalledAssignedSignalMessage` | *"…hasn't moved in 1 **days**."* | no — `STALLED_WATCH_DAYS = 14` |
| 8 | `OverloadStreakSignalMessage` | *"…of the last 1 **snapshots**."* | no — `OVERLOAD_STREAK_WATCH = 8` |
| 9 | `ReadFirstOverloadBody` | *"…of the last 1 **snapshots**."* | no — same gate |
| 10 | `SwitchingSampleLine` | *"1 **events** over 14 d"* | no — `SWITCHING_MIN_EVENTS = 5` |
| 11 | `SwitchingAriaLabel` | *"(1 total **events** over 14 **days**)"* | no — same gate. Both clauses |
| 12 | `DriftAriaLabel` | *"…over the last 1 **days**…"* | no — window is a constant |
| 13 | `ThroughputGlossaryDefinition` | *"…in the last 1 **days**."* | no — constant |
| 14 | `LongStaleGlossaryDefinition` | *"…in over 1 **days**."* | no — constant |
| 15 | `IndicatorCalculationStaleness` | *"Good under 1 **days**. Watch from 1 **days**, continuing from 7 **days**."* | no — constants (three sites) |
| 16 | `IndicatorCalculationActivity` | *"…in the last 1 **days**."* | no — constant. `"{good_count} or more issues"` is **correct** and stays |
| 17 | `IndicatorCalculationLongStale` | *"…untouched for 1 **days** or more."* | no — constant |

**Four are live. Thirteen are correct only because a constant somewhere
else is not 1** — a threshold, a schema `CHECK`, a default. That is the
exact shape this release exists to remove: a property that holds by
accident, with nothing that fails when the accident ends. Someone
lowering `STALLED_WATCH_DAYS` to 1 should not be able to introduce a
grammar defect, and after this round they cannot.

**Correctly left alone**, and the rule says why without a judgement
call: `OverloadStreakValue` (`"1 of last 14"` — no noun),
`StalledDaysValue` (`"1 d"` — unit), `PointsValue` (`"1 pt"` — unit,
and round 1 was right to name it), `IndicatorExplanationLongStale`
(`"1 / 12 of in-flight issues"` — the noun names the set),
`IndicatorCalculationActivity`'s `"1 or more issues"`,
`CognitiveSwitchingSignalMessage` (`"1.0 issues"` — a float has no
singular form), `ConfirmDeleteIssueCascadeNote`'s `referent`.

## 4. The architect's own error, stated because it changes what you read

The review this round answers named **two** strings as the mandatory
correction — `ReadFirstWipBody` and `ReadFirstStalledBody` — and argued
they were urgent because `FR-PER-006` makes the read-first callout the
one callout `/today` shows.

**Both are rows 5 and 6 above: neither can render its defect today.**
The prominence argument was sound about the slot and wrong about the
render, because it was reasoned from the string and never from the
branch that reaches it. The live defect in that callout is row 1, which
that review did not contain at all. The correction stands — they are
still fixed here, for the by-construction reason, not the live one —
but it is row 1 that makes this round mandatory, and the method that
found row 1 was the one this handoff asked round 1 for in §4 and did not
itself apply: re-derive the population, do not inherit a list.

## 5. Acceptance

1. **The guard, failing first.** Report its output on `ccbf67c`'s
   strings — the list it finds, verbatim, before any fix — then green.
   A guard first seen green has not been seen.
2. **A plant, after green**: revert any one of the seventeen to its
   `ccbf67c` text and show the guard failing **by name**, with the
   rendered string in the failure message.
3. All seventeen through `counted()`; `ReadFirstLongStaleBody`'s
   hand-written `if` deleted; `ConfirmDeleteIssueCascadeNote`'s
   `referent` untouched.
4. **The gap in the guard, stated rather than papered over.** It walks
   `MessageKey::all()`, so a count-bearing variant with no `1` sample in
   `all()` is invisible to it. Say in your report what you did about
   that and why; if the answer is "nothing, and here is why that is
   proportionate", that is an acceptable answer and a better one than a
   mechanism nobody asked for. Do not add an allowlist.
5. `MessageKey::all()`'s population **re-measured with the runner** —
   `page_title_scan`'s own assertion, read back from its output — and
   re-pinned in the same commit. **Do not predict it**: four rounds in
   this series have reported an intermediate figure that the runner
   contradicted, every conclusion right and every number recalled rather
   than read. Paste the figure from the run.
6. `DEC-007` re-counted from `test.yml`'s own block, fmt and clippy at
   0, three consecutive `cargo test --workspace` runs green.
7. One commit, pushed to `main`, CI watched to completion — *How work
   lands*, in this series' README.

## 6. Prohibited

No locale mechanism, no `fluent`, no plural-rules crate, no public API —
`counted()` stays private and English-only, and `RFC 006` is not
reopened. No allowlist, in the guard or beside it. **`NFR-LANG-005`** is
the requirement to stay inside — *full internationalisation is deferred
and the architecture should avoid decisions that would preclude it* — and
a private English-only helper does not preclude anything. Round 1's
handoff and report both cited **`FR-I18N`**, which is not an id this
repository has; that was mine first and is corrected here rather than
carried. No new `MessageKey`, no re-wording beyond agreement: a string
that reads correctly at 1 and 2 after the fix is finished, even if you
can see a better sentence. Do not touch `requirements.md` — `§10.59`
already records this round and its rule.

## 7. Required review-request format

Workflow §9.2. Lead with the guard's pre-fix output and whether it
matched §3's seventeen.
