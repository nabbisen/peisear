# GRD-001 — the helper five scans share

**Target**: 0.52.0, first round. **Carried from `DOC-005`'s review**,
deferred past `0.51.0` because it is guard infrastructure and that
release had already absorbed three non-documentation rounds.

**Do this before `GRD-002`.** `GRD-002` splits five of the same files;
fixing the helper first means editing each once.

---

## §1 — the defect

`strip_line_comments` is defined **six** times:

| File | Line | Shape |
|---|---|---|
| `crates/peisear-web/src/prose_scan.rs` | 158 | naive |
| `crates/peisear-web/src/dm_fallback_boundary_scan.rs` | 127 | naive |
| `crates/peisear-web/src/static_js_scan.rs` | 91 | naive |
| `crates/peisear-web/src/touch_target_scan.rs` | 284 | naive |
| `crates/peisear-web/src/test_harness_scan.rs` | 48 | naive |
| `crates/peisear-web/src/alignment_scan.rs` | 81 | **string-aware — correct** |

The naive five are the same five lines:

```rust
source.lines().map(|line| line.split("//").next().unwrap_or("")).collect::<Vec<_>>().join("\n")
```

**`//` inside a string literal is not a comment.** Five component files
carry `xmlns="http://www.w3.org/2000/svg"` — `components/teams.rs`,
`components/projects.rs`, `components/sprints.rs`, `components/layout.rs`,
`components/notifications.rs`. In each, every naive scan truncates the
line at the `//` inside the URL and loses the rest of it.

**Why it is worse than a lost line.** `touch_target_scan` asserts that
every interactive element declares a touch target. With quote pairing
corrupted from the first `xmlns` onward, its answer for those five files
is not *wrong* — it is **unrelated to the question**, and it reports
clean either way. That is `§10.36` in a guard `NFR-A11Y-007` depends on,
and `NFR-A11Y-007`'s own status rests on it.

**`alignment_scan.rs:81` is already the fix.** Written in `0.51.0`,
independently, correct: it tracks string state and backslash escapes and
cuts only at a `//` outside a string. So this round is **five copies
adopting one that already works**, not writing one.

## §2 — `QA-002`'s duplication call, reversed where it governs

`QA-002` chose duplication for this helper deliberately — *"small enough
that sharing costs more than it saves."* That call is recorded in
`rfcs/handoffs/004b-direct-manipulation-board/BOARD-001-board-copy-and-parity.md:82`
and cited as precedent in two source comments
(`tests/status_control.rs:75`, `src/contrast_scan.rs:63`). **It is in
neither specification** — the exact shape `RFC 0019 §3` ruled on at
`0.51.0`.

**Reverse it for this helper, and state the test that distinguishes the
cases**, in `requirements.md` under `NFR-MNT-004` or its nearest
maintainability neighbour — wherever the existing text governs helper
duplication; if no requirement governs it, say so rather than inventing
a home, and escalate.

The distinction to write down, as plainly as it reads:

> **A straight-line helper may be duplicated. A parser may not.**
> `collect_rs_files` and `extract_hidden_field` read one thing and
> return it; a second copy is either right or obviously wrong.
> `strip_line_comments` carries state across a line and has edge cases,
> so six copies are six chances to be wrong *differently* — and were:
> five shared one bug and the sixth, written later and independently,
> did not have it. **Duplication's real cost is not the lines. It is
> that a defect found in one copy does not reach the others, and no
> single copy tells you how many siblings are wrong.**

`QA-002`'s reasoning was sound for the cost it priced and omitted this
one. Say that; do not write it up as an error.

**Leave the convention standing for the straight-line helpers.**
`collect_rs_files` and `extract_hidden_field` stay duplicated. Update
the two comments that cite `strip_line_comments` as their precedent
(`tests/status_control.rs:75`, `src/contrast_scan.rs:63`) so they cite
the rule above instead of an example that no longer holds.

## §3 — the shape

