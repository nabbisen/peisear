//! Project detail (board + list view) and issue CRUD pages.

use axum::response::Html;
use leptos::prelude::*;

use super::{Column, layout::AppShell};
use peisear_core::{
    AssigneeOption, CurrentUser, DisplayHealthState, Issue, IssueStatus, Priority, Project,
    UserLoad,
    project_health::{HealthScore, Indicator, ProjectHealthReport},
};
use peisear_i18n::{Field, Locale, MessageKey, NavSection, TrendDirectionLabel};

use super::t;

/// Project-detail page: header + board/list view toggle.
#[component]
pub fn ProjectDetailPage(
    user: CurrentUser,
    project: Project,
    columns: Vec<Column>,
    view_mode: String,
    all_issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    health: ProjectHealthReport,
    flash: Option<String>,
    /// Currently-applied status filter, e.g. "open" /
    /// "in_progress" / "done". Empty string = no filter.
    active_status: String,
    /// Currently-applied assignee filter: a user id, the literal
    /// string "unassigned", or empty for no filter.
    active_assignee: String,
    /// Currently-applied sort key: "priority" / "created" /
    /// "updated". Empty string = storage-default order.
    active_sort: String,
) -> impl IntoView {
    let title = t(MessageKey::ProjectDetailPageTitle {
        project_name: project.name.clone(),
    });
    let is_board = view_mode == "board";

    let board_link = format!("/projects/{}?view=board", project.id);
    let list_link = format!("/projects/{}?view=list", project.id);
    let edit_link = format!("/projects/{}/edit", project.id);
    let new_issue_link = format!("/projects/{}/issues/new", project.id);
    let calendar_link = format!("/projects/{}/calendar", project.id);
    let project_id = project.id.clone();
    let project_id_for_board = project.id.clone();
    let project_id_for_list = project.id.clone();

    let board_classes = if is_board {
        "join-item btn btn-sm btn-active btn-primary"
    } else {
        "join-item btn btn-sm btn-ghost"
    };
    let list_classes = if is_board {
        "join-item btn btn-sm btn-ghost"
    } else {
        "join-item btn btn-sm btn-active btn-primary"
    };

    let desc_node = if project.description.is_empty() {
        ().into_any()
    } else {
        view! {
            <p class="text-sm text-base-content/60 max-w-3xl">{project.description.clone()}</p>
        }
        .into_any()
    };

    let name_for_breadcrumb = project.name.clone();
    let name_for_header = project.name.clone();

    let breadcrumb = super::breadcrumb::render_breadcrumb(vec![
        super::breadcrumb::BreadcrumbItem::link(t(MessageKey::ProjectsSectionName), "/projects"),
        super::breadcrumb::BreadcrumbItem::current(name_for_breadcrumb),
    ]);
    let back_link = super::breadcrumb::render_back_link(NavSection::Projects, "/projects");

    view! {
        <AppShell title=title user=user flash=flash>
            {breadcrumb}
            {back_link}
            <div class="flex flex-wrap items-start justify-between gap-3 mb-4">
                <div class="min-w-0">
                    <h1 class="text-2xl font-semibold tracking-tight truncate">{name_for_header}</h1>
                    {desc_node}
                </div>

                <div class="flex items-center gap-2 shrink-0">
                    <div class="join">
                        <a href=board_link class=board_classes>{t(MessageKey::ViewToggleBoard)}</a>
                        <a href=list_link class=list_classes>{t(MessageKey::ViewToggleList)}</a>
                    </div>
                    <a href=calendar_link class="btn btn-ghost btn-sm">
                        {t(MessageKey::CalendarBreadcrumbWord)}
                    </a>
                    <a href=edit_link class="btn btn-ghost btn-sm">{t(MessageKey::EditWord)}</a>
                    <a href=new_issue_link class="btn btn-primary btn-sm">{t(MessageKey::NewIssueLabel)}</a>
                </div>
            </div>

            <HealthStrip health=health/>

            <WorkloadStrip workload=workload/>

            // Populated by board.js when a status-change request is
            // rejected or conflicts; empty (and hidden) otherwise.
            {is_board.then(|| view! {
                <div id="board-status" role="status" class="text-sm text-base-content/70 mb-2 empty:hidden"></div>
            })}

            {if is_board {
                view! { <BoardView project_id=project_id_for_board columns=columns assignees=assignees.clone()/> }.into_any()
            } else {
                view! {
                    <ListView
                        project_id=project_id_for_list
                        issues=all_issues
                        assignees=assignees.clone()
                        active_status=active_status
                        active_assignee=active_assignee
                        active_sort=active_sort
                    />
                }.into_any()
            }}

            // DnD script is loaded only in board mode. `data-project-id`
            // on the board div is how the JS picks up which project it
            // belongs to, avoiding string-interpolated inline JS.
            {is_board.then(|| view! {
                <div id="board-root" data-project-id=project_id.clone() class="hidden"></div>
                <script src="/static/board.js" defer=true></script>
            })}
        </AppShell>
    }
}

/// Project-level health indicators. Three stats — Throughput,
/// Staleness, Activity — each rendered as a labelled chip with a
/// Project-level health summary. Per the V2.1 brief §1.1 and
/// the user's UI/UX guidance:
///
/// - Composite 0–100 score is the headline.
/// - One natural-language sentence summarises which indicator(s)
///   are pulling the score down.
/// - Per-indicator breakdown is collapsed by default into a
///   `<details>` so the standard board view stays uncluttered
///   ("Minimal by Default", §0.3).
/// - Indicator chips reuse the `Indicator` shape; adding a new
///   metric only requires a new entry in
///   `peisear_core::project_health::ALL_INDICATORS` — UI here
///   does not change.
///
/// Empty projects (no issues yet) skip the section entirely
/// rather than show "Insufficient" chips that explain nothing.
#[component]
fn HealthStrip(health: ProjectHealthReport) -> impl IntoView {
    if health.raw.total_issues == 0 {
        return view! {
            <section class="mb-3">
                <div class="text-xs text-base-content/60 italic">
                    {t(MessageKey::HealthEmptyMessage)}
                </div>
            </section>
        }
        .into_any();
    }

    // §17.1 / FR-HLT-008: no headline score, no 0-100 figure
    // anywhere in health presentation. The composite keeps its
    // state badge, trend chip, and summary sentence — it just
    // renders as one more chip alongside the individual
    // indicators (composite_row below) instead of a separate,
    // more prominent box carrying a number.
    let summary = Locale::English.render(health.score.summary.clone());

    // Phase B PR3 (B-2): explainability — collect human-language
    // sentences describing each indicator that's not at Good.
    // The list is computed before consuming `health.indicators`
    // for the chip row below. `human_explanation` takes `raw`
    // (I18N-002) so its typed parameters come from the same raw
    // numbers `format_value` uses, not a pre-formatted string.
    let explanations: Vec<String> = health
        .indicators
        .iter()
        .filter_map(|i| i.human_explanation(&health.raw))
        .map(|key| Locale::English.render(key))
        .collect();

    let composite_chip = composite_row(&health.score);
    let indicator_rows = health
        .indicators
        .into_iter()
        .map(indicator_row)
        .collect_view();

    view! {
        <section class="mb-4" aria-label=t(MessageKey::ProjectHealthSectionLabel)>
            <div class="flex items-center gap-2 mb-1">
                <h3 class="text-xs uppercase tracking-wide text-base-content/60">
                    {t(MessageKey::HealthHeading)}
                </h3>
            </div>

            <p class="text-sm text-base-content/70 mb-2">{summary}</p>

            <details class="text-xs">
                <summary class="cursor-pointer text-base-content/60 hover:text-base-content">
                    {t(MessageKey::IndicatorsSummaryLabel)}
                </summary>
                // Phase B PR3 (B-2): human-language
                // explanation list. Each non-Good indicator
                // contributes a sentence describing what's
                // happening, in the user's own terms ("3 issues
                // haven't moved in over two weeks") rather than
                // the score's terms ("long_stale_ratio = 0.3").
                // Per decision B-E5, readability beats
                // calculation transparency.
                //
                // The list comes before the chip row so the
                // user reads the story first, then can
                // double-check against the numbers if they
                // want.
                {(!explanations.is_empty()).then(|| view! {
                    <ul class="mt-2 ml-4 list-disc text-base-content/80 leading-relaxed space-y-1">
                        {explanations.into_iter().map(|line| view! {
                            <li>{line}</li>
                        }).collect_view()}
                    </ul>
                })}
                // The composite sits first but at the same visual
                // weight as the six individual indicators —
                // FR-HLT-008 requires "alongside", not "above".
                <div class="mt-2 flex flex-wrap items-center gap-3">
                    {composite_chip}
                    {indicator_rows}
                </div>
            </details>
        </section>
    }
    .into_any()
}

