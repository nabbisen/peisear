//! `/settings/notifications` page.
//!
//! Layout (top to bottom):
//!
//! 1. Header: title + "Silence all" link to its right (subtle,
//!    not prominent — the user can always find it but it isn't
//!    bait).
//! 2. A line stating the current email opt-in status
//!    (`email_globally_on`). The prompt that sets it lives at
//!    `/inbox` now, not here (`INBOX-001`, RFC 003 D2) — this
//!    page only reports the resulting state.
//! 3. **Per-kind preferences table** wrapped in a folded
//!    `<details>`. Default closed; users who want defaults
//!    don't need to open it.
//!
//! Form encoding for per-kind table:
//!
//! - One `<form>` covers all kinds. Submit is one POST to
//!   `/settings/notifications`.
//! - Channel checkbox names: `channel__{kind}__{channel}`.
//! - Min-severity radio names: `min_severity__{kind}`.
//!   Values: `info` (default) or `watch`.
//!
//! The flat naming avoids needing a nested-struct deserialiser.
//! See [`super::super::handlers::notification_preferences::save_preferences`]
//! for the matching parser.

use axum::response::Html;
use leptos::prelude::*;
use std::collections::HashMap;

use peisear_core::CurrentUser;
use peisear_core::notifications::{
    Preference, Severity,
    channel::{EMAIL, IN_APP, WEBHOOK},
    kind,
};
use peisear_i18n::{MessageKey, NotificationChannelLabel, NotificationKindLabel};

use super::layout::AppShell;
use super::{kind_label_for, t};

#[component]
pub fn PreferencesPage(
    user: CurrentUser,
    prefs: Vec<Preference>,
    email_globally_on: bool,
    unread_count: i64,
) -> impl IntoView {
    // Lookup map: kind -> Preference. Absent keys mean the
    // user has not configured that kind, so the row renders
    // with system defaults (in_app on, others off, severity
    // info).
    let pref_map: HashMap<String, Preference> =
        prefs.into_iter().map(|p| (p.kind.clone(), p)).collect();

    let kind_rows = kind::all_user_facing()
        .iter()
        .map(|k| {
            let pref_clone = pref_map.get(*k).cloned();
            render_kind_row(k, pref_clone)
        })
        .collect_view();

    let email_status = if email_globally_on {
        view! {
            <span class="text-xs text-success">{t(MessageKey::EmailOptInOnStatus)}</span>
        }
        .into_any()
    } else {
        view! {
            <span class="text-xs text-base-content/70">
                {t(MessageKey::EmailOptInOffStatus)}
            </span>
        }
        .into_any()
    };

    let notifications_heading = t(MessageKey::NotificationsSectionName);
    let notifications_breadcrumb = notifications_heading.clone();

    view! {
        <AppShell title=t(MessageKey::NotificationPreferencesPageTitle)
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/settings">{t(MessageKey::SettingsSectionName)}</a></li>
                    <li>{notifications_breadcrumb}</li>
                </ul></div>
                <div class="flex items-center justify-between mb-4">
                    <h1 class="text-xl font-semibold">{notifications_heading}</h1>
                    <form method="post" action="/settings/notifications/silence-all"
                          onsubmit="return confirm('Silence all notification kinds? \
                                                    You can re-enable them any time.')">
                        <button type="submit" class="btn btn-sm btn-ghost text-base-content/70"
                                aria-label=t(MessageKey::SilenceAllAriaLabel)>
                            {t(MessageKey::SilenceAllButton)}
                        </button>
                    </form>
                </div>

                <p class="text-sm text-base-content/70 mb-3">
                    {t(MessageKey::DefaultsInAppLead)} {email_status}
                </p>

                <details class="card bg-base-100 border border-base-300 shadow-sm">
                    <summary class="card-body cursor-pointer py-3 flex flex-row items-center justify-between gap-2">
                        <span class="font-medium">{t(MessageKey::PerKindDeliverySummary)}</span>
                        <span class="text-xs text-base-content/70">
                            {t(MessageKey::ClickToExpandHint)}
                        </span>
                    </summary>
                    <div class="px-4 pb-4">
                        <form method="post" action="/settings/notifications" class="space-y-4">
                            <div class="overflow-x-auto">
                                <table class="table table-sm" aria-label=t(MessageKey::NotificationKindsTableAriaLabel)>
                                    <thead>
                                        <tr>
                                            <th>{t(MessageKey::KindColumnHeading)}</th>
                                            <th class="text-center">{t(MessageKey::NotificationChannelName { channel: NotificationChannelLabel::InApp })}</th>
                                            <th class="text-center">{t(MessageKey::NotificationChannelName { channel: NotificationChannelLabel::Email })}</th>
                                            <th class="text-center">{t(MessageKey::NotificationChannelName { channel: NotificationChannelLabel::Webhook })}</th>
                                            <th>{t(MessageKey::MinSeverityColumnHeading)}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {kind_rows}
                                    </tbody>
                                </table>
                            </div>
                            <p class="text-xs text-base-content/70 italic">
                                {t(MessageKey::ChannelStubDisclaimer)}
                            </p>
                            <div class="text-right">
                                <button type="submit" class="btn btn-sm btn-primary">
                                    {t(MessageKey::SavePreferencesButton)}
                                </button>
                            </div>
                        </form>
                    </div>
                </details>
            </div>
        </AppShell>
    }
}

