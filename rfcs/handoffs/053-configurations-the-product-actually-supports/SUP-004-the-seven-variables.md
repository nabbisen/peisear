# SUP-004 — the seven variables

**0.59.0, fourth of four. Blocked on nothing** — independent of the
PostgreSQL rounds and may run in parallel with any of them.

`RFC 021` §6. Condition (2) alone: the email channel is reachable from
the default install and needs no install path of its own.

## §1 — measured, 2026-10-29

Seven `SMTP_*` variables are read by the code:

```
SMTP_HOST   SMTP_USER   SMTP_PASSWORD   SMTP_FROM_ADDRESS
SMTP_PORT   SMTP_FROM_NAME   SMTP_TLS_MODE
```

The startup warning names **four** of them
(`peisear-notify/src/config.rs:149`). Operator-facing documentation
names **zero** — the only hit outside `peisear-notify`'s own rustdoc
is in `requirements.md`, describing `docs_scan`'s rule rather than
telling an operator anything.

**`SMTP_TLS_MODE` is the one that makes this more than prose.** A
transport-security mode an operator cannot read about is one they
cannot set deliberately; they get whatever the default is and no way
to know whether it is what their mail provider requires.

## §2 — what to build

**All seven in `configuration.md`**, where configuration is
documented, each with its default and what it does. For
`SMTP_TLS_MODE`, the accepted values and what each one means, read
from the code rather than from what the name suggests.

**Fix the warning too.** It names four variables as though they were
the set, which teaches the subset: an operator who sets exactly those
four and finds the channel still wrong has been misled by the one
message the product does volunteer. Either name all seven, or name
none and point at the documentation — **the second is probably
better** and is this round's own judgment to make and justify: a
startup warning that lists variables will drift from the code again,
and `§10.36`'s question applies to a message as much as to a check.
Whichever you choose, say why.

## §3 — the guard that will object, and the right response

`docs_scan` rule 4 reads documented defaults against `config.rs`.
`SMTP_PORT`'s default lives in `peisear-notify`, not `config.rs`, and
rule 7 already knows that (`docs_scan.rs` names
`peisear-notify`'s own `SMTP_PORT` default explicitly).

**Check whether rule 4's population reaches these seven at all**, by
running it, not by reading it. If it does not, that is a finding to
report — a documented default no guard compares to the code is the
drift this project keeps finding — but **widening the rule is not this
round's work.** Say so and leave it.

## §4 — what this round may not do

- **It may not add `SMTP_*` keys to `.env.example`.** The file is
  commented at every built-in default by design (`0.53.0`), and
  rule 7 refuses an uncommented key with no default to compare.
  Documentation is the deliverable, not a template.
- **It may not change what the channel does.** No new variable, no
  changed default, no behaviour. If a default looks wrong, report it.

## §5 — gates

`DEC-007` re-extracted and counted with the runner. `docs_scan`,
`changelog_scan`, `spec_scan` green. `fmt`, `clippy`, rustdoc with
`CONTRIBUTING.md`'s own flag, `mdbook build docs` with linkcheck, all
after the last edit, quoted. If the warning's text changes,
`prose_scan`'s population question is the same one `TTY-001` answered
by running it rather than reading its doc comment.

## §6 — escalate rather than deciding

- If a variable's real behaviour differs from what its name implies,
  so documenting it accurately would read as a defect report.
- If `SMTP_TLS_MODE`'s accepted values turn out to be unclear in the
  code itself.

## §7 — exit condition

1. All seven documented in `configuration.md` with defaults, and
   `SMTP_TLS_MODE`'s values explained.
2. The startup warning no longer teaches a subset; the choice made
   and justified.
3. Rule 4's reach over these seven checked by running it, and stated
   either way.
4. `.env.example` unchanged; no behaviour changed.
5. The gates in §5, re-run after the last edit, quoted.
