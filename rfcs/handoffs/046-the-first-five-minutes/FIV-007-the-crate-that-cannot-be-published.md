# FIV-007 — the crate that cannot be published

**Target**: 0.53.0, blocking. **Found 2026-10-17 by the release's own
`DEC-056` leg 3**, after legs 1 and 2 had already run. `§10.44`.

```
error: proc macro panicked
  --> src/static_assets.rs:30:30
30 | static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../static");
   = help: message: "…/package/peisear-web-0.53.0/../../static" is not a directory
error: failed to verify package tarball
```

**`peisear-web` 0.53.0 cannot be published**, and `cargo install
peisear` from crates.io would fail the same way — the release's own
headline fix, broken at the one boundary nothing tested.

---

## §1 — the constraint, which decides the shape of the fix

**Cargo cannot include files from outside a package directory.** There
is no `include` entry, no path, and no flag that makes
`../../static` part of `peisear-web`'s tarball. Verified at the tag:
`cargo package -p peisear-web --list` → zero `static/` entries, and no
manifest carries an `include`.

So the embed source has to live **inside** the crate that embeds it.
Everything below follows from that, and **if you find a way to keep
`static/` at the workspace root and still publish, say so before
building it** — that would be a better answer than this one.

## §2 — move `static/` into `crates/peisear-web/static/`

- `include_dir!("$CARGO_MANIFEST_DIR/static")`, no `..` at all.
- Add it to `peisear-web`'s `Cargo.toml` `include` list so it is in
  the tarball — **and prove it is**, with `cargo package -p
  peisear-web --list` showing all eight files.

**Five readers resolve that path and each must follow**, none of them
through the router:

| reader | what it reads `static/` for |
|---|---|
| `static_js_scan` | every file's extension; `.js` contents (`STATIC-001`, `BOARD-001`) |
| `undo_toast_attachment_scan` | `static/*.js` |
| `dm_fallback_boundary_scan` | `static/` |
| `docs_scan` rule 3 | the deployment manifest against the real directory |
| `docs/operations/deployment.md` | the manifest itself |

**Enumerate them and state the count you found** — the table above is
mine and may be short. A scan that silently stops reaching its subject
is `§10.39`'s class and this project has had it twice.

## §3 — the guard, which is the part that outlives the fix

**Add `cargo publish --workspace --dry-run` as a CI job.** Not a
release-cycle checklist line: it is a command, so it belongs where
commands run, and a checklist entry would be the ritual exit criterion
9 was withdrawn for.

- It is the **only** check that builds a crate from its own package.
  Every other gate builds in the workspace, where `../../static`
  resolves and this defect is invisible.
- **Make it fail the way it failed here**, once, before trusting it:
  plant the broken path back, watch the job go red, restore. A guard
  never seen to fail is `§10.36`.
- Cost it honestly — it packages and verifies seven crates. If that is
  too slow for every push, say so and propose where it runs instead;
  **do not silently make it `workflow_dispatch`-only**, which would
  leave it exactly as absent as it was today.

## §4 — `NFR-REL-011`'s Option C job tests the wrong artefact

The job runs `cargo install --path crates/peisear`, which has the
workspace beside it. `install.md`'s Option C is `cargo install
peisear`, which does not. **That is why the job was green while the
path was broken.**

`--path` cannot be made to test the real thing before publication, and
`cargo install peisear` cannot be tested before the version exists on
crates.io. **Say what you think closes this honestly** — the dry run
in `§3` is the nearest pre-publication proxy and may be the whole
answer. The entry is `Partial` until something does.

## §5 — specification work

- **`§10.44`** is open and is this round's to close, **on a successful
  `cargo publish --workspace --dry-run`**, quoted.
- **`NFR-REL-011`** returns to `Implemented` only if `§4` is answered
  with something real; otherwise it stays `Partial` and says why.
- **`§10.41`** is **not** reopened. Its defect — an installed binary
  with no assets — is genuinely fixed wherever the workspace exists.
  Resist the tidiness of reopening it; this is a different boundary.

## §6 — gates

- `DEC-007` from a fresh block. **This moves**: `static_js_scan` and
  friends change their path, and a new test may come with `§3`.
  Account for the delta exactly.
- `fmt`, `clippy` both profiles, rustdoc after the last edit, `mdbook`.
- **`cargo publish --workspace --dry-run`, quoted in full** — the
  round's whole point.
- The three browser gates: `static/` moving is exactly the shape that
  breaks a page without breaking a test.
- **Re-run `NFR-REL-011`'s three walks.** The archive and image build
  from the workspace and should be unaffected; prove it rather than
  assuming it.

## §7 — escalate rather than deciding

- **If keeping `static/` at the workspace root turns out to be
  possible** and publishable. Better answer; bring it.
- **If the dry run is too slow for CI.** Bring the number and a
  proposal; do not quietly narrow it.
- **If moving the directory breaks a scan you cannot repoint without
  widening it.** A wider rule is a design change (`§10.39`).

## §8 — exit condition

`static/` inside `crates/peisear-web/`, in the tarball and shown to be
by `cargo package --list`; every reader repointed with the count
stated; `cargo publish --workspace --dry-run` passing **and** added as
a CI job **and** seen to fail against the planted old path; `§4`
answered; `DEC-007`'s delta accounted for exactly; all gates re-run
after the last edit; `§10.44` closed on the dry run's own output.
