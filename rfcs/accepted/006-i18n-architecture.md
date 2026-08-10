# RFC 0006: Internationalisation architecture and vocabulary guard

**Status**: Accepted
**Target**: 0.21.0
**Related spec sections**: §3 (vocabulary), §28.6 (computation vs presentation), §34 (language and locale)
**Related requirements**: `NFR-LANG-001`, `NFR-LANG-003`, `NFR-LANG-005`, `FR-HLT-006`, `FR-HLT-009`, `NFR-MNT-001`, `NFR-MNT-002`
**Governing decision**: `DEC-022`
**Last updated**: 2026-07-31

## Summary

Route every user-visible string through a lookup table, ship one locale
(English), and use the resulting table as the enforcement point for the
non-evaluative vocabulary rules in §1.7.

This RFC is not primarily about languages. `NFR-LANG-005` keeps additional
locales deferred. It is about creating **one place where all user-visible copy
lives**, because §1.7 is currently a convention that nothing checks.

## Background

`FR-HLT-006` (explanation neutrality) and `NFR-LANG-001` (non-evaluative
vocabulary) are both **P0**. The 0.19.1 baseline records both as *"Implemented
by convention; no automated guard exists."*

Release 0.20.0 exists because that convention failed, in three places at once:
a `Score N / 100` badge, a `Concern` severity mapped to danger colouring, and
the literal string `"Failed to update status. Please refresh."` — a phrase §1.7
names explicitly as prohibited. Each was found by reading code. None would have
survived a lint over a string table.

Shipping further screens before the guard exists means finding the next batch
the same way.

Two supporting drivers:

- `project-instructions-rust-gui.md` requires GUI multilingual support, while
  `NFR-LANG-003`/`NFR-LANG-005` defer it. `DEC-022` resolves the conflict:
  adopt the architecture, defer the locales.
- The mockup (`peisear-mockup-v0.7.0`) already proves the pattern with
  `Locale::tr` plus a `FORBIDDEN_LABELS` test. `DEC-023` cherry-picks the guard.

## Requirements

### Must

1. Every user-visible string resolves through a lookup keyed by a stable
   identifier.
2. A missing rendering for any key is a **compile-time** failure, not a runtime
   fallback to the key name or to an empty string.
3. An automated guard asserts that no table entry contains prohibited §1.7
   vocabulary, and it runs as a blocking CI gate.
4. The guard covers **notification bodies and email copy**, not only HTML.
5. English ships. The design must not assume a single locale.
6. No behaviour change visible to users beyond copy that the guard rejects.

### Nice

7. Message parameters are typed rather than positional.
8. The design does not preclude locale-aware date and number formatting
   (`NFR-LANG-004`, P3, currently unimplemented).

## Design

### D1 — Compile-time tables, not runtime files

Locale data is Rust source compiled into the binary — the mockup's approach —
rather than JSON/Fluent/gettext loaded at startup.

Rationale: peisear is a single self-hosted binary with migrations already
embedded at compile time (`NFR-CMP-002`); adding a runtime asset-loading path
contradicts that shape. It adds no dependency (`NFR-SEC-007`), no parse-failure
mode at boot, and it makes requirement 2 achievable through the type system.

Cost: adding a locale requires a rebuild. Acceptable — operators already build
or download a binary per release, and locales are deferred regardless.

*Alternative considered*: a crate such as `fluent`. Rejected for this phase —
it buys plural/gender machinery we do not need for one locale, at the cost of
a dependency and runtime resolution.

### D2 — A `peisear-i18n` crate

A new workspace member holding `Locale`, the message-key type, and the tables.

The problem it solves: **user-visible strings originate in at least three
crates today.** `peisear-core::Indicator::human_explanation()` returns English
prose. `peisear-notify` composes notification and email bodies. `peisear-web`
renders HTML. A table living in `peisear-web` cannot serve the other two, and
requirement 4 would be unmeetable.

`peisear-i18n` is a leaf crate with no workspace dependencies. `-core`,
`-notify`, and `-web` depend on it. No cycles.

