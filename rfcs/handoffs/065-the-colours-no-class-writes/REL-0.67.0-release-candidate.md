# `REL-0.67.0` — release candidate

**Recommendation: cut it.** Exit criteria 1–6 are met, measured at this tree
rather than carried from the rounds' reports. **Criterion 7 is the owner's**,
and none of `DEC-056`'s four legs runs until that approval.

Written 2026-11-23 against `main` at `2136f72` plus this cut's own `DEC-028`
sweep.

---

## 1. What this release is

**`0.66.0` taught the contrast check to read all four of the theme's colour
families. It reads only the classes a component writes.** Eighteen shipped
sites got their colour from the served stylesheets instead — a colour no
component class carries, and therefore one no guard could see:

- **seventeen form placeholders at 2.54:1**, set by `input::placeholder` in
  *both* served sheets;
- **the account menu's own e-mail line at 1.85:1** — the worst text in the
  product — where `.menu-title`'s `/40` multiplied with that span's own
  `opacity-70`.

**Eight table headers were repaired alongside them**, at **4.53:1** — a
*pass* by 0.03, on the `/60` tier `NFR-A11Y-005` itself rejected when it
chose the floor, and one that fails the moment a table renders on the page
body. **The row and the series index first said all three families failed;
`PLC-002` measured otherwise and the overstatement was mine.**

Every repair **removes or re-floors a colour; none adds one**, and the theme
is untouched. `contrast_scan` now carries **three mechanisms** — the `/70`
muted floor, `§10.61`'s semantic-family ban, and this release's sheet-leak
guard — each closing a gap the one before it could not see.

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff, in dependency order | **Met** — `2be1002`, `4d4ed28`, `6f82875`, each reviewed and accepted, each its own commit |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, **with captured logs** | **Met** — §3 |
| 3 | Requirements amendments applied; nothing `Divergent` without a gap entry | **Met** — `NFR-A11Y-005` amended, `§10.62` opened and closed, zero `Divergent` statuses |
| 4 | `CHANGELOG.md` records the *why* | **Met** — one `Fixed` family across three rounds, stating **eighteen**, not twenty-six |
| 5 | Affected documentation updated in the same release | **Met**, checked rather than assumed — §5 |
| 6 | Tarball built and extraction verified | **Met** — §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — in the release commit, with the `0.67.0` row removed |

---

## 3. Criterion 2, measured at this tree

```
commands (+++ markers):   42   (expected 42)
non-zero exits:            0
tests passed (summed):   691
tests failed (summed):     0
```

