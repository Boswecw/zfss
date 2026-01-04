//! ZFSS Configuration
//!
//! Environment-driven configuration for PostgreSQL connection and app settings.

use anyhow::{Context, Result};
use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Settings {
    /// PostgreSQL database URL
    pub database_url: String,

    /// Current user ID (for operations)
    pub current_user_id: String,

    /// Window always on top for quick capture
    pub always_on_top: bool,
}

impl Settings {
    /// Load settings from environment variables
    pub fn from_env() -> Result<Self> {
        // Required: DATABASE_URL
        let database_url = env::var("ZFSS_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .context("ZFSS_DATABASE_URL or DATABASE_URL must be set")?;

        // Optional: Current user (defaults to 'system')
        let current_user_id = env::var("ZFSS_USER_ID").unwrap_or_else(|_| "system".to_string());

        // Optional: Always on top (defaults to false)
        let always_on_top = env::var("ZFSS_ALWAYS_ON_TOP")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        Ok(Self {
            database_url,
            current_user_id,
            always_on_top,
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Check database URL format
        if !self.database_url.starts_with("postgres://")
            && !self.database_url.starts_with("postgresql://")
        {
            anyhow::bail!("DATABASE_URL must start with postgres:// or postgresql://");
        }

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost/zfss".to_string(),
            current_user_id: "system".to_string(),
            always_on_top: false,
        }
    }
}
