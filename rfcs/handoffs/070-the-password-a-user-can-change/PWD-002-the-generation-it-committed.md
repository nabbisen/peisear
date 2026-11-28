# `PWD-002` — the generation the change actually committed

**Series**: `070`, **reopened**. **Governing RFC**: `RFC 025`
(`rfcs/accepted/`), accepted 2026-11-27.
**Requirement**: `FR-AUTH-006`, **`Implemented`** — and one of its own
`MUST`s has a measured hole.
**Release**: `0.72.0` — the row was authorized 2026-11-28; **the cut is held
for this unit.** That is the architect's judgement, recorded in `§2`, not a
gate you can clear by finishing early.

`PWD-001` was reviewed and **accepted**. Nothing below reopens that
acceptance. Three items were named as owed back in the next round touching
this code, and this is that round.

---

## §1 — the defect, and it is mostly the architect's

```
handlers/settings.rs:208   let user = users::find_by_id(&state.db, rid.as_str())
handlers/settings.rs:219   users::change_password(&state.db, &user.id, &new_hash).await?;
handlers/settings.rs:220   let new_generation = user.session_generation + 1;
```

Line 220 adds one to the value read at line 208. The `UPDATE` at line 219
adds one to whatever the row holds **at commit**. Those are the same number
only if nothing else wrote the row in between.

**The window is not a bus cycle.** Between the read and the write the handler
runs two Argon2 operations — `password::verify` on the current password and
`password::hash` on the new one. Measured 2026-11-28 on the architect's
machine, `Argon2::default()`, release build, warm, five iterations:

```
verify  mean   8.3 ms
hash    mean   8.3 ms
window  mean  16.6 ms   + two database round trips
```

If `logout` commits inside that window, the row goes `G → G+1` (logout) and
then `G+1 → G+2` (this `UPDATE`), while the cookie on the response carries
`G+1`. `extractors.rs:83` — the single choke point `RFC 024` built — compares
`user.session_generation != token_generation` and refuses it. **The user is
signed out of the exact browser `FR-AUTH-006`'s second sentence promises will
stay signed in.**

**Honest about likelihood**: both writers are the same account's, and a human
who signs out everywhere on one device and changes the password on another
does so seconds apart, not milliseconds. It fails safe — no credential
survives that should have died; one dies that should have lived, and the user
signs back in with the password they just chose. **Low likelihood, safe
direction. That is not why the release is held.**

**And the cause is the architect's own instruction.** `PWD-001`'s `§3` step 5
read:

> *take the generation from the value you just wrote, not from a re-read, and
> not from the stale row you loaded*

That is self-contradictory: with the statement as written there **is** no
value you just wrote — only the stale row and a re-read, both of which it
forbids. You implemented the only reading available and documented it in the
handler's own comment. The defect is in the instruction first.

## §2 — why a 16.6 ms window holds a release

It does not, on its own. **The register does.**

`FR-AUTH-006` ships in `0.72.0` as `Implemented`, with this sentence
normative: *every other session of that account MUST become invalid, and the
session that made the change MUST remain valid.* Measurement contradicts the
second clause. Shipping it means `requirements.md` asserts a guarantee the
product does not make — and this document's `§10.36` register exists for
exactly that failure, thirteen instances of it.

The alternatives were weighed and both are worse. Weakening the sentence to
`SHOULD` trades a security-relevant guarantee for a bug. Marking the entry
anything other than `Implemented` is a status this document does not carry —
`FR-AUTH-003` already tried claiming a `Divergent` it never had, and that
correction is in the `0.71.0` amendment note.

**And there is a second half, which is the acceptance citation.**
`FR-AUTH-006`'s `*Acceptance*` field currently says
`changing_the_password_refuses_the_other_devices_session` shows *"a second,
never-replayed session dies"*. It cannot. `jwt::issue` builds `iat` in whole
seconds (`jwt.rs:95`), so two logins for one account inside one second produce
identical claims and a byte-identical token; measured in the architect's walk
of your own round's release binary, device A and device B held the **same**
token. There is one credential there, held twice — which proves the
requirement, and does not prove what the citation says. `§4.1` is the remedy.

So the requirement this release exists to deliver cannot cite honest
acceptance yet. That is the hold.

## §3 — the mechanism, walked before it was handed over

`RETURNING session_generation`. The handler stops computing the number and is
handed the one that was committed.

**Measured 2026-11-28, not assumed** — the last handoff asserted a mechanism
without walking it, which is `§1`:

- **No trigger on `users`.** The two migration sets declare **11** distinct
  trigger names across **8** files, firing on **7** tables — `issues` (15
  occurrences), `projects`, `sprints`, `teams`, `team_memberships`,
  `user_capacities`, `user_view_states` (3 each). **`users`: 0.**
- **`issues.rs`'s own rejection of `RETURNING` does not reach here.** Its
  comment (`issues.rs:599`–`610`) rejects `RETURNING` because
  `issues_updated_at` is an `AFTER` trigger whose nested `UPDATE` runs *after*
  `RETURNING` has computed its row, so it would hand back a value the row has
  already moved past. That reason is trigger-specific. Read it anyway before
  you write the statement; do not take this paragraph's word for it.
