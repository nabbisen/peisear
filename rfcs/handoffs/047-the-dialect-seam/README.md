# Handoffs — the dialect seam

**0.54.0.** `RFC 018` **P1 + P2**, plus one rule carried from 0.53.0.
One subject: **the storage layer stops being bound to SQLite by prose
and by a type alias** — without a line of PostgreSQL being written.

`RFC 018` is accepted (owner, 2026-10-11). Its §6 states the order and
each phase's gate; **which release a phase lands in is `ROADMAP.md`'s
to say**, and that table has 0.54.0 authorized since 2026-10-14.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| DIA-001 | [the errors that are prose](./DIA-001-the-errors-that-are-prose.md) | **P1, `RFC 018` §4.** `translate_trigger_error` matches `sqlx::Error`'s rendered string against five needles, **two of them prefixes of the real message**. A reworded trigger is a silently broken validation path. Codes raised by the database, matched on the code, with a both-directions guard between the migrations and the translation | — | **Closed** — 3 rounds |
| DIA-002 | [the seam](./DIA-002-the-seam.md) | **P2.** A `postgres` feature off by default, a dialect module, `compile_error!` guards. **No PostgreSQL code** — this builds the place a backend would go and proves the place is empty. The list of differences *deliberately left unabstracted* is the round's real output | DIA-001 | **Closed** — 2 rounds |
| DIA-003 | [the example file nobody reads](./DIA-003-the-example-file-nobody-reads.md) | **Carried from 0.53.0.** `.env.example` shipped two defects at once because nothing in this project reads it. `docs_scan` rule 7: every uncommented `KEY=value` must match `config.rs`'s own default | — | **Closed** — 1 round |
| REL-0.54.0 | the release candidate | The cut, `DEC-028`, and `DEC-056`'s four legs. Written when the three rounds close | all three | **Accepted** — awaiting the owner’s approval of the four legs |

**Status lives in the table above, not in a message.** Updated when a
round closes, so whoever picks the next one up reads it here.

---

## Why P1 is first, and it is not a preference

`DIA-002`'s dialect module owns the error translation, among other
things. Building the seam around prose matching means building it
twice. **`DIA-001` decides what the seam carries; `DIA-002` carries
it.**

`DIA-003` touches neither — a different crate and a different
subject.

## What this series is not

**Not the PostgreSQL backend.** P3 ports the schema and re-implements
the triggers; P4 is the conformance suite and the operations story.
Neither is here, and a round that finds itself writing SQL for a
second backend has left its scope.

**Not a restatement of `NFR-CMP-003`.** The single-file-database
requirement is restated at **P4**, *"to say for the SQLite backend"*,
per the RFC. Two phases early it would be a claim about code that does
not exist.

**Not open to a fourth round.** 0.53.0 took seven rounds because two
were found while walking it, and the cost is on the record. Anything
found here goes to the architect and waits for its own release —
with one exception already named: a defect that **blocks the release**
is not a scope question, as `FIV-007` was not.

---

## What 0.53.0 left for whoever reads this first

- **`cargo publish --workspace --dry-run` is a CI job now**
  (`FIV-007`). It is the only check that builds a crate from its own
  package, and it caught a release-blocking defect at `DEC-056` leg 3
  — after the tag had been pushed. **Do not treat a green workspace
  build as evidence a crate can be published.**
- **Measure the figures in these handoffs rather than carrying them.**
  `RFC 018` says three call sites; there are five. It says 137
  runtime queries; there are 142. Both were right when written.
- **Edit a gate, run that gate** — including `browser-checks/*.mjs`,
  whose files are the source of a CI job.
