//! The English locale table.
//!
//! `render` is an exhaustive `match` with no wildcard (`_`) arm —
//! that is the whole of the compile-time guarantee `RFC 006`
//! requirement 2 asks for. Add a [`MessageKey`] variant without
//! adding an arm here and this function fails to compile. See
//! `I18N-001`'s review request for a demonstration (temporarily
//! removing an arm and capturing the resulting compiler error).
//!
//! The two `#![deny]`s below are what keep that guarantee from being
//! able to quietly dissolve: without them, a future `_ => ...` arm
//! added to any match in this file would compile cleanly and
//! silently stop distinguishing a missing key from a handled one
//! (`I18N-001-review.md` §4) — the exhaustiveness guarantee would go
//! back to being a convention, exactly the failure mode this release
//! exists to replace. Two lints, not one: `wildcard_enum_match_arm`
//! alone does **not** fire when the wildcard covers exactly one
//! remaining variant — verified empirically while implementing this
//! correction, not assumed from the lint's name — which is precisely
//! the shape a real regression would most likely take (one arm
//! quietly swapped for `_`, not several at once).
//! `match_wildcard_for_single_variants` covers that gap. Scoped to
//! this module rather than crate-wide, since both are restriction
//! lints that would also fire on unrelated matches over enums this
//! crate doesn't own.
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::match_wildcard_for_single_variants)]

use crate::message::{
    CalendarViewLabel, DriftDirectionLabel, EntityKind, Field, HealthStateLabel, IndicatorLabel,
    IssueStatusLabel, MessageKey, NavSection, NotificationChannelLabel, NotificationKindLabel,
    PriorityLabel, SprintStatusLabel, TeamRoleLabel, TrendDirectionLabel,
};

