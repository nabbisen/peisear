# TTY-002 — the archive, and the two walks that were testing different things

**0.58.1, second of two work rounds. Blocked on `TTY-001`** — this
round's central assertion is that `TTY-001`'s predicate holds on a real
Windows runner, which cannot be checked before it exists.

## §1 — what is missing

The `0.58.0` Release carries **four** per-platform archives.
`docs/getting-started/install.md`'s own table offers five, including
`peisear-<version>-x86_64-pc-windows-msvc.zip`. `0.56.0` and `0.57.0`
both shipped it. That is `§10.41` — an install path the artefact cannot
satisfy — live on the published Release, and a platform regression.

The Windows job built and packaged that `.zip` correctly. It then
failed its own walkthrough, so `upload-artifact` and `Attach to
Release` never ran. The cause is `§10.49`, which `TTY-001` fixes.

**Do not fetch, rebuild or hand-attach the `0.58.0` archive.** It was
never uploaded anywhere, and an archive that did not pass the
walkthrough is exactly what we would refuse from anyone else. The
Windows archive returns at `0.58.1`, built by the workflow, having
passed a walk.

## §2 — the finding behind the finding

The two walk steps in `.github/workflows/release.yml` have been
testing different things under one name.

- The Unix step (`shell: bash`) starts the binary with `./peisear > log
  2>&1 &`. A GitHub `bash` step's `stdin` is not a tty, and a
  non-interactive shell redirects a background job's `stdin` from
  `/dev/null` besides. **It has never once exercised the wizard.** Its
  green has always meant "boots with a non-tty stdin".
- The Windows step (`shell: pwsh`) starts it with `Start-Process`,
  which creates a fresh console. **It has always exercised the wizard**,
  and nobody intended it to.

So the pair's agreement before `0.58.0` was luck, and their
disagreement at `0.58.0` was the only reason we learned anything. That
is the shape of `§10.36`, which is open: a check whose green does not
mean what its name says.

**The repair must not be aimed at the check.** `-NoNewWindow`, or
redirecting `stdin` from `NUL`, would restore the fifth archive and
ship the stopped server. If `TTY-001` is in the tree, neither is
needed.

## §3 — what to build

**Divide the walk deliberately, on both platforms**, so that each case
states which property it establishes:

1. **An unattended start serves.** No terminal, or a terminal we are
   not the foreground of, no configuration — the process must come up
   and answer `/health`, register and authenticate, as the walk does
   today. This is the case that was missing, and it is the regression
   test for `§10.49`.
2. **An attended start prompts.** A real terminal, in the foreground,
   no configuration — the two questions appear and the three
   statements follow. `test.yml` already has the pty machinery
   (`script -qfec`, with `-f`; the here-string goes on `script`
   itself, not inside `-c`) — reuse it rather than reinventing it.

Name the steps for the property, not the platform, so a future reader
cannot mistake one for the other. Where a platform genuinely cannot
host a case, **say which case it is not running and why, in the step's
own name or a comment** — an unstated gap is how this one survived.

**`PEISEAR_SETUP=never` is the right tool for CI's own intent** where
CI needs a start that does not prompt — declared, rather than relying
on a runner's incidental tty-ness. Do not use it to paper over case 1:
case 1 must pass *without* it, because its whole point is the default
behaviour of a backgrounded first run.

## §4 — the proof this round must carry

**The release workflow is the gate you are editing, so run it.** Not
the parts of it that are convenient — the Windows job, on a real
runner, producing a `.zip` that passes a walk. A `workflow_dispatch`
run or a throwaway pre-release tag is acceptable; reading the YAML and
reasoning about it is not. This is `DEC`-adjacent house rule and the
reason it exists: editing a gate obliges running that gate.

**Case 1 must be seen to fail without `TTY-001`.** Plant the old
predicate (by file copy — never `git checkout` or `git stash` on a
working tree), confirm the new unattended case goes red, restore, and
report both runs with the job URLs.

**Rebuild before you trust a run.** A stale binary or a stale server on
the port reads a fix as present or absent when it is neither; stop
scratch servers by pid and check the bind.

## §5 — the documents this round owns

