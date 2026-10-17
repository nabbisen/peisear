# DIA-003 — the example file nobody reads

**Target**: 0.54.0. **Independent of `DIA-001`/`DIA-002`** — different
crate, different subject. Run it whenever there is capacity.

**Carried from 0.53.0**, where `.env.example` shipped two defects at
once: a published `JWT_SECRET` placeholder, and `BIND_ADDR=0.0.0.0:3000`
after the code default had moved to loopback. Both lived there because
**nothing in this project reads that file** — `docs_scan` rule 4
compares `configuration.md`'s table against `config.rs`; the example
file is in no rule's scope and no test's.

---

## §1 — the rule

**`docs_scan` rule 7**: every uncommented `KEY=value` in
`.env.example` must match `config.rs`'s own default for that key.

**An example file may document a default; it must never silently
override one.** That sentence is already written at the top of
`.env.example` — this round makes it checkable.

**A compiled test can read what a reviewer here cannot.** The
permission rule on this machine denies every `.env*` path to the
tools, and the file is public, committed, and part of the documented
start path. The scan is the only reader it can have.

## §2 — the shape, and its bounds

- **Uncommented lines only.** A commented line is documentation and
  may show anything, including a value that is not the default — that
  is what the SMTP block does and it is correct.
- **Compare against `config.rs`'s literal defaults**, the same source
  rule 4 already parses. Reuse its helpers rather than writing a
  second parser; two parsers of one file is `§10.38`'s class.
- **A key `config.rs` does not know is a failure**, not a skip —
  `RUST_LOG` lives in `main.rs` and `SMTP_*` in `peisear-notify`, so
  decide where each is read from and say so. **An unknown key silently
  passing is the hole this rule exists to close.**

**Today the file has zero uncommented lines**, so the rule passes
trivially on arrival. **That is the thing to be suspicious of**: a
rule that has only ever been green against an empty population has
been measured on zero opportunities — `§10.34`'s own phrase for why a
different rule was argued down.

**So plant content, not a file.** `docs_scan` already takes every
document through its `Input` struct as a `&str`
(`docs_scan.rs:355`); add `env_example` beside the others, and the
planted cases become ordinary unit tests:

| planted content | expected |
|---|---|
| uncommented `BIND_ADDR=0.0.0.0:3000`, `config.rs` defaulting to loopback | **fails** |
| the same line commented | passes |
| an uncommented key `config.rs` does not know | **fails** |
| the real file, via `the_repository_satisfies_the_live_rules` | passes |

*This replaces an earlier instruction to edit `.env.example` and watch
the live rule go red.* **That cannot be done here** — every file tool
is denied on `.env*`, and the owner offering to lift the rule is not a
reason to need it. The `Input` shape is better regardless: four cases
instead of one, and none of them touches a committed file. **The guard
reads what nobody here can, and these tests prove it reads it
correctly without anyone reading it.**

## §3 — what this is not

- **Not a parser for `.env` syntax.** `KEY=value`, `#` comments,
  nothing else. Quoting, multi-line values and interpolation are not
  in this file and must not be anticipated.
- **Not a check that the file is complete.** It may omit a variable
  entirely; silence is not a claim.
- **Not a reader of `.env`.** Only the committed example.

## §4 — gates

- `DEC-007` from a fresh block — **this moves**, by the rule's own
  tests. Account exactly.
- The usual four, after the last edit.
- **`§2`'s four planted cases**, each shown to do what it claims.
  **No `.env*` path is written by this round**, and none needs to be.

## §5 — exit condition

Rule 7 in `docs_scan`, reusing rule 4's own default parsing; comments
exempt and the reason written down; an unknown key failing rather than
skipping, with each key's source crate stated; **the rule seen to fail
against planted content rather than an edited file**, with all four
cases of `§2` demonstrated; no `.env*` path written; `DEC-007`'s delta
accounted for.
