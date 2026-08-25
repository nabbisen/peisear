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
//!
//! `QA-010` §2 — **the same defect exists one level down.** 125 of
//! `MessageKey`'s 520 variants are parameterised by one of fourteen
//! label enums (`IssueStatusLabel`, `PriorityLabel`, ...), each with
//! its own hand-maintained `pub fn all() -> [Self; N]`. The array
//! length is part of the type and moves with the literal, so the
//! compiler has no opinion about whether the enum still has that many
//! variants — reproduced by shrinking `IssueStatusLabel::all()` from
//! `[_; 3]` to `[_; 2]` (dropping `Done`) and watching `cargo test
//! --workspace` stay green before writing anything here (`evidence/
//! section2-reproduce-plant.log`). `label_enum_all_blocks`
//! auto-discovers every enum with this signature shape rather than
//! naming the fourteen — a fifteenth one added later is covered
//! without touching this file, and the fourteen are not hardcoded
//! anywhere for a sixteenth to eventually drift past. Two checks per
//! enum: membership (every declared variant named in its own `all()`,
//! word-boundary matched, comments stripped — identical technique to
//! `MessageKey`'s own check above) and the declared array length
//! against the actual declared variant count, since membership alone
//! passes a list that names one variant twice and omits another.
//! `QA-010` §1: **nothing here was broken before this handoff** — all
//! fourteen enums pass both checks as the tree stands; this is a
//! tripwire, not a repair.

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

/// Every variant name declared in `pub enum <enum_name> { ... }`. Reads
/// the enum's own formatting, enforced continuously by `cargo fmt
/// --check`: a variant name is a line indented by exactly four spaces,
/// starting with an uppercase letter — a struct variant's fields are
/// always indented one level deeper, and the enum's own closing brace
/// is unindented. Shared between `MessageKey` and every label enum
/// below it in this file — same shape, same convention.
fn declared_variant_names(source: &str, enum_name: &str) -> Vec<String> {
    let start_marker = format!("pub enum {enum_name} {{");
    let start = source
        .find(&start_marker)
        .unwrap_or_else(|| panic!("message.rs declares `pub enum {enum_name} {{`"));
    let after_start = &source[start + start_marker.len()..];
    let end = after_start
        .find("\n}\n")
        .unwrap_or_else(|| panic!("`{enum_name}` is closed with an unindented `}}`"));
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
    let declared = declared_variant_names(&source, "MessageKey");
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

/// One label enum's `all()` — auto-discovered from a `pub fn all() ->
/// [<name>; <declared_len>]` signature, rather than a hardcoded list of
/// the fourteen enums that happen to have one today. A fifteenth such
/// enum, added in this same style, is picked up automatically; naming
/// them would just move `QA-009`'s own defect one level sideways, into
/// this guard's own hardcoded list.
struct LabelEnumAll {
    name: String,
    declared_len: usize,
    all_body_without_comments: String,
}

/// Finds every `pub fn all() -> [<Name>; <N>] { ... [ ... ] ... }`
/// signature in `source` and returns each one's type name, the array
/// length in the signature, and the array literal's own contents with
/// `//` comments stripped. `MessageKey::all()` returns `Vec<MessageKey>`,
/// not `[X; N]`, so it never matches this pattern — this only ever
/// finds the fixed-size label enums.
fn label_enum_all_blocks(source: &str) -> Vec<LabelEnumAll> {
    const MARKER: &str = "pub fn all() -> [";
    let mut out = Vec::new();
    let mut search_from = 0;
    while let Some(rel_idx) = source[search_from..].find(MARKER) {
        let idx = search_from + rel_idx;
        let after_marker = &source[idx + MARKER.len()..];
        search_from = idx + MARKER.len();

        let name: String = after_marker
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let Some(after_semi) = after_marker[name.len()..].strip_prefix("; ") else {
            continue;
        };
        let digits: String = after_semi
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let Ok(declared_len) = digits.parse::<usize>() else {
            continue;
        };
        let after_digits = &after_semi[digits.len()..];
        let Some(fn_brace) = after_digits.find('{') else {
            continue;
        };
        let after_fn_brace = &after_digits[fn_brace + 1..];
        let Some(array_open) = after_fn_brace.find('[') else {
            continue;
        };
        let after_array_open = &after_fn_brace[array_open + 1..];
        let Some(array_close) = after_array_open.find(']') else {
            continue;
        };
        let array_body = &after_array_open[..array_close];
        let without_comments = array_body
            .lines()
            .map(|line| line.split("//").next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n");

        out.push(LabelEnumAll {
            name,
            declared_len,
            all_body_without_comments: without_comments,
        });
    }
    out
}

/// `QA-010` §2: `MessageKey::all()` is now provably complete, but 125
/// of its variants are parameterised by one of fourteen label enums
/// (`IssueStatusLabel`, `PriorityLabel`, ...), and those enums'
/// coverage of *themselves* is exactly their own hand-maintained
/// `all()` — the same shape `MessageKey::all()` had, one level down.
/// Reproduced first: `IssueStatusLabel::all()` shrunk from `[X; 3]`
/// (`Open`, `InProgress`, `Done`) to `[X; 2]` (`Done` dropped),
/// `cargo test --workspace` stayed at 202 passed, 0 failed — see
/// `evidence/section2-reproduce-plant.log` in the review request.
/// Nothing was broken before this handoff (`QA-010` §1): all fourteen
/// enums pass both checks below as the tree stands.
#[test]
fn every_label_enum_variant_appears_at_least_once_in_its_all() {
    let source = message_rs_source();
    let label_enums = label_enum_all_blocks(&source);
    assert!(
        label_enums.len() >= 14,
        "found only {} label enums with a `pub fn all() -> [X; N]` signature -- \
         expected at least the fourteen QA-010 named; the discovery pattern in \
         label_enum_all_blocks may no longer match this file's formatting",
        label_enums.len()
    );

    let mut offenders = Vec::new();
    for label_enum in &label_enums {
        let declared = declared_variant_names(&source, &label_enum.name);
        for variant in &declared {
            let needle = format!("{}::{variant}", label_enum.name);
            if !appears_at_word_boundary(&label_enum.all_body_without_comments, &needle) {
                offenders.push(format!("  {}::{variant}", label_enum.name));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "these label enum variants never appear in their own all() -- every \
         MessageKey variant parameterised by one of them is unchecked for that \
         value:\n{}",
        offenders.join("\n")
    );
}

/// `QA-010` §2's second half: membership alone passes a list that
/// names one variant twice and omits another, as long as a third
/// variant happens to still be named somewhere — the declared array
/// length is right there in the signature and catches that for free.
#[test]
fn every_label_enum_all_length_matches_its_declared_variant_count() {
    let source = message_rs_source();
    let label_enums = label_enum_all_blocks(&source);

    let mut offenders = Vec::new();
    for label_enum in &label_enums {
        let declared = declared_variant_names(&source, &label_enum.name);
        if declared.len() != label_enum.declared_len {
            offenders.push(format!(
                "  {}: `all()` declares [_; {}], but {} variants are declared on the enum",
                label_enum.name,
                label_enum.declared_len,
                declared.len()
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "these label enums' all() array length disagrees with their own declared \
         variant count:\n{}",
        offenders.join("\n")
    );
}