pub(crate) fn render(key: MessageKey) -> String {
    match key {
        MessageKey::Forbidden => "permission denied".to_string(),
        MessageKey::NotFound => "resource not found".to_string(),
        MessageKey::InternalError => "An internal error occurred. Please try again.".to_string(),
        MessageKey::OptimisticLockConflict { entity } => format!(
            "Someone else updated this {} while you were editing. \
             Please reload the page and re-apply your change so you \
             don't overwrite their work.",
            entity_label(entity)
        ),
        MessageKey::LockValueUnreadable => {
            "This page is showing an earlier version. Reload to see the current state.".to_string()
        }
        MessageKey::FieldRequired { field } => format!("{} is required.", field_label(field)),
        MessageKey::FieldMustBePositiveInteger { field } => {
            format!("{} must be a positive integer.", field_label(field))
        }
        MessageKey::FieldMustBeDateFormat { field } => {
            format!("{} must be in YYYY-MM-DD format.", field_label(field))
        }
        MessageKey::FieldMustBeDatetimeFormat { field } => {
            format!("{} must be in YYYY-MM-DDTHH:MM format.", field_label(field))
        }
        MessageKey::InvalidStatus => "Invalid status".to_string(),
        MessageKey::InvalidPriority => "Invalid priority".to_string(),

        // ---- I18N-004: IndicatorKind ----
        MessageKey::IndicatorName { label } => indicator_label(label).to_string(),

        // ---- I18N-002: format_value ----
        MessageKey::IndicatorValueUnavailable => "—".to_string(),
        MessageKey::IndicatorValueThroughput { done, total } => throughput_value(done, total),
        MessageKey::IndicatorValueStaleness { days } => format!("{days} d"),
        MessageKey::IndicatorValueActivity { count } => format!("{count}"),
        MessageKey::IndicatorValueBusFactorSolo => "solo".to_string(),
        MessageKey::IndicatorValueBusFactor { pct } => format!("{pct}% on top"),
        MessageKey::IndicatorValueLongStale { stale, in_flight } => {
            format!("{stale} / {in_flight}")
        }
        MessageKey::IndicatorValueWipAllWithin => "all within".to_string(),
        MessageKey::IndicatorValueWipOver { count } => format!("{count} over"),

        // ---- I18N-002: human_explanation ----
        MessageKey::IndicatorExplanationThroughput { done, total } => format!(
            "Throughput is {} — fewer issues are reaching Done than the rest of the project's history.",
            throughput_value(done, total)
        ),
        MessageKey::IndicatorExplanationStaleness { days } => {
            format!("The oldest in-flight issue has been open for {days} d.")
        }
        MessageKey::IndicatorExplanationActivity { count } => {
            format!("Issue activity in the last two weeks is {count}.")
        }
        // I18N-004 fix for ISSUE-006 finding 2 -- see message.rs's
        // doc comment on this variant.
        MessageKey::IndicatorExplanationBusFactorSolo => {
            "In-flight work is currently carried by one person.".to_string()
        }
        MessageKey::IndicatorExplanationBusFactor { pct } => {
            format!("{pct}% on top of in-flight work is concentrated on one person.")
        }
        MessageKey::IndicatorExplanationLongStale { stale, in_flight } => format!(
            "{stale} / {in_flight} of in-flight issues haven't been touched in over two weeks."
        ),
        // I18N-004 fix for ISSUE-006 finding 3 -- the count is now a
        // typed parameter rather than an embedded "N over" string,
        // which is what produced the doubling.
        MessageKey::IndicatorExplanationWipCompliance { count } => {
            format!("{count} active assignees are over their WIP limit.")
        }

        // ---- I18N-002/004: project_health::summarize ----
        MessageKey::HealthSummaryHealthy => "Looking healthy.".to_string(),
        // I18N-004: the only two reachable shapes now -- see this
        // variant's doc comment in message.rs.
        MessageKey::HealthSummaryOneWatch { label } => {
            format!("{} is worth a glance.", indicator_label(label))
        }
        MessageKey::HealthSummaryTwoWatch { first, second } => format!(
            "{} and {} are worth a glance.",
            indicator_label(first),
            indicator_label(second)
        ),

        // ---- I18N-002: user_burnout::summarize ----
        MessageKey::BurnoutSummarySteady => "Steady so far.".to_string(),
        MessageKey::BurnoutSummaryOverloadOnly { days } => format!(
            "you've been over capacity for {days} recent snapshots — \
             consider whether some work can wait or move"
        ),
        MessageKey::BurnoutSummaryStalledOnly { days } => format!(
            "an assigned issue has been stuck for {days} days — \
             worth a quick check whether it's blocked"
        ),
        MessageKey::BurnoutSummaryBoth {
            overload_days,
            stalled_days,
        } => format!(
            "you've been over capacity for {overload_days} recent snapshots — \
             consider whether some work can wait or move; \
             an assigned issue has been stuck for {stalled_days} days — \
             worth a quick check whether it's blocked"
        ),

        // ---- I18N-003: peisear_notify::edge ----
        MessageKey::NotificationBurnoutOverloadTitle => {
            "Sustained over-capacity streak".to_string()
        }
        MessageKey::NotificationBurnoutOverloadBody { streak_snapshots } => format!(
            "Your in-flight load has been over capacity for {streak_snapshots} \
             consecutive snapshots. This is a description of the recent rhythm, \
             not an evaluation of your work — many streaks have legitimate causes. \
             You can review at /today."
        ),
        MessageKey::NotificationBurnoutStalledTitle => "Long-stalled assigned work".to_string(),
        MessageKey::NotificationBurnoutStalledBody { stalled_days } => format!(
            "An assigned issue has been in flight for {stalled_days} days. \
             May be worth a glance — sometimes a quick check-in turns out to be \
             all that's needed. Visit /today for context."
        ),

        // ---- I18N-005a: components/{layout,breadcrumb,error_page} ----
        MessageKey::AppBrandName => "Issue Tracker".to_string(),
        MessageKey::NavBellLabelNone => "Notifications".to_string(),
        MessageKey::NavBellLabelUnread { count } => format!("Notifications ({count} unread)"),
        MessageKey::NavBellCount { count } => bell_count(count),
        MessageKey::NavSearchFormLabel => "Search projects and open issues".to_string(),
        MessageKey::NavSearchPlaceholder => "Search...".to_string(),
        MessageKey::NavSearchQueryLabel => "Search query".to_string(),
        MessageKey::NavSearchSuggestionsLabel => "Search suggestions".to_string(),
        MessageKey::NavLinkToday => "Today".to_string(),
        MessageKey::NavLinkTeams => "Teams".to_string(),
        MessageKey::NavLinkInbox => "Inbox".to_string(),
        MessageKey::NavLinkSettings => "Settings".to_string(),
        MessageKey::NavSignOut => "Sign out".to_string(),
        MessageKey::BreadcrumbNavLabel => "Breadcrumb".to_string(),
        MessageKey::BackToSection { section } => format!("Back to {}", nav_section(section)),
        MessageKey::ErrorPageTitle => "Error — Issue Tracker".to_string(),
        MessageKey::ErrorPageGoHomeLink => "Go home".to_string(),

        // ---- I18N-005b: components/{issues,projects} ----
        MessageKey::IssueStatusName { label } => issue_status_label(label).to_string(),
        MessageKey::PriorityName { label } => priority_label(label).to_string(),
        MessageKey::FieldLabel { field } => field_label(field).to_string(),
        MessageKey::ProjectsSectionName => "Projects".to_string(),
        MessageKey::ViewToggleBoard => "Board".to_string(),
        MessageKey::ViewToggleList => "List".to_string(),
        MessageKey::EditWord => "Edit".to_string(),
        MessageKey::CancelButton => "Cancel".to_string(),
        MessageKey::SaveButton => "Save".to_string(),
        MessageKey::DeleteButton => "Delete".to_string(),
        MessageKey::NoValuePlaceholder => "—".to_string(),
        MessageKey::StoryPointsHint => "story points".to_string(),
        MessageKey::PointsValue { points } => format!("{points} pt"),
        MessageKey::HealthEmptyMessage => {
            "No issues yet — health indicators will appear once work starts.".to_string()
        }
        MessageKey::ProjectHealthSectionLabel => "Project health".to_string(),
        MessageKey::HealthHeading => "Health".to_string(),
        MessageKey::IndicatorsSummaryLabel => "Indicators".to_string(),
        MessageKey::WorkloadHeading => "Workload".to_string(),
        MessageKey::WorkloadSetCapacityLink => "(set your capacity)".to_string(),
        MessageKey::WorkloadTitle {
            display_name,
            in_flight_issues,
        } => format!("{display_name} — {in_flight_issues} in-flight issues"),
        MessageKey::WorkloadHintLabel => "Workload:".to_string(),
        MessageKey::EmptyBoardHint => "Drop issues here".to_string(),
        MessageKey::MoveIssueAriaLabel {
            issue_title,
            target,
        } => format!("Move \"{issue_title}\" to {}", issue_status_label(target)),
        MessageKey::FilterSortAriaLabel => "Filter and sort issues".to_string(),
        MessageKey::AllStatusesOption => "All statuses".to_string(),
        MessageKey::AnyoneOption => "Anyone".to_string(),
        MessageKey::UnassignedOption => "Unassigned".to_string(),
        MessageKey::SortByFieldLabel => "Sort by".to_string(),
        MessageKey::SortDefaultOption => "Default".to_string(),
        MessageKey::SortRecentlyCreatedOption => "Recently created".to_string(),
        MessageKey::SortRecentlyUpdatedOption => "Recently updated".to_string(),
        MessageKey::ApplyButton => "Apply".to_string(),
        MessageKey::ResetFilterAriaLabel => {
            "Show this list with no filter or sort applied".to_string()
        }
        MessageKey::ResetLink => "Reset".to_string(),
        MessageKey::UpdatedColumnHeading => "Updated".to_string(),
        MessageKey::EmptyIssueListMessage => "No issues yet.".to_string(),
        MessageKey::EffortEstimateTooltip => "Effort estimate".to_string(),
        MessageKey::ProjectDetailPageTitle { project_name } => {
            format!("{project_name} — Issue Tracker")
        }
        MessageKey::IssueNewPageTitle { project_name } => format!("New issue — {project_name}"),
        MessageKey::NewIssueLabel => "New issue".to_string(),
        MessageKey::NewIssueTitlePlaceholder => "What needs to happen?".to_string(),
        MessageKey::NewIssueDescriptionPlaceholder => {
            "Describe the problem, the steps to reproduce, or the acceptance criteria.".to_string()
        }
        MessageKey::CreateIssueButton => "Create issue".to_string(),
        MessageKey::SubIssueNewPageTitle { parent_title } => {
            format!("New sub-issue — {parent_title}")
        }
        MessageKey::NewSubIssueLabel => "New sub-issue".to_string(),
        MessageKey::SubIssueNewPageIntro => "This sub-issue follows its parent's sprint. \
             You can give it its own assignee, status, priority, and effort."
            .to_string(),
        MessageKey::NewSubIssueTitlePlaceholder => {
            "What needs to happen for this part?".to_string()
        }
        MessageKey::NewSubIssueDescriptionPlaceholder => {
            "Describe this sub-task in more detail if useful.".to_string()
        }
        MessageKey::CreateSubIssueButton => "Create sub-issue".to_string(),
        MessageKey::IssueDetailPageTitle {
            issue_title,
            project_name,
        } => format!("{issue_title} — {project_name}"),
        MessageKey::SubIssuesLabel => "Sub-issues".to_string(),
        MessageKey::AddSubIssueLink => "+ Add sub-issue".to_string(),
        MessageKey::SubIssuesEmptyMessage => {
            "No sub-issues yet. Break this work into smaller pieces \
             if it helps you track them — they share this issue's project \
             and sprint, but can have their own assignee, status, and effort."
                .to_string()
        }
        MessageKey::SubIssueAriaLabel { title, status } => {
            format!("{title}, status {}", issue_status_label(status))
        }
        MessageKey::SprintAssignmentLabel => "Sprint assignment".to_string(),
        MessageKey::SprintFieldLabel => "Sprint:".to_string(),
        MessageKey::SprintSelectAriaLabel => "Select sprint for this issue".to_string(),
        MessageKey::NoSprintOption => "(no sprint)".to_string(),
        MessageKey::SprintAssignmentHelperText => {
            "Sprint assignment is independent from this issue's status and priority — \
             adding to a sprint commits the work; the team decides what 'committed' means."
                .to_string()
        }
        MessageKey::IssueStatusAriaLabel => "Issue status".to_string(),
        MessageKey::IssueStatusGroupAriaLabel { issue_title } => {
            format!("Status for \"{issue_title}\"")
        }
        MessageKey::NoDescriptionProvided => "No description provided.".to_string(),
        MessageKey::CreatedAt { formatted } => format!("Created {formatted}"),
        MessageKey::UpdatedAt { formatted } => format!("Updated {formatted}"),
        MessageKey::ProjectsListPageTitle => "Projects — Issue Tracker".to_string(),
        MessageKey::ProjectsSubheading => "Your issue-tracking workspaces".to_string(),
        MessageKey::NewProjectLabel => "New project".to_string(),
        MessageKey::ProjectsEmptyMessage => "No projects yet.".to_string(),
        MessageKey::CreateFirstProjectButton => "Create your first project".to_string(),
        MessageKey::NoDescriptionShort => "No description".to_string(),
        MessageKey::ProjectNewPageTitle => "New project — Issue Tracker".to_string(),
        MessageKey::NewBreadcrumbWord => "New".to_string(),
        MessageKey::ProjectNamePlaceholder => "e.g. Customer Portal".to_string(),
        MessageKey::ProjectDescriptionPlaceholder => "What is this project about?".to_string(),
        MessageKey::TeamFieldLabel => "Team".to_string(),
        MessageKey::OptionalHint => "optional".to_string(),
        MessageKey::PersonalNoTeamOption => "Personal (no team)".to_string(),
        MessageKey::TeamHelperText => "If you choose a team, members of that team can see and \
             contribute to this project per their team role."
            .to_string(),
        MessageKey::CreateProjectButton => "Create project".to_string(),
        MessageKey::ProjectEditPageTitle { project_name } => {
            format!("Edit {project_name} — Issue Tracker")
        }
        MessageKey::EditProjectHeading => "Edit project".to_string(),
        MessageKey::DeleteProjectHeading => "Delete project".to_string(),
        MessageKey::DeleteProjectWarning => {
            "Permanently remove this project and all its issues.".to_string()
        }
        MessageKey::IssueDeletedFlash => "Issue deleted".to_string(),
        MessageKey::ProjectDeletedFlash => "Project deleted".to_string(),

        // ---- I18N-005c: components/{sprints,teams} ----
        MessageKey::SprintStatusName { label } => sprint_status_label(label).to_string(),
        MessageKey::TeamRoleName { label } => team_role_label(label).to_string(),
        MessageKey::NewSprintLink => "+ New sprint".to_string(),
        MessageKey::SprintsPageTitle { team_name } => format!("Sprints — {team_name}"),
        MessageKey::SprintsSectionName => "Sprints".to_string(),
        MessageKey::SprintsListAriaLabel => "Sprint list".to_string(),
        MessageKey::SprintCardSummaryCompleted {
            completed_points,
            committed_points,
            carried_over_points,
        } => format!(
            "{completed_points} of {committed_points} pt completed · \
             {carried_over_points} carried over"
        ),
        MessageKey::SprintCardSummaryActive {
            completed_points,
            committed_points,
            in_flight_points,
        } => format!(
            "{completed_points} of {committed_points} pt completed · \
             {in_flight_points} pt in flight"
        ),
        MessageKey::SprintCardSummaryPlanned {
            committed_points,
            committed_count,
        } => format!("{committed_points} pt committed across {committed_count} issues"),
        MessageKey::SprintCardAriaLabel {
            name,
            status,
            dates,
            completed_points,
            committed_points,
            carried_over_points,
            committed_count,
        } => {
            let status_text = sprint_status_label(status);
            let summary_text = sprint_card_summary_text(
                status,
                completed_points,
                committed_points,
                carried_over_points,
                committed_count,
            );
            format!("{name} ({status_text}, {dates}). {summary_text}")
        }
        MessageKey::VelocityBarAriaLabel {
            name,
            completed_points,
            carried_over_points,
        } => format!(
            "{name}: {completed_points} pt completed, {carried_over_points} pt carried over"
        ),
        MessageKey::SprintsEmptyMessageAdmin => {
            "No sprints yet. Create one to start time-boxing your team's work.".to_string()
        }
        MessageKey::SprintsEmptyMessageNonAdmin => {
            "No sprints yet. An admin can create one when the team is ready to time-box work."
                .to_string()
        }
        MessageKey::SprintsOptionalNote => {
            "Sprints are optional — you can use peisear without them.".to_string()
        }
        MessageKey::CompletedWorkHeading => "Completed work this period".to_string(),
        MessageKey::RecentCompletedSprintsAriaLabel => "Recent completed sprints".to_string(),
        MessageKey::VelocityCaptionLead => "Each pair of bars: ".to_string(),
        MessageKey::CaptionWordCompleted => "completed".to_string(),
        MessageKey::VelocityCaptionMiddle => " (filled) and ".to_string(),
        MessageKey::CaptionWordCarriedOver => "carried over".to_string(),
        MessageKey::VelocityCaptionTail => {
            " (light). The dotted line is the median completed across these sprints. \
             Numbers describe what happened — they don't grade it."
                .to_string()
        }
        MessageKey::BarChartAriaLabel => "Bar chart of recent sprint outcomes".to_string(),
        MessageKey::MedianLabel { median } => format!("median {median}"),
        MessageKey::NewSprintLabel => "New sprint".to_string(),
        MessageKey::SprintNamePlaceholder => "e.g. Sprint 5".to_string(),
        MessageKey::GoalFieldPlaceholder => "What's this sprint trying to achieve?".to_string(),
        MessageKey::SprintPlannedNoticeLead => "The sprint will be created in ".to_string(),
        MessageKey::CaptionWordPlanned => "planned".to_string(),
        MessageKey::SprintPlannedNoticeTail => {
            " state. Start it explicitly when you're ready — the calendar \
             dates don't auto-promote."
                .to_string()
        }
        MessageKey::CreateSprintButton => "Create sprint".to_string(),
        MessageKey::StartSprintLabel => "Start sprint".to_string(),
        MessageKey::CompleteSprintLabel => "Complete sprint".to_string(),
        MessageKey::GoalFieldPrefixLabel => "Goal: ".to_string(),
        MessageKey::SummaryHeading => "Summary".to_string(),
        MessageKey::CommittedStatLabel => "Committed".to_string(),
        MessageKey::CompletedStatLabel => "Completed".to_string(),
        MessageKey::InFlightStatLabel => "In flight".to_string(),
        MessageKey::CarriedOverHeading => "Carried over".to_string(),
        MessageKey::PointsUnitSuffix => "pt".to_string(),
        MessageKey::IssuesCountText { count } => format!("{count} issues"),
        MessageKey::BurndownHeading => "Burndown".to_string(),
        MessageKey::BurndownSectionAriaLabel => "Sprint burndown".to_string(),
        MessageKey::BurndownCaptionLead => "Two cumulative lines: ".to_string(),
        MessageKey::CaptionWordCommitted => "committed".to_string(),
        MessageKey::BurndownCaptionMiddle => " (the work added to the sprint) and ".to_string(),
        MessageKey::BurndownCaptionTail => " (work finished). The gap between them is in flight. \
             No ideal line, no prediction — what's happening is what you see."
            .to_string(),
        MessageKey::BurndownChartAriaLabel {
            first_label,
            last_label,
            max_val,
        } => format!("Burndown chart from {first_label} to {last_label}, max value {max_val}"),
        MessageKey::IssuesInSprintAriaLabel => "Issues in sprint".to_string(),
        MessageKey::IssuesHeading => "Issues".to_string(),
        MessageKey::NoIssuesInSprintMessage => {
            "No issues in this sprint yet. Open an issue and select this sprint \
             from the sprint dropdown to add it."
                .to_string()
        }
        MessageKey::SprintIssuesAriaLabel => "Sprint issues".to_string(),
        MessageKey::EditSprintPageTitle { sprint_name } => format!("Edit {sprint_name}"),
        MessageKey::EditSprintHeading => "Edit sprint".to_string(),
        MessageKey::SprintPlanPageTitle { sprint_name } => format!("Sprint Plan — {sprint_name}"),
        MessageKey::SprintPlanBreadcrumbWord => "Plan".to_string(),
        MessageKey::BacklogHeading => "Backlog".to_string(),
        MessageKey::SprintItemsHeading => "Sprint Items".to_string(),
        MessageKey::BacklogFilterAriaLabel => "Filter backlog".to_string(),
        MessageKey::AllPrioritiesOption => "All priorities".to_string(),
        MessageKey::AllProjectsOption => "All projects".to_string(),
        MessageKey::MoveToSprintButton => "→ Sprint".to_string(),
        MessageKey::MoveToBacklogButton => "← Backlog".to_string(),
        MessageKey::BacklogRowAriaLabel { title, points } => {
            format!("{title}, {points} pt, in Backlog")
        }
        MessageKey::SprintItemRowAriaLabel { title, points } => {
            format!("{title}, {points} pt, in Sprint Items")
        }
        MessageKey::CommittedTotalLabel { committed_points } => {
            format!("committed: {committed_points} pt")
        }
        MessageKey::NoBacklogIssuesMessage => {
            "No backlog issues match the current filters.".to_string()
        }
        MessageKey::NoSprintItemsInPlanMessage => {
            "No issues in this sprint yet. Move some from the backlog.".to_string()
        }
        MessageKey::SprintPlanNotEditableMessage => {
            "This sprint's plan can no longer be edited.".to_string()
        }
        MessageKey::NewTeamLink => "+ New team".to_string(),
        MessageKey::TeamsEmptyIntro => {
            "Teams group people who collaborate on projects. You can keep working \
             with personal projects without joining a team — teams are optional."
                .to_string()
        }
        MessageKey::TeamsEmptyCta => {
            "Create one if a project will involve more than just you.".to_string()
        }
        MessageKey::YourTeamsAriaLabel => "Your teams".to_string(),
        MessageKey::TeamRoleAriaLabel { team_name, role } => {
            format!("{team_name} (role: {})", team_role_label(role))
        }
        MessageKey::NewTeamLabel => "New team".to_string(),
        MessageKey::TeamNamePlaceholder => "e.g. Frontend Engineering".to_string(),
        MessageKey::SlugFieldLabel => "URL slug".to_string(),
        MessageKey::OptionalAutoDerivedHint => "optional — auto-derived".to_string(),
        MessageKey::SlugPlaceholder => "e.g. frontend-eng".to_string(),
        MessageKey::SlugHelperText => {
            "Lowercase letters, digits, and hyphens. Used in the team's URL.".to_string()
        }
        MessageKey::TeamDescriptionPlaceholder => "What does this team work on?".to_string(),
        MessageKey::NewTeamIntro => {
            "You'll be added as the team's admin. You can invite others by email \
             after the team is created."
                .to_string()
        }
        MessageKey::CreateTeamButton => "Create team".to_string(),
        MessageKey::EditTeamSettingsAriaLabel => "Edit team settings".to_string(),
        MessageKey::InviteMemberSummary => "Invite a member".to_string(),
        MessageKey::ByEmailHint => "by email".to_string(),
        MessageKey::EmailPlaceholderExample => "alice@example.com".to_string(),
        MessageKey::AddButton => "Add".to_string(),
        MessageKey::InviteHelperText => {
            "The user must have a peisear account already (registration via email \
             is not yet automatic from the invite — that's a Phase 2 improvement)."
                .to_string()
        }
        MessageKey::MembersHeading => "Members".to_string(),
        MessageKey::TeamMembersAriaLabel => "Team members".to_string(),
        MessageKey::JoinedColumnHeading => "Joined".to_string(),
        MessageKey::TeamPrivacyFootnote => {
            "Privacy note: project trends and workload distribution are visible \
             to all team members. Personal sustainability data (your burnout panel, \
             your dashboard) remains visible to you only — admin role is a \
             management role, not an oversight role."
                .to_string()
        }
        MessageKey::DetachFromTeamAriaLabel => "Detach from team".to_string(),
        MessageKey::DetachButton => "Detach".to_string(),
        MessageKey::TeamProjectsAriaLabel => "Team projects".to_string(),
        MessageKey::NoProjectsInTeamMessage => {
            "No projects yet. Create one and assign it to this team from the \
             new-project form, or move an existing personal project here from \
             its settings."
                .to_string()
        }
        MessageKey::ChangeRoleAriaLabel => "Change role".to_string(),
        MessageKey::LeaveTeamAriaLabel => "Leave team".to_string(),
        MessageKey::LeaveButton => "Leave".to_string(),
        MessageKey::RemoveMemberAriaLabel => "Remove member".to_string(),
        MessageKey::RemoveButton => "Remove".to_string(),
        MessageKey::YouSuffix => "(you)".to_string(),
        MessageKey::EditTeamPageTitle { team_name } => format!("Edit {team_name}"),
        MessageKey::TeamSettingsHeading => "Team settings".to_string(),
        MessageKey::SlugFixedNotice => "Slug (URL identifier) is fixed at create time.".to_string(),
        MessageKey::SprintCreatedFlash => "Sprint created".to_string(),
        MessageKey::SprintUpdatedFlash => "Sprint updated".to_string(),
        MessageKey::SprintStartedFlash => "Sprint started".to_string(),
        MessageKey::SprintCompletedFlash => "Sprint completed".to_string(),
        MessageKey::SprintDeletedFlash => "Sprint deleted".to_string(),
        MessageKey::SprintAssignmentSavedFlash => "Sprint assignment saved".to_string(),
        MessageKey::TeamCreatedFlash => "Team created".to_string(),
        MessageKey::MemberAddedFlash => "Member added".to_string(),
        MessageKey::RoleUpdatedFlash => "Role updated".to_string(),
        MessageKey::LastAdminDemotionError => {
            "This is the last admin of the team — promote another member to admin \
             first, then change this role."
                .to_string()
        }
        MessageKey::LastAdminRemovalError => {
            "This is the last admin of the team — assign another admin before \
             removing this one."
                .to_string()
        }
        MessageKey::YouLeftTeamFlash => "You left the team".to_string(),
        MessageKey::MemberRemovedFlash => "Member removed".to_string(),
        MessageKey::TeamUpdatedFlash => "Team updated".to_string(),
        MessageKey::ProjectDetachedFlash => "Project detached".to_string(),
        MessageKey::NoUserWithEmailFound { email } => {
            format!("No user with email '{email}' was found.")
        }

        // ---- I18N-005d: components/me ----
        MessageKey::PaceValue { days_per_point } => format!("≈ {days_per_point:.1} d / pt"),
        MessageKey::ReadFirstOverloadTitle => "You've been over capacity for a while.".to_string(),
        MessageKey::ReadFirstOverloadBody {
            overload_streak_days,
            window_days,
        } => format!(
            "Over capacity for {overload_streak_days} of the last {window_days} snapshots. \
             A short break or a backlog re-prioritisation often helps here."
        ),
        MessageKey::ReadFirstStalledTitle => {
            "An assigned issue hasn't moved in a while.".to_string()
        }
        MessageKey::ReadFirstStalledBody {
            stalled_assigned_max_days,
        } => format!(
            "Your oldest in-flight assigned issue has been open for \
             {stalled_assigned_max_days} days. Worth a look — it may be blocked, or worth \
             re-scoping."
        ),
        MessageKey::ReadFirstWipTitle => "WIP is over your limit.".to_string(),
        MessageKey::ReadFirstWipBody {
            current_wip,
            effective_wip_limit,
        } => format!(
            "You have {current_wip} issues in progress; your effective limit is \
             {effective_wip_limit}. Pushing one to Done before starting more keeps focus crisp."
        ),
        MessageKey::ReadFirstLongStaleTitle => {
            "Some long-stale issues are still assigned to you.".to_string()
        }
        MessageKey::ReadFirstLongStaleBody { long_stale_count } => {
            let plural = if long_stale_count == 1 {
                "issue"
            } else {
                "issues"
            };
            format!(
                "{long_stale_count} {plural} haven't been touched in over two weeks. \
                 Closing or re-assigning them clears your queue."
            )
        }
        MessageKey::PersonalDashboardTitle => "My dashboard".to_string(),
        MessageKey::NothingToShowMessage => "Nothing to show yet.".to_string(),
        MessageKey::PersonalDashboardSubtitle { display_name } => {
            format!("Personal metrics for {display_name}. Visible only to you.")
        }
        MessageKey::ReadFirstAriaLabel => "What to read first".to_string(),
        MessageKey::RightNowHeading => "Right now".to_string(),
        MessageKey::WipChipLabel => "WIP".to_string(),
        MessageKey::LoadChipLabel => "Load".to_string(),
        MessageKey::LoadChipTooltip => "Sum of effort across your in-flight issues".to_string(),
        MessageKey::PeriodHintTooltip => {
            "Your capacity for today comes from a row with a period; check Settings.".to_string()
        }
        MessageKey::ThisPeriodHint => "(this period)".to_string(),
        MessageKey::RhythmAriaLabel => {
            "Rhythm — open to see throughput, long-stale count, and pace".to_string()
        }
        MessageKey::RhythmSummaryLabel => "Rhythm".to_string(),
        MessageKey::ThroughputTooltip => "Issues you have moved to Done".to_string(),
        MessageKey::ThroughputChipLabel => "Throughput".to_string(),
        MessageKey::LongStaleChipLabel => "Long-stale".to_string(),
        MessageKey::PaceTooltip => {
            "Coarse calendar-time-per-point on recent done issues. Phase 1 approximation; \
             do not over-interpret."
                .to_string()
        }
        MessageKey::PaceChipLabel => "Pace".to_string(),
        MessageKey::WhatDoTheseMeanLabel => "What do these mean?".to_string(),
        MessageKey::WipGlossaryDefinition => {
            " — issues currently In Progress assigned to you, vs. your effective WIP limit. \
             Limit comes from your personal setting, the project default, or the system \
             default of 3."
                .to_string()
        }
        MessageKey::LoadGlossaryDefinition => {
            " — sum of effort points across your in-flight (Open or In Progress) issues, \
             vs. your capacity if you've set one."
                .to_string()
        }
        MessageKey::ThroughputGlossaryDefinition { window_days } => {
            format!(" — issues you have moved to Done in the last {window_days} days.")
        }
        MessageKey::LongStaleGlossaryDefinition { window_days } => format!(
            " — in-flight issues assigned to you that have not been touched in over \
             {window_days} days."
        ),
        MessageKey::PaceGlossaryDefinition => {
            " — active in-progress time per story point on your recently-completed \
             estimated issues, reconstructed from the issue event log. Treat as a \
             self-reflection prompt rather than a measurement; the number reflects how \
             recent issues actually went, not how future ones will."
                .to_string()
        }
        MessageKey::SustainabilityHeading => "Sustainability".to_string(),
        MessageKey::SustainabilityGlossaryDefinition => {
            " — a couple of streak-style signals based on the periodic snapshots taken in \
             the background: how many consecutive snapshots you have been over capacity, \
             and how long your oldest assigned issue has been without a status change. The \
             panel is muted by default and only opens itself when something is worth a \
             glance. Visible to you only."
                .to_string()
        }
        MessageKey::PatternsSubheading => "Patterns".to_string(),
        MessageKey::PatternsGlossaryDefinition => {
            " — descriptive numbers about your recent rhythm: whether your \
             dwell-time-per-point is drifting, and how often you switch to in_progress on \
             a typical active day. These are facts about how the last few weeks went, not \
             judgements. They sit inside the Sustainability panel and have no warning \
             palette of their own. Visible to you only."
                .to_string()
        }
        MessageKey::OverloadStreakChipLabel => "Over-capacity streak".to_string(),
        MessageKey::OldestStalledChipLabel => "Oldest stalled".to_string(),
        MessageKey::PatternsDisclaimer => {
            "These are descriptions of recent rhythm, not evaluations. Many patterns have \
             legitimate reasons behind them."
                .to_string()
        }
        MessageKey::SustainabilityPrivacyNote => {
            "These signals are visible to you only. They are not used for evaluation; \
             they exist so you can pace yourself."
                .to_string()
        }
        MessageKey::OverloadStreakValue {
            overload_streak_days,
            window_days,
        } => format!("{overload_streak_days} of last {window_days}"),
        MessageKey::StalledDaysValue {
            stalled_assigned_max_days,
        } => format!("{stalled_assigned_max_days} d"),
        MessageKey::OverloadStreakAriaLabel {
            overload_streak_days,
            is_watch,
        } => format!(
            "Overload streak: {overload_streak_days} consecutive snapshots over capacity \
             ({}).",
            if is_watch { "watch" } else { "steady" }
        ),
        MessageKey::StalledAriaLabel {
            stalled_assigned_max_days,
            is_watch,
        } => format!(
            "Oldest stalled assigned issue: {stalled_assigned_max_days} days ({}).",
            if is_watch { "watch" } else { "steady" }
        ),
        MessageKey::DriftInsufficientDataAriaLabel => {
            "Estimation drift: not enough completed estimated work in the last 28 days \
             to compare halves of the window."
                .to_string()
        }
        MessageKey::PaceDriftChipLabel => "Pace drift".to_string(),
        MessageKey::NeedMoreDataLabel => "need more data".to_string(),
        MessageKey::DriftDirectionWord { direction } => drift_direction_word(direction).to_string(),
        MessageKey::DriftValueLine {
            recent_median_days_per_point,
            older_median_days_per_point,
        } => format!(
            "recent {recent_median_days_per_point:.2} vs. {older_median_days_per_point:.2} \
             d / pt"
        ),
        MessageKey::DriftAriaLabel {
            recent_median_days_per_point,
            older_median_days_per_point,
            window_days,
            direction,
        } => format!(
            "Estimation drift: recent {recent_median_days_per_point:.2} d/pt vs. older \
             {older_median_days_per_point:.2} d/pt over the last {window_days} days ({}).",
            drift_trend_phrase(direction)
        ),
        MessageKey::SwitchingInsufficientDataAriaLabel => {
            "Switching pattern: not enough events in the last 14 days to characterise a \
             rhythm."
                .to_string()
        }
        MessageKey::SwitchingChipLabel => "Switching".to_string(),
        MessageKey::SwitchingMedianValue { median } => switching_median_text(median),
        MessageKey::SwitchingSampleLine {
            total_events_observed,
            window_days,
        } => format!("{total_events_observed} events over {window_days} d"),
        MessageKey::SwitchingAriaLabel {
            median,
            total_events_observed,
            window_days,
        } => format!(
            "Switching pattern: median {} pickups per active day ({total_events_observed} \
             total events over {window_days} days). For context only — high or low here \
             is not a quality judgement.",
            switching_median_number(median)
        ),

        // ---- I18N-005d: components/settings ----
        MessageKey::WipLimitExplanation { default_wip_limit } => format!(
            "How many issues you want to have In Progress at once. This is about \
             cognitive load — a small number of actively-worked issues, distinct from \
             the points-budget above. Leave blank to use the project default (or \
             {default_wip_limit})."
        ),
        MessageKey::NoCapacitySetTodayLabel => "no capacity set for today".to_string(),
        MessageKey::ConflictLabel => "Conflict: ".to_string(),
        MessageKey::CapacityOverlapGuidanceLead => {
            "Close the conflicting row first (use the ".to_string()
        }
        MessageKey::CloseOnDateActionWord => "Close on date".to_string(),
        MessageKey::CapacityOverlapGuidanceTail => {
            " action), or adjust the new period so it doesn't overlap.".to_string()
        }
        MessageKey::SettingsSectionName => "Settings".to_string(),
        MessageKey::SettingsSubtitle { display_name } => {
            format!("Personal preferences for {display_name}.")
        }
        MessageKey::CapacitySectionAriaLabel => "Capacity".to_string(),
        MessageKey::WorkloadCapacityHeading => "Workload capacity".to_string(),
        MessageKey::CapacityExplanationParagraph => {
            "Capacity rows describe how many story points you can comfortably carry, \
             optionally bounded by a period. The row whose period covers today is your \
             effective capacity. Periods may not overlap; leave both bounds blank for an \
             open-ended default."
                .to_string()
        }
        MessageKey::EffectiveCapacityTodayAriaLabel { points } => match points {
            Some(p) => format!("Effective capacity today: {p} pt"),
            None => "Effective capacity today: no capacity set for today".to_string(),
        },
        MessageKey::EffectiveTodayLabel => "Effective today: ".to_string(),
        MessageKey::CapacityRowsTableAriaLabel => "Capacity rows".to_string(),
        MessageKey::PointsColumnHeading => "Points".to_string(),
        MessageKey::FromColumnHeading => "From".to_string(),
        MessageKey::FromDateFieldLabel => "From (YYYY-MM-DD)".to_string(),
        MessageKey::ToColumnHeading => "To".to_string(),
        MessageKey::ToDateFieldLabel => "To (YYYY-MM-DD)".to_string(),
        MessageKey::NoteColumnHeading => "Note".to_string(),
        MessageKey::ActionsColumnHeading => "Actions".to_string(),
        MessageKey::AddCapacityRowSummary => "Add a capacity row".to_string(),
        MessageKey::AddCapacityRowFormAriaLabel => "Add capacity row".to_string(),
        MessageKey::PointsPlaceholderExample => "e.g. 10".to_string(),
        MessageKey::NoteFieldPlaceholder => "optional context".to_string(),
        MessageKey::AddRowButton => "Add row".to_string(),
        MessageKey::CapacityOverlapHelperText => {
            "Both date fields are optional. Leave blank to mean \"from the dawn of \
             time\" (start) or \"until further notice\" (end). Adding a row that \
             overlaps an existing one will fail; close the existing row first."
                .to_string()
        }
        MessageKey::WipLimitLabel => "WIP limit".to_string(),
        MessageKey::InProgressIssuesHint => "in-progress issues".to_string(),
        MessageKey::CapacityRowAriaLabel { points, from, to } => {
            format!("Capacity {points} points, period {from} to {to}.")
        }
        MessageKey::CloseOnDateSummary => "Close on date…".to_string(),
        MessageKey::CloseThisRowAriaLabel => "Close this row on a specific date".to_string(),
        MessageKey::CloseOnLabel => "Close on".to_string(),
        MessageKey::CloseButton => "Close".to_string(),
        MessageKey::EditRowAriaLabel => "Edit row".to_string(),
        MessageKey::RemoveThisRowAriaLabel => "Remove this row".to_string(),

        // ---- I18N-005d: components/{notification_preferences,notifications} ----
        MessageKey::EmailNotificationsHeading => "Email notifications".to_string(),
        MessageKey::FirstTimeEmailPromptAriaLabel => "First-time email setup prompt".to_string(),
        MessageKey::EmailOptInPromptBody => {
            "Would you like notifications by email as well as in-app? You can change \
             this any time."
                .to_string()
        }
        MessageKey::EmailOptInYesButton => "Yes, send me email".to_string(),
        MessageKey::EmailOptInNoButton => "Just in-app, thanks".to_string(),
        MessageKey::EmailOptInOnStatus => "✓ Email opt-in is on by default.".to_string(),
        MessageKey::EmailOptInOffStatus => {
            "Email opt-in is off (in-app only by default). Per-kind overrides below.".to_string()
        }
        MessageKey::NotificationPreferencesPageTitle => "Notification preferences".to_string(),
        MessageKey::NotificationsSectionName => "Notifications".to_string(),
        MessageKey::SilenceAllAriaLabel => "Silence all notification kinds".to_string(),
        MessageKey::SilenceAllButton => "Silence all".to_string(),
        MessageKey::DefaultsInAppLead => "Defaults: in-app delivery on for all kinds. ".to_string(),
        MessageKey::PerKindDeliverySummary => "Per-kind delivery".to_string(),
        MessageKey::ClickToExpandHint => "Click to expand".to_string(),
        MessageKey::NotificationKindsTableAriaLabel => "Notification kinds".to_string(),
        MessageKey::KindColumnHeading => "Kind".to_string(),
        MessageKey::MinSeverityColumnHeading => "Min severity".to_string(),
        MessageKey::ChannelStubDisclaimer => {
            "Email and webhook are stubs in this release — they log the dispatch \
             attempt but don't yet send. The channel structure is ready for the \
             upcoming wasm-smtp integration."
                .to_string()
        }
        MessageKey::SavePreferencesButton => "Save preferences".to_string(),
        MessageKey::NotificationKindPreferencesAriaLabel { kind } => {
            format!("{} preferences", notification_kind_label(kind))
        }
        MessageKey::InAppForKindAriaLabel { kind } => {
            format!("In-app for {}", notification_kind_label(kind))
        }
        MessageKey::EmailForKindAriaLabel { kind } => {
            format!("Email for {}", notification_kind_label(kind))
        }
        MessageKey::WebhookForKindAriaLabel { kind } => {
            format!("Webhook for {}", notification_kind_label(kind))
        }
        MessageKey::MinSeverityForKindAriaLabel { kind } => {
            format!("Minimum severity for {}", notification_kind_label(kind))
        }
        MessageKey::AllSeverityOption => "All".to_string(),
        MessageKey::WatchOnlySeverityOption => "Watch only".to_string(),
        MessageKey::NotificationKindName { kind } => notification_kind_label(kind).to_string(),
        MessageKey::NotificationChannelName { channel } => {
            notification_channel_label(channel).to_string()
        }
        MessageKey::NoNotificationsYetStatus => "No notifications yet.".to_string(),
        MessageKey::UnreadOfTotalStatus {
            unread_count,
            total,
        } => {
            format!("{unread_count} unread of {total}.")
        }
        MessageKey::AllReadStatus { total } => format!("All read. {total} total."),
        MessageKey::MarkAllReadAriaLabel => "Mark all notifications as read".to_string(),
        MessageKey::MarkAllReadButton => "Mark all read".to_string(),
        MessageKey::InboxEmptyMessage => {
            "You'll see notifications here when something needs a glance — warnings \
             about your workload, project health changes, that sort of thing."
                .to_string()
        }
        MessageKey::InboxEmptyFooterLead => "Configure delivery in ".to_string(),
        MessageKey::SettingsLinkWord => "settings".to_string(),
        MessageKey::InboxEmptyFooterTail => ".".to_string(),
        MessageKey::NotificationListAriaLabel => "Notification list".to_string(),
        MessageKey::UnreadWord => "Unread".to_string(),
        MessageKey::ReadWord => "Read".to_string(),
        MessageKey::NotificationRowAriaLabel {
            is_unread,
            title,
            kind,
            timestamp,
        } => format!(
            "{} notification: {title} ({}, {timestamp}).",
            if is_unread { "Unread" } else { "Read" },
            notification_kind_label(kind)
        ),
        MessageKey::SentViaPrefix => "Sent via ".to_string(),
        MessageKey::ViewContextLinkLabel => "View context →".to_string(),
        MessageKey::MarkAsReadAriaLabel => "Mark as read".to_string(),
        MessageKey::MarkReadButton => "Mark read".to_string(),

        // ---- INBOX-001: silence-resume banner ----
        MessageKey::SilenceResumeBannerAriaLabel => "Notifications silenced".to_string(),
        MessageKey::SilenceResumeBannerMessage => {
            "You've silenced all notification kinds. Resume to receive them again.".to_string()
        }
        MessageKey::ResumeNotificationsAriaLabel => "Resume all notification kinds".to_string(),
        MessageKey::ResumeNotificationsButton => "Resume notifications".to_string(),

        // ---- I18N-005d: components/search ----
        MessageKey::SearchWord => "Search".to_string(),
        MessageKey::SearchPageTitleWithQuery { q } => format!("Search: {q}"),
        MessageKey::SearchFieldLabel => "Search projects and open issues".to_string(),
        MessageKey::SearchPlaceholder => "Type to search...".to_string(),
        MessageKey::ResultsForHeadingPrefix => "Results for ".to_string(),
        MessageKey::NoQueryGuidanceMessage => {
            "Enter a search term above to find projects and open issues.".to_string()
        }
        MessageKey::OpenIssuesSectionName => "Open issues".to_string(),
        MessageKey::NoMatchesInCategoryMessage => "No matches in this category.".to_string(),
        MessageKey::PreviousPageLink => "← Previous".to_string(),
        MessageKey::NextPageLink => "Next →".to_string(),
        MessageKey::ProjectHitTypeLabel => "Project".to_string(),
        MessageKey::OpenIssueHitTypePrefix { project_name } => {
            format!("Open issue · {project_name}")
        }
        MessageKey::SubIssueHitTypePrefix {
            project_name,
            parent_title,
        } => format!("Open issue · {project_name} / {parent_title}"),

        // ---- I18N-005d: handlers/{settings,notification_preferences,notifications} ----
        MessageKey::WipLimitSavedFlash => "WIP limit saved".to_string(),
        MessageKey::CapacityRowAddedFlash => "Capacity row added".to_string(),
        MessageKey::CapacityRowUpdatedFlash => "Capacity row updated".to_string(),
        MessageKey::CapacityRowRemovedFlash => "Capacity row removed".to_string(),
        MessageKey::RowClosedFlash => "Row closed".to_string(),
        MessageKey::PreferencesSavedFlash => "Preferences saved".to_string(),
        MessageKey::AllNotificationsSilencedFlash => "All notifications silenced".to_string(),
        MessageKey::MarkedAsReadFlash { count } => format!("Marked {count} as read"),

        // ---- I18N-005e: error.rs (ApiAppError) ----
        MessageKey::ApiUnauthorizedMessage => "Authentication required.".to_string(),
        MessageKey::ApiForbiddenMessage => {
            "You do not have permission to access this resource.".to_string()
        }
        MessageKey::ApiNotFoundMessage => "Resource not found.".to_string(),
        MessageKey::ApiOptimisticLockConflictMessage { entity } => format!(
            "Someone else updated this {} while you were editing. Reload and re-apply your change.",
            entity_label(entity)
        ),

        // ---- I18N-005e: components/auth.rs, handlers/auth.rs ----
        MessageKey::LoginPageTitle => "Sign in — Issue Tracker".to_string(),
        MessageKey::RegisterPageTitle => "Create account — Issue Tracker".to_string(),
        MessageKey::SignInTaglineText => "Sign in to your workspace".to_string(),
        MessageKey::RegisterTaglineText => "Create your account".to_string(),
        MessageKey::SignInWord => "Sign in".to_string(),
        MessageKey::CreateAccountButton => "Create account".to_string(),
        MessageKey::PasswordFieldLabel => "Password".to_string(),
        MessageKey::DisplayNameFieldLabel => "Display name".to_string(),
        MessageKey::PasswordMinLengthHint => "8+ characters".to_string(),
        MessageKey::NoAccountPrompt => "No account? ".to_string(),
        MessageKey::CreateOneLinkWord => "Create one".to_string(),
        MessageKey::AlreadyHaveAccountPrompt => "Already have an account? ".to_string(),
        MessageKey::InvalidCredentialsMessage => "Invalid email or password.".to_string(),
        MessageKey::EmailAlreadyExistsMessage => {
            "An account with this email already exists.".to_string()
        }
        MessageKey::InvalidInputFallbackMessage => "Invalid input.".to_string(),

        // ---- I18N-005e: handlers/issues.rs ----
        MessageKey::InvalidAssigneeMessage => {
            "Selected user is not a valid assignee for this project.".to_string()
        }
        MessageKey::SubIssueCannotNestLongMessage => {
            "Sub-issues cannot have their own sub-issues. Promote the parent to a top-level \
             issue first, or add this work as a sibling sub-issue under the same parent."
                .to_string()
        }

        // ---- I18N-005e: handlers/sprints.rs ----
        MessageKey::SprintNameRequiredMessage => "Sprint name is required.".to_string(),
        MessageKey::SubIssueFollowsParentSprintMessage => {
            "Sub-issues follow the parent's sprint. Change the parent's sprint instead.".to_string()
        }
        MessageKey::SprintsPersonalProjectMessage => {
            "Sprints are a team feature; this is a personal project.".to_string()
        }
        MessageKey::SprintProjectTeamMismatchMessage => {
            "Sprint and project must belong to the same team.".to_string()
        }
        MessageKey::CannotAssignToCompletedSprintMessage => {
            "Cannot assign issues to a completed sprint.".to_string()
        }

        // ---- I18N-005e: handlers/teams.rs ----
        MessageKey::TeamNameRequiredMessage => "Team name is required.".to_string(),
        MessageKey::SlugDerivationFailedMessage => {
            "Could not derive a URL slug from the name. Try setting one explicitly \
             (lowercase letters, digits, hyphens)."
                .to_string()
        }
        MessageKey::InvalidRoleMessage => "Invalid role.".to_string(),

        // ---- I18N-005e: handlers/settings.rs ----
        MessageKey::CapacityPointsRequiredMessage => "Capacity points are required.".to_string(),
        MessageKey::WipLimitMustBePositiveIntegerMessage => {
            "WIP limit must be a positive integer.".to_string()
        }
        MessageKey::PeriodStartMustBeDateFormatMessage => {
            "Period start must be in YYYY-MM-DD format.".to_string()
        }
        MessageKey::PeriodEndMustBeDateFormatMessage => {
            "Period end must be in YYYY-MM-DD format.".to_string()
        }

        // ---- I18N-006: peisear-core/src/lib.rs ----
        MessageKey::IndicatorDescription { label } => indicator_description(label).to_string(),
        MessageKey::WipAriaLabel {
            current_wip,
            effective_wip_limit,
            state,
        } => format!(
            "WIP: {current_wip} of {effective_wip_limit} ({}).",
            health_state_label(state)
        ),
        MessageKey::LongStaleAriaLabel {
            long_stale_count,
            state,
        } => format!(
            "Long-stale assigned issues: {long_stale_count} ({}).",
            health_state_label(state)
        ),
        MessageKey::CompositeAriaLabel { state } => {
            format!("Composite: {}.", health_state_label(state))
        }
        MessageKey::IndicatorAriaLabel {
            label,
            value,
            state,
        } => format!(
            "{}: {} ({}). {}",
            indicator_label(label),
            render(*value),
            health_state_label(state),
            indicator_description(label)
        ),

        // ---- I18N-006: peisear-storage/src/user_capacities.rs ----
        MessageKey::PeriodStartMustPrecedeEndMessage => {
            "The From date must be on or before the To date.".to_string()
        }
        MessageKey::CapacityPeriodOverlapMessage {
            row_id,
            period_start,
            period_end,
            points,
        } => format!(
            "row {row_id} ({period_start} to {period_end}, {points} pt) overlaps the proposed period"
        ),

        // ---- I18N-006: peisear-storage/src/sprints.rs ----
        MessageKey::SprintEndDateMustBeOnOrAfterStartMessage => {
            "Sprint end date must be on or after start date.".to_string()
        }
        MessageKey::SprintAlreadyActiveMessage => "Sprint is already active.".to_string(),
        MessageKey::SprintCannotRestartCompletedMessage => {
            "Cannot restart a completed sprint.".to_string()
        }
        MessageKey::OtherSprintActiveInTeamMessage { sprint_name } => format!(
            "Another sprint ({sprint_name}) is currently active in this team. Complete \
             it before starting a new one."
        ),
        MessageKey::SprintNotStartedYetMessage => "Sprint hasn't been started yet.".to_string(),
        MessageKey::SprintAlreadyCompletedMessage => "Sprint is already completed.".to_string(),
        MessageKey::SprintActiveCannotBeDeletedMessage => {
            "An active sprint cannot be deleted. Complete it first, then delete it.".to_string()
        }

        // ---- I18N-006: peisear-storage/src/teams.rs ----
        MessageKey::TeamSlugCannotBeEmptyMessage => "Team URL slug cannot be empty.".to_string(),
        MessageKey::TeamSlugAlreadyExistsMessage { slug } => {
            format!("A team with slug '{slug}' already exists.")
        }
        MessageKey::UserAlreadyTeamMemberMessage { user_id } => {
            format!("User {user_id} is already a member of this team.")
        }

        // ---- I18N-006: peisear-storage/src/issues.rs (translate_trigger_error) ----
        MessageKey::SubIssueCannotHaveSubIssueMessage => {
            "sub-issue cannot have a sub-issue".to_string()
        }
        MessageKey::SubIssueMustShareProjectMessage => {
            "sub-issue must share project with its parent".to_string()
        }
        MessageKey::IssueCannotBeOwnParentMessage => {
            "an issue cannot be its own parent".to_string()
        }
        MessageKey::CannotDemoteIssueWithSubIssuesMessage => {
            "cannot demote an issue that has its own sub-issues".to_string()
        }
        MessageKey::IssuePlannedEndBeforeStartMessage => {
            "Planned end date must be on or after planned start date.".to_string()
        }

        // ---- I18N-006: handlers/api_users.rs (BurnoutSignal.label) ----
        MessageKey::OverloadStreakSignalMessage {
            overload_streak_days,
            window_days,
        } => {
            format!("Over capacity for {overload_streak_days} of the last {window_days} snapshots.")
        }
        MessageKey::StalledAssignedSignalMessage {
            stalled_assigned_max_days,
        } => format!(
            "Oldest in-flight assigned issue hasn't moved in {stalled_assigned_max_days} days."
        ),
        MessageKey::EstimationDriftUpSignalMessage => {
            "Recent issues are taking longer per point than older ones.".to_string()
        }
        MessageKey::EstimationDriftDownSignalMessage => {
            "Recent issues are completing faster per point than older ones.".to_string()
        }
        MessageKey::CognitiveSwitchingSignalMessage {
            switches_per_day_median,
        } => format!(
            "Switching between {switches_per_day_median:.1} issues per active day on average."
        ),

        // ---- I18N-007: components/issues.rs (render_trend_chip) ----
        MessageKey::TrendLabelFlat => "flat".to_string(),
        MessageKey::TrendLabel { direction, delta } => match direction {
            TrendDirectionLabel::Up => format!("+{delta}"),
            TrendDirectionLabel::Down => format!("-{delta}"),
        },
        MessageKey::TrendAriaFlat => "trend: roughly flat".to_string(),
        MessageKey::TrendAriaLabel { direction, delta } => match direction {
            TrendDirectionLabel::Up => format!("trend: up by {delta} points"),
            TrendDirectionLabel::Down => format!("trend: down by {delta} points"),
        },

        // ---- I18N-007: components/issues.rs (composite_row) ----
        MessageKey::CompositeLabel => "Composite".to_string(),

        // ---- I18N-007: components/sprints.rs (burndown legend) ----
        MessageKey::BurndownLegendCommitted => "Committed".to_string(),
        MessageKey::BurndownLegendCompleted => "Completed".to_string(),

        // ---- I18N-007: components/me.rs ----
        MessageKey::CurrentLoadSectionLabel => "Current load".to_string(),
        MessageKey::LoadWithCapacityValue {
            in_flight_points,
            capacity_points,
        } => format!("{in_flight_points} / {capacity_points} pt"),
        MessageKey::LoadNoCapacityValue { in_flight_points } => {
            format!("{in_flight_points} pt · no limit")
        }
        MessageKey::RecentThroughputValue {
            recent_done_count,
            window_days,
        } => format!("{recent_done_count} done in last {window_days}d"),
        MessageKey::ProjectCalendarPrivacyFootnote => {
            "Calendar note: this view shows planned issue work for this project. Personal \
             schedules are not aggregated here. Each member's individual calendar is private \
             to that person."
                .to_string()
        }
        MessageKey::PersonalCalendarPrivacyFootnote => "Private to you".to_string(),
        MessageKey::PersonalCalendarPageTitle => "Calendar".to_string(),
        MessageKey::ProjectCalendarPageTitle { project_name } => {
            format!("Calendar — {project_name}")
        }
        MessageKey::CalendarBreadcrumbWord => "Calendar".to_string(),
        MessageKey::CalendarViewName { view } => calendar_view_label(view).to_string(),
        MessageKey::CalendarCellAriaLabel { month, day, count } => {
            format!("{} {day}, {count} issues scheduled", month_name(month))
        }
        MessageKey::CrowdingChipAriaLabel { state } => {
            format!("Crowded day: {}.", health_state_label(state))
        }
        MessageKey::CalendarUtcNote => "Times are shown in UTC.".to_string(),
        MessageKey::SprintBandAriaLabel { sprint_name } => {
            format!("Active sprint: {sprint_name}")
        }
        MessageKey::NoPlannedIssuesMessage => {
            "No issues with a planned date in this range.".to_string()
        }
        MessageKey::CalendarViewSwitcherAriaLabel => "Change calendar view and date".to_string(),
        MessageKey::CalendarMoreIssuesLabel { count } => format!("+{count} more"),

        // ---- CONF-001: the confirmation interstitial ----
        MessageKey::ConfirmDeleteHeading { entity_name } => format!("Delete {entity_name}?"),
        MessageKey::ConfirmDeleteCannotBeUndoneNote => "This cannot be undone.".to_string(),
        MessageKey::ConfirmDeleteProjectCascadeNote => {
            "All its issues will be deleted too. This cannot be undone.".to_string()
        }
        MessageKey::StatusChangedAnnouncement { status } => {
            format!("Moved to {}.", issue_status_label(status))
        }
        MessageKey::UndoButtonLabel => "Undo".to_string(),
        MessageKey::StatusChangeUndoConflictMessage => {
            "Another member changed this issue first. The current status is now shown.".to_string()
        }
        MessageKey::StatusChangeUndoUnavailableMessage => {
            "This change could not be completed. Reload to see the current state.".to_string()
        }
        MessageKey::BoardReloadMessage => {
            "This page is showing an earlier version of the board. Reload to see the current state."
                .to_string()
        }
        MessageKey::BoardConflictMessage => {
            "Another member changed this issue first. The board now shows the current state."
                .to_string()
        }
        MessageKey::BoardUnavailableMessage => {
            "This status change could not be completed. The card has been returned to its previous column."
                .to_string()
        }
        MessageKey::ConfirmDeleteSprintPlannedNote => {
            "Issues currently linked to it will be unlinked.".to_string()
        }
        MessageKey::ConfirmDeleteSprintCompletedNote => {
            "Historical numbers will be lost.".to_string()
        }
    }
}

