# TTY-001 — the predicate

**0.58.1, first of two work rounds.** `RFC 020` §7, amended 2026-10-28.
Authorized by the owner the same day. This round is the product fix;
`TTY-002` is the archive and the CI that proves this round did not
regress.

**Read first:** `§10.49` in `docs/specification/requirements.md`. It
carries the measurements, and this handoff does not repeat them.

## §1 — what is wrong, in one line

`wizard::should_run_wizard` is
`stdin_is_terminal && !configuration_exists`
(`crates/peisear-web/src/wizard.rs:29`). The first operand answers *is
a terminal attached to stdin*. The decision needs *can anyone answer a
question right now*. A background job has a terminal and no reader, and
a first run there stops dead: `SIGTTIN`, nothing listening, no `.env`
written, the only trace a question in a redirected log.

**This is not a Windows bug.** It was found by the Windows release job
because that is the only place our own checks ever put a terminal in
front of the binary — the Unix walk step's `stdin` has never been a
tty, so it has never exercised the wizard at all. Reproduced on Linux
at tag `0.58.0`, twice.

## §2 — the predicate to build

Replace the first operand with *can we actually prompt*. Keep
`configuration_exists` exactly as it is — it is not implicated.

**Unix: ask the kernel, because it knows.** "Can I read this terminal"
is exactly "am I the foreground process group of it" — the same
condition the kernel evaluates when it raises `SIGTTIN`. Compare the
terminal's foreground process group against our own process group. Both
`rustix` and `libc` are already in the dependency tree as transitive
dependencies, so neither adds a new supply-chain risk; **prefer the
one that needs no `unsafe` in our crates** (`rustix::termios` and
`rustix::process` expose safe wrappers). If the only workable path is
`libc`, the `unsafe` block gets a comment naming the invariant it
relies on.

**Windows: keep `is_terminal`.** There is no foreground-process-group
equivalent, and the signals that look promising are wrong in the case
that matters most — a user who double-clicks `peisear.exe` gets a
console holding only their own process, indistinguishable from the
`Start-Process` case, and that user *is* reading. A double-click and a
terminal must both prompt. The asymmetry is deliberate: `RFC 020` §7.2
states the reason, and a comment at the `cfg` should point there rather
than restating it.

**The escape hatch, which is the Windows answer and the operator's
answer everywhere.** One launch-time environment variable:

- `PEISEAR_SETUP=never` — do not prompt, whatever the terminal says.
- `PEISEAR_SETUP=auto` — the default, and the behaviour above.

Two values, deliberately. **`always` is excluded**: a reader who wants
the wizard with configuration present deletes `.env`, which the wizard
itself already tells them, and a reader piping answers in can write the
`.env` directly. Adding a third value would buy a case that already has
two answers.

**Unrecognised values are refused at startup, not ignored.** A
misspelled `PEISEAR_SETUP=nerver` that silently means `auto` is the
same class of defect as the one this round is fixing: a signal read as
its own absence. Name the variable, the value received, and the two
accepted values — the shape `ConfigError::InvalidBindAddr` already
uses.

**`PEISEAR_SETUP` is not a `.env` key, and must not be added to
`.env.example`.** `.env`'s own existence already means *configured*, so
the variable would be unreachable there by construction. It is read
from the process environment at launch and nowhere else. Note that
`docs_scan` rule 7 (`docs_scan.rs:689`) refuses an `.env.example` key
no source recognises — **if you see that failure, the fix is to take
the key out of `.env.example`, not to teach the rule about it.** It is
also the reason this variable carries a `PEISEAR_` prefix when the four
`Config::from_env` reads do not: those four are `.env` vocabulary, this
one is not.

## §3 — being unable to prompt is not an error

When the predicate is false and no configuration exists, **start** —
loopback, a generated signing key beside the database, `.env`
untouched. `Config::from_env` already does precisely this and the
piped/unconfigured CI case already proves it, so this should be the
absence of new code rather than the presence of any. Do not add a
warning that the wizard was skipped on the path a container takes every
time: `WIZ-001` settled that a configured start's output does not move,
and a container start is the same start.

