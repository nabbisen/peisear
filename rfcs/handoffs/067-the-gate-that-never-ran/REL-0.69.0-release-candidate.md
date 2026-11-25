# `REL-0.69.0` — release candidate

**Recommendation: cut it.** Exit criteria 1–6 are met, measured at this tree.
**Criterion 7 is the owner's**, and none of `DEC-056`'s four legs runs until
that approval.

Written 2026-11-25 against `main` at `e96e51d` plus this cut's own `DEC-028`
sweep.

---

## 1. What this release is

**Two things that nothing had ever executed, and a rule that lived only in
Rust.**

- **A gate had never run.** `disclosure-marker-gate.mjs` checks that
  thirteen `<summary>` controls keep `display: list-item` and a ≥44×44
  target — properties `touch_target_scan` **cannot** see, because it reads
  which helper a `<summary>` was given and never the style the browser
  computes. `git log -S` finds **no commit that ever wired it**; it was
  cited as a required re-run by two source comments, five register
  passages and eight handoffs. **A human typing its name was the only thing
  that had ever run it.** It now runs in CI and **passed on the runner**.
- **Nothing ever asked the registry.** `image.yml` proved a push at the
  moment it ran; `NFR-REL-010`'s property is that the image is *publicly
  pullable*. There is now an **anonymous** post-publish check — no job
  credentials, because a credentialed call would pass on a private package
  and prove the wrong thing.
- **The register's reading rule was unwritten.** An entry's **last**
  `*Status*:` line governs — `spec_scan::governing_status_token`'s `rfind`,
  implemented for releases and **stated in no sentence of the document**.
  `§0.6` now states it, the three real multi-status entries mark their
  earlier line inline, and a `spec_scan` rule fails when a fourth lands
  unmarked.

**And the release's own record carries the reason the last two existed: me.**
§8.

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff, in dependency order | **Met** — `29f6f7f`, `4c6a7ab`, `e96e51d` |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, with captured logs | **Met** — §3 |
| 3 | Requirements amendments applied; nothing `Divergent` without a gap entry | **Met** — `§0.6` amended, `§10.64` opened and closed, Appendix A unchanged and **correct as it already was** (§8) |
| 4 | `CHANGELOG.md` records the *why* | **Met** — three `Fixed` bullets, each about what a reader could not rely on |
| 5 | Affected documentation updated in the same release | **Met**, checked by reading — §5 |
| 6 | Tarball built and extraction verified | **Met** — §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — in the release commit, with the `0.69.0` row removed |

---

## 3. Criterion 2, measured at this tree

```
commands (+++ markers):   42   (expected 42)
non-zero exits:            0
tests passed (summed):   699
tests failed (summed):     0
```