/// Shared by `IndicatorValueThroughput` and
/// `IndicatorExplanationThroughput` so the two stay byte-identical in
/// how they render the same underlying value — the explanation
/// sentence embeds exactly what the value chip shows, not a
/// re-derived approximation of it.
fn throughput_value(done: i64, total: i64) -> String {
    let pct = (done * 100) / total;
    format!("{done} / {total} ({pct}%)")
}

/// The navbar bell badge's own visible number: `IndicatorValueActivity`'s
/// bare-count shape, plus a display cap so the badge doesn't grow
/// wider than its fixed-size circle for a busy inbox.
fn bell_count(count: i64) -> String {
    if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    }
}

/// The three destinations a back-link can name. Lowercase throughout,
/// consistent with `NavSignOut`'s sentence-case convention ("Sign
/// out", not "Sign Out") — the leading word of the enclosing "Back
/// to " phrase carries the capital, not this word. `I18N-005a-review.md`
/// §2's own evidence for why this needed a table: the two call sites
/// this crate never saw had already drifted to "Projects" and
/// "sprints".
fn nav_section(section: NavSection) -> &'static str {
    match section {
        NavSection::Projects => "projects",
        NavSection::Issues => "issues",
        NavSection::Sprints => "sprints",
    }
}

/// The six indicator explanation sentences `IndicatorKind::description()`
/// used to hardcode before `I18N-006` §4 removed it. Shared by
/// `MessageKey::IndicatorDescription`'s own arm and by
/// `MessageKey::IndicatorAriaLabel`'s composed sentence, same
/// helper-sharing shape as `indicator_label`.
fn indicator_description(label: IndicatorLabel) -> &'static str {
    match label {
        IndicatorLabel::Throughput => "Share of issues that have reached Done.",
        IndicatorLabel::Staleness => "Age of the oldest issue still Open or In Progress.",
        IndicatorLabel::Activity => "Issues created or finished in the last 14 days.",
        IndicatorLabel::BusFactor => "Concentration of in-flight work on a single user.",
        IndicatorLabel::LongStale => "Share of in-flight issues untouched for over two weeks.",
        IndicatorLabel::WipCompliance => "Share of active users currently over their WIP limit.",
    }
}

