//! HTTP handlers grouped by resource.

pub mod auth;
pub mod issues;
pub mod me;
pub mod notifications;
pub mod notification_preferences;
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
        "Invalid input.".to_string()
    } else {
        out.join(" ")
    }
}
