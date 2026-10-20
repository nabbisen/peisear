# CON-002 — the suite that runs whole

**0.56.0, `RFC 018` P4, round 2 of 4.** Blocked on `CON-001` only
for the crates above `peisear-storage`; the storage crate's own
suite can be worked on in parallel if `CON-001` stalls.

P4's gate says *"both backends green on one suite."* Today the
PostgreSQL job runs a **list of names**, and a list of names cannot
be green about what is not on it.

---

## §1 — the census, measured 2026-10-20 at `c98739a`

`cargo test -p peisear-storage [--no-default-features --features
postgres] --lib -- --list`, counted per module:

| Module | under `sqlite` | under `postgres` |
|---|---|---|
| `dialect::tests` | 12 | — (`cfg`'d out, correctly) |
| `dialect::postgres_tests` | — | 28 |
| `migration_drift::tests` | 5 | 5 |
| `optimistic_lock_guards::tests` | 5 | 5 |
| `pool::tests` | 7 | 8 |
| `search::tests` | 2 | 2 |
| **total** | **31** | **48** |

**48 tests compile under `postgres`. CI runs 31 of them** — the 28
of `dialect::postgres_tests` by module filter, plus 3 of
`pool::tests` by exact name (`test.yml:596–604`).

**17 compile and are never run.** The 5 `migration_drift`, the 5
`optimistic_lock_guards`, the 2 `search`, and 5 of `pool::tests`.

**Their current pass/fail is not measured in this handoff.** This
machine has no container runtime and no local PostgreSQL, so the
figure could not be taken honestly here. `PG-004` reported *11 pass
/ 18 fail* against a 29-test selection; that is a different
population from the 48 above and **must not be carried forward as
if it described this one.** Measuring the 17 against the CI service
is this round's first act, before any edit.

## §2 — the second defect: a filter that matches nothing is green

Measured on this tree:

```
$ cargo test -p peisear-storage --lib a_test_name_that_does_not_exist_anywhere
test result: ok. 0 passed; 0 failed; 0 ignored; 31 filtered out
EXIT=0
```

**Exit 0.** So each of the four filtered lines in
`test-postgres-connection` stops examining anything the moment its
test is renamed, moved or deleted, and the job stays green. The
module filter has the same property if `postgres_tests` is renamed.

This is `§10.36`'s class — *a check that cannot tell "nothing wrong"
from "nothing examined"* — and it is opened as **`§10.46`** in this
release's `requirements.md` amendment. It is also `§10.39`'s class
(a rule keyed on a string) wearing a second hat; cite `§10.36`,
which is the one that names the consequence.

**The exposure is exactly those four lines, and that was measured
too.** The other ~35 PostgreSQL-free invocations use
`cargo test -p peisear-web --test <file>`, and a missing `--test`
target **fails**:

```
$ cargo test -p peisear-web --test no_such_integration_file
EXIT=101
```

So this is not a sweep of CI. It is four lines.

## §3 — the change

Run the suite whole: `cargo test -p peisear-storage
--no-default-features --features postgres --lib`, no filter. The
four filtered lines go away, and with them the defect in §2 — not
because a guard now catches it, but because **there is no longer a
name for anything to drift from.** That is the better repair of the
two, and it is the one P4 asked for anyway.

Three helpers block it. They build a `sqlite://` pool regardless of
the feature in force:

| Site | What it is |
|---|---|
| `optimistic_lock_guards.rs:49` | the fixture for all 5 guard tests |
| `pool/tests.rs:8` | a fixture helper |
| `pool/tests.rs:99` | a second fixture helper |

`dialect::postgres_tests::migrated_pool` already has the right
shape — read `DATABASE_URL`, connect, migrate — and its doc comment
already names `pool::tests` as the precedent it followed. Follow it
back.

**Not every `sqlite://` in the crate is one of these.** These stay
exactly as they are, and a round that "fixes" them has broken
something:

- `pool/tests.rs:270`, `:312` — the scheme-**rejection** tests.
  Their subject *is* the literal `sqlite://` string, under the
  PostgreSQL branch. They are backend-specific on purpose.
- `pool.rs:30`, `:84`, `:199` — production parsing and its doc.
- `dialect/tests.rs:12`, `:207` — inside a module that is
  `#[cfg(all(test, feature = "sqlite"))]`. Never compiled under
  `postgres`; not a blocker; leave them.

## §4 — gates

- **The unfiltered command, green**, quoted in full, with its test
  count. 48 is the number to beat or to explain.
- The four filtered lines **deleted** from `test.yml`, not left
  beside the unfiltered one.
- **`DEC-007` unchanged** unless this round adds or removes a test;
  if it does, the per-crate block moves by exactly that delta and
  the delta is stated.
- The `sqlite` `--lib` run still lists **31**.
- `fmt` and `clippy` **both ways** — the feature-off build is the
  one that ships.
- `rustdoc`, `mdbook`, `publish-dry-run`.
- **`test.yml` is itself a CI gate's source. Editing it means
  running it**, not reading the other call sites — the standing rule
  after `browser-checks/*.mjs`.

## §5 — escalate rather than deciding

- **If one of the 17 fails for a reason that is not a helper.** A
  production query or a guard behaving differently on PostgreSQL is
  a `PG-004` regression and a requirement question, not a fixture
  fix. Stop and report it with the failure text.
- **If making a helper backend-agnostic changes what a test
  asserts.** A guard test that no longer proves the guard is worse
  than one that only runs on SQLite. Bring it.
- **If the 48 cannot all be made to run** — say which, why, and what
  the bound on the claim is. A partial suite stated as partial is
  acceptable here; a partial suite stated as whole is the defect
  this round exists to close.

## §6 — exit condition

The 17 unrun tests measured against the CI service and that figure
reported **before** any edit; the three helpers reading their URL
the way `migrated_pool` does; the five sites in §3's second list
untouched; `test.yml` running the `postgres` `--lib` suite unfiltered
with the four name-filtered lines deleted; `test.yml`'s own gate
re-run after the last edit; `§10.46` opened in `requirements.md`
with the two measured exit codes as its evidence; `DEC-007`
reconciled; all gates re-run.
