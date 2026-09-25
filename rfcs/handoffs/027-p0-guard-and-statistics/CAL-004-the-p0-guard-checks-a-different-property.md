# CAL-004 — `FR-CAL-007`'s mandated guard checks a different property

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none — the gap is in the check, not the behaviour.
**Related requirement**: **`FR-CAL-007`, P0**.
**Source**: `REQ-001`'s audit §2. **Depends on**: nothing.

---

## 1. The gap

`FR-CAL-007` is **P0**: *the calendar MUST NOT display occupancy rate,
week-over-week comparison, free-hours totals, or any derived efficiency
figure.* Its status says **RFC 0002 mandates a guard test asserting these
strings are absent.**

**The behaviour holds** — `REQ-001` checked, and no efficiency copy renders on
either calendar. **The guard does not check what the requirement says it
checks.** `no_efficiency_metric_and_crowding_chip_carries_no_quantity` asserts
no `%`, no `" of "`, no ratio: a real property, and a different one.

So the named concepts — *occupancy*, *week-over-week*, *free hours* — could
appear tomorrow in words carrying no quantity at all (*"your calendar is
lightly booked this week"*) and the P0 guard would stay green.

**This is `§10.16` and `§10.28`'s family**: an apparatus described as covering
more than it does. It is the third instance, and the first on a P0.

## 2. What to do

**Extend the guard to assert the requirement's own concepts**, keeping the
existing quantity assertions — they catch a different thing and both are
wanted.

- **The terms come from the requirement and `SPEC §16.6`**, not from
  invention: occupancy, week-over-week (and its ordinary spellings), free
  hours / free time. **`SPEC §16.6`'s own example is *"free time: 4 hours"***,
  which the requirement quotes as *"seemingly benign"* — that phrase is the
  test case the rationale hands you.
- **Where the guard reads matters more than what it greps.** State plainly
  **what surface it scans** — the rendered calendar pages, the message table,
  or both — and what that leaves uncovered. A guard over the message table
  cannot see copy composed at a call site; a guard over two rendered pages
  cannot see a third page added later.
- **Do not let the doc comment claim more than the code does.** `§10.28` is the
  entry about exactly this failure, and this handoff exists because of its
  sibling. Say what is checked, on what, and what is not.
- **Vocabulary, not layout**: this is `peisear-i18n`'s guard family or a scan
  module, not a browser check. Follow whichever the existing assertion lives in
  rather than starting a third home.

## 3. Verification

- **Each new term fails the guard when planted** — one at a time, by file copy,
  restored byte-identical, as `ORD-002` and `SPRINT-005` did. **A guard never
  seen to fail is not yet evidence**, and this one has been green for three
  releases while checking something else.
- **`"free time: 4 hours"` specifically**, since the rationale names it.
- **The existing quantity assertions still pass unmodified** — they are not
  being replaced.
- **No user-visible string changes.** Nothing in the product says these words
  today; if the guard fires on real copy, **stop and report it** — that is a
  live P0 violation and a different handoff.
- `DEC-007`: report the new count, last recorded **346**. A new test file means
  the `CONTRIBUTING.md` block **and** the `test.yml` job, both authorised.
- `fmt`, `clippy`, three consecutive workspace runs. The overflow gate is
  untouched.

## 4. Escalate rather than deciding

- **If the guard fires on copy that ships.** Stop; that is the P0 itself.
- **If the terms cannot be checked on the surface where they would appear** —
  say where the blind spot is rather than covering a surface that happens to be
  reachable.
- **If a term produces false positives** on legitimate copy. Report the
  sentence; I will decide whether the term or the copy moves.

## 5. Exit condition

The guard asserts the concepts `FR-CAL-007` names as well as the quantities it
already catches, each demonstrated by a plant; its doc says what it scans and
what it does not; `FR-CAL-007`'s status can be written as Met rather than as a
mandate outstanding.
