# FIV-001 — the key that is published

**Target**: 0.53.0, first. `RFC 017` proposals **4** and **§5**, which
the RFC couples deliberately: §4 alone leaves a newcomer's laptop
reachable from the LAN, just no longer with a known key; §5 alone still
ships a known key to anyone who can reach the loopback port.

**This round closes `§10.40` and moves `NFR-SEC-004` off `Divergent`.**
It is the reason the release is ordered the way it is: the other three
rounds make it easier to start peisear, and starting peisear is
currently what puts a published signing key into use.

---

## §1 — what is actually there

`crates/peisear-web/src/config.rs:26-32`, verified against the source
rather than the RFC's paraphrase:

```rust
let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
    tracing::warn!("JWT_SECRET is not set; falling back to a development-only default. ...");
    "dev-only-insecure-change-me".to_string()
});
```

and `config.rs:34`:

```rust
let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
```

Together: an instance started with no configuration serves every
interface and signs its sessions with a string anyone can read in the
repository. The only notice is one `warn!` in a log.

## §2 — the key

**Generate once, persist, never fall back.**

1. `JWT_SECRET` set in the environment **wins**, unchanged. This is not
   a migration for anyone who configured the variable.
2. Otherwise read the key file. **If it exists and is well-formed, use
   it.**
3. Otherwise generate **32 random bytes** from a CSPRNG, write them,
   and use them.
4. If the file cannot be written, or exists and is **not** well-formed,
   **fail with a message naming the path**. Never fall back to a
   constant. Never regenerate over a damaged file.
5. **Delete `"dev-only-insecure-change-me"` from the source.** The
   round is not done while the literal exists.

**Where the file goes, and this is the part most likely to be got
wrong.** The RFC says *"beside the database"*. `DATABASE_URL` defaults
to `sqlite://data/app.db` but an operator may point it anywhere —
**derive the key's path from the resolved database path, not from a
fixed `data/`**. A key written to `data/` while the database lives in
`/srv/peisear/` is a key outside the directory the operator backs up,
and it will be discovered the first time someone restores.

**If the database is not a file** (`sqlite::memory:`, or any URL with no
filesystem path), there is nothing to sit beside: **require
`JWT_SECRET` and fail with that message.** Say so in the error, not
only in the documentation.

**Three implementation notes from the security review (2026-10-11),
carried verbatim because each is a way to get this subtly wrong:**

1. **Atomic write.** Write to a temp file in the same directory and
   rename into place. Two processes starting concurrently — a restart
   loop, a crashed first boot — must not be able to read a
   partially-written key.
2. **A corrupt or wrong-length file fails loudly, not silently
   regenerated.** Silent regeneration invalidates every session without
   the visible event this change exists to guarantee.
3. **`mode 0600` is POSIX-only.** Proposal 1 ships a Windows binary and
   Rust's `std::fs` permission bits do not restrict access on Windows
   the way they do on Unix. Either use the Windows ACL equivalent or
   **document it as a known platform difference** — what is not
   acceptable is the phrase "mode 0600" silently meaning nothing on one
   of five shipped targets. **Say which you did and why.**

**Testability.** `Config::from_env()` reaching the real filesystem is
what makes this hard to test; `SEC-001` already put three tests in
`config.rs`. Factor the key resolution as a function over an explicit
directory (or an injected reader/writer) so the five cases above are
unit-testable without a real `data/`, and test **all five**, including
both failure paths. A failure path with no test is the one that will
be wrong.

## §3 — the bind

Default `BIND_ADDR` to **`127.0.0.1:3000`**.

**It is a behaviour change** for anyone relying on the implicit default
to reach their instance from another machine. Per the RFC: **a
changelog entry with a named remedy** — set `BIND_ADDR=0.0.0.0:3000` —
not a silent correction.

`docs/security/hardening.md` was corrected at `61e7366` to say the
default is `0.0.0.0`. **That correction is true until this lands and
must be revised with it, in the same commit.** A security document that
describes the previous release's default is worse than one that says
nothing.

## §4 — documentation that must move with the code