/// The three `DisplayHealthState::glyph()` accessible-name words,
/// typed since `I18N-006` §3 split them from the (non-language)
/// symbol, which stays in `peisear-core`.
fn health_state_label(state: HealthStateLabel) -> &'static str {
    match state {
        HealthStateLabel::Insufficient => "no data",
        HealthStateLabel::Good => "good",
        HealthStateLabel::Watch => "watch",
    }
}

fn calendar_view_label(view: CalendarViewLabel) -> &'static str {
    match view {
        CalendarViewLabel::Day => "Day",
        CalendarViewLabel::Week => "Week",
        CalendarViewLabel::Month => "Month",
    }
}

/// `CAL-002` §2.2: the calendar cell aria-label's date is rendered
/// here, not interpolated as a pre-formatted string built by the
/// caller — `peisear-i18n` has no `chrono` dependency (`I18N-001`
/// §4.1), so `month`/`day` arrive as plain integers and the
/// month-name mapping (an English-prose decision a future locale
/// would want to own) lives in this crate, not `components/calendar.rs`.
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "?",
    }
}

fn indicator_label(label: IndicatorLabel) -> &'static str {
    match label {
        IndicatorLabel::Throughput => "Throughput",
        IndicatorLabel::Staleness => "Oldest in-flight",
        IndicatorLabel::Activity => "Activity (14d)",
        IndicatorLabel::BusFactor => "Bus factor",
        IndicatorLabel::LongStale => "Long-stale",
        IndicatorLabel::WipCompliance => "WIP compliance",
    }
}

