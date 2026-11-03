# PLT-001 — the platform nothing compiles

**Every platform but Linux is first compiled at the moment the release
archive is being produced.**

`§10.50`, measured 2026-10-29 and re-verified 2026-11-03: **all 47 of
`test.yml`'s jobs run on `ubuntu-latest`. Not one runs on Windows or
macOS.** The only workflow that compiles any other platform is
`release.yml` (`windows-latest`, `macos-latest`, `macos-15-intel`,
`ubuntu-24.04-arm`, `ubuntu-latest`), and it fires on a tag or a manual
dispatch — the latest point at which a finding can still be acted on
without re-cutting.

**Twice in one series, `release.yml` found what `test.yml` structurally
could not:**

- `TTY-002` added a helper to `main.rs` with **no `#[cfg]`, dead on
  Windows**, which `test.yml`'s own `RUSTFLAGS: -D warnings` would have
  rejected had any job compiled it there.
- `release.yml`'s first dispatch failed on **both** macOS runners with
  `script: illegal option -- f` — BSD `script` is not GNU `script`, a
  difference no `ubuntu-latest` job can express.

**Neither was hard to find. Both were simply never looked for.** And
`0.58.0` lost a published platform to this class (`§10.49`).

## §0 — what this round is

**One `windows-latest` job in `.github/workflows/test.yml`**, running
`cargo clippy -p peisear -- -D warnings`, which is the remedy `§10.50`
names itself.

**Windows only. Not macOS** — the recorded incident was Windows, macOS
costs runner minutes Linux does not, and one platform bought cheaply
beats two argued about. **Not cross-compilation from Linux**: `§10.50`
already reasons that it probably will not work, since `libsqlite3-sys`
builds C. Do not spend the round proving that again unless the real
runner refuses you.

## §1 — `-D warnings` is the entire point, and the job must be able to fail

**A Windows job without `-D warnings` is a check that cannot fail** on
the defect that motivated this entry: `TTY-002`'s dead helper was a
*warning*, not an error. `§10.36`'s whole class.

So: `-D warnings`, **no `continue-on-error`**, and the job in the same
required set as its siblings. If it is advisory, it is decoration.

## §2 — seen red, and this one costs a push

**The plant cannot be done locally** — there is no Windows here. It has
to run on a real `windows-latest` runner, which means a branch, a push,
and reading the run.

- **Plant a Windows-only defect** and watch the new job fail while the
  Linux jobs stay green. The honest shape is the one that already
  happened: a function or import that is **live on Unix and dead on
  Windows**, so `-D warnings` fires on `dead_code`/`unused_imports`
  there and nowhere else. `TTY-002`'s own defect is the model.
- **Quote the run**: the job's conclusion, the diagnostic text, and the
  fact that the `ubuntu-latest` clippy job at `test.yml:39` passed in the
  same run. **A red Windows job alongside a red Linux job proves
  nothing** — the asymmetry is the finding.
- Revert, push, confirm green.

## §3 — the trap: this round changes a number several documents quote

**`test.yml`'s job count is cited in prose.** `§10.50` says *"all 47"*;
review reports and release records quote *"48 checks `success`"* from
`gh pr checks`. **Adding a job changes both.**

- **Report the new counts** — jobs in `test.yml`, and checks on a PR —
  measured, not inferred.
- **Do not edit `§10.50`.** Register entries are the architect's
  (`CFG-001`'s division). Report the figures; the architect updates the
  entry and closes it.
- `§10.16`'s own lesson is this exact shape: *a number in prose goes
  stale on every release and nothing notices.*

## §4 — gates

`cargo fmt --check`; `clippy --workspace --all-targets -- -D warnings`
(the Linux one, unchanged); `mdbook build docs` if any document changes;
the full per-crate suite with `DEC-007` from the runner — **this round
should move the count by zero**; say so explicitly, and remember
`test.yml`'s own block lines define that count.

**And the new job's own cost, measured**: its wall-clock duration from
the run page. The owner cares about continuous cost; a number beats an
assurance.

## §5 — escalate rather than deciding

- **If `cargo clippy -p peisear -- -D warnings` is red on `windows-latest`
  today**, for a reason that predates this round: **stop and report it.**
  That is a finding — a real defect that has been invisible — and fixing
  it is its own round, not this one's tail.
- If the job needs more than a toolchain and a checkout (a C toolchain
  setup step, a cache), **name what and why** rather than accreting steps
  until it goes green.
- If Windows runner minutes turn out to cost materially more than
  expected, **report the measurement** and let the architect decide
  whether it stays.

## §6 — exit conditions

1. One `windows-latest` job in `test.yml` running
   `cargo clippy -p peisear -- -D warnings`, not advisory.
2. **Seen red on a real runner** with a Windows-only defect, the Linux
   clippy job green in the same run, both quoted; reverted and green
   after.
3. The new job-count and PR-check-count figures reported for the
   architect to apply to `§10.50`.
4. The job's measured duration reported.
5. `§4`'s gates after the last edit, `DEC-007` delta (zero) from the
   runner, and CI's own conclusion quoted.
