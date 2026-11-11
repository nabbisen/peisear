# REL-0.62.0 — release candidate

**Status**: started 2026-11-12 on the owner's instruction to begin.
**The cut is not approved** — that is Release cycle **exit criterion 7**,
a separate decision from the authorization of this release's row in
*What ships next* (2026-11-10, re-authorized 2026-11-11). Nothing below
has been tagged, published or released.

**Series**: `060` — unit three of the accessibility sequence. Four
rounds, four commits (PRs #47–#50), each followed by the architect's own
closing commit; eighteen commits since the `0.61.0` tag.

---

## §0 — whose round this is: **split, and the split is the point**

The owner's instruction of 2026-11-12: *"You can ask dev team to help the
preps at least partially. To actually cut the release is not their role
but ours."*

- **The dev team's**: `PREP-001`, `PREP-002`, `PREP-003` below. All three
  are evidence-gathering with stated acceptance — regenerate, run, walk,
  and **report what was actually seen**.
- **The architect's**: the `DEC-028` sweep (§4), the claim (§2), the
  changelog's own wording and its dating, `What ships next`, and the
  release recommendation.
- **Nobody's until the owner approves**: the four legs (§9). The dev team
  does not bump the version, does not write the dated heading, does not
  tag, publish or create a Release — not as a step, not in passing, and
  not even as a dry run.

## §1 — the exit criteria, and where each one is answered

| # | Criterion | Where |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff | **Met.** `VIEW-001`, `VIEW-001 r2`, `VIEW-003`, `NAME-001` — PRs #47–#50, each reviewed before or on landing, each closed in this series' own README |
| 2 | `fmt`, `clippy`, the full suite, **with captured logs** | **`PREP-002`** |
| 3 | Requirements amendments applied | **Met already, not at the tag**: `FR-NAV-001` names `Me`; `NFR-A11Y-012` is Met for all five; `FR-NAV-006` and `external-design §5.10` state the section-order rule once; `§10.55` and `§10.56` are filed |
| 4 | `CHANGELOG.md` records the *why* | §4 — architect's. The heading is renamed and dated **on the day the tag is pushed**, not today (`0.60.0` is where those two days first diverged) |
| 5 | Affected documentation updated | §4 pass 5 **and `PREP-001`** |
| 6 | Real binaries, and an upgrade that is a non-event | **`PREP-003`** |
| 7 | Release recommendation; **owner approves** | **blocked**: `FOCUS-001`, §4a |
| 8 | *What ships next* reconciled | after approval, with the tag |

## §2 — the claim

**A project screen opens on your work. The analysis about it is one line
until you ask. An undo appears once, where you acted. And the word for
your own page is the word the page answers to.**

Before this release, a reader opening a project's list view on a phone
saw **no issue at all** without scrolling — the first row began at 89 %
of the viewport, behind nineteen controls — moving an issue said so
twice, ~700 px apart, with **Undo** only on the far copy, and the word
`Today` led to a page called *My dashboard*.

## §3 — `DEC-055`: no archive is due

`CHANGELOG.md` holds the series of the workspace version. `0.62.0` is in
the same `0.60–0.69` series `0.60.0` opened, and that series' predecessor
archive was written at that cut. **Nothing moves this time.**
`changelog_scan` is green; stated because *"no archive is due"* is a
claim, not an absence.

## §4 — the architect's own half, in progress

### §4a — pass 1 found a P1 regression, and the cut waits for it

**Pass 1 is not a formality and this is why it exists.** `FR-DM-006`'s
own text still specifies a five-second window this release deleted, and
following that thread into the shipped scripts found something worse:
the undo offer ends with `region.textContent = ""`, which removes the
**focused** Undo button. Measured with real key events on the
issue-detail surface — Tab reaches Undo in three presses, Enter
activates it, `document.activeElement` is **`BODY`** immediately and
after the request settles. `NFR-A11Y-002` (**P1**) says focus lands on
`body` on no measured path, and counts the undo's own endings among its
eight sites.

**The cause is this architect's own instruction** in `VIEW-003`'s
handoff, and the review that accepted the round — also his — checked
that Tab *reaches* Undo and never pressed it. `FOCUS-001` is the repair;
`§10.57` is the class; `NFR-A11Y-002` records the divergence, dated and
bounded to the one surface actually measured.

**Nothing is released.** The cut waits for `FOCUS-001` to land and be
reviewed.

### §4b — the rest of the architect's half

`FR-DM-006` still describes the five-second window, its pause/resume
pair and a toast position; its acceptance still cites
`undo_toast_attachment_scan` **(12)**, which is 4. Both are pass 1's and
are amended with `FOCUS-001`'s own landing, so the entry is rewritten
once against the mechanism that actually ships rather than twice.

`DEC-028`'s five passes over both specifications and the reader-facing
documentation; the `[Unreleased]` section's own wording — including the
correction `VIEW-003`'s review named, that *"right next to what it
undoes"* is generous at desktop width (measured **dy 23 px, dx 384 px**:
the same row, at its far end); both Appendix C baseline rows and both
*Covers release* headers; and the recommendation itself. **Not started at
the time this file was written**, and this line is updated when it is.

