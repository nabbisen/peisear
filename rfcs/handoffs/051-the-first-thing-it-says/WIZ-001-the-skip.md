# WIZ-001 — the skip

**0.58.0, round 1 of 3.** `RFC 020`, and deliberately **not** the
wizard.

`RFC 020` §3: *"**Not a prompt in a container.** Detect a
non-interactive stdin and use defaults. A wizard that blocks a
`docker run` is an outage."* **That is the only thing in this release
that can hurt a user**, so it is built and proven first, on a stub
with nothing to prompt with.

---

## §1 — the property, and why it comes before the feature

A wizard that asks a question nobody can answer hangs the process.
`peisear` runs in a container image this project publishes, in systemd
units its own documentation describes, and in CI. **In all three,
stdin has no terminal and a prompt is an outage.**

So this round adds the detection and the walk that proves it, with a
wizard body that prints one line and returns. **When `WIZ-002`
replaces that body, the proof is already standing.**

This is `CON-001`'s shape from 0.56.0 — the prerequisite round, built
and proven before the thing that depends on it.

## §2 — the two skip conditions, measured

**1. stdin is not a terminal.** `std::io::IsTerminal` is in the
standard library — stable since 1.70, and this workspace's MSRV is
**1.88** (`Cargo.toml`'s own `rust-version`, exercised by CI's MSRV
job). **No dependency.** Nothing in the workspace uses it yet, so
this round introduces the first use.

**2. configuration already exists.** Measured in
`crates/peisear-web/src/config.rs:26`–`:50`: `Config::from_env`
reads four variables and **every one of them already has a working
answer without a `.env`** —

| Variable | Without configuration |
|---|---|
| `DATABASE_URL` | defaults to `sqlite://data/app.db` |
| `BIND_ADDR` | defaults to `127.0.0.1:3000` |
| `COOKIE_SECURE` | defaults to `false` |
| `JWT_SECRET` | **generated** and persisted beside the database (`RFC 017` §4) |

**This answers `RFC 020` §5's second open question.** *"What happens
on the second run after a partial answer?"* — a `.env` that exists
but lacks a variable is **already a working configuration**, because
the default fills it. So the skip condition is **existence, not
completeness**: if a `.env` exists, or any of the four variables is
set in the environment, the wizard does not run. Asking again
because one line is missing would be the wizard surprising an
operator who has already configured the instance, which `RFC 020` §3
forbids.

**State both conditions in the code as a single predicate with its
reason**, not as two scattered `if`s.

## §3 — how this is verified, because nobody here can read the file

**`.env` and `.env.example` are behind a standing deny rule on file
tools.** Neither the dev team nor the architect can open them. That
is not an obstacle to be worked around — it is the reason this
project already has the right idiom, and it is in the repository.

`.github/workflows/env-example-walk.yml`'s own comment:

> *"the one walk nobody here can perform by reading the file. This job
> never reads `.env.example` either: it copies it, as the documented
> path instructs, and asserts on whether the server then runs — the
> job's own log is the entire answer, never a tool here opening the
> file."*

**Follow that.** A CI job that runs the binary and asserts on
**behaviour**:

| Case | Expected |
|---|---|
| stdin piped (`< /dev/null`), no configuration | the stub line **absent**; the server starts on defaults |
| stdin piped, configuration present | stub line absent; server starts |
| stdin a terminal, no configuration | stub line **present** |
| stdin a terminal, configuration present | stub line absent |

**The third case needs a pty**, which is the only genuinely new
machinery here. `script -qec` is on the GitHub runner and is the
cheapest route; if something else is simpler, say what and why.

**Two rules for the tests, both load-bearing:**

- **Rust code may read a `.env`; a tool call may not.** A test that
  needs to check a written file uses `fs::read_to_string`. Nothing in
  this round requires a human or an agent to open one.
- **Never write into the repository.** Every case runs in a temp
  directory. `.gitignore` carries `.env*` so an accident cannot be
  committed, but that is the backstop, not the plan — `§10.45` is the
  register entry for a generated secret reaching a commit.

## §4 — gates

- **All four cases in §3 green in CI**, each asserting on the log,
  and **the terminal case seen to fail** when the detection is
  reverted — the stub line must appear in the piped case with the
  check removed. A skip never seen to not-skip is not evidence.
- `DEC-007` — the delta stated. This round adds tests.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  `publish-dry-run`, CI; **`test.yml` edited means `test.yml`'s own
  gate runs.**
- **The default start is unchanged**: `peisear` with a valid `.env`,
  or with the environment set, starts exactly as it does today, with
  the same output. State the before/after startup lines.

## §5 — escalate rather than deciding

- **If a pty cannot be arranged on the runner.** Bring it. A skip
  condition proven only in the negative direction is weaker than
  this release needs, and the alternative (a unit test over an
  injected `is_terminal` flag plus a CI job for the piped cases) is a
  judgement about how much proof is enough — the architect's.
- **If `is_terminal` behaves differently under the container image**
  than under the runner. That is the case that matters most and the
  one the published artefact actually meets.
- **If the skip predicate needs a third condition.** Say which and
  why; two is what the RFC names.

## §6 — exit condition

`std::io::IsTerminal` used with no new dependency; the skip stated as
one predicate carrying both conditions and its reason; a stub wizard
body that prints one line and returns; the four cases of §3 green in
CI with the terminal case genuinely exercised through a pty, and the
detection seen to fail when reverted; every test running in a temp
directory with nothing written into the repository; the default start
byte-identical in its output; `DEC-007` reconciled with its delta
stated; all gates re-run after the last edit.
