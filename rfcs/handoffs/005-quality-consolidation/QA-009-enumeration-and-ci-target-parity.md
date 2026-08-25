# QA-009 — What the guards are enumerated over

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: **P0** for §2 — a P0 requirement's guard has a live hole.
P1 for §3, P2 for §4.
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §3, §14
**Depends on**: nothing. `QA-008` is closed.

---

## 1. The theme, and why §2 outranks everything else in this series

Every guard `peisear-i18n` owns iterates `MessageKey::all()`. Every guard
`peisear-web` owns iterates a file list or a text block. **None of them has
ever had its enumeration checked.** A guard is only as wide as the set it walks,
and in one case that set is provably too narrow **right now**.

## 2. `MessageKey::all()` is missing five variants today

Not a hypothetical. Measured:

| | |
|---|---|
| Variants declared in `enum MessageKey` | **520** (395 unit, 125 with fields) |
| Distinct variants reachable from `all()` (by `std::mem::discriminant`) | **515** |
| Missing | **5** |

The five:

```
EmailForKindAriaLabel
InAppForKindAriaLabel
MinSeverityForKindAriaLabel
NotificationKindPreferencesAriaLabel
WebhookForKindAriaLabel
```

All five are live, all rendered by
`components/notification_preferences.rs`, and all five are **`aria-label`
copy** — text that only a screen-reader user ever hears. `find_violations` has
never seen any of them. That is `NFR-LANG-001`, **P0**, with a five-string hole
in it, and the strings it misses are the ones no reviewer would catch by eye.

**Their current wording looks fine** ("Email for …", "Minimum severity for …").
That is not the point and should not be reported as reassurance: they were
unchecked, and the check is what this project offers in place of hoping.

### 2.1 First, confirm and close

Reproduce the count before building anything — the two numbers, and the five
names. Then add the five to `all()` and **report what `find_violations` says
about them once it can see them.** If any violates §1.7, stop and report
before touching the copy.

### 2.2 Then guarantee it cannot recur

The property: **every variant of `MessageKey` appears at least once in
`all()`.**

Not "`all()` equals the variant set" — 125 variants carry fields and need
representative values, which is exactly what `all()` supplies, and 646 entries
covering 515 variants is correct and intended. At least once, per variant.

Three shapes. **A fourth is what I would rather have**, as in `QA-005` §3:

- **(a) Scan `message.rs` as source text.** Extract variant names between
  `pub enum MessageKey {` and its close; assert each appears in `all()`'s body.
  This is exactly how the five above were found, and it is the same family as
  `prose_scan`, `static_js_scan` and `dec_007_scan` — no dependency, no macro.
  Its limits are that family's known shape: a name inside a comment in `all()`
  would satisfy it, which is `QA-004`'s hole again and is handled the same way.
- **(b) A compiler-forced exhaustive `match`.** A helper matching every variant
  with no wildcard; the compiler refuses to build when a variant is added.
  Strongest against the "someone forgot" case at the moment of adding — but the
  arm and `all()` are still two separate hand-edits, so it moves the gap rather
  than closing it unless the match's output is what the test compares against.
- **(c) Declare the enum through a macro** that emits the variants and `all()`
  together. Closes the class outright, and makes a 1,700-line enum that people
  read and annotate heavily into macro input. I think the cost is too high; say
  so if you disagree.

I lean to **(a)**, and not strongly. Say which you chose and what the others
would have caught that yours does not.

### 2.3 Then ask the same question of the other four

`prose_scan`, `static_js_scan`, `test_harness_scan` and the language guard each
walk a set too — a glob over `crates/**`, over `static/*.js`, over
`crates/*/tests/**.rs`. **Do those sets have the same defect?** A file the glob
does not reach is a file the guard does not guard.

Report what you find. Fix only what is trivially the same shape as §2, and say
which you judged trivial. If one of them has a real hole, that is a finding of
its own and I want to see it before it moves.

## 3. The twenty named targets are not pinned to their CI jobs

Deleting `test-peisear-web-smoke` from `.github/workflows/test.yml` leaves both
`dec_007` guards green. A whole integration suite stops running in CI and
nothing says so.

**The ambiguity was mine** — `QA-008` §4 said "every `cargo test -p …` line has
a corresponding `run:` line", the `for` loop is one line, and treating it as one
shape was a fair reading.

What makes it closable: **the block names all twenty targets literally**, in the
`for t in …` list, across continuation lines. No YAML parsing and no shell
expansion — read from `for t in ` to `; do`, drop the `\` continuations, split
on whitespace. Then require a `run:` line naming each target.

The consequence is larger than the gap `QA-005` closed: one lost job there was
one target; here it is any of twenty.

Keep the existing one-directional rule — CI legitimately runs `fmt`, `clippy`,
`build`, `msrv`, which the block never mentions.

**Plant two cases**, one at a time: delete one per-target job, and delete the
`for` loop's line from the block.

## 4. Job-level YAML semantics — lower, and bounded

`if: false` and `continue-on-error: true` both leave an uncommented `run:` line
in place. `QA-008` named this and stopped there deliberately, correctly.

Ranked below §2 and §3 because a job disabled in place is a rarer accident than
a job deleted, and because interpreting it is where this family stops being a
text scan.

**Attempt it only if §2 and §3 are done and it stays a text scan.** A `run:`
line under a job whose block contains `if: false` is findable by indentation
without a YAML parser — if that turns out to need real parsing, report it and
stop. Do not add a dependency for this.

## 5. Escalate rather than deciding

- **If §2's counts do not reproduce, stop and report.** Everything here rests
  on them.
- If any of the five violates §1.7 once visible to `find_violations`, stop
  before changing copy.
- If §2.3 finds a real hole in one of the other four guards' file sets, stop
  and report it rather than folding a second finding into this handoff.
- If §4 needs a YAML parser to be useful at all, report and stop — as in
  `QA-008` §6.

## 6. Acceptance

1. §2's counts reproduced; the five added to `all()`; `find_violations`'
   verdict on them reported.
2. An exhaustiveness guarantee in place, running in CI, demonstrated by
   removing a variant from `all()` and watching it fail — and by adding a new
   variant without an `all()` entry, which is the case that actually happens.
3. §2.3 answered for all four other guards.
4. §3's check present, both plants demonstrated.
5. §4 attempted or reported as out of reach, with the reason.
6. Every plant one at a time, each reverted, `git diff` clean between.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. §2.2's choice as prose. §2.3's answer per guard, including the
ones that turn out to be fine. Each plant transcript separately.
