# SUP-003 — the install path, and what may say so

**0.59.0, third of four. Blocked on `SUP-002`** — `RFC 021` §4 is
explicit that (1) and (2) are written **last**, because a documented
install path for something the suite has not yet proven is how this
defect started.

**Do not start this round until `SUP-002` has reported green.** If it
reported defects instead, this round does not run at all and
`RFC 021` §5 is the subject.

## §1 — the contradiction to remove

Measured 2026-10-29: `docs/operations/` documents a PostgreSQL
deployment on three pages — `backup.md` (a full section, every command
proven against a real server), `scaling.md`, `background-jobs.md` —
while `docs/getting-started/` never mentions it. `install.md`'s three
install paths are SQLite, every one.

An operator who meets `backup.md` first believes the backend is
supported and then finds no way to start it. `RFC 021` §3's negative
half is the rule this breaks: **operations documentation may not exist
for a configuration that fails the rule**, because a reader meets it
already believing they are running the thing.

## §2 — what to build: conditions (1) and (2)

**(1) Reachable from getting-started.** A reader who has read nothing
gets from `install.md` to a running PostgreSQL instance. That means a
real command, not a pointer to the backup page — and it means saying
which install paths support it. The container image, the archives and
`cargo install` are three different answers and this round must say
which of them can run PostgreSQL at all, measured rather than assumed
(`SUP-001` changed what the default build links; check, do not
inherit the old belief).

**(2) Variables documented where configuration is documented.**
`configuration.md`, not a crate's rustdoc. `DATABASE_URL`'s
PostgreSQL form belongs there with its own example.

**(4) The version range, as `SUP-002` actually exercised it** —
copied from that round's own measurement, not re-reasoned here.

**And reconcile the three operations pages** against what now exists.
Where one of them describes something the install path does not
support, it is the page that is wrong.

## §3 — the guards this round must expect

`docs_scan` rule 4 ties documented defaults to `config.rs`, and rule 7
ties `.env.example`'s uncommented keys to their own defaults. A
PostgreSQL `DATABASE_URL` example is prose, not a default — **it must
not become an uncommented `.env.example` key**, which would make the
example file name a backend the default build may not even carry.

`docs_scan`'s install-path rules are the ones most likely to object
here. **If a guard objects, report it** — do not widen a rule to
accept a new document shape without saying so, which is the
`§10.39`-adjacent move this project has already corrected once.

## §4 — what this round may not do

**It may not claim a supported configuration that `SUP-002` did not
exercise.** If `SUP-002` ran one PostgreSQL version, this round
documents one. If it skipped a test as backend-specific, this round
does not imply that area is covered.

**It may not write an operations page for anything new.** The three
that exist are to be reconciled, not joined by a fourth; `RFC 021`
§3's negative half cuts both ways.

## §5 — gates

`DEC-007` re-extracted and counted with the runner — likely unchanged,
but measured. `docs_scan`, `changelog_scan`, `spec_scan` green. Then
`fmt`, `clippy`, rustdoc with `CONTRIBUTING.md`'s own flag,
`mdbook build docs` with linkcheck, `publish --dry-run`, after the
last edit, quoted.

**`NFR-REL-011` is in scope.** It requires every documented install
path to be exercised end to end, reaching an authenticated session. A
new documented install path means that walk must exist for it, in CI,
not by hand. Whether the entry's own status moves is a question to
answer either way rather than leave unasked.

## §6 — escalate rather than deciding

- If an install path cannot support PostgreSQL and the honest answer
  is to document it as unsupported for that path — that is a design
  call about what the three paths mean.
- If `NFR-REL-011`'s walk for PostgreSQL cannot be built at
  acceptable CI cost.
- If reconciling an operations page would mean deleting documented
  behaviour someone may be relying on.

## §7 — exit condition

1. `install.md` reaches a running PostgreSQL instance, naming which
   install paths support it, measured.
2. `configuration.md` documents `DATABASE_URL`'s PostgreSQL form; the
   version range stated as `SUP-002` exercised it.
3. The three operations pages reconciled; any that describes the
   unsupported is corrected.
4. `.env.example` unchanged, or the reason it changed stated.
5. `NFR-REL-011`'s walk exists for the new path, in CI, and the
   entry's own status question answered either way.
6. **Carried in from `SUP-002`'s review — one line, and this round is
   the one that edits `test.yml` anyway** (item 5 above). The postgres
   job's `services:` block passes `--health-*` options and nothing
   else, so its container runs at PostgreSQL's default **64 MB** of
   shared memory — and after `SUP-002` that single container now runs
   **33** `--test` steps with **no restart between them**, which is
   exactly the accumulation that exhausted 64 MB locally (`53100`,
   *"could not resize shared memory segment"*). CI passes today, 33 of
   33 green, so this is headroom and not a defect — **but the headroom
   is unmeasured and the failure mode reads like a disk problem.** Add
   `--shm-size=512m` to that `options:` block, matching the local
   invocation the `SUP-002` handoff's own `§0` documents, so the two
   sides stop differing in a way nobody chose. Run the postgres job and
   quote it; editing a gate obliges running that gate.
7. The gates in §5, re-run after the last edit, quoted.