**One `#[cfg(test)]` module, six call sites, zero copies.**

`crates/peisear-web/src/scan_support.rs`, declared in `lib.rs` as
`#[cfg(test)] mod scan_support;` beside the scans it serves, exporting
`pub(crate) fn strip_line_comments`. Lift `alignment_scan.rs:81`'s body
**verbatim** — it is the reviewed, correct one; do not rewrite it while
moving it.

Then the module's own tests, which nothing currently has:

- a line holding `xmlns="http://www.w3.org/2000/svg"` survives whole;
- a real trailing `// comment` is cut;
- a `//` inside a string **after** an escaped quote (`"a\"b // c"`) is
  still inside the string and not cut;
- an unterminated quote does not panic or swallow the next line.

**Name the file this helper's new home and nothing else's.** If a second
shared scan helper appears later, that is its own decision; a module
called `scan_support` that accretes is `§10.36`'s shape one level up.

## §4 — measure before and after, per scan

**This is the round's real exit condition, not the refactor.**

Before touching anything, run each of the six scans and **record what it
reports**: the number of files it examined, the number of elements or
lines it matched, and its pass/fail. Then fix, then run again and record
the same three. A table of six rows, before and after, in the review
request.

**A scan whose numbers do not move has not been shown to have been
broken.** Five of these read five files containing `xmlns`, so at least
`touch_target_scan`, `prose_scan`, `static_js_scan`,
`dm_fallback_boundary_scan` and `test_harness_scan` should each see
*more* content afterwards. If one of them does not move, that is a
finding about that scan — report it rather than explaining it away.

The scans do not currently report a matched count. **Adding a temporary
count for the measurement is correct and should not ship**; say in the
review request how you obtained each number.

## §5 — what the fixed guards find is not this round's work

If `touch_target_scan` now reads the five component files properly and
finds interactive elements with no declared touch target, **report the
count and the sites, and stop.** `NFR-A11Y-007` is a P1 requirement and
a real violation there is its own round with its own handoff — possibly
its own release. Repairing it inside a guard round would repeat exactly
what `0.51.0` had to apologise for in its own changelog.

The same for `prose_scan` (hardcoded prose), `static_js_scan`, and the
rest.

## §6 — gates

- `DEC-007`: extract the block **fresh** from `.github/CONTRIBUTING.md`,
  three consecutive runs. **442 is the baseline**; the helper's own new
  tests raise it, so state the arithmetic — how many tests you added and
  what the block reports — rather than only the total.
- `fmt --check`, `clippy --workspace --all-targets -D warnings`,
  `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc
  --workspace --no-deps --keep-going` (baseline: 8 private-link
  warnings, 0 broken).
- `mdbook build` with linkcheck, if any document changed.
- No migration expected.
- Browser gates are not expected to move — nothing rendered changes.
  **Say that you did not run them and why**, rather than running them to
  produce a clean line.

## §7 — escalation

- **No requirement governs helper duplication.** Then §2's rule has no
  home. Report it; do not invent a requirement, and do not leave the
  rule only in a source comment — that is the defect `RFC 0019` just
  closed nineteen instances of.
- **A sixth scan turns out to need a different helper** — e.g. one that
  must keep comments. Then the shared module is the wrong shape and the
  round should stop and say so, rather than growing a parameter.
- **A scan's before/after numbers are identical across the board.** Then
  either the measurement or the premise is wrong, and the round has not
  established what it claims. Stop and report both numbers.

## §8 — exit condition

Six copies gone and one `pub(crate)` helper in `scan_support.rs`, lifted
verbatim from `alignment_scan.rs:81`, with its own four tests; a six-row
before/after table showing what each scan examined and reported, with
the method for each number stated; `QA-002`'s duplication call reversed
for this helper and the straight-line/parser rule written where a
requirement governs it, with the two citing comments updated; anything
the repaired guards now find **reported and not fixed**; `DEC-007`
re-measured with its arithmetic shown.
