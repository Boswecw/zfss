//! Decision model - Declared intent (append-only)
//!
//! Decisions are explicit, attributable, and append-only.
//! Only Stewards can make decisions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Decision types from ZFSS doctrine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    FixNow,
    FixLater,
    DocumentClarify,
    WontFix,
    DeEscalate,
}

impl DecisionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DecisionType::FixNow => "FixNow",
            DecisionType::FixLater => "FixLater",
            DecisionType::DocumentClarify => "DocumentClarify",
            DecisionType::WontFix => "WontFix",
            DecisionType::DeEscalate => "DeEscalate",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "FixNow" => Some(DecisionType::FixNow),
            "FixLater" => Some(DecisionType::FixLater),
            "DocumentClarify" => Some(DecisionType::DocumentClarify),
            "WontFix" => Some(DecisionType::WontFix),
            "DeEscalate" => Some(DecisionType::DeEscalate),
            _ => None,
        }
    }

    /// Whether this decision requires action
    pub fn requires_action(&self) -> bool {
        matches!(
            self,
            DecisionType::FixNow | DecisionType::FixLater | DecisionType::DocumentClarify
        )
    }
}

/// Full Decision record from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub issue_id: String,
    pub decision_type: String,
    pub rationale: String,
    pub decided_by: String,
    pub decided_at: DateTime<Utc>,
    pub steward_deadline_days: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes_id: Option<String>,
}

/// Input for creating a new decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionCreate {
    pub issue_id: String,
    pub decision_type: DecisionType,
    pub rationale: String,
    #[serde(default = "default_deadline_days")]
    pub steward_deadline_days: i32,
}

fn default_deadline_days() -> i32 {
    crate::constraints::DEFAULT_STEWARD_DEADLINE_DAYS
}

/// Decision timeline entry (for history view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionHistoryEntry {
    pub id: String,
    pub decision_type: String,
    pub rationale: String,
    pub decided_by: String,
    pub decided_at: DateTime<Utc>,
    pub is_current: bool,
}