fn entity_label(entity: EntityKind) -> &'static str {
    match entity {
        EntityKind::Issue => "issue",
        EntityKind::Project => "project",
        EntityKind::Sprint => "sprint",
        EntityKind::Team => "team",
        EntityKind::CapacityPeriod => "capacity period",
        EntityKind::TeamMembership => "team membership",
    }
}

fn field_label(field: Field) -> &'static str {
    match field {
        Field::EffortPoints => "Effort",
        Field::CapacityPoints => "Capacity points",
        Field::CloseDate => "Close date",
        Field::Title => "Title",
        Field::Description => "Description",
        Field::Status => "Status",
        Field::Priority => "Priority",
        Field::Assignee => "Assignee",
        Field::Name => "Name",
        Field::StartDate => "Start date",
        Field::EndDate => "End date",
        Field::Goal => "Goal",
        Field::Role => "Role",
        Field::Email => "Email",
        Field::Project => "Project",
        Field::PlannedStartDate => "Planned start date",
        Field::PlannedEndDate => "Planned end date",
    }
}

fn issue_status_label(label: IssueStatusLabel) -> &'static str {
    match label {
        IssueStatusLabel::Open => "Open",
        IssueStatusLabel::InProgress => "In Progress",
        IssueStatusLabel::Done => "Done",
    }
}

