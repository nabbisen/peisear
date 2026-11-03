# WLK-001 — the gate nobody triggers

**`env-example-walk.yml` asserts the documented copy-and-run path, and
has run three times in its life.**

Measured 2026-11-03: `on: workflow_dispatch:` **only**, with a required
`ref` input. Its entire run history is **three manual dispatches, the
last on 2026-10-16** — before `0.57.0`, `0.58.0`, `0.58.1`, `0.59.0` and
`0.60.0` shipped.

It is **not** a weak job. Since `0.53.0` round 2 it asserts: non-zero
exit if the documented path does not reach a listening server. `§10.40`
records the one dispatch that caught a real defect and the owner's own fix
behind it. **The defect is that it is not running**, which is `§10.36`'s
class in a shape that entry has not held before: not a check that examines
nothing, but a check that examines nothing **because nobody triggered
it.**

**And the surface it guards just grew.** `QS-003` (2026-11-03) added
`cp .env.example .env` to `docs/getting-started/build-from-source.md`, so
that command is now documented on **two** pages, guarded by a job last run
eighteen days and four releases ago.

## §0 — the decision, already made

**A `push` trigger scoped to the paths that can break it.** Not every
push, not a schedule. The architect's reasoning, so this round does not
re-open it: a real gate that is simply not running wants to run **exactly
when its own inputs change**, and a path filter says that precisely while
costing nothing on the pushes that cannot affect it.

**Paths to filter on, as the architect reads the dependency** — verify
each against the workflow before trusting this list, and report any you
add or drop with the reason:

- `.env.example` — the file the walk copies.
- the configuration sources the walk's success depends on
  (`crates/peisear-web/src/config.rs` and its module, wherever
  `Config::from_env` and the variable resolvers now live).
- `docs/getting-started/build-from-source.md` and
  `.github/CONTRIBUTING.md` — the two documents that publish the command.
- the workflow itself.

**This does not widen `NFR-REL-011`.** That entry's `0.53.0` round 2
decision excluded this reader deliberately and said in terms that the walk
*"stands on its own merit as a regression guard for the contributor path;
it does not need this entry's authority to justify existing."* **Quote
that if the question arises; do not re-decide it.**

## §1 — the engineering content, which is the `ref` input

**`workflow_dispatch` has a required `ref` input; `push` has no inputs.**
So the job's own steps must work under both event types without breaking
the dispatch path, which is the one that has actually been used.

- On `push`, the pushed ref is what to walk.
- On `workflow_dispatch`, the input still governs.
- **The dispatch path must keep working**, and you must show it does —
  not by reading the YAML, by dispatching it.

## §2 — seen red, and the plant may not touch `.env.example`

**`.env.example` is covered by the owner's own permission rule.** Do not
read it, copy it, or print it locally. `QS-003` hit this and did the right
thing: it proved what it could with equivalent environment variables and
**stated plainly** what it had therefore not re-proven. Do the same.

**The job itself may copy the file** — it already does, on a runner, and
its log is the only thing read back. That is the existing design and it
is deliberate (`env-example-walk.yml`'s own header comment says so).

So the plant is: **break something the walk depends on — in the
configuration code, not the example file — push, and read the job's own
log.** It must exit non-zero and say why. Then revert, push, and confirm
it goes green on the push trigger rather than on a dispatch.

**Report the run ids for both.** A trigger that fires is the claim; a run
page is the evidence.

## §3 — the trap

**A path-filtered trigger that lists a path wrongly is silent.** Spell a
path wrong and the job simply never fires again, and nothing says so —
the same shape as a lookup that returns `None` and passes
(`§10.36` instance 6). **So the plant above must be in a file the filter
names**, and you must confirm the run was triggered **by push** and not by
your own dispatch. Read the run's own event on the run page.

**Second trap**: do not make the push path advisory to keep `main` green.
If it fires and fails, that is the gate working.

## §4 — gates

`cargo fmt --check`; `clippy --workspace --all-targets -- -D warnings`;
`mdbook build docs` if any document changes; the full per-crate suite with
`DEC-007` from the runner — **zero delta expected**, stated explicitly.
CI's own conclusion quoted, plus the two `env-example-walk` run ids from
`§2`.

## §5 — escalate rather than deciding

- If the walk turns out to be **red on `main` today** — it has not run
  since 2026-10-16 and four releases have landed — **stop and report
  it.** That is a live defect in a documented path, and it outranks the
  trigger that found it.
- If a path the filter needs cannot be named without reading a file the
  permission rule covers, **say so** rather than guessing at its contents.
- If `push` turns out to be the wrong event for a reason the architect has
  not seen, name it; a schedule was the considered alternative.

## §6 — exit conditions

1. `env-example-walk.yml` fires on `push`, filtered to the paths that can
   break it, with the final path list and the reason for any change to
   the list above.
2. The `workflow_dispatch` path still works, **shown by dispatching it**.
3. **Seen red by push**, then green, with both run ids and the run's own
   event confirmed as `push`.
4. Nothing local read or copied from `.env.example`, and any claim this
   prevented you from proving stated plainly.
5. `§4`'s gates after the last edit, `DEC-007` delta (zero), CI quoted.
