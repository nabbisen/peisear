# SUP-002 — the suite on both backends

**0.59.0, second of four. Blocked on `SUP-001`** — running the suite
under `--features postgres` means little while that build also links
SQLite.

**This round decides whether the rest of the release happens.**
`RFC 021` §4 puts it first among the four conditions because it is
the one that can falsify the others: if PostgreSQL does not actually
work, an install path for it must not be written, and `RFC 021` §5
(end it) becomes the answer instead. Report what you find; do not
make the finding fit the plan.

## §1 — the population

**3 of 33** `peisear-web` integration binaries run against PostgreSQL
in CI today (`optimistic_lock`, `optimistic_lock_atomicity`,
`board_card_lock_value`). The other **30** have never run against it.
Among them are the only tests asserting `NFR-PRIV-007`'s contributor
floor and `NFR-PRIV-001`'s capacity visibility, both **P0**.

**Run the whole population, unfiltered.** `CON-002` already replaced a
hand-picked name list for `peisear-storage` with an unfiltered run,
for the reason `§10.46` records: a list cannot tell *green* from
*examined nothing*. Do not build a second list here. Where a test is
genuinely SQLite-specific, **name it and gate it**, as `CON-002` did
with its two exceptions, and say why in the test's own source rather
than in CI configuration.

## §2 — expect defects, and treat them as the deliverable

`CON-006` found that `SUM(int8)` promotes to `numeric` where
`SUM(int4)` promotes to `int8`. `CON-003` found a `SELECT`-list alias
inside `HAVING` that PostgreSQL rejects, and closed it at the one
occurrence while explicitly leaving the class open. **Thirty binaries
have never rendered against this backend**; the alias class alone is
reason to expect more.

For each defect: fix it, and say whether the fix closes the instance
or the class. Those are different claims and this series has already
conflated them once.

## §3 — the version range, which comes out of this round

`RFC 021` §3 condition (4). The range is **what CI actually runs
against**, not a guess and not what the ecosystem supports.
`backup.md` was proven against `postgres:17`. If this round runs one
version, the declared range is one version, and saying so plainly is
the honest answer — a range of one is still falsifiable, which is the
point of the condition.

If more than one is worth running, that is a cost question: state it
and let `SUP-003` document whatever this round actually exercised.

## §4 — the proof

- **The unfiltered run, green, with its own count**, against a real
  server in CI. Quote the number of binaries and tests that ran; a
  conclusion without a count is what `§10.46` is about.
- **Every exception named**, with the reason in source.
- **A test seen to fail** on the backend for at least one defect this
  round fixes, so the suite is known to be able to report red.
- **Database-per-test isolation already exists** (`CON-003`'s
  `TestDatabase`); if 30 more binaries break it, that is a finding
  about the harness and belongs in the report.

## §5 — gates

`DEC-007` re-extracted and counted with the runner — **this round
will move it**, so measure, do not derive. Then `fmt`, `clippy`
(workspace and each single-backend set), rustdoc with
`CONTRIBUTING.md`'s own flag, `mdbook`, `publish --dry-run`, all after
the last edit, quoted.

CI time will rise. Say by how much, since the owner's own ordering
puts continuing cost above one-off cost.

## §6 — escalate rather than deciding

- **If the suite reveals defects whose repair is open-ended.** Stop
  and report. `RFC 021` §5 exists for exactly this and choosing it is
  the owner's call, not this round's.
- If a test's failure is arguably correct behaviour differing between
  backends rather than a defect — that is a design question.
- If isolation or CI runtime makes the unfiltered run impractical,
  report the measurement rather than quietly reintroducing a list.

## §7 — exit condition

1. All 33 binaries run against PostgreSQL in CI, unfiltered, with
   every exception named and justified in source.
2. Every defect found reported with instance-or-class stated.
3. At least one of them seen red before green on this backend.
4. The version range this round actually exercised, stated as a
   number of versions and not a feeling.
5. `DEC-007` measured with the runner; CI runtime delta stated.
6. The gates in §5, re-run after the last edit, quoted.
