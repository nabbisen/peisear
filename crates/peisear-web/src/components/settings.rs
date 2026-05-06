//! Settings page: personal capacity (story points) and personal
//! WIP limit (count of in-progress issues).

use axum::response::Html;
use leptos::prelude::*;

use super::layout::AppShell;
use peisear_core::{CurrentUser, personal_metrics::DEFAULT_WIP_LIMIT};

#[component]
pub fn SettingsPage(
    user: CurrentUser,
    capacity_points: Option<i64>,
    wip_limit: Option<i64>,
    flash: Option<String>,
) -> impl IntoView {
    let cap_value = capacity_points.map(|n| n.to_string()).unwrap_or_default();
    let wip_value = wip_limit.map(|n| n.to_string()).unwrap_or_default();
    let display_name = user.display_name.clone();
    let default_wip_hint = format!("Leave blank to use the project default (or {DEFAULT_WIP_LIMIT}).");

    view! {
        <AppShell title="Settings".to_string() user=user flash=flash>
            <div class="max-w-xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li>"Settings"</li>
                </ul></div>
                <h1 class="text-xl font-semibold mb-1">"Settings"</h1>
                <p class="text-sm text-base-content/60 mb-6">
                    "Personal preferences for " {display_name} "."
                </p>

                <div class="card bg-base-100 border border-base-300 shadow-sm mb-4">
                    <form method="post" action="/settings/capacity" class="card-body gap-3">
                        <h2 class="text-base font-medium">"Workload capacity"</h2>
                        <p class="text-sm text-base-content/70">
                            "How many story points you can comfortably carry at once. \
                             Project pages will show a warning when your in-flight load \
                             exceeds this value. Leave blank to opt out of warnings."
                        </p>
                        <label class="form-control w-full max-w-xs">
                            <div class="label py-1">
                                <span class="label-text text-sm">"Capacity"</span>
                                <span class="label-text-alt text-xs opacity-60">"story points"</span>
                            </div>
                            <input type="number" name="capacity_points" min="1" max="999"
                                   value=cap_value
                                   placeholder="e.g. 10"
                                   class="input input-bordered input-sm w-full"
                                   aria-describedby="capacity-help"/>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                        </div>
                    </form>
                </div>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action="/settings/wip-limit" class="card-body gap-3">
                        <h2 class="text-base font-medium">"WIP limit"</h2>
                        <p class="text-sm text-base-content/70">
                            "How many issues you want to have In Progress at once. \
                             This is about cognitive load — a small number of \
                             actively-worked issues, distinct from the points-budget \
                             above. " {default_wip_hint}
                        </p>
                        <label class="form-control w-full max-w-xs">
                            <div class="label py-1">
                                <span class="label-text text-sm">"WIP limit"</span>
                                <span class="label-text-alt text-xs opacity-60">"in-progress issues"</span>
                            </div>
                            <input type="number" name="wip_limit" min="1" max="99"
                                   value=wip_value
                                   placeholder=DEFAULT_WIP_LIMIT.to_string()
                                   class="input input-bordered input-sm w-full"
                                   aria-describedby="wip-help"/>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

pub fn render_settings(
    user: CurrentUser,
    capacity_points: Option<i64>,
    wip_limit: Option<i64>,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SettingsPage
                user=user
                capacity_points=capacity_points
                wip_limit=wip_limit
                flash=flash
            />
        }
    })
}