fn sprint_status_label(label: SprintStatusLabel) -> &'static str {
    match label {
        SprintStatusLabel::Planned => "Planned",
        SprintStatusLabel::Active => "Active",
        SprintStatusLabel::Completed => "Completed",
    }
}

/// Same three templates as `MessageKey::SprintCardSummary*`
/// (deliberately not shared with those arms — I18N-005c-review §3's
/// point is that `SprintCardAriaLabel` composes its own summary
/// clause from typed data in one place, not that the two sibling
/// keys must be deduplicated against each other). `carried_over_points`
/// and `committed_count` are unused outside their own status branch;
/// callers pass `0` for whichever doesn't apply.
fn sprint_card_summary_text(
    status: SprintStatusLabel,
    completed_points: i64,
    committed_points: i64,
    carried_over_points: i64,
    committed_count: i64,
) -> String {
    match status {
        SprintStatusLabel::Completed => format!(
            "{completed_points} of {committed_points} pt completed · \
             {carried_over_points} carried over"
        ),
        SprintStatusLabel::Active => {
            let in_flight_points = committed_points - completed_points;
            format!(
                "{completed_points} of {committed_points} pt completed · \
                 {in_flight_points} pt in flight"
            )
        }
        SprintStatusLabel::Planned => {
            format!("{committed_points} pt committed across {committed_count} issues")
        }
    }
}

