# SEC-001 — what leaves the process, and what the operator knows

**Three items from `RFC 0018` §2's security assessment**, plus one rename
and one durability note carried from `STORE-002`'s review. **All of them are
about information leaving the process, or about an operator not being told
something true.**

**Reviews**: `.git-exclude/reviewed/STORE-002-review.md` (§3, §4).
**Target**: 0.50.0, third round.

---

## §1 — the DSN in the startup log

`crates/peisear/src/main.rs:22-27` logs
`database = %config.database_url` at `INFO`, **every start**, and **nothing
in the workspace redacts it** (searched: no `redact`, no custom `Display`).

Today it is a file path and harmless. **The day `RFC 0018` P3 lands it is
`postgres://user:password@host/db`** — in journald, in any log shipper, in
every support paste an operator sends you.

**Redact at the source, not at the call site.** A helper beside `Config`
that returns a loggable form — **scheme, host and path; userinfo dropped
entirely** — so a future second log line cannot reintroduce the leak by
forgetting. Keep the information that is useful in a startup line (which
database, where) and drop only the part that is a credential.

**Test it with a URL that has a password in it**, not only with today's
`sqlite://data/app.db`. A redactor only exercised on input that has nothing
to redact is `§10.36` again.

## §2 — the secret that is one `{:?}` away

`crates/peisear-web/src/config.rs:5-11` is
`#[derive(Clone, Debug)]` over a struct whose second field is
**`pub jwt_secret: String`**.

**Nothing debug-prints it today** — I grepped `{:?}` and `debug(` across both
crates and found no `Config` or state formatting. **This is a latent hazard,
not a live leak**, and that is exactly when it is cheap to close: one
careless `tracing::debug!(?config)` during a future investigation puts the
session signing key in a log file.

**Write `Debug` by hand**, printing `jwt_secret` as a fixed placeholder and
`database_url` through §1's redactor. **Check `AppState` too** — it carries
`jwt_secret` as well, and whatever it derives is the same question.

**A test that asserts the secret's own value does not appear in the
formatted output**, with a recognisable sentinel secret, so the guard is seen
to work rather than assumed.

## §3 — data at rest, in the threat model

`docs/security/` is **194 lines across two files** and contains **zero
occurrences of "encrypt", "at rest" or "disk"**.

`hardening.md:11-38`'s threat model takes three threats seriously — external
attackers on the HTTP endpoint, local users on a shared host, a lost session
token — and **the database file is not among them**, nor are its backups.
*What the operator does* (`:126-175`) covers TLS, `JWT_SECRET` hygiene,
security headers, rate limiting, OS hardening and backups, and **not
encryption**.

**For a product whose defining constraint set is personal-data privacy
(`NFR-PRIV-001..008`, Met since 0.37.0), that is the asset most worth naming
and the only one unnamed.**

Add it, in that document's own voice:

- **The asset**: `data/app.db`, its WAL and `-shm` siblings, and every
  backup taken from it. **Anything that can read the file has every user's
  personal sustainability data**, which `NFR-PRIV-001` otherwise restricts
  to its subject.
- **The control**: **volume encryption** (LUKS, dm-crypt, the cloud volume).
  Say plainly that **peisear does not encrypt its own storage** — an
  operator who reads that page end to end currently has no reason to know
  it.
- **Declined, on the record** (owner, 2026-10-11): database-level
  encryption. SQLCipher is a driver-stack change under `sqlx`; column-level
  encryption would break the comparisons the personal metrics are computed
  from. **And the answer does not change with PostgreSQL** — core PostgreSQL
  has no transparent data encryption either, so volume encryption is the
  control on both.

**Do not overstate it.** This section documents a gap and its mitigation; it
is not a claim that peisear protects data at rest.

## §4 — the durability guarantee, where an operator reads

`STORE-002` chose `synchronous = FULL` and explained it **in `pool.rs`**,
which is the right place for a maintainer and the wrong place for an
operator.

**One short entry under operations** — `backup.md` or `deployment.md`,
whichever you judge the better home, and say which and why: peisear fsyncs
the WAL **on every commit**, so a committed change survives a power cut; the
measured cost was ~2 ms on a ~440 ms write path on NVMe, **and on storage
with slow fsync the trade is different**.

**It is not configurable, deliberately.** An environment variable for
`synchronous` would be *a lever that can silently disable a durability
guarantee*, the same shape as the overflow gate's fixture override and
declined for the same reason. **Say that too** — an operator who wants the
knob should find the reason it is absent, not the absence.

## §5 — the rename from `STORE-002`'s review

`crates/peisear-storage/src/store002_guards.rs` → **`optimistic_lock_guards.rs`**,
and the one `mod` line with it. It is the only ID-named module in the
workspace; its eighteen siblings are named for their subjects.

## §6 — gates

- **`DEC-007` will move** — §1's and §2's tests. **Expect 430; say the
  number and name each addition.** §5's rename must not change it.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken).
- **`mdbook build` with linkcheck** — §3 and §4 add prose to documents the
  book serves.
- **The three browser gates are not required** unless you touch something
  they read; say which you ran and why.
- **No migration.** The candidate's database check stays at **21 with one
  applied**.
- **Read the startup output once, by eye**, with a password-bearing
  `DATABASE_URL` set, and confirm what lands in the terminal.

## §7 — escalate rather than deciding

- **If redacting the DSN would remove information the startup line needs**
  to be useful — that is a trade and it is mine.
- **If `AppState` or anything else turns out to already print a secret.**
  That is a live leak, not a latent one, and it changes this round's
  priority.
- **If `hardening.md`'s threat model cannot absorb §3 without restructuring
  it** — say so rather than reshaping a security document on your own
  judgement.

## §8 — exit condition

A redactor with a password-bearing test; a hand-written `Debug` proven by a
sentinel; data at rest named in the threat model with volume encryption as
its control and the declined alternatives on the record; the durability
guarantee written where an operator reads, with its absence of a knob
explained; the module renamed; `DEC-007` stated with each addition named;
`mdbook build` clean.
