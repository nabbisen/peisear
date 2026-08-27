//! A locale that exists only here, in test code — never a
//! [`peisear_i18n::Locale`] variant, never shipped.
//!
//! `I18N-001` §4.5 / RFC 006 open question 4's default: proving the
//! mechanism is not secretly English-shaped requires a second,
//! genuinely distinct rendering path exercised by the same tests as
//! the shipped English table — without committing to a real second
//! locale (Japanese, the obvious candidate) that would then drift
//! unmaintained while `NFR-LANG-005` keeps it unshipped. Every value
//! below is deliberately unlike English (bracketed `[fx …]` tokens)
//! so a test comparing the two outputs proves rendering switches
//! wholesale, not that it happens to differ in one place.
//!
//! Same enforced-exhaustiveness lints as `src/en.rs`/`src/locale.rs`
//! (`I18N-001-review.md` §4), extended here per
//! `I18N-002-003-review.md` §1.5: this file's `match` matters more
//! than test code usually would, since it's what proves the
//! mechanism isn't English-shaped. A wildcard arm here would let
//! the locale-switching test keep passing while silently not
//! covering every key — the guard's own guard, unguarded. Both
//! lints, not one — `match_wildcard_for_single_variants` is the one
//! that catches the realistic single-arm-swapped-for-`_` regression
//! (verified empirically in the `I18N-001` correction;
//! `wildcard_enum_match_arm` alone does not fire on it).
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::match_wildcard_for_single_variants)]

use peisear_i18n::{
    DriftDirectionLabel, EntityKind, Field, HealthStateLabel, IndicatorLabel, IssueStatusLabel,
    MessageKey, NavSection, NotificationChannelLabel, NotificationKindLabel, PriorityLabel,
    SprintStatusLabel, TeamRoleLabel, TrendDirectionLabel,
};

