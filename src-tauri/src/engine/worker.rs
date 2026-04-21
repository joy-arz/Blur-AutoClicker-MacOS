use std::f64::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Emitter, Manager};

use crate::app_state::ClickerStatusPayload;
use crate::dev_logger::DEV_LOGGER;
use crate::engine::start_clicker as engine_start;
use crate::engine::stats::{print_run_stats, record_run};
use crate::ClickerSettings;
use crate::ClickerState;
use crate::STATUS_EVENT;

use super::failsafe::should_stop_for_failsafe;
use super::mouse::{get_cursor_pos, move_mouse, send_clicks, smooth_move};
use super::rng::FastRng;
use super::ClickerConfig;
use super::PositionMode;
use super::RunOutcome;
use super::CLICK_COUNT;

// -- Tauri-aware commands --

pub fn start_clicker_inner(app: &AppHandle) -> Result<ClickerStatusPayload, String> {
    DEV_LOGGER.log("WORKER", "start_clicker_inner called");
    let state = app.state::<ClickerState>();
    if state.running.load(Ordering::SeqCst) {
        DEV_LOGGER.log("WORKER", "Clicker already running, returning error");
        return Err(String::from("Clicker is already running"));
    }

    {
        *state.last_error.lock().unwrap() = None;
        *state.stop_reason.lock().unwrap() = None;
    }

    let settings = state.settings.lock().unwrap().clone();
    let telemetry_enabled = settings.telemetry_enabled;
    DEV_LOGGER.log(
        "WORKER",
        &format!("settings.mouse_button='{}'", settings.mouse_button),
    );
    DEV_LOGGER.log(
        "WORKER",
        &format!("settings.position_enabled={}", settings.position_enabled),
    );
    DEV_LOGGER.log(
        "WORKER",
        &format!(
            "settings.position_x={}, position_y={}",
            settings.position_x, settings.position_y
        ),
    );
    DEV_LOGGER.log(
        "WORKER",
        &format!(
            "settings.position_mode='{}', position_enabled={}",
            settings.position_mode, settings.position_enabled
        ),
    );
    let config = build_config(&settings)?;
    DEV_LOGGER.log(
        "WORKER",
        &format!(
            "config built successfully: button={}, interval={}, click_speed_from_settings={}, mode={}",
            config.button, config.interval, settings.click_speed, settings.mode
        ),
    );
    state.running.store(true, Ordering::SeqCst);
    let running = state.running.clone();
    let app_handle = app.clone();

    std::thread::spawn(move || {
        let outcome = engine_start(config, running.clone());
        running.store(false, Ordering::SeqCst);

        print_run_stats(outcome.click_count, outcome.elapsed_secs, outcome.avg_cpu);

        record_run(
            outcome.click_count,
            outcome.elapsed_secs,
            outcome.avg_cpu,
            telemetry_enabled,
        );

        let state = app_handle.state::<ClickerState>();
        *state.stop_reason.lock().unwrap() = Some(outcome.stop_reason.clone());
        *state.last_error.lock().unwrap() = None;
        emit_status(&app_handle);
    });

    let payload = current_status(app);
    emit_status(app);
    Ok(payload)
}

pub fn stop_clicker_inner(
    app: &AppHandle,
    stop_reason: Option<String>,
) -> Result<ClickerStatusPayload, String> {
    let state = app.state::<ClickerState>();
    state.running.store(false, Ordering::SeqCst);
    if let Some(reason) = stop_reason {
        *state.stop_reason.lock().unwrap() = Some(reason);
    }
    let payload = current_status(app);
    emit_status(app);
    Ok(payload)
}

