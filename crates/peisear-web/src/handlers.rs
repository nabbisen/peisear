//! HTTP handlers grouped by resource.

pub mod api_users;
pub mod auth;
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
#[cfg(test)]
const VALIDATOR_DERIVE_MESSAGES: &[(&str, &str)] = &[
    (
        "handlers/auth.rs: RegisterForm.email",
        "Please enter a valid email address.",
    ),
    (
        "handlers/auth.rs: RegisterForm.display_name",
        "Display name must be between 1 and 80 characters.",
    ),
    (
        "handlers/auth.rs: RegisterForm.password",
        "Password must be at least 8 characters.",
    ),
    (
        "handlers/projects.rs: ProjectForm.name",
        "Name is required (max 120 chars).",
    ),
    (
        "handlers/projects.rs: ProjectForm.description",
        "Description must be under 4000 chars.",
    ),
    (
        "handlers/issues.rs: IssueForm.title",
        "Title is required (max 200 chars).",
    ),
    (
        "handlers/issues.rs: IssueForm.description",
        "Description too long (max 10,000 chars).",
    ),
];

#[cfg(test)]
mod tests {
    use super::VALIDATOR_DERIVE_MESSAGES;

    #[test]
    fn validator_derive_messages_contain_no_prohibited_vocabulary() {
        for (site, message) in VALIDATOR_DERIVE_MESSAGES {
            let violations = peisear_i18n::find_violations(message);
            assert!(
                violations.is_empty(),
                "{site} ({message:?}) contains prohibited vocabulary: {:?}",
                violations.iter().map(|v| v.phrase).collect::<Vec<_>>()
            );
        }
    }
}
