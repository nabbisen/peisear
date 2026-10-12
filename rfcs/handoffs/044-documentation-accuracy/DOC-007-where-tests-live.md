# DOC-007 — where tests live

**Three things, and the title means two of them.** Where test *code* sits
in the tree, and where test *databases* sit on the machine. Plus the scan
whose needle the first one unblocks.

**Reviews carried**: `.git-exclude/reviewed/REG-001-review.md` §3 (as
corrected), `.git-exclude/reviewed/DOC-004-review.md` §3.
**Governing rule**: `.git-exclude/rules/project-instructions-rust.md`.
**Target**: 0.51.0.

---

## §1 — classify before moving anything

`NFR-MNT-005` reads **Divergent** (`§10.37`), against the owner's standing
instruction:

> ⭕ **Good:** `src/some_mod.rs` and `src/some_mod/tests.rs`.
> ❌ **Bad:** `#[test]` modules placed inside `src/some_mod.rs`.

**25 files under `crates/*/src/` carry test attributes, holding 79 test
functions. They are not one kind, and I will not have 25 files moved on the
assumption that they are.** My own measurement, for you to check and
correct:

| Kind | What | Examples | Count |
|---|---|---|---|
| **1** | **Implementation with a test module appended** — the ❌ example exactly | `peisear-core/src/lib.rs` (13 public items, 3 tests), `components.rs`, `handlers.rs`, `search.rs`, `me.rs`, `edge.rs`, both `config.rs`, `pool.rs` | ~11 |
| **2** | **Modules that *are* a test** — zero public implementation, existing in `src/` only because they read the source tree (`env!("CARGO_MANIFEST_DIR")`) or private items | the ten scans, `optimistic_lock_guards.rs`; and `docs_scan.rs`/`changelog_scan.rs`, whose one or two helpers serve only their own tests | ~13 |
| **3** | **`lib.rs` files that only declare `#[cfg(test)] mod`** | `peisear-web/src/lib.rs` | 1 |

**Kind 1 moves.** `src/<mod>.rs` keeps the implementation; its tests become
`src/<mod>/tests.rs`. That is the instruction, literally.

**Kind 2 is a judgement and it is mine, not yours.** Moving
`docs_scan.rs` to `docs_scan/tests.rs` leaves a parent file containing a
`mod` line and nothing else — which satisfies the letter and may be worse
than what it replaces. **Report your counts and your reading; do not move
kind 2 until I have answered.** Kind 3 is not a violation at all.

**If my three kinds are wrong, say so with the counts** — I derived them by
counting public items against test attributes, which is a proxy, not a
reading.

## §2 — then the needle, which the move unblocks

`updated_at_authority_scan` enforces `NFR-CONC-003` — *the application MUST
NOT write `updated_at`* — with **one literal string**:

```rust
if source.contains("updated_at = CURRENT_TIMESTAMP") {
```

**The requirement bans the write; the scan bans one spelling of it.**
`SET updated_at = ?`, `= datetime('now')`, or a literal all pass. **And
`0021` made it worse**: the triggers now say `strftime(…)`, so the likeliest
future mistake is copying the migration's own new idiom — the one spelling
the scan cannot see.

**Why it waits for §1.** A needle matching `SET updated_at` generally would
flag `pool.rs`'s own planted `UPDATE` from `REG-001`. **Once kind 1 has
moved, test code is at a predictable path** and the scan can exclude
`*/tests.rs` and `*/tests/*` **by path rather than by name** — which is the
difference between a rule with edges and a rule with a list.

**Widen it, exclude test paths, and plant both halves**: an application
write in a non-test file is caught; the same statement in a `tests.rs` is
not.

**If kind 2 stays put**, `optimistic_lock_guards.rs` is a test module under
`peisear-storage/src` that the path rule will not match. **Say so** — that
is the exclusion list growing, and it is the signal that §1's answer was
wrong.

## §3 — and where the test *databases* live

`crates/peisear-web/tests/common/server.rs:52-55`:

```rust
tempfile::Builder::new().prefix("peisear-web-it-").tempdir()   // no tempdir_in
```

`tempdir()` uses `std::env::temp_dir()` — **`/tmp`, which here is `tmpfs`,
30 GB, shared with everything else on the machine** — and **33 test files**
use this harness. `DOC-004`'s flake was a `500` from a test that touches
none of its work, beside a `/tmp` ENOSPC. **That is the chain.**

**Measure before moving.** tmpfs is very likely why 438 tests run as fast
as they do, and trading that for robustness is a decision that needs a
number:

- `cargo test --workspace --locked`, wall time, **three runs**, with the
  default temp root.
- The same, with the harness pointed at a disk-backed directory under
  `.git-exclude/tmp/`.
- **Report both and the ratio**, and name the machine.

**My prior, to argue against**: if the cost is small, move it — a release
gate that depends on unrelated machine state is worse than a slower one,
and this one fails as *"the optimistic lock is broken"*. **If the cost is
large**, leave it and write the failure mode down where the next person
meets it, so a `500` in an untouched test is read correctly the first time.

**Either way the decision is recorded in the harness**, beside the call,
with the number and the date — the shape `STORE-002` used for
`synchronous`.

## §4 — gates

- **`DEC-007`**: §1 moves tests without changing them, so **the count must
  not move**. If it does, something was lost. §2 adds a plant or two.
  **State the arithmetic.**
- **`§10.37` and `NFR-MNT-005`**: if kind 1's move closes the divergence,
  **say so and leave the status to the candidate** — a status change is a
  `DEC-028` amendment, not a round's side effect.
- **Every scan still green after the move**: the ten kind-2 modules read
  the source tree, and moving kind 1's files changes what that tree looks
  like. **`dec_007_fs_scan` in particular** enumerates test files.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken), `mdbook build`.
- **No migration.**

## §5 — escalate rather than deciding

- **Kind 2, always.** Report and stop.
- **If moving kind 1 breaks a test's access to a private item** — that is
  the carve-out I wrongly assumed existed, and finding one real instance
  would change the rule's reading.
- **If `dec_007_fs_scan`'s own enumeration changes meaning** when test
  files move.
- **If the tmpfs measurement is within noise**, say so with the spread
  rather than picking a winner.

## §6 — exit condition

Kind 1 moved to `src/<mod>/tests.rs` with `DEC-007` unchanged; kind 2
reported and untouched; the authority needle widened with test paths
excluded and both halves planted; three timed runs on each temp root with
the ratio and the machine named, the decision recorded beside the call; every
scan green.
