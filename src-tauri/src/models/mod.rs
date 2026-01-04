//! ZFSS Data Models
//!
//! Canonical objects: Signal, Issue, Decision, Artifact, Response
//! All models follow append-only semantics where applicable.

pub mod artifact;
pub mod decision;
pub mod ids;
pub mod issue;
pub mod response;
pub mod signal;
pub mod user;

// Re-export common types
pub use artifact::{Artifact, ArtifactCreate, ArtifactSummary, ArtifactType};
pub use decision::{Decision, DecisionCreate, DecisionHistoryEntry, DecisionType};
pub use ids::{ArtifactId, AttachmentId, DecisionId, IdError, IssueId, ResponseId, SignalId};
pub use issue::{
    Classification, Frequency, Issue, IssueCreate, IssueStatus, IssueSummary, Severity,
};
pub use response::{ApprovalState, Response, ResponseChannel, ResponseCreate, ResponseSummary};
pub use signal::{
    Signal, SignalCreate, SignalEnvironment, SignalReporter, SignalSource, SignalStatus,
    SignalSummary,
};
pub use user::{CurrentUser, User, UserRole};
