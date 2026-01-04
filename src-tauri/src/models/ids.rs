//! Typed ID generation for ZFSS canonical objects.
//!
//! Each ID follows the pattern: {prefix}_{random_alphanumeric}
//! Examples: sig_AbCd1234EfGh5678IjKl, iss_XyZw9876VuTs5432QpOn

use crate::constraints::ID_SUFFIX_LENGTH;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdError {
    #[error("Invalid ID format: expected prefix '{expected}', got '{actual}'")]
    InvalidPrefix { expected: String, actual: String },

    #[error("Invalid ID format: {0}")]
    InvalidFormat(String),
}

/// Generate a random alphanumeric suffix
fn generate_suffix() -> String {
    let mut rng = rand::thread_rng();
    (0..ID_SUFFIX_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..=9 => (b'0' + idx) as char,
                10..=35 => (b'a' + idx - 10) as char,
                _ => (b'A' + idx - 36) as char,
            }
        })
        .collect()
}

/// Macro to define typed ID structs
macro_rules! define_typed_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Create a new random ID
            pub fn new() -> Self {
                Self(format!("{}_{}", $prefix, generate_suffix()))
            }

            /// Parse from string, validating the prefix
            pub fn from_string(s: String) -> Result<Self, IdError> {
                let expected_prefix = concat!($prefix, "_");
                if !s.starts_with(expected_prefix) {
                    return Err(IdError::InvalidPrefix {
                        expected: $prefix.to_string(),
                        actual: s.split('_').next().unwrap_or("").to_string(),
                    });
                }

                let suffix = &s[expected_prefix.len()..];
                if suffix.len() < 16 {
                    return Err(IdError::InvalidFormat(format!(
                        "ID suffix too short: expected at least 16 chars, got {}",
                        suffix.len()
                    )));
                }

                if !suffix.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(IdError::InvalidFormat(
                        "ID suffix must be alphanumeric".to_string(),
                    ));
                }

                Ok(Self(s))
            }

            /// Get the inner string
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume and return the inner string
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> String {
                id.0
            }
        }
    };
}

// Define all typed IDs
define_typed_id!(SignalId, "sig");
define_typed_id!(IssueId, "iss");
define_typed_id!(DecisionId, "dec");
define_typed_id!(ArtifactId, "art");
define_typed_id!(ResponseId, "rsp");
define_typed_id!(AttachmentId, "att");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_id_generation() {
        let id = SignalId::new();
        assert!(id.as_str().starts_with("sig_"));
        assert!(id.as_str().len() >= 20);
    }

    #[test]
    fn test_signal_id_parsing() {
        let id = SignalId::from_string("sig_AbCdEfGhIjKlMnOpQrSt".to_string());
        assert!(id.is_ok());
    }

    #[test]
    fn test_invalid_prefix() {
        let id = SignalId::from_string("iss_AbCdEfGhIjKlMnOpQrSt".to_string());
        assert!(id.is_err());
    }

    #[test]
    fn test_short_suffix() {
        let id = SignalId::from_string("sig_short".to_string());
        assert!(id.is_err());
    }
}
