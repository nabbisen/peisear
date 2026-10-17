# REL-0.54.0 — release candidate

**Target**: 0.54.0, *the dialect seam*. `RFC 018` P1 + P2, plus
`docs_scan` rule 7. Three rounds, all closed 2026-10-18.

**What is new about this release, and it decides most of what follows**:
0.54.0 is **the first release since 0.53.0 shipped installable
artefacts**, and it carries **a new migration**. Every 0.53.0 install
has a database; `0022` is the first thing this project has ever asked
an existing one to apply. **That upgrade is the release's real gate**,
not the tag.

---

## §0 — already done; reconcile, do not redo

| round | what it left for the cut |
|---|---|
| `DIA-001` | `0015`/`0016` byte-identical to `0.53.0`; `0022` adds coded triggers; the guard reads live triggers |
| `DIA-002` | `postgres` feature, off by default, **published for the first time**; `dialect` module |
| `DIA-003` | `docs_scan` rule 7 |

`DEC-007` has moved **486 → 500** across the release. The three rounds'
own figures are reconciled; **measure it fresh anyway.**

## §1 — the ordinary cut

- **Version**: `0.53.0` → `0.54.0`. Both `release.yml` and `image.yml`
  compare this against the tag (`FIV-002` round 2) and **fire for real
  on `release: published`** — 0.53.0 was the first release where that
  happened and it worked; it is no longer novel, but it is still the
  thing a missed bump breaks.
- `CHANGELOG.md`: `## [Unreleased]` → `## [0.54.0] — <the date you
  actually cut it>`. **0.53.0's section carried a date two days before
  its release** because the candidate wrote it and the release slipped;
  write this one at the cut, not before.
- `ROADMAP.md`: the 0.54.0 row leaves **when the release is cut**, not
  when the tag is written — `0.53.0`'s own note records why that
  distinction is in this document.

## §2 — the upgrade, which is this release's falsification

**Walk it with real binaries, not a test harness.** `DIA-001` round 2's
unit test proves the migration chain applies; this proves a **user's**
database survives.

1. `cargo install peisear --version 0.53.0` into a scratch root.
2. Run it from an empty directory; register an account; confirm
   `/projects` authenticated. **That database is the artefact under
   test.**
3. Build the candidate and run it against **that same database**.
4. **It must start**, apply `0022`, and the original session's cookie
   must still authenticate — the discriminating check, not "it
   started": a key file or schema mishandled shows up as a `401`, not
   a crash.
5. Report the startup lines verbatim.

**If it fails, that is a stop**, not a finding to carry. 0.53.0's own
leg 3 is the precedent for what a late discovery costs.

## §3 — the feature that publishes for the first time

`peisear-storage` ships a `postgres` feature to crates.io in this
release. **Nothing in the cut changes it** — `DIA-002` round 2 put the
caveat in `lib.rs`'s first paragraph, which is what a `docs.rs` reader
meets.

**Confirm two things and change neither:**

- `cargo publish --workspace --dry-run` green (it is a CI job now, so
  say you read the job's result as well as running it).
- The crate-root paragraph still says `postgres` has no implementation.
  **A release is exactly when a caveat gets tidied away by someone
  improving the prose.**

## §4 — `DEC-028`: both specifications, five passes

- **Pass 5 greps for the instruction, not the document** — 0.53.0
  caveated one of three files carrying the same sentence.
- **Appendix A recomputed from its own rows**, never adjusted by the
  delta. It currently reads `69 | 56 | 5 | 0 | 2 | 6`; check every
  row's cells sum to its own total after your amendments.
- **`NFR-CMP-003` is not restated.** `RFC 018` puts that at P4. Say you
  left it and why.
- **`§11`/Appendix C's 0.54.0 baseline row** names what shipped and
  what it does not prove — the `postgres` feature has no
  implementation behind it, and that belongs in the row.

## §5 — the gate table, re-measured against the candidate commit

- `DEC-007` from a fresh block, three consecutive runs.
- `fmt`, `clippy` **both profiles and with `--features postgres`**,
  `rustdoc` **after the last edit**, `mdbook` + linkcheck.
- **`cargo publish --workspace --dry-run`** — the gate that stopped
  0.53.0 at leg 3.
- **The three browser gates.**
- **Both feature configurations compile**: default, and
  `--no-default-features --features postgres`.

## §6 — what is already `Implemented` and must stay honest

`NFR-REL-009`, `NFR-REL-010` and `NFR-REL-011` all closed at 0.53.0 on
artefacts existing. **They are not re-earned by this candidate** — but
after leg 4 the archives and the image must appear again, and the
architect verifies that. **If a workflow fails on this release, those
entries move back**, and the candidate should say that is the rule
rather than assuming green.

## §7 — escalate rather than deciding

- **If the §2 upgrade fails.** Stop.
- **If a gate moves and you cannot account for the delta.**
- **If the version bump breaks a workflow's tag comparison.** That is
  leg 4 already past; report, do not re-cut.

## §8 — exit condition

Version bumped and the changelog section dated at the cut; **the §2
upgrade walked with real binaries and reported verbatim**; both
specifications amended under `DEC-028` with Appendix A recomputed from
its rows and `NFR-CMP-003` deliberately untouched; Appendix C's 0.54.0
row naming what the release does not prove; `ROADMAP.md` reconciled;
every §5 gate re-measured against the candidate commit; the
crate-root `postgres` caveat confirmed intact; and a recommendation to
the architect — **not a tag**. The four legs are the architect's, on
the owner's approval.
