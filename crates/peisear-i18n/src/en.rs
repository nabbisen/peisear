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
    EntityKind, Field, IndicatorLabel, IssueStatusLabel, MessageKey, NavSection, PriorityLabel,
    SprintStatusLabel, TeamRoleLabel,
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
            summary,
        } => format!("{name} ({status}, {dates}). {summary}"),
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
