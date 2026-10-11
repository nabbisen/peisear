# REG-001 — the check that examined nothing

**The last content round before 0.50.0's candidate.** One register entry,
three carried fixes — and the register entry's own subject is the reason all
three exist.

**Review**: `.git-exclude/reviewed/SEC-001-review.md` (§2, §3, §4).
**Target**: 0.50.0, fourth round.

---

## §1 — `§10.36`, the entry

Add to `requirements.md`'s `§10` register — a `DEC-028` amendment, so it
lands with this round rather than with the candidate:

> **`§10.36` — a check that cannot tell *nothing wrong* from *nothing
> examined*.**

**Adjacent to `§10.17` and not the same.** There, an assertion is wrong.
Here, the assertion is right and **its input is empty for a reason it never
asks about**, so it reports success having looked at nothing.

**Five instances, all measured, all from the last two releases** — write them
into the entry, because a class with no instances is a slogan:

| # | Instance | How it ended |
|---|---|---|
| 1 | **Rule 5 read "no git here" as "no citations."** `git -C <dir> ls-files` answers against an *enclosing* repository, so every extracted tree under a gitignored path returned zero | Found by running the release gate (`9dd7900`) |
| 2 | **Rule 6 passed by matching nothing** — a hardcoded URL prefix that would stop matching if the repository ever moved | Fixed at `11f7704` |
| 3 | **The self-path test passed vacuously after a rename** — an absent path is trivially absent | Fixed in `DOCS-008` §3 |
| 4 | **A redactor exercised only on input with nothing to redact** | Avoided in `SEC-001` §1, by instruction |
| 5 | **The round-trip test stopped catching truncation** while a flake was being fixed — both sides of the comparison lost the same digits | §2 below, **introduced after the class was named** |

**Instance 5 is the one worth stating plainly in the entry.** The class was
already written down and under review when a careful round reintroduced it
anyway, while fixing something else. **The rule is not "remember this";
it is a question to ask of a green check**: *what did it examine, and how
would I know if the answer were nothing?*

**Keep it short.** This register's value is that each entry is readable in
one sitting.

## §2 — put the truncation assertion back, deterministically

`crates/peisear-storage/src/pool.rs`'s round-trip test. `SEC-001` replaced
`timestamp_subsec_millis() != 0` — correctly, it is flaky — with
`updated_raw.contains('.')`, which **checks the stored text and leaves the
decode unguarded**. A decoder that truncates passes both halves, because
`round_trip(updated) == updated` compares two equally-truncated values.

**Two assertions, each proving one thing, neither flaky:**

1. **Keep** `contains('.')` on the **trigger-produced** text — it proves
   `0021`'s format. You already have this.
2. **Add a planted fraction.** `UPDATE issues SET updated_at = '…23.414'`
   **does not fire the trigger** — its `WHEN OLD.updated_at = NEW.updated_at`
   guard sees the values differ — so the planted value stands. Then assert
   the decoded `timestamp_subsec_millis()` is **414**.

**The clock stops being an input**, so there is nothing to retry and nothing
to coincide. **Confirm the trigger really does not fire** on that UPDATE
rather than assuming it — read the value back and check it is still `.414`.

## §3 — the same retry, one file over

`crates/peisear-web/tests/optimistic_lock.rs`'s
`issue_edited_twice_in_the_same_second_with_the_same_token_must_409` carries
the identical `.000` exposure `SEC-001` found: a whole-second seed token, one
legitimate edit, and no check that the token actually moved.

**Apply the same bounded retry** `optimistic_lock_guards.rs` now uses. It is
`STORE-001`'s test and `SEC-001` was right to leave it alone; **this round
has the scope.**

**Why it matters more than one-in-a-thousand sounds**: every candidate runs
`DEC-007` three times, every release. A flake at that rate fires during a
release verification sooner or later, and it fires as *"the lock is
broken"* — the most alarming possible false alarm, on the requirement this
release exists to protect.

## §4 — `backup.md`'s two words

From `SEC-001`'s review §3:

- **Drop the internal citation.** *"`doc-screenshots.mjs`'s own fixture
  override … declined in `DOCS-007`'s review"* means nothing to an operator.
  **Keep the reasoning** — a knob that can silently trade away a durability
  guarantee is worse than no knob — and cut the reference.
- **"write-amplification profile" reads backwards.** `NORMAL` does *fewer*
  fsyncs, which is the reason someone would want it. **"fewer-fsync
  profile."**

`hardening.md`'s requirement IDs stay — those resolve in the published book.

## §5 — gates

- **`DEC-007`**: §2 adds an assertion to an existing test and §3 changes one,
  so **expect 431 unchanged**. **If it moves, say why.**
- **Three consecutive runs**, and **say whether any of the three flaked** —
  this round's whole subject is checks that mislead, and a silent re-run
  would be the wrong instinct to practise here.
- **`DEC-028`**: §1 amends `requirements.md`. **Both specifications still
  read `Covers release: 0.49.0`** — the 0.50.0 baseline rows are the
  candidate's job, not this round's. Add the entry without touching the
  headers.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken), **`mdbook build`** (both documents
  changed).
- **No migration.** The candidate's database check stays at **21 with one
  applied**.

## §6 — escalate rather than deciding

- **If the planted `UPDATE` does fire the trigger**, so a fraction cannot be
  planted — then the `WHEN` guard does not mean what both of us read it to
  mean, and that is worth knowing before the candidate.
- **If §3's retry cannot be made to converge** within its bound.
- **If writing `§10.36` turns up a sixth instance.** Add it, and say where
  you found it.

## §7 — exit condition

`§10.36` written with its five instances; the truncation assertion back and
deterministic, with the trigger's non-firing confirmed rather than assumed;
`optimistic_lock.rs` retrying like its sibling; `backup.md` readable by the
operator it is for; `DEC-007` **431** three times with any flake named.

**Then 0.50.0's content is complete** and the candidate is next.
