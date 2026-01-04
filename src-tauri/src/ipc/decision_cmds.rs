//! Decision IPC Commands
//!
//! Commands for recording and viewing decisions.
//! Only Stewards can make decisions (enforced here).

use crate::constraints::DEFAULT_STEWARD_DEADLINE_DAYS;
use crate::models::{Decision, DecisionCreate, DecisionHistoryEntry, DecisionType, UserRole};
use crate::repository;
use crate::state::AppState;
use std::sync::Arc;
use tauri::State;

/// Record a new decision on an issue (Steward only)
#[tauri::command]
pub async fn record_decision(
    issue_id: String,
    decision_type: String,
    rationale: String,
    steward_deadline_days: Option<i32>,
    state: State<'_, Arc<AppState>>,
) -> Result<Decision, String> {
    // Enforce Steward-only authority
    let role = state.current_user_role();
    if !role.can_make_decision() {
        return Err(format!(
            "Permission denied: only Stewards can make decisions (current role: {:?})",
            role
        ));
    }

    // Validate rationale length
    if rationale.len() < 10 {
        return Err("rationale must be at least 10 characters".to_string());
    }

    // Parse decision type
    let decision_type_enum = DecisionType::from_str(&decision_type).ok_or_else(|| {
        format!(
            "Invalid decision_type: '{}'. Valid: FixNow, FixLater, DocumentClarify, WontFix, DeEscalate",
            decision_type
        )
    })?;

    // Verify issue exists
    let issue = repository::get_issue(&state.pool, &issue_id)
        .await
        .map_err(|e| format!("Failed to get issue: {}", e))?
        .ok_or_else(|| format!("Issue not found: {}", issue_id))?;

    let decided_by = state.current_user_id();

    repository::append_decision(
        &state.pool,
        DecisionCreate {
            issue_id,
            decision_type: decision_type_enum,
            rationale,
            steward_deadline_days: steward_deadline_days.unwrap_or(DEFAULT_STEWARD_DEADLINE_DAYS),
        },
        &decided_by,
    )
    .await
    .map_err(|e| format!("Failed to record decision: {}", e))
}

/// Get a single decision by ID
#[tauri::command]
pub async fn get_decision(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<Decision>, String> {
    repository::get_decision(&state.pool, &id)
        .await
        .map_err(|e| format!("Failed to get decision: {}", e))
}

/// List all decisions for an issue (history)
#[tauri::command]
pub async fn list_decisions_for_issue(
    issue_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<DecisionHistoryEntry>, String> {
    repository::list_decisions_for_issue(&state.pool, &issue_id)
        .await
        .map_err(|e| format!("Failed to list decisions: {}", e))
}

/// Get the current (latest) decision for an issue
#[tauri::command]
pub async fn get_current_decision(
    issue_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<Decision>, String> {
    repository::get_current_decision_for_issue(&state.pool, &issue_id)
        .await
        .map_err(|e| format!("Failed to get current decision: {}", e))
}
