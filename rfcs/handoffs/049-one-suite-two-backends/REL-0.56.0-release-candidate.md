# REL-0.56.0 — release candidate

**The series' last item.** Six rounds closed: `CON-001` (the
passthrough), `CON-002` (the suite that runs whole), `CON-005` (the
transaction seam), `CON-006` (the integer widths), `CON-003` (the lock
tests), `CON-004` (the operations story). Four of those six were
discovered by measurement after the series was scoped as four — which
is the honest subject of this release's own notes.

`RFC 018` **P4's gate is met for the suite the RFC named**: both
backends green on `peisear-storage`'s own suite and the 37 lock tests,
and `NFR-CMP-003` restated for the SQLite backend.

---

## §0 — already done; reconcile, do not redo

- `backup.md`'s PostgreSQL section, its CI proof step, and the
  query-parameter correction (`786dfd5`).
- `NFR-CMP-003` scoped; `§10.47` closed; `§10.48` opened.
- `scaling.md` rewritten against the tree.
- `docs/development/migrations.md` and its own expiry clause.

**None of it is re-opened here.** Read it, confirm it still says what
it said, and move on.

## §1 — the ordinary cut

1. **Version** — `0.56.0` in the workspace `[workspace.package]`, and
   nowhere a second time.
2. **`CHANGELOG.md`** — a `[0.56.0]` section, dated. There is none
   today and `[Unreleased]` is empty (confirmed by `REL-0.56.0`'s own
   carried-items check, not assumed).
3. **`ROADMAP.md`** — `## What ships next` moves on; the shipped line
   for 0.56.0 is written; 0.57.0 becomes next. **That section is the
   single home for release numbers** and no other file gains one.
4. **Release cycle exit criteria 1–8**, each stated with what
   satisfied it. **There is no criterion 9** — one was drafted and
   withdrawn before first use, and its property became `NFR-REL-011`.
   A round that cites a ninth has misread the list.

## §2 — the claim this release makes, and the one it must not

**The claim**: a second backend an operator still cannot run, now
honest about why — and a conformance suite that would notice if it
broke.

**The claim to falsify before writing it**: *nothing an existing
SQLite operator runs has moved.* `CON-005` rewrote how fourteen
production writes open their transaction and `CON-006` widened every
integer column in `migrations-postgres/`. Both were built to leave
SQLite untouched. **Prove it rather than assert it:**

- `DEC-007` at **515**, unchanged across the whole series.
- `migrations/` (SQLite) **byte-identical** to `0.55.0` — state the
  `git diff 0.55.0..HEAD -- crates/peisear-storage/migrations/`
  output, which must be empty.
- **The upgrade walk**: install `0.55.0` from crates.io, open its
  database with the candidate, confirm the migration version does not
  move and an existing session cookie still answers 200. `0.54.0`
  and `0.55.0` both walked this; it is the only evidence that
  `CON-005`'s seam did not change SQLite's own behaviour in a way no
  test covers.

**The claim this release must not make**: that PostgreSQL is usable.
`scaling.md` and `backup.md` both already say it is not. **The
changelog must agree with them** — 37 of 335 integration tests run
against it, the P0 privacy tests among the 298 that do not, no
install path, no second archive, no second container tag.

## §3 — the correction `0.55.0` needs, stated in `[0.56.0]`

`0.55.0`'s own section said the remaining PostgreSQL suite failures
were *"not because a production query is wrong."* That gloss was
measured against the failures whose cause was a `sqlite://` test
helper and asserted as the cause of all of them; `BEGIN IMMEDIATE`
was a second cause, in production code. *"Every production query
runs"* **stands** — a transaction-control statement is not a query.

