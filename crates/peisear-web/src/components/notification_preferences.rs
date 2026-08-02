//! `/settings/notifications` page.
//!
//! Layout (top to bottom):
//!
//! 1. Header: title + "Silence all" link to its right (subtle,
//!    not prominent — the user can always find it but it isn't
//!    bait).
//! 2. **First-login email banner** — only when
//!    `global_acknowledged == false`. Two buttons: "Yes, send
//!    me email" and "Just in-app, thanks". Clicking either
//!    records acknowledgement and the banner stops appearing.
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

use super::layout::AppShell;

#[component]
pub fn PreferencesPage(
    user: CurrentUser,
    prefs: Vec<Preference>,
    global_acknowledged: bool,
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

    let banner = (!global_acknowledged).then(|| {
        view! {
            <section class="alert alert-info mb-4" role="status"
                     aria-label="First-time email setup prompt">
                <div class="flex-1">
                    <h2 class="font-medium">"Email notifications"</h2>
                    <p class="text-sm opacity-90 mt-1">
                        "Would you like notifications by email as well as in-app? "
                        "You can change this any time."
                    </p>
                </div>
                <form method="post" action="/settings/notifications/ack-global"
                      class="flex gap-2 flex-wrap items-center">
                    <input type="hidden" name="email_opt_in" value="yes"/>
                    <button type="submit" class="btn btn-sm btn-primary">
                        "Yes, send me email"
                    </button>
                </form>
                <form method="post" action="/settings/notifications/ack-global"
                      class="flex gap-2">
                    <input type="hidden" name="email_opt_in" value="no"/>
                    <button type="submit" class="btn btn-sm btn-ghost">
                        "Just in-app, thanks"
                    </button>
                </form>
            </section>
        }
    });

    let email_status = if email_globally_on {
        view! {
            <span class="text-xs text-success">"✓ Email opt-in is on by default."</span>
        }
        .into_any()
    } else {
        view! {
            <span class="text-xs text-base-content/60">
                "Email opt-in is off (in-app only by default). "
                "Per-kind overrides below."
            </span>
        }
        .into_any()
    };

    view! {
        <AppShell title="Notification preferences".to_string()
                  user=user
                  flash={None::<String>}
                  unread_count=unread_count>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/settings">"Settings"</a></li>
                    <li>"Notifications"</li>
                </ul></div>
                <div class="flex items-center justify-between mb-4">
                    <h1 class="text-xl font-semibold">"Notifications"</h1>
                    <form method="post" action="/settings/notifications/silence-all"
                          onsubmit="return confirm('Silence all notification kinds? \
                                                    You can re-enable them any time.')">
                        <button type="submit" class="btn btn-sm btn-ghost text-base-content/60"
                                aria-label="Silence all notification kinds">
                            "Silence all"
                        </button>
                    </form>
                </div>

                {banner}

                <p class="text-sm text-base-content/70 mb-3">
                    "Defaults: in-app delivery on for all kinds. " {email_status}
                </p>

                <details class="card bg-base-100 border border-base-300 shadow-sm">
                    <summary class="card-body cursor-pointer py-3 flex flex-row items-center justify-between gap-2">
                        <span class="font-medium">"Per-kind delivery"</span>
                        <span class="text-xs text-base-content/50">
                            "Click to expand"
                        </span>
                    </summary>
                    <div class="px-4 pb-4">
                        <form method="post" action="/settings/notifications" class="space-y-4">
                            <div class="overflow-x-auto">
                                <table class="table table-sm" aria-label="Notification kinds">
                                    <thead>
                                        <tr>
                                            <th>"Kind"</th>
                                            <th class="text-center">"In-app"</th>
                                            <th class="text-center">"Email"</th>
                                            <th class="text-center">"Webhook"</th>
                                            <th>"Min severity"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {kind_rows}
                                    </tbody>
                                </table>
                            </div>
                            <p class="text-xs text-base-content/50 italic">
                                "Email and webhook are stubs in this release — they log "
                                "the dispatch attempt but don't yet send. The channel "
                                "structure is ready for the upcoming wasm-smtp integration."
                            </p>
                            <div class="text-right">
                                <button type="submit" class="btn btn-sm btn-primary">
                                    "Save preferences"
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
    let label = kind::human_name(kind_id);

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

    view! {
        <tr aria-label=format!("{} preferences", label)>
            <td class="font-medium">{label}</td>
            <td class="text-center">
                <input type="checkbox" name=in_app_name class="checkbox checkbox-xs"
                       checked=in_app_checked
                       aria-label=format!("In-app for {}", label)/>
            </td>
            <td class="text-center">
                <input type="checkbox" name=email_name class="checkbox checkbox-xs"
                       checked=email_checked
                       aria-label=format!("Email for {}", label)/>
            </td>
            <td class="text-center">
                <input type="checkbox" name=webhook_name class="checkbox checkbox-xs"
                       checked=webhook_checked
                       aria-label=format!("Webhook for {}", label)/>
            </td>
            <td>
                <select name=sev_name class="select select-bordered select-xs"
                        aria-label=format!("Minimum severity for {}", label)>
                    <option value="info" selected=info_selected>"All"</option>
                    <option value="watch" selected={!info_selected}>"Watch only"</option>
                </select>
            </td>
        </tr>
    }
}

pub fn render_preferences(
    user: CurrentUser,
    prefs: Vec<Preference>,
    global_acknowledged: bool,
    email_globally_on: bool,
    unread_count: i64,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <PreferencesPage
                user=user
                prefs=prefs
                global_acknowledged=global_acknowledged
                email_globally_on=email_globally_on
                unread_count=unread_count
            />
        }
    })
}