- **`docs/security/hardening.md`**: the bind default, and the
  `JWT_SECRET` hygiene section. That section currently tells the reader
  to *"keep it out of source control"* and **never says what happens if
  the variable is unset** — which was the project doing the opposite of
  its own instruction. It must now describe the generated key: where it
  lives, that it is created on first run, that it must be backed up
  with the database, and that losing it logs everyone out.
- **`docs/getting-started/configuration.md`**: `JWT_SECRET` and
  `BIND_ADDR` defaults. **`docs_scan` rule 4 reads this table against
  `config.rs`** — a changed default with an unchanged table is a test
  failure, which is the guard working. Expect it; do not work around
  it.
- **`docs/operations/backup.md`**: the backup unit. It currently tells
  the operator to keep `JWT_SECRET` with their environment variables;
  for a generated key the file is part of what a backup must capture.
- **`README.md`'s Quick Start**: the `openssl rand` line is barrier 4
  in the RFC's own table. It goes.

## §5 — specification work (`DEC-028`)

- **`NFR-SEC-004`** moves off `Divergent` once the literal is gone —
  **and its text needs the amendment `RFC 017`'s review already
  named**: it reads *"Deployment secrets are supplied by the
  environment"*, and after this round "generated and persisted locally,
  with the environment variable taking precedence when set" is a
  second, equally valid path. Amend the text, then set the status. In
  that order: a status set against unamended text is the ambiguity
  `§10.40` is about.
- **`§10.40` closes**, with what the fix was and with the reading it
  states left intact.
- **`NFR-CMP-003`** says persistent state resides in a single SQLite
  file. A key file beside it is not application state, but it is state
  a restore needs. **Amend it to say what the backup unit now is** —
  the RFC's own words are *"the backup unit becomes the directory, not
  the file"*. Keep the `.backup`-while-running distinction `0.51.0`
  added; it is about the database and is unaffected.
- **Appendix A**: the Security row returns to `7 | 7`, and the
  non-functional totals with it.

## §6 — gates

- `DEC-007`: block extracted **fresh**. State `0.52.0`'s baseline
  (**451**) and the delta; new tests are expected and must be
  accounted for one by one.
- `fmt`, `clippy -D warnings`, `cargo doc` with broken-intra-doc-links
  denied, `mdbook build` with linkcheck.
- **`docs_scan` rule 4 must be green at the end and must have been red
  in between.** If it never went red, the configuration table was not
  the one the code reads, and that is a finding.
- **A real first run, twice.** Delete `data/`, start the binary with no
  environment at all: it comes up, the key file exists with the
  intended mode, and a session works. Stop it, start it again: **the
  same session is still valid** — that is the "sessions survive
  restarts" claim, and it is only true if the key was read rather than
  regenerated.
- **A real first run with the key file damaged**: truncate it to three
  bytes and start. It must fail, name the path, and **not** regenerate.
- **A real first run with `JWT_SECRET` set**: no key file is created.
- No migration expected.

## §7 — escalate rather than deciding

- **If the key cannot be placed beside the database** for a
  `DATABASE_URL` shape not considered here. Report the shape; do not
  invent a fallback location.
- **If the Windows permission story cannot be made honest** — neither
  an ACL equivalent nor a documented difference. That is a decision
  about what the project claims on a platform it ships, and it is the
  architect's.
- **If `NFR-CMP-003`'s amendment starts to look like a requirement
  change rather than a restatement.** The single-file database is a
  product property an operator chose peisear for; if adding a key file
  reads as weakening it, say so and stop rather than softening the
  wording.

## §8 — exit condition

The literal gone from the source; the key generated, persisted beside
the resolved database path, atomic, failing loudly on both failure
paths, with the Windows difference either handled or documented in
words; `BIND_ADDR` defaulting to loopback with the changelog entry
naming the remedy; `hardening.md`, `configuration.md`, `backup.md` and
`README.md` moved with it in the same round; `NFR-SEC-004` amended then
re-statused, `§10.40` closed, `NFR-CMP-003` amended, Appendix A
reconciled; `DEC-007` accounted for against 451; and **five real runs**
— clean first run, restart, damaged key, `JWT_SECRET` set, and a
non-file `DATABASE_URL` — each reported with what it printed.

---

# FIV-001 round 2 — the second door

