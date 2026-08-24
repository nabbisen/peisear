//! The confirmation interstitial (`CONF-001`, RFC 010).
//!
//! One component, three `GET` routes
//! (`handlers::{projects,issues,sprints}::delete_confirm`), four
//! originating controls — the sprint route serves both the planned
//! and completed cases via `consequence`, not a second component.
//!
//! A server-rendered page rather than a JS-only dialog, so the
//! confirmation cannot silently vanish when JavaScript is absent —
//! the exact defect the old inline confirm-dialog guard had (external
//! design §17.4, `DEC-021`). The originating control is now an
//! ordinary `<a>` to this page; the page's own form is what performs
//! the delete, posting to the same route the old dialog's button did.

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::CurrentUser;
use peisear_i18n::MessageKey;

use super::layout::AppShell;
use super::t;

#[component]
pub fn ConfirmDeletePage(
    user: CurrentUser,
    entity_name: String,
    consequence: String,
    confirm_action: String,
    cancel_href: String,
    /// Extra fields the target `POST` handler requires beyond the
    /// route's own path params — e.g. a sprint delete's
    /// `client_updated_at` for its optimistic-lock check. Empty for
    /// project/issue delete, which take no form body at all.
    hidden_fields: Vec<(String, String)>,
    #[prop(default = 0)] unread_count: i64,
) -> impl IntoView {
    let heading = t(MessageKey::ConfirmDeleteHeading { entity_name });
    let page_title = heading.clone();
    let hidden_inputs = hidden_fields
        .into_iter()
        .map(|(name, value)| view! { <input type="hidden" name=name value=value/> })
        .collect_view();

    view! {
        <AppShell title=page_title user=user flash={None::<String>} unread_count=unread_count>
            <div class="max-w-md mx-auto mt-8">
                <div class="card bg-base-100 border border-error/30 shadow-sm">
                    <div class="card-body">
                        <h1 class="text-lg font-semibold text-error">{heading}</h1>
                        <p class="text-sm text-base-content/70 mt-1">{consequence}</p>
                        <div class="card-actions justify-end mt-4">
                            <a href=cancel_href class="btn btn-ghost btn-sm">
                                {t(MessageKey::CancelButton)}
                            </a>
                            <form method="post" action=confirm_action>
                                {hidden_inputs}
                                <button type="submit" class="btn btn-error btn-sm">
                                    {t(MessageKey::DeleteButton)}
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </AppShell>
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_delete_confirmation(
    user: CurrentUser,
    entity_name: String,
    consequence: String,
    confirm_action: String,
    cancel_href: String,
    hidden_fields: Vec<(String, String)>,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <ConfirmDeletePage
                user=user
                entity_name=entity_name
                consequence=consequence
                confirm_action=confirm_action
                cancel_href=cancel_href
                hidden_fields=hidden_fields
                unread_count=unread_count
            />
        }
    })
}