fn team_role_label(label: TeamRoleLabel) -> &'static str {
    match label {
        TeamRoleLabel::Admin => "Admin",
        TeamRoleLabel::Member => "Member",
        TeamRoleLabel::Viewer => "Viewer",
    }
}

fn priority_label(label: PriorityLabel) -> &'static str {
    match label {
        PriorityLabel::Low => "Low",
        PriorityLabel::Medium => "Medium",
        PriorityLabel::High => "High",
        PriorityLabel::Urgent => "Urgent",
    }
}

fn drift_direction_word(direction: DriftDirectionLabel) -> &'static str {
    match direction {
        DriftDirectionLabel::Up => "longer per point",
        DriftDirectionLabel::Down => "shorter per point",
        DriftDirectionLabel::Steady => "steady",
    }
}

/// The longer "trending ..." phrase used only inside
/// `MessageKey::DriftAriaLabel`'s composed sentence — distinct
/// wording from `drift_direction_word`'s short chip form, same
/// `DriftDirectionLabel`.
fn drift_trend_phrase(direction: DriftDirectionLabel) -> &'static str {
    match direction {
        DriftDirectionLabel::Up => "trending up",
        DriftDirectionLabel::Down => "trending down",
        DriftDirectionLabel::Steady => "roughly steady",
    }
}