---

# The preparation rounds

Three, independent of each other, in any order. Each reports what it
**saw**, not what it expected to see.

## `PREP-001` — the images the documentation ships

`docs/src/assets/screenshots/` holds **seven** PNGs, published in the
book and in `README.md`. They are regenerated by
`node browser-checks/doc-screenshots.mjs`.

**They are stale, and two of the three ways they are stale are this
release's own headline.** The architect opened **two** of the seven:

- `today.png` — the navigation reads **Today**; the page's own title
  reads **My dashboard**. Both were changed by `NAME-001`.
- `list.png` — the navigation reads **Today**, the breadcrumb root reads
  **Today**, and the **health block is fully expanded** above the filters
  and the table, with `Indicators` and the workload chips visible. That
  is the arrangement `VIEW-001` and its round 2 replaced.

**The other five were not opened.** That is the bound on what is known,
and closing it is this round's first job.

**What to do**: regenerate all seven, then **look at every one** and say
in the report what each shows.

**Acceptance**:
- All seven regenerated, and **each one opened and described** — not
  "regenerated successfully".
- Every visible navigation word reads `Me`, every breadcrumb root reads
  `Me`.
- `board.png` and `list.png` show the health block and the workload strip
  **collapsed to one line each**, which is what this release is about.
  If either renders expanded, **stop and report it** — that would be a
  defect in the product, not in the screenshot.
- No image carries anything local: these are published artefacts, so the
  fixture's own data (`Demo Account`, `Website Launch`) is the only
  content that may appear. Check each image for a path, a hostname, a
  port or a real name before committing it.
- Image count unchanged at seven; none empty; `README.md`'s own image
  links still resolve (`docs_scan` rule 6 checks the last part).

## `PREP-002` — criterion 2, on `main`'s own tip

The criterion the release cycle itself names as *"the one that has
historically failed. It is a gate, not an aspiration."*

**What to do**, from `main` at its current tip, with output captured:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings`
- the full `DEC-007` block, **all 42 commands**, each one's own exit code
- `node browser-checks/overflow-gate.mjs`, `undo-mousedown-trap.mjs`,
  `drag-outcome-gate.mjs`
- `mdbook build docs`
- `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --workspace --no-deps --keep-going`

**Acceptance**:
- A table of command → exit code, with the **real** exit code of the
  command itself, never of a pipe's last stage.
- `DEC-007`'s total, re-derived from the runner. The architect's own
  figure is **670**; if yours differs, **say so rather than reconciling
  it** — a disagreement is information.
- Logs kept under `.git-exclude/`, not committed.
- If anything is red, that is the report. Do not re-run it green.

## `PREP-003` — an upgrade is a non-event, walked rather than reasoned

**No migration is added by this release** —
`git diff 0.61.0..HEAD -- crates/peisear-storage/migrations/` is empty.
That is a diff, not a walk, and the walk is what this round does.

**What to do**: build the real `0.61.0` binary from its tag in an
isolated worktree with its own target directory; boot it on a fresh
database; register a user and create a project and an issue **through
the real HTTP forms**; stop it; boot the candidate on **the same
database file**; and assert the data, the session and the new wording.

**Acceptance**:
- The project and the issue survive; the `0.61.0`-issued session cookie
  still authenticates.
- `/today`'s own `<title>` reads `Me — peisear` on the candidate and
  `My dashboard — peisear` on `0.61.0` — the release's own user-visible
  change, proven across the upgrade rather than in a unit test.
- Refuse to start if anything is already listening on the port. A stale
  server serves an old binary and reads a fix as absent; that has cost
  this project a round before.
- The worktree and its target directory **removed** when the round ends,
  and `df` quoted rather than `du`.

## §5 — what this release does not fix, stated not implied

Carried verbatim into the changelog's own section of that name: the ten
ungoverned `<details>`; the typography and colour floor, whose condition
is now met and which has no row yet; **`§10.55`** — at 768–800 px the
account menu's box covers the search field's right edge, halved by this
release and not removed; `§10.54`'s browser-gate repair; `§10.15`,
`§10.50`, `§10.32`, carried.

## §6 — the legs, and what happens only after approval

`DEC-056`'s four, in order, **none of them started, and none of them the
dev team's**:

1. the version bump to `0.62.0` and the `[0.62.0] — <tag day>` heading,
   in one commit;
2. a signed tag, pushed;
3. `cargo publish --workspace` from a detached worktree;
4. `gh release create 0.62.0 --notes-file --verify-tag` — which is what
   starts the platform archives, since `release.yml` fires on the release
   event and not on the tag.

## §7 — escalate rather than deciding

If a prep round finds something that looks like a product defect — an
expanded block in a regenerated screenshot, a red gate, a lost row across
the upgrade — **report it and stop**. Do not fix it inside a preparation
round: a candidate whose evidence was gathered and repaired in the same
pass cannot be read.

## §8 — exit conditions

- Three preparation reports, each naming what was seen.
- No gate re-run to make it green.
- No version bumped, no heading dated, no tag, no publish, no Release.
- Scratch target directories removed; `df` quoted.