**`[0.55.0]`'s own section is not edited.** Shipped history is not
rewritten (`0.53.0`'s precedent). `[0.56.0]` states the correction,
`§10.47` already records it, and `lib.rs`'s crate-root doc is already
corrected.

## §4 — a drift this release must reconcile

**The gate built to back `backup.md` no longer backs the part that
changed.** `test.yml`'s step is named *"backup.md's own PostgreSQL
commands, proved against a real server"* and its comment claims it
*"proves the exact commands that document names, not a paraphrase of
them."* Since `786dfd5` the document tells an operator **not** to use
`${DATABASE_URL%/*}/peisear_restored`, and the step still does
exactly that.

The step is not wrong as a mechanism test — its own `DATABASE_URL`
carries no query parameters, so the transform works there. **It is
wrong as a proof of the document**, and its name and comment now
overstate what it does.

**Reconcile by making it prove the harder case, not by weakening the
comment**: give the step a `DATABASE_URL` that carries a query
parameter (`?sslmode=disable` against this container) and build the
restore target the way the document now says — by hand, keeping the
parameters. The step then proves the guidance *including* the case
the document exists to warn about, which is strictly more than it
proved before.

**`§10.36`'s family**, and worth one line in the round's own report:
a gate whose stated purpose outlives what it verifies.

## §5 — `DEC-028`: both specifications, five passes

- `requirements.md` and `external-design.md` both amended.
- **Pass 5 greps for the instruction, not the document** — the
  distinction that has caught this twice.
- **Appendix A recomputed from its own rows**, never adjusted by
  delta. It is `69 | 57 | 4 | 0 | 2 | 6` (non-functional) and
  `99 | 77 | 4 | 16 | 0 | 2` (functional) at `0bd33f3`; both were
  recomputed in `CON-004`'s review and are consistent. **If §6 lands,
  both change** — recompute, do not add one.
- `docs_scan` rules 1–7 and `changelog_scan` rules 1–7 green,
  including rule 7's *every git tag has a section*.

## §6 — `NFR-CMP-007`, if and only if the owner has authorized it

`CON-004`'s review §6 proposed a sibling entry — *Persistent state on
the PostgreSQL backend* — with full text, Implemented, P2, and
Appendix A's Compatibility row becoming `7 | 7 | —`.

**Do not write it on this handoff's word.** The series README's own
State column says when the answer has arrived. Until then:

- If **authorized**: add it as written, recompute Appendix A from its
  rows (`70 | 58 | 4 | 0 | 2 | 6`), and reconcile
  `external-design.md`.
- If **declined**: say so in `[0.56.0]`'s own notes in one sentence,
  so the specification's silence about PostgreSQL is a recorded
  decision rather than a gap. **Silence with no record is the one
  outcome that is not acceptable.**
- If **unanswered**: everything else in this round can be done and
  this is the last edit — **but the record is still required.** Add,
  under `NFR-CMP-003`'s own amendment block, a paragraph naming the
  proposed sibling entry, its full text, the date it went to the
  owner, and that it is **undecided at the cut** — noting that
  `backup.md` already says all of it and what is open is whether the
  specification should require it. Do not add `NFR-CMP-007` itself,
  do not touch Appendix A, and do not write it into `[0.56.0]`'s
  notes as a decision. Do not block the rest on it; do not guess it.

  **Corrected 2026-10-22, after round 1.** This case first said only
  *"do not block the rest on it; do not guess it"* and said nothing
  about the record — which reaches the same outcome case 2 calls
  unacceptable, a specification silent about PostgreSQL with no trace
  of why. Round 1 followed this case exactly as written and is not at
  fault for where it led. The three cases now differ in what they
  decide, not in whether they leave a record.

## §7 — the gate table, re-measured against the candidate commit

Not quoted from any round. Re-run, after the last edit:

- `DEC-007`, fresh block, three runs — **515**.
- `cargo test -p peisear-storage --lib` — **31**; and
  `--no-default-features --features postgres --lib` unfiltered —
  **56**, 0 ignored.
- The three lock-test targets on both backends — **22 / 14 / 1**.
- `fmt`; `clippy --workspace --all-targets`; `clippy
  --no-default-features --features postgres --all-targets` for all
  four crates that carry the feature; both profiles.
- `rustdoc` with `-D rustdoc::broken_intra_doc_links`; `mdbook build`
  + linkcheck.
- `cargo publish --workspace --dry-run`.
- CI on the candidate commit: all 46 jobs. **Quote the run's own
  conclusion and its non-success count; do not sum the jobs.**

## §8 — what this release does not prove, stated not implied

- **298 of 335 integration tests have never run against PostgreSQL**,
  among them the only tests proving `NFR-PRIV-007`'s contributor floor
  and `NFR-PRIV-001`'s capacity visibility. Both are **P0**.
- **The alias class is open.** `CON-003` fixed the one `HAVING` in the
  crate; a `SELECT`-list alias used where PostgreSQL forbids it cannot
  be found by grepping a keyword, and thirty integration files have
  never rendered against the backend.
- **`§10.48` is open**: a `--features postgres` build still links the
  bundled SQLite C library.
- **No supported PostgreSQL version range is declared**, and
  `backup.md`'s unaliased-subquery observation is unmeasured on
  majors below 17.

**These belong in the release notes, not only in this handoff.** The
release whose subject is a second backend is the wrong place to be
vague about what the backend cannot do.

## §9 — escalate rather than deciding

- **If the upgrade walk moves anything.** Stop. That is the claim in
  §2 and it is the release's own gate.
- **If `migrations/` differs from `0.55.0` at all.** Stop.
- **If §6's answer has not arrived**, proceed without it per §6's
  third case; do not ask twice and do not guess.
- **Legs 1–4 of `DEC-056` are the architect's, on the owner's
  approval.** The dev team never tags, never publishes, never creates
  a release.

## §10 — exit condition

Version bumped once; `[0.56.0]` written and dated, stating `0.55.0`'s
correction and §8's four limits; `[0.55.0]` untouched; `ROADMAP.md`'s
`## What ships next` moved with the shipped line written and exit
criteria 1–8 each satisfied in writing; §2's claim falsified by a
`migrations/` diff that is empty, `DEC-007` at 515 and a completed
upgrade walk from `0.55.0` on crates.io; §4's CI step reconciled so it
proves the document it names, including the query-parameter case;
`DEC-028`'s five passes with Appendix A recomputed from its own rows;
§6 handled by whichever of its three cases applies; every gate in §7
re-run after the last edit and quoted from its own output.

**No tag. No publish. No release.** Those are four legs the architect
runs when the owner approves the candidate.
