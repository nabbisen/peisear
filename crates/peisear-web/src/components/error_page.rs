//! Error page rendered from [`AppError::into_response`].

use axum::response::Html;
use leptos::prelude::*;

use peisear_i18n::MessageKey;

use super::layout::PublicShell;
use super::t;

#[component]
pub fn ErrorPage(status: u16, #[prop(into)] message: String) -> impl IntoView {
    view! {
        <PublicShell title=t(MessageKey::ErrorPageTitle)>
            <div class="max-w-md mx-auto mt-12">
                <div class="card bg-base-100 shadow border border-base-300">
                    <div class="card-body text-center">
                        <div class="text-5xl font-bold text-error">{status}</div>
                        <p class="text-base-content/70 mt-2">{message}</p>
                        <div class="card-actions justify-center mt-4">
                            <a href="/" class="btn btn-primary btn-sm">{t(MessageKey::ErrorPageGoHomeLink)}</a>
                        </div>
                    </div>
                </div>
            </div>
        </PublicShell>
    }
}

pub fn render_error(status: u16, message: String) -> Html<String> {
    super::render_to_html(move || view! { <ErrorPage status=status message=message/> })
}