pub fn build_config(settings: &ClickerSettings) -> Result<ClickerConfig, String> {
    DEV_LOGGER.log("WORKER", "build_config called");
    if settings.click_speed <= 0.0 {
        DEV_LOGGER.log("WORKER", "Error: Click speed must be greater than zero");
        return Err(String::from("Click speed must be greater than zero"));
    }

    let base_interval_secs = match settings.click_interval.as_str() {
        "m" => 60.0 / settings.click_speed,
        "h" => 3600.0 / settings.click_speed,
        "d" => 86400.0 / settings.click_speed,
        _ => 1.0 / settings.click_speed,
    };

    let button = match settings.mouse_button.as_str() {
        "Right" => {
            DEV_LOGGER.log("WORKER", "mouse_button=Right -> button=1");
            1
        }
        "Middle" => {
            DEV_LOGGER.log("WORKER", "mouse_button=Middle -> button=2");
            2
        }
        _ => {
            DEV_LOGGER.log(
                "WORKER",
                &format!("mouse_button={} -> button=0 (Left)", settings.mouse_button),
            );
            0
        }
    };

    let time_limit_secs = if settings.time_limit_enabled {
        Some(match settings.time_limit_unit.as_str() {
            "m" => settings.time_limit * 60.0,
            "h" => settings.time_limit * 3600.0,
            _ => settings.time_limit,
        })
    } else {
        None
    };

    Ok(ClickerConfig {
        interval: base_interval_secs,
        variation: if settings.speed_variation_enabled {
            settings.speed_variation
        } else {
            0.0
        },
        limit: if settings.click_limit_enabled {
            settings.click_limit
        } else {
            0
        },
        duty: if settings.duty_cycle_enabled {
            settings.duty_cycle
        } else {
            0.01
        },
        time_limit: time_limit_secs.unwrap_or(0.0),
        button,
        double_click_enabled: settings.double_click_enabled,
        double_click_delay_ms: settings.double_click_delay,
        pos_x: if settings.position_enabled {
            settings.position_x
        } else {
            0
        },
        pos_y: if settings.position_enabled {
            settings.position_y
        } else {
            0
        },
        offset: 0.0,
        offset_chance: 0.0,
        smoothing: 0,
        corner_stop_enabled: settings.corner_stop_enabled,
        corner_stop_tl: settings.corner_stop_tl,
        corner_stop_tr: settings.corner_stop_tr,
        corner_stop_bl: settings.corner_stop_bl,
        corner_stop_br: settings.corner_stop_br,
        edge_stop_enabled: settings.edge_stop_enabled,
        edge_stop_top: settings.edge_stop_top,
        edge_stop_right: settings.edge_stop_right,
        edge_stop_bottom: settings.edge_stop_bottom,
        edge_stop_left: settings.edge_stop_left,
        position_mode: PositionMode::from_str(&settings.position_mode),
    })
}

pub fn current_status(app: &AppHandle) -> ClickerStatusPayload {
    let state = app.state::<ClickerState>();
    let last_error = state.last_error.lock().unwrap().clone();
    let stop_reason = state.stop_reason.lock().unwrap().clone();

    ClickerStatusPayload {
        running: state.running.load(Ordering::SeqCst),
        click_count: get_click_count(),
        last_error,
        stop_reason,
    }
}

pub fn emit_status(app: &AppHandle) {
    let _ = app.emit(STATUS_EVENT, current_status(app));
}

pub fn toggle_clicker_inner(app: &AppHandle) -> Result<ClickerStatusPayload, String> {
    let state = app.state::<ClickerState>();
    if state.running.load(Ordering::SeqCst) {
        stop_clicker_inner(app, Some(String::from("Stopped from hotkey")))
    } else {
        start_clicker_inner(app)
    }
}

pub fn now_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

// -- Engine loop --

