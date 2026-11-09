# REL-0.61.0 — release candidate

**Status**: assembled 2026-11-09 on the owner's authorization to build
the candidate. **The cut is not approved** — that is Release cycle
**exit criterion 7**, and it is a separate decision from the
authorization of this release's row in *What ships next* (2026-11-07).
Nothing below has been tagged, published or released.

**Series**: `054`, `056`, `057`, `058`, `059` — five handoff series,
**fourteen rounds**, one commit each (PRs #33–#46), plus `CSS-001`
applied directly by the architect and `REQ-005` closed as a non-round.

---

## §0 — whose round this is, and what it does not wait for

This is the architect's. Every round in it is closed and reviewed; no
dev-team turn is open. It does **not** wait for unit three of the
accessibility sequence (each view), which is named, unstarted, and has
no release number — the owner's own pacing, *"one by one with time we
need"*.

## §1 — the ordinary cut

- Criterion 1 — **met**: fourteen rounds, fourteen PR commits, in
  dependency order, each followed by the architect's own closing
  commit. `git log 0.60.0..HEAD` is the record.
- Criterion 2 — **§5**, with captured logs under `.git-exclude/`.
- Criterion 3 — **§4**.
- Criterion 4 — the `[Unreleased]` section is written for a reader and
  carries its own *What this release does not fix*. **The heading is
  renamed and dated on the day the tag is pushed**, not today:
  `0.60.0` is the release where the candidate day and the tag day first
  diverged and the section recorded the wrong one.
- Criterion 5 — **§4 pass 5**, and all six documentation screenshots
  regenerated and looked at.
- Criterion 6 — **§6**.
- Criterion 8 — after approval, with the tag.

## §2 — the claim

**The five destinations this product's specification names are visible,
labelled, and answer to the names they are called by — and an undo
stops counting down while a reader is reaching for it.**

Stated where a reader meets it: the navigation bar. Before this
release, four of the five sat behind a menu labelled with the reader's
own username, `Projects` was reachable only by clicking the logo, the
notification control was an unlabelled bell whose unread badge was
correct on fourteen of twenty-six pages, and `/inbox` greeted anyone
who clicked **Inbox** with a page titled *Notifications*.

**This is the first release since `0.46.0` whose subject is a screen a
user of this tracker looks at.**

## §3 — `DEC-055`: no archive is due

`CHANGELOG.md` holds the series of the workspace version, and `0.61.0`
is in the same `0.60–0.69` series `0.60.0` opened — the `0.50–0.59`
archive was written at that cut. **Nothing moves this time.**
`changelog_scan` enforces the rule and is green; stated here because
*"no archive is due"* is a claim, not an absence.

## §4 — `DEC-028`: both specifications, five passes

**Pass 1 — every entry the release touched.** `FR-NAV-001` (twice: met
in the markup 2026-11-04, met on the page 2026-11-07 — `§10.52`),
`FR-DM-006` (the undo window now stops while the reader is on it; the
entry's four `setTimeout` line numbers marked as the `0.44.0` snapshot
they are), `NFR-REL-007` and the twelve status corrections, already
carried. **`NFR-A11Y-012` is added** — a page title names the page and
the product, and a destination answers to the navigation's own word.

**Pass 2 — entries whose status cites a scope the release changed.**
Swept: `FR-NAV-002`/`003`, `FR-NTF-004`/`005`, `FR-SCH-001`,
`FR-DM-001`, `NFR-A11Y-002`/`004`/`005`/`007`. **Nothing found** — none
names where an affordance sits, and the ones that name counts were
re-derived in pass 4.

**Pass 3 — rule (e) by hand**, over the entries this release puts in
play. **Nothing found**: no stale-looking status cites a sibling that
is already clean.

**Pass 4 — anything that counts or summarises statuses.** Appendix A's
Accessibility row (11 → 12, 8 → 9) and the non-functional total
(70 → 71, 61 → 62) re-derived; `spec_scan`'s own population pin
169 → 170, which **went red before Appendix A's rule b did**, the order
`§10.36` asks for. `§9.1`'s test inventory is a dated `0.28.0` snapshot
and says so — correctly left.

**Pass 5 — the documents a reader outside this repository meets.** One
finding, and it is the largest in the sweep: `§10.53`.
`docs/operations/tailwind-local.md` described a product that stopped
existing at `0.32.0`, and **five documents repeated its claim**,
including `docs/security/hardening.md` telling an operator their CSP
still needs CDN origins. Page rewritten, five claims corrected, entry
opened and closed on sight.

**`external-design.md` has real content this release**, not a
"nothing changed" row: `§3.1`'s layout diagram and `§3.2`'s navigation
model both described a sidebar carrying Settings and omitting Search.
`§3.3`'s screen inventory needed no change — no screen was added,
removed or rerouted, checked rather than assumed.

## §5 — the gate table, re-measured against the candidate

All exit codes read from the real command, never from a pipeline's last
stage. Logs captured under `.git-exclude/tmp/u2/`.

| Gate | Result |
|---|---|
| `cargo fmt --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings` | exit **0** |
| **`DEC-007` block, three consecutive runs** | **42 commands each run, every recorded exit 0, 666 passed / 0 failed, three times** |
| `node browser-checks/overflow-gate.mjs` | exit **0** — 120/120 cells, 0 failing, **0 requests to any non-local origin** |
| `node browser-checks/undo-mousedown-trap.mjs` | exit **0** — all three draggable surfaces |
| `node browser-checks/drag-outcome-gate.mjs` | exit **0** — both surfaces, including `FLOW-001 r2`'s four pause/resume assertions |
| `mdbook build docs` (incl. linkcheck) | exit **0** |
| `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --workspace --no-deps --keep-going` | exit **0**, baseline warnings unchanged |
| `cargo publish --workspace --dry-run` (criterion 6) | CI job on every push; green at `2eb90e6`, and re-running on `42f2ef7` |

**`DEC-007`: 642 → 666**, the from measured at `NAV-001` round 3 and the
to measured three times here.

**A note on how this table was produced, because it nearly was not.**
Two versions of the block runner reported a confident
`total=0 failing_binaries=0` while being wrong — the first tallied
inside a subshell behind a pipe, so every increment was discarded; the
second died on `bc` not existing here, and the arithmetic error took the
loop out after one binary. **Both looked exactly like a clean run.** The
runner now only runs and logs, and a separate pass counts the log, so
the counter cannot impersonate the block. The first of the three runs
above was also discarded and re-done: it had been started while the
specification was still being edited, and caught a real dangling `§10.53`
citation mid-edit — which is why three *consecutive* runs are specified
against a frozen tree.

## §6 — the claims to falsify, with real binaries

1. **The navigation renders. Run against the candidate binary, at four
   widths:**

   | viewport | primary `<nav>` | links with size | phone menu |
   |---|---|---|---|
   | 390 | `none` | 0/4 | shown |
   | 768 | `flex` | 4/4 | hidden |
   | 1280 | `flex` | 4/4 | hidden |
   | 1920 | `flex` | 4/4 | hidden |

   This is the claim `§10.52` is about, and **the only instrument that
   can check it is a browser** — three rounds of render tests, a
   required prop and a compile-error plant all passed while the row was
   `display: none` at every width.
2. **An upgrade is a non-event — walked, not reasoned.** This release
   adds **no migration** (`git diff 0.60.0..HEAD -- crates/peisear-storage/migrations/`
   is empty), but the claim was run rather than read off the diff: a
   real `0.60.0` binary, built from the tag in a detached worktree,
   booted on a fresh database, registered a user and created a project
   and an issue through the real forms; then the candidate booted on
   **the same database file**:

   ```
   0.60.0  project page: title="Before The Upgrade — peisear"  has-new-nav-markup=false
   candidate                                                   has-new-nav-markup=true
     ok  the project survives          ok  the title names the product
     ok  the issue survives            ok  the primary nav markup is present
     ok  the session cookie still works  ok  the inbox answers to its own name
   ```

   *Two incidental findings from the walk, both mine.* The first attempt
   set `PEISEAR_SETUP=skip`, and **`0.60.0` refused to start, naming the
   variable and listing the two values it accepts** — this release's
   predecessor doing exactly what its own changelog claims. The second
   attempt failed with `409` on register because the *previous* run's
   server was still holding the port and `ready()` cannot tell a stale
   server from a fresh one; the walk now refuses to start if anything is
   already listening, which is this project's own
   stop-the-server-by-pid lesson, relearned.
3. **The documented install path still works.** `NFR-REL-011` is
   enforced by CI rather than by a person at a cut (there is no
   criterion 9), and the `Env Example Walk` job ran green on
   `2eb90e6`. Cited, not re-performed by hand.

## §7 — what this release does not fix, stated not implied

Carried verbatim into `CHANGELOG.md`'s own section of the same name:
`/today`'s name and its contents; where an outcome and its undo belong;
unit three, not started; `§10.15` (the shipped JavaScript is executed by
no test); `§10.50`; `§10.32`. And one this candidate adds: **nothing
makes a documentation page that describes a removed dependency fail
anything** — `§10.53`'s own closing note.

## §8 — the legs, and what happens only after approval

`DEC-056`'s four, in order, none of them started:

1. the version bump to `0.61.0` and the `[0.61.0] — <tag day>` heading,
   in one commit;
2. a signed tag, pushed;
3. `cargo publish --workspace` from a detached worktree;
4. `gh release create 0.61.0 --notes-file <the section> --verify-tag`.

**The version is still `0.60.0` in `Cargo.toml` and the section is
still `[Unreleased]`**, deliberately: `changelog_scan` requires the
workspace version to have a dated section, so the two move together,
and they move on the day the tag does.

## §9 — escalate rather than deciding

Two questions are the owner's and neither blocks this cut: whether
`/today` should carry the day's planned work above its metrics, and
whether an undo should stop living in a corner. Both are recorded in
`rfcs/handoffs/059-what-the-browser-actually-draws/README.md`.

## §10 — exit conditions

**Met.** §5's table is filled from real exit codes, §6's first two
claims have been run, and the third is cited from CI rather than
re-performed by hand. **The recommendation is issued: cut `0.61.0`.**
The owner approves or declines; nothing below criterion 7 has been
started, and the version in `Cargo.toml` is still `0.60.0`.
