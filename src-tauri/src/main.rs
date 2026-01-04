// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! ZFSS - Zen Feedback & Service System
//!
//! Desktop application for feedback metabolism.
//! DataForgeDB (local PostgreSQL) is the authoritative source of truth.

mod config;
mod constraints;
mod db;
mod ipc;
mod models;
mod repository;
mod state;

use crate::config::Settings;
use crate::constraints::HOTKEY_DEBOUNCE_MS;
use crate::db::create_pool;
use crate::ipc::signal_cmds::{capture_signal, get_signal, link_signal_to_issue, list_signals};
use crate::state::AppState;

use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use uuid::Uuid;

/// Load or create device ID for this installation
fn load_or_create_device_id() -> anyhow::Result<Uuid> {
    use directories::ProjectDirs;
    use std::fs;

    let dirs = ProjectDirs::from("com", "forge", "zfss")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    let data_dir = dirs.data_dir();
    fs::create_dir_all(data_dir)?;

    let device_id_path = data_dir.join("device_id.txt");

    if device_id_path.exists() {
        let content = fs::read_to_string(&device_id_path)?;
        if let Ok(id) = Uuid::parse_str(content.trim()) {
            return Ok(id);
        }
    }

    // Create new device ID
    let id = Uuid::new_v4();
    fs::write(&device_id_path, id.to_string())?;
    Ok(id)
}

/// Toggle window visibility
fn toggle_window(window: &tauri::WebviewWindow) {
    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env if present
    let _ = dotenvy::dotenv();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Load settings
            let settings = Settings::from_env().map_err(|e| {
                eprintln!("Configuration error: {}", e);
                e
            })?;

            // Validate settings
            settings.validate().map_err(|e| {
                eprintln!("Configuration validation error: {}", e);
                e
            })?;

            // Load device ID
            let device_id = load_or_create_device_id().map_err(|e| {
                eprintln!("Failed to load device ID: {}", e);
                e
            })?;

            println!("ZFSS starting...");
            println!("  Device ID: {}", device_id);
            println!(
                "  Database: {}",
                &settings.database_url[..settings.database_url.len().min(50)]
            );

            // Create database pool (async in sync context)
            let pool =
                tauri::async_runtime::block_on(async { create_pool(&settings.database_url).await })
                    .map_err(|e| {
                        eprintln!("Database connection failed: {}", e);
                        e
                    })?;

            println!("  Database connected!");

            // Create shared state
            let state = Arc::new(AppState::new(pool, settings, device_id));

            // Store state in Tauri
            app.manage(state.clone());

            // Register global hotkey (Ctrl+Alt+Z)
            let window = app.get_webview_window("main").unwrap();
            let last_toggle = Arc::new(std::sync::Mutex::new(Instant::now()));
            let last_toggle_clone = last_toggle.clone();

            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyZ);

            app.global_shortcut()
                .on_shortcut(shortcut, move |_app, _shortcut, _event| {
                    // Debounce
                    let mut last = last_toggle_clone.lock().unwrap();
                    if last.elapsed() < Duration::from_millis(HOTKEY_DEBOUNCE_MS) {
                        return;
                    }
                    *last = Instant::now();

                    toggle_window(&window);
                })?;

            println!("  Global hotkey registered: Ctrl+Alt+Z");
            println!("ZFSS ready!");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            capture_signal,
            list_signals,
            get_signal,
            link_signal_to_issue,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
