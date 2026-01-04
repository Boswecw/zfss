//! ZFSS Constants and Constraints
//!
//! Non-negotiable limits and patterns enforced across the system.

/// Maximum length for raw_text in signals (bytes)
pub const MAX_RAW_TEXT_BYTES: usize = 100_000;

/// Maximum length for title fields
pub const MAX_TITLE_LENGTH: usize = 500;

/// Maximum length for rationale (minimum is enforced by DB)
pub const MAX_RATIONALE_LENGTH: usize = 10_000;

/// Minimum length for rationale
pub const MIN_RATIONALE_LENGTH: usize = 10;

/// Global hotkey for signal capture
pub const HOTKEY_COMBO: &str = "Ctrl+Alt+Z";

/// Hotkey debounce in milliseconds
pub const HOTKEY_DEBOUNCE_MS: u64 = 500;

/// ID prefix patterns
pub const SIGNAL_ID_PREFIX: &str = "sig_";
pub const ISSUE_ID_PREFIX: &str = "iss_";
pub const DECISION_ID_PREFIX: &str = "dec_";
pub const ARTIFACT_ID_PREFIX: &str = "art_";
pub const RESPONSE_ID_PREFIX: &str = "rsp_";
pub const ATTACHMENT_ID_PREFIX: &str = "att_";

/// ID suffix length (random alphanumeric characters)
pub const ID_SUFFIX_LENGTH: usize = 20;

/// App version string
pub const APP_VERSION: &str = "zfss-v1.0.0";

/// Default steward deadline in days
pub const DEFAULT_STEWARD_DEADLINE_DAYS: i32 = 7;