pub fn start_clicker(config: ClickerConfig, running: Arc<AtomicBool>) -> RunOutcome {
    CLICK_COUNT.store(0, Ordering::SeqCst);

    let start_time = Instant::now();

    let mut rng = FastRng::new();
    let mut click_count: i64 = 0;
    let button = config.button;
    let cps = if config.interval > 0.0 {
        1.0 / config.interval
    } else {
        0.0
    };
    let batch_size = if !config.double_click_enabled && cps >= 50.0 {
        2usize
    } else {
        1usize
    };

    let batch_interval = config.interval * batch_size as f64;
    let has_fixed_position =
        config.position_mode == PositionMode::Fixed && (config.pos_x != 0 || config.pos_y != 0);
    let has_current_position = config.position_mode == PositionMode::Current;
    DEV_LOGGER.log(
        "WORKER",
        &format!(
            "position_mode={:?}, has_fixed_position={}, has_current_position={}, pos_x={}, pos_y={}",
            config.position_mode, has_fixed_position, has_current_position, config.pos_x, config.pos_y
        ),
    );
    let use_smoothing = config.smoothing == 1 && cps < 50.0;

    let mut target_x = config.pos_x;
    let mut target_y = config.pos_y;
    let mut next_batch_time = Instant::now();
    let mut stop_reason = String::from("Stopped");

    if has_fixed_position {
        DEV_LOGGER.log(
            "WORKER",
            &format!("Moving mouse to ({}, {})", target_x, target_y),
        );
        move_mouse(target_x, target_y);
    } else if has_current_position {
        DEV_LOGGER.log("WORKER", "Using current cursor position for clicking");
    } else {
        DEV_LOGGER.log(
            "WORKER",
            "No position set, will click at current cursor position",
        );
    }

    while running.load(Ordering::SeqCst) {
        if let Some(reason) = should_stop_for_failsafe(&config) {
            stop_reason = reason;
            break;
        }

        if config.limit > 0 && click_count >= config.limit as i64 {
            stop_reason = format!("Click limit reached ({})", config.limit);
            break;
        }

        if config.time_limit > 0.0 && start_time.elapsed().as_secs_f64() >= config.time_limit {
            stop_reason = format!("Time limit reached ({:.1}s)", config.time_limit);
            break;
        }

        let batch_duration = if config.variation > 0.0 {
            let std_dev = batch_interval * (config.variation / 100.0);
            rng.next_gaussian(batch_interval, std_dev)
        } else {
            batch_interval
        };
        let hold_ms = (config.interval * (config.duty.max(0.0) / 100.0) * 1000.0) as u32;

        next_batch_time += Duration::from_secs_f64(batch_duration.max(0.001));

        if has_fixed_position {
            if config.offset_chance <= 0.0 || rng.next_f64() * 100.0 <= config.offset_chance {
                let angle = rng.next_f64() * 2.0 * PI;
                let radius = rng.next_f64().sqrt() * config.offset;
                target_x = (config.pos_x as f64 + radius * angle.cos()) as i32;
                target_y = (config.pos_y as f64 + radius * angle.sin()) as i32;
            }

            if use_smoothing {
                let (cur_x, cur_y) = get_cursor_pos();
                if cur_x != target_x || cur_y != target_y {
                    let smooth_dur =
                        ((batch_duration * (0.2 + rng.next_f64() * 0.4)) * 1000.0) as u64;
                    smooth_move(
                        cur_x,
                        cur_y,
                        target_x,
                        target_y,
                        smooth_dur.clamp(15, 200),
                        &mut rng,
                    );
                }
            } else {
                move_mouse(target_x, target_y);
            }
        }

        let remaining_clicks = if config.limit > 0 {
            (config.limit as i64 - click_count).max(0) as usize
        } else {
            usize::MAX
        };

        let clicks_this_cycle = if config.double_click_enabled {
            remaining_clicks.min(2)
        } else {
            remaining_clicks.min(batch_size)
        };

        if clicks_this_cycle == 0 {
            stop_reason = format!("Click limit reached ({})", config.limit);
            break;
        }

        DEV_LOGGER.log(
            "WORKER",
            &format!(
                "Sending {} clicks with button={}",
                clicks_this_cycle, button
            ),
        );
        send_clicks(
            button,
            clicks_this_cycle,
            hold_ms,
            config.double_click_enabled,
            config.double_click_delay_ms,
            &running,
        );

        click_count += clicks_this_cycle as i64;
        CLICK_COUNT.store(click_count, Ordering::Relaxed);

        let remaining = next_batch_time.saturating_duration_since(Instant::now());
        if remaining > Duration::ZERO {
            sleep_interruptible(remaining, &running);
        }
    }

    running.store(false, Ordering::SeqCst);

    let elapsed_secs = start_time.elapsed().as_secs_f64();

    RunOutcome {
        stop_reason,
        click_count,
        elapsed_secs,
        avg_cpu: -1.0, // CPU measurement not implemented on macOS yet
    }
}

pub fn get_click_count() -> i64 {
    CLICK_COUNT.load(Ordering::Relaxed)
}

pub fn sleep_interruptible(remaining: Duration, running: &Arc<AtomicBool>) {
    let tick = Duration::from_millis(5);
    let start = Instant::now();
    while running.load(Ordering::SeqCst) && start.elapsed() < remaining {
        let left = remaining.saturating_sub(start.elapsed());
        std::thread::sleep(left.min(tick));
    }
}
