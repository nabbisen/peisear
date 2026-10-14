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

## §9 — the documented path still installs a published credential

`Config::from_env` calls `dotenvy::dotenv()`. The repository ships
`.env.example` with an **uncommented**
`JWT_SECRET=change-me-to-a-long-random-string`. Three documents tell
the reader to copy it: `README.md:64`, `.github/CONTRIBUTING.md:30`,
and `docs/getting-started/configuration.md:12`, whose *Minimum viable
`.env`* section this round rewrote to read **"That's it — no secret to
generate by hand."**

So the documented path sets `JWT_SECRET` to a published literal, the
generated branch never runs, and the sentence promising otherwise is
false. **Round 1 also deleted `README.md`'s `openssl rand -base64 48`
lines** — the instruction that previously told the reader to replace
the placeholder. The cause was left and the mitigation removed.

**Two changes, and they are not alternatives.**

1. **`.env.example`**: comment the `JWT_SECRET` line out, per the diff
   round 1 already wrote. **This is the owner's to apply** — every file
   tool in this project denies `.env*` paths, which round 1 reported
   honestly. Do not attempt to work around that rule.
2. **Refuse the placeholder at startup.** If `JWT_SECRET` resolves to
   the published literal, **fail with the same shape as the other
   `ConfigError` variants**, naming the remedy (unset it and let the
   key be generated). This is the half that reaches an instance
   *already running* from a copied `.env`, which the file edit cannot.
   `§10.40`'s own reading is why one rejected constant in source is not
   a credential: *a value refused is not a credential, only a value
   used is.* Say that in the code comment, so the next reader does not
   "fix" it by deleting the constant.

**Not a list.** One literal, because exactly one has ever been
published. If a second is ever tempting, that is a different design and
a different round.

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

## §12 — exit condition

`.env.example`'s line commented out (owner-applied) **or** reported
still pending with the status left `Divergent` — never a status moved
ahead of the tree; the placeholder refused at startup with a test for
it; `ConfigError`'s `Display` reaching the terminal, shown by pasting
five real runs' actual output; the temp file created `0600` and
`create_new`; `§10.40` closed and `NFR-SEC-004` re-statused **only
once both halves are true**; `DEC-007` accounted for against round 1's
baseline.
