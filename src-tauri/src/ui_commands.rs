use std::sync::atomic::Ordering;

use tauri::AppHandle;
use tauri::Manager;

use crate::app_state::AppInfoPayload;
use crate::app_state::ClickerStatusPayload;
use crate::app_state::PositionPayload;
use crate::engine::stats::CumulativeStats;
use crate::settings::ClickerSettings;
use crate::ClickerState;

use crate::engine::mouse::current_cursor_position;
use crate::engine::worker::current_status;
use crate::engine::worker::now_epoch_ms;
use crate::engine::worker::start_clicker_inner;
use crate::engine::worker::stop_clicker_inner;
use crate::hotkeys::register_hotkey_inner;

#[tauri::command]
pub fn start_clicker(app: AppHandle) -> Result<ClickerStatusPayload, String> {
    start_clicker_inner(&app)
}

#[tauri::command]
pub fn stop_clicker(app: AppHandle) -> Result<ClickerStatusPayload, String> {
    stop_clicker_inner(&app, Some(String::from("Stopped from UI")))
}

#[tauri::command]
pub fn toggle_clicker(app: AppHandle) -> Result<ClickerStatusPayload, String> {
    let state = app.state::<ClickerState>();
    if state.running.load(Ordering::SeqCst) {
        stop_clicker_inner(&app, Some(String::from("Stopped from toggle")))
    } else {
        start_clicker_inner(&app)
    }
}

#[tauri::command]
pub fn update_settings(
    app: AppHandle,
    settings: ClickerSettings,
) -> Result<ClickerSettings, String> {
    let state = app.state::<ClickerState>();
    let was_initialized = state.settings_initialized.load(Ordering::SeqCst);

    *state.settings.lock().unwrap() = settings.clone();

    if !was_initialized {
        state.settings_initialized.store(true, Ordering::SeqCst);
        log::info!("[Settings] First update_settings — initialized");
        return Ok(settings);
    }

    Ok(settings)
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<ClickerSettings, String> {
    let state = app.state::<ClickerState>();
    let settings = state.settings.lock().unwrap().clone();
    Ok(settings)
}

#[tauri::command]
pub fn reset_settings(app: AppHandle) -> Result<ClickerSettings, String> {
    let defaults = ClickerSettings::default();
    {
        let state = app.state::<ClickerState>();
        *state.settings.lock().unwrap() = defaults.clone();
    }
    register_hotkey_inner(&app, defaults.hotkey.clone())?;
    Ok(defaults)
}

#[tauri::command]
pub fn get_status(app: AppHandle) -> Result<ClickerStatusPayload, String> {
    Ok(current_status(&app))
}

#[tauri::command]
pub fn register_hotkey(app: AppHandle, hotkey: String) -> Result<String, String> {
    register_hotkey_inner(&app, hotkey)
}

#[tauri::command]
pub fn set_hotkey_capture_active(app: AppHandle, active: bool) -> Result<(), String> {
    let state = app.state::<ClickerState>();
    state.hotkey_capture_active.store(active, Ordering::SeqCst);

    if active {
        state
            .suppress_hotkey_until_ms
            .store(now_epoch_ms().saturating_add(250), Ordering::SeqCst);
    } else {
        state
            .suppress_hotkey_until_release
            .store(true, Ordering::SeqCst);
    }

    Ok(())
}

#[tauri::command]
pub fn pick_position() -> Result<PositionPayload, String> {
    let (x, y) =
        current_cursor_position().ok_or_else(|| String::from("Failed to read cursor position"))?;
    Ok(PositionPayload { x, y })
}

#[tauri::command]
pub fn get_app_info(app: AppHandle) -> Result<AppInfoPayload, String> {
    let version = app.package_info().version.to_string();
    Ok(AppInfoPayload {
        version,
        update_status: String::from("Update checks are disabled in development"),
        screenshot_protection_supported: false,
    })
}

#[tauri::command]
pub fn get_stats() -> Result<CumulativeStats, String> {
    crate::engine::stats::get_stats()
}

#[tauri::command]
pub fn reset_stats() -> Result<CumulativeStats, String> {
    crate::engine::stats::reset_stats()
}
