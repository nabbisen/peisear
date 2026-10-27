# WIZ-004 — the skip that agrees with the loader

**0.58.0, added after `WIZ-003`.** The series was scoped at three
rounds and is not open to a fourth — **except for the standing
exception, which this is**: a defect that blocks the release is not a
scope question.

**The wizard tells the operator *"No configuration found"* when
configuration was found, and then silently overrides it.**

---

## §1 — measured, 2026-10-27 at `edc51ae`

A parent directory holding a `.env` that sets `BIND_ADDR=127.0.0.1:3999`;
an empty child directory; `peisear` run from the child.

**Piped stdin** (the wizard correctly skipped):

```
INFO peisear: starting peisear database=sqlite://data/app.db addr=127.0.0.1:3999
INFO peisear: listening addr=127.0.0.1:3999
```

**The parent's `.env` was loaded.** The instance is configured.

**A terminal** (the wizard ran):

```
No configuration found. Setting up a new instance.
Where should the database live? …
Written to .env
INFO peisear: listening addr=127.0.0.1:3000
```

It wrote a **second** `.env` into the child directory — and the
instance came up on **3000**, not the operator's **3999**.

**So the product asserted something false and then acted on it.** The
sentence *"No configuration found"* is the first thing this release
exists to say, and in this case it is wrong.

## §2 — the cause: detection and loading disagree about where `.env` is

| | Resolution |
|---|---|
| **Detection**, `main.rs:50` | `Path::new(".env").exists()` — **cwd only** |
| **Loading**, `config.rs:28` | `dotenvy::dotenv()` — cwd **and every ancestor** |

dotenvy's own `find.rs:32`: *"Searches for `filename` in `directory`
and parent directories until found or root is reached."*

`main.rs`'s own comment states the assumption that fails: *"the second
call finds the file this one just wrote, **in the same cwd**"*. True
of the file the wizard writes; not true of the file it was looking
for.

## §3 — the fix, and a correction to the architect's own review

**Ask the loader.** `dotenvy::dotenv()` returns
`Result<PathBuf>` — the path it actually loaded. Resolve
configuration's existence with the same mechanism that consumes it,
rather than a second, narrower guess at where it lives.

The straightforward shape: call `dotenvy::dotenv()` **once, at the
top of `run()`, before the detection**; treat `Ok(_)` as *a `.env`
exists* (and keep the four `env::var` checks for the
environment-only case). `Config::from_env`'s own call then finds the
same file, or the one the wizard just wrote, and is harmless either
way.

**`WIZ-001`'s review praised the opposite ordering, and was wrong.**
It said running the detection *before* `from_env` mattered because
otherwise *"the four `env::var` checks would see `.env`'s own values
as set in the environment and the predicate would be circular."*
**There is no circularity.** `configuration_exists` is an OR over
*"a `.env` exists"* and *"a variable is set"*; if loading the `.env`
sets the variables, the OR is true for the same reason, not a
spurious one — the two signals collapse into one correct answer.
What the early ordering actually bought was a detection narrower
than the loader, which is this defect.

**Whichever shape is chosen, the property to hold is one sentence**:
*the wizard runs exactly when `Config::from_env` would find no
configuration.* Write it in the code as that sentence.

## §4 — gates

- **The case in §1, as a CI case**: parent `.env`, empty child, run
  from the child on a pty — **the wizard must not run**, and the
  instance must come up on the parent's own `BIND_ADDR`. That last
  clause is what makes it a real test rather than an absence check:
  binding 3999 proves the parent's file was both found *and* obeyed.
- **`WIZ-001`'s four existing cases still green**, unchanged.
- **Seen red**: revert the fix, watch the new case fail. The plant is
  the whole point; `WIZ-001` round 2 is the standard.
- `DEC-007` delta stated.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`,
  CI; **`test.yml` edited means `test.yml`'s own gate runs.**
- **No tool call opens any `.env`.** The parent file is created by
  the step with `printf`, and every assertion is behavioural — the
  bound address, the server answering. `env-example-walk.yml`'s own
  idiom, and the deny rule blocked exactly such a read during this
  review.

## §5 — escalate rather than deciding

- **If `dotenvy::dotenv()` at the top of `run()` changes anything
  about a configured start** — ordering, precedence, a variable that
  now wins where it did not. Stop and bring it; §2's claim is that
  the two resolutions should agree, not that startup should change.
- **If there is a case where the wizard should run despite an
  ancestor `.env`** — someone deliberately setting up a nested
  instance. Bring it; the architect's position is that *"No
  configuration found"* must never be printed when a `.env` will be
  loaded, and a nested instance is better served by an explicit
  answer than by a false sentence.

## §6 — exit condition

Detection and loading resolved by the same mechanism, with the
property written in the code as one sentence; the §1 case in CI,
asserting both that the wizard did not run and that the parent's
`BIND_ADDR` was obeyed; that case seen red with the fix reverted;
`WIZ-001`'s four cases still green; no `.env` opened by a tool call;
`DEC-007` reconciled; all gates re-run after the last edit.
