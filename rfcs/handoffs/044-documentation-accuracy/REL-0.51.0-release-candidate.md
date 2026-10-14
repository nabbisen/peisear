# REL-0.51.0 — release candidate

**Contents**: `DOC-001` through `DOC-007`, and `RFC 019` accepted.
**Every handoff reviewed and closed.**

**`DEC-028` is this round's work** — neither specification covers `0.51.0`
yet — **and it carries three amendments beyond the baseline rows**, §2.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## §1 — the ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only. **`0.50.0` → `0.51.0`.**

**No migration** — `0021` remains the most recent, **the second release
running**.

**Expected: 442**, from 431. Eleven tests: rule 1b's four and rule 2's one
(`DOC-004`), rule 5's three (`DOC-002`), the needle's two (`DOC-007`), and
`alignment_scan`'s two (`DOC-005`) — **minus** `DOC-002`'s three removed
git-dependent tests and `DOC-001`'s two deleted deferral tests, plus
`DOC-001`'s three plants. **State the arithmetic yourself; mine is from the
packages, not from a run.**

**Production source changed widely but shallowly.** `DOC-007` moved nine
files' test modules to `src/<mod>/tests.rs`, which is why the diff names
thirty-odd paths. **The behaviour changes are three:**

| Change | Where | Felt by |
|---|---|---|
| `grow_text()` and 41 sites | `components.rs`, eleven component files | **Yes — a label that sat above its neighbour now sits beside it** |
| Backlog row truncation | `sprint_plan.rs:413` | **Yes — the plan page no longer overflows at 320/390 px** |
| Six `card`/`block` links reverted to `grow()` | five files | No, by construction |

## §2 — `DEC-028`: the baseline rows, and three amendments

1. **`NFR-CMP-003`'s qualifier.** It reads *"a single SQLite file **so that
   backup is a file copy**"* — which is what both of this release's looser
   backup claims were restating, and `backup.md` has a section titled
   *"The wrong way"* forbidding `cp` on a running instance. **Say what is
   true: the file is the unit, and the copy is taken with `.backup` while
   it runs or `cp` once it is stopped.**
2. **`updated_at_authority_scan`'s own limit, in `NFR-CONC-003`.** The
   widened needle matches `SET updated_at =` — **only when `updated_at` is
   the first column after `SET`**, and this codebase writes multi-column
   updates (`issues.rs:496`, `user_capacities.rs:476`, `teams.rs:312`).
   **No live violation** — searched. **A guard whose coverage is narrower
   than the requirement it cites should say so in the requirement.**
3. **`§10.37` records what `DOC-007` settled and what it did not.**
   Kind 1 moved; **kind 2 — whether a module with no implementation is a
   violation at all — is open against the owner's own
   `project-instructions-rust.md`, which the requirement restates.**
   **Leave `NFR-MNT-005` at `Divergent`.** The repository's copy must not
   widen past its source, and that question is the owner's.

## §3 — the changelog

**This release's subject is documentation accuracy — and it contains two
things that are not documentation, because I scoped them in. Say so.**

1. **Lead with the two things a user sees.** A label that sat above its
   neighbour now sits beside it — the 44 px touch target was being applied
   without centring the content inside it, at 41 sites. And **the sprint
   plan no longer overflows on a phone**: a backlog row's project name had
   no truncation, latent for as long as the panel was empty.
2. **The front door describes the product.** `README.md` said *"issue
   tracking"*; the requirements say the aim is **not** *"getting tasks
   finished"* but keeping a project — and the people in it — sustainable.
   **The hero, overview and features now say that**, and the backup line
   no longer teaches what the operations guide forbids.
3. **The documents that describe the code now agree with it.** Three
   documents named six crates where there are seven; a deployment manifest
   listed two of eight shipped files; nineteen statements deferred to a
   private directory; **and nine decisions a requirement cites were stated
   nowhere at all.** All closed.
4. **And the guards grew to match.** Rule 1b checks the crate *count* and
   not only the names, rule 2 reads `CONTRIBUTING.md` too, rule 5 is
   narrowed to documents and switched on, and a new `alignment_scan`
   caught six misclassified sites on the day it was written.
5. **Say what this release does not fix**: `strip_line_comments` is
   copy-pasted into six scans and **five carry a bug** that corrupts their
   reading of five component files; the authority needle sees only the
   first column after `SET`; and `NFR-MNT-005`'s kind-2 question is open.

**Two things the notes must not claim**: that this was a documentation-only
release — beat 1 is a UI fix and a layout fix — and that the guards now
cover what they name; beat 5 says where they do not.

## §4 — verification before the candidate commit

- **`DEC-028`**: both specifications to `0.51.0`, a baseline row each, **the
  five-pass sweep reported pass by pass**, and §2's three amendments.
- `DEC-007` **442**, three consecutive runs, from a block extracted fresh
  out of the extracted tree's own `.github/CONTRIBUTING.md` — **which this
  release edited**, so the extraction earns its keep here.
- **All three browser scripts on the extracted tree**, in the job's order.
  **The overflow gate is the one that matters**: `sprint_plan` now carries
  real content in both panels for the first time, and `grow_text()` changed
  41 elements' display. **120 cells.**
- **`mdbook build` from the extracted tarball**, with **four** screenshots
  confirmed present in the output.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, **eight**
  private-link warnings, zero broken.
- **The database check**: build the `0.50.0` binary **from its tag's own
  commit, `f5323f6`** (`git rev-parse 0.50.0` returns the annotated tag
  object; use `0.50.0^{commit}`). Create a database, confirm **21**; boot
  the candidate against it and confirm **21 with nothing applied** — the
  first release since `0021` landed.
- `cargo publish --workspace --dry-run` from a **detached worktree of the
  candidate commit** — seven crates, every `warning:` line read.
- **A behavioural check, on the artefact**: at **320 px**, open a sprint
  plan with a populated backlog and confirm it does not overflow; and on
  any project page confirm *"(set your capacity)"* sits **beside**
  `WORKLOAD`, not above it.

## §5 — escalate rather than deciding

- **If the count is not 442**, or the overflow gate is not 120.
- **If any `DEC-007` run flakes.** `DOC-004` found one and traced it to
  `/tmp`; the harness still writes 33 databases there by measured choice.
  **Say so if it recurs**, with the disk state at the time.
- **If the five-pass sweep turns up anything** — this release rewrote the
  README, four operations and architecture documents, and nine requirement
  entries.
- **If the behavioural check at 320 px still overflows.**

## §6 — exit condition

A candidate commit on `main` at **0.51.0**; both specifications amended
with the baseline rows and §2's three amendments, and the sweep reported
pass by pass; a changelog that leads with the two visible fixes, says the
release is not documentation-only, and names what it leaves undone;
**442** green three times with any flake named; three browser scripts
clean at **120 cells**; the book building from the tarball with its four
images; a `0.50.0` database coming up at **21** with nothing applied; a dry
run of seven crates; and both behavioural checks confirmed on the
artefact. Then stop.
