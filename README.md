# Blur Auto Clicker for macOS

> **Original Project:** [Blur Auto Clicker v3.4.1](https://github.com/Blur009/Blur-AutoClicker) by Blur009 (Windows)
> **macOS Fork:** [joy-arz/Blur-AutoClicker-MacOS](https://github.com/joy-arz/Blur-AutoClicker-MacOS)

**Current Version:** 3.2.1 (macOS fork) - Aligned with Windows v3.4.1 bug fixes

<p align="center"><em>An accuracy and performance focused auto clicker for macOS</em></p>

---

## ⚠️ About This Fork

This is a **macOS fork** of the original [Blur Auto Clicker](https://github.com/Blur009/Blur-AutoClicker) project. The original was Windows-only; this version ports it to macOS using Rust + Tauri v2 while maintaining feature parity.

### Original Project
- **Author:** [Blur009](https://github.com/Blur009)
- **Platform:** Windows
- **License:** GPL-3.0

### This macOS Fork
- **Author:** [joy-arz](https://github.com/joy-arz)
- **Platform:** macOS (12.0+)
- **License:** GPL-3.0

---

## Changelog

### What's Different in This macOS Fork

This project has diverged significantly from the original Windows version:

| Aspect | Original (Windows) | This Fork (macOS) |
|--------|---------------------|-------------------|
| **Framework** | Tauri v1 (Windows) | Tauri v2 (macOS) |
| **Frontend** | React (same) | React + TypeScript |
| **Backend** | Rust + Windows APIs | Rust + Core Graphics |
| **Mouse Events** | SendInput API | CGEvent (HID) |
| **Hotkeys** | Windows global shortcuts | tauri-plugin-global-shortcut |
| **Settings Storage** | Custom JSON | tauri-plugin-store |
| **Telemetry** | Supabase backend | Disabled |
| **Auto-Updates** | tauri-plugin-updater | Disabled |
| **UI** | Original design | React + CSS (same design) |

### Features Added (Not from Original Blur)

These features were **added during the macOS port** based on research from other autoclickers:

| Feature | Source Inspiration | Purpose |
|---------|------------------|---------|
| **positionMode Field** | opautoclicker.com | Backend support for "Current" vs "Fixed" modes |
| **Current/Fixed UI Buttons** | opautoclicker.com | User selection of position mode |
| **Dev Logger** | Best practice (custom) | File-based debugging at `~/Library/Application Support/BlurAutoClicker/logs/` |
| **Cached Display Height** | MrTanoshii/rusty-autoclicker | Prevents coordinate mismatches |
| **OS Catch-up Delays (10ms)** | inket/Autoclick, MrTanoshii | Allows macOS to process mouse events |
| **PositionMode Enum** | MrTanoshii/rusty-autoclicker | Type-safe position handling in Rust |

### Bug Fixes (Not from Original)

These bugs were **discovered and fixed during this macOS port**:

| Bug | Source | Fix |
|-----|--------|-----|
| **Position mode not working** | Original issue | Backend now properly handles `positionMode` setting |
| **Coordinate Y-axis flip** | Original issue | CGEvent returns top-left origin coordinates directly |
| **UI bug - Fixed button set to Current** | Session debugging | "Fixed" button now correctly calls `positionMode: "fixed"` |
| **Mouse events not posting** | Original issue | CGEventFlags cleared, HID tap location, batched events |
| **Synthetic event detection** | Cookie Clicker/web apps | Clear flags to reduce synthetic event detection |
| **Variation std dev calculation** | Windows comparison | Fixed to `std_dev = interval * (variation / 100.0)` |
| **Mouse button stuck at high CPS** | Issue #114 (Windows) | Ensure mouse-up events are posted on early exit |
| **Tab change logic bug** | Code review | Fixed `lastPanel` tracking on tab switch |
| **Redundant status polling** | Code review | Removed 200ms polling, now uses event-based updates |
| **Version type mismatch** | Code review | Backend now uses string version consistently |
| **Top-level await in store** | Code review | Replaced with lazy initialization with fallback |
| **Rust telemetry on close** | Code review | Now uses actual settings from state, not defaults |
| **Multi-monitor failsafe** | [@0ywfe](https://github.com/0ywfe) | Use `CGDisplay::active_displays()` for proper multi-monitor support |
| **serde alias for corner fields** | [@0ywfe](https://github.com/0ywfe) | Fix `cornerStopTL` → `cornerStopTl` deserialization issue |

### Contributors

Thanks to our contributors:
- [@0ywfe](https://github.com/0ywfe) - Multi-monitor failsafe fix, serde aliases for settings

### v3.3.0 Alignment (Windows → macOS)

Changes from Windows v3.3.0 implemented in macOS fork:

| Feature | Windows v3.3.0 | macOS Status |
|---------|----------------|--------------|
| **Batched mouse events** | `send_batch()` for high CPS | ✅ Implemented |
| **Speed variation fix** | `std_dev = interval * (variation / 100.0)` | ✅ Implemented |
| **Duty cycle in simple mode** | Randomization and Duty cycle in simple mode | ✅ Already present |
| **UI icons instead of text** | Icons for top bar | Partial - uses existing UI |
| **Scrollbar for settings** | Settings page scrollbar | ✅ Already present |

### macOS-Specific Implementation Details

#### CGEvent Tap Location
- Uses `CGEventTapLocation::HID` for posting mouse events directly to HID system
- Alternative `Session` tap location available but HID proved more reliable

#### Event Flags
- Events are created with `CGEventFlags::empty()` to clear synthetic flags
- This helps events appear more like real hardware input

#### Coordinate System
- CGEvent returns screen coordinates with Y=0 at top-left (native macOS)
- No Y-axis transformation needed for Core Graphics events

#### Event Creation Flow
```
CGEventSource → CGEvent::new_mouse_event() → set_flags(empty) → post(HID)
```

#### Batch Event Optimization
For high CPS (>50) with no hold time and no double-click gap, events are batched:
```
Create all events upfront → Post all to HID tap
```
This matches Windows `SendInput` behavior for better performance.

### Features from Other Projects (Inspiration Only)

These features **inspired** improvements but were implemented differently:

- **inket/Autoclick** - Simpler codebase approach, higher CPS limit (900)
- **othyn/macos-auto-clicker** - SwiftUI patterns, keyboard pressing, i18n
- **MrTanoshii/rusty-autoclicker** - Rust architecture patterns, mouse handling

---

## Original Features (Windows → macOS Port)

### Core Features (Ported)
- ✅ High CPS Support (500+ CPS sustained)
- ✅ Toggle/Hold activation modes
- ✅ Multiple mouse buttons (Left, Middle, Right)
- ✅ Current/Fixed Position modes
- ✅ Double-click support
- ✅ Duty cycle control (0-100%)
- ✅ Speed variation (randomized CPS)
- ✅ Edge stops (top, right, bottom, left)
- ✅ Corner stops (TL, TR, BL, BR)
- ✅ Click limit
- ✅ Time limit
- ✅ Global hotkeys
- ✅ Settings persistence

### Features Not Yet Ported
- ❌ Visual overlay for edge/corner zones
- ❌ Human-like mouse movement mode
- ❌ Macro/sequence clicking
- ❌ Profile system

---

## Comparison with Other macOS Auto Clickers

| Feature | Blur (this fork) | inket/Autoclick | othyn/macos-auto-clicker |
|---------|------------------|------------------|--------------------------|
| **Max CPS** | 500+ | 900 | Not specified |
| **Position Modes** | Current + Fixed | Cursor + Fixed | Cursor + Fixed |
| **Mouse Buttons** | Left/Middle/Right | Left/Right | All |
| **Keyboard Pressing** | No | No | Yes |
| **Hold/Toggle Modes** | Yes | No | Yes |
| **Duty Cycle** | Yes | No | Yes |
| **Speed Variation** | Yes | No | Yes |
| **Edge/Corner Stops** | Yes | No | No |
| **Click/Time Limits** | Yes | Yes | Yes |
| **Hotkey Customization** | Yes | Limited | Yes |
| **Settings Persistence** | Yes | Yes | Yes |
| **Multiple Languages** | No | No | Yes |
| **Color Themes** | No | No | Yes |

---

## System Requirements

- **Operating System:** macOS 12.0 (Monterey) or later
- **Architecture:** Apple Silicon (M1/M2/M3) or Intel
- **Permissions:** Accessibility access required for mouse control

### Enabling Accessibility Access

This app requires **Accessibility access** to control the mouse and simulate clicks. Without it, the clicker will not function.

**To grant Accessibility access:**
1. Open **System Settings** → **Privacy & Security** → **Accessibility**
2. Click the lock icon to make changes (enter your password if prompted)
3. Click **+** and add `BlurAutoClicker` from Applications
4. Ensure the toggle next to `BlurAutoClicker` is turned **ON**

If the app doesn't appear in the list, browse to:
`/Applications/BlurAutoClicker.app`

---

## Installation

### DMG Installer
1. Download `.dmg` from [Releases](https://github.com/joy-arz/Blur-AutoClicker-MacOS/releases)
2. Open the `.dmg` file
3. Drag `BlurAutoClicker.app` to Applications
4. Right-click → Open (first time only to bypass gatekeeper)
5. Grant Accessibility permission when prompted

### Manual Build
```bash
git clone https://github.com/joy-arz/Blur-AutoClicker-MacOS.git
cd Blur-AutoClicker-MacOS
npm install
npm run tauri build
```

---

## Usage

### Default Hotkey
`Ctrl+Y` (maps to Cmd+Y on Mac keyboards)

### Quick Start
1. Launch app → Simple tab
2. Set CPS (e.g., 25 clicks per second)
3. Select mouse button (Left/Right/Middle)
4. Choose mode: Toggle (start/stop) or Hold (click while held)
5. Press `Ctrl+Y` to start → Press again to stop

### Position Mode (Advanced)
- **Current Position**: Clicks wherever cursor is located
- **Fixed Position**: Clicks at specific X/Y coordinates
  - Enter coordinates manually, or
  - Click "Pick" to capture cursor position with 3-second countdown

---

## Architecture

### Technology Stack
| Layer | Technology |
|-------|------------|
| Frontend | React + TypeScript + Vite |
| Desktop | Tauri v2 |
| Backend | Rust |
| Mouse Events | Core Graphics (CGEvent) |
| Hotkeys | tauri-plugin-global-shortcut |
| Storage | tauri-plugin-store |

### Key Files
```
src-tauri/src/
├── engine/
│   ├── worker.rs      # Main click loop
│   ├── mouse.rs       # Mouse event generation (CGEvent HID)
│   ├── failsafe.rs   # Edge/corner detection (top-left origin)
│   ├── stats.rs      # Statistics tracking
│   ├── rng.rs        # Fast random number generator
│   └── mod.rs        # ClickerConfig + PositionMode
├── hotkeys.rs         # Global shortcut handling
├── ui_commands.rs     # Tauri IPC commands
├── settings/
│   └── mod.rs         # ClickerSettings struct
├── app_state.rs       # Runtime state management
└── dev_logger.rs      # File-based debug logging

src/
├── store.ts           # Frontend settings state
├── App.tsx            # Main React component
├── hotkeys.ts         # Keyboard utilities
└── components/panels/ # UI panels (Simple, Advanced, Settings)
```

### Debug Logging
Logs are written to `~/Library/Application Support/BlurAutoClicker/logs/`
- Filename format: `blur_autoclicker_{timestamp}.log`
- Enable by building in debug mode or checking logs for issues

---

## Roadmap

- [x] Position mode selection UI (Current/Fixed)
- [x] OS catch-up delays for reliability
- [x] File-based debug logging
- [x] Proper CGEvent flag clearing for synthetic event handling
- [x] Batched mouse event sending for high CPS
- [x] Speed variation calculation aligned with Windows
- [ ] Multi-monitor support
- [ ] Human-like mode (smooth cursor movement)
- [ ] Visual overlay for edge/corner zones
- [ ] Profile system (save/load configurations)
- [ ] Keyboard key pressing (like othyn's app)
- [ ] CPU usage measurement (using mach_thread_time on macOS)

---

## License

GPL-3.0 - Same as original Blur Auto Clicker

---

## References

- [Original Blur Auto Clicker](https://github.com/Blur009/Blur-AutoClicker) - Windows version
- [inket/Autoclick](https://github.com/inket/Autoclick) - Reference for macOS native approach
- [othyn/macos-auto-clicker](https://github.com/othyn/macos-auto-clicker) - SwiftUI reference
- [MrTanoshii/rusty-autoclicker](https://github.com/MrTanoshii/rusty-autoclicker) - Rust cross-platform reference
- [opautoclicker.com](https://www.opautoclicker.com/) - Feature reference

---

*This is an unofficial macOS port. The original Blur Auto Clicker was designed for Windows only.*