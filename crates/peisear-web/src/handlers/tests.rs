/// The seven `#[validate(message = "...")]` literals `t()` cannot
/// reach — `validator`'s `message` argument parses to a plain
/// `String` at macro-expansion time, so the derive attributes below
/// cannot reference this array, and it must be kept in sync by hand.
/// The vocabulary guard is applied to these literals directly
/// instead (`I18N-006` §6: "the guard's purpose is that copy is
/// checked, not that it lives in the table"). Duplication against
/// the derive attributes is accepted here and only here, per the
/// handoff — a duplicated literal is a drift risk if either copy is
/// reworded without the other.
///
/// `I18N-007` §4 closes that drift risk: `validator_derive_messages_
/// still_match_their_source_files` asserts each literal below still
/// appears verbatim in the file `site` names, via `include_str!`.
/// Reword one side and only that test fails, instead of the two
/// copies silently diverging.
#[cfg(test)]
const VALIDATOR_DERIVE_MESSAGES: &[(&str, &str, &str)] = &[
    (
        "handlers/auth.rs",
        "RegisterForm.email",
        "Please enter a valid email address.",
    ),
    (
        "handlers/auth.rs",
        "RegisterForm.display_name",
        "Display name must be between 1 and 80 characters.",
    ),
    (
        "handlers/auth.rs",
        "RegisterForm.password",
        "Password must be at least 8 characters.",
    ),
    (
        "handlers/projects.rs",
        "ProjectForm.name",
        "Name is required (max 120 chars).",
    ),
    (
        "handlers/projects.rs",
        "ProjectForm.description",
        "Description must be under 4000 chars.",
    ),
    (
        "handlers/issues.rs",
        "IssueForm.title",
        "Title is required (max 200 chars).",
    ),
    (
        "handlers/issues.rs",
        "IssueForm.description",
        "Description too long (max 10,000 chars).",
    ),
];

use super::percent_encode_query;

/// `QA-020` §6 (RFC 005 §11): a flash message carrying every
/// character §2 named as the risk — `&`, `=`, `#`, `%`, `+`,
/// `?`, and a non-ASCII byte — must round-trip through the
/// query string byte-for-byte. No such message exists in this
/// codebase's copy today (§2: that is exactly why the old,
/// narrower space-only `str::replace` idiom was safe by luck
/// rather than by construction), so this message is synthetic —
/// §5 forbids new copy or `MessageKey`s, and this proves the
/// mechanism rather than any specific string.
#[derive(serde::Deserialize)]
struct FlashOnly {
    flash: String,
}

#[test]
fn percent_encode_query_survives_every_character_the_replace_idiom_could_not() {
    let message = "Sprint started & backlog updated — done? (100% #1, +1 more)";
    let encoded = percent_encode_query(message);
    let query = format!("flash={encoded}");
    let decoded: FlashOnly =
        serde_urlencoded::from_str(&query).expect("decode the encoded query string");
    assert_eq!(
        decoded.flash, message,
        "the flash message must round-trip through the query string byte-for-byte"
    );
}

#[test]
fn validator_derive_messages_contain_no_prohibited_vocabulary() {
    for (file, field, message) in VALIDATOR_DERIVE_MESSAGES {
        let violations = peisear_i18n::find_violations(message);
        assert!(
            violations.is_empty(),
            "{file} {field} ({message:?}) contains prohibited vocabulary: {:?}",
            violations.iter().map(|v| v.phrase).collect::<Vec<_>>()
        );
    }
}

/// `I18N-007` §4: closes the disclosed drift risk between this
/// array and the `#[validate(message = "...")]` attributes it
/// duplicates. `include_str!`'s path is relative to this file
/// (`src/handlers/tests.rs`, moved here by `DOC-007` from
/// `src/handlers.rs` — the three source files are now siblings
/// rather than one directory down), so each of the three source
/// files is read once and matched by name.
#[test]
fn validator_derive_messages_still_match_their_source_files() {
    const AUTH_RS: &str = include_str!("auth.rs");
    const PROJECTS_RS: &str = include_str!("projects.rs");
    const ISSUES_RS: &str = include_str!("issues.rs");

    for (file, field, message) in VALIDATOR_DERIVE_MESSAGES {
        let source = match *file {
            "handlers/auth.rs" => AUTH_RS,
            "handlers/projects.rs" => PROJECTS_RS,
            "handlers/issues.rs" => ISSUES_RS,
            other => panic!("VALIDATOR_DERIVE_MESSAGES names an unrecognised file: {other}"),
        };
        assert!(
            source.contains(message),
            "{file} {field}: {message:?} no longer appears verbatim in {file} -- \
             the #[validate(message = \"...\")] attribute and this array have diverged"
        );
    }
}