- **Walked on the linked SQLite.** `libsqlite3-sys 0.30.1` via `sqlx/sqlite`;
  `RETURNING` needs SQLite ≥ 3.35. In-memory table, caller reads `7`, a
  concurrent `+1` lands, then the `RETURNING` `UPDATE` → **`9`**, where the
  stale arithmetic would have said `8`. A missing row yields
  `sqlx::Error::RowNotFound`.
- **The `NotFound` path survives.** `RowNotFound` is exactly the shape
  `rows_affected() == 0` is reporting today, so the guard is replaced rather
  than dropped. Map it the way this crate already maps `RowNotFound`; find
  that site rather than inventing a form.
- **The dialect seam needs nothing new.** `dialect.rs:293` rewrites `?N` →
  `$N` on the PostgreSQL body, so `WHERE id = ?1 … RETURNING` crosses
  unchanged. PostgreSQL has had `RETURNING` since 8.2. `test.yml`'s
  real-server job is where that is proven, not here.

Shape, not text to paste:

```rust
pub async fn change_password(pool: &Pool, user_id: &str, new_hash: &str) -> StorageResult<i64>
```

— one `UPDATE … RETURNING session_generation`, `fetch_one`, `RowNotFound`
mapped to `StorageError::NotFound`. The handler then becomes

```rust
let new_generation = users::change_password(&state.db, &user.id, &new_hash).await?;
```

which is **one line shorter** than what it replaces.

**Delete the architect's sentence from the handler's doc comment.**
`handlers/settings.rs:188`–`192` quotes `PWD-001` `§3` step 5 and explains
that `new_generation` is *"computed from the row already in hand plus the one
the storage layer's single `UPDATE` is guaranteed to have applied"* — a
guarantee that does not exist. It now reads as a justification for the defect.
Replace it with what will be true: the generation comes from the statement
that wrote it.

## §4 — two tests whose names claim more than they establish

### §4.1 One rename

`changing_the_password_refuses_the_other_devices_session`
→ **`changing_the_password_refuses_a_credential_issued_before_it`**

`§2` has the measurement. The test is correct and its assertions stay exactly
as they are; what changes is that its name stops claiming a *distinct* second
session, which the product cannot mint inside one second. The new name is also
the one that survives a later per-device `jti` unchanged.

**Nothing is unproven about `FR-AUTH-006` by this and nothing is wrong in your
code.** `ROADMAP.md`'s *Unscheduled work* records the finding; the `jti`
question is `RFC 024` `§4.5`'s deferred one and is not yours to open here.

### §4.2 One missing assertion

`changing_the_password_works_and_the_acting_session_survives` asserts the
cookie the client **already had** still works. The requirement is stronger
than that: the response **re-issues** the session.

Assert that the response carries a session `Set-Cookie` and that its token
**differs** from the one the client sent. Without it the test passes against a
handler that sets no cookie at all — a check narrower than its name, which is
`§10.36`'s own class and the reason the name is in its table.

It is also what proves `§3`: a stale generation sets a cookie the next request
refuses, so this assertion plus the plant below is the fix's evidence.

### §4.3 One new plant

A storage test that stales the caller's read **on purpose**, so the
interleaving is real rather than timed:

1. read the user's `session_generation` → `G`;
2. call `users::advance_session_generation` (`users.rs:85` — the logout path,
   so this is the writer that actually races, not a fabricated one);
3. call `change_password`;
4. assert the returned value is **`G+2`**.

Watch it fail first: against the pre-fix arithmetic the handler's number is
`G+1`, and that single assertion is the whole defect. No sleep and no second
thread are needed, and a comment should say why — this is the first test in
this project to interleave two writers on `users`, and the next reader will
wonder what makes it deterministic.

## §5 — acceptance

1. **0 sites** compute a session generation from a value that was read
   rather than written — checked by grep, not by reading the diff.
2. The storage plant in `§4.3`, planted and watched fail.
3. The acting-session test asserts the re-issue (`§4.2`).
4. The rename landed (`§4.1`), and no other test name in the suite claims a
   distinguishable second session.
5. **`DEC-007` moves by exactly `+1`: 708 → 709**, counted with the runner
   per `.github/CONTRIBUTING.md`, never from the list above.
6. `cargo fmt --check`, `clippy -D warnings` on **both** feature sets, the
   42-command block, and `mdbook build` — exit codes read directly, not
   through a pipe's tail.

## §6 — what this unit does not do

No migration, no new column, no new table — `0025` stays reserved for
`FR-AUTH-007`. No `jti` and no change to `jwt::issue`'s signature or claims.
No new route, screen, copy string or `MessageKey`. **No change to
`FR-AUTH-006`'s normative sentence** — the sentence is right and the code is
what is wrong. No release leg: `DEC-056`'s four are the architect's, on the
owner's approval, and `0.72.0`'s cut is still unapproved.

## §7 — what the architect owes, and when

At the cut, not before:

- `FR-AUTH-006`'s `*Acceptance*` citation reworded to match `§4.1`, because
  it currently overstates what the product can exhibit (`§2`).
- `external-design.md`'s `SCR-22` amendment per `DEC-028` — a fifth section
  row and a state on that screen; **not** a new `§9` row and **not** a second
  use of `§5.3`'s session-notice flash.
- `§10.36`'s next instance, which this defect earns in a shape the register
  does not yet hold: **a requirement whose own `MUST` the code leaves a window
  in, found by reading the code and caught by no check at all** — not a check
  narrower than its name, but a guarantee narrower than its sentence.
- The candidate, the recommendation, and `DEC-056`'s four legs.
