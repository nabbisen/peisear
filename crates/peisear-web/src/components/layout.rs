//! Page layouts shared across all pages.
//!
//! - [`Base`] is the minimum HTML scaffold (`<!DOCTYPE>`, `<head>`,
//!   and an empty `<body>` that renders its children).
//! - [`AppShell`] wraps `Base` with the authenticated-user navbar and
//!   the flash-message banner.

use leptos::prelude::*;

use peisear_core::CurrentUser;

/// Minimum HTML scaffold. Children render inside `<main>`.
///
/// Tailwind + daisyUI are loaded from CDN so the app runs without a
/// Node toolchain. For production, ship them as local assets (see
/// README).
#[component]
pub fn Base(
    /// Page title shown in `<title>` and browser tab.
    #[prop(into)]
    title: String,
    /// Main page content.
    children: Children,
) -> impl IntoView {
    view! {
        <html lang="en" data-theme="corporate">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{title}</title>
                <link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css" rel="stylesheet"/>
                <script src="https://cdn.tailwindcss.com/3.4.15"></script>
                <link rel="stylesheet" href="/static/app.css"/>
                <script inner_html="tailwind.config = { darkMode: ['class', '[data-theme=\"dark\"]'] };"></script>
            </head>
            <body class="min-h-screen bg-base-200 text-base-content">
                {children()}
            </body>
        </html>
    }
}

/// Shell used for every authenticated page: navbar + flash + main.
#[component]
pub fn AppShell(
    #[prop(into)] title: String,
    user: CurrentUser,
    flash: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <Base title=title>
            <Navbar user=user/>
            <main class="container mx-auto px-4 py-6 max-w-6xl">
                <FlashBar flash=flash/>
                {children()}
            </main>
        </Base>
    }
}

/// Shell used for unauthenticated pages (login, register, error):
/// just the bare main container, no navbar.
#[component]
pub fn PublicShell(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Base title=title>
            <main class="container mx-auto px-4 py-6 max-w-6xl">
                {children()}
            </main>
        </Base>
    }
}

#[component]
fn Navbar(user: CurrentUser) -> impl IntoView {
    view! {
        <header class="navbar bg-base-100 shadow-sm border-b border-base-300 px-4">
            <div class="flex-1">
                <a href="/projects" class="text-lg font-semibold tracking-tight">
                    <span class="text-primary">"●"</span>" Issue Tracker"
                </a>
            </div>
            <div class="flex-none gap-2">
                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost btn-sm normal-case">
                        {user.display_name.clone()}
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                             fill="none" stroke="currentColor" stroke-width="2"
                             stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="6 9 12 15 18 9"/>
                        </svg>
                    </label>
                    <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-48 border border-base-300">
                        <li class="menu-title"><span class="text-xs opacity-70">{user.email}</span></li>
                        <li>
                            <form method="post" action="/logout">
                                <button type="submit" class="text-error">"Sign out"</button>
                            </form>
                        </li>
                    </ul>
                </div>
            </div>
        </header>
    }
}

#[component]
fn FlashBar(flash: Option<String>) -> impl IntoView {
    flash.map(|msg| {
        view! {
            <div role="alert" class="alert alert-info mb-4 text-sm">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                     class="stroke-current shrink-0 w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                <span>{msg}</span>
            </div>
        }
    })
}