/// The one-decimal rounding rule shared by the chip
/// (`switching_median_text`, below) and the aria sentence
/// (`MessageKey::SwitchingAriaLabel`), so the two surfaces cannot
/// disagree about decimals (`COPY-001` §2) — only the chip's " /
/// active day" suffix is surface-specific, which is why it lives in
/// `switching_median_text` and not here.
fn switching_median_number(median: f64) -> String {
    if median.fract() < 0.05 {
        format!("{median:.0}")
    } else {
        format!("{median:.1}")
    }
}

/// Shared by `MessageKey::SwitchingMedianValue` and, via
/// `switching_median_number`, `MessageKey::SwitchingAriaLabel` — see
/// that helper's doc comment.
fn switching_median_text(median: f64) -> String {
    format!("{} / active day", switching_median_number(median))
}

fn notification_kind_label(kind: NotificationKindLabel) -> &'static str {
    match kind {
        NotificationKindLabel::BurnoutOverload => "Sustained over-capacity streak",
        NotificationKindLabel::BurnoutStalled => "Long-stalled assigned work",
        NotificationKindLabel::ProjectTrendDecline => "Project health decline",
    }
}

fn notification_channel_label(channel: NotificationChannelLabel) -> &'static str {
    match channel {
        NotificationChannelLabel::InApp => "In-app",
        NotificationChannelLabel::Email => "Email",
        NotificationChannelLabel::Webhook => "Webhook",
    }
}
