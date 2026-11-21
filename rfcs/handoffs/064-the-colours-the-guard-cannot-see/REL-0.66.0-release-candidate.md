# `REL-0.66.0` — release candidate

**Recommendation: cut it.** Exit criteria 1–6 are met, measured at this tree
rather than carried from the rounds' reports. **Criterion 7 is the owner's**,
and none of `DEC-056`'s four legs runs until that approval.

Written 2026-11-22 against `main` at `566e224` plus this cut's own `DEC-028`
sweep.

---

## 1. What this release is

**A P1 requirement recorded `Met` was not met, and the reason was
structural.** `NFR-A11Y-005` says text must meet WCAG AA (4.5:1). Fifteen
shipped sites did not — **every destructive action in the product among
them**, plus the sign-in ↔ register links a new reader meets first. The
check that was supposed to see this read **one** of the four colour families
the product's theme defines, and the entry generalised past its own
evidence: *"the theme's own tokens were never the problem"* was true of
`base-content` and read as true of all of them.

Measured against the one theme the product ships (`data-theme="corporate"`,
hardcoded): `success` **3.03:1**, `error` **3.08:1**, `primary` **4.21:1**
on `base-100`, worse on `base-200`/`base-300`, against `base-content`'s
**17.21:1**.

**All fifteen are repaired with patterns the product already owned** — a
plain ghost or outline button inheriting `base-content`, a `link` whose own
underline carries the affordance, `base-content` for a status line. **No
colour was added, `app.css` is untouched, and the theme was not changed**:
darkening `--er`/`--su`/`--p` would have repaired these fifteen and put 37
passing `btn-primary` sites and every solid badge at risk. **The one red
that passes stays exactly where it was** — the solid confirm button on the
delete interstitial, 6.83:1.

**`contrast_scan` now reads all four families** and bans a semantic colour
in a text position, with three sites excluded by their own exact class value
**and by exact count**, so a later site cannot inherit an exemption written
for a glyph. Both guards were planted and watched failing; **both plants
were re-run independently at this candidate.**

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff in dependency order | **Met** — `bf02b24`, `56ed15b`, `e74ac95`, `566e224`, each reviewed and accepted, each its own commit, in order |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, **with captured logs** | **Met** — §3 |
| 3 | Requirements amendments applied; nothing `Divergent` without a gap entry | **Met** — `NFR-A11Y-005` amended, `§10.61` opened and closed, zero `Divergent` statuses; the sweep's own findings in §5 |
| 4 | `CHANGELOG.md` records the *why* | **Met** — one `Fixed` entry family across the three rounds: what a reader could not read, what changed, and what was deliberately left |
| 5 | Affected documentation updated in the same release | **Met**, and checked rather than assumed: `external-design.md`'s only colour rules are `§3.1`'s unread count and `SCR-05`'s notification row, neither touched, and it states no link styling and no destructive-action colour |
| 6 | Tarball built and extraction verified | **Met** — §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — in the release commit, with the `0.66.0` row removed |

---

## 3. Criterion 2, measured at this tree

The `DEC-007` block exactly as `.github/CONTRIBUTING.md` writes it, in a
scratch target directory, every exit code written to the log:

```
commands (+++ markers):   42   (expected 42)
non-zero exits:            0
tests passed (summed):   689
tests failed (summed):     0
```