- `install.md`'s five-row table, correct again once the archive is
  back. If anything in it still cannot be satisfied at `0.58.1`, the
  row goes or the text says so — a table that offers a file the Release
  lacks is the defect, not the table's formatting.
- `§10.41`: state whether this round closes it or only this instance of
  it. They are different claims.

## §6 — gates

`DEC-007` as always, re-extracted fresh from `.github/CONTRIBUTING.md`
lines 96–115, counted with the runner. Then `cargo clippy --workspace
--all-targets`, `cargo doc --workspace --no-deps`, `mdbook build docs`
and the full suite, **after the last edit**, quoted verbatim.

Editing `release.yml` also means the browser-checks rule applies if you
touch anything under `browser-checks/` — run that gate, do not reason
about its other call sites.

## §7 — escalate rather than deciding

- If case 2 (attended start prompts) cannot be hosted on a Windows
  runner at acceptable cost, **report that and leave it unrun with the
  gap named** — do not quietly let case 1 stand for both, which is the
  defect this round exists to remove.
- If the Windows job passes case 1 only with `-NoNewWindow` or a `NUL`
  redirect, stop: that means `TTY-001` did not fix what it claims, and
  that is a `TTY-001` finding, not a CI workaround.

## §8 — exit condition

1. `peisear-0.58.1-x86_64-pc-windows-msvc.zip` produced by the
   workflow, having passed a walk, and attached — demonstrated on a
   real run, URL in the report.
2. Both properties in §3 asserted by name, on the platforms that can
   host them, with every unhosted case stated.
3. Case 1 seen red against the old predicate and green with it, both
   runs reported.
4. `install.md` offers exactly what the Release carries.
5. `§10.41`'s status stated as either closed or closed-at-this-instance,
   with the reason.
6. **Carried in from `TTY-001`.**
   `docs/getting-started/configuration.md` already says
   `PEISEAR_SETUP=never` is "what the published archives' own release
   walk uses on Windows" — present tense, written before this round
   existed. **Make that sentence true, or change it.** If this round's
   Windows walk declares its intent that way, the sentence stands and
   the report says so; if it does not, the sentence is wrong and goes.
   Either is fine; leaving it unexamined is not, and a document
   asserting the state of a file it does not own is how a release
   record came to claim a release that had not happened.
7. **Carried in from `TTY-001` round 2.**
   `crates/peisear/src/main.rs` gates `is_foreground_stdin` with
   `#[cfg(unix)]`, but its helper `resolve_is_foreground` carries no
   attribute and the `#[cfg(windows)]` branch does not call it — so on
   a Windows build it is compiled and unused, which is a `dead_code`
   warning. **It is not fatal**: `test.yml` sets
   `RUSTFLAGS: -D warnings` but every one of its jobs is
   `ubuntu-latest`, and `release.yml` sets no `RUSTFLAGS`, so the
   archive still builds. Add `#[cfg(unix)]` to it — one line; the test
   module is already `#[cfg(all(test, unix))]` — and confirm it on the
   real Windows job rather than by reading the attribute.
8. **The gap item 7 exposes, to decide and state rather than
   necessarily to fix.** Every job in `test.yml` runs on
   `ubuntu-latest`. The only Windows compilation in this repository is
   in `release.yml`, which runs **on a tag** — so platform-asymmetric
   `#[cfg]` code, of which this series has added three pieces, is
   first compiled for Windows at the moment the archive is being
   produced. That is how `0.58.0` lost its archive, one level up: the
   only check that put a terminal in front of the binary was the
   release walk, too late to do anything but fail the release. This is
   `§10.36`'s question asked of the platform matrix — a green CI
   cannot tell *Windows is fine* from *Windows was never built*.
   A `windows-latest` job running `cargo clippy -p peisear` would
   close it; cross-compiling from Linux probably will not, since
   `libsqlite3-sys` builds C. **If the cost is judged too high, say so
   in the report and name the gap** — do not leave the matrix looking
   complete. Opening a `§10` entry for it is the architect's call;
   bring the finding, not the register edit.
9. The gates in §6, re-run after the last edit, quoted.