/// Render the trend indicator next to the score. `Unavailable`
/// returns nothing — no past data, nothing to compare. Otherwise
/// renders an arrow + signed delta with `aria-label` for screen
/// readers.
///
/// We do not colour-code the trend itself: improvement and
/// decline are inherently neutral facts, and the score state
/// (Good / Watch / Concern) already conveys the absolute
/// health. Adding a colour to the trend would push toward
/// performance-evaluation territory, which V2.1 §0.2 declines
/// to encourage.
fn render_trend_chip(trend: peisear_core::project_health::Trend) -> impl IntoView {
    use peisear_core::project_health::Trend;
    let (glyph, label, aria) = match trend {
        Trend::Unavailable => return view! { <span class="hidden"></span> }.into_any(),
        Trend::Flat => (
            "→",
            t(MessageKey::TrendLabelFlat),
            t(MessageKey::TrendAriaFlat),
        ),
        Trend::Up { delta } => (
            "↑",
            t(MessageKey::TrendLabel {
                direction: TrendDirectionLabel::Up,
                delta,
            }),
            t(MessageKey::TrendAriaLabel {
                direction: TrendDirectionLabel::Up,
                delta,
            }),
        ),
        Trend::Down { delta } => (
            "↓",
            t(MessageKey::TrendLabel {
                direction: TrendDirectionLabel::Down,
                delta,
            }),
            t(MessageKey::TrendAriaLabel {
                direction: TrendDirectionLabel::Down,
                delta,
            }),
        ),
    };
    view! {
        <span class="text-xs text-base-content/60 ml-1"
              role="group"
              aria-label=aria.clone()
              title=aria>
            <span aria-hidden="true">{glyph}</span>
            " " {label}
        </span>
    }
    .into_any()
}

/// Render the composite score as one more chip, at the same visual
/// weight as [`indicator_row`]'s output — `FR-HLT-008` requires it
/// sit alongside the individual indicators, not above them as a
/// headline, and carry no 0–100 figure. It keeps its state badge
/// (clamped to the `Watch` ceiling) and its trend chip.
fn composite_row(score: &HealthScore) -> impl IntoView {
    let state = DisplayHealthState::from(score.state);
    let badge_class = format!("badge badge-sm {}", state.badge_class());
    let glyph = state.glyph();
    let aria_label = t(MessageKey::CompositeAriaLabel {
        state: state.to_i18n_label(),
    });
    let trend_chip = render_trend_chip(score.trend);
    view! {
        <div class="flex items-center gap-2 px-2 py-1 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria_label.clone()
             title=aria_label>
            <span class="text-xs text-base-content/70">{t(MessageKey::CompositeLabel)}</span>
            <span class=badge_class>
                <span aria-hidden="true">{glyph}</span>
            </span>
            {trend_chip}
        </div>
    }
}

/// Render one indicator chip (label + value + state badge with
/// glyph). The glyph + aria-label combination satisfies the
/// "color-only" anti-pattern check.
fn indicator_row(ind: Indicator) -> impl IntoView {
    let state = DisplayHealthState::from(ind.state);
    let badge_class = format!("badge badge-sm {}", state.badge_class());
    let glyph = state.glyph();
    // I18N-004: the indicator's name no longer lives on `Indicator`
    // itself (`IndicatorKind::label()` removed) — rendered here via
    // the same MessageKey `summarize`'s sentences use.
    let label_text = Locale::English.render(MessageKey::IndicatorName {
        label: ind.kind.to_i18n_label(),
    });
    let value_text = Locale::English.render(ind.value_display.clone());
    let aria_label = t(MessageKey::IndicatorAriaLabel {
        label: ind.kind.to_i18n_label(),
        value: Box::new(ind.value_display),
        state: state.to_i18n_label(),
    });
    view! {
        <div class="flex items-center gap-2 px-2 py-1 rounded border border-base-300 bg-base-100"
             role="group"
             aria-label=aria_label.clone()
             title=aria_label>
            <span class="text-xs text-base-content/70">{label_text}</span>
            <span class=badge_class>
                <span class="mr-1" aria-hidden="true">{glyph}</span>
                {value_text}
            </span>
        </div>
    }
}

