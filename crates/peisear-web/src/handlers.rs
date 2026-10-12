//! HTTP handlers grouped by resource.

pub mod api_users;
pub mod auth;
pub mod calendar;
pub mod issues;
pub mod me;
pub mod notification_preferences;
pub mod notifications;
pub mod projects;
pub mod redirects;
pub mod root;
pub mod search;
pub mod settings;
pub mod sprints;
pub mod teams;

/// URL-safe percent-encoding for redirect query strings —
/// `QA-020` (`NFR-*`, RFC 005 §11). One copy, shared by every
/// handler that puts dynamic text (a rendered flash/error message,
/// or a filter value) into a `Location` header's query component.
/// Avoids pulling in a `urlencoding` dependency for what used to be
/// a handful of call sites and is now the crate's one answer to
/// this problem.
///
/// Was two byte-for-byte identical copies (`handlers/teams.rs`,
/// `handlers/sprints.rs`), a third differently-named one
/// (`handlers/settings.rs`'s own `percent_encode_for_query`,
/// invisible to a search for this function's name), plus 23 sites
/// doing a narrower hand-rolled `str::replace` of the space
/// character alone — correct only because, at every site it was
/// ever applied to, the underlying copy happened to be plain ASCII
/// with no `&`, `=`, `#`, `%`, `+`, `?`, or non-ASCII byte.
/// `QA-018`'s audit found the first gap this project shipped by
/// exactly that kind of luck running out (`NFR-CONC-003`); this is
/// the same shape, caught before it did. `one_encoder_scan` bans the
/// narrower idiom's exact spelling by name — see there, not here,
/// for why it isn't quoted in this comment.
pub(crate) fn percent_encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            b' ' => out.push('+'),
            other => out.push_str(&format!("%{:02X}", other)),
        }
    }
    out
}

pub(crate) fn format_validation(errors: &validator::ValidationErrors) -> String {
    let mut out = Vec::new();
    for (_, errs) in errors.field_errors() {
        for e in errs {
            if let Some(msg) = &e.message {
                out.push(msg.to_string());
            } else {
                out.push(format!("{:?}", e.code));
            }
        }
    }
    if out.is_empty() {
        crate::components::t(peisear_i18n::MessageKey::InvalidInputFallbackMessage)
    } else {
        out.join(" ")
    }
}

#[cfg(test)]
mod tests;
