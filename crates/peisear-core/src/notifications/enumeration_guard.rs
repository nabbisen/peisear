//! `QA-010` §3 — `channel::ALL_CHANNELS` and `kind::all_user_facing()`
//! are `&str` constants, not enum variants, so there is no type for
//! the compiler to be exhaustive over: nothing stops a new `pub
//! const` from being added to either module without also being
//! added to its list. `peisear-web`'s
//! `components::every_declared_notification_kind_has_a_label`
//! already guards the next seam over (a kind with no i18n label
//! fails a test), but that guard's own reach is exactly
//! `all_user_facing()`'s completeness — the same shape one crate
//! further out. Both lists are complete today (`QA-010` §1: this is
//! a tripwire, not a repair).
//!
//! **Lives in its own `src/notifications/enumeration_guard.rs`**
//! (`DOC-007`, moved out of an inline module in `lib.rs`), not under a
//! top-level `tests/` directory: `peisear-core` has no `tests/`
//! directory, and its only CI job (`test-libs`) runs `cargo test -p
//! peisear-core --lib` — a file under a top-level `tests/` would build
//! a separate integration-test binary that job never invokes, so it
//! would compile locally and never run in CI, the exact defect
//! `QA-005` exists to prevent one layer up. A file declared with `mod
//! enumeration_guard;` from inside `notifications`, by contrast, is
//! still part of the library crate `--lib` compiles and runs — the
//! split changes where the text lives, not which binary runs it.
//!
//! **Reads the constant declarations as source text**, the same
//! family as `enumeration_guard.rs` in `peisear-i18n` — no
//! dependency added for two eight-line lists. `find_matching_block`
//! brace-counts rather than assuming a fixed indent depth (`kind`
//! and `channel`'s own bodies contain a function, whose own `{ }`
//! would break the "next unindented `}`" shortcut `message.rs`'s
//! scan relies on); it does not account for a `{`/`}` inside a
//! string literal, which neither module has today.

use super::channel;

/// `GLOBAL` is deliberately excluded from
/// `kind::all_user_facing()` — it is a sentinel preferences row,
/// never a real notification kind (see `kind::GLOBAL`'s own doc
/// comment). Named here, with its reason, the way
/// `static_js_scan` names `search.js` in its own allowlist —
/// visible in the code the guard runs, not only in a document
/// this file doesn't read.
///
/// `PROJECT_TREND_DECLINE` is excluded for a different reason, and this
/// is the same kind of named exclusion (`NTF-001`): it is a **real**
/// kind with **no emitter**, so offering a preference for it (and
/// requiring it silenced before the "everything is silenced" banner)
/// answers a question about a notification that cannot arrive. The
/// exclusion is honest only while that is true --
/// `peisear-notify`'s `edge::tests::every_emitted_kind_is_user_facing`
/// fails the day an emitter is written for it, and this test's own
/// second half fails if the kind is listed here *and* offered.
const KIND_EXCLUDED_FROM_ALL_USER_FACING: &[&str] = &["GLOBAL", "PROJECT_TREND_DECLINE"];

/// Everything between `source`'s `start_marker` and that
/// block's matching close brace, brace-depth counted so a
/// nested `{ }` (a function body inside the module) does not
/// end the scan early.
fn find_matching_block<'a>(source: &'a str, start_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("lib.rs contains `{start_marker}`"));
    let after_marker = &source[start + start_marker.len()..];
    let bytes = after_marker.as_bytes();
    let mut depth = 1i32;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return &after_marker[..i];
                }
            }
            _ => {}
        }
    }
    panic!("`{start_marker}` block never closes");
}

/// Everything between `source`'s `start_marker` (ending in the
/// array literal's own opening `[`) and that array's matching
/// close `]`. `[X; N]`-fixed-size array parsing has enum
/// variants to worry about nesting braces around; a plain
/// `&[A, B, C]` slice literal like `ALL_CHANNELS`'s has no
/// nested `[`/`]` at all in this codebase's style, so a single
/// close bracket is its own end — no depth counting needed.
fn find_matching_array_literal<'a>(source: &'a str, start_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("expected to find `{start_marker}`"));
    let after_marker = &source[start + start_marker.len()..];
    let end = after_marker
        .find(']')
        .unwrap_or_else(|| panic!("`{start_marker}` array literal never closes"));
    &after_marker[..end]
}