/// A horizontal strip of per-user load chips, one per assignee
/// candidate. Renders an empty `<div>` when there is no in-flight
/// work to show.
///
/// Shows only in-flight load, never a capacity value or a state
/// derived from one (`NFR-PRIV-001`, `DEC-019`): another member's
/// capacity, WIP limit, or over/under-capacity standing must not
/// reach this surface in any form — text, colour, or attribute. A
/// chip labelled with a person's name and their in-flight load is
/// not an aggregate that could resolve to an individual — it *is*
/// individual workload, which `NFR-PRIV-002` permits sharing
/// regardless of how many members the strip lists (`ISSUE-003`
/// ruling: an earlier `NFR-PRIV-007` single-member suppression here
/// was a misapplication of that requirement and has been removed).
#[component]
fn WorkloadStrip(workload: Vec<UserLoad>) -> impl IntoView {
    let any_signal = workload.iter().any(|u| u.in_flight_issues > 0);
    if !any_signal {
        return view! { <div class="hidden"></div> }.into_any();
    }

    let chips = workload
        .into_iter()
        .map(|u| {
            let label = t(MessageKey::PointsValue {
                points: u.in_flight_points,
            });
            let title = t(MessageKey::WorkloadTitle {
                display_name: u.display_name.clone(),
                in_flight_issues: u.in_flight_issues,
            });
            view! {
                <div class="flex items-center gap-2 px-2 py-1 rounded border border-base-300 bg-base-100"
                     title=title>
                    <span class="text-xs font-medium">{u.display_name}</span>
                    <span class="badge badge-sm badge-ghost">{label}</span>
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="mb-4">
            <div class="flex items-center gap-2 mb-1">
                <h3 class="text-xs uppercase tracking-wide text-base-content/60">
                    {t(MessageKey::WorkloadHeading)}
                </h3>
                <a href="/settings" class="text-xs link link-hover opacity-60">
                    {t(MessageKey::WorkloadSetCapacityLink)}
                </a>
            </div>
            <div class="flex flex-wrap items-center gap-2">{chips}</div>
        </section>
    }
    .into_any()
}

/// Inline hint shown below the issue form, summarising current
/// in-flight workload per assignee candidate. SSR-only: this is a
/// static snapshot rendered at request time.
///
/// Shows only in-flight load, never a capacity value or a state
/// derived from one (`NFR-PRIV-001`, `DEC-019`) — the projected
/// "would this push someone over capacity" annotation this hint
/// used to show for the selected assignee is removed; it disclosed
/// another member's capacity standing regardless of whether that
/// member happened to be the viewer. Renders an empty `<div>` when
/// there's nothing to show (see [`WorkloadStrip`]'s doc comment on
/// why there is no single-member suppression here either —
/// `ISSUE-003` ruling).
#[component]
fn WorkloadHint(workload: Vec<UserLoad>) -> impl IntoView {
    if workload.is_empty() {
        return view! { <div class="hidden"></div> }.into_any();
    }

    let chips = workload
        .into_iter()
        .map(|u| {
            let snapshot = t(MessageKey::PointsValue {
                points: u.in_flight_points,
            });
            view! {
                <span class="inline-flex items-center gap-1">
                    <span class="text-base-content/70">{u.display_name}</span>
                    <span class="badge badge-xs badge-ghost">{snapshot}</span>
                </span>
            }
        })
        .collect_view();

    view! {
        <div class="text-xs text-base-content/60 -mt-1">
            <span class="font-medium mr-2">{t(MessageKey::WorkloadHintLabel)}</span>
            <span class="inline-flex items-center gap-3 flex-wrap">{chips}</span>
        </div>
    }
    .into_any()
}

#[component]
fn BoardView(
    project_id: String,
    columns: Vec<Column>,
    assignees: Vec<AssigneeOption>,
) -> impl IntoView {
    view! {
        <div class="grid gap-3 md:grid-cols-3" id="board">
            {columns.into_iter().map(|column| {
                let status_dot = match column.status.as_str() {
                    "open" => "w-2 h-2 rounded-full bg-info",
                    "in_progress" => "w-2 h-2 rounded-full bg-warning",
                    _ => "w-2 h-2 rounded-full bg-success",
                };
                let status_slug = column.status.as_str();
                let label = t(MessageKey::IssueStatusName {
                    label: column.status.to_i18n_label(),
                });
                let count = column.issues.len();
                let is_empty = column.issues.is_empty();
                let project_id = project_id.clone();
                let assignees_for_col = assignees.clone();
                view! {
                    <section class="bg-base-100 border border-base-300 rounded-lg flex flex-col min-h-[200px]">
                        <header class="flex items-center justify-between px-3 py-2 border-b border-base-300">
                            <div class="flex items-center gap-2">
                                <span class=status_dot></span>
                                <h2 class="text-sm font-medium">{label}</h2>
                            </div>
                            <span class="badge badge-ghost badge-sm">{count}</span>
                        </header>
                        <div class="p-2 flex-1 flex flex-col gap-2 column-drop"
                             data-status=status_slug>
                            {column.issues.into_iter().map(|issue| {
                                view! { <IssueCard project_id=project_id.clone() issue=issue assignees=assignees_for_col.clone()/> }
                            }).collect_view()}
                            {is_empty.then(|| view! {
                                <div class="text-xs text-base-content/40 text-center py-4 italic">
                                    {t(MessageKey::EmptyBoardHint)}
                                </div>
                            })}
                        </div>
                    </section>
                }
            }).collect_view()}
        </div>
    }
}

/// Look up an assignee's display name by id. Returns the literal id
/// when no candidate matches — that happens if the assignee was once
/// a team member who has since been removed (the FK ON DELETE SET NULL
/// keeps that from happening for the project owner today, but the
/// graceful fallback prepares us for the team feature where former
/// members may still appear historically).
fn assignee_label<'a>(id: &'a str, assignees: &'a [AssigneeOption]) -> &'a str {
    assignees
        .iter()
        .find(|a| a.id == id)
        .map(|a| a.display_name.as_str())
        .unwrap_or(id)
}

#[component]
fn IssueCard(project_id: String, issue: Issue, assignees: Vec<AssigneeOption>) -> impl IntoView {
    let href = format!("/projects/{}/issues/{}", project_id, issue.id);
    let badge = format!("badge badge-sm {}", issue.priority.badge_class());
    let date = issue.updated_at.format("%m-%d").to_string();
    let issue_id = issue.id.clone();
    // The optimistic-lock value the page rendered this card with.
    // `board.js` reads it and sends it back on drop (DEV-001); the
    // keyboard status form below sends the same value in a hidden
    // field (DEV-002). §21.4 rejects a status change whose value no
    // longer matches the stored row, on either path — both go
    // through the single `apply_status_change` lock check.
    let updated_at = issue.updated_at.to_rfc3339();
    let updated_at_for_form = updated_at.clone();
    let effort_node = issue.effort.map(|e| {
        let label = t(MessageKey::PointsValue { points: e });
        view! {
            <span class="badge badge-sm badge-outline" title=t(MessageKey::EffortEstimateTooltip)>
                {label}
            </span>
        }
    });
    let assignee_node = issue.assignee_id.as_ref().map(|aid| {
        let name = assignee_label(aid, &assignees).to_string();
        view! {
            <span class="badge badge-sm badge-ghost" title=t(MessageKey::FieldLabel { field: Field::Assignee })>
                {name}
            </span>
        }
    });

    // Keyboard-operable status control (DEV-002, `FR-DM-002`): one
    // submit button per status other than the card's current one.
    // Plain form POST, no scripting (`DEC-021`). Each button names
    // both the issue and the target status so a screen-reader user
    // stepping through a column doesn't hear "Done, Done, Done"
    // with no idea which issue each belongs to.
    let status_form_action = format!("/projects/{}/issues/{}/status/board", project_id, issue.id);
    let title_for_status = issue.title.clone();
    let current_status = issue.status;
    let status_buttons = IssueStatus::all()
        .into_iter()
        .filter(|s| *s != current_status)
        .map(|target| {
            let label = t(MessageKey::IssueStatusName {
                label: target.to_i18n_label(),
            });
            let aria_label = t(MessageKey::MoveIssueAriaLabel {
                issue_title: title_for_status.clone(),
                target: target.to_i18n_label(),
            });
            view! {
                <button
                    type="submit"
                    name="status"
                    value=target.as_str()
                    class="btn btn-ghost btn-xs min-h-11 min-w-11 px-2 normal-case"
                    aria-label=aria_label>
                    {label}
                </button>
            }
        })
        .collect_view();

    view! {
        // The outer element carries the drag identity
        // (`board.js` matches `.issue-card`, reads
        // `data-issue-id`/`data-updated-at`, and drags this whole
        // node). It must be the draggable unit rather than the
        // inner `<a>`, because a `<form>` cannot nest inside an
        // `<a>` (invalid HTML) — the link and the form are siblings
        // here, both children of this div, so dragging moves them
        // together and the two controls stay paired after a drop.
        <div
            class="issue-card bg-base-100 border border-base-300 hover:border-primary rounded-md p-3 shadow-sm cursor-grab active:cursor-grabbing transition"
            data-issue-id=issue_id
            data-updated-at=updated_at
            draggable="true">
            // `draggable="false"`: an <a href> is draggable by
            // browser default. Without this, dragstart still
            // bubbles to the outer div's listener (so the drop
            // logic works), but the drag itself is a link drag —
            // carrying the href in dataTransfer and showing the
            // link ghost — since two nested drag sources now
            // exist (DEV-002-005-review.md §1.3). This makes the
            // outer div the sole drag source.
            <a href=href class="block" draggable="false">
                <div class="text-sm font-medium line-clamp-2">{issue.title}</div>
                <div class="flex items-center justify-between gap-2 mt-2 text-[11px] text-base-content/60">
                    <div class="flex items-center gap-1 flex-wrap">
                        <span class=badge>{t(MessageKey::PriorityName { label: issue.priority.to_i18n_label() })}</span>
                        {effort_node}
                        {assignee_node}
                    </div>
                    <span>{date}</span>
                </div>
            </a>
            <form
                method="post"
                action=status_form_action
                class="flex flex-wrap items-center gap-1 mt-2 pt-2 border-t border-base-300">
                <input type="hidden" name="client_updated_at" value=updated_at_for_form/>
                {status_buttons}
            </form>
        </div>
    }
}

#[component]
fn ListView(
    project_id: String,
    issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
    /// Active status filter (empty = "all").
    active_status: String,
    /// Active assignee filter (user id, "unassigned", or empty).
    active_assignee: String,
    /// Active sort key.
    active_sort: String,
) -> impl IntoView {
    let is_empty = issues.is_empty();
    let toolbar_action = format!("/projects/{project_id}");
    let reset_href = toolbar_action.clone();
    let assignees_for_toolbar = assignees.clone();
    view! {
        <>
        // Filter & sort toolbar. Submits as a normal HTML GET
        // form: the browser appends the chosen values as query
        // parameters on the project URL, and the handler parses
        // them out of `ProjectViewQuery`. This is the URL-primary
        // half of v2.1 §4.4. The handler also persists the
        // selection as the user's saved default for this project.
        //
        // No JavaScript: a plain `<select onchange="form.submit()">`
        // would be slightly nicer UX but adds JS for marginal
        // gain. The "Apply" button keeps the page accessible to
        // any user agent.
        <form method="get" action=toolbar_action
              class="flex flex-wrap items-end gap-2 mb-3"
              aria-label=t(MessageKey::FilterSortAriaLabel)>
            // Hidden field so toolbar submission keeps us in list
            // view. Without this, picking a filter would bounce
            // the user back to the board view default.
            <input type="hidden" name="view" value="list"/>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">{t(MessageKey::FieldLabel { field: Field::Status })}</span>
                </div>
                <select name="status" class="select select-sm select-bordered">
                    <option value="" selected=active_status.is_empty()>{t(MessageKey::AllStatusesOption)}</option>
                    {IssueStatus::all().into_iter().map(|s| {
                        let s_str = s.as_str().to_string();
                        let selected = active_status == s_str;
                        let label = t(MessageKey::IssueStatusName { label: s.to_i18n_label() });
                        view! {
                            <option value=s_str.clone() selected=selected>{label}</option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">{t(MessageKey::FieldLabel { field: Field::Assignee })}</span>
                </div>
                <select name="assignee" class="select select-sm select-bordered">
                    <option value="" selected=active_assignee.is_empty()>{t(MessageKey::AnyoneOption)}</option>
                    <option value="unassigned"
                            selected={active_assignee == "unassigned"}>
                        {t(MessageKey::UnassignedOption)}
                    </option>
                    {assignees_for_toolbar.into_iter().map(|a| {
                        let a_id = a.id.clone();
                        let selected = active_assignee == a_id;
                        view! {
                            <option value=a_id selected=selected>
                                {a.display_name}
                            </option>
                        }
                    }).collect_view()}
                </select>
            </label>

            <label class="form-control">
                <div class="label py-0">
                    <span class="label-text text-xs">{t(MessageKey::SortByFieldLabel)}</span>
                </div>
                <select name="sort" class="select select-sm select-bordered">
                    <option value="" selected=active_sort.is_empty()>{t(MessageKey::SortDefaultOption)}</option>
                    <option value="priority"
                            selected={active_sort == "priority"}>{t(MessageKey::FieldLabel { field: Field::Priority })}</option>
                    <option value="created"
                            selected={active_sort == "created"}>{t(MessageKey::SortRecentlyCreatedOption)}</option>
                    <option value="updated"
                            selected={active_sort == "updated"}>{t(MessageKey::SortRecentlyUpdatedOption)}</option>
                </select>
            </label>

            <button type="submit" class="btn btn-sm btn-primary">{t(MessageKey::ApplyButton)}</button>
            // "Reset" links back to the bare list URL with no
            // filter/sort params. Per the handler logic, a bare
            // URL does NOT clear the saved server default — the
            // user would need to explicitly choose "All / Anyone
            // / Default" and click Apply to overwrite the saved
            // state. This is a deliberate trade-off: the
            // alternative ("clicking Reset wipes saved state")
            // would conflict with users navigating via generic
            // links, who would otherwise lose their filter
            // every time.
            <a href=reset_href class="btn btn-sm btn-ghost"
               aria-label=t(MessageKey::ResetFilterAriaLabel)>
                {t(MessageKey::ResetLink)}
            </a>
        </form>

        <div class="card bg-base-100 border border-base-300">
            <div class="overflow-x-auto">
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>{t(MessageKey::FieldLabel { field: Field::Title })}</th>
                            <th class="w-32">{t(MessageKey::FieldLabel { field: Field::Status })}</th>
                            <th class="w-28">{t(MessageKey::FieldLabel { field: Field::Priority })}</th>
                            <th class="w-20">{t(MessageKey::FieldLabel { field: Field::EffortPoints })}</th>
                            <th class="w-32">{t(MessageKey::FieldLabel { field: Field::Assignee })}</th>
                            <th class="w-32">{t(MessageKey::UpdatedColumnHeading)}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {issues.into_iter().map(|issue| {
                            let href = format!("/projects/{}/issues/{}", project_id, issue.id);
                            let pri_class = format!("badge badge-sm {}", issue.priority.badge_class());
                            let updated = issue.updated_at.format("%Y-%m-%d %H:%M").to_string();
                            let status_text = t(MessageKey::IssueStatusName { label: issue.status.to_i18n_label() });
                            let priority_text = t(MessageKey::PriorityName { label: issue.priority.to_i18n_label() });
                            let effort_text = match issue.effort {
                                Some(e) => t(MessageKey::PointsValue { points: e }),
                                None => t(MessageKey::NoValuePlaceholder),
                            };
                            let assignee_text = match issue.assignee_id.as_ref() {
                                Some(aid) => assignee_label(aid, &assignees).to_string(),
                                None => t(MessageKey::NoValuePlaceholder),
                            };
                            view! {
                                <tr class="hover">
                                    <td>
                                        <a href=href class="link link-hover font-medium">
                                            {issue.title}
                                        </a>
                                    </td>
                                    <td><span class="badge badge-sm badge-ghost">{status_text}</span></td>
                                    <td><span class=pri_class>{priority_text}</span></td>
                                    <td class="text-xs text-base-content/70">{effort_text}</td>
                                    <td class="text-xs text-base-content/70">{assignee_text}</td>
                                    <td class="text-xs text-base-content/60">{updated}</td>
                                </tr>
                            }
                        }).collect_view()}
                        {is_empty.then(|| view! {
                            <tr>
                                <td colspan="6" class="text-center py-8 text-base-content/60 italic">
                                    {t(MessageKey::EmptyIssueListMessage)}
                                </td>
                            </tr>
                        })}
                    </tbody>
                </table>
            </div>
        </div>
        </>
    }
}

/// Form for creating a new issue under a project.
#[component]
pub fn IssueNewPage(
    user: CurrentUser,
    project: Project,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    flash: Option<String>,
) -> impl IntoView {
    let title = t(MessageKey::IssueNewPageTitle {
        project_name: project.name.clone(),
    });
    let back_link = format!("/projects/{}", project.id);
    let submit_action = format!("/projects/{}/issues/new", project.id);
    let name_for_breadcrumb = project.name.clone();
    let back_link_for_breadcrumb = back_link.clone();

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-2xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">{t(MessageKey::ProjectsSectionName)}</a></li>
                    <li><a href=back_link_for_breadcrumb>{name_for_breadcrumb}</a></li>
                    <li>{t(MessageKey::NewIssueLabel)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-4">{t(MessageKey::NewIssueLabel)}</h1>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=submit_action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Title })}</span></div>
                            <input type="text" name="title" required=true maxlength="200" autofocus=true
                                   class="input input-bordered input-sm w-full"
                                   placeholder=t(MessageKey::NewIssueTitlePlaceholder)/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span></div>
                            <textarea name="description" rows="6" maxlength="10000"
                                      class="textarea textarea-bordered textarea-sm w-full font-mono text-xs"
                                      placeholder=t(MessageKey::NewIssueDescriptionPlaceholder)></textarea>
                        </label>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <label class="form-control w-full">
                                <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Status })}</span></div>
                                <select name="status" class="select select-bordered select-sm w-full">
                                    {statuses.into_iter().map(|s| {
                                        let selected = s.as_str() == "open";
                                        let label = t(MessageKey::IssueStatusName { label: s.to_i18n_label() });
                                        view! {
                                            <option value=s.as_str() selected=selected>{label}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Priority })}</span></div>
                                <select name="priority" class="select select-bordered select-sm w-full">
                                    {priorities.into_iter().map(|p| {
                                        let selected = p.as_str() == "medium";
                                        let label = t(MessageKey::PriorityName { label: p.to_i18n_label() });
                                        view! {
                                            <option value=p.as_str() selected=selected>{label}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::EffortPoints })}</span>
                                    <span class="label-text-alt text-xs opacity-60">{t(MessageKey::StoryPointsHint)}</span>
                                </div>
                                <select name="effort" class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>{t(MessageKey::NoValuePlaceholder)}</option>
                                    {peisear_core::EFFORT_PRESETS.iter().map(|n| {
                                        view! {
                                            <option value=n.to_string()>{n.to_string()}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Assignee })}</span>
                                </div>
                                <select name="assignee_id" class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>{t(MessageKey::NoValuePlaceholder)}</option>
                                    {assignees.into_iter().map(|a| {
                                        view! {
                                            <option value=a.id>{a.display_name}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>
                        </div>

                        <WorkloadHint workload=workload/>

                        <div class="card-actions justify-end mt-2">
                            <a href=back_link class="btn btn-ghost btn-sm">{t(MessageKey::CancelButton)}</a>
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::CreateIssueButton)}</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

/// Form for creating a new sub-issue under a parent issue
/// (Phase C PR1, peisear-feature-spec-v2.1 §8.3).
///
/// Mirrors `IssueNewPage` but:
///
/// - Posts to the parent's `/sub-issues/new` endpoint, so the
///   handler can wire the new row to its parent at insert
///   time.
/// - Has no Sprint picker — sub-issues follow the parent's
///   sprint (decision: parent sprint follow-up rule), so
///   exposing a separate selector here would imply
///   independence the system doesn't actually grant.
/// - The breadcrumb threads through the parent: Projects →
///   Project → Parent issue → "New sub-issue", so the user
///   can navigate up either way.
/// - The "Cancel" link returns to the parent detail page,
///   not the project board, since that's where the user came
///   from.
#[component]
pub fn SubIssueNewPage(
    user: CurrentUser,
    project: Project,
    parent: Issue,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    flash: Option<String>,
) -> impl IntoView {
    let title = t(MessageKey::SubIssueNewPageTitle {
        parent_title: parent.title.clone(),
    });
    let project_href = format!("/projects/{}", project.id);
    let parent_href = format!("/projects/{}/issues/{}", project.id, parent.id);
    let submit_action = format!(
        "/projects/{}/issues/{}/sub-issues/new",
        project.id, parent.id
    );
    let project_name = project.name.clone();
    let parent_title = parent.title.clone();
    let parent_href_for_cancel = parent_href.clone();

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-2xl mx-auto">
                <div class="breadcrumbs text-sm mb-2"><ul>
                    <li><a href="/projects">{t(MessageKey::ProjectsSectionName)}</a></li>
                    <li><a href=project_href>{project_name}</a></li>
                    <li><a href=parent_href>{parent_title}</a></li>
                    <li>{t(MessageKey::NewSubIssueLabel)}</li>
                </ul></div>

                <h1 class="text-xl font-semibold mb-1">{t(MessageKey::NewSubIssueLabel)}</h1>
                <p class="text-sm text-base-content/60 mb-4">
                    {t(MessageKey::SubIssueNewPageIntro)}
                </p>

                <div class="card bg-base-100 border border-base-300 shadow-sm">
                    <form method="post" action=submit_action class="card-body gap-3">
                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Title })}</span>
                            </div>
                            <input type="text" name="title" required=true maxlength="200" autofocus=true
                                   class="input input-bordered input-sm w-full"
                                   placeholder=t(MessageKey::NewSubIssueTitlePlaceholder)/>
                        </label>

                        <label class="form-control w-full">
                            <div class="label py-1">
                                <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span>
                            </div>
                            <textarea name="description" rows="6" maxlength="10000"
                                      class="textarea textarea-bordered textarea-sm w-full font-mono text-xs"
                                      placeholder=t(MessageKey::NewSubIssueDescriptionPlaceholder)></textarea>
                        </label>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Status })}</span>
                                </div>
                                <select name="status"
                                        class="select select-bordered select-sm w-full">
                                    {statuses.into_iter().map(|s| {
                                        let selected = s.as_str() == "open";
                                        let label = t(MessageKey::IssueStatusName { label: s.to_i18n_label() });
                                        view! {
                                            <option value=s.as_str() selected=selected>{label}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Priority })}</span>
                                </div>
                                <select name="priority"
                                        class="select select-bordered select-sm w-full">
                                    {priorities.into_iter().map(|p| {
                                        let selected = p.as_str() == "medium";
                                        let label = t(MessageKey::PriorityName { label: p.to_i18n_label() });
                                        view! {
                                            <option value=p.as_str() selected=selected>{label}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::EffortPoints })}</span>
                                    <span class="label-text-alt text-xs opacity-60">{t(MessageKey::StoryPointsHint)}</span>
                                </div>
                                <select name="effort"
                                        class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>{t(MessageKey::NoValuePlaceholder)}</option>
                                    {peisear_core::EFFORT_PRESETS.iter().map(|n| {
                                        view! {
                                            <option value=n.to_string()>{n.to_string()}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>

                            <label class="form-control w-full">
                                <div class="label py-1">
                                    <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Assignee })}</span>
                                </div>
                                <select name="assignee_id"
                                        class="select select-bordered select-sm w-full">
                                    <option value="" selected=true>{t(MessageKey::NoValuePlaceholder)}</option>
                                    {assignees.into_iter().map(|a| {
                                        view! {
                                            <option value=a.id>{a.display_name}</option>
                                        }
                                    }).collect_view()}
                                </select>
                            </label>
                        </div>

                        <div class="card-actions justify-end mt-2">
                            <a href=parent_href_for_cancel class="btn btn-ghost btn-sm">{t(MessageKey::CancelButton)}</a>
                            <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::CreateSubIssueButton)}</button>
                        </div>
                    </form>
                </div>
            </div>
        </AppShell>
    }
}

/// Issue detail page. Read-only by default; the parent `editing`
/// parameter switches to the edit form. Phase B PR3 (B-3) split
/// view and edit modes into separate URLs:
/// `/projects/{id}/issues/{issue_id}` for view, `/edit` suffix for
/// edit. The legacy `?edit=1` query parameter 308-redirects to
/// the new edit URL.
#[component]
pub fn IssueDetailPage(
    user: CurrentUser,
    project: Project,
    issue: Issue,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    sprint_options: Vec<(String, String)>,
    current_sprint_id: Option<String>,
    /// Sub-issues of this issue (Phase C PR1).
    sub_issues: Vec<Issue>,
    /// The parent issue if this row is a sub-issue. None for
    /// top-level issues.
    parent_issue: Option<Issue>,
    flash: Option<String>,
    editing: bool,
) -> impl IntoView {
    let title = t(MessageKey::IssueDetailPageTitle {
        issue_title: issue.title.clone(),
        project_name: project.name.clone(),
    });
    let project_href = format!("/projects/{}", project.id);
    let issue_href = format!("/projects/{}/issues/{}", project.id, issue.id);
    // Phase B PR3 (B-3): edit URL is explicit, not a query
    // parameter. Refresh, browser-back, and "Open in new tab"
    // now consistently land on the right mode.
    let edit_href = format!("/projects/{}/issues/{}/edit", project.id, issue.id);
    // `CONF-001`: `GET` here renders the confirmation interstitial,
    // `POST` performs the delete — same path, so this href also
    // serves as the originating control's link target.
    let delete_href = format!("/projects/{}/issues/{}/delete", project.id, issue.id);
    let submit_action = issue_href.clone();
    let project_name_for_breadcrumb = project.name.clone();
    let project_href_for_breadcrumb = project_href.clone();
    let issue_title_for_breadcrumb = issue.title.clone();

    let body = if editing {
        view! {
            <IssueEditForm
                submit_action=submit_action
                issue=issue.clone()
                issue_href=issue_href
                priorities=priorities
                statuses=statuses
                assignees=assignees.clone()
                workload=workload
            />
        }
        .into_any()
    } else {
        view! {
            <IssueView
                issue=issue.clone()
                assignees=assignees.clone()
                edit_href=edit_href
                delete_href=delete_href
            />
        }
        .into_any()
    };

    let has_sprint_options = !sprint_options.is_empty();
    let sprint_action = format!("/projects/{}/issues/{}/sprint", project.id, issue.id);
    let current_sprint_for_select = current_sprint_id.clone();
    let sprint_options_view = sprint_options
        .into_iter()
        .map(|(id, name)| {
            let selected = current_sprint_for_select.as_deref() == Some(id.as_str());
            view! {
                <option value=id selected=selected>{name}</option>
            }
        })
        .collect_view();
    let no_sprint_selected = current_sprint_id.is_none();
    // Phase C PR1: sub-issues don't get their own sprint
    // selector — they follow the parent's sprint per
    // peisear-feature-spec-v2.1 §8.5. The sprint membership is
    // still visible on the detail page (rendered in the meta
    // row up top), so the user can see what sprint they're in;
    // they just can't change it from this page. To re-sprint
    // a sub-issue, the user changes the parent's sprint
    // (which propagates).
    let show_sprint_picker = has_sprint_options && issue.is_top_level();

    // Phase C PR1: sub-issue card. Top-level issues with no
    // children get a "+ Add sub-issue" affordance; ones with
    // children get the list. Sub-issues themselves don't show
    // this section at all (1-level rule means they can't have
    // children).
    let new_sub_issue_href = format!(
        "/projects/{}/issues/{}/sub-issues/new",
        project.id, issue.id
    );
    let sub_issues_card = if issue.is_top_level() {
        Some(view! {
            <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                     aria-label=t(MessageKey::SubIssuesLabel)>
                <div class="card-body py-3">
                    <div class="flex items-center justify-between mb-2">
                        <h2 class="text-sm font-medium">{t(MessageKey::SubIssuesLabel)}</h2>
                        <a href=new_sub_issue_href class="btn btn-ghost btn-xs">
                            {t(MessageKey::AddSubIssueLink)}
                        </a>
                    </div>
                    {if sub_issues.is_empty() {
                        view! {
                            <p class="text-xs italic text-base-content/50">
                                {t(MessageKey::SubIssuesEmptyMessage)}
                            </p>
                        }.into_any()
                    } else {
                        let project_id_for_links = project.id.clone();
                        view! {
                            <ul class="divide-y divide-base-200">
                                {sub_issues.into_iter().map(|si| {
                                    let detail_href = format!(
                                        "/projects/{}/issues/{}",
                                        project_id_for_links, si.id
                                    );
                                    let status_badge_class = format!(
                                        "badge badge-xs {}",
                                        match si.status {
                                            IssueStatus::Open => "badge-ghost",
                                            IssueStatus::InProgress => "badge-primary",
                                            IssueStatus::Done => "badge-success",
                                        }
                                    );
                                    let status_label = t(MessageKey::IssueStatusName {
                                        label: si.status.to_i18n_label(),
                                    });
                                    let aria = t(MessageKey::SubIssueAriaLabel {
                                        title: si.title.clone(),
                                        status: si.status.to_i18n_label(),
                                    });
                                    view! {
                                        <li class="py-2 flex items-center gap-2"
                                            aria-label=aria>
                                            <span class=status_badge_class>
                                                {status_label}
                                            </span>
                                            <a href=detail_href class="text-sm hover:underline flex-1">
                                                {si.title}
                                            </a>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }}
                </div>
            </section>
        })
    } else {
        None
    };

    let sprint_card = show_sprint_picker.then(|| view! {
        <section class="card bg-base-100 border border-base-300 shadow-sm mt-4"
                 aria-label=t(MessageKey::SprintAssignmentLabel)>
            <div class="card-body py-3">
                <form method="post" action=sprint_action
                      class="flex items-center gap-2 flex-wrap">
                    <label class="text-sm font-medium" for="sprint-select">{t(MessageKey::SprintFieldLabel)}</label>
                    <select id="sprint-select" name="sprint_id"
                            class="select select-bordered select-sm flex-1 min-w-[14rem]"
                            aria-label=t(MessageKey::SprintSelectAriaLabel)>
                        <option value="" selected=no_sprint_selected>{t(MessageKey::NoSprintOption)}</option>
                        {sprint_options_view}
                    </select>
                    <button type="submit" class="btn btn-ghost btn-sm">{t(MessageKey::SaveButton)}</button>
                </form>
                <p class="text-xs text-base-content/60 mt-1">
                    {t(MessageKey::SprintAssignmentHelperText)}
                </p>
            </div>
        </section>
    });

    // Phase C PR1: parent-aware breadcrumb.
    // For sub-issues, insert the parent issue as a link
    // between the project and the current issue. The user
    // sees "Projects / FooProject / Parent Title / This sub-
    // issue" and can navigate up either to the project or to
    // the parent.
    let breadcrumb_items = {
        let mut items = vec![
            super::breadcrumb::BreadcrumbItem::link(
                t(MessageKey::ProjectsSectionName),
                "/projects",
            ),
            super::breadcrumb::BreadcrumbItem::link(
                project_name_for_breadcrumb,
                project_href_for_breadcrumb.clone(),
            ),
        ];
        if let Some(parent) = &parent_issue {
            let parent_href = format!("/projects/{}/issues/{}", project.id, parent.id);
            items.push(super::breadcrumb::BreadcrumbItem::link(
                parent.title.clone(),
                parent_href,
            ));
        }
        items.push(super::breadcrumb::BreadcrumbItem::current(
            issue_title_for_breadcrumb,
        ));
        items
    };

    view! {
        <AppShell title=title user=user flash=flash>
            <div class="max-w-3xl mx-auto">
                {super::breadcrumb::render_breadcrumb(breadcrumb_items)}
                {super::breadcrumb::render_back_link(
                    NavSection::Issues,
                    project_href_for_breadcrumb,
                )}
                {body}
                {sub_issues_card}
                {sprint_card}
            </div>
        </AppShell>
    }
}

#[component]
fn IssueEditForm(
    submit_action: String,
    issue: Issue,
    issue_href: String,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
) -> impl IntoView {
    let current_status = issue.status.as_str();
    let current_priority = issue.priority.as_str();
    let current_effort = issue.effort;
    let current_assignee_id = issue.assignee_id.clone();
    let title_value = issue.title.clone();
    let description = issue.description.clone();
    // CAL-001 (RFC 002): `datetime-local` wants `YYYY-MM-DDTHH:MM`.
    // Empty string when unset renders an empty input, not "0000-...".
    let planned_start_value = issue
        .planned_start_at
        .map(|d| d.format("%Y-%m-%dT%H:%M").to_string())
        .unwrap_or_default();
    let planned_end_value = issue
        .planned_end_at
        .map(|d| d.format("%Y-%m-%dT%H:%M").to_string())
        .unwrap_or_default();
    // RFC3339 captured at render time. The handler verifies
    // this against the issue's current `updated_at` per
    // peisear-feature-spec-v2.1 §21.4 and rejects with 409 if
    // someone else has saved an edit between this render and
    // the form submission.
    let client_updated_at = issue.updated_at.to_rfc3339();

    view! {
        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <form method="post" action=submit_action class="card-body gap-3">
                // Optimistic-lock guard. If a concurrent edit
                // lands first, this stale value triggers the
                // 409 conflict path. See §21.4.
                <input type="hidden" name="client_updated_at" value=client_updated_at/>

                <label class="form-control w-full">
                    <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Title })}</span></div>
                    <input type="text" name="title" required=true maxlength="200"
                           value=title_value
                           class="input input-bordered input-sm w-full"/>
                </label>

                <label class="form-control w-full">
                    <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Description })}</span></div>
                    <textarea name="description" rows="8" maxlength="10000"
                              class="textarea textarea-bordered textarea-sm w-full font-mono text-xs">
                        {description}
                    </textarea>
                </label>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <label class="form-control w-full">
                        <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Status })}</span></div>
                        <select name="status" class="select select-bordered select-sm w-full">
                            {statuses.into_iter().map(|s| {
                                let selected = s.as_str() == current_status;
                                let label = t(MessageKey::IssueStatusName { label: s.to_i18n_label() });
                                view! {
                                    <option value=s.as_str() selected=selected>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1"><span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Priority })}</span></div>
                        <select name="priority" class="select select-bordered select-sm w-full">
                            {priorities.into_iter().map(|p| {
                                let selected = p.as_str() == current_priority;
                                let label = t(MessageKey::PriorityName { label: p.to_i18n_label() });
                                view! {
                                    <option value=p.as_str() selected=selected>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1">
                            <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::EffortPoints })}</span>
                            <span class="label-text-alt text-xs opacity-60">{t(MessageKey::StoryPointsHint)}</span>
                        </div>
                        <select name="effort" class="select select-bordered select-sm w-full">
                            <option value="" selected=current_effort.is_none()>{t(MessageKey::NoValuePlaceholder)}</option>
                            {peisear_core::EFFORT_PRESETS.iter().map(|n| {
                                let selected = current_effort == Some(*n);
                                view! {
                                    <option value=n.to_string() selected=selected>{n.to_string()}</option>
                                }
                            }).collect_view()}
                            // If the issue has a non-preset effort value, render
                            // an extra <option> so the form preserves it instead
                            // of silently coercing it to "—" on save.
                            {match current_effort {
                                Some(n) if !peisear_core::EFFORT_PRESETS.contains(&n) => {
                                    Some(view! {
                                        <option value=n.to_string() selected=true>
                                            {n.to_string()}
                                        </option>
                                    })
                                }
                                _ => None,
                            }}
                        </select>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1">
                            <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::Assignee })}</span>
                        </div>
                        <select name="assignee_id" class="select select-bordered select-sm w-full">
                            <option value="" selected=current_assignee_id.is_none()>{t(MessageKey::NoValuePlaceholder)}</option>
                            {assignees.into_iter().map(|a| {
                                let selected = current_assignee_id.as_deref() == Some(a.id.as_str());
                                view! {
                                    <option value=a.id selected=selected>{a.display_name}</option>
                                }
                            }).collect_view()}
                        </select>
                    </label>
                    <label class="form-control w-full">
                        <div class="label py-1">
                            <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::PlannedStartDate })}</span>
                        </div>
                        <input type="datetime-local" name="planned_start_at"
                               value=planned_start_value
                               class="input input-bordered input-sm w-full"/>
                    </label>

                    <label class="form-control w-full">
                        <div class="label py-1">
                            <span class="label-text text-sm">{t(MessageKey::FieldLabel { field: Field::PlannedEndDate })}</span>
                        </div>
                        <input type="datetime-local" name="planned_end_at"
                               value=planned_end_value
                               class="input input-bordered input-sm w-full"/>
                    </label>
                </div>

                <WorkloadHint workload=workload/>

                <div class="card-actions justify-end mt-2">
                    <a href=issue_href class="btn btn-ghost btn-sm">{t(MessageKey::CancelButton)}</a>
                    <button type="submit" class="btn btn-primary btn-sm">{t(MessageKey::SaveButton)}</button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn IssueView(
    issue: Issue,
    assignees: Vec<AssigneeOption>,
    edit_href: String,
    delete_href: String,
) -> impl IntoView {
    let pri_class = format!("badge badge-sm {}", issue.priority.badge_class());
    let created = issue.created_at.format("%Y-%m-%d %H:%M").to_string();
    let updated = issue.updated_at.format("%Y-%m-%d %H:%M").to_string();
    let has_desc = !issue.description.is_empty();
    let description = issue.description.clone();
    let assignee_node = issue.assignee_id.as_ref().map(|aid| {
        let name = assignee_label(aid, &assignees).to_string();
        view! {
            <span class="badge badge-sm badge-ghost" title=t(MessageKey::FieldLabel { field: Field::Assignee })>
                {name}
            </span>
        }
    });

    view! {
        <div class="flex items-start justify-between gap-3 mb-3">
            <h1 class="text-xl font-semibold tracking-tight">{issue.title}</h1>
            <div class="flex gap-2 shrink-0">
                <a href=edit_href.clone() class="btn btn-ghost btn-sm">{t(MessageKey::EditWord)}</a>
                <a href=delete_href class="btn btn-ghost btn-sm text-error">
                    {t(MessageKey::DeleteButton)}
                </a>
            </div>
        </div>

        // Phase B PR3 (B-4): status segment control. Three
        // mutually-exclusive segments (Open / In Progress /
        // Done), with the current status highlighted. Display
        // only — clicking does NOT mutate; the user clicks
        // Edit to change. Direct-manipulation status changes
        // come in Phase D.
        //
        // Visually: the active segment uses `btn-primary` so
        // it stands out against the inactive `btn-ghost`
        // siblings. The shared `btn-disabled` cursor keeps it
        // clear that this is a read-only display, not an
        // interactive switch.
        //
        // Accessibility: the wrapping div carries
        // `role="group"` + `aria-label`, and each segment is
        // a button with `aria-pressed` reflecting whether it
        // matches the current status. Screen readers
        // announce "Open, pressed; In Progress, not pressed;
        // Done, not pressed" so the segmented semantics carry
        // through.
        <div class="join mb-3" role="group" aria-label=t(MessageKey::IssueStatusAriaLabel)>
            {IssueStatus::all().into_iter().map(|s| {
                let is_current = s == issue.status;
                let pressed = if is_current { "true" } else { "false" };
                // `btn-primary` highlights the active segment;
                // others are `btn-ghost` so they recede.
                // `cursor-default` signals "click does nothing
                // here." All segments stay enabled for screen
                // reader navigation; their `aria-pressed`
                // attribute carries the active/inactive
                // semantics. The buttons have `type="button"`
                // (not submit) and no click handler, so they
                // are inert by design.
                let cls = if is_current {
                    "join-item btn btn-sm btn-primary cursor-default"
                } else {
                    "join-item btn btn-sm btn-ghost cursor-default"
                };
                let label = t(MessageKey::IssueStatusName { label: s.to_i18n_label() });
                view! {
                    <button type="button"
                            class=cls
                            aria-pressed=pressed
                            tabindex="-1">
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>

        <div class="flex flex-wrap items-center gap-2 text-xs text-base-content/70 mb-4">
            <span class=pri_class>{t(MessageKey::PriorityName { label: issue.priority.to_i18n_label() })}</span>
            {issue.effort.map(|e| {
                let label = t(MessageKey::PointsValue { points: e });
                view! {
                    <span class="badge badge-sm badge-outline" title=t(MessageKey::EffortEstimateTooltip)>
                        {label}
                    </span>
                }
            })}
            {assignee_node}
            <span>"·"</span>
            <span>{t(MessageKey::CreatedAt { formatted: created })}</span>
            <span>"·"</span>
            <span>{t(MessageKey::UpdatedAt { formatted: updated })}</span>
        </div>

        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                {if has_desc {
                    view! {
                        <pre class="whitespace-pre-wrap break-words font-sans text-sm leading-relaxed">
                            {description}
                        </pre>
                    }.into_any()
                } else {
                    view! {
                        <p class="text-sm italic text-base-content/50">{t(MessageKey::NoDescriptionProvided)}</p>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_project_detail(
    user: CurrentUser,
    project: Project,
    columns: Vec<Column>,
    view_mode: String,
    all_issues: Vec<Issue>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    health: ProjectHealthReport,
    flash: Option<String>,
    // Phase A Step 3 (v2.1 §4.4): the currently-active filter
    // and sort, computed by the handler from URL params merged
    // with server-saved defaults. Empty string means "no
    // constraint" — the UI dropdown shows that as the default
    // option.
    active_status: String,
    active_assignee: String,
    active_sort: String,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <ProjectDetailPage
                user=user
                project=project
                columns=columns
                view_mode=view_mode
                all_issues=all_issues
                assignees=assignees
                workload=workload
                health=health
                flash=flash
                active_status=active_status
                active_assignee=active_assignee
                active_sort=active_sort
            />
        }
    })
}

pub fn render_issue_new(
    user: CurrentUser,
    project: Project,
    priorities: Vec<Priority>,
    statuses: Vec<IssueStatus>,
    assignees: Vec<AssigneeOption>,
    workload: Vec<UserLoad>,
    flash: Option<String>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <IssueNewPage
                user=user
                project=project
                priorities=priorities
                statuses=statuses
                assignees=assignees
                workload=workload
                flash=flash
            />
        }
    })
}

/// Render the sub-issue creation form (Phase C PR1). Wrapper
/// around [`SubIssueNewPage`] that supplies the priority and
/// status enum lists from `peisear_core` defaults — handlers
/// don't need to know about those.
pub fn render_new_sub_issue_form(
    user: CurrentUser,
    project: Project,
    parent: Issue,
    assignees: Vec<AssigneeOption>,
) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <SubIssueNewPage
                user=user
                project=project
                parent=parent
                priorities=Priority::all().to_vec()
                statuses=IssueStatus::all().to_vec()
                assignees=assignees
                flash=None
            />
        }
    })
}

