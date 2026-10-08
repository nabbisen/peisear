# DEP-001 — two yanked crates in the lockfile

**Found by the `0.43.0` candidate's `cargo publish --dry-run`**, reported by
the dev team as pre-existing and correctly not treated as a candidate
finding. It is scheduled now rather than carried.

**One is in the shipped binary and one is not**, and the handoff is written
around that difference. Neither is known to be vulnerable: *yanked* means the
publisher withdrew the version, which can be a security withdrawal or a
mistaken publish, and we do not know which.

---

## §1 — what is true, verified

I resolved both against crates.io and traced both with `cargo tree -i`:

| Crate | In lock | Yanked | Nearest non-yanked | Reached via | Shipped? |
|---|---|---|---|---|---|
| `spin` | `0.9.8` | yes | **`0.9.9`** | `flume 0.11.1` → `sqlx-sqlite 0.8.6` → `sqlx 0.8.6` → `peisear-storage` | **yes** |
| `chacha20` | `0.10.0` | yes | **`0.10.2`** (`0.10.1` is also yanked) | `rand 0.10.1` → `rust-multipart-rfc7578_2 0.9.0` → `axum-test 20.0.0` → `peisear-web` **[dev-dependencies]** | **no — test only** |

**Both updates resolve, and I checked rather than assuming.**
`cargo update -p spin --precise 0.9.9` and
`cargo update -p chacha20 --precise 0.10.2` each succeed and together move
**four lines of `Cargo.lock` and nothing else** — same minor version in both
cases, no transitive movement, no `Cargo.toml` change. I reverted the lockfile
byte-for-byte afterwards; the tree you receive is unchanged.

So the work is not the update. **The work is the gates**, because a dependency
change is exactly the kind that compiles on one toolchain and not another.

## §2 — what to do

1. **Both updates, in one commit**, `Cargo.lock` only. No `Cargo.toml` edit:
   the version requirements already admit these versions, which is why
   `--precise` was unnecessary and `cargo update -p <crate>` alone would do.
2. **Run the full gate set**, and the MSRV job is the one that matters here:
   `NFR-CMP-001` declares **`1.88.0`** and a CI job builds at it. A patch
   bump should not raise an MSRV, but *should not* is not *does not*, and
   this requirement's own correction records an MSRV that was false because
   nothing exercised the claim. **Build at the MSRV, not only on the pinned
   toolchain.**
3. `DEC-007` three consecutive runs, **367** expected and unchanged. `fmt`,
   `clippy --workspace --all-targets -- -D warnings`, and the
   `rustdoc-links` gate. The overflow gate is not implicated; say so rather
   than running it.
4. **Confirm the dry run is clean of both advisories afterwards**:
   `cargo publish --workspace --dry-run` from a detached worktree. That is
   where this was found, so it is where it has to be seen gone.

## §3 — what not to do

- **Do not run a bare `cargo update`.** That would move all 186 dependencies
  behind latest and turn a four-line change into an unreviewable one. Two
  named crates, nothing else.
- **Do not add a vulnerability scanner in this handoff.** `cargo-audit` or
  `cargo-deny` in CI is a reasonable thing to want and it is **a decision,
  not an implementation detail** — a new gate that fails the build on a
  third party's advisory feed has a continuous maintenance cost and is the
  owner's call, not mine and not yours. If you think it is worth proposing,
  say so in the report in one paragraph and stop there.
- **Do not reword `NFR-CMP-001` or any requirement.** `DEC-028` work is the
  architect's.

## §4 — escalate rather than deciding

- **If either update moves anything beyond those four lines.**
- **If the MSRV build fails**, or if either crate's new version raises a
  minimum toolchain above `1.88.0`. That converts this from a lockfile bump
  into an `NFR-CMP-001` question and it stops being your call.
- **If `spin 0.9.9` is itself yanked by the time you run it** — report the
  state you find rather than reaching for `0.12.3`, which is a different
  minor and would not be a lockfile-only change.
- **If the dry run still reports an advisory** for anything after the bump.

## §5 — exit condition

A commit touching `Cargo.lock` and nothing else, four lines; the full gate
set green including a build at the **MSRV**; `DEC-007` 367 three times; and a
clean `cargo publish --workspace --dry-run` from a detached worktree with
**neither advisory** present. Then stop.

**This is a small handoff and it should stay small.** If the measurement
turns it into something else, that is a finding and a separate handoff, not a
reason to grow this one.
