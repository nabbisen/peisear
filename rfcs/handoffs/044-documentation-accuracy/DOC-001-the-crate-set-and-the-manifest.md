# DOC-001 — the crate set and the manifest

**`DOCS-006`'s countable half**, deferred from 0.49.0 when `DOCS-005` wrote
two of its rules as assertions that the gap *exists*. **Close the documents,
then flip the rules.**

**Target**: 0.51.0, first round.

---

## §1 — the crate set

| Document | What it says | Measured |
|---|---|---|
| `docs/architecture/workspace-layout.md` | enumerates the workspace | **zero** mentions of `peisear-i18n` |
| `docs/architecture/crate-boundaries.md` | *"why there are N crates and what belongs in each"* | **zero** |
| `docs/README.md` | *"why there are **six** crates"* | **zero**, and the count is wrong |

**Seven crates.** `peisear-i18n` has existed since `RFC 006` and is the sole
source of every user-visible string; a reader following
`crate-boundaries.md` to decide where a change belongs is told it does not
exist.

**Write it in each document's own register** — `workspace-layout.md`
enumerates files and directories, `crate-boundaries.md` argues about
responsibility, `docs/README.md` is an index. **Three different jobs; do not
paste one sentence into all three.**

**And say what the crate is for**, not only that it is there:
`crate-boundaries.md`'s table is the one a contributor reads before putting
a string somewhere, and `I18N-004`/`I18N-005a` already settled that the
answer is *every* user-visible string, including indicator labels that used
to live in `peisear-core`.

## §2 — the manifest

`docs/operations/deployment.md:37-39`'s *What to ship* lists **two** files
under `static/`:

```
└── static/              # Required at runtime
    ├── app.css
    └── board.js
```

`static/` holds **eight**: `app.css`, `board.js`, `calendar.js`,
`daisyui.min.css`, `dm.js`, `plan.js`, `search.js`, `tailwind.css`.

**An operator following this ships a deployment whose calendar, sprint
plan, dark mode and search are broken**, and whose stylesheets are two
thirds missing.

**Do not fix it from `ls`.** The manifest's claim is *required at runtime* —
derive the set from **what the rendered HTML actually references**, and
report the method. If a file in `static/` turns out to be referenced by
nothing, **say so rather than listing it**: that is a dead asset and a
different finding, and shipping it because it happens to exist is the same
mistake in the other direction.

## §3 — then the rules, which were written waiting for this

`docs_scan.rs` carries two tests that assert the gap **exists**:

- `rule_1_deferred_names_the_architecture_docs_gap` (`:454`)
- `rule_3_deferred_names_the_missing_manifest_entries` (`:494`)

**They pass today because the documents are wrong.** Closing §1 and §2 makes
both fail, which is the point: they were written to fail the moment someone
fixed what they describe.

- **Rule 1 widens** to all three documents, as `DOCS-005`'s own §1 table
  always intended — `mentions_crate` already handles both naming
  conventions.
- **Rule 3 becomes exhaustive in both directions**: nothing the manifest
  lists is missing from `static/` (live today), **and** nothing in
  `static/` is missing from the manifest.
- **Delete both deferral tests.** They are scaffolding with their purpose
  served; leaving them as `#[ignore]`d sediment is how a register entry
  becomes a graveyard.

**`live()`'s filter** currently excludes their messages. **Remove the
exclusions with the tests**, or the suite keeps a hole shaped like a defect
that no longer exists.

## §4 — gates

- **`DEC-007` will move twice over** — two tests deleted, and whatever rule
  1's widening and rule 3's second direction need. **State the arithmetic**:
  431, minus what, plus what, equals what.
- **Both scans must be seen to fail.** Plant a missing crate in each of the
  three documents and a missing file in the manifest; `§10.36` is three
  weeks old and its subject is a rule that passes having examined nothing.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken).
- **`mdbook build` with linkcheck** — three documents change.
- **No migration**, no browser gate unless you touch something they read.

## §5 — escalate rather than deciding

- **If a file in `static/` is referenced by nothing.** Name it; do not
  quietly add it to the manifest or quietly delete it.
- **If rule 1 cannot cover all three documents** without a per-document
  special case that makes it unreadable. A rule that needs three branches
  may be three rules, and that is worth saying.
- **If closing the documents turns up a fourth that enumerates crates.**

## §6 — exit condition

Three documents naming seven crates, each in its own register; a manifest
derived from what the HTML references, with its method stated and any dead
asset named; rules 1 and 3 widened, both planted and seen to fail; both
deferral tests and their `live()` exclusions gone; `DEC-007` stated as
arithmetic; `mdbook build` clean.
