use crate::settings::ClickerSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryData {
    pub version: String,
    pub click_speed: f64,
    pub click_interval: String,
    pub mouse_button: String,
    pub mode: String,
    pub duty_cycle_enabled: bool,
    pub duty_cycle: f64,
    pub speed_variation_enabled: bool,
    pub speed_variation: f64,
    pub double_click_enabled: bool,
    pub click_limit_enabled: bool,
    pub time_limit_enabled: bool,
    pub corner_stop_enabled: bool,
    pub edge_stop_enabled: bool,
    pub position_enabled: bool,
    pub telemetry_enabled: bool,
}

impl TelemetryData {
    pub fn from_settings(settings: &ClickerSettings, version: String) -> Self {
        Self {
            version,
            click_speed: settings.click_speed,
            click_interval: settings.click_interval.clone(),
            mouse_button: settings.mouse_button.clone(),
            mode: settings.mode.clone(),
            duty_cycle_enabled: settings.duty_cycle_enabled,
            duty_cycle: settings.duty_cycle,
            speed_variation_enabled: settings.speed_variation_enabled,
            speed_variation: settings.speed_variation,
            double_click_enabled: settings.double_click_enabled,
            click_limit_enabled: settings.click_limit_enabled,
            time_limit_enabled: settings.time_limit_enabled,
            corner_stop_enabled: settings.corner_stop_enabled,
            edge_stop_enabled: settings.edge_stop_enabled,
            position_enabled: settings.position_enabled,
            telemetry_enabled: settings.telemetry_enabled,
        }
    }
}

pub async fn send_settings_telemetry(_data: TelemetryData) -> Result<(), String> {
    // Telemetry is disabled on macOS for now
    // The original Windows version sent data to a Supabase backend
    // On macOS, we skip this to avoid backend dependency issues
    log::info!("[Telemetry] Settings telemetry recorded locally (macOS)");
    Ok(())
}