pub fn render(key: MessageKey) -> String {
    match key {
        MessageKey::Forbidden => "[fx] access denied".to_string(),
        MessageKey::NotFound => "[fx] nothing here".to_string(),
        MessageKey::InternalError => "[fx] internal trouble — try again".to_string(),
        MessageKey::OptimisticLockConflict { entity } => format!(
            "[fx] {} changed elsewhere while editing",
            entity_label(entity)
        ),
        MessageKey::LockValueUnreadable => "[fx] stale page — reload".to_string(),
        MessageKey::FieldRequired { field } => format!("[fx] {} needed", field_label(field)),
        MessageKey::FieldMustBePositiveInteger { field } => {
            format!("[fx] {} needs a positive whole number", field_label(field))
        }
        MessageKey::FieldMustBeDateFormat { field } => {
            format!("[fx-date-format] {}", field_label(field))
        }
        MessageKey::FieldMustBeDatetimeFormat { field } => {
            format!("[fx-datetime-format] {}", field_label(field))
        }
        MessageKey::InvalidStatus => "[fx] bad status".to_string(),
        MessageKey::InvalidPriority => "[fx] bad priority".to_string(),

        // ---- I18N-004: IndicatorKind ----
        MessageKey::IndicatorName { label } => format!("[fx-name] {}", indicator_label(label)),

        // ---- I18N-002: format_value ----
        MessageKey::IndicatorValueUnavailable => "[fx-none]".to_string(),
        MessageKey::IndicatorValueThroughput { done, total } => {
            format!("[fx] {done} of {total} done")
        }
        MessageKey::IndicatorValueStaleness { days } => format!("[fx] {days} days old"),
        MessageKey::IndicatorValueActivity { count } => format!("[fx-activity-{count}]"),
        MessageKey::IndicatorValueBusFactorSolo => "[fx-solo]".to_string(),
        MessageKey::IndicatorValueBusFactor { pct } => format!("[fx] {pct} pct concentrated"),
        MessageKey::IndicatorValueLongStale { stale, in_flight } => {
            format!("[fx] {stale} of {in_flight} stale")
        }
        MessageKey::IndicatorValueWipAllWithin => "[fx-wip-ok]".to_string(),
        MessageKey::IndicatorValueWipOver { count } => format!("[fx] {count} over wip"),

        // ---- I18N-002: human_explanation ----
        MessageKey::IndicatorExplanationThroughput { done, total } => {
            format!("[fx] throughput note: {done} of {total}")
        }
        MessageKey::IndicatorExplanationStaleness { days } => {
            format!("[fx] stale note: {days} days")
        }
        MessageKey::IndicatorExplanationActivity { count } => {
            format!("[fx] activity note: {count}")
        }
        MessageKey::IndicatorExplanationBusFactorSolo => "[fx] solo note".to_string(),
        MessageKey::IndicatorExplanationBusFactor { pct } => {
            format!("[fx] concentration note: {pct} pct")
        }
        MessageKey::IndicatorExplanationLongStale { stale, in_flight } => {
            format!("[fx] stale note: {stale} of {in_flight}")
        }
        MessageKey::IndicatorExplanationWipCompliance { count } => {
            format!("[fx] wip note: {count}")
        }

        // ---- I18N-002/004: project_health::summarize ----
        MessageKey::HealthSummaryHealthy => "[fx-healthy]".to_string(),
        MessageKey::HealthSummaryOneWatch { label } => {
            format!("[fx] {} worth a look", indicator_label(label))
        }
        MessageKey::HealthSummaryTwoWatch { first, second } => format!(
            "[fx] {} and {} worth a look",
            indicator_label(first),
            indicator_label(second)
        ),

        // ---- I18N-002: user_burnout::summarize ----
        MessageKey::BurnoutSummarySteady => "[fx-steady]".to_string(),
        MessageKey::BurnoutSummaryOverloadOnly { days } => {
            format!("[fx] overload note: {days} days")
        }
        MessageKey::BurnoutSummaryStalledOnly { days } => {
            format!("[fx] stalled note: {days} days")
        }
        MessageKey::BurnoutSummaryBoth {
            overload_days,
            stalled_days,
        } => format!("[fx] overload {overload_days}, stalled {stalled_days}"),

        // ---- I18N-003: peisear_notify::edge ----
        MessageKey::NotificationBurnoutOverloadTitle => "[fx-overload-title]".to_string(),
        MessageKey::NotificationBurnoutOverloadBody { streak_snapshots } => {
            format!("[fx] overload body: {streak_snapshots}")
        }
        MessageKey::NotificationBurnoutStalledTitle => "[fx-stalled-title]".to_string(),
        MessageKey::NotificationBurnoutStalledBody { stalled_days } => {
            format!("[fx] stalled body: {stalled_days}")
        }

        // ---- I18N-005a: components/{layout,breadcrumb,error_page} ----
        MessageKey::AppBrandName => "[fx-brand]".to_string(),
        MessageKey::NavBellLabelNone => "[fx-bell-none]".to_string(),
        MessageKey::NavBellLabelUnread { count } => format!("[fx-bell-unread-{count}]"),
        MessageKey::NavBellCount { count } => format!("[fx-bell-count-{count}]"),
        MessageKey::NavSearchFormLabel => "[fx-search-form]".to_string(),
        MessageKey::NavSearchPlaceholder => "[fx-search-placeholder]".to_string(),
        MessageKey::NavSearchQueryLabel => "[fx-search-query]".to_string(),
        MessageKey::NavSearchSuggestionsLabel => "[fx-search-suggestions]".to_string(),
        MessageKey::NavLinkToday => "[fx-today]".to_string(),
        MessageKey::NavLinkTeams => "[fx-teams]".to_string(),
        MessageKey::NavLinkInbox => "[fx-inbox]".to_string(),
        MessageKey::NavLinkSettings => "[fx-settings]".to_string(),
        MessageKey::NavSignOut => "[fx-sign-out]".to_string(),
        MessageKey::BreadcrumbNavLabel => "[fx-breadcrumb]".to_string(),
        MessageKey::BackToSection { section } => format!("[fx-back-to] {}", nav_section(section)),
        MessageKey::ErrorPageTitle => "[fx-error-title]".to_string(),
        MessageKey::ErrorPageGoHomeLink => "[fx-go-home]".to_string(),

        // ---- I18N-005b: components/{issues,projects} ----
        MessageKey::IssueStatusName { label } => {
            format!("[fx-status] {}", issue_status_label(label))
        }
        MessageKey::PriorityName { label } => format!("[fx-priority] {}", priority_label(label)),
        MessageKey::FieldLabel { field } => format!("[fx-field] {}", field_label(field)),
        MessageKey::ProjectsSectionName => "[fx-projects-section]".to_string(),
        MessageKey::ViewToggleBoard => "[fx-view-board]".to_string(),
        MessageKey::ViewToggleList => "[fx-view-list]".to_string(),
        MessageKey::EditWord => "[fx-edit]".to_string(),
        MessageKey::CancelButton => "[fx-cancel]".to_string(),
        MessageKey::SaveButton => "[fx-save]".to_string(),
        MessageKey::DeleteButton => "[fx-delete]".to_string(),
        MessageKey::NoValuePlaceholder => "[fx-no-value]".to_string(),
        MessageKey::StoryPointsHint => "[fx-story-points]".to_string(),
        MessageKey::PointsValue { points } => format!("[fx-points-{points}]"),
        MessageKey::HealthEmptyMessage => "[fx-health-empty]".to_string(),
        MessageKey::ProjectHealthSectionLabel => "[fx-health-section]".to_string(),
        MessageKey::HealthHeading => "[fx-health-heading]".to_string(),
        MessageKey::IndicatorsSummaryLabel => "[fx-indicators-summary]".to_string(),
        MessageKey::WorkloadHeading => "[fx-workload-heading]".to_string(),
        MessageKey::WorkloadSetCapacityLink => "[fx-set-capacity]".to_string(),
        MessageKey::WorkloadTitle {
            display_name,
            in_flight_issues,
        } => format!("[fx-workload-title] {display_name} {in_flight_issues}"),
        MessageKey::WorkloadHintLabel => "[fx-workload-hint]".to_string(),
        MessageKey::EmptyBoardHint => "[fx-empty-board]".to_string(),
        MessageKey::MoveIssueAriaLabel {
            issue_title,
            target,
        } => format!("[fx-move] {issue_title} {}", issue_status_label(target)),
        MessageKey::FilterSortAriaLabel => "[fx-filter-sort]".to_string(),
        MessageKey::AllStatusesOption => "[fx-all-statuses]".to_string(),
        MessageKey::AnyoneOption => "[fx-anyone]".to_string(),
        MessageKey::UnassignedOption => "[fx-unassigned]".to_string(),
        MessageKey::SortByFieldLabel => "[fx-sort-by]".to_string(),
        MessageKey::SortDefaultOption => "[fx-sort-default]".to_string(),
        MessageKey::SortRecentlyCreatedOption => "[fx-sort-created]".to_string(),
        MessageKey::SortRecentlyUpdatedOption => "[fx-sort-updated]".to_string(),
        MessageKey::ApplyButton => "[fx-apply]".to_string(),
        MessageKey::ResetFilterAriaLabel => "[fx-reset-aria]".to_string(),
        MessageKey::ResetLink => "[fx-reset]".to_string(),
        MessageKey::UpdatedColumnHeading => "[fx-updated-heading]".to_string(),
        MessageKey::EmptyIssueListMessage => "[fx-empty-issues]".to_string(),
        MessageKey::EffortEstimateTooltip => "[fx-effort-estimate]".to_string(),
        MessageKey::ProjectDetailPageTitle { project_name } => {
            format!("[fx-project-detail-title] {project_name}")
        }
        MessageKey::IssueNewPageTitle { project_name } => {
            format!("[fx-new-issue-title] {project_name}")
        }
        MessageKey::NewIssueLabel => "[fx-new-issue]".to_string(),
        MessageKey::NewIssueTitlePlaceholder => "[fx-new-issue-title-ph]".to_string(),
        MessageKey::NewIssueDescriptionPlaceholder => "[fx-new-issue-desc-ph]".to_string(),
        MessageKey::CreateIssueButton => "[fx-create-issue]".to_string(),
        MessageKey::SubIssueNewPageTitle { parent_title } => {
            format!("[fx-new-sub-issue-title] {parent_title}")
        }
        MessageKey::NewSubIssueLabel => "[fx-new-sub-issue]".to_string(),
        MessageKey::SubIssueNewPageIntro => "[fx-sub-issue-intro]".to_string(),
        MessageKey::NewSubIssueTitlePlaceholder => "[fx-new-sub-issue-title-ph]".to_string(),
        MessageKey::NewSubIssueDescriptionPlaceholder => "[fx-new-sub-issue-desc-ph]".to_string(),
        MessageKey::CreateSubIssueButton => "[fx-create-sub-issue]".to_string(),
        MessageKey::IssueDetailPageTitle {
            issue_title,
            project_name,
        } => format!("[fx-issue-detail-title] {issue_title} {project_name}"),
        MessageKey::SubIssuesLabel => "[fx-sub-issues]".to_string(),
        MessageKey::AddSubIssueLink => "[fx-add-sub-issue]".to_string(),
        MessageKey::SubIssuesEmptyMessage => "[fx-sub-issues-empty]".to_string(),
        MessageKey::SubIssueAriaLabel { title, status } => {
            format!("[fx-sub-issue-aria] {title} {}", issue_status_label(status))
        }
        MessageKey::SprintAssignmentLabel => "[fx-sprint-assignment]".to_string(),
        MessageKey::SprintFieldLabel => "[fx-sprint-field]".to_string(),
        MessageKey::SprintSelectAriaLabel => "[fx-sprint-select]".to_string(),
        MessageKey::NoSprintOption => "[fx-no-sprint]".to_string(),
        MessageKey::SprintAssignmentHelperText => "[fx-sprint-helper]".to_string(),
        MessageKey::IssueStatusAriaLabel => "[fx-issue-status-aria]".to_string(),
        MessageKey::IssueStatusGroupAriaLabel { issue_title } => {
            format!("[fx-issue-status-group-aria] {issue_title}")
        }
        MessageKey::NoDescriptionProvided => "[fx-no-description-provided]".to_string(),
        MessageKey::CreatedAt { formatted } => format!("[fx-created] {formatted}"),
        MessageKey::UpdatedAt { formatted } => format!("[fx-updated] {formatted}"),
        MessageKey::ProjectsListPageTitle => "[fx-projects-list-title]".to_string(),
        MessageKey::ProjectsSubheading => "[fx-projects-subheading]".to_string(),
        MessageKey::NewProjectLabel => "[fx-new-project]".to_string(),
        MessageKey::ProjectsEmptyMessage => "[fx-projects-empty]".to_string(),
        MessageKey::CreateFirstProjectButton => "[fx-create-first-project]".to_string(),
        MessageKey::NoDescriptionShort => "[fx-no-description-short]".to_string(),
        MessageKey::ProjectNewPageTitle => "[fx-project-new-title]".to_string(),
        MessageKey::NewBreadcrumbWord => "[fx-new-breadcrumb]".to_string(),
        MessageKey::ProjectNamePlaceholder => "[fx-project-name-ph]".to_string(),
        MessageKey::ProjectDescriptionPlaceholder => "[fx-project-desc-ph]".to_string(),
        MessageKey::TeamFieldLabel => "[fx-team-field]".to_string(),
        MessageKey::OptionalHint => "[fx-optional]".to_string(),
        MessageKey::PersonalNoTeamOption => "[fx-personal-no-team]".to_string(),
        MessageKey::TeamHelperText => "[fx-team-helper]".to_string(),
        MessageKey::CreateProjectButton => "[fx-create-project]".to_string(),
        MessageKey::ProjectEditPageTitle { project_name } => {
            format!("[fx-project-edit-title] {project_name}")
        }
        MessageKey::EditProjectHeading => "[fx-edit-project]".to_string(),
        MessageKey::DeleteProjectHeading => "[fx-delete-project]".to_string(),
        MessageKey::DeleteProjectWarning => "[fx-delete-project-warning]".to_string(),
        MessageKey::IssueDeletedFlash => "[fx-issue-deleted]".to_string(),
        MessageKey::ProjectDeletedFlash => "[fx-project-deleted]".to_string(),

        // ---- I18N-005c: components/{sprints,teams} ----
        MessageKey::SprintStatusName { label } => {
            format!("[fx-sprint-status] {}", sprint_status_label(label))
        }
        MessageKey::TeamRoleName { label } => format!("[fx-team-role] {}", team_role_label(label)),
        MessageKey::NewSprintLink => "[fx-new-sprint-link]".to_string(),
        MessageKey::SprintsPageTitle { team_name } => format!("[fx-sprints-title] {team_name}"),
        MessageKey::SprintsSectionName => "[fx-sprints-section]".to_string(),
        MessageKey::SprintsListAriaLabel => "[fx-sprint-list]".to_string(),
        MessageKey::SprintCardSummaryCompleted {
            completed_points,
            committed_points,
            carried_over_points,
        } => format!(
            "[fx-summary-completed] {completed_points} {committed_points} {carried_over_points}"
        ),
        MessageKey::SprintCardSummaryActive {
            completed_points,
            committed_points,
            in_flight_points,
        } => {
            format!("[fx-summary-active] {completed_points} {committed_points} {in_flight_points}")
        }
        MessageKey::SprintCardSummaryPlanned {
            committed_points,
            committed_count,
        } => format!("[fx-summary-planned] {committed_points} {committed_count}"),
        MessageKey::SprintCardAriaLabel {
            name,
            status,
            dates,
            completed_points,
            committed_points,
            carried_over_points,
            committed_count,
        } => format!(
            "[fx-sprint-card-aria] {name} {} {dates} {completed_points} {committed_points} \
             {carried_over_points} {committed_count}",
            sprint_status_label(status)
        ),
        MessageKey::VelocityBarAriaLabel {
            name,
            completed_points,
            carried_over_points,
        } => format!("[fx-bars-bar] {name} {completed_points} {carried_over_points}"),
        MessageKey::SprintsEmptyMessageAdmin => "[fx-sprints-empty-admin]".to_string(),
        MessageKey::SprintsEmptyMessageNonAdmin => "[fx-sprints-empty-non-admin]".to_string(),
        MessageKey::SprintsOptionalNote => "[fx-sprints-optional]".to_string(),
        MessageKey::CompletedWorkHeading => "[fx-completed-work]".to_string(),
        MessageKey::RecentCompletedSprintsAriaLabel => "[fx-recent-completed]".to_string(),
        MessageKey::VelocityCaptionLead => "[fx-bars-lead]".to_string(),
        MessageKey::CaptionWordCompleted => "[fx-word-completed]".to_string(),
        MessageKey::VelocityCaptionMiddle => "[fx-bars-middle]".to_string(),
        MessageKey::CaptionWordCarriedOver => "[fx-word-carried-over]".to_string(),
        MessageKey::VelocityCaptionCarriedOverClose => "[fx-bars-carried-over-close]".to_string(),
        MessageKey::VelocityCaptionMedianSentence => "[fx-bars-median-sentence]".to_string(),
        MessageKey::VelocityCaptionClosingNote => "[fx-bars-closing-note]".to_string(),
        MessageKey::ChartTableSummaryLabel => "[fx-chart-table-summary]".to_string(),
        MessageKey::VelocitySummaryPointsList {
            points_list,
            sprint_count,
        } => format!("[fx-bars-summary] {points_list} {sprint_count}"),
        MessageKey::VelocitySummaryMedianClause { median } => {
            format!("[fx-bars-summary-median-{median}]")
        }
        MessageKey::VelocityTableAriaLabel => "[fx-bars-table]".to_string(),
        MessageKey::VelocityTableSprintHeader => "[fx-bars-table-sprint-header]".to_string(),
        MessageKey::MedianRowLabel => "[fx-median-row]".to_string(),
        MessageKey::BarChartAriaLabel {
            sprint_count,
            min_completed,
            max_completed,
        } => format!("[fx-bar-chart] {sprint_count} {min_completed} {max_completed}"),
        MessageKey::MedianLabel { median } => format!("[fx-median-{median}]"),
        MessageKey::NewSprintLabel => "[fx-new-sprint]".to_string(),
        MessageKey::SprintNamePlaceholder => "[fx-sprint-name-ph]".to_string(),
        MessageKey::GoalFieldPlaceholder => "[fx-goal-ph]".to_string(),
        MessageKey::SprintPlannedNoticeLead => "[fx-planned-lead]".to_string(),
        MessageKey::CaptionWordPlanned => "[fx-word-planned]".to_string(),
        MessageKey::SprintPlannedNoticeTail => "[fx-planned-tail]".to_string(),
        MessageKey::CreateSprintButton => "[fx-create-sprint]".to_string(),
        MessageKey::StartSprintLabel => "[fx-start-sprint]".to_string(),
        MessageKey::CompleteSprintLabel => "[fx-complete-sprint]".to_string(),
        MessageKey::GoalFieldPrefixLabel => "[fx-goal-prefix]".to_string(),
        MessageKey::SummaryHeading => "[fx-summary]".to_string(),
        MessageKey::CommittedStatLabel => "[fx-committed-stat]".to_string(),
        MessageKey::CompletedStatLabel => "[fx-completed-stat]".to_string(),
        MessageKey::InFlightStatLabel => "[fx-in-flight]".to_string(),
        MessageKey::CarriedOverHeading => "[fx-carried-over-heading]".to_string(),
        MessageKey::PointsUnitSuffix => "[fx-pt]".to_string(),
        MessageKey::IssuesCountText { count } => format!("[fx-issues-count-{count}]"),
        MessageKey::BurndownHeading => "[fx-burndown]".to_string(),
        MessageKey::BurndownSectionAriaLabel => "[fx-burndown-section]".to_string(),
        MessageKey::BurndownCaptionLead => "[fx-burndown-lead]".to_string(),
        MessageKey::CaptionWordCommitted => "[fx-word-committed]".to_string(),
        MessageKey::BurndownCaptionMiddle => "[fx-burndown-middle]".to_string(),
        MessageKey::BurndownCaptionTail => "[fx-burndown-tail]".to_string(),
        MessageKey::BurndownChartAriaLabel {
            first_label,
            last_label,
            max_val,
        } => format!("[fx-burndown-chart] {first_label} {last_label} {max_val}"),
        MessageKey::BurndownSummary {
            day_count,
            first_label,
            last_label,
            first_committed,
            last_committed,
            first_completed,
            last_completed,
            gap,
        } => format!(
            "[fx-burndown-summary] {day_count} {first_label} {last_label} {first_committed} \
             {last_committed} {first_completed} {last_completed} {gap}"
        ),
        MessageKey::BurndownTableAriaLabel => "[fx-burndown-table]".to_string(),
        MessageKey::BurndownTableDateHeader => "[fx-burndown-table-date-header]".to_string(),
        MessageKey::IssuesInSprintAriaLabel => "[fx-issues-in-sprint]".to_string(),
        MessageKey::IssuesHeading => "[fx-issues-heading]".to_string(),
        MessageKey::NoIssuesInSprintMessage => "[fx-no-issues-in-sprint]".to_string(),
        MessageKey::SprintIssuesAriaLabel => "[fx-sprint-issues]".to_string(),
        MessageKey::EditSprintPageTitle { sprint_name } => {
            format!("[fx-edit-sprint-title] {sprint_name}")
        }
        MessageKey::EditSprintHeading => "[fx-edit-sprint]".to_string(),
        MessageKey::SprintPlanPageTitle { sprint_name } => {
            format!("[fx-sprint-plan-title] {sprint_name}")
        }
        MessageKey::SprintPlanBreadcrumbWord => "[fx-sprint-plan-breadcrumb]".to_string(),
        MessageKey::BacklogHeading => "[fx-backlog-heading]".to_string(),
        MessageKey::SprintItemsHeading => "[fx-sprint-items-heading]".to_string(),
        MessageKey::BacklogFilterAriaLabel => "[fx-backlog-filter]".to_string(),
        MessageKey::AllPrioritiesOption => "[fx-all-priorities]".to_string(),
        MessageKey::AllProjectsOption => "[fx-all-projects]".to_string(),
        MessageKey::MoveToSprintButton => "[fx-move-to-sprint]".to_string(),
        MessageKey::MoveToBacklogButton => "[fx-move-to-backlog]".to_string(),
        MessageKey::BacklogRowAriaLabel { title, points } => {
            format!("[fx-backlog-row] {title} {points}")
        }
        MessageKey::SprintItemRowAriaLabel { title, points } => {
            format!("[fx-sprint-item-row] {title} {points}")
        }
        MessageKey::CommittedTotalLabel { committed_points } => {
            format!("[fx-committed-total] {committed_points}")
        }
        MessageKey::NoBacklogIssuesMessage => "[fx-no-backlog-issues]".to_string(),
        MessageKey::NoSprintItemsInPlanMessage => "[fx-no-sprint-items-in-plan]".to_string(),
        MessageKey::SprintPlanNotEditableMessage => "[fx-sprint-plan-not-editable]".to_string(),
        MessageKey::NewTeamLink => "[fx-new-team-link]".to_string(),
        MessageKey::TeamsEmptyIntro => "[fx-teams-empty-intro]".to_string(),
        MessageKey::TeamsEmptyCta => "[fx-teams-empty-cta]".to_string(),
        MessageKey::YourTeamsAriaLabel => "[fx-your-teams]".to_string(),
        MessageKey::TeamRoleAriaLabel { team_name, role } => {
            format!("[fx-team-role-aria] {team_name} {}", team_role_label(role))
        }
        MessageKey::NewTeamLabel => "[fx-new-team]".to_string(),
        MessageKey::TeamNamePlaceholder => "[fx-team-name-ph]".to_string(),
        MessageKey::SlugFieldLabel => "[fx-slug-field]".to_string(),
        MessageKey::OptionalAutoDerivedHint => "[fx-optional-auto]".to_string(),
        MessageKey::SlugPlaceholder => "[fx-slug-ph]".to_string(),
        MessageKey::SlugHelperText => "[fx-slug-helper]".to_string(),
        MessageKey::TeamDescriptionPlaceholder => "[fx-team-desc-ph]".to_string(),
        MessageKey::NewTeamIntro => "[fx-new-team-intro]".to_string(),
        MessageKey::CreateTeamButton => "[fx-create-team]".to_string(),
        MessageKey::EditTeamSettingsAriaLabel => "[fx-edit-team-settings]".to_string(),
        MessageKey::InviteMemberSummary => "[fx-invite-member]".to_string(),
        MessageKey::ByEmailHint => "[fx-by-email]".to_string(),
        MessageKey::EmailPlaceholderExample => "[fx-email-ph]".to_string(),
        MessageKey::AddButton => "[fx-add]".to_string(),
        MessageKey::InviteHelperText => "[fx-invite-helper]".to_string(),
        MessageKey::MembersHeading => "[fx-members]".to_string(),
        MessageKey::TeamMembersAriaLabel => "[fx-team-members]".to_string(),
        MessageKey::JoinedColumnHeading => "[fx-joined]".to_string(),
        MessageKey::TeamPrivacyFootnote => "[fx-privacy-footnote]".to_string(),
        MessageKey::DetachFromTeamAriaLabel => "[fx-detach-from-team]".to_string(),
        MessageKey::DetachButton => "[fx-detach]".to_string(),
        MessageKey::TeamProjectsAriaLabel => "[fx-team-projects]".to_string(),
        MessageKey::NoProjectsInTeamMessage => "[fx-no-projects-in-team]".to_string(),
        MessageKey::ChangeRoleAriaLabel => "[fx-change-role]".to_string(),
        MessageKey::LeaveTeamAriaLabel => "[fx-leave-team]".to_string(),
        MessageKey::LeaveButton => "[fx-leave]".to_string(),
        MessageKey::RemoveMemberAriaLabel => "[fx-remove-member]".to_string(),
        MessageKey::RemoveButton => "[fx-remove]".to_string(),
        MessageKey::YouSuffix => "[fx-you]".to_string(),
        MessageKey::EditTeamPageTitle { team_name } => {
            format!("[fx-edit-team-title] {team_name}")
        }
        MessageKey::TeamSettingsHeading => "[fx-team-settings]".to_string(),
        MessageKey::SlugFixedNotice => "[fx-slug-fixed]".to_string(),
        MessageKey::SprintCreatedFlash => "[fx-sprint-created]".to_string(),
        MessageKey::SprintUpdatedFlash => "[fx-sprint-updated]".to_string(),
        MessageKey::SprintStartedFlash => "[fx-sprint-started]".to_string(),
        MessageKey::SprintCompletedFlash => "[fx-sprint-completed]".to_string(),
        MessageKey::SprintDeletedFlash => "[fx-sprint-deleted]".to_string(),
        MessageKey::SprintAssignmentSavedFlash => "[fx-sprint-assignment-saved]".to_string(),
        MessageKey::TeamCreatedFlash => "[fx-team-created]".to_string(),
        MessageKey::MemberAddedFlash => "[fx-member-added]".to_string(),
        MessageKey::RoleUpdatedFlash => "[fx-role-updated]".to_string(),
        MessageKey::LastAdminDemotionError => "[fx-last-admin-demotion]".to_string(),
        MessageKey::LastAdminRemovalError => "[fx-last-admin-removal]".to_string(),
        MessageKey::YouLeftTeamFlash => "[fx-you-left-team]".to_string(),
        MessageKey::MemberRemovedFlash => "[fx-member-removed]".to_string(),
        MessageKey::TeamUpdatedFlash => "[fx-team-updated]".to_string(),
        MessageKey::ProjectDetachedFlash => "[fx-project-detached]".to_string(),
        MessageKey::NoUserWithEmailFound { email } => format!("[fx-no-user-email] {email}"),

        // ---- I18N-005d: components/me ----
        MessageKey::PaceValue { days_per_point } => format!("[fx-pace-value] {days_per_point}"),
        MessageKey::ReadFirstOverloadTitle => "[fx-rf-overload-title]".to_string(),
        MessageKey::ReadFirstOverloadBody {
            overload_streak_days,
            window_days,
        } => format!("[fx-rf-overload-body] {overload_streak_days} {window_days}"),
        MessageKey::ReadFirstStalledTitle => "[fx-rf-stalled-title]".to_string(),
        MessageKey::ReadFirstStalledBody {
            stalled_assigned_max_days,
        } => format!("[fx-rf-stalled-body] {stalled_assigned_max_days}"),
        MessageKey::ReadFirstWipTitle => "[fx-rf-wip-title]".to_string(),
        MessageKey::ReadFirstWipBody {
            current_wip,
            effective_wip_limit,
        } => format!("[fx-rf-wip-body] {current_wip} {effective_wip_limit}"),
        MessageKey::ReadFirstLongStaleTitle => "[fx-rf-long-stale-title]".to_string(),
        MessageKey::ReadFirstLongStaleBody { long_stale_count } => {
            format!("[fx-rf-long-stale-body] {long_stale_count}")
        }
        MessageKey::PersonalDashboardTitle => "[fx-dashboard-title]".to_string(),
        MessageKey::NothingToShowMessage => "[fx-nothing-to-show]".to_string(),
        MessageKey::PersonalDashboardSubtitle { display_name } => {
            format!("[fx-dashboard-subtitle] {display_name}")
        }
        MessageKey::ReadFirstAriaLabel => "[fx-rf-aria]".to_string(),
        MessageKey::RightNowHeading => "[fx-right-now]".to_string(),
        MessageKey::WipChipLabel => "[fx-wip-chip]".to_string(),
        MessageKey::LoadChipLabel => "[fx-load-chip]".to_string(),
        MessageKey::LoadChipTooltip => "[fx-load-tooltip]".to_string(),
        MessageKey::PeriodHintTooltip => "[fx-period-hint-tooltip]".to_string(),
        MessageKey::ThisPeriodHint => "[fx-this-period]".to_string(),
        MessageKey::RhythmAriaLabel => "[fx-rhythm-aria]".to_string(),
        MessageKey::RhythmSummaryLabel => "[fx-rhythm-summary]".to_string(),
        MessageKey::ThroughputTooltip => "[fx-throughput-tooltip]".to_string(),
        MessageKey::ThroughputChipLabel => "[fx-throughput-chip]".to_string(),
        MessageKey::LongStaleChipLabel => "[fx-long-stale-chip]".to_string(),
        MessageKey::PaceTooltip => "[fx-pace-tooltip]".to_string(),
        MessageKey::PaceChipLabel => "[fx-pace-chip]".to_string(),
        MessageKey::WhatDoTheseMeanLabel => "[fx-what-do-these-mean]".to_string(),
        MessageKey::WipGlossaryDefinition => "[fx-wip-glossary-def]".to_string(),
        MessageKey::LoadGlossaryDefinition => "[fx-load-glossary-def]".to_string(),
        MessageKey::ThroughputGlossaryDefinition { window_days } => {
            format!("[fx-throughput-glossary-def] {window_days}")
        }
        MessageKey::LongStaleGlossaryDefinition { window_days } => {
            format!("[fx-long-stale-glossary-def] {window_days}")
        }
        MessageKey::PaceGlossaryDefinition => "[fx-pace-glossary-def]".to_string(),
        MessageKey::SustainabilityHeading => "[fx-sustainability-heading]".to_string(),
        MessageKey::SustainabilityGlossaryDefinition => {
            "[fx-sustainability-glossary-def]".to_string()
        }
        MessageKey::PatternsSubheading => "[fx-patterns-subheading]".to_string(),
        MessageKey::PatternsGlossaryDefinition => "[fx-patterns-glossary-def]".to_string(),
        MessageKey::OverloadStreakChipLabel => "[fx-overload-streak-chip]".to_string(),
        MessageKey::OldestStalledChipLabel => "[fx-oldest-stalled-chip]".to_string(),
        MessageKey::PatternsDisclaimer => "[fx-patterns-disclaimer]".to_string(),
        MessageKey::SustainabilityPrivacyNote => "[fx-sustainability-privacy]".to_string(),
        MessageKey::OverloadStreakValue {
            overload_streak_days,
            window_days,
        } => format!("[fx-overload-streak-value] {overload_streak_days} {window_days}"),
        MessageKey::StalledDaysValue {
            stalled_assigned_max_days,
        } => format!("[fx-stalled-days-value] {stalled_assigned_max_days}"),
        MessageKey::OverloadStreakAriaLabel {
            overload_streak_days,
            is_watch,
        } => format!("[fx-overload-streak-aria] {overload_streak_days} {is_watch}"),
        MessageKey::StalledAriaLabel {
            stalled_assigned_max_days,
            is_watch,
        } => format!("[fx-stalled-aria] {stalled_assigned_max_days} {is_watch}"),
        MessageKey::DriftInsufficientDataAriaLabel => "[fx-drift-insufficient-aria]".to_string(),
        MessageKey::PaceDriftChipLabel => "[fx-pace-drift-chip]".to_string(),
        MessageKey::NeedMoreDataLabel => "[fx-need-more-data]".to_string(),
        MessageKey::DriftDirectionWord { direction } => {
            format!(
                "[fx-drift-direction-word] {}",
                drift_direction_word(direction)
            )
        }
        MessageKey::DriftValueLine {
            recent_median_days_per_point,
            older_median_days_per_point,
        } => format!(
            "[fx-drift-value-line] {recent_median_days_per_point} {older_median_days_per_point}"
        ),
        MessageKey::DriftAriaLabel {
            recent_median_days_per_point,
            older_median_days_per_point,
            window_days,
            direction,
        } => format!(
            "[fx-drift-aria] {recent_median_days_per_point} {older_median_days_per_point} \
             {window_days} {}",
            drift_direction_word(direction)
        ),
        MessageKey::SwitchingInsufficientDataAriaLabel => {
            "[fx-switching-insufficient-aria]".to_string()
        }
        MessageKey::SwitchingChipLabel => "[fx-switching-chip]".to_string(),
        MessageKey::SwitchingMedianValue { median } => format!("[fx-switching-median] {median}"),
        MessageKey::SwitchingSampleLine {
            total_events_observed,
            window_days,
        } => format!("[fx-switching-sample] {total_events_observed} {window_days}"),
        MessageKey::SwitchingAriaLabel {
            median,
            total_events_observed,
            window_days,
        } => format!("[fx-switching-aria] {median} {total_events_observed} {window_days}"),

        // ---- I18N-005d: components/settings ----
        MessageKey::WipLimitExplanation { default_wip_limit } => {
            format!("[fx-wip-limit-explanation] {default_wip_limit}")
        }
        MessageKey::NoCapacitySetTodayLabel => "[fx-no-capacity-today]".to_string(),
        MessageKey::ConflictLabel => "[fx-conflict-label]".to_string(),
        MessageKey::CapacityOverlapGuidanceLead => "[fx-overlap-guidance-lead]".to_string(),
        MessageKey::CloseOnDateActionWord => "[fx-close-on-date-word]".to_string(),
        MessageKey::CapacityOverlapGuidanceTail => "[fx-overlap-guidance-tail]".to_string(),
        MessageKey::SettingsSectionName => "[fx-settings-section]".to_string(),
        MessageKey::SettingsSubtitle { display_name } => {
            format!("[fx-settings-subtitle] {display_name}")
        }
        MessageKey::CapacitySectionAriaLabel => "[fx-capacity-section-aria]".to_string(),
        MessageKey::WorkloadCapacityHeading => "[fx-workload-capacity-heading]".to_string(),
        MessageKey::CapacityExplanationParagraph => "[fx-capacity-explanation]".to_string(),
        MessageKey::EffectiveCapacityTodayAriaLabel { points } => {
            format!("[fx-effective-capacity-aria] {points:?}")
        }
        MessageKey::EffectiveTodayLabel => "[fx-effective-today-label]".to_string(),
        MessageKey::CapacityRowsTableAriaLabel => "[fx-capacity-rows-table-aria]".to_string(),
        MessageKey::PointsColumnHeading => "[fx-points-heading]".to_string(),
        MessageKey::FromColumnHeading => "[fx-from-heading]".to_string(),
        MessageKey::FromDateFieldLabel => "[fx-from-date-field]".to_string(),
        MessageKey::ToColumnHeading => "[fx-to-heading]".to_string(),
        MessageKey::ToDateFieldLabel => "[fx-to-date-field]".to_string(),
        MessageKey::NoteColumnHeading => "[fx-note-heading]".to_string(),
        MessageKey::ActionsColumnHeading => "[fx-actions-heading]".to_string(),
        MessageKey::AddCapacityRowSummary => "[fx-add-capacity-summary]".to_string(),
        MessageKey::AddCapacityRowFormAriaLabel => "[fx-add-capacity-form-aria]".to_string(),
        MessageKey::PointsPlaceholderExample => "[fx-points-placeholder]".to_string(),
        MessageKey::NoteFieldPlaceholder => "[fx-note-placeholder]".to_string(),
        MessageKey::AddRowButton => "[fx-add-row-button]".to_string(),
        MessageKey::CapacityOverlapHelperText => "[fx-overlap-helper-text]".to_string(),
        MessageKey::WipLimitLabel => "[fx-wip-limit-label]".to_string(),
        MessageKey::InProgressIssuesHint => "[fx-in-progress-hint]".to_string(),
        MessageKey::CapacityRowAriaLabel { points, from, to } => {
            format!("[fx-capacity-row-aria] {points} {from} {to}")
        }
        MessageKey::CloseOnDateSummary => "[fx-close-on-date-summary]".to_string(),
        MessageKey::CloseThisRowAriaLabel => "[fx-close-this-row-aria]".to_string(),
        MessageKey::CloseOnLabel => "[fx-close-on-label]".to_string(),
        MessageKey::CloseButton => "[fx-close-button]".to_string(),
        MessageKey::EditRowAriaLabel => "[fx-edit-row-aria]".to_string(),
        MessageKey::RemoveThisRowAriaLabel => "[fx-remove-row-aria]".to_string(),

        // ---- I18N-005d: components/{notification_preferences,notifications} ----
        MessageKey::EmailNotificationsHeading => "[fx-email-notifications-heading]".to_string(),
        MessageKey::FirstTimeEmailPromptAriaLabel => "[fx-first-time-email-aria]".to_string(),
        MessageKey::EmailOptInPromptBody => "[fx-email-optin-body]".to_string(),
        MessageKey::EmailOptInYesButton => "[fx-email-optin-yes]".to_string(),
        MessageKey::EmailOptInNoButton => "[fx-email-optin-no]".to_string(),
        MessageKey::EmailOptInOnStatus => "[fx-email-optin-on]".to_string(),
        MessageKey::EmailOptInOffStatus => "[fx-email-optin-off]".to_string(),
        MessageKey::NotificationPreferencesPageTitle => "[fx-notif-prefs-title]".to_string(),
        MessageKey::NotificationsSectionName => "[fx-notifications-section]".to_string(),
        MessageKey::SilenceAllAriaLabel => "[fx-silence-all-aria]".to_string(),
        MessageKey::SilenceAllButton => "[fx-silence-all-button]".to_string(),
        MessageKey::DefaultsInAppLead => "[fx-defaults-in-app-lead]".to_string(),
        MessageKey::PerKindDeliverySummary => "[fx-per-kind-delivery]".to_string(),
        MessageKey::ClickToExpandHint => "[fx-click-to-expand]".to_string(),
        MessageKey::NotificationKindsTableAriaLabel => "[fx-notif-kinds-table-aria]".to_string(),
        MessageKey::KindColumnHeading => "[fx-kind-heading]".to_string(),
        MessageKey::MinSeverityColumnHeading => "[fx-min-severity-heading]".to_string(),
        MessageKey::ChannelStubDisclaimer => "[fx-channel-stub-disclaimer]".to_string(),
        MessageKey::SavePreferencesButton => "[fx-save-preferences-button]".to_string(),
        MessageKey::NotificationKindPreferencesAriaLabel { kind } => {
            format!("[fx-kind-prefs-aria] {}", notification_kind_label(kind))
        }
        MessageKey::InAppForKindAriaLabel { kind } => {
            format!(
                "[fx-in-app-for-kind-aria] {}",
                notification_kind_label(kind)
            )
        }
        MessageKey::EmailForKindAriaLabel { kind } => {
            format!("[fx-email-for-kind-aria] {}", notification_kind_label(kind))
        }
        MessageKey::WebhookForKindAriaLabel { kind } => {
            format!(
                "[fx-webhook-for-kind-aria] {}",
                notification_kind_label(kind)
            )
        }
        MessageKey::MinSeverityForKindAriaLabel { kind } => format!(
            "[fx-min-severity-for-kind-aria] {}",
            notification_kind_label(kind)
        ),
        MessageKey::AllSeverityOption => "[fx-all-severity-option]".to_string(),
        MessageKey::WatchOnlySeverityOption => "[fx-watch-only-severity-option]".to_string(),
        MessageKey::NotificationKindName { kind } => {
            format!("[fx-kind-name] {}", notification_kind_label(kind))
        }
        MessageKey::NotificationChannelName { channel } => {
            format!("[fx-channel-name] {}", notification_channel_label(channel))
        }
        MessageKey::NoNotificationsYetStatus => "[fx-no-notifications-yet]".to_string(),
        MessageKey::UnreadOfTotalStatus {
            unread_count,
            total,
        } => {
            format!("[fx-unread-of-total] {unread_count} {total}")
        }
        MessageKey::AllReadStatus { total } => format!("[fx-all-read-status] {total}"),
        MessageKey::MarkAllReadAriaLabel => "[fx-mark-all-read-aria]".to_string(),
        MessageKey::MarkAllReadButton => "[fx-mark-all-read-button]".to_string(),
        MessageKey::InboxEmptyMessage => "[fx-inbox-empty-message]".to_string(),
        MessageKey::InboxEmptyFooterLead => "[fx-inbox-empty-footer-lead]".to_string(),
        MessageKey::SettingsLinkWord => "[fx-settings-link-word]".to_string(),
        MessageKey::InboxEmptyFooterTail => "[fx-inbox-empty-footer-tail]".to_string(),
        MessageKey::NotificationListAriaLabel => "[fx-notif-list-aria]".to_string(),
        MessageKey::UnreadWord => "[fx-unread-word]".to_string(),
        MessageKey::ReadWord => "[fx-read-word]".to_string(),
        MessageKey::NotificationRowAriaLabel {
            is_unread,
            title,
            kind,
            timestamp,
        } => format!(
            "[fx-notif-row-aria] {is_unread} {title} {} {timestamp}",
            notification_kind_label(kind)
        ),
        MessageKey::SentViaPrefix => "[fx-sent-via-prefix]".to_string(),
        MessageKey::ViewContextLinkLabel => "[fx-view-context-link]".to_string(),
        MessageKey::MarkAsReadAriaLabel => "[fx-mark-as-read-aria]".to_string(),
        MessageKey::MarkReadButton => "[fx-mark-read-button]".to_string(),

        // ---- INBOX-001: silence-resume banner ----
        MessageKey::SilenceResumeBannerAriaLabel => "[fx-silence-resume-banner-aria]".to_string(),
        MessageKey::SilenceResumeBannerMessage => "[fx-silence-resume-banner-message]".to_string(),
        MessageKey::ResumeNotificationsAriaLabel => "[fx-resume-notifications-aria]".to_string(),
        MessageKey::ResumeNotificationsButton => "[fx-resume-notifications-button]".to_string(),

        // ---- I18N-005d: components/search ----
        MessageKey::SearchWord => "[fx-search-word]".to_string(),
        MessageKey::SearchPageTitleWithQuery { q } => format!("[fx-search-title-query] {q}"),
        MessageKey::SearchFieldLabel => "[fx-search-field-label]".to_string(),
        MessageKey::SearchPlaceholder => "[fx-search-placeholder-fx]".to_string(),
        MessageKey::ResultsForHeadingPrefix => "[fx-results-for-prefix]".to_string(),
        MessageKey::NoQueryGuidanceMessage => "[fx-no-query-guidance]".to_string(),
        MessageKey::OpenIssuesSectionName => "[fx-open-issues-section]".to_string(),
        MessageKey::NoMatchesInCategoryMessage => "[fx-no-matches-category]".to_string(),
        MessageKey::PreviousPageLink => "[fx-previous-page-link]".to_string(),
        MessageKey::NextPageLink => "[fx-next-page-link]".to_string(),
        MessageKey::ProjectHitTypeLabel => "[fx-project-hit-type]".to_string(),
        MessageKey::OpenIssueHitTypePrefix { project_name } => {
            format!("[fx-open-issue-hit-type] {project_name}")
        }
        MessageKey::SubIssueHitTypePrefix {
            project_name,
            parent_title,
        } => format!("[fx-sub-issue-hit-type] {project_name} {parent_title}"),

        // ---- I18N-005d: handlers/{settings,notification_preferences,notifications} ----
        MessageKey::WipLimitSavedFlash => "[fx-wip-limit-saved]".to_string(),
        MessageKey::CapacityRowAddedFlash => "[fx-capacity-row-added]".to_string(),
        MessageKey::CapacityRowUpdatedFlash => "[fx-capacity-row-updated]".to_string(),
        MessageKey::CapacityRowRemovedFlash => "[fx-capacity-row-removed]".to_string(),
        MessageKey::RowClosedFlash => "[fx-row-closed]".to_string(),
        MessageKey::PreferencesSavedFlash => "[fx-preferences-saved]".to_string(),
        MessageKey::AllNotificationsSilencedFlash => "[fx-all-notifs-silenced]".to_string(),
        MessageKey::MarkedAsReadFlash { count } => format!("[fx-marked-as-read] {count}"),

        // ---- I18N-005e: error.rs (ApiAppError) ----
        MessageKey::ApiUnauthorizedMessage => "[fx-api-unauthorized]".to_string(),
        MessageKey::ApiForbiddenMessage => "[fx-api-forbidden]".to_string(),
        MessageKey::ApiNotFoundMessage => "[fx-api-not-found]".to_string(),
        MessageKey::ApiOptimisticLockConflictMessage { entity } => {
            format!("[fx-api-lock-conflict] {}", entity_label(entity))
        }

        // ---- I18N-005e: components/auth.rs, handlers/auth.rs ----
        MessageKey::LoginPageTitle => "[fx-login-title]".to_string(),
        MessageKey::RegisterPageTitle => "[fx-register-title]".to_string(),
        MessageKey::SignInTaglineText => "[fx-sign-in-tagline]".to_string(),
        MessageKey::RegisterTaglineText => "[fx-register-tagline]".to_string(),
        MessageKey::SignInWord => "[fx-sign-in-word]".to_string(),
        MessageKey::CreateAccountButton => "[fx-create-account-button]".to_string(),
        MessageKey::PasswordFieldLabel => "[fx-password-field]".to_string(),
        MessageKey::DisplayNameFieldLabel => "[fx-display-name-field]".to_string(),
        MessageKey::PasswordMinLengthHint => "[fx-password-hint]".to_string(),
        MessageKey::NoAccountPrompt => "[fx-no-account-prompt]".to_string(),
        MessageKey::CreateOneLinkWord => "[fx-create-one-link]".to_string(),
        MessageKey::AlreadyHaveAccountPrompt => "[fx-already-have-account]".to_string(),
        MessageKey::InvalidCredentialsMessage => "[fx-invalid-credentials]".to_string(),
        MessageKey::EmailAlreadyExistsMessage => "[fx-email-already-exists]".to_string(),
        MessageKey::InvalidInputFallbackMessage => "[fx-invalid-input-fallback]".to_string(),

        // ---- I18N-005e: handlers/issues.rs ----
        MessageKey::InvalidAssigneeMessage => "[fx-invalid-assignee]".to_string(),
        MessageKey::SubIssueCannotNestLongMessage => "[fx-sub-issue-nest-long]".to_string(),

        // ---- I18N-005e: handlers/sprints.rs ----
        MessageKey::SprintNameRequiredMessage => "[fx-sprint-name-required]".to_string(),
        MessageKey::SubIssueFollowsParentSprintMessage => {
            "[fx-sub-issue-follows-parent]".to_string()
        }
        MessageKey::SprintsPersonalProjectMessage => "[fx-sprints-personal-project]".to_string(),
        MessageKey::SprintProjectTeamMismatchMessage => {
            "[fx-sprint-project-team-mismatch]".to_string()
        }
        MessageKey::CannotAssignToCompletedSprintMessage => {
            "[fx-cannot-assign-completed-sprint]".to_string()
        }

        // ---- I18N-005e: handlers/teams.rs ----
        MessageKey::TeamNameRequiredMessage => "[fx-team-name-required]".to_string(),
        MessageKey::SlugDerivationFailedMessage => "[fx-slug-derivation-failed]".to_string(),
        MessageKey::InvalidRoleMessage => "[fx-invalid-role]".to_string(),

        // ---- I18N-005e: handlers/settings.rs ----
        MessageKey::CapacityPointsRequiredMessage => "[fx-capacity-points-required]".to_string(),
        MessageKey::WipLimitMustBePositiveIntegerMessage => {
            "[fx-wip-limit-positive-int]".to_string()
        }
        MessageKey::PeriodStartMustBeDateFormatMessage => {
            "[fx-period-start-date-format]".to_string()
        }
        MessageKey::PeriodEndMustBeDateFormatMessage => "[fx-period-end-date-format]".to_string(),

        // ---- I18N-006: peisear-core/src/lib.rs ----
        MessageKey::IndicatorDescription { label } => {
            format!("[fx-indicator-description] {}", indicator_label(label))
        }
        MessageKey::WipAriaLabel {
            current_wip,
            effective_wip_limit,
            state,
        } => format!(
            "[fx-wip-aria] {current_wip}/{effective_wip_limit} {}",
            health_state_label(state)
        ),
        MessageKey::LongStaleAriaLabel {
            long_stale_count,
            state,
        } => format!(
            "[fx-long-stale-aria] {long_stale_count} {}",
            health_state_label(state)
        ),
        MessageKey::CompositeAriaLabel { state } => {
            format!("[fx-composite-aria] {}", health_state_label(state))
        }
        MessageKey::IndicatorAriaLabel {
            label,
            value,
            state,
        } => format!(
            "[fx-indicator-aria] {} {} {}",
            indicator_label(label),
            render(*value),
            health_state_label(state)
        ),

        // ---- HLT-001 ----
        MessageKey::IndicatorBasisLinkText => "[fx-basis-link-text]".to_string(),
        MessageKey::IndicatorBasisLinkAriaLabel { label } => {
            format!("[fx-basis-link-aria] {}", indicator_label(label))
        }
        MessageKey::IndicatorBasisPageTitle { label } => {
            format!("[fx-basis-page-title] {}", indicator_label(label))
        }
        MessageKey::IndicatorBasisAriaLabel { label } => {
            format!("[fx-basis-aria] {}", indicator_label(label))
        }
        MessageKey::IndicatorBasisEmptyMessage => "[fx-basis-empty]".to_string(),
        MessageKey::IndicatorCalculationSummaryLabel => "[fx-calc-summary]".to_string(),
        MessageKey::IndicatorCalculationThroughput {
            good_pct,
            watch_pct,
        } => format!("[fx-calc-throughput] {good_pct} {watch_pct}"),
        MessageKey::IndicatorCalculationStaleness {
            watch_days,
            concern_days,
        } => format!("[fx-calc-staleness] {watch_days} {concern_days}"),
        MessageKey::IndicatorCalculationActivity {
            good_count,
            watch_count,
            window_days,
        } => format!("[fx-calc-activity] {good_count} {watch_count} {window_days}"),
        MessageKey::IndicatorCalculationBusFactor {
            watch_pct,
            concern_pct,
        } => format!("[fx-calc-bus-factor] {watch_pct} {concern_pct}"),
        MessageKey::IndicatorCalculationLongStale {
            watch_pct,
            concern_pct,
            window_days,
        } => format!("[fx-calc-long-stale] {watch_pct} {concern_pct} {window_days}"),
        MessageKey::IndicatorCalculationWipCompliance {
            watch_pct,
            concern_pct,
        } => format!("[fx-calc-wip-compliance] {watch_pct} {concern_pct}"),

        // ---- I18N-006: peisear-storage/src/user_capacities.rs ----
        MessageKey::PeriodStartMustPrecedeEndMessage => "[fx-period-start-precede-end]".to_string(),
        MessageKey::CapacityPeriodOverlapMessage {
            row_id,
            period_start,
            period_end,
            points,
        } => format!("[fx-capacity-overlap] {row_id} {period_start} {period_end} {points}"),

        // ---- I18N-006: peisear-storage/src/sprints.rs ----
        MessageKey::SprintEndDateMustBeOnOrAfterStartMessage => {
            "[fx-sprint-end-after-start]".to_string()
        }
        MessageKey::SprintAlreadyActiveMessage => "[fx-sprint-already-active]".to_string(),
        MessageKey::SprintCannotRestartCompletedMessage => "[fx-sprint-cannot-restart]".to_string(),
        MessageKey::OtherSprintActiveInTeamMessage { sprint_name } => {
            format!("[fx-other-sprint-active] {sprint_name}")
        }
        MessageKey::SprintNotStartedYetMessage => "[fx-sprint-not-started]".to_string(),
        MessageKey::SprintAlreadyCompletedMessage => "[fx-sprint-already-completed]".to_string(),
        MessageKey::SprintActiveCannotBeDeletedMessage => {
            "[fx-sprint-active-cannot-be-deleted]".to_string()
        }

        // ---- I18N-006: peisear-storage/src/teams.rs ----
        MessageKey::TeamSlugCannotBeEmptyMessage => "[fx-team-slug-empty]".to_string(),
        MessageKey::TeamSlugAlreadyExistsMessage { slug } => {
            format!("[fx-team-slug-exists] {slug}")
        }
        MessageKey::UserAlreadyTeamMemberMessage { user_id } => {
            format!("[fx-user-already-member] {user_id}")
        }

        // ---- I18N-006: peisear-storage/src/issues.rs (translate_trigger_error) ----
        MessageKey::SubIssueCannotHaveSubIssueMessage => "[fx-sub-issue-cannot-nest]".to_string(),
        MessageKey::SubIssueMustShareProjectMessage => "[fx-sub-issue-share-project]".to_string(),
        MessageKey::IssueCannotBeOwnParentMessage => "[fx-issue-own-parent]".to_string(),
        MessageKey::CannotDemoteIssueWithSubIssuesMessage => {
            "[fx-cannot-demote-with-subs]".to_string()
        }
        MessageKey::IssuePlannedEndBeforeStartMessage => {
            "[fx-planned-end-before-start]".to_string()
        }

        // ---- I18N-006: handlers/api_users.rs (BurnoutSignal.label) ----
        MessageKey::OverloadStreakSignalMessage {
            overload_streak_days,
            window_days,
        } => format!("[fx-overload-streak-signal] {overload_streak_days}/{window_days}"),
        MessageKey::StalledAssignedSignalMessage {
            stalled_assigned_max_days,
        } => format!("[fx-stalled-assigned-signal] {stalled_assigned_max_days}"),
        MessageKey::EstimationDriftUpSignalMessage => "[fx-drift-up-signal]".to_string(),
        MessageKey::EstimationDriftDownSignalMessage => "[fx-drift-down-signal]".to_string(),
        MessageKey::CognitiveSwitchingSignalMessage {
            switches_per_day_median,
        } => format!("[fx-switching-signal] {switches_per_day_median:.1}"),

        // ---- I18N-007: components/issues.rs (render_trend_chip) ----
        MessageKey::TrendLabelFlat => "[fx-trend-flat]".to_string(),
        MessageKey::TrendLabel { direction, delta } => match direction {
            TrendDirectionLabel::Up => format!("[fx-trend-up] {delta}"),
            TrendDirectionLabel::Down => format!("[fx-trend-down] {delta}"),
        },
        MessageKey::TrendAriaFlat => "[fx-trend-aria-flat]".to_string(),
        MessageKey::TrendAriaLabel { direction, delta } => match direction {
            TrendDirectionLabel::Up => format!("[fx-trend-aria-up] {delta}"),
            TrendDirectionLabel::Down => format!("[fx-trend-aria-down] {delta}"),
        },

        // ---- I18N-007: components/issues.rs (composite_row) ----
        MessageKey::CompositeLabel => "[fx-composite-label]".to_string(),

        // ---- I18N-007: components/sprints.rs (burndown legend) ----
        MessageKey::BurndownLegendCommitted => "[fx-legend-committed]".to_string(),
        MessageKey::BurndownLegendCompleted => "[fx-legend-completed]".to_string(),

        // ---- I18N-007: components/me.rs ----
        MessageKey::CurrentLoadSectionLabel => "[fx-current-load]".to_string(),
        MessageKey::LoadWithCapacityValue {
            in_flight_points,
            capacity_points,
        } => format!("[fx-load-with-capacity] {in_flight_points}/{capacity_points}"),
        MessageKey::LoadNoCapacityValue { in_flight_points } => {
            format!("[fx-load-no-capacity] {in_flight_points}")
        }
        MessageKey::RecentThroughputValue {
            recent_done_count,
            window_days,
        } => format!("[fx-recent-throughput] {recent_done_count}/{window_days}"),
        MessageKey::ProjectCalendarPrivacyFootnote => "[fx-project-calendar-privacy]".to_string(),
        MessageKey::PersonalCalendarPrivacyFootnote => "[fx-personal-calendar-privacy]".to_string(),
        MessageKey::PersonalCalendarPageTitle => "[fx-personal-calendar-title]".to_string(),
        MessageKey::ProjectCalendarPageTitle { project_name } => {
            format!("[fx-project-calendar-title] {project_name}")
        }
        MessageKey::CalendarBreadcrumbWord => "[fx-calendar-breadcrumb]".to_string(),
        MessageKey::CalendarViewName { view } => format!("[fx-calendar-view] {view:?}"),
        MessageKey::CalendarCellAriaLabel { month, day, count } => {
            format!("[fx-calendar-cell] {month}-{day} {count}")
        }
        MessageKey::CrowdingChipAriaLabel { state } => {
            format!("[fx-crowding-chip] {}", health_state_label(state))
        }
        MessageKey::CalendarUtcNote => "[fx-calendar-utc-note]".to_string(),
        MessageKey::SprintBandAriaLabel { sprint_name } => {
            format!("[fx-sprint-band] {sprint_name}")
        }
        MessageKey::NoPlannedIssuesMessage => "[fx-no-planned-issues]".to_string(),
        MessageKey::CalendarViewSwitcherAriaLabel => "[fx-calendar-view-switcher]".to_string(),
        MessageKey::CalendarMoreIssuesLabel { count } => format!("[fx-calendar-more] {count}"),
        MessageKey::ConfirmDeleteHeading { entity_name } => {
            format!("[fx-confirm-delete-heading] {entity_name}")
        }
        MessageKey::ConfirmDeleteCannotBeUndoneNote => "[fx-confirm-cannot-be-undone]".to_string(),
        MessageKey::ConfirmDeleteProjectCascadeNote => "[fx-confirm-project-cascade]".to_string(),
        MessageKey::ConfirmDeleteSprintPlannedNote => "[fx-confirm-sprint-planned]".to_string(),
        MessageKey::ConfirmDeleteSprintCompletedNote => "[fx-confirm-sprint-completed]".to_string(),
        MessageKey::StatusChangedAnnouncement { status } => {
            format!("[fx-status-changed] {}", issue_status_label(status))
        }
        MessageKey::UndoButtonLabel => "[fx-undo]".to_string(),
        MessageKey::StatusChangeUndoConflictMessage => "[fx-undo-conflict]".to_string(),
        MessageKey::StatusChangeUndoUnavailableMessage => "[fx-undo-unavailable]".to_string(),
        MessageKey::BoardReloadMessage => "[fx-board-reload]".to_string(),
        MessageKey::BoardConflictMessage => "[fx-board-conflict]".to_string(),
        MessageKey::BoardUnavailableMessage => "[fx-board-unavailable]".to_string(),
        MessageKey::ConfirmDeleteIssueCascadeNote { sub_issue_count } => {
            format!("[fx-confirm-issue-cascade] {sub_issue_count}")
        }
    }
}