/// Everything the issue detail page needs to render, grouped so
/// `render_issue_detail` takes one parameter instead of thirteen
/// (`clippy::too_many_arguments`, DEV-008). Internal to this crate —
/// callers are handlers in the same crate, so it stays `pub(crate)`
/// rather than exported.
pub(crate) struct IssueDetailView {
    pub user: CurrentUser,
    pub project: Project,
    pub issue: Issue,
    pub priorities: Vec<Priority>,
    pub statuses: Vec<IssueStatus>,
    pub assignees: Vec<AssigneeOption>,
    pub workload: Vec<UserLoad>,
    /// Sprints in the project's team that the user can pick
    /// from. Empty vec when the project is personal (no team)
    /// or the team has no `planned`/`active` sprints.
    pub sprint_options: Vec<(String, String)>,
    /// The sprint id this issue is currently in, if any.
    pub current_sprint_id: Option<String>,
    /// Sub-issues of this issue (Phase C PR1). Always empty for
    /// sub-issues themselves (one-level rule); may be empty for
    /// top-level issues that haven't been broken down yet.
    pub sub_issues: Vec<Issue>,
    /// The parent issue if this row is a sub-issue. Used for
    /// breadcrumb context.
    pub parent_issue: Option<Issue>,
    pub flash: Option<String>,
    pub editing: bool,
}

pub(crate) fn render_issue_detail(view: IssueDetailView) -> Html<String> {
    super::render_to_html(move || {
        view! {
            <IssueDetailPage
                user=view.user
                project=view.project
                issue=view.issue
                priorities=view.priorities
                statuses=view.statuses
                assignees=view.assignees
                workload=view.workload
                sprint_options=view.sprint_options
                current_sprint_id=view.current_sprint_id
                sub_issues=view.sub_issues
                parent_issue=view.parent_issue
                flash=view.flash
                editing=view.editing
            />
        }
    })
}
