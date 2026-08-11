# I18N-005a — Shell and navigation copy through the table

**Issued by**: Architect
**Date**: 2026-08-10
**Priority**: P1
**Governing RFC**: [006](../../done/006-i18n-architecture.md) §D5
**Depends on**: I18N-002 (landed)
**Establishes the pattern** that I18N-005b–e follow.

---

## 1. Purpose

Convert the application shell and navigation — the smallest surface group, and
therefore the right one to establish the pattern on.

Files: `components/layout.rs`, `components/breadcrumb.rs`,
`components/error_page.rs`, plus any shell strings in `app.rs` or `shell`-level
code. Roughly a dozen user-visible literals.

**This handoff's real output is the pattern.** Four more surface groups follow
it, and decisions made here — key naming, parameter shapes, how a converted
component reads — get inherited by all of them. Getting the shape right matters
more than getting through the strings.

## 2. Why this group first

`me.rs`, `issues.rs` and `sprints.rs` carry roughly 35, 35 and 31 user-visible
literals respectively. The shell carries about a dozen, and they are the
simplest — labels and headings, few parameters, no conditional prose.

Establishing the pattern on 300 strings would mean discovering a bad decision
300 strings in.

## 3. Change scope

- `crates/peisear-web/src/components/{layout,breadcrumb,error_page}.rs`
- Shell-level strings in `app.rs` / `shell.rs` if any
- `crates/peisear-i18n/src/` — new `MessageKey` variants and renderings

Not the page components. Those are 005b–e.

## 4. Required implementation

### 4.1 Find the strings honestly

Grep is a starting point, not a survey. A user-visible string can be a `&str`
literal, a `format!`, a `.to_string()`, an `aria-label`, a `title=`, a `<button>`
body, or placeholder text.

**Report the count you found and how you found it.** If you convert eleven
strings and there were fourteen, the guard covers eleven and the crate looks
converted. That gap is the failure mode for this whole sub-series.

### 4.2 Key naming

Follow the grain I18N-001 established and I18N-002 confirmed: **one variant per
message**, typed parameters for closed sets.

Name keys for **what the message says**, not where it appears. `SkipToContent`,
not `LayoutLink3`. A key named for its location becomes wrong the moment the
layout changes, and the point of the table is that copy outlives layout.

### 4.3 Attributes are copy too

`aria-label`, `title`, and placeholder text are read by users — some of them by
users who have no other way to read the interface. They go through the table.

This is easy to skip because it does not look like text.

### 4.4 Do not reword anything

Byte-identical relocation. If a string is wrong, that is a finding to report,
not a change to make — the same rule I18N-002 followed, and the reason
`ISSUE-006` was found rather than silently patched.

## 5. Required tests

1. The guard covers every new entry and passes.
2. Every new key has a rendering — the exhaustive `match` guarantees it;
   confirm it still holds.
3. Rendered output unchanged. Assert on rendered HTML, not on keys.
4. Existing suites pass with unchanged counts — `breadcrumb` and `smoke`
   exercise this surface directly.

## 6. Acceptance criteria

1. No user-visible literal remains in the converted files — including
   attributes.
2. Guard covers the new entries and passes.
3. Rendered output byte-identical.
4. fmt and clippy exit 0; suite counts unchanged.
5. The §4.1 survey is reported: how many strings, how found, what was excluded
   and why.

## 7. Prohibited

- Do not reword. Report instead.
- Do not convert page components — 005b–e own those.
- Do not add a second shipping locale.
- Do not weaken the guard. A rejection means the copy is wrong; report it.

## 8. Evidence

- Changed-file list.
- The §4.1 survey.
- Before/after rendered HTML for one representative screen.
- Guard output over the new entries.
- fmt, clippy, full suite.

## 9. Review focus to request

1. **The pattern**, more than the strings: does a converted component still
   read clearly, or has indirection made it harder to see what a screen says?
   If the second, say so now — four more groups inherit this.
2. Key naming: message-shaped rather than location-shaped?
3. Anything found in §4.1 that was ambiguous about being user-visible.

**Escalate rather than deciding** if a string's correct rendering is ambiguous,
if the guard rejects existing copy, or if the pattern feels wrong at this scale
— it will feel worse at thirty-five strings per file.
