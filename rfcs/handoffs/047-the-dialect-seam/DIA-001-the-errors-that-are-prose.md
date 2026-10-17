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
