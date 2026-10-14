# STORE-001 — the lock token that does not move

**`RFC 018` P0**, accepted 2026-10-11. **Target**: 0.50.0, first round.

**`NFR-CONC-001` is P0 and has a window in which it cannot do its job.**
SQLite's `CURRENT_TIMESTAMP` is whole-second; `NFR-CONC-002` reuses
`updated_at` as the optimistic-lock token; so **two writes to the same row
inside one second leave the token unchanged**, and a client holding a stale
token compares equal and is admitted.

My own run, with the trigger shape copied verbatim from
`0017_updated_at_single_authority.sql`:

```
after insert    | 2026-10-11 05:29:06
after update A  | 2026-10-11 05:29:06
after update B  | 2026-10-11 05:29:06
```

**Three mutations, one token.**

---

## §1 — demonstrate it first, through the handlers

**I verified the mechanism in SQLite. I did not demonstrate a lost update
through the product, and until you do, this is a proven mechanism and a
probable defect.** Do not start with the fix.

**The demonstration is simpler than it sounds and needs one user**, because
the defect is that a token can be spent twice:

1. Create an issue. Read its `updated_at` — call it **T0**.
2. Submit an edit carrying **T0**. It succeeds; the trigger fires.
3. **Submit a second, different edit carrying T0 again**, inside the same
   second.

**Expected by `NFR-CONC-001`: 409.** Expected by this defect: **200, and the
second edit silently overwrites the first.** Whichever you observe, report
it — *the demonstration is the deliverable of this step*, not a formality
before the fix.

Put it in the **`optimistic_lock`** test crate, where its siblings live. **It
must fail before the fix and pass after**; a test that passes on the
unfixed tree is testing something else, and `§10.36`'s whole subject is
checks that cannot tell the difference.

**If it does not fail**, stop and say so. Something upstream — a re-read, a
timing accident, a serialisation — is protecting the path, and I would
rather know what than ship a migration against a defect that isn't there.

## §2 — the fix: seven triggers, and not the defaults

Millisecond precision: `strftime('%Y-%m-%d %H:%M:%f','now')` in place of
`CURRENT_TIMESTAMP`. A new forward-only migration, **`0021`**
(`NFR-CMP-004`), dropping and recreating each trigger — SQLite cannot alter
one in place.

**Exactly seven triggers, all of them timestamp bumps** (the other four in
the schema are constraint triggers and are not touched):

| Migration | Triggers |
|---|---|
| `0014` | `sprints_updated_at`, `teams_updated_at`, `team_memberships_updated_at`, `user_capacities_updated_at` |
| `0017` | `issues_updated_at`, `projects_updated_at`, `user_view_states_updated_at` |

**Leave the column defaults alone.** `DEFAULT CURRENT_TIMESTAMP` appears
**19 times across ten migrations**, and SQLite cannot change a default
without rebuilding the table — twelve table rebuilds for no benefit:

- The default applies at **INSERT**; the lock is spent on **UPDATE**.
- A row inserted at second precision whose first update fires the trigger
  moves to millisecond precision, **which differs from the token the client
  read**, so the stale write is correctly rejected. The window closes on the
  first update either way.

**If you find a path where the default's precision is load-bearing, say so
rather than rebuilding tables on your own judgement.**

**Keep the `WHEN OLD.updated_at = NEW.updated_at` guard** exactly as it is.
It exists because the application once wrote the column itself; removing it
is a separate question with its own history (`QA-018`, `QA-019`).

## §3 — the way this fix silently does nothing

**The round trip, not the storage.** The token leaves the database as a
`DateTime<Utc>`, is rendered into a form by `to_rfc3339()` (20 call sites),
comes back as a string, and is compared by **equality** in
`check_optimistic_lock` (`peisear-web/src/error.rs:306`).

**Milliseconds must survive every leg.** If `sqlx`'s SQLite `DATETIME`
decoding truncates the fraction, or a render path formats to seconds, the
stored value gains precision the comparison never sees and **the
demonstration in §1 still fails after the migration** — a fix that looks
landed and changes nothing.

**Assert the round trip directly**, not only the end-to-end behaviour: a
value written by the trigger, read back, rendered, parsed, and compared
equal **with its milliseconds intact**.

**And both shapes must parse.** Rows written before `0021` keep
`YYYY-MM-DD HH:MM:SS`; rows written after carry `.SSS`. A decoder that
handles one and not the other breaks every pre-migration row. **Test both**
— write a row, migrate, read it, then update it and read it again.

## §4 — gates

- **`DEC-007` will move.** §1's demonstration, §3's round-trip assertion, and
  whatever §2 needs. **Say the number and what each added test is.**
- **`0021` is the eleventh migration**, the first since `0020`. The release
  candidate's database check changes from *"comes up at 20 with nothing
  applied"* to *"comes up at 21 with one applied"* — **note it in your
  package** so the candidate round does not read the old expectation as a
  failure.
- **The three browser gates**, because the trigger fires on every mutation
  the drag surfaces make.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken).
- **A `0.49.0` database, migrated.** Build the `0.49.0` binary from its tag's
  own commit, create a database, write a row, then run the candidate binary
  against that file and confirm **both** the old row and a newly updated one
  read back correctly.

## §5 — escalate rather than deciding

- **If §1's demonstration does not fail on the unfixed tree.**
- **If the round trip cannot preserve milliseconds** without changing what
  goes over the wire — that is a wire-format decision and it is mine.
- **If a column default turns out to be load-bearing** for the lock.
- **If `strftime` and `CURRENT_TIMESTAMP` disagree about the clock** in any
  way that matters (UTC, the `'now'` modifier's resolution).

## §6 — exit condition

A demonstration in `optimistic_lock` that **failed before the migration and
passes after**, reported with both runs; migration `0021` rewriting seven
triggers and no table; a round-trip assertion proving the milliseconds
survive storage, decoding, rendering and parsing; both timestamp shapes
reading correctly across the migration boundary; `DEC-007` stated with each
new test named; and the candidate-round expectation of **21** recorded for
whoever cuts 0.50.0.
