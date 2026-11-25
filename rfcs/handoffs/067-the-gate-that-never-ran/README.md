# 067 — The gate that never ran

**Release `0.69.0`** · row authorized 2026-11-25 (**the row, not the cut**:
`ROADMAP.md`'s *Release cycle* exit criterion 7 is a separate decision,
against a release recommendation this series does not write).

Three releases hardened what the guards **read** — the muted floor, the
semantic families, the colours a stylesheet applies, the non-text
properties. **This series is about two things nothing ever executed**, and
about the class behind them.

---

## 1. What was measured, with the commands

Every figure here was taken at `d52c76d` on 2026-11-25. **Re-measure before
each round and say what you measured. If a figure here disagrees with the
tree, the tree is right and this file is wrong** — say so in the round's
report rather than working around it. That instruction is `063`'s, and the
architect broke it four times by carrying this file's own figures (§5).

### 1.1 A gate that has never run

```bash
git log -S 'disclosure-marker-gate' -- .github/workflows/test.yml   # no commits
for f in $(git ls-files | grep -E '\.(mjs|sh|py)$' | grep -v '^crates/'); do
  printf '%-45s %s\n' "$f" "$(grep -rl "$(basename $f)" .github/workflows/)"
done
```

`browser-checks/disclosure-marker-gate.mjs` enforces `§10.58` over
**thirteen** `<summary>` controls — `display: list-item` and ≥44×44 at two
widths — which `touch_target_scan` **cannot** see, by its own documented
limit: it reads which helper a `<summary>` was given, never the style the
browser computes from it.

**No commit ever wired it.** The second command walks **every** tracked
script outside `crates/`, not a sample: it is the **only** gate absent from
every workflow. `doc-screenshots.mjs` is absent *deliberately* and
`browser-checks/README.md` says so in its own words; `cdp.mjs` is the shared
library, named only in a comment.

**It is cited as authority by things that cannot invoke it**: two
source-code comments — `components/me.rs:358`, `components/issues.rs:386` —
telling a future editor it re-runs when those lines change; five passages in
`docs/specification/requirements.md`; and eight handoffs across series
061–065 listing it as a required re-run. **A human typing its name is the
only thing that has ever run it.**

**Run at the proposal — exit 0**, every one of the thirteen passing:

```
[disclosure-marker-gate] all thirteen checkable summaries (DISC-001 three +
MARK-001 six + MARK-002 four): list-item display, >=44x44, at both widths.
```

with `overflow-gate` (120 cells, 0 failing) and `occlusion-gate` (120 cells,
0 failing) run beside it as controls. **So this series wires a passing gate,
not a failing one.** Its port is `4233`; the four in CI use `4173`, `4174`,
`4175`, `4177`.

### 1.2 ~~A status that asserts an absence, false for eight releases~~ — **corrected 2026-11-25 by `IMG-001`**

**This section was wrong in both of its claims, and both errors were the
architect's.** It read: *"`NFR-REL-010` reads `Divergent` — 'no image has
ever been published' … the tags list reads `0.61.0` … `0.68.0` — eight
releases … the only genuinely `Divergent` entry of the 171."*

1. **The registry holds eighteen tags**, `0.53.0`–`0.68.0` plus `latest`.
   The architect's own command ended `jq '.tags[-8:]'` — **he read his own
   slice as the data.** `IMG-001` re-read the full list twice and said
   plainly that nothing in the registry, the workflow source or the run
   history explained the disagreement. Nothing could: it was in his shell.
2. **`NFR-REL-010` reads `Implemented` (0.53.0)**, and has throughout. The
   entry carries **two** `*Status*:` lines, and
   `spec_scan::governing_status_token` takes the **last** —
   `body.rfind("*Status*:")`, by design. His parser took the first.
   `rule_b` cross-checks Appendix A against governing statuses and **was
   green the whole time.** Appendix A's `| Release |` row holds no
   `Divergent`: `NFR-REL-004` is `Partial`, `NFR-REL-005` is `Not defined`.