*Alternative considered*: table in `peisear-web`, with `-core` and `-notify`
returning plain `&'static str` keys. Rejected — nothing would then guarantee
that a key emitted by `-core` has a rendering, which defeats requirement 2.

### D3 — The domain emits keys, not prose

This is the substantive change, and the largest.

`peisear-core` currently generates user-visible sentences. That is a
presentation responsibility sitting in the computation crate, in tension with
`FR-HLT-009` ("computation and presentation are separate concerns") and
`NFR-MNT-001` ("metric computation MUST be implemented as pure functions in the
domain crate"). The separation the baseline claims is partially fictional
today.

Under this RFC, `human_explanation()` and its siblings return a **message
descriptor** — a key plus typed parameters — rather than a `String`.
`peisear-web` and `peisear-notify` render it.

Consequences:

- The domain states *what is true* (`three issues have not moved in over two
  weeks`, as data). Presentation decides how to say it, in which language, with
  which severity clamp.
- Every user-visible sentence becomes reachable from one table, so the guard
  in D4 is genuinely exhaustive rather than covering markup only.
- `FR-HLT-009` becomes structurally true instead of aspirational.

The message-key type is an enum, exhaustively matched per locale, satisfying
requirement 2.

### D4 — The vocabulary guard

A test in `peisear-i18n` that walks every entry of every locale table and
asserts no prohibited term appears.

The prohibited set is §1.7 in full, not the mockup's five-word subset:
evaluative phrasing, judgement, directives (`you should`, `you must`),
`velocity`, ranking vocabulary, completion-rate emphasis, and failure framing
(`Failed to`, `Error:`). Matching is case-insensitive and word-boundary aware.

Scope limits, stated so the guard is not over-trusted:

- It covers **copy**, not interpolated data. An issue titled "velocity spike"
  is user data and is not a violation.
- It cannot judge tone, only vocabulary. `FR-HLT-006` still needs human review.
- It cannot catch a prohibited word assembled at runtime from fragments.
  Assembling user-visible sentences by concatenation is therefore prohibited;
  compose through parameters.

The guard is a **blocking** gate from 0.21.0 (see open question 3).

### D5 — Delivery

Not one sweep. `peisear-web` is ~12.8k LOC; a single change touching every
string is unreviewable and conflicts with everything.

Delivered as sequenced handoffs:

1. `peisear-i18n` crate, `Locale`, key type, English table, guard test, CI wiring.
2. `peisear-core` message descriptors (D3) — the boundary change.
3. `peisear-notify` bodies and email copy — requirement 4.
4. `peisear-web` by surface group: shell and navigation; project and issue;
   team and sprint; today, inbox, settings; errors and validation.

**0.21.0 is not complete until every shipped user-visible string is converted.**
Partial migration is the failure mode to avoid: a guard covering half the copy
invites the belief that the copy is covered.

### D6 — Conversion conventions

*Added 2026-08-10, established by handoff I18N-005a and its review. Recorded
here rather than in the handoff index because these are design decisions: they
bind every conversion handoff, and a handoff index is not a design authority
(RFC 000, "letting handoffs override RFC decisions").*

1. **A `String` parameter carries user data only.** Anything that is our own
   copy is a key.

   I18N-005a shipped `BackToLabel { label: String }`. Three call sites passed
   our own words as raw strings — the guard never saw them, and they had
   already drifted to three different casings. A `String` parameter is a hole
   in the guard's coverage whenever what flows into it is copy rather than
   data, and the hole is invisible: the table looks fully converted.

2. **Render inline; pre-bind only when earned.** A short helper —
   `t(MessageKey::X)` — keeps rendering readable inside markup. Pre-bind only
   where a string is reused, or where selecting it needs conditional logic.
   Under this rule `Navbar` went from ten pre-bindings to two, which is what
   keeps the preamble from growing with the string count.

3. **Parameterise a closed set only when more than one message embeds it.**
   Flat variants when a value always stands alone. `IndicatorLabel` earns its
   shape by appearing in three sentence templates; navigation words appear in
   one place each.

4. **Attributes are copy.** `aria-label`, `title`, placeholder text — read by
   users, some of whom have no other way to read the interface. Decorative
   glyphs behind `aria-hidden`, and protocol values such as `role` and `type`,
   are not.

5. **"Rendered output unchanged" means semantically identical** — same visible
   text, same attribute set, same values. Ordering may differ *only* where an
   attribute became dynamic, which Leptos's SSR renderer causes by moving
   `class` to the end of the tag. Verify tag by tag; report anything outside
   those two categories.

6. **Name keys for what the message says, not where it appears.** A key named
   for its location is wrong the moment the layout changes, and the point of
   the table is that copy outlives layout.

## Test plan

| Check | Mechanism |
|---|---|
| Every key has a rendering in every locale | Exhaustive `match` — compile-time |
| No prohibited vocabulary in any table entry | `peisear-i18n` guard test (D4), blocking |
| No raw key string reaches rendered output | Integration assertion that responses contain no `[a-z_]+\.[a-z_]+` key-shaped literals in visible text |
| The table mechanism is not English-shaped | A fixture locale with distinct values; assert rendering switches wholesale |
| Notification and email bodies resolve through the table | `peisear-notify` test |
| No message is rendered unescaped | Review plus a test that a parameter containing `<script>` is escaped in HTML output |

**Known consequence**: the 61 existing web integration tests assert against
hardcoded English literals. They will continue to pass, since English ships,
but they become brittle. Converting them to assert via the table is
**explicitly out of scope for 0.21.0** — it doubles the change size for no
correctness gain while one locale exists. Revisit when a second locale lands.

## Security and privacy considerations

- **Escaping.** Interpolated parameters carry user data (issue titles, display
  names). Leptos escapes by default; any message rendered as raw HTML would be
  an XSS vector. No message may be rendered unescaped — asserted by test.
- **Email is outside the browser's escaping.** `peisear-notify` composes bodies
  delivered by SMTP. Parameter handling there needs its own review; this is why
  requirement 4 exists.
- **No personal data in keys or table entries.** Personal values arrive as
  parameters at render time and are subject to `NFR-PRIV-001` at the call site,
  not at the table.
- No change to authorisation, session handling, or the privacy boundary.

## Out of scope

- Shipping a second locale (`NFR-LANG-005`, deferred).
- Locale-aware date/number formatting (`NFR-LANG-004`, P3) — must not be
  precluded, not delivered.
- Locale negotiation, `Accept-Language`, or a language switcher.
- Converting existing tests to table-driven assertions.
- The `SPEC §28.1` indicator-set divergence (§10.1) — separate and still open.

## Open questions

1. **Does `peisear-core` keep any prose at all?** D3 says no. A narrower
   variant leaves core returning prose and accepts that those sentences sit
   outside the guard. *Default: full D3 — a guard with a known hole in the
   health explanations, which are exactly where `FR-HLT-006` applies, is not
   worth building.*
2. **Crate or module?** D2 proposes a crate. A module inside `peisear-core`
   would avoid a workspace member, at the cost of making `-notify` depend on
   `-core` for copy. *Default: crate.*
3. **Blocking gate immediately, or advisory for one release?** *Default:
   blocking from 0.21.0. An advisory guard on a P0 requirement is a guard
   nobody fixes.*
4. **Port the mockup's Japanese table now?** It exists and is free to take. But
   `DEC-022` ships one locale, and an unshipped second table would drift.
   *Default: do not port. Use the D-test fixture locale to prove the mechanism
   instead; port JA when a locale is actually scheduled.*
5. **Does the guard run over `AppError` and validation messages?** They are
   user-visible and DEV-001 already corrects two of them. *Default: yes — they
   are copy and belong in the table.*

## References

- `DEC-022`, `DEC-023` — approved decisions, 2026-07-31
- Requirements baseline §1.7, §5.4; `FR-HLT-006`, `FR-HLT-009`, `NFR-LANG-001`
- External design §10 (message and vocabulary catalogue)
- `peisear-mockup-v0.7.0`: `crates/peisear-web/src/i18n/`,
  `tests/invariants.rs` (`FORBIDDEN_LABELS`)
- DEV-001, DEV-004 handoffs — the vocabulary defects motivating the guard