fn entity_label(entity: EntityKind) -> &'static str {
    match entity {
        EntityKind::Issue => "[fx-issue]",
        EntityKind::Project => "[fx-project]",
        EntityKind::Sprint => "[fx-sprint]",
        EntityKind::Team => "[fx-team]",
        EntityKind::CapacityPeriod => "[fx-capacity-period]",
        EntityKind::TeamMembership => "[fx-team-membership]",
    }
}

fn field_label(field: Field) -> &'static str {
    match field {
        Field::EffortPoints => "[fx-effort]",
        Field::CapacityPoints => "[fx-capacity-points]",
        Field::CloseDate => "[fx-close-date]",
        Field::Title => "[fx-title]",
        Field::Description => "[fx-description]",
        Field::Status => "[fx-status-field]",
        Field::Priority => "[fx-priority-field]",
        Field::Assignee => "[fx-assignee]",
        Field::Name => "[fx-name]",
        Field::StartDate => "[fx-start-date]",
        Field::EndDate => "[fx-end-date]",
        Field::Goal => "[fx-goal]",
        Field::Role => "[fx-role]",
        Field::Email => "[fx-email]",
        Field::Project => "[fx-project]",
        Field::PlannedStartDate => "[fx-planned-start-date]",
        Field::PlannedEndDate => "[fx-planned-end-date]",
    }
}

