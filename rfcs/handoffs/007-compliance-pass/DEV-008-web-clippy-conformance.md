# DEV-008 — Clear the clippy debt in `peisear-web`

**Issued by**: Architect
**Date**: 2026-08-01
**Priority**: P1 — `NFR-MNT-007`; closes the workspace clippy gate
**Governing decision**: ISSUE-002 decision
**Depends on**: **DEV-001, DEV-002, DEV-003 and DEV-004 must all have landed.**
Do not start before then.
**Position**: last unit in RFC 007.

---

## 1. Purpose

`cargo clippy --workspace --all-targets -- -D warnings` still exits 101. DEV-007
cleared `peisear-storage`; the remaining findings are in `peisear-web`.

This is the last thing standing between the release and exit criterion 2.

## 2. Background — why this exists separately

Clippy checks crates in dependency order and stops a downstream crate once an
upstream one fails to compile. While `peisear-storage` was broken, `peisear-web`
was never linted at all: its findings were **invisible, not absent**. Clearing
storage is what made them reachable.

The three currently known:

| Finding | Location |
|---|---|
| `too_many_arguments` (13/7) — `render_issue_detail` | `components/issues.rs` |
| `unnecessary_sort_by` ×2 — `apply_filter_and_sort`'s descending sorts | `handlers/issues.rs` |

**Treat that list as a lower bound, not a specification.** Because `-D warnings`
fails the library target, the `--all-targets` test targets have never been
linted either. There are fourteen test crates. The true count is unknown.

The dependency on four other handoffs is not politeness: both known findings sit
in files that DEV-001, DEV-002, DEV-003 and DEV-004 all edit. Starting early
means racing them.

## 3. Two phases — measure, then fix

### Phase 1 — establish the true count, and stop

Once the four handoffs have landed:

```bash
cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tee dev008-full-count.log
```

Report, before changing anything:

- Total findings, by (lint kind, file).
- Which are in library targets and which in test targets.
- Whether any requires a behaviour change to clear.
- Whether any would undo work from DEV-001..004 — in particular DEV-004's
  `DisplayHealthState` clamp or DEV-001/002's shared `apply_status_change`.

**Escalate rather than proceeding if any of these hold:**

- More than **15** findings in total.
- Any finding that cannot be cleared without changing observable behaviour.
- Any finding whose fix would revert or weaken a DEV-001..004 change.
- Any finding requiring a change to a `peisear-core` or `peisear-storage`
  signature — those crates are clean and are not this handoff's to reopen.

Three findings is a handoff. Thirty is a scope conversation, and it is mine to
have with the owner, not yours to absorb.

### Phase 2 — fix, once Phase 1 is cleared

Only after Phase 1 is reported and either falls under the thresholds or is
explicitly authorised.

## 4. Change scope

`crates/peisear-web/src/**` and `crates/peisear-web/tests/**`, limited to what
the findings require.

## 5. Non-change scope

- **No behaviour change.** Same rendering, same routes, same status codes.
- `peisear-core` and `peisear-storage` — both clean; do not reopen.
- Do not weaken anything DEV-001..004 established. Specifically: the
  `DisplayHealthState` clamp must remain the only route to a health badge, and
  `apply_status_change` must remain the single lock check.
- Do not touch `static/board.js`.
- No new tests beyond what a fix forces. The existing suite is the safety net.

## 6. Required implementation

Clear the findings so the workspace gate exits 0.

**Carry DEV-007's ambition limit.** Smallest correct change that engages with
each finding. Do not redesign the component layer. If the thirteen-argument
`render_issue_detail` signals something structural about how components are
composed — it may well — **report it, do not fix it here.** That is an RFC for a
later slot, and this release is a correction pass.

Guidance for the two known kinds:

1. **`unnecessary_sort_by`** — mechanical. `.sort_by(|a, b| b.x.cmp(&a.x))`
   becomes `.sort_by_key(|b| std::cmp::Reverse(b.x))`. No signature change, no
   call sites affected. Verify the ordering is genuinely unchanged, including
   for equal keys — `sort_by_key` is stable, as is `sort_by`, so it should be,
   but the filter/sort behaviour is covered by the `view_state` suite and that
   must stay green.

2. **`too_many_arguments` on `render_issue_detail`** — a parameter struct, as
   DEV-007 did in storage. Group by meaning rather than position. Name it for
   what it *is* to the page, not for its shape. This is a component-internal
   type in `peisear-web`; it does not cross a crate boundary and should not be
   made `pub` beyond what the call site needs.

## 7. On `#[allow]`

Same rule as DEV-007: permitted only where the lint is genuinely wrong for the
case, with an adjacent comment saying why. Suppression added to make the gate
pass without engaging with the finding will be rejected.

If suppression looks necessary in more than two places, escalate — that is a
design conversation.

## 8. Acceptance criteria

1. `cargo clippy --workspace --all-targets -- -D warnings` **exits 0.** This is
   the first time in the project's history that will be true; it is the point of
   the handoff.
2. `cargo fmt --all -- --check` exits 0 (run `cargo fmt` once after edits).
3. Full per-crate test suite passes with unchanged counts, including
   `board_keyboard` and the expanded `health_explainability`.
4. No behaviour change.
5. Every `#[allow]` added, if any, carries an adjacent justification.

## 9. Required evidence

- `dev008-full-count.log` from Phase 1 — the complete finding set **before** any
  fix. This is the first honest measurement of `peisear-web`'s lint surface and
  is worth keeping regardless of what the fix looks like.
- Clippy output after: exit 0, no output.
- Full per-crate test output.
- Changed-file list, with test-target changes separated from library changes.
- For any new type: one line on what it represents.
- Anything you left alone under the ambition limit, per §6.

## 10. Review focus to request

1. Whether any new parameter struct expresses a page concept or merely satisfies
   the lint.
2. Whether the `unnecessary_sort_by` rewrites preserve ordering exactly.
3. Anything reported under §6's "report it, do not fix it here".

**Escalate rather than deciding** on any Phase 1 threshold in §3, or if clearing
a finding appears to require touching a clean crate.
