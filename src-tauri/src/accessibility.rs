use std::process::Command;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
}

pub fn check_accessibility() -> bool {
    unsafe { AXIsProcessTrusted() }
}

pub fn open_accessibility_settings() {
    if let Err(e) = Command::new("open")
        .args(["x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"])
        .spawn()
    {
        log::error!("[Accessibility] Failed to open System Settings: {}", e);
    }
}

#[tauri::command]
pub fn check_accessibility_permission() -> bool {
    check_accessibility()
}

#[tauri::command]
pub fn request_accessibility_permission() {
    open_accessibility_settings();
}

#[tauri::command]
pub fn open_accessibility_settings_cmd() {
    open_accessibility_settings();
}