**`DEC-007`: 696 → 699**, the three new `spec_scan` tests (two plants, one
real-tree rule). Log: 1,828 lines.

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres` | exit **0** |
| `cargo build --release -p peisear` | exit **0** |
| CI on each round | **50/50, 0 not-success**, read per job |
| **the new browser step** | the `browser-overflow-gate` job's **own step list** on `29f6f7f`: five steps, `disclosure-marker-gate.mjs` among them, `success` |

**Re-verified by me rather than accepted, in each round's review:**

- the gate's guard, **planted three ways** — a new `.mjs` nothing runs, the
  wired `run:` line deleted, an exemption renamed away — each failing by
  name, tree clean after each;
- the new `image.yml` step's own logic, **run both ways**: `0.68.0` → `200`
  and present in `tags/list`; `0.999.99-does-not-exist` → `404` and absent;
- the superseded-status rule, **planted on the real tree**: a second
  unmarked `*Status*:` line inserted into a live entry fails
  `every_superseded_status_line_is_marked_on_the_real_tree` — **and `rule_b`
  with it**, which is itself the proof that `rule_b` reads the last line;
- the multi-status population: **3**, and `NFR-CMP-007` carries **one**
  status line, not the seven I claimed.

---

## 4. Criterion 6 — the archive, extracted and walked

```
archive: peisear-0.68.0-x86_64-unknown-linux-gnu.tar.gz   8,396,750 bytes
tar tzf → peisear      ← one member, at the root (NFR-REL-002)
```

The archive says `0.68.0` because `Cargo.toml` still does — the bump is held
for the release commit (§7), and `release.yml` refuses an archive whose
version does not match the tag.

**Walked the extracted binary**: unattended start, `/health` **200**,
register → authenticated `/projects` **200**, and `/today` **serves a
`<summary>` disclosure control** — the element family this release's gate
now watches. Server stopped by pid; **0 listeners** afterwards.

**What this walk does not cover, stated.** It confirms the artefact runs and
serves a disclosure control; it does **not** measure that control's computed
`display` or hit area — that is the gate's job, and the gate ran on the
runner (§3). One backend, one platform, one user.

**One thing only the real cut can show**: `image.yml`'s new step is guarded
by `if: github.event_name == 'release'`, so it has **never run in CI** —
honestly stated by the round, which planted it locally both ways instead. It
fires for the first time at leg 4. **I will read its output and report it**,
pass or fail; a failure there means the image did not become publicly
pullable and is a finding, not a retry.

---

## 5. Criterion 5 and the `DEC-028` sweep

`requirements.md` was already at `0.69.0` — `STAT-001` moved the header
fields at the series close, after `IMG-001` correctly declined to move them
mid-series on my mistaken instruction. This cut added its Appendix C
baseline row.

**`external-design.md` is unamended, and this time the reason is in the
document rather than in its absence**: `§5.1` already fixes the disclosure
mechanism as native `<details>`/`<summary>` with no scripting, and the
44 × 44 target rule already names *"disclosure toggles"* among the surfaces
it applies to. **Both re-read at this cut.** The release adds the
*confirmation*, not the property — and a confirmation belongs to the gate
inventory, not to an external-design statement.

Both Appendix C tables **re-rendered with the pinned `mdbook`**: one table
each, 19 and 13 rows, **zero** stray pipe paragraphs.

---

## 6. What is not fixed

- **`FR-HLT-005` stays `Implemented, unverified`** — the round read the test
  itself, found the same smoke-check body `QA-018` quoted, and stated what a
  real test would assert instead of softening the status.
- **A guard cannot stop a reader parsing the first field.** Only `§0.6`'s
  sentence does, and only for a reader who looks. Written into `§10.64`
  rather than claimed as closed.
- The **17** in-scope small-text sites, the **19** `opacity` sites, the nine
  `link link-hover` sites, the **`xs`** tier — each figure re-derived at this
  release's scoping, after the carried ones were found wrong.
- **The browser-rendered contrast sweep — named a fifth time.** I undertook
  to bring it as `0.70.0`'s own scope rather than name it a sixth, and that
  is still owed.
- **A Pages deployment wedged since 2026-11-18**; `proc-macro-error2` via
  three parents, none of which an upgrade clears.

---

## 7. What the release commit will contain

`Cargo.toml`'s bump, `Cargo.lock`, `CHANGELOG.md`'s `[Unreleased]` heading
becoming `## [0.69.0] — <cut date>`, and `ROADMAP.md`'s criterion-8
reconciliation, in one commit — `changelog_scan` derives the version from
`Cargo.toml` and requires a dated section for it.

---

## 8. The honest part

**This release has three rounds. One was real as I scoped it. The other two
exist because I misread the same thing three times.**

| what I claimed | what is true | the tool that fooled me |
|---|---|---|
| the registry holds **eight** tags, `0.61.0`–`0.68.0` | **eighteen**, from `0.53.0` | my own `jq '.tags[-8:]'` — a slice I wrote, then read as the list |
| `NFR-REL-010` is `Divergent`, *"the only one of 171"* | its governing status read **`Implemented`** throughout | a regex taking the **first** `*Status*:` where the rule is the **last** |
| **eight** statuses assert an absence | **one** (`FR-HLT-005`) | the same first-match parse |
| `NFR-CMP-007` carries **seven** status lines | **one** | an entry span that ran past a `##` heading |

**`rule_b` was green the whole time.** It cross-checks Appendix A against
*governing* statuses — the project's own checker was stating the correct
value while I wrote a release proposal over the top of it. The dev team
caught the first pair, and then caught the fourth **inside the handoff for
the round about the error**, and recorded it as an instance rather than
quietly fixing the number.

**`§10.64` is the entry for this**, and it names my `jq` slice beside the
document's three, because the pattern is one pattern: **reading the shape of
my own output as the shape of the world.** The ROADMAP row and the series
README were corrected in place with the wrong text quoted, not rewritten.

**What the release itself is worth is unchanged by that**: a gate that
guards thirteen controls now runs, the registry is asked whether anyone can
actually pull what we publish, and the document finally states how to read
it. **Two of the three only exist because the proposal was wrong — and all
three are things the project needed.**

**One question, and it is the only one this document asks:** cut `0.69.0`,
or hold it?