**What was genuinely missing**: nothing ever asked the registry. `IMG-001`
added an anonymous post-publish pullability check to `image.yml` — planted
failing and passing, and re-run by the architect at review (`0.68.0` → 200
and present, a nonexistent tag → 404 and absent).

### 1.3 ~~The population behind both~~ — **corrected 2026-11-25**

The *"eight of the 171 statuses assert an absence"* table came from that
same first-match parse. Re-derived against **governing** statuses: **one**,
`FR-HLT-005` (*Implemented, unverified*). The other seven were superseded
openings the document keeps deliberately.

**What was said to be there, and what actually is — corrected a second
time, by `STAT-001` itself.** This section's own first correction (above)
named **four** entries carrying more than one `*Status*:` line —
`FR-NAV-003`, `NFR-REL-009`, `NFR-REL-010`, `NFR-CMP-007` at "seven"
— with only **one** marking its earlier status as superseded. `STAT-001`
walked each entry's own span directly rather than trusting the table:
`NFR-CMP-007` carries **one** `*Status*:` line, not seven — it is not a
multi-status entry at all. **The real population is three**
(`FR-NAV-003`, `NFR-REL-009`, `NFR-REL-010`), and none of the three
marked its earlier line before this round. **The rule that the last one
governs was stated nowhere in this document — only in `spec_scan`'s
Rust** — this part held; `§0.6` states it now. A reader going top-down
met the superseded value first. That was round 3's subject, and the
architect was its first recorded victim, twice over in this file alone.

---

## 2. The rounds

