//! Login and registration pages.

use axum::response::Html;
use leptos::prelude::*;

use super::layout::PublicShell;

#[component]
fn Brand(subtitle: &'static str) -> impl IntoView {
    view! {
        <div class="text-center mb-6">
            <div class="text-2xl font-semibold tracking-tight">
                <span class="text-primary">"●"</span>" Issue Tracker"
            </div>
            <div class="text-sm text-base-content/60 mt-1">{subtitle}</div>
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
        <PublicShell title="Sign in — Issue Tracker">
            <div class="max-w-sm mx-auto mt-12">
                <Brand subtitle="Sign in to your workspace"/>
                <div class="card bg-base-100 shadow border border-base-300">
                    <form method="post" action="/login" class="card-body gap-3">
                        <FlashInline flash=flash/>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Email"</span></div>
                            <input type="email" name="email" autocomplete="email" required=true
                                   value=email
                                   class="input input-bordered input-sm w-full"/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Password"</span></div>
                            <input type="password" name="password" autocomplete="current-password"
                                   required=true minlength="8"
                                   class="input input-bordered input-sm w-full"/>
                        </label>

                        <button type="submit" class="btn btn-primary btn-sm mt-2">"Sign in"</button>
                        <div class="text-center text-xs mt-1 text-base-content/60">
                            "No account? "
                            <a href="/register" class="link link-primary">"Create one"</a>
                        </div>
                    </form>
                </div>
            </div>
        </PublicShell>
    }
}

/// Registration form page.
#[component]
pub fn RegisterPage(
    flash: Option<String>,
    email: String,
    display_name: String,
) -> impl IntoView {
    view! {
        <PublicShell title="Create account — Issue Tracker">
            <div class="max-w-sm mx-auto mt-12">
                <Brand subtitle="Create your account"/>
                <div class="card bg-base-100 shadow border border-base-300">
                    <form method="post" action="/register" class="card-body gap-3">
                        <FlashInline flash=flash/>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Display name"</span></div>
                            <input type="text" name="display_name" required=true maxlength="80"
                                   value=display_name
                                   class="input input-bordered input-sm w-full"/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">"Email"</span></div>
                            <input type="email" name="email" autocomplete="email" required=true
                                   value=email
                                   class="input input-bordered input-sm w-full"/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Password"</span>
                                <span class="label-text-alt text-xs opacity-60">"8+ characters"</span>
                            </div>
                            <input type="password" name="password" autocomplete="new-password"
                                   required=true minlength="8"
                                   class="input input-bordered input-sm w-full"/>
                        </label>

                        <button type="submit" class="btn btn-primary btn-sm mt-2">"Create account"</button>
                        <div class="text-center text-xs mt-1 text-base-content/60">
                            "Already have an account? "
                            <a href="/login" class="link link-primary">"Sign in"</a>
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
pub fn render_register(
    flash: Option<String>,
    email: String,
    display_name: String,
) -> Html<String> {
    super::render_to_html(move || {
        view! { <RegisterPage flash=flash email=email display_name=display_name/> }
    })
}
