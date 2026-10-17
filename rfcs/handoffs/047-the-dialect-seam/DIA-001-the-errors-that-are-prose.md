# DIA-001 — the errors that are prose

**Target**: 0.54.0, first. `RFC 018` **P1** (§4). Gate, from the RFC:
*"existing validation tests green, no message text in any match arm."*

**Worth doing even if PostgreSQL never ships** — the RFC's own test for
a prerequisite, and this one passes it: a reworded trigger message is
a silently broken validation path today.

---

## §1 — what is there, measured 2026-10-17

`crates/peisear-storage/src/issues.rs:386`, `translate_trigger_error`,
matches `sqlx::Error`'s **rendered string** against five needles and
maps each to a `MessageKey`.

**The RFC says three call sites. There are five** — lines 233, 308,
371, 519, 744. *Measure the rest of this section yourself too*; the
figures below are mine, from today, and the RFC's were right when it
was written.

**Five distinct `RAISE(ABORT, …)` messages, eight occurrences** across
`crates/peisear-storage/migrations/`:

| message | occurrences |
|---|---|
| `sub-issue must share project with its parent` | 2 |
| `sub-issue cannot have a sub-issue (1-level only)` | 2 |
| `Planned end date must be on or after planned start date.` | 2 |
| `cannot demote an issue that has its own sub-issues; promote them first` | 1 |
| `an issue cannot be its own parent` | 1 |

**Two of the five needles are prefixes of the real message**, not equal
to it — `contains` hides that today. A message reworded *after* the
matched prefix keeps working; one reworded *within* it fails silently
and no test says so.

## §2 — what to build

**An application error code raised by the database and matched on the
code**, never on prose.

- SQLite has no `ERRCODE`, so the code goes **in** the `RAISE` message
  in a form that cannot be mistaken for prose — a short stable token.
  **Choose the shape and say why**; a bare word that could appear in
  an unrelated message is a worse needle than today's sentence.
- The translation becomes code → `MessageKey`. **No message text in
  any match arm** — the RFC's gate, and it is checkable by reading the
  function.
- **The human sentence stays where it already lives**, in
  `peisear-i18n`'s `MessageKey`. This round moves *matching* off prose;
  it does not move *text*.

**PostgreSQL is not in this round.** Design the code set so
`RAISE EXCEPTION … ERRCODE` can carry the same values later, and
**write that intention down**; do not add a second backend's syntax to
a migration now.

## §3 — the guard, because this is a needle-and-haystack defect again

Every `RAISE(ABORT, …)` in a migration must carry a code the
translation knows, and every code the translation knows must appear in
a migration. **Both directions**, as a test — `§10.39` is exactly this
class and this project has had it twice.

**Plant it both ways before trusting it**: add a `RAISE` with an
unknown code, watch it fail; remove a code the translation knows,
watch it fail. A guard never seen to fail is `§10.36`.

## §4 — specification work (`DEC-028`)

- **`NFR-MNT-*`** — the brittleness this removes is a maintainability
  property. Check whether an entry already claims it; **report rather
  than invent one** if none does.
- The `translate_trigger_error` doc comment says *"brittle to RAISE
  message wording — keep the test for it close"*. After this round
  that sentence is false. It is also the honest record of why the
  round happened: **rewrite it, do not delete it.**

## §5 — gates

- `DEC-007` from a fresh block. **This moves** — new tests for the
  guard, and possibly fewer for the old needles. Account exactly.
- `fmt`, `clippy` both profiles, rustdoc after the last edit, `mdbook`.
- **Migrations change, so `cargo test -p peisear-storage --lib` is the
  one to watch**, and a fresh database must still apply the whole
  migration chain.
- `publish-dry-run` runs itself in CI now (`FIV-007`); **confirm it
  stayed green** rather than assuming migrations cannot affect it.

## §6 — escalate rather than deciding

- **If a code cannot be carried through SQLite's `RAISE` without
  looking like prose.** Bring the shapes you tried.
- **If the five messages turn out not to be five** — the figures above
  are mine and a week old by the time you read them.
- **If a migration needs rewriting rather than amending.** Migration
  history is append-only here; changing a shipped one is a decision,
  not a step.

## §7 — exit condition

No message text in any match arm, shown by reading the function; the
code set chosen with its shape reasoned and its PostgreSQL future
written down; the both-directions guard built and **seen to fail both
ways**; the stale doc comment rewritten rather than deleted; every
existing validation test green; `DEC-007`'s delta accounted for
exactly; all gates re-run after the last edit.

---

# DIA-001 round 2 — the amended migrations break every 0.53.0 install

Round 1's code is accepted (see the review): the codes, the registry,
the both-directions guard and the real-trigger test are all right, and
the `strip_prefix` bug you caught yourselves is the best thing in the
round. **One decision has to be reversed, and it is not about the
code.**

## §8 — `sqlx` refuses to run a modified migration

`0015_sub_issues.sql` and `0016_issue_planned_dates.sql` were amended
in place. **Both shipped in 0.53.0** (`git ls-tree 0.53.0` lists them),
and `sqlx` records every applied migration's checksum:

```rust
// sqlx-core-0.8.6/src/migrate/migrator.rs:176
if migration.checksum != applied_migration.checksum {
    return Err(MigrateError::VersionMismatch(migration.version));
}
```

**Demonstrated at the review, not argued.** A database created by a
build carrying 0.53.0's own migration text — 21 migrations applied,
`/health` 200 — then opened by a build carrying this round's:

```
Error: migration failed: migration 15 was previously applied but has been modified
```

