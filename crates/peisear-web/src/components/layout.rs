//! Page layouts shared across all pages.
//!
//! - [`Base`] is the minimum HTML scaffold (`<!DOCTYPE>`, `<head>`,
//!   and an empty `<body>` that renders its children).
//! - [`AppShell`] wraps `Base` with the authenticated-user navbar and
//!   the flash-message banner.

use leptos::prelude::*;

use peisear_core::CurrentUser;
use peisear_i18n::MessageKey;

use super::{grow, t};

/// Minimum HTML scaffold. Children render inside `<main>`.
///
/// `ASSET-001` (`DEC-051`): Tailwind and DaisyUI are vendored under
/// `static/`, not loaded from CDN. `NFR-CMP-002` says self-hostable;
/// with both CDNs unreachable the pre-`ASSET-001` app rendered
/// unstyled -- `.btn` at 17px instead of 44px, the account menu
/// unable to collapse, `.btn` `display: inline` instead of `flex` --
/// so every guarantee `RFC 012` established was contingent on a third
/// party being reachable by the end user's browser. `cargo build` now
/// produces a fully self-contained styled application; see
/// `style/tailwindcss/README.md` for how the two vendored files are
/// pinned and regenerated.
///
/// **Stylesheet order matches the CDN-served page's own cascade,
/// verified empirically rather than assumed** (`document.styleSheets`
/// against the running, pre-`ASSET-001` app): DaisyUI's `<link>`
/// loads synchronously in document order, but the Play CDN `<script>`
/// this replaced injected its generated `<style>` tag at the *end* of
/// `<head>` -- after `app.css`, not at the script tag's own position.
/// `static/tailwind.css` keeps that same last position so a
/// same-specificity conflict (`app.css`'s own `.line-clamp-2` and
/// Tailwind's native utility of the same name both exist) resolves
/// the same way it did before.
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
                <link href="/static/daisyui.min.css" rel="stylesheet"/>
                <link rel="stylesheet" href="/static/app.css"/>
                <link href="/static/tailwind.css" rel="stylesheet"/>
            </head>
            <body class="min-h-screen bg-base-200 text-base-content">
                {children()}
            </body>
        </html>
    }
}

/// Shell used for every authenticated page: navbar + flash + main.
///
/// `unread_count` drives the bell badge in the navbar. Threaded
/// through every handler that returns the shell so the badge
/// stays in sync with reality across page loads.
#[component]
pub fn AppShell(
    #[prop(into)] title: String,
    user: CurrentUser,
    flash: Option<String>,
    #[prop(default = 0)] unread_count: i64,
    children: Children,
) -> impl IntoView {
    view! {
        <Base title=title>
            <Navbar user=user unread_count=unread_count/>
            <main class="container mx-auto px-4 py-6 max-w-6xl">
                <FlashBar flash=flash/>
                {children()}
            </main>
            // Global search typeahead. Loaded once per authed
            // page because the navbar input lives in every
            // AppShell render. Deferred so the script doesn't
            // block first paint — the input remains usable as a
            // plain HTML form (Enter submits to /search) until
            // the JS finishes parsing.
            <script src="/static/search.js" defer=true></script>
        </Base>
    }
}

/// Shell used for unauthenticated pages (login, register, error):
/// just the bare main container, no navbar.
#[component]
pub fn PublicShell(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <Base title=title>
            <main class="container mx-auto px-4 py-6 max-w-6xl">
                {children()}
            </main>
        </Base>
    }
}

