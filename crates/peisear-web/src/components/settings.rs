//! Settings page: period-aware capacity rows + personal WIP limit.
//!
//! ## 0.12.0 design
//!
//! Capacity is no longer one number; it's a table of rows, each
//! valid over an optional period. The page renders:
//!
//! 1. **Today's effective capacity** at the top, so the user can
//!    see the answer to "what's my number right now?" without
//!    consulting the table.
//! 2. **Existing capacity rows** in a table, with edit and remove
//!    actions inline. Open-ended rows (no period_end) are
//!    visually distinguished and have a "Close on date" helper.
//! 3. **Add new row** form below the table, with explicit
//!    period_start / period_end / note fields. The form rejects
//!    overlap by surfacing the conflict via the `error` query
//!    param after redirect.
//! 4. **WIP limit** form, unchanged from before.
//!
//! Accessibility: each row has an aria-label summarising it, the
//! conflict message renders inside `<div role="alert">`, and form
//! inputs are labelled.

use axum::response::Html;
use leptos::prelude::*;

use super::layout::AppShell;
use peisear_core::{CurrentUser, personal_metrics::DEFAULT_WIP_LIMIT};
use peisear_storage::user_capacities::CapacityRow;

#[component]
pub fn SettingsPage(
    user: CurrentUser,
    wip_limit: Option<i64>,
    capacity_rows: Vec<CapacityRow>,
    effective_today: Option<i64>,
    flash: Option<String>,
    error: Option<String>,
) -> impl IntoView {
    let wip_value = wip_limit.map(|n| n.to_string()).unwrap_or_default();
    let display_name = user.display_name.clone();
    let default_wip_hint =
        format!("Leave blank to use the project default (or {DEFAULT_WIP_LIMIT}).");

    let effective_label = match effective_today {
        Some(n) => format!("{n} pt"),
        None => "no capacity set for today".to_string(),
    };

    let capacity_rows_view = capacity_rows
        .into_iter()
        .map(render_capacity_row)
        .collect_view();

    let error_block = error.as_ref().map(|msg| {
        view! {
            <div role="alert"
                 class="alert alert-warning text-sm mb-4"
                 aria-live="polite">
                <span class="font-medium">"Conflict: "</span>
                <span>{msg.clone()}</span>
                <p class="text-xs mt-1 opacity-80">
                    "Close the conflicting row first (use the "
                    <em>"Close on date"</em>
                    " action), or adjust the new period so it doesn't overlap."
                </p>
            </div>
        }
    });

    view! {
        <AppShell title="Settings".to_string() user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li>"Settings"</li>
                </ul></div>
                <h1 class="text-xl font-semibold mb-1">"Settings"</h1>
                <p class="text-sm text-base-content/60 mb-6">
                    "Personal preferences for " {display_name} "."
                </p>

                {error_block}

                <section class="card bg-base-100 border border-base-300 shadow-sm mb-4"
                         aria-label="Capacity">
                    <div class="card-body gap-3">
                        <h2 class="text-base font-medium">"Workload capacity"</h2>
                        <p class="text-sm text-base-content/70">
                            "Capacity rows describe how many story points you \
                             can comfortably carry, optionally bounded by a \
                             period. The row whose period covers today is \
                             your effective capacity. Periods may not overlap; \
                             leave both bounds blank for an open-ended default."
                        </p>

                        <div class="text-sm py-2 px-3 rounded bg-base-200/60"
                             role="status"
                             aria-label=format!("Effective capacity today: {}", effective_label.clone())>
                            <span class="text-base-content/60">"Effective today: "</span>
                            <span class="font-medium">{effective_label.clone()}</span>
                        </div>

                        <div class="overflow-x-auto">
                            <table class="table table-sm" aria-label="Capacity rows">
                                <thead>
                                    <tr>
                                        <th>"Points"</th>
                                        <th>"From"</th>
                                        <th>"To"</th>
                                        <th>"Note"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {capacity_rows_view}
                                </tbody>
                            </table>
                        </div>

                        <details class="mt-2">
                            <summary class="cursor-pointer text-sm">
                                "Add a capacity row"
                            </summary>
                            <form method="post" action="/settings/capacity"
                                  class="mt-3 flex flex-wrap items-end gap-3"
                                  aria-label="Add capacity row">
                                <label class="form-control">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">"Points"</span>
                                    </div>
                                    <input type="number" name="points" min="1" max="999"
                                           required
                                           placeholder="e.g. 10"
                                           class="input input-bordered input-sm w-24"/>
                                </label>
                                <label class="form-control">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">"From (YYYY-MM-DD)"</span>
                                    </div>
                                    <input type="date" name="period_start"
                                           class="input input-bordered input-sm"/>
                                </label>
                                <label class="form-control">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">"To (YYYY-MM-DD)"</span>
                                    </div>
                                    <input type="date" name="period_end"
                                           class="input input-bordered input-sm"/>
                                </label>
                                <label class="form-control flex-1 min-w-[12rem]">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">"Note"</span>
                                    </div>
                                    <input type="text" name="note" maxlength="120"
                                           placeholder="optional context"
                                           class="input input-bordered input-sm w-full"/>
                                </label>
                                <button type="submit" class="btn btn-primary btn-sm">
                                    "Add row"
                                </button>
                            </form>
                            <p class="mt-2 text-xs text-base-content/60">
                                "Both date fields are optional. Leave blank to mean \
                                 \"from the dawn of time\" (start) or \"until further \
                                 notice\" (end). Adding a row that overlaps an \
                                 existing one will fail; close the existing row first."
                            </p>
                        </details>
                    </div>
                </section>

                <section class="card bg-base-100 border border-base-300 shadow-sm"
                         aria-label="WIP limit">
                    <form method="post" action="/settings/wip-limit"
                          class="card-body gap-3">
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
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <button type="submit" class="btn btn-primary btn-sm">"Save"</button>
                        </div>
                    </form>
                </section>
            </div>
        </AppShell>
    }
}