fn render_kind_row(kind_id: &'static str, pref: Option<Preference>) -> impl IntoView {
    let label = kind_label_for(kind_id);
    let label_text = label
        .map(|k| t(MessageKey::NotificationKindName { kind: k }))
        .unwrap_or_else(|| kind_id.to_string());

    let in_app_checked = match &pref {
        Some(p) => p.channels.iter().any(|c| c == IN_APP),
        // Default smart: in-app is on for unconfigured kinds.
        None => true,
    };
    let email_checked = match &pref {
        Some(p) => p.channels.iter().any(|c| c == EMAIL),
        None => false,
    };
    let webhook_checked = match &pref {
        Some(p) => p.channels.iter().any(|c| c == WEBHOOK),
        None => false,
    };
    let min_sev = pref
        .as_ref()
        .map(|p| p.min_severity)
        .unwrap_or(Severity::Info);
    let info_selected = matches!(min_sev, Severity::Info);

    let in_app_name = format!("channel__{kind_id}__{IN_APP}");
    let email_name = format!("channel__{kind_id}__{EMAIL}");
    let webhook_name = format!("channel__{kind_id}__{WEBHOOK}");
    let sev_name = format!("min_severity__{kind_id}");

    let row_kind = label.unwrap_or(NotificationKindLabel::BurnoutOverload);

    view! {
        <tr aria-label=t(MessageKey::NotificationKindPreferencesAriaLabel { kind: row_kind })>
            <td class="font-medium">{label_text}</td>
            <td class="text-center">
                <input type="checkbox" name=in_app_name class="checkbox"
                       checked=in_app_checked
                       aria-label=t(MessageKey::InAppForKindAriaLabel { kind: row_kind })/>
            </td>
            <td class="text-center">
                <input type="checkbox" name=email_name class="checkbox"
                       checked=email_checked
                       aria-label=t(MessageKey::EmailForKindAriaLabel { kind: row_kind })/>
            </td>
            <td class="text-center">
                <input type="checkbox" name=webhook_name class="checkbox"
                       checked=webhook_checked
                       aria-label=t(MessageKey::WebhookForKindAriaLabel { kind: row_kind })/>
            </td>
            <td>
                <select name=sev_name class="select select-bordered select-xs"
                        aria-label=t(MessageKey::MinSeverityForKindAriaLabel { kind: row_kind })>
                    <option value="info" selected=info_selected>{t(MessageKey::AllSeverityOption)}</option>
                    <option value="watch" selected={!info_selected}>{t(MessageKey::WatchOnlySeverityOption)}</option>
                </select>
            </td>
        </tr>
    }
}

pub fn render_preferences(
    user: CurrentUser,
    prefs: Vec<Preference>,
    email_globally_on: bool,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <PreferencesPage
                user=user
                prefs=prefs
                email_globally_on=email_globally_on
                unread_count=unread_count
            />
        }
    })
}
