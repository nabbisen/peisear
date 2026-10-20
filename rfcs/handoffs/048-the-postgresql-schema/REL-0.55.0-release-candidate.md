# REL-0.55.0 — release candidate

**Target**: 0.55.0, *a second backend that runs*. `RFC 018` P3 plus
the queries, four rounds, all closed 2026-10-20.

**What is different about this release**: 0.54.0's novel risk was a
migration existing users had to apply. **0.55.0's is the opposite** —
the SQLite set has not changed at all, and what ships instead is a
second backend with real code behind a feature that was empty last
release. **Nothing an existing operator runs should move; everything
new is reachable only by choosing it.** Both halves need proving.

---

## §0 — already done; reconcile, do not redo

| round | what it left for the cut |
|---|---|
| `PG-001` | `connect`, pool options, a PostgreSQL service in CI, URL-scheme rejection both ways |
| `PG-002` | `migrations-postgres/`, the dialect-aware embed, the filename drift guard |
| `PG-003` | eleven triggers, `ERRCODE`, the shared validation list |
| `PG-004` | `dialect::sql`, six divergent constructs, `0023`'s `rowid` surrogate |

`DEC-007`: **511 → 514** across this release's own rounds (509 at the
start of `PG-003`). Measure it fresh.

## §1 — the ordinary cut

- **Version**: `0.54.0` → `0.55.0`.
- `CHANGELOG.md`: `## [Unreleased]` → `## [0.55.0] — <the date you
  cut it>`.
- **`ROADMAP.md`'s 0.55.0 row leaves, and the "Shipped" line lands,
  when the release is *cut* — not when this candidate is written.**
  This is now an **exit condition**, not a warning: 0.53.0's row said
  shipped while leg 3 was failing, and 0.54.0's candidate wrote the
  line before the legs ran and was saved only by their succeeding.
  **Leave both to the architect**, who runs the legs and knows when
  they finished.

## §2 — the claim to falsify: nothing an existing user runs has moved

`git diff 0.54.0..HEAD -- crates/peisear-storage/migrations/` is
**empty**, and the `postgres` feature is off by default. So an
existing SQLite operator upgrading should see **no migration, no new
behaviour, nothing**.

**That is a claim, so walk it**, the same shape 0.54.0's §2 used and
for the opposite reason:

1. `cargo install peisear --version 0.54.0` into a scratch root; run
   it from an empty directory; register; confirm `/projects`.
2. Run the candidate against **that same database**.
3. **Assert the migration count did not change** — read
   `_sqlx_migrations` directly, before and after. Still **22**.
4. The original session's cookie still authenticates.

**A migration appearing here would be the defect**, and it is exactly
what a stray file in `migrations/` would cause. Report both counts
verbatim.

## §3 — the feature that now has code behind it

`peisear-storage` published a `postgres` feature at 0.54.0 with
`unimplemented!()` behind it. **At 0.55.0 it has a real backend.** The
crate root already says so accurately — `PG-004` rewrote it — so this
round's job is to **check it still does** and fix what did not keep up:

- **`lib.rs:21`'s own next paragraph is stale**: *"The current
  implementation is backed by SQLite via `sqlx` … trait abstraction
  becomes useful once a second backend (PostgreSQL, per the roadmap)
  **is in flight**; until then it is speculative infrastructure."*
  A second backend is no longer *per the roadmap* — it is in the
  crate, described two paragraphs above. **The sentence contradicts
  its own neighbour.**
- **`289` appears in two published doc comments** (`lib.rs:10`,
  `dialect.rs:67`), both hedged *"roughly"*, when `PG-004` measured
  **284** at close and said why the five went. **`dialect.rs` is the
  file that already carries "the two should never be quoted
  interchangeably"** — about a different pair of numbers, for exactly
  this reason.

**Do not soften the "not yet proven" half.** The crate root says the
conformance suite does not exist and that some test helpers still
hardcode `sqlite://`. That is true and it is the sentence most likely
to be tidied away by someone improving the prose at release time.

## §4 — `DEC-028`: both specifications, five passes

- **Pass 1** amends from the code: anything claiming PostgreSQL has
  no implementation.
- **Pass 5 greps for the instruction, not the document** — `0.53.0`
  caveated one of three files carrying one sentence.
- **Appendix A recomputed from its own rows**, never by the delta.
- **`NFR-CMP-003` is still not restated.** `RFC 018` puts it at P4 and
  this release is not P4. Say you left it and why.
- **`§11`/Appendix C's 0.55.0 row** names what shipped **and what it
  does not prove**: no conformance suite, no two-backend run of the full
  storage and lock tests, `NFR-CMP-003` untouched.

## §5 — the gate table, re-measured against the candidate commit

- `DEC-007` from a fresh block, three runs, **feature absent**.
- `fmt`; `clippy` **both profiles and both feature configurations**;
  `rustdoc` **under both features**, after the last edit; `mdbook`.
- **`cargo publish --workspace --dry-run`** — and
  `cargo package -p peisear-storage --list` showing **both** migration
  directories, `0023` included.
