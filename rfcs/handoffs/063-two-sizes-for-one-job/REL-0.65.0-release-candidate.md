# `REL-0.65.0` — release candidate

**Recommendation: cut it.** Exit criteria 1–6 are met, with the evidence
below measured at this tree rather than carried from the rounds' own
reports. **Criterion 7 is the owner's**, and nothing in `DEC-056`'s four
legs runs until that approval.

Candidate written 2026-11-21 against `main` at `b21a408` plus this cut's own
`DEC-028` sweep.

---

## 1. What this release is

**Two sizes for one job.** The product had two sizes for the same piece of
text — a form label, a field hint, a line of small print — and **the line
between them was not a design decision: it was which screens had been
read.** `0.64.0` read three screens and raised their small text; this
release reads the five it did not.

**30 of 47 in-scope sites move up one step across five screens; 17 are left
with a stated reason each; the census runs 68 → 38.** No requirement is
amended, none is added, and **`§10.60` stays closed** — the series was
repeatedly in a position to write the type rule that entry declined to
write, and each time decided the site in front of it instead. The two
patterns with no counterpart anywhere else in the product (eight stat
captions, five chart captions) were **decided as sets and recorded**, not
generalised into a floor.

Two guards changed shape rather than gaining exceptions: a browser gate
stopped quoting a presentation class to validate its own fixture and matches
a **named hook** instead, and `css_scan`'s allowlist-integrity test was
**widened** to a second kind of consumer, with the rename it exists to catch
**planted** to prove it.

