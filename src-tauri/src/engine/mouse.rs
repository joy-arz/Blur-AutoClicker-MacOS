use core_graphics::display::CGDisplay;
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use super::rng::FastRng;
use crate::dev_logger::DEV_LOGGER;

/// Cached display height for coordinate conversion
static DISPLAY_HEIGHT: OnceLock<i32> = OnceLock::new();

fn get_display_height() -> i32 {
    *DISPLAY_HEIGHT.get_or_init(|| CGDisplay::main().bounds().size.height as i32)
}

/// Get current cursor position in screen coordinates (top-left origin)
/// Note: CGEvent.location() already returns screen coordinates, no flip needed
pub fn current_cursor_position() -> Option<(i32, i32)> {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).ok()?;
    let event = CGEvent::new(source).ok()?;
    let loc = event.location();
    // CGEvent returns screen coordinates directly (Y=0 at top-left)
    Some((loc.x as i32, loc.y as i32))
}

#[inline]
pub fn get_cursor_pos() -> (i32, i32) {
    current_cursor_position().unwrap_or((0, 0))
}

fn create_mouse_event(
    event_type: CGEventType,
    point: CGPoint,
    button: CGMouseButton,
) -> Option<CGEvent> {
    CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .ok()
        .and_then(|source| CGEvent::new_mouse_event(source, event_type, point, button).ok())
}

#[inline]
pub fn move_mouse(x: i32, y: i32) {
    eprintln!("[MOUSE] move_mouse to ({}, {})", x, y);
    let point = CGPoint::new(x as f64, y as f64);
    if let Some(event) = create_mouse_event(CGEventType::MouseMoved, point, CGMouseButton::Left) {
        let _ = event.post(CGEventTapLocation::HID);
    }
}

pub fn send_clicks(
    button: i32,
    count: usize,
    hold_ms: u32,
    use_double_click_gap: bool,
    double_click_delay_ms: u32,
    running: &Arc<AtomicBool>,
) {
    eprintln!(
        "[MOUSE] send_clicks: button={}, count={}, hold_ms={}",
        button, count, hold_ms
    );
    if count == 0 {
        eprintln!("[MOUSE] send_clicks early return: count=0");
        return;
    }

    let mouse_button = match button {
        0 => CGMouseButton::Left,
        1 => CGMouseButton::Right,
        _ => CGMouseButton::Left,
    };
    let down_event = match button {
        0 => CGEventType::LeftMouseDown,
        1 => CGEventType::RightMouseDown,
        _ => CGEventType::OtherMouseDown,
    };
    let up_event = match button {
        0 => CGEventType::LeftMouseUp,
        1 => CGEventType::RightMouseUp,
        _ => CGEventType::OtherMouseUp,
    };

    let loc = current_cursor_position().unwrap_or((0, 0));
    let point = CGPoint::new(loc.0 as f64, loc.1 as f64);
    DEV_LOGGER.log(
        "MOUSE",
        &format!(
            "send_clicks: button={}, count={}, loc=({}, {})",
            button, count, loc.0, loc.1
        ),
    );
    DEV_LOGGER.log(
        "MOUSE",
        &format!(
            "  down_event={:?}, up_event={:?}, mouse_button={:?}",
            down_event, up_event, mouse_button
        ),
    );

    for index in 0..count {
        if !running.load(Ordering::SeqCst) {
            return;
        }

        if let Some(event) = create_mouse_event(down_event, point, mouse_button) {
            eprintln!("[MOUSE] Posted mouse DOWN event");
            let _ = event.post(CGEventTapLocation::HID);
        } else {
            DEV_LOGGER.log("MOUSE", "Failed to create mouse DOWN event");
            eprintln!("[MOUSE] Failed to create mouse DOWN event");
        }

        // Let OS catch up (required on macOS per rusty-autoclicker)
        std::thread::sleep(Duration::from_millis(10));

        if hold_ms > 0 {
            std::thread::sleep(Duration::from_millis(hold_ms as u64));
        }

        if let Some(event) = create_mouse_event(up_event, point, mouse_button) {
            eprintln!("[MOUSE] Posted mouse UP event");
            let _ = event.post(CGEventTapLocation::HID);
        } else {
            DEV_LOGGER.log("MOUSE", "Failed to create mouse UP event");
            eprintln!("[MOUSE] Failed to create mouse UP event");
        }

        // Let OS catch up after release too
        std::thread::sleep(Duration::from_millis(10));

        if index + 1 < count && use_double_click_gap && double_click_delay_ms > 0 {
            std::thread::sleep(Duration::from_millis(double_click_delay_ms as u64));
        }
    }
}

#[inline]
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

#[inline]
pub fn cubic_bezier(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    let u = 1.0 - t;
    u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}

pub fn smooth_move(
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    duration_ms: u64,
    _rng: &mut FastRng,
) {
    if duration_ms < 5 {
        move_mouse(end_x, end_y);
        return;
    }

    let (sx, sy) = (start_x as f64, start_y as f64);
    let (ex, ey) = (end_x as f64, end_y as f64);
    let (dx, dy) = (ex - sx, ey - sy);
    let distance = (dx * dx + dy * dy).sqrt();
    if distance < 1.0 {
        return;
    }

    let steps = (duration_ms as usize).clamp(10, 200);
    let step_dur = Duration::from_millis(duration_ms / steps as u64);

    for i in 0..=steps {
        let t = ease_in_out_quad(i as f64 / steps as f64);
        move_mouse(
            cubic_bezier(t, sx, sx + dx * 0.33, ex - dx * 0.33, ex) as i32,
            cubic_bezier(t, sy, sy + dy * 0.33, ey - dy * 0.33, ey) as i32,
        );
        if i < steps {
            std::thread::sleep(step_dur);
        }
    }
}
