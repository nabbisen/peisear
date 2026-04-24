//! Pure domain types shared across the workspace.
//!
//! This crate intentionally depends only on `serde`, `chrono`, and
//! `thiserror`; it has no knowledge of axum, sqlx, jsonwebtoken, or any
//! other runtime concern. Consumers include:
//!
//! - `peisear-storage`: constructs values of these types from DB rows
//! - `peisear-web`: receives them from storage and feeds them to
//!   handlers / Leptos components
//! - future `peisear-cli` or admin tools that will speak the same
//!   domain vocabulary without pulling in the web stack

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    Open,
    InProgress,
    Done,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in_progress",
            Self::Done => "done",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::InProgress => "In Progress",
            Self::Done => "Done",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "open" => Some(Self::Open),
            "in_progress" => Some(Self::InProgress),
            "done" => Some(Self::Done),
            _ => None,
        }
    }

    pub fn all() -> [IssueStatus; 3] {
        [Self::Open, Self::InProgress, Self::Done]
    }
}

impl fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Urgent => "urgent",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "urgent" => Some(Self::Urgent),
            _ => None,
        }
    }

    pub fn all() -> [Priority; 4] {
        [Self::Low, Self::Medium, Self::High, Self::Urgent]
    }

    /// daisyUI badge class mapping — kept in core so any future
    /// read-only surface (email summary, future client, etc.) can
    /// reuse the canonical severity palette.
    pub fn badge_class(&self) -> &'static str {
        match self {
            Self::Low => "badge-ghost",
            Self::Medium => "badge-info",
            Self::High => "badge-warning",
            Self::Urgent => "badge-error",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub title: String,
    pub description: String,
    pub status: IssueStatus,
    pub priority: Priority,
    pub position: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Compact view of the authenticated user attached to requests.
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: String,
    pub email: String,
    pub display_name: String,
}

impl From<User> for CurrentUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            display_name: u.display_name,
        }
    }
}