**`DEC-007`: 689 → 691**, the two new guard tests. Log: 1,819 lines.

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres` | exit **0** |
| `cargo build --release -p peisear` | exit **0** |
| CI on `6f82875` (the last round) | **50/50 jobs, 0 failed** |
| CI on `2136f72` (the series closing) | **50/50 jobs, 0 failed** |

**Both of the new guard's plants re-run at this candidate**, in a throwaway
worktree, rather than taken from the round's report:

- a `<p class="stat-title">` added to a component → **FAILED**, naming
  `.stat-title -> oklch(var(--bc)/.6)`;
- `app.css`'s placeholder override removed → **FAILED**, naming both
  acknowledged rules whose `beaten_by_needle` had vanished.

Baseline 5 passed each time; tree clean afterwards.

---

## 4. Criterion 6 — the archive, extracted and walked

```
archive: peisear-0.66.0-x86_64-unknown-linux-gnu.tar.gz   8,396,658 bytes
tar tzf → peisear      ← one member, at the root (NFR-REL-002)
```

The archive says `0.66.0` because `Cargo.toml` still does — the bump is held
for the release commit (§7), and `release.yml:113` refuses to attach an
archive whose version does not match the tag.

**Walked the extracted binary**, and this walk checks the repairs in the
artefact rather than the pages that carry them:

| step | result |
|---|---|
| unattended start | served with defaults, `/health` **200**, bound to `127.0.0.1` |
| register → `/projects` | authenticated **200** |
| the served `app.css` | carries `.input::placeholder`, **the `-moz-` variants**, and the `thead` rule — and **zero** occurrences of the stale *"play CDN"* line |
| the account menu, served | `opacity-70` **0**, `text-base-content/70` **1** |
| shutdown | stopped by pid; **0 listeners** |

**What this walk does not cover, stated rather than implied.** It verifies
that the *served stylesheet* carries the rules and that the repaired markup
reaches the page; it does **not** re-measure a rendered colour — that was
`PLC-001`'s and `PLC-002`'s work, with both cascades planted, and this
container cannot render `-moz-` at all. One backend, one platform, one user,
as always.

---

## 5. Criterion 5 and the `DEC-028` sweep

Both specifications are at `0.67.0`. `requirements.md` records the
amendment and `§10.62`; its `Amended` field already moved to
`2026-11-23 (0.67.0)` **in the round itself**, with the prior entry kept.

**`external-design.md` is unamended, and the check names what it read**: its
only live mentions of a placeholder are `§3`'s design note — *a
`placeholder` disappears the moment a reader types, so it was never the
label* — and the `SCR`-level rows requiring an explicit `<label>` rather
than placeholder-only. Both were re-read; neither is touched or
contradicted by darkening the hint text. The muted tier's `/60 → /70` move
appears in Appendix C's `0.28.0` row as **history**, not as a live
statement.

New `0.67.0 (this baseline)` rows in both Appendix C tables, **re-rendered
with the pinned `mdbook` to verify**: one table each, 17 and 11 rows, zero
stray pipe paragraphs, both new rows inside `<td>`.

---

## 6. What is not fixed

- The **21** remaining smallest-text sites, the **nine** `link link-hover`
  sites, the **fifteen** bare-`opacity` sites, the **`xs` control tier** —
  recorded candidates.
- **A browser-rendered contrast sweep**, the only check that resolves a
  cascade in every case — named with its cost by `PLC-003`, not built.
- **A Pages deployment wedged since 2026-11-18**, which GitHub refuses to
  cancel by any route this token has. The book's link check runs on every
  commit since `0.65.0` moved the lock; only publication waits.
- **`proc-macro-error2`**, still arriving via `syn_derive 0.2.0`.

---

## 7. What the release commit will contain, and why it is held

`Cargo.toml`'s bump, `Cargo.lock`, `CHANGELOG.md`'s `[Unreleased]` heading
becoming `## [0.67.0] — <cut date>`, and `ROADMAP.md`'s criterion-8
reconciliation, in one commit — `changelog_scan` derives the version from
`Cargo.toml` and **requires a dated section for it**.

---

## 8. The honest part

**The serious one is mine, and it is not a figure.** CI failed on
`1cea899` — my own commit handing `PLC-003` — and **I never armed a
watcher**, so `main` sat red until the next round found it. The cause was my
own handoff citing `§10.23`, a number `spec_scan` counts across `rfcs/`. That
is `§10.54`'s own lesson — *the release's author stopped reading CI* —
committed by the person who has cited that entry at others in four
consecutive series. Recorded in memory; the round's handling of it was
exactly right.

**And it happened a second time, on this candidate's own commit.** `d07c5ed`
turned CI red the same way — this document cites the same dangling register
number, for the same historical fact, moving that counted population
4 → 5. *(The number is deliberately not repeated in this paragraph: writing
it here is what moved the count to 6 on the fix commit, which is the
clearest demonstration available of why the recurrence is structural.)* **This time the
watcher was armed**, the failure was read from the job's own log, the
population was **re-derived by hand** (three historical citations plus my
two), and the constant moved with its reason. The recurrence is structural
and is now a recorded candidate: `docs_scan` rule 5 excludes
`rfcs/handoffs/` as *execution records*, and this count does not, so every
handoff that mentions one of four numbers reddens `main`.

**Five more of my figures or instructions were corrected by these rounds**:
the claim that all three families failed; a predicted 6.34:1 where three
methods settle on **6.42:1**; an instruction to measure a placeholder *per
background* when the control paints its own; an instruction to measure a
search box at 390 px where the responsive design hides it; and a nesting
read I got wrong while reviewing, where their reading was right.

**One question, and it is the only one this document asks:** cut `0.67.0`,
or hold it?