**It does not start.** Loud, named, and unrecoverable without manual
surgery on someone else's database.

## §9 — the precedent you cited expired three days before you cited it

`§7` answers the migration question in place, on
*"no production use yet"*. That was the owner's ruling on 2026-10-14
**and it carried its own expiry**: *"expires at the first real
deployment."*

**0.53.0 shipped on 2026-10-17** — seven crates on crates.io, five
archives on the Release, `ghcr.io/nabbisen/peisear` at `0.53.0` and
`latest` — and its entire subject was making installation easy for
someone with nothing. **Anyone can have a database now.**

**The architect wrote that expiry and did not flag it when scoping
this release**, three days after shipping the thing that triggered it.
The citation was reasonable; the precedent was not.

**And the handoff listed this as an escalation trigger**: *"If a
migration needs rewriting rather than amending — migration history is
append-only here; changing a shipped one is a decision, not a step."*
`§7` reports it as *"answered rather than escalated"*. The answer was
wrong, which is the case escalation exists for — **when a trigger
fires, the report is the deliverable, not the decision.**

## §10 — what to do instead

**Restore `0015` and `0016` byte-for-byte to their 0.53.0 content**
(`git show 0.53.0:<path>` — reading a committed file, not working
around anything), and add **`0022`** that drops and re-creates the
five triggers with the coded messages.

- **Verify the restoration by checksum, not by eye**: a database
  created before your change must start cleanly against the restored
  files. The walk above is the shape; run it.
- **`0022` is additive**, so an existing database applies it and a
  fresh database gets the same end state by a different route.
  **Prove both**: upgrade a 0.53.0-era database, and migrate a new one
  from empty, and show the five triggers identical in each.
- **The both-directions guard reads migration text.** Codes now live
  in `0022` while `0015`/`0016` carry the original prose — **so the
  guard must not demand that every `RAISE` carry a code**, or the
  restored files fail it. Decide what it asserts now and say why;
  this is the round's real design question, not the restoration.

## §11 — exit condition

`0015` and `0016` byte-identical to `0.53.0`, verified by comparing
against `git show 0.53.0:<path>`; `0022` adding the coded triggers;
**an upgrade from a 0.53.0-created database shown to start**, and a
from-empty migration shown to reach the same triggers; the guard
re-stated to survive both files coexisting, with its new assertion
reasoned; `DEC-007`'s delta accounted for; all gates re-run, including
`publish-dry-run`.

---

# DIA-001 round 3 — the guard knows a filename, not the property

Round 2 is accepted (see the review). The restoration is byte-exact,
`0022` is correct, and the upgrade is proved rather than argued. **One
small thing, and it is about durability rather than correctness.**

## §12 — both checks are keyed on today's artefacts

- **`all_raise_messages` reads one named file.** `trigger_error_codes_migration()`
  returns `0022_trigger_error_codes.sql`, and both directions run
  against that file alone.
- **The upgrade test iterates a hard-coded list of four trigger
  names**, asserting each one's `sqlite_master.sql` is identical
  across the two databases and contains `[E_`.

**Both are right today and neither survives the next migration.** Add
`0023` tomorrow with a new validation trigger and an uncoded
`RAISE(ABORT, 'some new rule')`:

- `translate_trigger_error` will not recognise it, so the error reaches
  the user as a raw `Database` error instead of a typed `Validation` —
  **the exact defect this whole round exists to prevent**;
- the guard reads `0022` and stays green;
- the upgrade test checks four names that do not include it and stays
  green.

**This is `§10.39` again** — *a rule named for a subject and keyed on a
string enforces the string.* The subject is *every live trigger that
raises*; the string is a filename and a list of four.

**Round 2's reasoning for the re-scope was sound** and is not being
reversed: `0015`/`0016` carry uncoded prose forever by design, so a
guard over all migration *files* would fail on them permanently. The
conclusion that follows is not "scope to one file" but **"stop reading
files."**

## §13 — assert the end state, which you already have the machinery for

The upgrade test already opens a migrated database and reads
`sqlite_master`. Generalise it:

> **Every trigger in `sqlite_master` whose SQL contains
> `RAISE(ABORT,` must carry a code `TRIGGER_ERROR_CODES` knows** —
> and every code it knows must appear in some live trigger.

- **No filename, no name list.** `0015`/`0016`'s prose is irrelevant
  because those triggers no longer exist after `0022` drops them —
  which is the property the file-based guard could only approximate.
- **A future migration is covered the day it lands**, with nobody
  remembering to extend a list.
- **It also catches a case neither check can see today**: a trigger
  dropped and never re-created, leaving a code in the registry that
  nothing raises.

**Keep the planted-parameter shape** — `known` stays a parameter so a
shrunk or widened copy can be planted without touching the committed
`const`. That part of round 1 was right and is why these plants are
possible at all.

**Then decide what remains of the file-based guard.** My expectation is
that `all_raise_messages` and `trigger_error_codes_migration` can both
go, because the database answers the same question better. **If you
find a property only the file version can check, say what it is and
keep it** — but say it, rather than keeping both because removing code
feels risky.

## §14 — exit condition

Both directions asserted against **live triggers read from a migrated
database**, with no filename and no hard-coded name list; a planted
shrunk and a planted widened `known` list each shown to fail; the
four-name loop in the upgrade test replaced by the general form, with
the identical-across-both-databases assertion kept; the file-based
guard removed or its surviving reason stated; `DEC-007`'s delta
accounted for; all gates re-run, `publish-dry-run` included.
