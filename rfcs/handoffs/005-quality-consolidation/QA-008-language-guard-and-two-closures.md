# QA-008 — The language guard, and two holes the last three handoffs left

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P1 — 0.27.0
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §3, plus
§1's bulk-route rule
**Depends on**: nothing. `QA-003` through `QA-007` are all closed.

---

## 1. Three items, one theme

Each is a case where **something reads as covered and is not**, and each was
found by planting rather than by reading. They are independent; land them in
any order.

1. §2 — the English renderer accepts non-English copy.
2. §3 — `mark-all-read` can be made inert with the suite green.
3. §4 — the CI job that runs the four guards can be deleted with the suite
   green.

## 2. RFC 005 §3 — a guard, not a conversion

**Read the RFC's §3 first.** It was reconciled against the code on 2026-08-25
and rewritten: the conversion it originally asked for is **already done**, and
you should not go looking for Japanese copy to translate. Every occurrence in
the tree is a comment citing the Japanese source documents, or one test fixture
(`escape_like_meta("ログイン")`), and all of those stay exactly as they are.

**The live gap is the reverse direction.** Planted into `en.rs`:

```rust
MessageKey::NewSubIssueLabel => "新しいサブ課題".to_string(),
```

`cargo test --workspace`: **195 passed, 0 failed.** `prose_scan` tests
`is_ascii_alphabetic` and does not see a non-Latin literal at all;
`find_violations` looks for prohibited English phrases and finds none in a
string containing no English.

**Reproduce that before you fix it.**

### 2.1 What to assert

Nothing the English renderer produces may contain characters from a **non-Latin
script**.

**Not "ASCII only".** The shipped copy legitimately uses `—`, `←`, `✓`, `⚠`
and curly quotes; a guard that rejects those would fail on the current tree and
be weakened until it passed, which is worse than no guard. Whatever rule you
choose, **run it against the message table as it stands and report the hit
count**. If it is not zero, the rule is wrong, not the copy.

### 2.2 Two shapes — pick one and say why

- **Scan `en.rs` as source text**, like `prose_scan` and `static_js_scan`.
  Cheap, no rendering. Blind to a string assembled from parts, and it would
  read comments unless you exclude them — and the comments in that file are
  exactly the citations that must stay.
- **Render every `MessageKey::all()` variant and check the output.** Stronger:
  it checks what a user sees, auto-covers new keys, and has no comment problem.
  `find_violations` already works this way. Watch the representative parameters
  — a check over rendered output must not flag interpolated content that is
  legitimately the user's.

I lean to the second and am not confident. Say which you chose and what the
other one would have missed.

### 2.3 Where it runs

**It must run in CI.** That was `QA-005`'s whole point, and a guard added the
week after that handoff which is not in a CI job would be a joke at this
project's expense. `peisear-i18n` already has a job, and the fact being
asserted is about that crate's own source; `peisear-web`'s lib job holds the
other four. Either is defensible — say which and why.

## 3. The bulk-route rule — `mark-all-read` can be made inert

`QA-007` added `mark_all_read_does_not_affect_another_users_notifications`.
Nothing asserts the route does anything at all. I replaced its predicate with
one matching nothing:

```sql
-  WHERE user_id = ?1 AND read_at IS NULL
+  WHERE user_id = 'nobody'
```

`cargo test --workspace`: **195 passed, 0 failed.** The button is inert for
everyone and the suite is green.

**A negative assertion alone is satisfied by deleting the feature.** RFC 005 §1
now records this as a rule for every bulk row: a route that writes an unbounded
set needs a positive assertion — that it affects the caller's own rows — as
well as the cross-user one.

Add it: alice has two unread notifications, alice posts `/inbox/mark-all-read`,
both are read. Plant the inert predicate above and show it failing.

**Then sweep §1's table for other bulk rows in the same state.** `silence-all`
is the obvious candidate. Report what you find; do not fix beyond
`mark-all-read` in this handoff unless a row is trivially the same shape, and
say which you judged trivial.

## 4. Pin the `DEC-007` block to `test.yml`

I deleted `test-peisear-web-lib` from `.github/workflows/test.yml` and changed
nothing else. **195 passed, 0 failed.**

`dec_007_scan` pins the block to the workspace members. Nothing pins the block
to CI, so `§10.16` — the gap `QA-005` exists to close — is reconstructible by
deleting twenty lines of YAML, with `CONTRIBUTING.md` still claiming the guards
run.

**The chain terminates**: workspace members → `DEC-007` block → CI jobs. Pin
the third link and no fourth artefact is making claims about the others.

Assert that every `cargo test -p …` line in the block has a corresponding
`run:` line in `test.yml`. As with `QA-005` §3, this is a fact about two files'
text and needs nothing external.

**Do not require the reverse.** CI legitimately runs things the block does not
— `fmt`, `clippy`, `build`, `msrv`. A bidirectional check would fail on the
current tree.

**Expect the same class of hole this guard has hit three times**: a line that
looks like coverage but is not. A commented-out `run:`, a job with `if: false`,
a `continue-on-error: true`. Decide how far to go and say where you stopped —
I would rather have a guard that catches the deletion case with its limits
written down than one that tries to interpret YAML semantics.

**This changes `dec_007_scan`'s scope again**, or adds a sibling. Either is
fine; say which and why. If it becomes a sibling, the module doc's history
paragraph should say where the other half went.

## 5. Also, from `QA-005` round 2's review

That module doc's history lists four holes; the first — a substring match on
the bare crate name — was never one. `QA-004` round 1 shipped
`appears_at_word_boundary` specifically to defeat it and proved it by plant. It
was designed out, not shipped and later closed. Correct that bullet while you
are in the file.

## 6. Escalate rather than deciding

- **If §2, §3 or §4 does not reproduce, stop and report** before implementing.
- If §2.1's rule finds hits in the current message table, stop — the rule is
  wrong and I want to see the hits before any copy changes.
- If §3's sweep finds a bulk route that is **already** broken rather than
  merely untested, stop and report. That is a live defect, not a coverage gap.
- If pinning the block to `test.yml` turns out to need YAML parsing to be
  useful at all, report that rather than adding a dependency.

## 7. Acceptance

1. §2: guard present, running in CI, with the rule reported against the current
   table at zero hits; §2.2's choice explained.
2. §3: positive assertion added, demonstrated against the inert plant; the
   sweep reported.
3. §4: block-to-CI check present, demonstrated against the job deletion; its
   limits written down.
4. §5 corrected.
5. Every plant one at a time, each reverted, `git diff` clean between.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. §2.2's and §4's design choices as prose, not table rows. Each
plant transcript separately.