| round | what it owes | state |
|---|---|---|
| [`GATE-001`](./GATE-001-the-gate-that-never-ran.md) | the gate wired, and a guard that keeps gates wired | **closed** 2026-11-25 (`29f6f7f`) — accepted, no findings; the gate now runs on the runner and passes there, and the guard was re-planted three ways at review |
| [`IMG-001`](./IMG-001-the-image-that-was-there-all-along.md) | `NFR-REL-010` corrected, and a check that asks the registry | **closed** 2026-11-25 (`4c6a7ab`) — accepted; it found the round's own premise false, left the status alone, and built the check that was genuinely missing |
| [`STAT-001`](./STAT-001-the-rule-that-lived-only-in-the-checker.md) | the document's own reading rule, the real multi-status entries, and `FR-HLT-005`'s one real absence claim | **closed** 2026-11-25 — `§0.6` states the rule, three entries marked (not four — `NFR-CMP-007` re-measured at one `*Status*:` line, not seven, a fifth instance of the same pattern caught by this round's own re-derivation), `§10.64` opened and closed, `FR-HLT-005` re-confirmed unverified; replaces the voided `ABS-001` (its population was eight only under a first-match parse; it is one) |

**Dependency order is the table's order.** `GATE-001` stands alone.
`IMG-001` needs nothing from it but shares its shape (*a claim nothing
executes*). `STAT-001` reads `IMG-001`'s finding as its own
starting point, so it goes last — and it exists because `ABS-001`, as
scoped, turned out not to.

---

## 6. Series closed — the ledger, stated plainly

**One of its three rounds was real exactly as scoped. One found its
own premise false and built what was genuinely missing instead. One
exists only because a third premise — stated in this very file at
`§1.3` — was itself the same reading error, one level up.**

- `GATE-001`: `disclosure-marker-gate.mjs`, passing and unwired since
  series `060`, now runs in CI. No finding against the round.
- `IMG-001`: `NFR-REL-010` was never `Divergent` at the point anyone in
  this series read it — its own last `*Status*:` line already said
  `Implemented`. The round left the status alone, corrected the
  misreading in place, and built the anonymous post-publish
  pullability check that was genuinely missing.
- `STAT-001`: `§1.3`'s own *"eight of 171 statuses assert an
  absence"* was the identical first-line-not-last-line error that
  produced `§1.2`'s wrong `NFR-REL-010` reading, one level up. The
  real population is **one** (`FR-HLT-005`). `§0.6` now states which
  line governs; the three real multi-status entries
  (`FR-NAV-003`, `NFR-REL-009`, `NFR-REL-010`) mark theirs; `§10.64`
  records all four instances of the pattern this series found,
  including the jq slice that produced `§1.2`'s own tag count and the
  fourth multi-status entry this very round's own table named and
  then found was not one.

`Covers release`/`Amended` moved to `0.69.0` in `requirements.md`
itself, here at the close, per `§3.1`'s own instruction and `IMG-001`'s
own deferral of it.

---

## 3. What each round owes, in common

1. **Measure at the tree, quote the command.** A figure from this file is a
   claim to check, never a premise.
2. **Plant the failure before trusting the guard.** A new or changed check
   must be watched failing on a deliberate break — in a scratch `git
   worktree` or by file copy, **never** by `git checkout`/`git stash` on
   this tree — and the plant removed with `git status` clean before and
   after. Rebuild the binary the check actually reads before trusting a run.
3. **Real exit codes.** `cmd | tail -1; echo $?` reports `tail`'s status;
   capture the command's own.
4. **`DEC-007`'s block in full** at the round's close — 42 commands, and the
   count stood at **694 passed, 0 failed** at `0.68.0`. State the new count
   and what moved it.
5. **Scratch under `.git-exclude/tmp/`** with `CARGO_TARGET_DIR` pointed
   there — never `/tmp`, never the repository's own `target/` — and removed
   when the round closes.
6. **`CHANGELOG.md` under `[Unreleased]`**, saying what a reader could not
   rely on rather than naming a mechanism. No dated section: the version in
   `Cargo.toml` and the dated heading are coupled by `changelog_scan` and
   belong to the release commit, which is not this series' business.
7. **No release legs.** `DEC-056`'s four are the architect's, on the owner's
   approval at exit criterion 7.
8. **Nothing published carries local information.** No screenshot, no
   rendered capture, no path from a private directory in any file under
   `docs/` or `rfcs/`. `docs_scan`'s own rule covers the citation half;
   the privacy half is a per-artefact exit condition of the round.

---

## 4. Deliberately not in this series

- **The 17 in-scope smallest-text sites** (`sprints.rs` 13, `teams.rs` 2,
  `sprint_plan.rs` 2 of **39** render sites), the **19** `opacity-N` sites,
  the **nine** `link link-hover` sites, the **`xs` control tier**
  (`btn-xs` 23, `badge-xs` 11, `input-xs` 3, `select-xs` 3) — all text or
  type questions, each figure re-derived at this proposal rather than
  carried.
- **`proc-macro-error2`** — three direct parents today (`leptos_macro
  0.8.16`, `reactive_stores_macro 0.4.2`, `syn_derive 0.2.0` via `rstml`);
  `leptos 0.8.23` is available against the pinned `0.8.19` and `syn_derive
  0.2.0` is its own latest, so an upgrade removes at most one of three paths
  and cannot clear the warning.
- **The browser-rendered contrast sweep** — named five times now, including
  by the architect's own `/50` error, which it would have caught. It is not
  a line in another release's *deliberately not* list: the architect brings
  it as `0.70.0`'s own scope.
- **The wedged Pages deployment** — `pages/builds/latest` returns 404 and the
  Pages API reports no status; six series have named it and it is not this
  series' business.

---

## 5. Why this file's own figures are suspect, stated once

**Three of four candidates this series re-measured were wrong as carried,
and one of them shipped.** *"The 21 remaining smallest-text sites"* was
`063`'s census **out-of-scope column** — the architect carried the
*excluded* population as the remaining work through two release candidates,
`ROADMAP.md`, and `CHANGELOG.md`'s shipped `[0.68.0]` section. *"Fifteen
bare-`opacity` sites"* had **no derivation anywhere in the repository**. The
`xs` tier figure was two releases stale.

**Which is why §3.1 is first in the list of what every round owes.**