fn issue_status_label(label: IssueStatusLabel) -> &'static str {
    match label {
        IssueStatusLabel::Open => "[fx-open]",
        IssueStatusLabel::InProgress => "[fx-in-progress]",
        IssueStatusLabel::Done => "[fx-done]",
    }
}

fn sprint_status_label(label: SprintStatusLabel) -> &'static str {
    match label {
        SprintStatusLabel::Planned => "[fx-planned]",
        SprintStatusLabel::Active => "[fx-active]",
        SprintStatusLabel::Completed => "[fx-completed]",
    }
}

fn team_role_label(label: TeamRoleLabel) -> &'static str {
    match label {
        TeamRoleLabel::Admin => "[fx-admin]",
        TeamRoleLabel::Member => "[fx-member]",
        TeamRoleLabel::Viewer => "[fx-viewer]",
    }
}

fn priority_label(label: PriorityLabel) -> &'static str {
    match label {
        PriorityLabel::Low => "[fx-low]",
        PriorityLabel::Medium => "[fx-medium]",
        PriorityLabel::High => "[fx-high]",
        PriorityLabel::Urgent => "[fx-urgent]",
    }
}

fn nav_section(section: NavSection) -> &'static str {
    match section {
        NavSection::Projects => "[fx-projects]",
        NavSection::Issues => "[fx-issues]",
        NavSection::Sprints => "[fx-sprints]",
    }
}