What is worth one line, **only on the Unix background case**, is that
the operator who went looking will find something: if we would have
prompted but cannot because we are not in the foreground, say so once
on stderr, naming `PEISEAR_SETUP=never` as the way to make it
deliberate. That is a message to someone who is already confused, not
a per-start warning to a container. Keep it to one line and put it
through the message table `prose_scan` enforces.

## §4 — the documents this round owns

The variable and its explanation ship together — `RFC 020` §1's own
property, applied to this round's own work.

- `docs/getting-started/configuration.md`: `PEISEAR_SETUP`, its two
  values, its default, and that it is read from the launch environment
  and never from `.env`. This page already addresses "someone writing a
  systemd unit, or deploying without a terminal to answer a prompt"
  (line 18), so the variable belongs in that paragraph's company.
- `docs/getting-started/first-run.md`, lines 58–65. **It carries the
  same conflation that caused this defect** — it tells the reader the
  questions are skipped when configuration exists, "including in a
  container, a systemd unit, or CI, where there is nobody to answer
  them", running two different conditions together under one sentence.
  Today it is incomplete rather than wrong: it never says what a first
  run with *no* configuration and *no reader* does, which is the case
  that stops. Separate the two conditions and state the background case
  plainly — a reader who backgrounds a first run should find the answer
  here before they find it in a stopped job.
- Whatever `docs_scan` rule 4 requires of a documented default. **Check
  whether rule 4's population includes this variable at all** — it
  reads `config.rs`, and `PEISEAR_SETUP` is not a `Config::from_env`
  read. If the rule cannot see it, say so in the report and leave the
  rule alone; widening it is a design change, not this round's.

`install.md`'s Windows row and the archive belong to `TTY-002`.

## §5 — gates

`DEC-007`'s per-crate test block in `.github/CONTRIBUTING.md`, lines
96–115, re-extracted fresh rather than quoted from memory. **Count with
the runner** (`cargo test --list`), never a grep — `#[tokio::test(flavor
= ...)]` has defeated a pattern twice in this repository.

Then, after the round's **last** edit and not before: `cargo clippy
--workspace --all-targets`, `cargo doc --workspace --no-deps`, `mdbook
build docs`, and the full suite. Quote the result lines verbatim rather
than summing them in prose.

## §6 — the proof this round must carry

**A test seen to fail.** The predicate's whole value is a case nobody
could previously express, so a test that passes on the fixed tree and
also passes on the unfixed one proves nothing. Plant the old predicate
back, show the new test red, restore it, show it green, and report both
runs. `§10.36` is open in this repository precisely because a green
check that cannot distinguish *nothing wrong* from *nothing examined*
has shipped here before.

**Keep the pure functions pure.** `should_run_wizard` takes booleans
today and is tested without a process environment or a real terminal;
whatever decides *can we prompt* should be shaped the same way — the
syscall at the edge, the decision in a function a test can call with
arguments. `env_file()` is the pattern.

## §7 — escalate rather than deciding

Stop and report, rather than choosing, if:

- the foreground check cannot be made to work without `unsafe` **and**
  without a new non-transitive dependency;
- `prose_scan`, `docs_scan` or any other guard objects in a way whose
  honest fix looks like changing the guard;
- the one-line stderr message in §3 turns out to be reachable on a
  container or CI start, which would make it the per-start warning
  `WIZ-001` refused.

## §8 — exit condition

1. A first run with no configuration, backgrounded from an interactive
   shell (`./peisear > log 2>&1 &`), **serves** — something listening
   on the configured address, no `SIGTTIN`, no prompt.
2. A first run with no configuration in the foreground of a real pty
   still asks the two questions and makes the three statements,
   byte-identical to `0.58.0`.
3. `PEISEAR_SETUP=never` skips the wizard with a terminal attached and
   no configuration; an unrecognised value is refused by name.
4. A configured start's output has not moved.
5. (1) and (3) each seen to fail against the old predicate, both runs
   reported.
6. `PEISEAR_SETUP` documented; `.env.example` unchanged.
7. The gates in §5, re-run after the last edit, quoted.
