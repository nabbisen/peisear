# GRD-003 — the needle that cannot see what it bans

**Target**: 0.52.0, after `GRD-002`. Last of the three. It edits doc
comments across twenty-one files, so it should land when nothing else is
moving them.

**Everything below was measured at `a68b6d9` on 2026-10-14.** Re-measure
before you start; `GRD-001` and `GRD-002` both touched files in this
population.

---

## §1 — the rule, and what it actually enforces

`docs_scan` rule 5 (`DOC-002`, `RFC 019`) says: no document under
`docs/`, no RFC outside `rfcs/handoffs/`, and no published doc comment
under `crates/*/src/` cites the project's gitignored private area. Four
named exceptions. It is live and enforced, not reported.

**It matches the literal path.** A citation that spells the path is
caught. A citation written as the bare file name — the form this project
actually uses — spells no path at all, and the rule has never seen one.

Today the rule reports **zero**. Inside the very surfaces it scans sit
**64 citations of files that exist only in the private area**, naming
**29 distinct private files**. The rule is clean and its subject is not:
`§10.36`, in a guard `RFC 019` asked for.

**`DOC-002` closed 19 citations of the path spelling** — fifteen
resolved to a rewrite or a cut, four kept as the exceptions. Those
nineteen and these sixty-four were in the same files at the same time.
The round counted the spelling and the rule inherited the same boundary,
so the larger population was never in view. **That is the lesson this
round exists to record**: a rule named for a subject and keyed on a
string enforces the string.

## §2 — the sixty-four

| Surface | occurrences | distinct private files | files |
|---|---|---|---|
| Published doc comments (`///`, `//!` under `crates/*/src/`) | **60** | 27 | 18 |
| Documents (`docs/`, `rfcs/` minus handoffs) | **4** | 4 | 3 |
| **Rule 5's scope, total** | **64** | **29** | **21** |

Per file, so you can tell when you are done:

| n | file |
|---|---|
| 24 | `crates/peisear-i18n/src/message.rs` |
| 5 | `crates/peisear-web/src/touch_target_scan.rs` |
| 5 | `crates/peisear-web/src/dec_007_scan.rs` |
| 4 | `crates/peisear-web/src/components.rs` |
| 3 | `crates/peisear-web/src/prose_scan.rs` |
| 3 | `crates/peisear-i18n/src/en.rs` |
| 2 | `docs/specification/requirements.md` |
| 2 | `crates/peisear-web/src/test_harness_scan.rs` |
| 2 | `crates/peisear-web/src/static_js_scan.rs` |
| 2 | `crates/peisear-web/src/dm_fallback_boundary_scan.rs` |
| 2 | `crates/peisear-web/src/components/sprint_plan.rs` |
| 1 | `rfcs/done/007-compliance-pass.md` |
| 1 | `rfcs/done/004d-direct-manipulation-calendar.md` |
| 1 | `crates/peisear-web/src/dec_007_ci_scan.rs` |
| 1 | `crates/peisear-web/src/components/tests.rs` |
| 1 | `crates/peisear-web/src/components/issues.rs` |
| 1 | `crates/peisear-web/src/components/calendar.rs` |
| 1 | `crates/peisear-web/src/components/breadcrumb.rs` |
| 1 | `crates/peisear-storage/src/projects.rs` |
| 1 | `crates/peisear-i18n/src/locale.rs` |
| 1 | `crates/peisear-core/src/lib.rs` |

**The two in `docs/specification/requirements.md` are the worst of the
sixty-four** and should be read first: the normative document cites, by
name, a file no reader of this repository can open.

**Out of scope — do not touch.** A further **16 occurrences across 11
files** sit in `//` implementation comments and under `crates/*/tests/`.
`DOC-002 §5` drew that line deliberately: a `//` comment never reaches a
published crate's documentation. Sweeping them in would triple the diff
and widen the rule by accident. Report the count; leave them.

## §3 — rule 5 scans more than rustdoc renders

Of the **60** doc-comment occurrences, **36** are on surfaces rustdoc
actually publishes. The other **24** are not: twenty in the seven scan
modules `lib.rs` declares `#[cfg(test)]`, one in `components/tests.rs`,
three in `peisear-i18n`'s private `mod en`. Rustdoc renders none of
them.

**This is not a hole and it must not be fixed.** Rule 5's population —
every `///`/`//!` line under `crates/*/src/` — is a deliberate
over-approximation of what docs.rs reads, and an over-approximation on a
*ban* costs nothing. Parsing `#[cfg(test)]` to narrow it would add
attribute handling to a rule whose whole design is a filesystem walk, to
permit something nobody wants to write.

What is wrong is that the module's doc comment calls this population
"what rustdoc, and so docs.rs, actually reads" without saying it is
wider. **State the over-approximation in one line where the rule is
described.** Same shape, same remedy, as `NFR-CONC-003` at 0.51.0 and
`NFR-A11Y-007` at `a68b6d9`: a guard whose scope differs from its stated
subject says so in writing, and this is the third time in two releases.

## §4 — the needle

Three constraints. The first is not negotiable.

**1. The needle must not read the private directory.** It is gitignored.
In any tree that lacks it — a published crate, a `cargo package`
verification build, the extracted worktrees this project's own release
gate uses, CI — a needle built from its listing is empty, and the rule
reports clean because it examined nothing. That is `§10.36` exactly, and
it is the failure `REL-0.49.0` already found once in this same rule
(`§10.x` row 1: `git -C` answering against an enclosing repository). The
needle is a **shape**, matched against the text, with no filesystem
dependency.

**2. It must catch both spellings.** 47 of the 60 doc-comment
occurrences write the name with `.md`; 13 do not — `` `X-review.md` §3 ``
and `` `X-review` §3 `` are both house style. A rule that catches only
the first teaches authors to write the second.

