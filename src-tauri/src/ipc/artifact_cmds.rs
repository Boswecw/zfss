//! Artifact IPC Commands
//!
//! Commands for creating, listing, and verifying artifacts.
//! Engineers create artifacts, Stewards verify them.

use crate::models::{Artifact, ArtifactCreate, ArtifactSummary, ArtifactType, UserRole};
use crate::repository;
use crate::state::AppState;
use std::sync::Arc;
use tauri::State;

/// Create a new artifact (Engineer or Steward only)
#[tauri::command]
pub async fn create_artifact(
    issue_id: String,
    artifact_type: String,
    title: String,
    description: Option<String>,
    ref_url: Option<String>,
    note: Option<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<Artifact, String> {
    // Enforce Engineer or Steward authority
    let role = state.current_user_role();
    if !role.can_create_artifact() {
        return Err(format!(
            "Permission denied: only Engineers or Stewards can create artifacts (current role: {:?})",
            role
        ));
    }

    // Validate title
    if title.trim().is_empty() {
        return Err("title cannot be empty".to_string());
    }

    if title.len() > 500 {
        return Err("title cannot exceed 500 characters".to_string());
    }

    // Parse artifact type
    let artifact_type_enum = ArtifactType::from_str(&artifact_type).ok_or_else(|| {
        format!(
            "Invalid artifact_type: '{}'. Valid: Code, Logic, Knowledge, Test, Law",
            artifact_type
        )
    })?;

    // Verify issue exists
    repository::get_issue(&state.pool, &issue_id)
        .await
        .map_err(|e| format!("Failed to get issue: {}", e))?
        .ok_or_else(|| format!("Issue not found: {}", issue_id))?;

    let created_by = state.current_user_id();

    repository::append_artifact(
        &state.pool,
        ArtifactCreate {
            issue_id,
            artifact_type: artifact_type_enum,
            title,
            description,
            ref_url,
            note,
        },
        &created_by,
    )
    .await
    .map_err(|e| format!("Failed to create artifact: {}", e))
}

/// Get a single artifact by ID
#[tauri::command]
pub async fn get_artifact(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<Artifact>, String> {
    repository::get_artifact(&state.pool, &id)
        .await
        .map_err(|e| format!("Failed to get artifact: {}", e))
}

/// List all artifacts for an issue
#[tauri::command]
pub async fn list_artifacts_for_issue(
    issue_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<ArtifactSummary>, String> {
    repository::list_artifacts_for_issue(&state.pool, &issue_id)
        .await
        .map_err(|e| format!("Failed to list artifacts: {}", e))
}

/// Verify an artifact (Steward only)
#[tauri::command]
pub async fn verify_artifact(
    artifact_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Artifact, String> {
    // Enforce Steward-only authority
    let role = state.current_user_role();
    if !role.can_verify_artifact() {
        return Err(format!(
            "Permission denied: only Stewards can verify artifacts (current role: {:?})",
            role
        ));
    }

    let verified_by = state.current_user_id();

    repository::verify_artifact(&state.pool, &artifact_id, &verified_by)
        .await
        .map_err(|e| format!("Failed to verify artifact: {}", e))
}

/// Check if an issue has any verified artifacts
#[tauri::command]
pub async fn has_verified_artifact(
    issue_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<bool, String> {
    repository::has_verified_artifact(&state.pool, &issue_id)
        .await
        .map_err(|e| format!("Failed to check verified artifacts: {}", e))
}