fn indicator_label(label: IndicatorLabel) -> &'static str {
    match label {
        IndicatorLabel::Throughput => "[fx-throughput]",
        IndicatorLabel::Staleness => "[fx-staleness]",
        IndicatorLabel::Activity => "[fx-activity]",
        IndicatorLabel::BusFactor => "[fx-busfactor]",
        IndicatorLabel::LongStale => "[fx-longstale]",
        IndicatorLabel::WipCompliance => "[fx-wipcompliance]",
    }
}

fn health_state_label(state: HealthStateLabel) -> &'static str {
    match state {
        HealthStateLabel::Insufficient => "[fx-hs-insufficient]",
        HealthStateLabel::Good => "[fx-hs-good]",
        HealthStateLabel::Watch => "[fx-hs-watch]",
    }
}

fn drift_direction_word(direction: DriftDirectionLabel) -> &'static str {
    match direction {
        DriftDirectionLabel::Up => "[fx-drift-up]",
        DriftDirectionLabel::Down => "[fx-drift-down]",
        DriftDirectionLabel::Steady => "[fx-drift-steady]",
    }
}

fn notification_kind_label(kind: NotificationKindLabel) -> &'static str {
    match kind {
        NotificationKindLabel::BurnoutOverload => "[fx-kind-burnout-overload]",
        NotificationKindLabel::BurnoutStalled => "[fx-kind-burnout-stalled]",
        NotificationKindLabel::ProjectTrendDecline => "[fx-kind-project-trend-decline]",
    }
}

fn notification_channel_label(channel: NotificationChannelLabel) -> &'static str {
    match channel {
        NotificationChannelLabel::InApp => "[fx-channel-in-app]",
        NotificationChannelLabel::Email => "[fx-channel-email]",
        NotificationChannelLabel::Webhook => "[fx-channel-webhook]",
    }
}
