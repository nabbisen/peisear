# DOCS-002 round 2 — the gate nobody is told about

**Review:** `.git-exclude/reviewed/DOCS-002-review.md` (§5)
**Round 1:** `61c7b96`, accepted — the ten links are fixed and the gate
has been seen to fail. **This handoff touches no doc link and no Rust
source.** Two findings in the gate's *surroundings*, both small.

The escalation is closed and needs nothing from you: `[`crate::AppError`]`
resolves, so the de-linked tenth is a link again (`7592fbf`, architect's).
The review's §1 records why, and the rule it generalises to.

## §1 — F-1: `CONTRIBUTING.md` has no section for the new gate

`.github/CONTRIBUTING.md`'s **"Before you open a pull request"** mirrors
each CI gate with the exact local command — Formatting (`:43`), Linting
(`:49`), Build, Tests. The `rustdoc-links` job added in round 1 has no
counterpart, so a contributor who runs the whole documented checklist
passes locally and learns about this gate from a red CI run.

Add a section in the same shape as the two above it. It needs:

- the gate's **exact** command, so local and CI cannot drift:
  `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --workspace --no-deps --keep-going`
- one sentence on why `--keep-going` is not optional — without it the run
  stops scheduling crates at the first failure and can report clean
  without ever reaching `peisear-web`, which is how round 1's own count
  came in short.
- one sentence that `rustdoc::private_intra_doc_links` is deliberately
  **not** denied, so nobody adds it and finds eight failures.

Placement is yours; beside Linting is the natural slot (same tier).

**No scan test.** `DEC-007`'s scan covers the test block against
test.yml's test jobs and correctly does not fire here, and a general
"every CI job needs a `CONTRIBUTING.md` section" rule would be wrong —
the browser gate does not run from a one-line local command. A rule with
exceptions costs more to maintain than this buys. One section, no
mechanism. **Do not add a guard for this**; if you think that is the
wrong call, say so rather than building one.

## §2 — F-2: the job comment cites a path no clone contains

`.github/workflows/test.yml`, the `rustdoc-links` comment, last clause:

> `# eight -- see `DOCS-002`'s review for the ruling.`

Reviews live under `.git-exclude/`, which is gitignored (`.gitignore:13`)
and holds nothing tracked. For every reader but us that is a dead
reference.

**Delete the clause.** The comment already states the ruling in full
immediately above it — eight deliberate cross-references to
implementation detail, permanent rather than pending, with the kinds
named. It loses nothing. Do **not** substitute a pointer to the
requirements register either: the reason belongs where the lint is
configured, and a second indirection is the same defect with a longer
path.

## §3 — scope and gates

Two files: `.github/CONTRIBUTING.md`, `.github/workflows/test.yml`. No
Rust source, no doc comment, no test. Expected diff: one new section and
one deleted clause.

Before reporting:

- `cargo fmt --all -- --check` — exit 0.
- The gate command from §1, **run exactly as you wrote it into
  `CONTRIBUTING.md`**, copied from that file rather than retyped — exit
  0. This is the one check that matters here: the point of the section
  is that the command in it works.
- `DEC-007`, one run. 367/0 expected, unchanged — nothing here is a test.
- No `actionlint` run is being asked for; if one exists locally, say so.

**Not in scope**, both recorded in the review's §6 as the architect's:
`test.yml:69`'s stale "eighteen rendered pages" (the fixture holds 24)
and the duplicated `/.git-exclude/` in `.gitignore`. Leave both.

---
**Who holds what**: dev team — §1 and §2, one change. Architect — the
register items at 0.43.0 and §6's two. **What's next**: a review request
package for this, then DOCS-002 closes.
