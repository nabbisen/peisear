# FIV-001 — the key that is published

**Target**: 0.53.0, first. `RFC 0017` proposals **4** and **§5**, which
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
  **and its text needs the amendment `RFC 0017`'s review already
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
