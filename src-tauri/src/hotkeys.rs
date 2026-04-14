use crate::AppHandle;
use crate::ClickerState;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HotkeyBinding {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
    pub main_key: String,
}

pub fn register_hotkey_inner(app: &AppHandle, hotkey: String) -> Result<String, String> {
    let binding = parse_hotkey_binding(&hotkey)?;

    let shortcut = build_shortcut_from_binding(&binding)?;

    let state = app.state::<ClickerState>();
    *state.registered_hotkey.lock().unwrap() = Some(binding.clone());

    let _ = app.global_shortcut().unregister_all();

    app.global_shortcut()
        .on_shortcut(shortcut, move |app_handle, _shortcut, event| {
            if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                handle_hotkey_pressed(app_handle);
            } else {
                handle_hotkey_released(app_handle);
            }
        })
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;

    Ok(format_hotkey_binding(&binding))
}

pub fn normalize_hotkey(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace("control", "ctrl")
        .replace("command", "super")
        .replace("meta", "super")
        .replace("win", "super")
        .replace("option", "alt")
}

pub fn parse_hotkey_binding(hotkey: &str) -> Result<HotkeyBinding, String> {
    let normalized = normalize_hotkey(hotkey);
    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut super_key = false;
    let mut main_key: Option<String> = None;

    for token in normalized.split('+').map(str::trim) {
        if token.is_empty() {
            return Err(format!(
                "Invalid hotkey '{}': found empty key token",
                hotkey
            ));
        }

        match token {
            "alt" | "option" => alt = true,
            "ctrl" | "control" => ctrl = true,
            "shift" => shift = true,
            "super" | "command" | "cmd" | "meta" | "win" => super_key = true,
            _ => {
                if main_key.replace(token.to_string()).is_some() {
                    return Err(format!(
                        "Invalid hotkey '{}': use modifiers first and only one main key",
                        hotkey
                    ));
                }
            }
        }
    }

    let main_key =
        main_key.ok_or_else(|| format!("Invalid hotkey '{}': missing main key", hotkey))?;

    Ok(HotkeyBinding {
        ctrl,
        alt,
        shift,
        super_key,
        main_key,
    })
}

pub fn build_shortcut_from_binding(binding: &HotkeyBinding) -> Result<Shortcut, String> {
    use std::str::FromStr;

    let mut key_str = String::new();

    if binding.ctrl {
        key_str.push_str("ctrl+");
    }
    if binding.alt {
        key_str.push_str("alt+");
    }
    if binding.shift {
        key_str.push_str("shift+");
    }
    if binding.super_key {
        key_str.push_str("super+");
    }

    key_str.push_str(&binding.main_key);

    Shortcut::from_str(&key_str)
        .map_err(|e| format!("Failed to parse shortcut '{}': {}", key_str, e))
}

pub fn format_hotkey_binding(binding: &HotkeyBinding) -> String {
    let mut parts: Vec<String> = Vec::new();

    if binding.ctrl {
        parts.push(String::from("ctrl"));
    }
    if binding.alt {
        parts.push(String::from("alt"));
    }
    if binding.shift {
        parts.push(String::from("shift"));
    }
    if binding.super_key {
        parts.push(String::from("super"));
    }

    parts.push(binding.main_key.clone());
    parts.join("+")
}

pub fn start_hotkey_listener(_app: AppHandle) {
    // Hotkey listening is handled by tauri-plugin-global-shortcut
}

pub fn handle_hotkey_pressed(app: &AppHandle) {
    let mode = {
        let state = app.state::<ClickerState>();
        let mode = state.settings.lock().unwrap().mode.clone();
        mode
    };

    if mode == "Toggle" {
        let _ = crate::engine::worker::toggle_clicker_inner(app);
    } else if mode == "Hold" {
        let _ = crate::engine::worker::start_clicker_inner(app);
    }
}

pub fn handle_hotkey_released(app: &AppHandle) {
    let mode = {
        let state = app.state::<ClickerState>();
        let mode = state.settings.lock().unwrap().mode.clone();
        mode
    };

    if mode == "Hold" {
        let _ = crate::engine::worker::stop_clicker_inner(
            app,
            Some(String::from("Stopped from hold hotkey")),
        );
    }
}
