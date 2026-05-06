//! Inbox page for `/inbox`.
//!
//! Renders the user's notification history with read/unread
//! distinction, mark-as-read action per row, and a "mark all
//! read" affordance. Each row links to a relevant page
//! (e.g. `/today` for burnout kinds) when applicable.
//!
//! Design notes:
//! - Unread rows render with a subtle left-border accent;
//!   read rows fade slightly. Avoids loud "NEW" badges that
//!   pressure the user to clear the inbox.
//! - The "mark all read" button is in the header, not at the
//!   bottom, so it's reachable without scrolling. We don't
//!   confirm — the action is reversible only conceptually
//!   (you've now read them all), and adding a confirm step
//!   adds friction to a low-stakes action.
//! - Empty state explicitly says "you'll see notifications
//!   when something needs a glance" — frames the inbox as a
//!   reflective surface, not a queue.

use axum::response::Html;
use leptos::prelude::*;

use peisear_core::CurrentUser;
use peisear_core::notifications::{Notification, Severity, kind};

use super::layout::AppShell;

#[component]
pub fn InboxPage(
    user: CurrentUser,
    items: Vec<Notification>,
    unread_count: i64,
    flash: Option<String>,
) -> impl IntoView {
    let has_items = !items.is_empty();
    let has_unread = unread_count > 0;

    let header_status_text = if items.is_empty() {
        "No notifications yet.".to_string()
    } else if has_unread {
        format!("{} unread of {}.", unread_count, items.len())
    } else {
        format!("All read. {} total.", items.len())
    };

    let row_views = items.into_iter().map(render_row).collect_view();

    view! {
        <AppShell title="Notifications".to_string()
                  user=user
                  flash=flash
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="flex items-center justify-between mb-4">
                    <div>
                        <h1 class="text-xl font-semibold">"Notifications"</h1>
                        <p class="text-sm text-base-content/60">{header_status_text}</p>
                    </div>
                    {has_unread.then(|| view! {
                        <form method="post" action="/inbox/mark-all-read">
                            <button type="submit" class="btn btn-sm btn-ghost"
                                    aria-label="Mark all notifications as read">
                                "Mark all read"
                            </button>
                        </form>
                    })}
                </div>

                {(!has_items).then(|| view! {
                    <div class="card bg-base-100 border border-base-300 shadow-sm">
                        <div class="card-body items-center text-center py-12">
                            <div class="text-base-content/30 mb-2">
                                <svg xmlns="http://www.w3.org/2000/svg" width="36" height="36"
                                     viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                     stroke-width="1.5" aria-hidden="true">
                                    <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/>
                                    <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"/>
                                </svg>
                            </div>
                            <p class="text-sm text-base-content/60">
                                "You'll see notifications here when something needs a glance — "
                                "warnings about your workload, project health changes, that sort of thing."
                            </p>
                            <p class="text-xs text-base-content/50 mt-1">
                                "Configure delivery in "
                                <a href="/settings/notifications" class="link link-primary">"settings"</a>
                                "."
                            </p>
                        </div>
                    </div>
                })}

                {has_items.then(|| view! {
                    <ul class="space-y-2" aria-label="Notification list">
                        {row_views}
                    </ul>
                })}
            </div>
        </AppShell>
    }
}

fn render_row(n: Notification) -> impl IntoView {
    let is_unread = n.read_at.is_none();
    let severity_class = match n.severity {
        Severity::Watch => "border-l-warning",
        Severity::Info => "border-l-info",
    };
    let unread_class = if is_unread { "" } else { "opacity-70" };
    let card_class = format!(
        "card bg-base-100 border border-base-300 border-l-4 {} {} shadow-sm",
        severity_class, unread_class
    );

    let kind_label = kind::human_name(&n.kind).to_string();
    let timestamp = n.created_at.format("%Y-%m-%d %H:%M UTC").to_string();
    let aria = format!(
        "{} notification: {} ({}, {}).",
        if is_unread { "Unread" } else { "Read" },
        n.title,
        kind_label,
        timestamp,
    );

    let link_target = link_target_for_kind(&n.kind);
    let mark_read_action = format!("/inbox/{}/read", n.id);
    let dispatched_text = if n.dispatched_via.is_empty() {
        None
    } else {
        Some(
            n.dispatched_via
                .iter()
                .map(|c| peisear_core::notifications::channel::human_name(c).to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    };

    view! {
        <li class=card_class aria-label=aria>
            <div class="card-body p-4">
                <div class="flex items-start justify-between gap-3">
                    <div class="flex-1 min-w-0">
                        <div class="flex items-baseline gap-2 flex-wrap">
                            {is_unread.then(|| view! {
                                <span class="badge badge-xs badge-primary"
                                      aria-label="Unread">
                                    "•"
                                </span>
                            })}
                            <h3 class="font-medium">{n.title}</h3>
                            <span class="text-xs text-base-content/50">
                                {kind_label.clone()}
                            </span>
                        </div>
                        <p class="text-sm text-base-content/70 mt-1">{n.body}</p>
                        <div class="text-xs text-base-content/50 mt-2 flex items-center gap-3 flex-wrap">
                            <span>{timestamp}</span>
                            {dispatched_text.map(|t| view! {
                                <span class="opacity-70">
                                    "Sent via " {t}
                                </span>
                            })}
                            {link_target.map(|t| view! {
                                <a href={t} class="link link-primary">
                                    "View context →"
                                </a>
                            })}
                        </div>
                    </div>
                    {is_unread.then(|| view! {
                        <form method="post" action=mark_read_action>
                            <button type="submit"
                                    class="btn btn-ghost btn-xs"
                                    aria-label="Mark as read">
                                "Mark read"
                            </button>
                        </form>
                    })}
                </div>
            </div>
        </li>
    }
}

/// Where this kind of notification's "View context" link sends
/// the user. Burnout kinds go to `/today`; project trend kinds
/// will eventually take a `project_id` payload and link to that
/// project's page (placeholder for now).
fn link_target_for_kind(kind_str: &str) -> Option<String> {
    match kind_str {
        kind::BURNOUT_OVERLOAD | kind::BURNOUT_STALLED => Some("/today".to_string()),
        kind::PROJECT_TREND_DECLINE => Some("/projects".to_string()),
        _ => None,
    }
}

pub fn render_inbox(
    user: CurrentUser,
    items: Vec<Notification>,
    unread_count: i64,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <InboxPage
                user=user
                items=items
                unread_count=unread_count
                flash=flash
            />
        }
    })
}
