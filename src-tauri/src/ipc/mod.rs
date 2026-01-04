//! ZFSS IPC Commands
//!
//! Tauri commands for frontend-backend communication.

pub mod signal_cmds;

// Re-export commands for registration
pub use signal_cmds::{capture_signal, get_signal, link_signal_to_issue, list_signals};