**Added by the architect at round 1's review, 2026-10-14.** Round 1's
code is accepted: the literal is deleted, the key is generated,
persisted beside the resolved database path, atomic, and fails loudly
on both failure paths. **`§10.40` is reopened and `NFR-SEC-004` is back
to `Divergent`** for something round 1 did not close and partly made
worse. This round closes it. **`FIV-002`–`FIV-004` are not blocked by
it** — they are blocked by round 1, which is accepted.

## §9 — the documented path installs a placeholder nothing ever asks you to replace

**Rewritten twice on 2026-10-14, both times by the owner. This is the
version to build.** Round 1 asked for a refusal; the architect's review
downgraded it to a warning over upgrade breakage; the owner answered
*"Refuse is acceptable. You don't have to consider on migration now,
for there is no production use yet."* **It refuses.**

`Config::from_env` calls `dotenvy::dotenv()`. The repository ships
`.env.example` with an **uncommented**
`JWT_SECRET=change-me-to-a-long-random-string`. Three documents tell
the reader to copy it: `README.md:64`, `.github/CONTRIBUTING.md:30`,
and `docs/getting-started/configuration.md:12`, whose *Minimum viable
`.env`* section this round rewrote to read **"That's it — no secret to
generate by hand."**

So the documented path sets `JWT_SECRET`, the generated branch never
runs, and the sentence promising otherwise is false. **Round 1 also
deleted `README.md`'s `openssl rand -base64 48` lines** — the
instruction that previously told the reader to replace the
placeholder. The cause was left and the mitigation removed.

**What class of defect this is.** The owner's ruling, 2026-10-14: *"I
don't think it is a serious issue. Just design issue."* **A placeholder
the user actively installs is a usability defect, not a credential in
source** — unlike the deleted literal, which was used silently by a
user who did nothing. `§10.40` records why that distinction settles an
ambiguity the entry says was its root cause. **The class changed; the
remedy did not get weaker.**

**Why refusal rather than a warning, now that the objection is gone.**
The objection was that refusing fails an instance that ran before the
upgrade. **There is no production use yet**, so that cost is
hypothetical and the simpler rule wins: a refusal is binary and cannot
be learned-around, where a warning that fires on every start is a
warning an operator stops reading. It also matches `§10.40`'s own
reading — *a value refused is not a credential, only a value used is* —
so the entry closes cleanly instead of closing on a judgement about
how loud a log line is.

**Three changes.**

1. **Refuse to start when `JWT_SECRET` resolves to the published
   placeholder.** A new `ConfigError` variant, failing with the same
   shape as the others, **naming the remedy**: unset the variable and
   let the key be generated. Name the consequence too — that this value
   is published in the repository and anyone can forge a session with
   it. §10's fix is what makes this message actually reach the
   terminal; without it this variant prints as a struct dump.
2. **`.env.example`**: comment the `JWT_SECRET` line out. **The
   owner's to apply** — every file tool here denies `.env*`; do not
   work around it. **This is now release-gating, and §9b says why.**
3. **Put the `openssl rand` guidance back somewhere it belongs**, or
   say why it is no longer needed now that the key generates itself.
   Round 1 removed it as obsolete, and it was obsolete *for the path
   where generation runs*. Decide and state which.

**Not a list.** One literal, because exactly one has ever been
published. If a second is ever tempting, that is a different design and
a different round.

**Not a wizard.** The owner's other suggestion is `RFC 020`, proposed
separately and **not in 0.53.0**. Do not start one here.

## §9b — refusing makes the owner's one-line edit release-gating

**This is the consequence of choosing refusal, and it is not optional.**

With a warning, an uncommented `.env.example` meant noise. **With a
refusal it means the documented quick-start path fails outright**:
`cp .env.example .env` then `cargo run` would exit with an error, for
every new user, on the first command the README gives them — in the
release whose entire subject is a newcomer's first five minutes.

**So the two land together or neither lands.** Treat this as a release
gate, not a courtesy:

- If `.env.example`'s line is commented out, the refusal is correct and
  invisible: nothing sets the variable, the key generates, nobody ever
  meets the new error.
- If it is still uncommented when this round is ready, **say so plainly
  in the report and do not ship the refusal into that tree.** A round
  that makes the README's first command fail is worse than the defect
  it closes.