/// Every `pub const <NAME>: &str = "..."` declared directly in
/// `module_body` — a scalar string constant, distinct from an
/// aggregate like `ALL_CHANNELS: &[&str]`, which this pattern
/// does not match (its type is `&[&str]`, not `&str`).
fn declared_str_const_names(module_body: &str) -> Vec<String> {
    module_body
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("pub const ")?;
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_uppercase() || *c == '_' || c.is_ascii_digit())
                .collect();
            if name.is_empty() {
                return None;
            }
            let after_name = &rest[name.len()..];
            after_name
                .trim_start()
                .starts_with(": &str = \"")
                .then_some(name)
        })
        .collect()
}

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

#[test]
fn every_notification_kind_except_global_appears_in_all_user_facing() {
    let source = include_str!("../lib.rs");
    let kind_body = find_matching_block(source, "pub mod kind {");
    let declared = declared_str_const_names(kind_body);
    assert!(
        declared.len() >= 3,
        "found suspiciously few kind constants ({}) -- the module-body \
         parsing assumption this scan depends on may have changed",
        declared.len()
    );

    let all_user_facing_body = find_matching_block(
        kind_body,
        "pub fn all_user_facing() -> &'static [&'static str] {",
    );

    let missing: Vec<&String> = declared
        .iter()
        .filter(|name| !KIND_EXCLUDED_FROM_ALL_USER_FACING.contains(&name.as_str()))
        .filter(|name| !appears_at_word_boundary(all_user_facing_body, name))
        .collect();
    assert!(
        missing.is_empty(),
        "these notification kinds never appear in kind::all_user_facing() -- \
         every guard downstream that iterates it (peisear-web's \
         every_declared_notification_kind_has_a_label, the preferences page) \
         never sees them:\n{}",
        missing
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // The exclusion itself must still be real, the same way
    // `static_js_scan::search_js_allowlist_entry_still_matches_something`
    // proves its own allowlist entry isn't stale -- if `GLOBAL`
    // were ever added to `all_user_facing()`, that is a live bug
    // (a sentinel row rendering as a real kind), not a guard
    // update.
    for excluded in KIND_EXCLUDED_FROM_ALL_USER_FACING {
        assert!(
            !appears_at_word_boundary(all_user_facing_body, excluded),
            "{excluded} is listed as excluded from all_user_facing() but appears \
             in it -- either the exclusion is stale (remove it here: e.g. an emitter \
             now exists for PROJECT_TREND_DECLINE, which is the moment it belongs in \
             the list) or a sentinel is now rendering as a real notification kind \
             (a live bug)"
        );
    }
}

#[test]
fn every_channel_constant_appears_in_all_channels() {
    let source = include_str!("../lib.rs");
    let channel_body = find_matching_block(source, "pub mod channel {");
    let declared = declared_str_const_names(channel_body);
    assert!(
        declared.len() >= 3,
        "found suspiciously few channel constants ({}) -- the module-body \
         parsing assumption this scan depends on may have changed",
        declared.len()
    );

    let all_channels_body =
        find_matching_array_literal(channel_body, "pub const ALL_CHANNELS: &[&str] = &[");

    let missing: Vec<&String> = declared
        .iter()
        .filter(|name| !appears_at_word_boundary(all_channels_body, name))
        .collect();
    assert!(
        missing.is_empty(),
        "these channel constants never appear in channel::ALL_CHANNELS:\n{}",
        missing
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Sanity check: `channel::all()` is documented as returning
/// `ALL_CHANNELS` verbatim, which is what
/// `every_channel_constant_appears_in_all_channels` actually
/// scans. If `all()` ever stopped simply returning the constant,
/// this guard would be checking the wrong thing without any
/// test failing to say so -- this makes that assumption an
/// explicit, checked fact instead of an implicit one.
#[test]
fn channel_all_returns_all_channels_verbatim() {
    assert_eq!(channel::all(), channel::ALL_CHANNELS);
}
