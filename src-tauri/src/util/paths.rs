//! Path utilities for ZFSS
//!
//! Platform-specific paths for app data.

use anyhow::Result;
use directories::ProjectDirs;
use std::path::PathBuf;

/// Get the app data directory
pub fn app_data_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "forge", "zfss")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    Ok(dirs.data_dir().to_path_buf())
}

/// Get the cache directory
pub fn cache_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "forge", "zfss")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    Ok(dirs.cache_dir().to_path_buf())
}

/// Get the config directory
pub fn config_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "forge", "zfss")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    Ok(dirs.config_dir().to_path_buf())
}
