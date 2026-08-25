//! `QA-009` §2 (RFC 005 §3) — every guard `peisear-i18n` owns iterates
//! [`MessageKey::all`](peisear_i18n::MessageKey::all), so a variant
//! missing from it is invisible to `find_violations`, the rendering
//! tests, and the language guard alike, regardless of how thorough any
//! of them are. `all()` is 520 variants' worth of hand-written `vec!`
//! and `.extend(...)` calls, built by the same hands that add copy, in
//! a separate place, with nothing connecting the two — and it was
//! already short five live variants (`EmailForKindAriaLabel`,
//! `InAppForKindAriaLabel`, `MinSeverityForKindAriaLabel`,
//! `NotificationKindPreferencesAriaLabel`, `WebhookForKindAriaLabel`,
//! all `notification_preferences.rs`'s per-kind `aria-label` copy) when
//! this handoff started. Reproduced by counting declared variants
//! against variants reachable from `all()`'s body before writing
//! anything — 520 declared, 515 reachable, the same five names — see
//! `evidence/section2-reproduce-counts.log` in the review request. The
//! five are now added to `all()`, and every guard that walks it (this
//! crate's `guard.rs` and `language_guard.rs`) has run clean against
//! them since.
//!
//! **Shape chosen: scan `message.rs` as source text**, the same family
//! as `prose_scan`, `static_js_scan` and `dec_007_scan` — no macro, no
//! derive, no new dependency. A compiler-forced exhaustive `match` (the
//! handoff's option (b)) is stronger at the instant a variant is
//! *added* — it fails to compile immediately rather than waiting for a
//! test run — but the match's arms and `all()`'s entries would still be
//! two separate hand-edits; satisfying the compiler by adding an arm
//! doesn't add the corresponding `all()` entry, so it moves the gap
//! rather than closing it, exactly as the handoff itself notes. A macro
//! generating the enum and `all()` together (option (c)) closes the
//! class outright, but at the cost of turning a 1,700-line enum that
//! is read and doc-commented variant by variant into macro input —
//! too expensive for what this buys, agreeing with the handoff's own
//! stated preference.
//!
//! **Closes (a)'s own known hole rather than documenting it as a
//! limit.** A plain substring check for `MessageKey::<name>` in `all()`
//! would report a variant as covered merely because its name is a
//! *prefix* of another covered variant's name — this codebase has three
//! such pairs today (`IndicatorValueBusFactor` /
//! `IndicatorValueBusFactorSolo`, `IndicatorExplanationBusFactor` /
//! `IndicatorExplanationBusFactorSolo`, `TrendLabel` /
//! `TrendLabelFlat`), found while writing this guard, not hypothesised.
//! If `IndicatorValueBusFactor` disappeared from `all()` tomorrow, a
//! substring scan would keep passing because
//! `MessageKey::IndicatorValueBusFactorSolo` is still there and
//! contains `MessageKey::IndicatorValueBusFactor` as a literal prefix.
//! Matched at a word boundary instead — the same technique
//! `dec_007_scan` already uses for the identical reason (a crate name
//! that is a substring of another crate's name).
//!
//! **Also strips `all()`'s own `//` comments before scanning it**,
//! the same reason `prose_scan` strips comments (`QA-002`, RFC 005
//! §10.3): a variant name that only appears inside a comment (`// TODO:
//! add MessageKey::Foo`) reads as coverage and is not. `all()` has no
//! block comments in this codebase's style, so line-comment stripping
//! is sufficient.

use std::fs;
use std::path::Path;

/// `crates/peisear-i18n/src/message.rs`'s own source, read as text —
/// no `syn`/`proc-macro2` dependency added for this, matching
/// `dec_007_scan`'s "a guard is not worth a new dependency" reasoning
/// (`QA-004` §6).
fn message_rs_source() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src").join("message.rs");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// True if `needle` appears in `haystack` at a word boundary — not
/// merely as a substring of a longer identifier. Identical definition
/// to `peisear-web`'s `dec_007_scan::appears_at_word_boundary`,
/// reproduced locally rather than shared across crates for a
/// fifteen-line helper.
fn appears_at_word_boundary(haystack: &str, needle: &str) -> bool {
    let bytes = haystack.as_bytes();
    let is_ident_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut start = 0;
    while let Some(rel) = haystack[start..].find(needle) {
        let idx = start + rel;
        let before_ok = idx == 0 || !is_ident_char(bytes[idx - 1]);
        let after_idx = idx + needle.len();
        let after_ok = after_idx >= bytes.len() || !is_ident_char(bytes[after_idx]);
        if before_ok && after_ok {
            return true;
        }
        start = idx + 1;
    }
    false
}

/// Every variant name declared in `pub enum MessageKey { ... }`. Reads
/// the enum's own formatting, enforced continuously by `cargo fmt
/// --check`: a variant name is a line indented by exactly four spaces,
/// starting with an uppercase letter — a struct variant's fields are
/// always indented one level deeper, and the enum's own closing brace
/// is unindented.
fn declared_variant_names(source: &str) -> Vec<String> {
    let start_marker = "pub enum MessageKey {";
    let start = source
        .find(start_marker)
        .expect("message.rs declares `pub enum MessageKey {`");
    let after_start = &source[start + start_marker.len()..];
    let end = after_start
        .find("\n}\n")
        .expect("`MessageKey` is closed with an unindented `}`");
    let body = &after_start[..end];

    body.lines()
        .filter_map(|line| {
            if !line.starts_with("    ") || line.starts_with("     ") {
                return None;
            }
            let rest = &line[4..];
            let mut chars = rest.chars();
            let first = chars.next()?;
            if !first.is_ascii_uppercase() {
                return None;
            }
            Some(
                std::iter::once(first)
                    .chain(chars.take_while(|c| c.is_alphanumeric() || *c == '_'))
                    .collect(),
            )
        })
        .collect()
}

/// `MessageKey::all()`'s own body, with `//` line comments stripped.
fn all_fn_body_without_comments(source: &str) -> String {
    let start_marker = "pub fn all() -> Vec<MessageKey> {";
    let start = source
        .find(start_marker)
        .expect("message.rs declares `pub fn all() -> Vec<MessageKey> {`");
    let after_start = &source[start + start_marker.len()..];
    let end = after_start
        .find("\n    }\n")
        .expect("`all()` is closed with a four-space-indented `}`");
    let body = &after_start[..end];

    body.lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn every_message_key_variant_appears_at_least_once_in_all() {
    let source = message_rs_source();
    let declared = declared_variant_names(&source);
    assert!(
        declared.len() >= 100,
        "found suspiciously few MessageKey variants ({}) -- the enum-body parsing \
         assumption this scan depends on may have changed",
        declared.len()
    );

    let all_body = all_fn_body_without_comments(&source);
    let missing: Vec<&String> = declared
        .iter()
        .filter(|name| !appears_at_word_boundary(&all_body, &format!("MessageKey::{name}")))
        .collect();

    assert!(
        missing.is_empty(),
        "these MessageKey variants never appear in MessageKey::all() -- every guard \
         that iterates all() (find_violations, the language guard, the rendering \
         tests) never sees them:\n{}",
        missing
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