**3. It must not need an exception list.** The module's own comment is
explicit: *"a rule whose exception list keeps growing is a rule whose
subject is narrower than its name."*

**Measured, a shape that satisfies all three**: a token ending `-review`
or `-review.md` whose prefix is an identifier shape — an uppercase ID
followed by `-` and a digit (`TT-003-`, `I18N-005c-`, `QA-001-`,
`REL-0.51.0-`). Against the live tree that matches 57 of the 60
doc-comment occurrences and 3 of the 4 document occurrences, with **zero
false positives and no exceptions**. Without the identifier-prefix
requirement the same suffix matches two ordinary prose words in
`rfcs/done/000-rfc-lifecycle-policy.md` and would need its own carve-out
— which is constraint 3 failing on the first try.

**The residual, which must be written down rather than discovered
later.** The private area's own naming is not uniform: of the files
there today, most end `-review`, and the rest end `-decision`,
`-decisions`, `-recommendation` or nothing at all. One of those
non-conforming names accounts for **4 of the 64** (`ISSUE-006-decision`,
already named in `rfcs/done/007-compliance-pass.md`). Widen the suffix
set if you can do it without an exception list; if you cannot, **say in
the rule which shapes it does not catch**. Do not leave the number
implicit, and do not claim coverage the measurement does not support.

**This bounds the round's own measurement too.** The 64 above were
counted against the private directory's actual listing, which only the
owner's tree has. **Measure your own progress with the needle, not with
that listing** — and expect the needle's number to be lower by the
residual, not equal to 64. Reporting "zero" from a needle whose residual
you did not state is the defect this round is about, committed by the
round itself.

## §5 — clearing the sixty-four

**The ruling, which is not open**: a bad citation is fixed by extracting
the substance, never by publishing the private file. Nothing moves out
of the private area into the repository in this round.

Per citation, in this order:

1. **The sentence already carries the substance.** Cut the pointer. Most
   of the sixty-four read like *"`X-review` §3's correction. The original
   shape passed …"* — the correction is stated; the citation only names
   who said it, and names it unreachably.
2. **It does not.** State the substance in a clause, then cut the
   pointer. This is the expensive case and the valuable one.
3. **Re-point to a published home only when that document actually says
   it.** 26 of the 29 cited IDs have a handoff under `rfcs/handoffs/`;
   `DEV-001-004`, `DEV-002-005` and `ISSUE-006` do not (7 of the 64).
   **A review's §3 is not its handoff's §3** — re-pointing a section
   number without reading the target is a worse defect than the one
   being fixed, because it reads as resolvable and is wrong. Check, or
   drop the section number.

**`message.rs` holds 24 of the 64.** Expect it to be most of the round.

## §6 — order

Clear first, widen second. Rule 5's own module doc warns about the
opposite — *"hitting a rule against a tree a later handoff is what
fixes"* — and names `DOCS-005` landing two half-rules to avoid it. **The
suite is green at every commit.** If you widen first you land 60-odd
failures and then edit prose under a red suite, which is how a citation
gets cut rather than extracted.

## §7 — specification work (`DEC-028`)

- **`§10.x`**: a new entry. The class is not "rule 5 was too narrow" —
  it is **a rule named for a subject and keyed on a string, which
  enforces the string**, and the evidence is that `DOC-002` fixed 19 of
  one spelling while 64 of another sat in the same files. `§10.36` is
  the nearest neighbour and should be cross-referenced, not reused: that
  entry is about a check that cannot tell "nothing wrong" from "nothing
  examined"; this one examined, and answered about the wrong thing.
- **`§11`'s 0.51.0 row** already carries this as carried work. Its
  figures were corrected after the release shipped; **re-read it rather
  than trusting the sentence you remember**.
- **The over-approximation of §3**, stated once where the rule is
  described.
- **`§11`'s 0.52.0 row** closes the carry with the measured final count
  and the residual from §4.

## §8 — gates

- `DEC-007`: block extracted **fresh** from `.github/CONTRIBUTING.md`.
  State `GRD-002`'s post-round baseline (**446**) and the delta. New
  tests for the widened needle are expected; prose edits are not. A
  change you cannot account for is a stop.
- `fmt --check`, `clippy --workspace --all-targets -D warnings`,
  `cargo doc` with broken-intra-doc-links denied. **The private-link
  warning count will move** — cutting a citation can orphan an intra-doc
  link. State the before and after; 0 broken is the gate, the private
  count is a number to report.
- `mdbook build` with linkcheck, for `requirements.md`.
- No migration. Browser gates not expected to move — say you did not run
  them and why.

## §9 — escalation

- **A citation whose substance you cannot recover** from the sentence,
  the code, or a published handoff. Do not guess it and do not read it
  out of the private area into the repository. **Leave the citation,
  list it, and stop on that one** — it is the architect's to resolve.
- **The needle's residual cannot be closed without an exception list.**
  Report both options with their counts rather than choosing; an
  exception list is a design change to a rule whose own comment forbids
  growing one.
- **The widened rule fires on something that is not a private file.**
  That is a finding about the needle, not a licence to add an exception.
  Report the token and where.

## §10 — exit condition

The needle widened, shape-based, reading nothing under the gitignored
path; its residual stated in the rule in writing, with the number; rule
5's population described as the over-approximation it is; the citations
in its scope cleared by extraction or cut, with the final count
**measured by the needle** and the method named; the 16 out-of-scope
occurrences reported and untouched; `DEC-007` accounted for against 446;
`§10.x` carrying the new class and `§11`'s two rows reconciled; **and a
before/after showing the widened rule failing on a planted citation and
passing on the cleared tree** — a needle never seen to fire is not
evidence that it can.