**`DEC-007`: 687 → 689**, the two new guard tests. **That figure needed
settling rather than copying**: `CLR-003` round 1 reported the block as
**687** while stating in the same report that `peisear-web --lib` had gone
246 → 248 — two readings that cannot both hold. Round 2 reported 689; I ran
the block myself instead of choosing. **689 is right, round 1's figure was
stale, and my own review of that round let it through.**

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres` | exit **0** |
| `cargo build --release -p peisear` | exit **0** |
| CI on `566e224` | **50/50 jobs, 0 failed** — where the browser gates ran on the final tree |

**Both plants re-run at this candidate, not taken from the reports.** The
ban: restoring `btn btn-outline btn-error btn-sm` in a throwaway worktree
made `no_semantic_colour_in_a_text_position` fail, naming the file and the
class value (baseline 3 passed). The counts: adding a third
`class="text-primary"` made `every_named_exclusion_matches_its_exact_live_count`
fail with `left: 3, right: 2` and a message that explains both directions.

---

## 4. Criterion 6 — the archive, extracted and walked

Built the way `release.yml` builds it (`-C dist`, files at the archive root
per `NFR-REL-002`):

```
archive: peisear-0.65.0-x86_64-unknown-linux-gnu.tar.gz   8,397,522 bytes
tar tzf → peisear      ← one member, at the root
```

**The archive says `0.65.0` because `Cargo.toml` still does** — the bump is
held for the release commit (§7), and `release.yml:113` refuses to attach an
archive whose version does not match the tag, so the published one will say
`0.66.0` or the release fails loudly.

**Walked the extracted binary** — and this walk reaches the repairs
themselves, not just the pages they live on:

| step | result |
|---|---|
| unattended start | served with defaults, **no wizard**; `/health` **200**; bound to `127.0.0.1` only |
| `/login` | **`link-primary` = 0** — the token is gone from the served page; the plain underlined `link` is there |
| `/register` (POST) → `/projects` | **303**, then **200** authenticated |
| team detail | **`text-error` = 0**, nine plain `btn-ghost` controls |
| project **edit** page (where the danger zone lives) | **`btn-outline` = 1, `btn-error` = 0, `text-error` = 0** — `projects.rs`'s repaired delete affordance, served |
| delete **confirmation** page | **`text-error` = 0** (the heading's colour gone) and **`btn-error` = 1** — the one red that passes, still exactly where the design wants it, on the step that commits |
| shutdown | server stopped by pid; **0 listeners** afterwards |

**What this walk does not cover, stated rather than implied.** The
e-mail-opt-in status line renders its **off** branch for a fresh account
(*"Email opt-in is off…"*), so the repaired on-state line was **not** on any
page I fetched — its repair rests on `CLR-002`'s own measurement and on the
source. The two `btn-xs` remove buttons sit behind `confirm()` dialogs in
table rows a one-user fixture does not populate; the sprint delete
affordances need sprint data this walk has none of. And as always: one
backend (SQLite), one platform (x86-64 Linux), one user. The five platform
archives and the container image are built by CI on the tag.

---

## 5. The `DEC-028` sweep

Both specifications are at `0.66.0`. **This is the first release since
`0.62.0` to amend a requirement**, so `requirements.md`'s header says so,
and its `Amended` field moves to `2026-11-22 (0.66.0)` — **the field
`§10.36` instance 7 asked every sweep to check, checked again here, and this
time it moved.** New `0.66.0 (this baseline)` rows in both Appendix C
tables, **re-rendered with the pinned `mdbook` to verify**: one table each,
16 and 10 rows, **zero** stray pipe paragraphs, the new rows inside `<td>`.

**And the sweep caught one of mine before it shipped.** Writing
`external-design.md`'s new header I cited *"§11.4's unread count"* and
*"§17.6's notification row"* — **both section numbers invented**. The rules
live at **`§3.1`** and **`SCR-05`**. Corrected, and the header now records
that they were located by reading the document rather than cited from
memory, which is the lesson instance 8 exists for.

---

## 6. What is not fixed, and is not fixed by this release

- **Nine `link link-hover` sites with no underline at rest** — distinguished
  from their surroundings by position alone until pointed at. An affordance
  question rather than a contrast one; recorded as a candidate.
- **Fifteen bare-`opacity` sites**, each carrying two conditions now — and
  one of them compounds a card's own `format!`-built fade, which
  `css_scan`'s doc records as its own blind spot, so that site hides from
  two scans rather than one.
- **The `xs` control tier** (18 `btn-xs`, 2 `select-xs`) and **the 21
  remaining smallest-text sites** — both recorded, neither this release's.
- **A GitHub Pages deployment wedged since 2026-11-18**, which GitHub
  refuses to cancel by any route this token has. Mitigated at `0.65.0` by
  moving the `pages` lock onto the deploy job, so the book's link check runs
  on every commit (**it ran on `566e224`**) and only publication waits.
- **`proc-macro-error2`**, still reaching the graph through
  `syn_derive 0.2.0`, its own latest version.

---

## 7. What the release commit will contain, and why it is held

As `0.65.0` held it: **`Cargo.toml`'s bump, `Cargo.lock`, `CHANGELOG.md`'s
`[Unreleased]` heading becoming `## [0.66.0] — <cut date>`**, and
`ROADMAP.md`'s criterion-8 reconciliation, in one commit — because
`changelog_scan` derives the version from `Cargo.toml` and **requires a
dated section for it**, so bumping either alone turns the suite red.

Nothing above is committed. `main` is honestly `0.65.0` plus unreleased
work until the owner approves.

---

## 8. The honest part

**Five of the architect's own figures or instructions were corrected by
these rounds**: a ratio asserted against the wrong background (three sites
at 2.50:1, not 3.08:1); an after-figure that ignored ambient opacity
(6.42:1, not 17.21:1); a hover measurement this environment cannot produce
at all; *thirteen* repairs where the total is fifteen; and a stale
`DEC-007` figure that **my own review let through**.

**Two more were caught by me, in this cut's own sweep**: the invented
section citations above, and the `"text-primary"` exclusion — two words
wide — which would have absorbed the next instance of the very defect this
release exists to prevent, with both new tests green.

Every one was found the same way: by someone re-deriving instead of
trusting the sentence in front of them.

**One question, and it is the only one this document asks:** cut `0.66.0`,
or hold it?