One colour conversion was tried and **reverted on measurement**: inside
`alert-warning`, which supplies its own foreground token, the guarded
spelling is a hue change rather than a spelling change — `9.01:1` against
`6.61:1` — so the bare spelling ships and the exception is recorded at the
site.

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff in dependency order | **Met** — six round commits (`bb79b15`, `924b88f`, `a9130dd`, `1fa6226`, `ae6fcf0`, `58dd2be`, `b21a408`), each reviewed and accepted, each its own commit, in order |
| 2 | `cargo fmt --check`, `cargo clippy -D warnings`, full per-crate suite, **with captured logs** | **Met** — see §3 |
| 3 | Requirements amendments applied; nothing left `Divergent` without a compliance-gap entry | **Met** — no entry amended or added this release; **zero** `*Status*: **Divergent**` lines in `requirements.md`; the `DEC-028` sweep's own findings are in §5 and recorded as `§10.36` instance 8 |
| 4 | `CHANGELOG.md` records the *why*, not only the what | **Met** — four `[Unreleased]` entries, one per round, each saying what moved **and what was left and why**, including the reverted conversion and its measured reason |
| 5 | Affected documentation updated in the same release | **Met** — and checked rather than assumed: `external-design.md` **describes no text sizes at all** (`§5.9` governs order within a list, `§5.10` the order of a screen's sections), so there is nothing in it to amend; what it did need was the repair in §5 |
| 6 | Tarball built and extraction verified | **Met** — see §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — done in the release commit, with the `0.65.0` row removed per the rule that a shipped release's record is `CHANGELOG.md` and Appendix C |

---

## 3. Criterion 2, measured at this tree

**The `DEC-007` block exactly as `.github/CONTRIBUTING.md` writes it**, run
in a scratch target directory, every command's exit code written to the log
so the count is read from the artefact rather than from memory:

```
commands (+++ markers):   42   (expected 42)
non-zero exits:            0
"failures:" lines:         0
"panicked at" lines:       0
test result lines:        53
tests passed (summed):   687
tests failed (summed):     0
```

**`DEC-007`: 687, unchanged from `0.64.0`** — this release adds and removes
no test; the only test-file edits were two assertion values and one rename.
Log: 1,819 lines, `.git-exclude/tmp/cand065/dec007.log`.

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres -- -D warnings` | exit **0** |
| `cargo build --release -p peisear` | exit **0**, binary 23,925,704 bytes |
| CI on `b21a408` (the last round) | **50/50 jobs, 0 failed** — this is where the browser gates ran on the final tree |
| `Docs` on `26b6c13` | `mdbook build (with link check)` **success** — the first since 2026-11-18, see §6 |

**Counted from the right field.** The summing command initially reported
`0` tests passed because it added the wrong awk field — the word *passed*
rather than the number before it. Recounted from field 4: **687**. Noted
because this is the second time that slip has happened here, and the figure
is load-bearing.

---

## 4. Criterion 6 — the archive, extracted and walked

Built the way `release.yml` does it (`-C dist`, so files sit at the archive
root per `NFR-REL-002`):

```
archive: peisear-0.64.0-x86_64-unknown-linux-gnu.tar.gz   8,395,939 bytes
tar tzf → peisear          ← one member, at the root, no intermediate directory
extracted → 23,925,704 bytes
```

**The archive says `0.64.0` because `Cargo.toml` still does** — the version
bump is held for the release commit (§7). `release.yml:113` refuses to
attach an archive whose `Cargo.toml` version does not match the tag, so the
published archive will say `0.65.0` or the release will fail loudly.

**Walked the extracted binary**, not a debug build:

| step | result |
|---|---|
| unattended start | served with defaults, **no wizard** (`walk.log` has no *"No configuration found"*) |
| bind | `127.0.0.1:4199`, confirmed by `ss -ltnp` against the process's own pid |
| `/health` | **200** |
| `/login` — a screen this release changed | the raised centred line present (**1**), the old `text-xs` spelling **absent (0)** |
| `/register` (POST) | **303**, and the session it issues authenticates `/projects` → **200** |
| `/settings` — changed | the raised helper note present (**1**) |
| team detail — changed | the invite panel's raised qualifier present (**1**) |

**Every remaining `text-xs` on the walked pages is accounted for**, rather
than counted and left unexplained: one on `/settings` and three on team
detail — `layout.rs`'s account-menu email line (out of scope: navigation
chrome), plus `teams.rs`'s `(you)` marker and privacy footnote, both left
with a stated reason by `SIZE-002`. The `(you)` marker renders the converted
colour spelling, which is the one conversion that **did** hold.

Server stopped by pid; the port shows **0** listeners afterwards.

**What this walk does not cover, stated rather than implied**: one backend
(SQLite, not PostgreSQL), one platform (x86-64 Linux), one user, and no
sprint data — so the sprint list, sprint detail and sprint-plan screens were
**not** opened by this walk. Their sites were measured per-site by the
rounds and swept by the browser gates in CI on `b21a408`. The five platform
archives and the container image are built by CI on the tag, and that is
where their evidence comes from, as it did for `0.63.0` and `0.64.0`.

---

## 5. The `DEC-028` sweep, and what it found in this cut's own documents

The sweep is the reason this candidate took longer than its content
suggests. **It found three defects, all the architect's, none of which any
gate in this repository can see** — recorded as **`§10.36` instance 8** and
repaired here:

1. **`requirements.md`'s header described the wrong release.** It read
   `Covers release: 0.64.0` while its parenthetical described `0.63.0`'s
   three closures, named `0.62.0` as *"the release before it"*, and ended
   *"implementation through 0.63.0"*. The version moved at that cut; the
   body did not.
2. **Both documents' `Amended` field read `2026-11-01 (0.60.0)`** while
   `0.62.0` had amended `FR-DM-006`, closed `NFR-A11Y-012`'s last exception
   and added `external-design §5.10` — two releases stale, **in the very
   field `§10.36` instance 7 recorded *"so the next sweep checks whether it
   moved"***. Two sweeps did not check.
3. **In both documents, the three newest Appendix C rows were not rows.**
   `0.62.0`, `0.63.0` and `0.64.0` had each been prepended **above** the
   table's header and delimiter, so the pinned `mdbook 0.5.4` renders them
   as paragraph text with literal pipes — `<p>| <strong>0.64.0 (this
   baseline)</strong> | …` — in the published book, for three releases.
   **Measured by rendering the fragment, not reasoned from the CommonMark
   table rule.**

Repaired: both headers rewritten (and the `0.64.0` mis-description named in
the new header rather than quietly dropped), both `Amended` fields corrected
to the release that actually amended them, **nine rows relocated below their
delimiters**, and new `0.65.0 (this baseline)` rows added to both
appendices. **Re-rendered to verify**: one table per appendix, 15 and 9
rows, **zero** stray pipe paragraphs, and `0.65.0 (this baseline)` inside a
`<td>`.

---

## 6. What is not fixed, and is not fixed by this release

- **A GitHub Pages deployment wedged since 2026-11-18** (run
  `44281512009`). GitHub refuses to clear it by every route available —
  `cancel` says *"completed"*, force-cancel says *"not been queued yet"*, a
  deployment rejection says *"no pending deployment requests"* — so every
  later `Docs` run queues behind it. **Mitigated, not cured**: `26b6c13`
  moved the `pages` lock onto the deploy job, so the book's own link check
  runs again (**verified on that commit**) and a wedge now costs the
  deployment alone. The published book is unaffected: nothing under `docs/`
  changed between the last successful deploy and this candidate, apart from
  the sweep in §5, which will publish when the queue clears.
- **The 21 out-of-scope smallest-text sites**, the `xs` control tier (19
  `btn-xs`, 3 `select-xs`), the 54 `label-text text-sm` tokens redundant
  with their own component class, and the fifteen remaining bare-`opacity`
  sites — all recorded as candidates in `ROADMAP.md`, the last now carrying
  **two** conditions per site because of what `SIZE-003` measured.
- **`proc-macro-error2`** still reaches the build graph through
  `syn_derive 0.2.0`, which is its own latest version. Unscheduled, watched.

---

## 7. What the release commit will contain, and why it is held

Held deliberately, as `0.64.0` held it: **`Cargo.toml`'s version bump,
`Cargo.lock`, and `CHANGELOG.md`'s `[Unreleased]` heading becoming
`## [0.65.0] — <cut date>`** are one commit, because `changelog_scan`
derives the version from `Cargo.toml` and **requires a dated section for
it** — bumping either alone turns the suite red. `ROADMAP.md`'s criterion-8
reconciliation lands in the same commit.

Nothing above has been committed. `main` currently builds, tests and ships
as `0.64.0` + unreleased work, which is the honest state until the owner
approves.

---

## 8. The honest part

**Three of the series' six corrected reasons were mine**: a `24px → 32px`
cost that `min-h-11` made impossible; a precedent member (`issues.rs:649`)
that was not the shape I cited it as; and the *"the two colour spellings
composite identically"* generalization I carried for three rounds until a
round measured 2.4 points and reverted my own instruction. **All three of
this cut's sweep defects are also mine**, and one of them is the recurrence
of a class I recorded, named as the architect's lapse, eleven releases ago.

Every one was caught the same way: by someone re-deriving instead of
trusting the sentence in front of them. That is the arrangement working, and
it is the reason each handoff ends with *if a measurement contradicts this
file, the measurement wins*.

**One question, and it is the only one this document asks:** cut `0.65.0`,
or hold it?
