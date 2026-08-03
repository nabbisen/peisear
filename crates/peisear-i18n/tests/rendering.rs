//! Rendering-mechanism tests (`I18N-001` §5, tests 4–5): locale
//! switching is wholesale, and no raw key ever leaks into rendered
//! text.

mod common;

use peisear_i18n::{Locale, MessageKey};

/// Test 4: the fixture locale renders every key differently from
/// English — not just some keys, which would suggest the mechanism
/// quietly falls through to English somewhere in the middle (the
/// exact failure mode `I18N-001` §4.5 exists to catch).
#[test]
fn fixture_locale_renders_every_key_differently_from_english() {
    for key in MessageKey::all() {
        let english = Locale::English.render(key);
        let fixture = common::fixture_locale::render(key);
        assert_ne!(
            english, fixture,
            "{key:?} rendered identically in English and the fixture locale — \
             the mechanism may be silently falling through to English"
        );
        // Distinct is necessary but not sufficient — every fixture
        // rendering also carries its own `[fx` marker, so a switch
        // that produced *some other* English-shaped string (rather
        // than genuinely routing through the fixture table) would
        // still be caught.
        assert!(
            fixture.contains("[fx"),
            "{key:?}'s fixture rendering {fixture:?} doesn't look like it came from the fixture table"
        );
    }
}

/// Test 5: no rendered output contains a raw, key-shaped literal
/// (RFC 006's test plan: no `[a-z_]+\.[a-z_]+`-shaped text reaches
/// visible output). This crate has no real surfaces to check yet, so
/// this checks its own rendering path — the property a converted
/// surface will need to keep holding once handoff 4 wires this crate
/// into `peisear-web`.
#[test]
fn no_rendered_output_contains_a_key_shaped_literal() {
    fn looks_key_shaped(word: &str) -> bool {
        // `foo.bar_baz` shape: lowercase segments joined by a dot,
        // each segment lowercase-with-underscores. Matches how a
        // `Debug`-formatted key or a stray format-string typo would
        // look if it leaked through unrendered.
        let Some((left, right)) = word.split_once('.') else {
            return false;
        };
        let is_key_segment =
            |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase() || c == '_');
        is_key_segment(left) && is_key_segment(right)
    }

    for key in MessageKey::all() {
        for rendered in [
            Locale::English.render(key),
            common::fixture_locale::render(key),
        ] {
            for word in rendered.split_whitespace() {
                let trimmed = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.');
                assert!(
                    !looks_key_shaped(trimmed),
                    "{key:?} rendered {rendered:?}, which contains a key-shaped \
                     literal ({trimmed:?}) — a raw key leaked into visible text"
                );
            }
        }
    }
}
