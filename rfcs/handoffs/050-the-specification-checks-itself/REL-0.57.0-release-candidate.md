# REL-0.57.0 — release candidate

**The series' last item.** Five rounds closed — `SPEC-001` (the
citation rules), `SPEC-002` (the table and the vocabulary),
`SPEC-003` (the counts nothing can check), `SPEC-005` (the counts a
step can check), `SPEC-004` (the table re-derived) — two of them
added after the series was scoped as three.

`§10.42`'s argument is answered: `requirements.md` now has a
mechanical reader.

---

## §0 — already done; reconcile, do not redo

Every rule, both prose fixes, the re-derived table, the re-pointed
0.43.0 question, and the seven CI steps. **Read them, confirm they
still say what they said, and move on.**

## §1 — the ordinary cut

1. **Version** — `0.57.0` in `[workspace.package]`, nowhere a second
   time.
2. **`CHANGELOG.md`** — a `[0.57.0]` section, dated. Confirm
   `[Unreleased]` is empty first, by reading it.
3. **`ROADMAP.md`** — `## What ships next` moves on; the shipped line
   is written; `0.58.0` (`RFC 020`) becomes next. **That section is
   the single home for release numbers.**
4. **Release cycle exit criteria 1–8**, each stated with what
   satisfied it. **There is no criterion 9.**

## §2 — the claim this release makes

**The claim**: the specification is now read by something that fails
when it stops being true, and the first reading found more wrong with
it than anyone expected.

**What it found, and the notes should say all of it:**

- **Thirteen dangling `§10.N` citations** in `requirements.md` and
  `external-design.md`, pointing at four numbers that had entries in
  an older compact register but no heading — a migration **24 of 28
  complete**, finished by `SPEC-001`.
- **Nine stale areas in Appendix A**, in a table whose own 0.43.0 note
  said fifteen rows *"were not re-derived and may carry the same
  drift"*. Calendar's row still read `Specified: 7` while all eight
  of its entries read `Met` or `Implemented`.
- **A vocabulary with two spellings of one status** (`Partial` and
  `Partially implemented`), and `Met` — 31 entries — with no column
  in the table it feeds.
- **Three wrong test counts of eight**: `view_state` 5→9,
  `aggregate_privacy` 12→13, `optimistic_lock` 21→22.

**The claim to falsify**: *nothing a user runs changed.* This release
touched one production file — none. **Prove it**: the diff against
`0.56.0` restricted to `crates/*/src` outside `peisear-web/src/spec_scan*`
and `peisear-web/src/*/tests.rs`, stated as empty or explained line
by line.

**The claim this release must not make**: that the specification is
now verified. **Six rules read it; `§10.34`/`REQ-003` already
established that whether a requirement's status matches the shipped
code is not mechanically decidable**, and `SPEC-003` established the
same for prose counts. The notes say what is checked **and what is
not** — `spec_scan`'s own module doc is the model and the changelog
should borrow from it.

## §3 — `DEC-028`: both specifications, five passes

- **Pass 5 greps for the instruction, not the document.**
- **Appendix A recomputed from its own rows** — and this release is
  the one where that is finally guarded. It is
  `99 | 96 | 1 | 2 | 0 | 0` and `69 | 60 | 3 | 0 | 2 | 4` at
  `34697e6`, **derived independently in review and agreeing cell for
  cell across all 26 rows.** Rule (a) and rule (b) both check it now;
  the pass is reading them, not re-adding.
- `docs_scan`, `changelog_scan` and **`spec_scan`** green — the new
  one is part of the sweep from this release on.
- `external-design.md`: it gained `§10.N` citations that `SPEC-001`
  resolved. Confirm its own Appendix row is written.

## §4 — the gate table, re-measured against the candidate commit

Not quoted from any round:

- **`DEC-007` — 549.** Measured at `34697e6` in review; re-measure at
  the candidate. `515` at `0.56.0`, `+14` `SPEC-001`, `+17`
  `SPEC-002`, `+2` `SPEC-003`, `+1` `SPEC-005`, `+0` `SPEC-004`.
- `spec_scan` — **34** tests; `peisear-web --lib` — **155**.
- The seven `SPEC-005` CI steps green, and **one of them seen red**
  by a planted absence at least once in this release — it was, in
  `SPEC-005` round 2; cite that rather than re-planting.
- `fmt`; `clippy --workspace --all-targets`, both profiles; `clippy`
  under `--features postgres` for all four crates that carry it;
  `rustdoc`; `mdbook` + linkcheck; `publish-dry-run`.
- CI on the candidate: all 46 jobs. **Quote the run's own conclusion
  and its non-success count; do not sum the jobs.**

## §5 — what this release does not prove, stated not implied

- **A requirement's status is not checked against the code.**
  `§10.34`/`REQ-003` measured four candidate rules against six
  historical instances; the best scored 4 of 6 and was argued down by
  its own authors. Nothing here changes that.
- **Most prose counts stay unchecked**, by design and in writing —
  `SPEC-003`'s three shapes. Only the eight `` `name` (N tests) ``
  citations are held to the runner.
- **The 24 dual-register numbers are an unchecked population**,
  recorded in the module doc and in a named test.
- **Thirty-seven `file:line` citations remain the fragile form**, and
  the class demonstrated itself twice during this release. Recorded
  on `ROADMAP.md` for a later round, not fixed here.
- **`optimistic_lock_atomicity`'s own `14`** sits outside the
  extractor's pattern by decision; the population is eight, not nine.

## §6 — escalate rather than deciding

- **If any rule goes red on the candidate.** Stop. A guard that is
  red at the cut is either a true finding or a broken rule, and
  shipping it either way is wrong.
- **If the production diff in §2 is not empty.** Stop and explain it
  line by line; this release claims to have changed nothing a user
  runs.
- **Legs 1–4 of `DEC-056` are the architect's, on the owner's
  approval.** The dev team never tags, never publishes, never creates
  a release.

## §7 — exit condition

Version bumped once; `[0.57.0]` written and dated, stating §2's four
findings and §5's five limits; `ROADMAP.md`'s `## What ships next`
moved with the shipped line written and exit criteria 1–8 each
satisfied in writing; §2's claim falsified by an empty production
diff against `0.56.0`; `DEC-028`'s five passes with Appendix A read
from its own now-guarded rules rather than re-added; every gate in §4
re-run after the last edit and quoted from its own output.

**No tag. No publish. No release.** Those are four legs the architect
runs when the owner approves the candidate.
