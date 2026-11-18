//! Login and registration pages.

use axum::response::Html;
use leptos::prelude::*;
use peisear_i18n::{Field, MessageKey};

use super::layout::PublicShell;
use super::{grow, t};

#[component]
fn Brand(subtitle: String) -> impl IntoView {
    view! {
        <div class="text-center mb-6">
            <div class="text-2xl font-semibold tracking-tight">
                <span class="text-primary">"●"</span>" "{t(MessageKey::AppBrandName)}
            </div>
            <div class="text-sm text-base-content/70 mt-1">{subtitle}</div>
        </div>
    }
}

#[component]
fn FlashInline(flash: Option<String>) -> impl IntoView {
    flash.map(|msg| view! { <div class="alert alert-warning text-xs py-2">{msg}</div> })
}

/// Login form page.
#[component]
pub fn LoginPage(flash: Option<String>, email: String) -> impl IntoView {
    view! {
        <PublicShell title=t(MessageKey::LoginPageTitle)>
            <div class="max-w-sm mx-auto mt-12">
                <Brand subtitle=t(MessageKey::SignInTaglineText)/>
                <div class="card bg-base-100 shadow border border-base-300">
                    <form method="post" action="/login" class="card-body gap-3">
                        <FlashInline flash=flash/>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Email })}</span></div>
                            <input type="email" name="email" autocomplete="email" required=true
                                   value=email
                                   class=grow("input input-bordered input-sm w-full")/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::PasswordFieldLabel)}</span></div>
                            <input type="password" name="password" autocomplete="current-password"
                                   required=true minlength="8"
                                   class=grow("input input-bordered input-sm w-full")/>
                        </label>

                        <button type="submit" class=grow("btn btn-primary btn-sm mt-2")>{t(MessageKey::SignInWord)}</button>
                        <div class="text-center text-xs mt-1 text-base-content/70">
                            {t(MessageKey::NoAccountPrompt)}
                            <a href="/register" class=grow("link link-primary inline-flex items-center align-middle")>{t(MessageKey::CreateOneLinkWord)}</a>
                        </div>
                    </form>
                </div>
            </div>
        </PublicShell>
    }
}

/// Registration form page.
#[component]
pub fn RegisterPage(flash: Option<String>, email: String, display_name: String) -> impl IntoView {
    view! {
        <PublicShell title=t(MessageKey::RegisterPageTitle)>
            <div class="max-w-sm mx-auto mt-12">
                <Brand subtitle=t(MessageKey::RegisterTaglineText)/>
                <div class="card bg-base-100 shadow border border-base-300">
                    <form method="post" action="/register" class="card-body gap-3">
                        <FlashInline flash=flash/>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::DisplayNameFieldLabel)}</span></div>
                            <input type="text" name="display_name" required=true maxlength="80"
                                   value=display_name
                                   class=grow("input input-bordered input-sm w-full")/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Email })}</span></div>
                            <input type="email" name="email" autocomplete="email" required=true
                                   value=email
                                   class=grow("input input-bordered input-sm w-full")/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::PasswordFieldLabel)}</span>
                                // `LBL-001` (series `063`): raised to
                                // `text-sm`, matching the label and
                                // the control beside it in the same
                                // row -- one size per row, the same
                                // shape `0.64.0` fixed on the five
                                // screens it read. Measured: +4px at
                                // both widths, no new wrap (the string
                                // is short at either size). Contrast
                                // measured as rendered, on this
                                // `bg-base-100` page: **6.51:1**,
                                // clears 4.5:1 with real margin.
                                // `LBL-001` round 2 (`§10.60`'s own
                                // condition): colour moved to the
                                // guarded `text-base-content/70` --
                                // this span is a single level deep
                                // with no faded ancestor (bare
                                // `opacity` fades an element and its
                                // subtree; `/70` fades text only, so
                                // the two are equivalent here and not
                                // in general), confirmed before
                                // converting, not assumed. Re-measured
                                // after: **6.42:1**, unchanged within
                                // pixel noise from the pre-conversion
                                // 6.51:1.
                                <span class="label-text-alt text-sm text-base-content/70">{t(MessageKey::PasswordMinLengthHint)}</span>
                            </div>
                            <input type="password" name="password" autocomplete="new-password"
                                   required=true minlength="8"
                                   class=grow("input input-bordered input-sm w-full")/>
                        </label>

                        <button type="submit" class=grow("btn btn-primary btn-sm mt-2")>{t(MessageKey::CreateAccountButton)}</button>
                        <div class="text-center text-xs mt-1 text-base-content/70">
                            {t(MessageKey::AlreadyHaveAccountPrompt)}
                            <a href="/login" class=grow("link link-primary inline-flex items-center align-middle")>{t(MessageKey::SignInWord)}</a>
                        </div>
                    </form>
                </div>
            </div>
        </PublicShell>
    }
}

/// Render [`LoginPage`] to an HTML response.
pub fn render_login(flash: Option<String>, email: String) -> Html<String> {
    super::render_to_html(move || view! { <LoginPage flash=flash email=email/> })
}

/// Render [`RegisterPage`] to an HTML response.
pub fn render_register(flash: Option<String>, email: String, display_name: String) -> Html<String> {
    super::render_to_html(move || {
        view! { <RegisterPage flash=flash email=email display_name=display_name/> }
    })
}