- **The three browser gates.**
- **The CI PostgreSQL job**, read by job rather than by run.

## §6 — what this release does not prove, stated not implied

- **No conformance suite.** `RFC 018` P4 runs the storage and lock
  tests against both backends; this release runs a targeted set.
- **The postgres-feature `--lib` suite still fails outside that set**,
  because test helpers hardcode a `sqlite://` pool — **not because a
  production query is wrong.** That distinction belongs in the
  changelog, not only in a doc comment.
- **`NFR-CMP-003`** still describes a single-file database without
  qualification.

## §7 — escalate rather than deciding

- **If §2's walk shows any migration applied.** Stop.
- **If a gate moves and you cannot account for the delta.**
- **If the crate root's "not yet proven" half needs weakening** to
  stay accurate — that would mean something shipped that this
  candidate did not expect, and it is a stop.

## §8 — exit condition

Version bumped; changelog dated at the cut and naming what is not
proven; **`ROADMAP.md`'s row and shipped line left to the architect**;
§2's walk performed with both migration counts reported verbatim and
**22 before and after**; §3's two stale items fixed and the crate
root's unproven half intact; both specifications amended under
`DEC-028` with Appendix A recomputed from its rows and `NFR-CMP-003`
deliberately untouched; every §5 gate re-measured; and a
recommendation to the architect — **not a tag**.

---

# REL-0.55.0 round 2 — 0.54.0's release notes are gone

The candidate is accepted on every gate (see the review) and its §2
walk holds. **One blocker, and it is not in the code.**

## §9 — the changelog lost a released version

`CHANGELOG.md`'s headings now read **`[Unreleased]` → `[0.55.0]` →
`[0.53.0]`.** At the `0.54.0` tag they read `[Unreleased]` →
`[0.54.0]` → `[0.53.0]`.

**`b002284` — this candidate — removed `## [0.54.0] — 2026-10-18`**
(`git log -S`), and with it that section's entire body. The only
surviving mention of the version anywhere in the file is a passing
clause inside `0.55.0`'s own prose. **There is no archive file**; this
project has never had one.

**What is not damaged, and why.** The `0.54.0` tag's own message links
to `blob/0.54.0/CHANGELOG.md`, pinned to the tag — so a reader
following the released link still gets the real notes, and the GitHub
Release body carries its own copy. **The project's own
"pin the link to the tag, never to `main`" rule is what saved this**
(`docs/development/changelog-and-releases.md`). The damage is confined
to `main`'s canonical file — which is also the file that document
calls *"the only place release notes are written."*

**How §1 produced it**: the step is *"`## [Unreleased]` → `## [0.55.0]
— <date>`"*, and a rename of the heading **below** `[Unreleased]`
rather than an insertion above `[0.54.0]` consumes the previous
section. 0.54.0's own candidate did the same step correctly, so this
is a slip, not a misunderstanding.

**Restore it from the tag, not from memory**: `git show
0.54.0:CHANGELOG.md`, lifting the section between `[0.55.0]` and
`[0.53.0]` byte-for-byte. **Verify by checksum**, the way `DIA-001`
round 2 verified its own restoration — a release-notes section
restored approximately is worse than one restored visibly late.

## §10 — the guard that would have caught it, and did not exist

`changelog_scan` passed **8/8** on a file that had just lost a
release. It checks each section's own shape — heading form, ISO date,
slug, series — and **never asks which versions ought to have a
section at all.** `§10.36`'s question, on the document that is this
project's only record of what each release did.

**The check is cheap and exact**: every `X.Y.Z` tag in the repository
has a matching `## [X.Y.Z]` heading. Measured at this review, that
rule fails on exactly one tag today — `0.54.0` — and would have
failed the moment `b002284` was written.

- **Tags, not a hand-maintained list.** The tag is the fact that a
  release happened; a list would be a second place to forget.
- **It must survive an archive.** This project keeps *"the archive of
  older series"* as a named future (`DEC-055`), so when one exists the
  rule reads *"a section in `CHANGELOG.md` **or** in the archive"* —
  **write the rule so that extension is a widening, not a rewrite**,
  and say how.
- **Seen to fail before trusted**: with `0.54.0` restored, delete it
  again in a scratch copy, watch the rule go red, restore. A guard
  never seen to fail is `§10.36` twice over.

**If reading git tags from a test is not available** — the scan reads
files, not a repository — **say so and bring the shape you reached.**
A rule over `git tag` output captured at build time, or over the
versions `Cargo.lock`/the changelog itself already names, may be the
honest compromise; what is not acceptable is a hand-written list of
versions.

## §11 — exit condition

`## [0.54.0] — 2026-10-18`'s section restored **byte-for-byte from the
tag**, verified by checksum, sitting between `[0.55.0]` and
`[0.53.0]`; a scan rule asserting every release tag has a section,
written so an archive widens it rather than replaces it; that rule
**seen to fail** against a deleted section and then pass; `DEC-007`'s
delta accounted for; every gate re-run — and `§2`'s walk **not**
repeated, since nothing in this round touches a migration.