#[component]
fn Navbar(user: CurrentUser, unread_count: i64) -> impl IntoView {
    // Pre-bound: selecting the right variant needs the conditional
    // below (`I18N-005a-review.md` §6's first pre-bind case).
    let bell_aria = t(if unread_count > 0 {
        MessageKey::NavBellLabelUnread {
            count: unread_count,
        }
    } else {
        MessageKey::NavBellLabelNone
    });
    let unread_badge = (unread_count > 0).then(|| {
        view! {
            <span class="badge badge-sm badge-primary absolute -top-1 -right-1 px-1 min-h-0 h-4">
                {t(MessageKey::NavBellCount { count: unread_count })}
            </span>
        }
    });

    view! {
        <header class="navbar bg-base-100 shadow-sm border-b border-base-300 px-4">
            <div class="flex-1">
                <a href="/projects" class=grow("text-lg font-semibold tracking-tight inline-flex items-center")>
                    <span class="text-primary">"●"</span>" "{t(MessageKey::AppBrandName)}
                </a>
            </div>

            // Global search (Phase A Step 4, v2.1 §4.5).
            // The form submits to /search for the HTML results
            // page. The vanilla JS at /static/search.js attaches
            // a typeahead dropdown to this input via the data
            // attribute below — JS-disabled clients still get a
            // working search box, just without the dropdown
            // preview.
            //
            // The whole block is wrapped in `relative` so the
            // dropdown's `absolute` positioning can be relative
            // to the input, not the whole navbar. `min-w` keeps
            // the box from collapsing on small viewports; mobile
            // refinement (drawer placement) is Phase E mobile QA.
            <div class="flex-none mx-2 hidden sm:block">
                <form method="get" action="/search"
                      class="relative"
                      role="search"
                      aria-label=t(MessageKey::NavSearchFormLabel)>
                    <input type="search"
                           name="q"
                           placeholder=t(MessageKey::NavSearchPlaceholder)
                           autocomplete="off"
                           class=grow("input input-bordered input-sm w-64")
                           data-typeahead="global"
                           aria-label=t(MessageKey::NavSearchQueryLabel)/>
                    // Container the JS populates with the
                    // typeahead dropdown. Empty until the
                    // server returns hits.
                    <div data-typeahead-dropdown=""
                         class="absolute left-0 right-0 mt-1 z-50 hidden bg-base-100 border border-base-300 rounded-md shadow-lg max-h-96 overflow-y-auto"
                         role="listbox"
                         aria-label=t(MessageKey::NavSearchSuggestionsLabel)>
                    </div>
                </form>
            </div>

            // `LAYOUT-006`: this block was `flex-none`, and that — not any
            // `min-width: auto` — was what made every page overflow a 320px
            // phone by 24px for a 21-character display name. `flex-none` is
            // `flex-shrink: 0`, so nothing on this side of the row could
            // give anything up and `min-w-0` was inert at every level of the
            // nesting (measured, one level at a time, before any edit).
            // Dropping `flex-none` is the whole change here: a flex item's
            // default is `flex: 0 1 auto`, so removing it restores the
            // shrink that `flex: none` had taken away. Better than pairing
            // `flex-none` with a `shrink` that contradicts it and resolves
            // on stylesheet order.
            //
            // The name gives way and nothing else does: the bell is
            // `shrink-0` below, and the button's own `min-w-11` (via
            // `grow()`) floors it at 44px, so `NFR-A11Y-007` holds without a
            // rule of its own — measured at 44x44 with the name removed
            // entirely, which is also the proof that brand + bell + button
            // do fit 320 and the floor is not the problem (§5's first
            // escalation, checked and not triggered).
            <div class="min-w-0 gap-2">
                <a href="/inbox"
                   class=grow("btn btn-ghost btn-sm relative shrink-0")
                   aria-label=bell_aria>
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18"
                         viewBox="0 0 24 24" fill="none" stroke="currentColor"
                         stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                         aria-hidden="true">
                        <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/>
                        <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"/>
                    </svg>
                    {unread_badge}
                </a>
                <div class="dropdown dropdown-end min-w-0">
                    // `LAYOUT-006`: the name shows at every width and gives
                    // way with an ellipsis only when the row cannot fit —
                    // no breakpoint, no character limit, nothing hidden on
                    // phones. `max-w-full` is the load-bearing one and the
                    // least obvious: without it the label keeps its
                    // content width, the `<span>` never reaches its
                    // `truncate`, and 320 stalls 2px short (measured:
                    // 24 -> 2 without it, 24 -> 0 with it). The `<span>`
                    // exists so there is something to truncate — a bare
                    // text node cannot be.
                    //
                    // `flex-nowrap` because DaisyUI's own `.btn` sets
                    // `flex-wrap: wrap`: once the button starts shrinking,
                    // the chevron drops onto a second line inside it and
                    // the layout is what gives way instead of the name.
                    // Costs 22px of visible name at 320 and keeps the
                    // button one line, which is the decision this handoff
                    // made.
                    <label tabindex="0" class=grow("btn btn-ghost btn-sm normal-case min-w-0 max-w-full flex-nowrap")>
                        <span class="truncate min-w-0">{user.display_name.clone()}</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
                             fill="none" stroke="currentColor" stroke-width="2"
                             stroke-linecap="round" stroke-linejoin="round"
                             class="shrink-0">
                            <polyline points="6 9 12 15 18 9"/>
                        </svg>
                    </label>
                    <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-48 border border-base-300">
                        // `LAYOUT-001`: an unbreakable email (no spaces) is a flex
                        // item that ignores `w-48` unless `min-w-0` opts it out of
                        // the default content-based minimum; without it, and while
                        // the menu is closed (`visibility: hidden`, still in layout),
                        // the escaped width drove the whole document's horizontal
                        // scroll. `overflow-hidden` + `truncate` keep it inside the
                        // box in both the closed and open states.
                        <li class="menu-title w-full min-w-0 overflow-hidden"><span class="text-xs opacity-70 truncate">{user.email}</span></li>
                        // `TT-004`: DaisyUI's own `.menu` selector already
                        // blockifies a plain <a> here to `display: grid` with
                        // `align-items: center`, so the target grows without
                        // needing an explicit flex wrapper -- verified by
                        // measurement, not assumed (`TT-004` §2's own trap).
                        <li><a href="/today" class=grow("")>{t(MessageKey::NavLinkToday)}</a></li>
                        <li><a href="/teams" class=grow("")>{t(MessageKey::NavLinkTeams)}</a></li>
                        <li><a href="/inbox" class=grow("")>{t(MessageKey::NavLinkInbox)}</a></li>
                        <li><a href="/settings" class=grow("")>{t(MessageKey::NavLinkSettings)}</a></li>
                        <li>
                            <form method="post" action="/logout">
                                // The DaisyUI menu-item grid/padding lands on
                                // this <form> (its own direct-child selector
                                // target), not the <button> inside it, so the
                                // actual clickable element needed its own
                                // target -- reusing the .btn family (already
                                // guarded, already centers correctly) rather
                                // than hand-building flex centering.
                                <button type="submit"
                                        class=grow("btn btn-ghost btn-sm text-error w-full justify-start normal-case")>
                                    {t(MessageKey::NavSignOut)}
                                </button>
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
