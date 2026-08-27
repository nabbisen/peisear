# TT-003 — make the touch-target rule unconstructible

**Governing RFC**: [012](../../accepted/012-touch-target-conformance.md),
step 3, `DEC-049` as amended
**Target release**: 0.31.0
**Depends on**: `TT-002`, both rounds — closed. **Read both reviews**
(`TT-002-review.md`, `TT-002-round2-review.md`); §4 and §5 below come from
them, not from `TT-002`'s own report.

## 1. What this is

`TT-002` made 139 controls conform. **Nothing stops the 140th from not
conforming**, and nothing notices if one of the 139 loses its `grow()` — the
five rendered tests cover four specific controls.

This handoff closes that, and closes three smaller things the previous two
rounds surfaced.

**This is the last item in RFC 012.** When it lands, `NFR-A11Y-007` is met and
guarded, and **Definition of Done item 5 — open since 0.19.1, the oldest
condition in that table — closes**. Write it accordingly.

## 2. The size guard

Extend `touch_target_scan`. Today it makes one class unconstructible
(`checkbox-xs`); it must now make the **rule** unconstructible:

> Every site in `src/components/` carrying a sizing class
> (`btn-sm`/`btn-xs`/`input-sm`/`input-xs`/`select-sm`/`select-xs`) reaches a
> 44 px target.

Since `TT-002`, that means: **the class string containing the sizing class is
the argument of a `grow(...)` call.** `min-h-11` no longer appears in component
source at all — it is composed at runtime — so a guard hunting the literal finds
nothing and passes vacuously. **Do not write that guard.**

**Read `TOUCH_TARGET` and `grow`'s own behaviour rather than hardcoding
either.** The guard lives in the same crate; it has no excuse for a copy. The
review of round 2 records that the 44 px value currently has one home in
production and seven in test code — do not make it eight.

### Three traps, each of which this project has already fallen into

1. **A prose mention must neither satisfy nor break it.** `QA-004` found a guard
   satisfied by a mention in a doc comment; `QA-005` found one satisfied by a
   commented-out line. The inverse is equally wrong: a doc comment that says
   *"`btn-sm` is 32 px"* must not fail this guard. **Scan class-value string
   literals, not lines, and not raw file text.**
2. **No exception list.** `TT-002` round 2 removed the last three literals
   precisely so this guard would need none. If you find yourself adding one,
   **stop and report** — `touch_target_scan`'s own doc comment already records
   why: a rule that fails on a correct tree gets weakened until it passes, and
   an exception list is how that weakening looks in practice.
3. **State the scope limit in the module's doc comment.** This guard keys off
   sizing classes, so it covers class-carrying controls only. Plain `<a>` links,
   breadcrumbs and whole-card links are **outside the counting method, not
   inside it and passing** — `TT-001` §5, now a named limit on `NFR-A11Y-007`.
   Someone will read a green result as *"every interactive element conforms."*
   The doc comment is where that gets corrected, because it is where they will
   be standing.

**Plant it**: add a control with a sizing class and no `grow(...)`, confirm it
fails; remove `grow(...)` from an existing one, confirm it fails. Separately.

## 3. The `DEC-007` filesystem→block scan — `§10.16` reopening

`dec_007_scan` and `dec_007_ci_scan` check **block → CI**. Nothing checks
**filesystem → block**.

Verified during round 1's review: deleting the `touch_target` lines from
`.github/CONTRIBUTING.md` leaves all three scans green. So **any new
`crates/*/tests/*.rs` gets no loop entry and no CI job, silently**, and runs
only under `cargo test --workspace` — which is precisely the exposure `§10.16`
was opened to describe and was believed closed at 0.27.0.

`TT-002` hit this and wired `touch_target` in by hand. The gap is that nothing
would have told them.

**Add a scan**: every `crates/*/tests/*.rs` appears in `CONTRIBUTING.md`'s
`DEC-007` block. Combined with the existing block→CI scan, that gives
filesystem→CI transitively — **say so in the doc comment**, because the
transitivity is the actual guarantee and it is not obvious from either scan
alone.

**Plant it**: add a throwaway test file, confirm the scan fails, delete it.

**Scope**: integration test files. Do **not** extend to `--lib` targets or
benches in this handoff; if you find that boundary is wrong, report it.

## 4. Collapse the test-side copies

`board_keyboard.rs` and `confirmation.rs` assert the literal `"min-h-11"` /
`"min-w-11"` — **six sites across two files**. Make them read
`peisear_web::components::TOUCH_TARGET`, as `touch_target.rs` already does.

This fails *safe* today, which is why it was not a round-3 correction. It is
still `grow()`'s premise undone one layer up, and this is the handoff that
collapses it.

## 5. Confirm there is no third unscoped assertion

`TT-002` gave 139 controls an identical class pair. **Every unscoped
`body.contains(...)` assertion about that pair on a shared page silently
widened** — it now passes on a neighbour's markup rather than its subject's.

Two were found, both by planting: the login page's password field masking the
email field, and the board's per-row `join` masking the card button. Both were
correct when written.

**Sweep `crates/peisear-web/tests/` for a third.** Any assertion that checks a
page body as a whole for a string that many controls now share is suspect —
whether or not it concerns touch targets, since this pattern is about
*assertions widening as the tree grows*, not about this feature.

**Report what you find, including "nothing."** A clean sweep is a result. What is
not acceptable is not looking, because two of two found so far were found only
because someone planted against them.

## 6. What must not change

- **No adjacency guard.** Clause (2) is structurally guaranteed for
  `Grow`-inside-a-positive-`gap` (`TT-001` §3.2, `DEC-049` clause 4), and the
  only `Expand` in the tree is the checkbox `<label>`, which participates in
  layout. There is no pattern to write one from and nothing for it to catch.
- **No control is resized.** `TT-002` did that. If this handoff changes a
  rendered pixel, something is wrong.
- **No new copy, no new `MessageKey`.**

## 7. Escalate rather than deciding

- **If the size guard cannot be written without an exception list** (§2 trap 2).
- **If scanning class-value literals turns out to need real Rust parsing** rather
  than the depth-counting/string work this project's other scans use.
  `JS-002` hit exactly this boundary and the answer was to name the limit rather
  than reach for a parser — the same answer may be right here, and a guard that
  covers most sites with its gap **stated** beats a parser dependency.
- **If §5's sweep finds something whose fix is not obvious.**

## 8. Exit condition

- `touch_target_scan` fails on a sizing-class site without `grow(...)`, with no
  exception list, and its doc comment states the plain-link scope limit.
- The `DEC-007` filesystem→block scan exists and is planted.
- Six test-side literals read `TOUCH_TARGET`.
- §5's sweep reported.
- `DEC-007` clean; three consecutive `cargo test --workspace` runs.

**And one statement for the release note**, which is mine to write but yours to
make true: **whether `NFR-A11Y-007` is now met in full, or met with a named
limit.** After `TT-001` §5 I believe it is the second, and the changelog will
say so plainly rather than claiming WCAG conformance the guard does not cover.
If your work here changes that assessment in either direction, **say so** — that
sentence is the one a reader will remember.

---

**Who holds what**: dev team — the guard and the three smaller items. **What's
blocked**: RFC 012 closes with this. **What's next**: review request; then the
`§10.15`/`§10.16` baseline updates and Definition of Done item 5.
