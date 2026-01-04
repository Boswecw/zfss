//! User and Role model
//!
//! Role-based authority is enforced at the service layer.
//! - Steward: Policy owner, vocabulary control, decisions, overrides
//! - Operator: Log Signals, link Issues, execute policy
//! - Engineer: Produce Artifacts
//! - AI: Normalize, suggest, flag (cannot decide, promise, prioritize, or close)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User roles with authority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Steward,
    Operator,
    Engineer,
    AI,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Steward => "Steward",
            UserRole::Operator => "Operator",
            UserRole::Engineer => "Engineer",
            UserRole::AI => "AI",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Steward" => Some(UserRole::Steward),
            "Operator" => Some(UserRole::Operator),
            "Engineer" => Some(UserRole::Engineer),
            "AI" => Some(UserRole::AI),
            _ => None,
        }
    }

    // Authority checks

    /// Can log new signals (everyone)
    pub fn can_log_signal(&self) -> bool {
        true
    }

    /// Can link signals to issues (Steward, Operator)
    pub fn can_link_signal(&self) -> bool {
        matches!(self, UserRole::Steward | UserRole::Operator)
    }

    /// Can create issues (Steward, Operator)
    pub fn can_create_issue(&self) -> bool {
        matches!(self, UserRole::Steward | UserRole::Operator)
    }

    /// Can make decisions (Steward only)
    pub fn can_make_decision(&self) -> bool {
        matches!(self, UserRole::Steward)
    }

    /// Can create artifacts (Steward, Engineer)
    pub fn can_create_artifact(&self) -> bool {
        matches!(self, UserRole::Steward | UserRole::Engineer)
    }

    /// Can verify artifacts (Steward only)
    pub fn can_verify_artifact(&self) -> bool {
        matches!(self, UserRole::Steward)
    }

    /// Can draft responses (Steward, Operator, AI)
    pub fn can_draft_response(&self) -> bool {
        matches!(self, UserRole::Steward | UserRole::Operator | UserRole::AI)
    }

    /// Can approve responses (Steward only)
    pub fn can_approve_response(&self) -> bool {
        matches!(self, UserRole::Steward)
    }

    /// Can close issues (Steward only)
    pub fn can_close_issue(&self) -> bool {
        matches!(self, UserRole::Steward)
    }

    /// Can modify vocabulary (Steward only)
    pub fn can_modify_vocabulary(&self) -> bool {
        matches!(self, UserRole::Steward)
    }
}

/// Full User record from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Get the typed role
    pub fn role_enum(&self) -> Option<UserRole> {
        UserRole::from_str(&self.role)
    }
}

/// Current user session (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: String,
    pub display_name: String,
    pub role: UserRole,
}

impl CurrentUser {
    /// Create from User record
    pub fn from_user(user: &User) -> Option<Self> {
        let role = UserRole::from_str(&user.role)?;
        Some(Self {
            id: user.id.clone(),
            display_name: user.display_name.clone(),
            role,
        })
    }
}
