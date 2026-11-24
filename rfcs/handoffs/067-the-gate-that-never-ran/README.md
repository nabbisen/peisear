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

### 1.2 A status that asserts an absence, false for eight releases

```bash
T=$(curl -s 'https://ghcr.io/token?scope=repository%3Anabbisen%2Fpeisear%3Apull&service=ghcr.io' | jq -r .token)
curl -s -o /dev/null -w '%{http_code}\n' -H "Authorization: Bearer $T" \
  https://ghcr.io/v2/nabbisen/peisear/manifests/0.68.0
curl -s -H "Authorization: Bearer $T" https://ghcr.io/v2/nabbisen/peisear/tags/list
```

`NFR-REL-010 — Published container image` reads *Status*: **Divergent** —
*"no image has ever been published, and `0.53.0` is the first Release that
can publish one."* **The image is published and publicly pullable**:
manifests for `0.68.0`, `0.67.0` and `latest` all return **200**, and the
tags list reads `0.61.0` … `0.68.0` — **eight releases**. GHCR issues an
anonymous pull token for a public package, so this needs **no credentials**.

It is the **only** genuinely `Divergent` entry of the 171 (the register's
two other `Divergent` strings are prose about entries that moved on, and
**the count was taken by parsing entries, not grepping lines** — a
line-anchored `^\*Status\*:` pattern cannot see a field wrapped onto the
next line, which is how the architect reported *"zero Divergent statuses"*
at `0.68.0`).

### 1.3 The population behind both

**Eight** of the 171 statuses assert an absence — *never*, *no image*, *not
yet*, *has not been*:

| entry | the absence it asserts |
|---|---|
| `NFR-REL-010` | *"no image has ever been published"* — **false, measured above** |
| `NFR-REL-009` | *"no Release has ever carried one"* |
| `NFR-REL-011` | *"`release.yml` … never extracts one"* |
| `NFR-SEC-004` | *"the generated branch never runs"* (`§10.40`, reopened) |
| `FR-HLT-005` | **Implemented, unverified** |
| `FR-DM-002` | *"the keyboard path was never the thing being added afterward"* |
| `FR-TEAM-005` | *"never in its source"* |
| `FR-PLAN-004` | *"never a hard limit or a target"* — a clause about the product, not a claim about the world |

**An absence claim is a measurement with no re-test**: nothing fails when the
world changes under it. That is `§10.36`'s own open class — *a check that
cannot tell nothing wrong from nothing examined* — one layer out, in the
document rather than in the scans.

---

## 2. The rounds

| round | what it owes | state |
|---|---|---|
| [`GATE-001`](./GATE-001-the-gate-that-never-ran.md) | the gate wired, and a guard that keeps gates wired | **handed** 2026-11-25 |
| `IMG-001` | `NFR-REL-010` corrected, and a check that asks the registry | not yet written |
| `ABS-001` | the eight absence statuses walked, and `§10.36`'s next instance | not yet written |

**Dependency order is the table's order.** `GATE-001` stands alone.
`IMG-001` needs nothing from it but shares its shape (*a claim nothing
executes*). `ABS-001` reads `IMG-001`'s finding as one of its eight, so it
goes last.

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
