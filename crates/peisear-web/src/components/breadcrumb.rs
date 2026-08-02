//! Breadcrumb and back-link components.
//!
//! Phase A Step 2 (v2.1 spec §4.4) consolidates the breadcrumb
//! markup that previously lived inline in each detail page. The
//! goals of this consolidation:
//!
//! 1. **Consistency.** Every detail page begins with the same
//!    leading entry (`Today` link), terminates the chain with
//!    `aria-current="page"` on the current node, and uses a
//!    uniform truncation rule on long names. Inline copies had
//!    drifted (e.g. some had `Projects` as the leading entry
//!    and others had `Teams`, but none had `Today`, breaking
//!    the v2.1 navigation entry-point story).
//!
//! 2. **A back link beneath the breadcrumb.** v2.1 §4.4 calls for
//!    a "← Back to {parent}" affordance directly under the
//!    breadcrumb. On desktop this duplicates information the
//!    breadcrumb already conveys; on mobile, where the
//!    breadcrumb is truncated to fit, a fixed-position "back"
//!    button is the actually-tappable target.
//!
//! 3. **A single place to evolve the markup.** The eventual UX
//!    refresh in Phase B will likely change classes and spacing.
//!    A single component lets that change happen in one diff.
//!
//! ## Why not the daisyUI `breadcrumbs` class on the inline `<ul>`
//!
//! The previous markup used daisyUI's `breadcrumbs` utility,
//! which we keep. The component just generates the same
//! markup with consistent ARIA and the `aria-current="page"`
//! marker on the terminal item. No visual change.
//!
//! ## Trait vs struct
//!
//! We use a plain struct ([`BreadcrumbItem`]) and a free function
//! ([`render_breadcrumb`]) rather than a Leptos `#[component]`
//! macro. The reason is that this code is consumed by other
//! `#[component]` functions and embedding a `<Component/>` from
//! a Vec of dynamic items is awkward in Leptos's macro syntax —
//! a function that returns `impl IntoView` composes more
//! cleanly inside a parent `view!` block.

use leptos::prelude::*;

/// One node in a breadcrumb trail.
///
/// `href` is `None` for the **current page** — that node is
/// rendered as plain text and tagged `aria-current="page"`.
/// All other nodes are clickable links.
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

impl BreadcrumbItem {
    /// A clickable ancestor node.
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
        }
    }

    /// The terminal "you are here" node (not clickable).
    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }
}

/// Render a breadcrumb trail.
///
/// A leading `Today` entry is prepended automatically so that
/// every detail page traces back to the v2.1 navigation entry
/// point. Callers should pass the *intermediate* and *terminal*
/// nodes only.
///
/// Pass-by-value (`Vec<BreadcrumbItem>`) so the resulting view
/// can outlive the call site without lifetime concerns.
pub fn render_breadcrumb(items: Vec<BreadcrumbItem>) -> impl IntoView {
    // Prepend the v2.1 navigation entry point. Intentionally
    // hard-coded here rather than required from every caller —
    // it's the same string on every page and forgetting it would
    // break the consistency the consolidation is designed to
    // enforce.
    let mut all = Vec::with_capacity(items.len() + 1);
    all.push(BreadcrumbItem::link("Today", "/today"));
    all.extend(items);

    let nodes = all
        .into_iter()
        .map(|item| {
            // Wrap each label in a span so we can attach
            // truncation classes uniformly. Long project / issue
            // titles otherwise blow out the navbar on narrow
            // viewports.
            let label_span = view! {
                <span class="max-w-[24ch] truncate inline-block align-bottom">
                    {item.label}
                </span>
            };
            match item.href {
                Some(href) => view! {
                    <li><a href=href>{label_span}</a></li>
                }
                .into_any(),
                None => view! {
                    // aria-current="page" tells assistive tech
                    // this is the user's current location. The
                    // visual cue is just the absence of a link
                    // colour.
                    <li aria-current="page">{label_span}</li>
                }
                .into_any(),
            }
        })
        .collect::<Vec<_>>();

    view! {
        // role/aria-label make the daisyUI `breadcrumbs` div
        // findable as the page's secondary navigation landmark.
        <nav class="breadcrumbs text-sm" aria-label="Breadcrumb">
            <ul>{nodes}</ul>
        </nav>
    }
}

/// Render a "← Back to {label}" button targeting `href`.
///
/// Sits directly beneath the breadcrumb on detail pages. Adds a
/// finger-friendly tap target on mobile, where the breadcrumb
/// itself often has to be truncated to fit the viewport.
///
/// Implementation note: we deliberately render this as `<a>`,
/// not `<button onclick="history.back()">`, because the latter
/// produces a different page than the breadcrumb claims when
/// the user arrived via a deep link (e.g. an email). Linking to
/// the canonical parent URL is the predictable behaviour.
pub fn render_back_link(label: impl Into<String>, href: impl Into<String>) -> impl IntoView {
    let label = label.into();
    let href = href.into();
    let aria = format!("Back to {label}");
    view! {
        <a href=href
           class="inline-flex items-center gap-1 text-sm text-base-content/70 \
                  hover:text-base-content mb-3"
           aria-label=aria>
            <span aria-hidden="true">"← "</span>
            <span>"Back to " {label}</span>
        </a>
    }
}