**Verify rather than assume.** The architect has never read that file —
every claim about its contents in this handoff is quoted from round
1's review request. **Walk `cp .env.example .env` and start the server**
as part of this round's gates, and report what actually happens. That
walk is the check; the file's text is hearsay until it runs.

## §10 — the error messages nobody sees

`ConfigError`'s `thiserror` messages are good — `KeyMalformed`'s names
the path, both byte counts, and the remedy including that deleting the
file invalidates every session. **A user never sees any of it.**
`main` returns `Result<(), Box<dyn Error>>`, and Rust's default
termination prints `Debug`:

```
Error: KeyMalformed { path: "data/jwt_secret", expected: 32, found: 3 }
```

Round 1's own evidence logs show this and read it as passing, because
the struct dump does technically name the path. **In the release whose
subject is output a beginner can act on, the remedy was written and
then not delivered.**

Handle the error in `main` — print `Display`, exit non-zero — rather
than returning it. Re-run all five real runs and **paste what the
terminal actually shows**, not the exit code.

## §11 — the temp file is world-readable for a moment

`write_key_file_atomically` does `fs::write(&tmp_path, bytes)` and
*then* `set_key_file_permissions`. The doc comment's claim is right
about `path` — the renamed file is never briefly world-readable — but
the **temp file holds the key at the default mode** for the window
between those two calls, in the same directory.

Create it restricted instead: `OpenOptions` with `.mode(0o600)` and
`.create_new(true)` under `#[cfg(unix)]`. `create_new` also turns a
leftover temp from a pid-wrapped earlier run into an error rather than
a silent clobber. **On Windows this stays the documented no-op** —
nothing about item 3 changes.

**Noted, not required**: two processes starting concurrently against
one empty data directory still generate two different keys, and the
later `rename` wins. Nothing reads a partial file, which is what the
security review asked for, so this is recorded rather than fixed — say
in the doc comment that last-writer-wins is the known behaviour, so the
next reader does not infer a guarantee the code does not make.

## §11b — the output the documentation still promises

**Added 2026-10-14, after round 2 was written**, from reading the
getting-started path end to end rather than from a scan.

`docs/getting-started/first-run.md:28-29` shows the reader what they
should see on a successful start:

> ```
> 2026-04-24T… INFO peisear: starting peisear database=sqlite://data/app.db addr=0.0.0.0:3000
> 2026-04-24T… INFO peisear: listening addr=0.0.0.0:3000
> ```

**Round 1 changed that default to `127.0.0.1:3000`.** A reader
following the page will not see those lines, and the page they were
sent to by `installation.md` is the one that tells them what a correct
start looks like. **This is `§10.34`'s shape** — a document saying one
thing while the release says another — in the release whose subject is
a newcomer's first five minutes.

**Round 1 swept the sibling and not this one.** `configuration.md`'s
reference table reads `127.0.0.1:3000` correctly; `first-run.md` was
missed. §4 of this handoff is titled *documentation that must move with
the code*, so the instruction existed — what was missing was a list of
the files it covered. **State the list you used this time.**

**Coordinate with `FIV-004` §2.** That round adds a plain line of
output naming the URL to open. Whatever this page shows must be what
the binary actually prints after both rounds land — so fix the address
here, and if `FIV-004` changes the shape of the output, that round
updates this block again. **Say which of the two you are leaving it to,
rather than both assuming the other.**

## §12 — exit condition

the published placeholder **refused** at startup, in a message naming
both the remedy and what the value costs the operator, with a test for
it; **`cp .env.example .env` walked and the resulting startup reported
verbatim** — and the refusal not shipped into a tree where that walk
fails (§9b); the `openssl rand` guidance restored or its removal
reasoned; `ConfigError`'s `Display` reaching the terminal, shown by pasting
five real runs' actual output; the temp file created `0600` and
`create_new`; `§10.40` closed and `NFR-SEC-004` re-statused **only
once both halves are true**; `first-run.md`'s expected-output block
showing the address the binary now actually binds, with the list of
documentation files checked for the old default stated; `DEC-007`
accounted for against round 1's baseline.
