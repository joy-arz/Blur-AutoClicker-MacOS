use core_graphics::display::CGDisplay;
use core_graphics::event::CGEvent;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use super::ClickerConfig;

/// Returns (local_x, local_y, display_width, display_height) for the display
/// that currently contains the cursor. Converts global CGEvent coords to
/// display-local coords so failsafe comparisons work on any monitor.
fn cursor_in_local_display() -> Option<(i32, i32, i32, i32)> {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).ok()?;
    let event = CGEvent::new(source).ok()?;
    let loc = event.location();
    let cx = loc.x;
    let cy = loc.y;

    if let Ok(displays) = CGDisplay::active_displays() {
        for &id in &displays {
            let bounds = CGDisplay::new(id).bounds();
            let ox = bounds.origin.x;
            let oy = bounds.origin.y;
            let sw = bounds.size.width;
            let sh = bounds.size.height;

            if cx >= ox && cx < ox + sw && cy >= oy && cy < oy + sh {
                return Some((
                    (cx - ox) as i32,
                    (cy - oy) as i32,
                    sw as i32,
                    sh as i32,
                ));
            }
        }
    }

    // Fallback: treat cursor as being on the main display
    let bounds = CGDisplay::main().bounds();
    Some((cx as i32, cy as i32, bounds.size.width as i32, bounds.size.height as i32))
}

pub fn should_stop_for_failsafe(config: &ClickerConfig) -> Option<String> {
    let (cursor_x, cursor_y, screen_w, screen_h) = cursor_in_local_display()?;

    if config.corner_stop_enabled {
        if cursor_x <= config.corner_stop_tl && cursor_y <= config.corner_stop_tl {
            return Some(String::from("Top-left corner failsafe"));
        }
        if cursor_x >= screen_w - config.corner_stop_tr && cursor_y <= config.corner_stop_tr {
            return Some(String::from("Top-right corner failsafe"));
        }
        if cursor_x <= config.corner_stop_bl && cursor_y >= screen_h - config.corner_stop_bl {
            return Some(String::from("Bottom-left corner failsafe"));
        }
        if cursor_x >= screen_w - config.corner_stop_br
            && cursor_y >= screen_h - config.corner_stop_br
        {
            return Some(String::from("Bottom-right corner failsafe"));
        }
    }

    if config.edge_stop_enabled {
        if cursor_y <= config.edge_stop_top {
            return Some(String::from("Top edge failsafe"));
        }
        if cursor_x >= screen_w - config.edge_stop_right {
            return Some(String::from("Right edge failsafe"));
        }
        if cursor_y >= screen_h - config.edge_stop_bottom {
            return Some(String::from("Bottom edge failsafe"));
        }
        if cursor_x <= config.edge_stop_left {
            return Some(String::from("Left edge failsafe"));
        }
    }

    None
}
