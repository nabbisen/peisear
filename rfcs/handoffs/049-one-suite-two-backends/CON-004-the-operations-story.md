# CON-004 — the operations story

**0.56.0, `RFC 018` P4, round 4 of 4.** Not blocked on the other
three in code, but written last so it can describe what they
actually shipped rather than what they intended to.

`RFC 018` P4's row has two halves. The first three rounds are the
conformance suite. This is the other half: *"the operations story
(backup is no longer `cp` on PostgreSQL)"*, the documentation, and
the gate's own closing clause — ***"`NFR-CMP-003` restated to say
for the SQLite backend."***

---

## §1 — `docs/operations/backup.md` is wholly SQLite

Read 2026-10-20. Every operative instruction in it assumes a file:

| Section | What it says |
|---|---|
| Online backup | `sqlite3 … ".backup …"` |
| Cold backup | `cp -r /var/lib/peisear/data/` |
| The wrong way | do not `cp` the live `app.db` |
| Restore | `cp` the db and the key file back |
| Offsite rotation | a cron job built on `.backup` |
| Durability of a single write | `synchronous = FULL`, WAL |

None of it is wrong. All of it is about one backend, and the
document never says so. A PostgreSQL operator reading it finds no
sentence addressed to them and no sentence telling them this is not
addressed to them — **which is the confusion the owner's philosophy
puts ahead of cost.**

What a PostgreSQL section needs: `pg_dump` and its restore; what
"the backup unit is the directory" becomes when there is no
directory; and the fact that the server's own WAL and point-in-time
recovery are the operator's to configure, not peisear's — stated
once, so the reader is not left expecting peisear to have an
opinion it does not have.

## §2 — the fact nothing documents: `JWT_SECRET` is mandatory on PostgreSQL

Measured by reading `crates/peisear-web/src/config.rs` at `c98739a`:

- `database_file_path` (`:225`) returns `None` for any URL that is
  not `sqlite://` with a real path.
- `resolve_generated_jwt_secret` (`:249`) then returns
  `ConfigError::NoFilesystemPathForKey`.

So with `DATABASE_URL=postgres://…` and `JWT_SECRET` unset,
**peisear refuses to start.** The error message is good — it names
the URL and says *"Set JWT_SECRET explicitly"* — and the refusal
itself is right: inventing a home for a signing key the operator
never asked for would be worse.

**But no document says it in advance.** Grepped across `docs/` and
`README.md`: not one line mentions PostgreSQL and the signing key
together. And the story those documents *do* tell — `RFC 017` §4's
generated key beside the database, `NFR-CMP-003`'s 0.53.0 amendment
making *the directory* the backup unit — is written for a file
beside a database file, which PostgreSQL does not have.

This is a first-five-minutes fact for every PostgreSQL operator and
it is currently learned by the server exiting. It belongs in the
documentation this round writes, next to the connection string, not
only in the error.

**Note on reading `.env.example`:** that file is covered by a
standing deny rule and the architect has not read it in preparing
this handoff. If the configuration documentation you write needs to
change it, change it — just do not assume anything stated here
describes its current contents.

## §3 — `NFR-CMP-003`, restated

Current text (`requirements.md:3699`): *"Persistent state MUST
reside in a single SQLite file…"*, Implemented, P2, with amendments
at 0.51.0 (the `.backup`/`cp` distinction) and 0.53.0 (the backup
unit becomes the directory).

The gate's words are *restated to say **for the SQLite backend***.
That is the whole change and it is deliberately small:

- **Scope it, do not weaken it.** Everything the entry currently
  requires stays required, of the SQLite backend. The `.backup`
  versus plain-`cp` distinction is the 0.51.0 amendment's whole
  subject and `§10.x` has a precedent about restating a correction
  loosely.
- **Then answer the question scoping it opens**: what, if anything,
  is required of the PostgreSQL backend's persistent state? *"The
  operator's own instance, backed up by the operator's own means,
  and `backup.md` MUST say so"* is a defensible answer. *Nothing at
  all* is also defensible for a backend no release ships a binary
  for. **Silence is not** — a requirement that says "SQLite does X"
  and is mute about the other backend reads as an oversight.
  Propose one; it is a requirement question, so bring it rather
  than deciding it.

Two documents lean on this entry and `DEC-028`'s five-pass sweep
will reach both; they are named here so they are not rediscovered:

- `docs/specification/external-design.md:210` — the
  external-entities table, already corrected once at 0.51.0 for
  restating this entry imprecisely.
- `docs/security/hardening.md:163` — *"`NFR-CMP-003`'s …"*, the
  key-file-with-the-database claim.

## §4 — gates

- `mdbook build`, and the new sections **read back as an operator**,
  not as an author — the discipline `DEC-028`'s pass 5 uses.
- Every command written into `backup.md` for PostgreSQL has been
  **run**, against the CI service or a local server, and its output
  seen. An untested `pg_dump` invocation in an operations document
  is the same defect class as a documented install path nobody
  walked (`§10.41`).
- `DEC-028`'s five passes, with Appendix A **recomputed from its own
  rows** — never adjusted by delta.
- `docs_scan` and `changelog_scan` green.
- `DEC-007` unchanged: this round should add no test. If it adds
  one, say why.

## §5 — escalate rather than deciding

- **The `NFR-CMP-003` sibling question in §3 is the owner's.**
  Propose, do not decide.
- **If writing the PostgreSQL backup section turns up a property
  peisear does not actually have** — some assumption in the code
  about a single file — stop. That is a finding about the backend,
  not about the document.
- **If the `JWT_SECRET` refusal looks like it should be fixed
  rather than documented.** It might be; `RFC 020`'s first-run
  wizard is 0.58.0 and is where that would live. Not here.

## §6 — exit condition

`backup.md` carrying a PostgreSQL section whose every command has
been run, and saying plainly which sections are about which backend;
the mandatory-`JWT_SECRET`-on-PostgreSQL fact documented where an
operator meets it rather than only in the error; `NFR-CMP-003`
scoped to the SQLite backend with nothing it required dropped; the
PostgreSQL sibling question proposed to the owner rather than
answered; `external-design.md:210` and `hardening.md:163` reconciled;
`DEC-028`'s five passes complete with Appendix A recomputed; all
gates re-run.
