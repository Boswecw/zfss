//! Artifact model - Proof of learning
//!
//! No Issue may close without a valid, verified Artifact.
//! Engineers create artifacts, Stewards verify them.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Valid artifact types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    Code,
    Logic,
    Knowledge,
    Test,
    Law,
}

impl ArtifactType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtifactType::Code => "Code",
            ArtifactType::Logic => "Logic",
            ArtifactType::Knowledge => "Knowledge",
            ArtifactType::Test => "Test",
            ArtifactType::Law => "Law",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Code" => Some(ArtifactType::Code),
            "Logic" => Some(ArtifactType::Logic),
            "Knowledge" => Some(ArtifactType::Knowledge),
            "Test" => Some(ArtifactType::Test),
            "Law" => Some(ArtifactType::Law),
            _ => None,
        }
    }

    /// Description of what this artifact type represents
    pub fn description(&self) -> &'static str {
        match self {
            ArtifactType::Code => "PR / Commit - actual code change",
            ArtifactType::Logic => "Prompt / Config - logic change",
            ArtifactType::Knowledge => "Docs / Encyclopedia - documentation",
            ArtifactType::Test => "Regression coverage - test addition",
            ArtifactType::Law => "Permanent Decision Record - policy change",
        }
    }
}

/// Full Artifact record from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub issue_id: String,
    pub artifact_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}

/// Input for creating a new artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactCreate {
    pub issue_id: String,
    pub artifact_type: ArtifactType,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Artifact summary for list views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSummary {
    pub id: String,
    pub artifact_type: String,
    pub title: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}
