# REL-0.38.0 — release candidate

**Contents**: `ORD-001` (`ff8fd12`, `6586b56`), `PLAN-003` (`e14c3e2`),
`ORD-002` (`c65c8b9`), `CAP-001` (`dd6221d`), `ORD-003` (`a509e81`, `3b529a6`),
plus the record work at `4b14fcd`, `c0ba02b`, `7497379`, `908c539`, `e3e9b40`,
`ec5e741`, `611dc14`. All five reviewed and closed.
**Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop.

**This is the only work in flight.** `RACE-001` and `RACE-002` are written and
target 0.39.0, and they do **not** open until this release is out — a commit
landing on `main` while the candidate is cut from its tip would either be
pulled into a release it was not reviewed for or move the tag under the cut.

---

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip. `Cargo.toml`,
`Cargo.lock`, `CHANGELOG.md` only.

**There IS a migration this time** — `0018_remove_issue_position.sql`, the
first since `0017` and the first `DROP COLUMN` this project has shipped. Say so
in the changelog; a reader who upgrades needs to know a column left.

**Expected: 296**, up from 0.37.0's 272. `DEC-007`'s block **and**
`.github/workflows/test.yml` both changed this release — two new test files
(`ordering.rs`, `capacity_atomicity.rs`) and their two CI jobs. That is worth a
line: the block has been stable for several releases.

**`Issue.position` is removed from `peisear-core`'s public API.** A breaking
change to a published crate, carried by the minor bump under 0.x. It goes in
the changelog explicitly, not left for a downstream build to discover.

## 2. The changelog

**This release changes what a lot of screens show, and almost none of it is a
feature.** Four orderings were wrong, one concurrency guarantee was not held,
and one planned feature was retired. The danger is the opposite of 0.37.0's:
that entry risked sounding like more happened than did, and this one risks
reading as a feature release.

Seven things.

1. **Lead with the retirement, because it is the decision, not the code.**
   **`FR-DM-001` is amended from five direct-manipulation surfaces to four, and
   is Met.** Issue-list reordering (D-5) will not be built. The product already
   answers *what do we do next* with priority bands, sprint membership and
   planned dates, and the sprint is the better answer — named, shared,
   time-boxed, on a page of its own. A manual order would have been a second
   answer with no name in the UI and no visible provenance, and RFC 0004's
   requirement 10 means it would have been per-row buttons on a phone, which do
   not order a long backlog. **RFC 0004 is closed at four of five.** Name the
   revisit trigger — it is in the requirement.

2. **What users will actually notice, as a short list and not a narrative.**
   Every one of these is a screen reading differently tomorrow:
   - the **issue list** no longer shows Done at the top — it is newest first;
   - each **board column** reads newest first, where it read oldest first;
   - the **sprint-plan backlog** reads urgent, high, medium, low — `high` was
     last, below `low`;
   - a **sprint's issues** read Open, In progress, Done — Done was first;
   - **lists of things created in the same second** (inbox, projects, sprints,
     search) read newest first instead of backwards;
   - `sort=priority`'s **tie order** changes, as a consequence of the storage
     order changing.

3. **Say what caused all four, once, instead of four times.** An `ORDER BY`
   whose terms did not determine an order: a **TEXT** column sorted
   alphabetically as though that meant severity or lifecycle, three times; and
   a **one-second timestamp** whose ties SQLite returns oldest-first, so every
   `DESC` ordering read backwards within a tie — 22 sites. `§10.29` has it.

4. **The capacity fix is the one with a wrong number rather than a wrong
   order**, and it deserves its own sentence. Three queries resolved *a user's
   current capacity* with `… DESC LIMIT 1` and a tie returned the **superseded**
   value. Underneath it, the check-then-write was not atomic: twelve concurrent
   saves stored six overlapping rows, in a file whose comment said that could
   not happen. `§10.30`.

5. **The honest paragraph, and do not soften it.** **The test suite found none
   of this.** No test referenced `position` at all; none pinned any of the four
   orders. They were found by reading code during a design review about
   something else, and then by following that thread — three of the four by the
   dev team, two of those by declining to follow an instruction of mine that
   was wrong. Three of the defects had shipped since `PLAN-001` or earlier and
   one since `0001`. **The suite grew from 272 to 296 and the value is in what
   those 24 will catch later**, not in anything they found.

6. **The specification is in the repository** (`DEC-020`, `c0ba02b`).
   `requirements.md` and `external-design.md` now live under
   `docs/specification/` with three superseded baselines under `history/`, and
   a README explaining the three classes of citation a reader will meet. The
   decision was open since 0.19.1. This is a real change for anyone reading the
   project and it should not be buried under the ordering work.

7. **What a reader should not conclude**, folded in:
   - **`§10.15` remains open**, and its entry was amended this release
     (`4b14fcd`): the gap is permanent, its size is not — 820 lines across 3
     files at 0.33.0, 1,663 across 5 at 0.37.0.
   - **`§10.17` remains open** by decision.
   - **Four more read-then-write races are known and scheduled for 0.39.0**
     (`RACE-001`, `RACE-002`) — including one that can leave a team with no
     administrator, and the optimistic lock itself. **Say this.** A release that
     fixes one instance of a shape and knows about four more should not imply
     the shape is closed.
   - The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

## 3. Verification before the candidate commit

- `DEC-007` **296**, three consecutive runs.
- **The overflow gate, 90/90.** It has not been run since `ORD-001`; no markup
  changed, but this is a release.
- `fmt`, `clippy --workspace --all-targets -D warnings`.
- **The migration on a database built by the released 0.37.0 binary** — the
  `ORD-001` shape, repeated on the final tree, since `0018` is what upgraders
  will run.
- `cargo publish --workspace --dry-run` from a detached worktree of the
  candidate commit.

## 4. Escalate rather than deciding

- **If the count is not 296**, or `DEC-007`'s block and `test.yml` disagree.
- **If the migration behaves differently on the final tree** than it did under
  `ORD-001`.
- **If `find_violations` flags the changelog** — report the phrase, do not
  reword around the check.
- **If anything in §2's list of six is not actually true of the built
  artefact.** Each is a claim about a rendered page; the ones with tests are
  covered, and `sort=priority`'s tie order is the one to look at by hand.

## 5. Exit condition

A candidate commit on `main`, the changelog written, 296 green three times, the
gate at 90/90, the migration exercised from a 0.37.0 database, and the dry run
clean. **Then stop and hand it back.**
