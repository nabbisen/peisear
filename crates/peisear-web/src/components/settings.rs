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
use super::t;
use peisear_core::{CurrentUser, personal_metrics::DEFAULT_WIP_LIMIT};
use peisear_i18n::MessageKey;
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

    let effective_label_aria = t(MessageKey::EffectiveCapacityTodayAriaLabel {
        points: effective_today,
    });
    let effective_label = match effective_today {
        Some(n) => t(MessageKey::PointsValue { points: n }),
        None => t(MessageKey::NoCapacitySetTodayLabel),
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
                <span class="font-medium">{t(MessageKey::ConflictLabel)}</span>
                <span>{msg.clone()}</span>
                <p class="text-xs mt-1 opacity-80">
                    {t(MessageKey::CapacityOverlapGuidanceLead)}
                    <em>{t(MessageKey::CloseOnDateActionWord)}</em>
                    {t(MessageKey::CapacityOverlapGuidanceTail)}
                </p>
            </div>
        }
    });

    let settings_heading = t(MessageKey::SettingsSectionName);
    let settings_breadcrumb = settings_heading.clone();

    view! {
        <AppShell title=t(MessageKey::SettingsSectionName) user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li>{settings_breadcrumb}</li>
                </ul></div>
                <h1 class="text-xl font-semibold mb-1">{settings_heading}</h1>
                <p class="text-sm text-base-content/60 mb-6">
                    {t(MessageKey::SettingsSubtitle { display_name })}
                </p>

                {error_block}

                <section class="card bg-base-100 border border-base-300 shadow-sm mb-4"
                         aria-label=t(MessageKey::CapacitySectionAriaLabel)>
                    <div class="card-body gap-3">
                        <h2 class="text-base font-medium">{t(MessageKey::WorkloadCapacityHeading)}</h2>
                        <p class="text-sm text-base-content/70">
                            {t(MessageKey::CapacityExplanationParagraph)}
                        </p>

                        <div class="text-sm py-2 px-3 rounded bg-base-200/60"
                             role="status"
                             aria-label=effective_label_aria>
                            <span class="text-base-content/60">{t(MessageKey::EffectiveTodayLabel)}</span>
                            <span class="font-medium">{effective_label}</span>
                        </div>

                        <div class="overflow-x-auto">
                            <table class="table table-sm" aria-label=t(MessageKey::CapacityRowsTableAriaLabel)>
                                <thead>
                                    <tr>
                                        <th>{t(MessageKey::PointsColumnHeading)}</th>
                                        <th>{t(MessageKey::FromColumnHeading)}</th>
                                        <th>{t(MessageKey::ToColumnHeading)}</th>
                                        <th>{t(MessageKey::NoteColumnHeading)}</th>
                                        <th>{t(MessageKey::ActionsColumnHeading)}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {capacity_rows_view}
                                </tbody>
                            </table>
                        </div>

                        <details class="mt-2">
                            <summary class="cursor-pointer text-sm">
                                {t(MessageKey::AddCapacityRowSummary)}
                            </summary>
                            <form method="post" action="/settings/capacity"
                                  class="mt-3 flex flex-wrap items-end gap-3"
                                  aria-label=t(MessageKey::AddCapacityRowFormAriaLabel)>
                                <label class="form-control">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">{t(MessageKey::PointsColumnHeading)}</span>
                                    </div>
                                    <input type="number" name="points" min="1" max="999"
                                           required
                                           placeholder=t(MessageKey::PointsPlaceholderExample)
                                           class="input input-bordered input-sm w-24"/>
                                </label>
                                <label class="form-control">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">{t(MessageKey::FromDateFieldLabel)}</span>
                                    </div>
                                    <input type="date" name="period_start"
                                           class="input input-bordered input-sm"/>
                                </label>
                                <label class="form-control">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">{t(MessageKey::ToDateFieldLabel)}</span>
                                    </div>
                                    <input type="date" name="period_end"
                                           class="input input-bordered input-sm"/>
                                </label>
                                <label class="form-control flex-1 min-w-[12rem]">
                                    <div class="label py-1">
                                        <span class="label-text text-sm">{t(MessageKey::NoteColumnHeading)}</span>
                                    </div>
                                    <input type="text" name="note" maxlength="120"
                                           placeholder=t(MessageKey::NoteFieldPlaceholder)
                                           class="input input-bordered input-sm w-full"/>
                                </label>
                                <button type="submit" class="btn btn-primary btn-sm">
                                    {t(MessageKey::AddRowButton)}
                                </button>
                            </form>
                            <p class="mt-2 text-xs text-base-content/60">
                                {t(MessageKey::CapacityOverlapHelperText)}
                            </p>
                        </details>
                    </div>
                </section>

                <section class="card bg-base-100 border border-base-300 shadow-sm"
                         aria-label=t(MessageKey::WipLimitLabel)>
                    <form method="post" action="/settings/wip-limit"
                          class="card-body gap-3">
                        <h2 class="text-base font-medium">{t(MessageKey::WipLimitLabel)}</h2>
                        <p class="text-sm text-base-content/70">
                            {t(MessageKey::WipLimitExplanation { default_wip_limit: DEFAULT_WIP_LIMIT })}
                        </p>
                        <label class="form-control w-full max-w-xs">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::WipLimitLabel)}</span>
                                <span class="label-text-alt text-xs opacity-60">{t(MessageKey::InProgressIssuesHint)}</span>
                            </div>
                            <input type="number" name="wip_limit" min="1" max="99"
                                   value=wip_value
                                   placeholder=DEFAULT_WIP_LIMIT.to_string()
                                   class="input input-bordered input-sm w-full"/>
                        </label>
                        <div class="card-actions justify-end mt-2">
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::SaveButton)}</button>
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
        .unwrap_or_else(|| t(MessageKey::NoValuePlaceholder));
    let to_str = row
        .period_end
        .map(|d| d.to_string())
        .unwrap_or_else(|| t(MessageKey::NoValuePlaceholder));
    let note_text = row.note.clone().unwrap_or_default();
    let aria = t(MessageKey::CapacityRowAriaLabel {
        points: row.points,
        from: from_str.clone(),
        to: to_str.clone(),
    });

    let from_value = row.period_start.map(|d| d.to_string()).unwrap_or_default();
    let to_value = row.period_end.map(|d| d.to_string()).unwrap_or_default();

    let update_action = format!("/settings/capacity/{}", row_id);
    let delete_action = format!("/settings/capacity/{}/delete", row_id);
    let close_action = format!("/settings/capacity/{}/close", row_id);

    // Optimistic-lock value for this row's three mutation
    // forms (update / close / delete). Cloned per form because
    // each form moves its own copy into the hidden input. Per
    // peisear-feature-spec-v2.1 §21.4 the handler verifies
    // these against the row's current `updated_at`.
    let client_updated_at = row.updated_at.to_rfc3339();
    let cua_update = client_updated_at.clone();
    let cua_close = client_updated_at.clone();
    let cua_delete = client_updated_at;

    let is_open_ended = row.period_end.is_none();
    let close_button = is_open_ended.then(|| {
        view! {
            <details class="dropdown dropdown-end">
                <summary class="btn btn-ghost btn-xs"
                         aria-label=t(MessageKey::CloseThisRowAriaLabel)>
                    {t(MessageKey::CloseOnDateSummary)}
                </summary>
                <div class="dropdown-content card card-compact w-64 p-2 shadow bg-base-100 border border-base-300">
                    <form method="post" action=close_action class="flex gap-2 items-end">
                        <input type="hidden" name="client_updated_at" value=cua_close/>
                        <label class="form-control flex-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">{t(MessageKey::CloseOnLabel)}</span>
                            </div>
                            <input type="date" name="period_end" required
                                   class="input input-bordered input-xs"/>
                        </label>
                        <button type="submit" class="btn btn-primary btn-xs">{t(MessageKey::CloseButton)}</button>
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
                        {t(MessageKey::PointsUnitSuffix)}
                    </summary>
                    <form method="post" action=update_action
                          class="dropdown-content card card-compact w-80 p-3 shadow bg-base-100 border border-base-300 z-10"
                          aria-label=t(MessageKey::EditRowAriaLabel)>
                        <input type="hidden" name="client_updated_at" value=cua_update/>
                        <label class="form-control">
                            <div class="label py-0">
                                <span class="label-text text-xs">{t(MessageKey::PointsColumnHeading)}</span>
                            </div>
                            <input type="number" name="points" min="1" max="999"
                                   value=row.points.to_string()
                                   class="input input-bordered input-xs"/>
                        </label>
                        <label class="form-control mt-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">{t(MessageKey::FromColumnHeading)}</span>
                            </div>
                            <input type="date" name="period_start"
                                   value=from_value
                                   class="input input-bordered input-xs"/>
                        </label>
                        <label class="form-control mt-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">{t(MessageKey::ToColumnHeading)}</span>
                            </div>
                            <input type="date" name="period_end"
                                   value=to_value
                                   class="input input-bordered input-xs"/>
                        </label>
                        <label class="form-control mt-1">
                            <div class="label py-0">
                                <span class="label-text text-xs">{t(MessageKey::NoteColumnHeading)}</span>
                            </div>
                            <input type="text" name="note" maxlength="120"
                                   value=note_text
                                   class="input input-bordered input-xs"/>
                        </label>
                        <button type="submit" class="btn btn-primary btn-xs mt-2">
                            {t(MessageKey::SaveButton)}
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
                        <input type="hidden" name="client_updated_at" value=cua_delete/>
                        <button type="submit" class="btn btn-ghost btn-xs text-error"
                                aria-label=t(MessageKey::RemoveThisRowAriaLabel)>
                            {t(MessageKey::RemoveButton)}
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
