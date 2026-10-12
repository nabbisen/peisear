# DOC-004 — the guards that read one file too few

**Three guards, each narrower than the claim it protects**, and one
convention that should have been written down fifty handoffs ago.

**Reviews carried**: `.git-exclude/reviewed/DOC-001-review.md` §3.
**Target**: 0.51.0. **Not in this round**: `NFR-MNT-005`'s test move and
`updated_at_authority_scan`'s needle — they belong together and are
`DOC-007`.

---

## §1 — rule 1 checks that crates are named, and nothing checks the number

**`DOC-001`'s review F-3, and it is larger than that review stated.**

Rule 1 reads the **root** `README.md`, `workspace-layout.md` and
`crate-boundaries.md`, and asks: *is each crate named?* **It never looks at
the count.** And `docs/README.md` — the fourth document making a claim
about the crate set — **is not in its population at all**.

**Measured, the claims about the crate set are:**

| Where | Claim | Denominator |
|---|---|---|
| `workspace-layout.md:3` | *"seven published crates"* | all seven |
| `docs/README.md:26` | *"seven crates"* | all seven |
| `crate-boundaries.md:3` | *"six implementation crates plus a thin facade"* | **six**, facade excluded |
| `crate-boundaries.md:7` | *"## The six implementation crates"* | **six** |

**Three documents, four claims, two denominators** — and `DOC-001` corrected
every one of them by hand with nothing left guarding any.

**And a naive rule would be wrong.** The same document also says *"Depends
on all four above"*, *"five of the six"*, *"four of those"*, *"four of the
other five"* — counts about subsets, not about the crate set. A rule that
matches a number word near the word *crate* would flag all of them.

**So: measure whether the rule is expressible before writing it.** Two
denominators and a page of subset counts may mean the honest answer is *no
rule*, and **declining with the reason recorded is an acceptable outcome**
— the same condition rule 6 was given about network calls.

**If you do write it**, `docs/README.md` must be in its population, because
that is the document whose claim was wrong and the one no rule reads.

## §2 — `CONTRIBUTING.md` says Rust 1.85, and rule 2 does not read it

`.github/CONTRIBUTING.md:21` — *"Install Rust 1.85+"*.
`Cargo.toml` — `rust-version` **1.88**. `NFR-CMP-001` records that **1.85
cannot build the workspace**: the dependency tree drifted past Edition
2024's own floor.

**`DOCS-003` fixed `installation.md` and rule 2 reads that file only**
(`docs_scan.rs:293-301`, matching a `**X.Y or later**` sentence). **So the
rule built to catch MSRV drift has the drift one document over** — and the
document it is missing is the one a contributor reads first.

**Fix the number, then widen the rule.** One design choice to make
deliberately rather than by accident: `CONTRIBUTING.md`'s sentence shape is
*"Install Rust 1.85+"*, not `installation.md`'s *"**X.Y or later**"*.

- **Reword the contributor doc to the canonical sentence** so one parser
  serves both — **preferred**, because a second pattern is a second thing
  to keep in step, and this project has refused that trade before.
- **Or teach the parser a second shape**, if the rewording reads badly in
  context.

**Say which you chose and why.**

## §3 — the scratch-hygiene convention

Every round builds under `.git-exclude/tmp/<round>-target` via
`CARGO_TARGET_DIR`, and **nothing ever removed one**. By 2026-10-11 there
were **65**, cleared on the owner's instruction; the disk had reached 91%.

**Write the convention down in both places it belongs:**

- **`.github/CONTRIBUTING.md`'s development setup** — one line: a round's
  scratch target directory is removed when the round closes.
- **The handoff exit-condition habit** — I will carry it in the handoffs I
  write; it belongs beside the gates, not as a separate ritual.

**One measured caveat worth including**, so nobody repeats my own error:
`/home` is Btrfs with `compress=zstd:1`, so `du` reports apparent size and
the 65 directories' **808 GB of `du`** freed **199 GB** of disk. **A build
tree costs about a quarter of what `du` says here.**

## §4 — gates

- **`DEC-007`** moves if §1's rule is written and if §2's widening adds a
  test. **State the arithmetic and name each addition.**
- **Anything you add must be seen to fail**: plant a wrong count, plant a
  wrong MSRV in `CONTRIBUTING.md`, confirm each is caught. `§10.36` is the
  register entry this project opened three weeks ago.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken), **`mdbook build`**.
- **`.github/CONTRIBUTING.md` is read by `dec_007_ci_scan`** for the
  per-crate test command block. **Extract the block fresh and confirm the
  scan still passes** — editing that file is exactly where that check
  earns its keep.
- **No migration.**

## §5 — escalate rather than deciding

- **If §1's rule cannot be written without a list of exceptions longer
  than the rule.** Decline it, say so, and say what a reader should do
  instead.
- **If rewording `CONTRIBUTING.md`'s MSRV sentence reads badly** in its
  own context.
- **If editing `CONTRIBUTING.md` moves `dec_007_ci_scan`'s own reading** of
  the test block in any way.

## §6 — exit condition

`docs/README.md`'s crate claim guarded, or a recorded reason it is not;
`CONTRIBUTING.md` naming **1.88** and rule 2 reading it, with the parser
choice stated; the scratch convention written in `CONTRIBUTING.md` with
the Btrfs caveat; every addition planted and seen to fail; `DEC-007`
stated as arithmetic; `dec_007_ci_scan` still green against a freshly
extracted block.