/// Render one capacity row as a table row with inline edit form,
/// remove button, and (for open-ended rows) a "Close on date"
/// action.
fn render_capacity_row(row: CapacityRow) -> impl IntoView {
    let row_id = row.id.clone();
    let from_str = row
        .period_start
        .map(|d| d.to_string())
        .unwrap_or_else(|| "—".into());
    let to_str = row
        .period_end
        .map(|d| d.to_string())
        .unwrap_or_else(|| "—".into());
    let note_text = row.note.clone().unwrap_or_default();
    let aria = format!(
        "Capacity {} points, period {} to {}.",
        row.points, from_str, to_str
    );

    let from_value = row
        .period_start
        .map(|d| d.to_string())
        .unwrap_or_default();
    let to_value = row.period_end.map(|d| d.to_string()).unwrap_or_default();

    let update_action = format!("/settings/capacity/{}", row_id);
    let delete_action = format!("/settings/capacity/{}/delete", row_id);
    let close_action = format!("/settings/capacity/{}/close", row_id);

    let is_open_ended = row.period_end.is_none();
    let close_button = is_open_ended.then(|| {
        view! {
            <details class="dropdown dropdown-end">
                <summary class="btn btn-ghost btn-xs"
                         aria-label="Close this row on a specific date">
                    "Close on date…"
                </summary>
                <div class="dropdown-content card card-compact w-64 p-2 shadow bg-base-100 border border-base-300">
                    <form method="post" action=close_action class="flex gap-2 items-end">
                        <label class="form-control flex-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">"Close on"</span>
                            </div>
                            <input type="date" name="period_end" required
                                   class="input input-bordered input-xs"/>
                        </label>
                        <button type="submit" class="btn btn-primary btn-xs">"Close"</button>
                    </form>
                </div>
            </details>
        }
    });

    view! {
        <tr aria-label=aria>
            <td>
                <details class="dropdown">
                    <summary class="cursor-pointer">
                        <span class="font-medium">{row.points}</span>
                        " pt"
                    </summary>
                    <form method="post" action=update_action
                          class="dropdown-content card card-compact w-80 p-3 shadow bg-base-100 border border-base-300 z-10"
                          aria-label="Edit row">
                        <label class="form-control">
                            <div class="label py-0">
                                <span class="label-text text-xs">"Points"</span>
                            </div>
                            <input type="number" name="points" min="1" max="999"
                                   value=row.points.to_string()
                                   class="input input-bordered input-xs"/>
                        </label>
                        <label class="form-control mt-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">"From"</span>
                            </div>
                            <input type="date" name="period_start"
                                   value=from_value
                                   class="input input-bordered input-xs"/>
                        </label>
                        <label class="form-control mt-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">"To"</span>
                            </div>
                            <input type="date" name="period_end"
                                   value=to_value
                                   class="input input-bordered input-xs"/>
                        </label>
                        <label class="form-control mt-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">"Note"</span>
                            </div>
                            <input type="text" name="note" maxlength="120"
                                   value=note_text
                                   class="input input-bordered input-xs"/>
                        </label>
                        <button type="submit" class="btn btn-primary btn-xs mt-2">
                            "Save"
                        </button>
                    </form>
                </details>
            </td>
            <td class="text-sm text-base-content/70">{from_str}</td>
            <td class="text-sm text-base-content/70">{to_str}</td>
            <td class="text-sm text-base-content/70 max-w-[12rem] truncate">
                {row.note.unwrap_or_default()}
            </td>
            <td>
                <div class="flex items-center gap-1">
                    {close_button}
                    <form method="post" action=delete_action
                          onsubmit="return confirm('Remove this capacity row?')">
                        <button type="submit" class="btn btn-ghost btn-xs text-error"
                                aria-label="Remove this row">
                            "Remove"
                        </button>
                    </form>
                </div>
            </td>
        </tr>
    }
}

pub fn render_settings(
    user: CurrentUser,
    wip_limit: Option<i64>,
    capacity_rows: Vec<CapacityRow>,
    effective_today: Option<i64>,
    flash: Option<String>,
    error: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SettingsPage
                user=user
                wip_limit=wip_limit
                capacity_rows=capacity_rows
                effective_today=effective_today
                flash=flash
                error=error
            />
        }
    })
}
