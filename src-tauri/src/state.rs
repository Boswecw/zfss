//! ZFSS Application State
//!
//! Shared state for Tauri application, injected into IPC commands.

use crate::config::Settings;
use crate::models::{CurrentUser, UserRole};
use sqlx::PgPool;
use std::sync::Mutex;
use std::time::Instant;
use uuid::Uuid;

/// Shared application state
pub struct AppState {
    /// PostgreSQL connection pool
    pub pool: PgPool,

    /// Application settings
    pub settings: Settings,

    /// Unique device identifier
    pub device_id: Uuid,

    /// Monotonic start time (for relative timestamps)
    pub mono_start: Instant,

    /// Current authenticated user
    pub current_user: Mutex<Option<CurrentUser>>,

    /// Last hotkey toggle time (for debounce)
    pub last_hotkey: Mutex<Instant>,
}

impl AppState {
    /// Create new application state
    pub fn new(pool: PgPool, settings: Settings, device_id: Uuid) -> Self {
        Self {
            pool,
            settings,
            device_id,
            mono_start: Instant::now(),
            current_user: Mutex::new(None),
            last_hotkey: Mutex::new(Instant::now()),
        }
    }

    /// Get the current user ID (from settings or authenticated user)
    pub fn current_user_id(&self) -> String {
        self.current_user
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|u| u.id.clone()))
            .unwrap_or_else(|| self.settings.current_user_id.clone())
    }

    /// Get the current user role (from settings or authenticated user)
    pub fn current_user_role(&self) -> UserRole {
        self.current_user
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|u| u.role))
            .or_else(|| UserRole::from_str(&self.settings.current_user_role))
            .unwrap_or(UserRole::Steward) // Default to Steward for local dev
    }

    /// Get monotonic milliseconds since app start
    pub fn mono_ms(&self) -> u64 {
        self.mono_start.elapsed().as_millis() as u64
    }

    /// Set the current user
    pub fn set_current_user(&self, user: Option<CurrentUser>) {
        if let Ok(mut guard) = self.current_user.lock() {
            *guard = user;
        }
    }
}
