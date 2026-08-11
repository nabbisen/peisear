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
use peisear_i18n::{MessageKey, NotificationChannelLabel, NotificationKindLabel};

use super::layout::AppShell;
use super::t;

/// See `notification_preferences::kind_label_for` — same shape, same
/// reason.
fn kind_label_for(kind_id: &str) -> Option<NotificationKindLabel> {
    match kind_id {
        kind::BURNOUT_OVERLOAD => Some(NotificationKindLabel::BurnoutOverload),
        kind::BURNOUT_STALLED => Some(NotificationKindLabel::BurnoutStalled),
        kind::PROJECT_TREND_DECLINE => Some(NotificationKindLabel::ProjectTrendDecline),
        _ => None,
    }
}

/// See `kind_label_for`. Notification rows can carry any channel
/// string a future release adds; unrecognised channels fall back to
/// their raw id, matching `channel::human_name`'s previous
/// `_ => id` behaviour.
fn channel_label_for(channel_id: &str) -> Option<NotificationChannelLabel> {
    match channel_id {
        peisear_core::notifications::channel::IN_APP => Some(NotificationChannelLabel::InApp),
        peisear_core::notifications::channel::EMAIL => Some(NotificationChannelLabel::Email),
        peisear_core::notifications::channel::WEBHOOK => Some(NotificationChannelLabel::Webhook),
        _ => None,
    }
}

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
        t(MessageKey::NoNotificationsYetStatus)
    } else if has_unread {
        t(MessageKey::UnreadOfTotalStatus {
            unread_count,
            total: items.len() as i64,
        })
    } else {
        t(MessageKey::AllReadStatus {
            total: items.len() as i64,
        })
    };

    let row_views = items.into_iter().map(render_row).collect_view();
    let notifications_heading = t(MessageKey::NotificationsSectionName);

    view! {
        <AppShell title=t(MessageKey::NotificationsSectionName)
                  user=user
                  flash=flash
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="flex items-center justify-between mb-4">
                    <div>
                        <h1 class="text-xl font-semibold">{notifications_heading}</h1>
                        <p class="text-sm text-base-content/60">{header_status_text}</p>
                    </div>
                    {has_unread.then(|| view! {
                        <form method="post" action="/inbox/mark-all-read">
                            <button type="submit" class="btn btn-sm btn-ghost"
                                    aria-label=t(MessageKey::MarkAllReadAriaLabel)>
                                {t(MessageKey::MarkAllReadButton)}
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
                                {t(MessageKey::InboxEmptyMessage)}
                            </p>
                            <p class="text-xs text-base-content/50 mt-1">
                                {t(MessageKey::InboxEmptyFooterLead)}
                                <a href="/settings/notifications" class="link link-primary">{t(MessageKey::SettingsLinkWord)}</a>
                                {t(MessageKey::InboxEmptyFooterTail)}
                            </p>
                        </div>
                    </div>
                })}

                {has_items.then(|| view! {
                    <ul class="space-y-2" aria-label=t(MessageKey::NotificationListAriaLabel)>
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

    let kind = kind_label_for(&n.kind);
    let kind_label = kind
        .map(|k| t(MessageKey::NotificationKindName { kind: k }))
        .unwrap_or_else(|| n.kind.clone());
    let timestamp = n.created_at.format("%Y-%m-%d %H:%M UTC").to_string();
    let aria = t(MessageKey::NotificationRowAriaLabel {
        is_unread,
        title: n.title.clone(),
        kind: kind.unwrap_or(NotificationKindLabel::BurnoutOverload),
        timestamp: timestamp.clone(),
    });

    let link_target = link_target_for_kind(&n.kind);
    let mark_read_action = format!("/inbox/{}/read", n.id);
    let dispatched_text = if n.dispatched_via.is_empty() {
        None
    } else {
        Some(
            n.dispatched_via
                .iter()
                .map(|c| {
                    channel_label_for(c)
                        .map(|ch| t(MessageKey::NotificationChannelName { channel: ch }))
                        .unwrap_or_else(|| c.clone())
                })
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
                                      aria-label=t(MessageKey::UnreadWord)>
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
                            {dispatched_text.map(|dispatched| view! {
                                <span class="opacity-70">
                                    {t(MessageKey::SentViaPrefix)} {dispatched}
                                </span>
                            })}
                            {link_target.map(|target| view! {
                                <a href={target} class="link link-primary">
                                    {t(MessageKey::ViewContextLinkLabel)}
                                </a>
                            })}
                        </div>
                    </div>
                    {is_unread.then(|| view! {
                        <form method="post" action=mark_read_action>
                            <button type="submit"
                                    class="btn btn-ghost btn-xs"
                                    aria-label=t(MessageKey::MarkAsReadAriaLabel)>
                                {t(MessageKey::MarkReadButton)}
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
