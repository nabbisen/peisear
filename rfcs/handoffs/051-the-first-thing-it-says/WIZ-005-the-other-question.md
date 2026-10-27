# WIZ-005 — the other question

**0.58.0, added after `WIZ-004`.** Same standing exception: a defect
that blocks the release is not a scope question.

**`WIZ-002` round 4 validated the database answer and re-asks on a bad
one. The address answer — the other of the two questions — is not
validated at all.** Four of the most natural answers write a `.env`
that cannot boot, and the wizard then never offers to fix it.

---

## §1 — measured, 2026-10-27 at `0ed2d98`

Answered the address question; held enter for the database:

| Answer | Result |
|---|---|
| `localhost:3000` | `Error: invalid socket address syntax` |
| `3000` | `Error: invalid socket address syntax` |
| `0.0.0.0` | `Error: invalid socket address syntax` |
| `127.0.0.1` | `Error: invalid socket address syntax` |

**Not typos. These are the four answers a person is most likely to
give** when shown `[127.0.0.1:3000]`: the name instead of the
address, the port alone, the publish-to-the-network value without a
port, and the shown address without its port.

**And the wizard announces success first.** Run 1, verbatim order:

```
Written to .env — edit it any time, or delete it to start over.
Error: invalid socket address syntax
```

**Then it locks itself out.** `.env` now exists, so
`configuration_exists` is true and the wizard is skipped:

| Run | Result |
|---|---|
| 2, on a terminal | `Error: invalid socket address syntax` — **no prompt** |
| 3, non-interactive | `Error: invalid socket address syntax` |

So a wizard whose whole purpose is to produce a working
configuration produces one that cannot start, and **the skip
condition that correctly protects containers guarantees the operator
is never offered a chance to correct it.** The only way out is to
know to edit or delete `.env` — *"read the documentation"*, reached
through the wizard rather than instead of it.

## §2 — the fix is the one `WIZ-002` round 4 already wrote, for the other question

**Validate the address answer before it becomes part of `Answers`,
and re-ask with the reason** — the same shape, the same loop, the
same in-memory plants.

The predicate is `std::net::SocketAddr::from_str` — the exact parse
`main.rs:114` performs, so the wizard refuses precisely what startup
would refuse, and nothing else. **Restating an existing refusal at
the prompt rather than inventing a second rule** is what round 4 did
for the database answer and is the reason that fix was right.

**Say the expected shape in the refusal**, because three of the four
failures above are a *form* mistake rather than a wrong value: an
address and a port, with the address numeric. Name both parts.

**Do not resolve a hostname.** `localhost:3000` is the most likely
answer and the most tempting thing to accept, and accepting it would
mean `ToSocketAddrs`, DNS at bind time, and a `BIND_ADDR` that no
longer means what it has always meant. **Refuse it and say what to
type instead** — `127.0.0.1:3000` keeps it private, which the prompt
already says.

**Do not silently complete a bare port either.** `3000` → choosing an
interface on the operator's behalf is exactly the guess that a wizard
about network exposure must not make. Re-ask.

## §3 — one secondary item, smaller and worth doing here

**When `BIND_ADDR` comes from `.env`, the startup error does not say
so.** `Error: invalid socket address syntax` names neither the
variable nor the file. For an operator who has a broken `.env` from
any source — this wizard, a hand edit, a copied file — that error is
a dead end.

Make it name the variable and its value, in the style
`config.rs`'s own `ConfigError` variants already use (`EmptySecret`,
`PublishedPlaceholder` and `NoFilesystemPathForKey` each name what
was wrong and what to do). **This is the half of the defect that
survives even after §2** — §2 stops the wizard writing a bad value;
§3 helps the operator who has one for any other reason.

## §4 — gates

- **The four answers in §1 planted in-memory**, via `run_wizard`'s
  generic `BufRead`/`Write`: each refused, each naming the expected
  shape, and the re-ask accepting a corrected second answer in the
  same run.
- **A CI case that proves the end state**: a terminal, a bad address
  answer followed by a good one, and `/health` answering on the
  corrected address — the address is the proof, as `WIZ-004`'s case 5
  established.
- **Seen red**: revert the validation, watch a planted case fail.
- `WIZ-001`/`WIZ-004`'s five cases still green, unchanged.
- §3's error message exercised by a test that sets `BIND_ADDR` to an
  unparseable value and asserts the message names the variable.
- `DEC-007` delta stated; `fmt`, `clippy` both ways, `rustdoc`,
  `mdbook`, `publish-dry-run`, CI; `test.yml`'s own gate if touched.
- **No tool call opens any `.env`.** Every assertion behavioural.

## §5 — escalate rather than deciding

- **If accepting `localhost:3000` by resolution looks right after
  all.** Bring it; it changes what `BIND_ADDR` means and that is a
  requirement question, not a prompt detail.
- **If `SocketAddr::from_str` refuses something startup accepts, or
  accepts something startup refuses.** The two must be the same
  predicate; if they cannot be, say why.
- **If §3's error change touches a `ConfigError` variant's own
  text** that a test or document quotes. Check before editing.

## §6 — exit condition

The address answer validated by the same parse startup performs,
with a refusal naming both the address and the port and a re-ask in
the same run; no hostname resolved and no bare port completed; the
four answers of §1 planted in-memory and seen red with the
validation reverted; a CI case proving the corrected address is the
one bound; `BIND_ADDR`'s own startup error naming the variable and
its value; the five existing skip cases unchanged and green;
`DEC-007` reconciled; no `.env` opened by a tool call; all gates
re-run after the last edit.
