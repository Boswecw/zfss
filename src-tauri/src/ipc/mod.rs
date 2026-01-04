//! ZFSS IPC Commands
//!
//! Tauri commands for frontend-backend communication.

pub mod artifact_cmds;
pub mod decision_cmds;
pub mod issue_cmds;
pub mod response_cmds;
pub mod signal_cmds;

// Re-export all commands for registration
pub use artifact_cmds::{
    create_artifact, get_artifact, has_verified_artifact, list_artifacts_for_issue, verify_artifact,
};
pub use decision_cmds::{
    get_current_decision, get_decision, list_decisions_for_issue, record_decision,
};
pub use issue_cmds::{create_issue, get_issue, list_issues, transition_issue};
pub use response_cmds::{
    approve_response, block_response, draft_response, get_response, list_responses_for_signal,
    mark_response_sent, submit_response,
};
pub use signal_cmds::{capture_signal, get_signal, link_signal_to_issue, list_signals};
