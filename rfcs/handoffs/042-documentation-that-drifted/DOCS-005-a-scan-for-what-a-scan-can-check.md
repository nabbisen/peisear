# DOCS-005 — a scan for what a scan can check

**Last of three. Land it after `DOCS-003` and `DOCS-004`**, because its rules
encode the state those produce — a scan written first would fail on day one
and acquire exceptions, which is how exception lists stop being counted
(`TT-007`). **Target**: 0.49.0.

**The sweep gained a fifth pass at `0424579`**, over the documents a reader
outside this repository meets. That pass's note says plainly: *"much of it is
mechanisable and should not be done by eye."* **This is that machine.**

**`REQ-003`'s finding bounds this handoff.** No text rule detects a stale
prose *claim* — four candidate rules were tried and none reached the bar. But
*"does the number in this document match the number in that file"* **is**
mechanisable, and that distinction is the whole design: **five rules that
compare a document against a source of truth, and nothing that tries to judge
prose.**

---

## §1 — the five rules

Each compares a **document** against a **source**. If a rule cannot be
written that way, it does not belong here — report it for pass 5 instead.

| # | the document | the source of truth | what drifted |
|---|---|---|---|
| 1 | `docs/architecture/workspace-layout.md`, `crate-boundaries.md`, `README.md` | `crates/*/` | **`peisear-i18n` missing for 27 releases** |
| 2 | `docs/getting-started/installation.md` | `Cargo.toml`'s `rust-version` | **1.85 named, cannot build** |
| 3 | `docs/operations/deployment.md`'s *What to ship* | `ls static/` | **2 of 8 files listed** |
| 4 | `docs/getting-started/configuration.md`, `docs/security/hardening.md` | `config.rs`'s `unwrap_or_else` defaults | **security doc said loopback; default is `0.0.0.0`** |
| 5 | every committed file | `.gitignore` | **126 citations to `.git-exclude/`** |

**Rule 5 is the one to scope carefully.** 126 citations exist today and
**`DOCS-006` (0.50.0) is what fixes them** — so this rule would fail against
the current tree. **Write it, and report its current count rather than
landing it failing.** Two options and the choice is yours with reasoning:
gate it behind a constant you flip when `DOCS-006` lands, or write it as a
`#[ignore]`d test with the count in its name. **Do not add 126 exceptions.**

## §2 — how each rule must be written

**Scoped to a source of truth you read at runtime, never to a figure copied
from this handoff.** Rule 1 must read `crates/` and assert each name appears;
it must **not** contain the string `peisear-i18n`. A rule hardcoding today's
answer is a second copy of the thing it checks — which is `§10.17`'s shape and
this project has four instances of it already.

**Rule 2**: parse `rust-version` from `Cargo.toml`; assert no *other*
version string appears in `installation.md` as a requirement. **Beware**: that
file legitimately mentions `rustc-1.91` in its apt instructions, so the rule
cannot simply ban every version number — **decide what it asserts and say
so.** If an honest rule is not writable, **report that**: a rule that needs
three exceptions is worse than pass 5 reading the file.

**Rule 4 is the hardest and the most valuable.** `config.rs` has the defaults
in `unwrap_or_else` closures. Asserting every documented default matches may
need a convention on the Rust side. **If so, propose it rather than
contorting the scan** — and note that this is the rule that would have caught
the security error.

**Rule 3**: read `static/`, assert each filename appears in the deployment
manifest. The simplest of the five, and a good one to write first.

## §3 — where it lives

**`crates/peisear-web/src/docs_scan.rs`**, beside `changelog_scan`,
`dec_007_scan`, `touch_target_scan`, `static_js_scan` and
`untrusted_id_scan`'s successor. **This project's established shape**: a
`#[test]` module reading files from `CARGO_MANIFEST_DIR`, no server, no
fixtures.

**Paths reach outside `peisear-web`** — `crates/`, `README.md`, `docs/`,
`.gitignore` — so the relative walk differs from the existing scans'.
`changelog_scan` already reaches the repository root; **follow it rather than
inventing a second way up.**

## §4 — gates

- **`DEC-007`** three consecutive runs; the count rises by the rules you
  write. **Report the composition.**
- **Every rule seen to fail**, each against its own plant: remove a crate name
  from the diagram, change the MSRV prose, drop a file from the manifest,
  alter a documented default. **Restore byte-identical and quote each
  failure.** Five rules, five plants — a rule that has never failed has not
  been shown to check anything.
- **`peisear-web --lib` stays green**, and if a new test *file* lands, both
  halves of `DEC-007`.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **No component changes.** If one is needed, a rule is being contorted.

## §5 — escalate rather than deciding

- **If any rule cannot be written honestly** — report it for pass 5. **Four of
  five is a better result than five with an exception list.**
- **If rule 4 needs a convention in `config.rs`.**
- **If a rule finds drift I have not listed.** Likely: the audits read five
  documents closely and there are more.
- **If rule 5's count is not 126** when you measure it — mine was taken with
  `git grep -o` and is a count of *occurrences*, not of distinct paths.

## §6 — exit condition

Four or five rules in `docs_scan.rs`, each reading its source of truth at
runtime rather than hardcoding today's answer, **each seen to fail on its own
plant**; rule 5 landed in whichever form you chose, with its count reported
and its reasoning stated; anything unwritable reported for pass 5; `DEC-007`
three times at the new figure with the composition named.

**Then the sweep's pass 5 has a machine for the countable half**, and what is
left for a reader is what a reader is for: whether the prose still describes
the product.
