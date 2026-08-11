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
    EntityKind, Field, IndicatorLabel, IssueStatusLabel, MessageKey, NavSection, PriorityLabel,
    SprintStatusLabel, TeamRoleLabel,
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
        MessageKey::VelocityCaptionTail => "[fx-bars-tail]".to_string(),
        MessageKey::BarChartAriaLabel => "[fx-bar-chart]".to_string(),
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
        MessageKey::IssuesInSprintAriaLabel => "[fx-issues-in-sprint]".to_string(),
        MessageKey::IssuesHeading => "[fx-issues-heading]".to_string(),
        MessageKey::NoIssuesInSprintMessage => "[fx-no-issues-in-sprint]".to_string(),
        MessageKey::SprintIssuesAriaLabel => "[fx-sprint-issues]".to_string(),
        MessageKey::EditSprintPageTitle { sprint_name } => {
            format!("[fx-edit-sprint-title] {sprint_name}")
        }
        MessageKey::EditSprintHeading => "[fx-edit-sprint]".to_string(),
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
