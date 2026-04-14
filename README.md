# Blur Auto Clicker for macOS

<div align="center">
    <img src="https://github.com/joy-arz/Blur-AutoClicker-MacOS/blob/main/public/V3.0.0_UI.png" width="600"/>
</div>
<p align="center"><em>An accuracy and performance focused auto clicker for macOS</em></p>

## ⚠️ Development Status

**This project is under active development. The autoclicker core functionality works, but the UI position mode selection is being refined.**

## Features

### Core Features
- **High CPS Support** - Sustained 500+ CPS on modern hardware
- **Toggle/Hold Modes** - Start/stop with hotkey or hold-to-click
- **Multiple Mouse Buttons** - Left, Middle, Right click
- **Current/Fixed Position Modes** - Click at cursor or specific coordinates
- **Double Click Support** - Single, double, or triple clicks
- **Duty Cycle Control** - Adjustable mouse button hold duration
- **Speed Variation** - Randomized CPS for natural clicking
- **Edge/Corner Stops** - Failsafe when cursor approaches boundaries
- **Click/Time Limits** - Stop after N clicks or T seconds
- **Global Hotkeys** - Works in background without focusing app
- **Settings Persistence** - All preferences saved between sessions

### Simple Mode
- On/Off indicator (green when active)
- CPS adjustment (per second/minute/hour/day)
- Mouse button selection
- Hold/Toggle activation
- Custom hotkey support

### Advanced Mode
- Duty cycle (0-100% hold duration)
- Speed variation (randomized CPS range)
- Corner stop (TL, TR, BL, BR thresholds)
- Edge stop (top, right, bottom, left thresholds)
- Click limit (stop after N clicks)
- Time limit (stop after T time)
- Position mode (current cursor vs fixed coordinates)
- Pick position with visual countdown

## Comparison with Other macOS Auto Clickers

| Feature | Blur Auto Clicker | inket/Autoclick | othyn/macos-auto-clicker |
|---------|-------------------|-----------------|--------------------------|
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
| **Native UI** | React/Web | AppKit | SwiftUI |

### What Blur Auto Clicker Does Better
1. **Accuracy at High CPS** - Engineered for precision at 500+ CPS
2. **Edge/Corner Stops** - Built-in failsafes for safety
3. **Duty Cycle Control** - Fine-tune mouse button hold duration
4. **Speed Variation** - Natural randomization of click timing
5. **Rust Backend** - Memory-safe, performant core

### What Others Do Better
1. **inket/Autoclick** - Simpler codebase, higher max CPS (900)
2. **othyn/macos-auto-clicker** - SwiftUI native, keyboard pressing, i18n, theming

## System Requirements

- **Operating System:** macOS 12.0 (Monterey) or later
- **Architecture:** Apple Silicon (M1/M2/M3) or Intel
- **Permissions:** Accessibility (for mouse control)

## Installation

### DMG Installer (Recommended)
1. Download `.dmg` from [Releases](https://github.com/joy-arz/Blur-AutoClicker-MacOS/releases)
2. Open the `.dmg` file
3. Drag `BlurAutoClicker.app` to Applications
4. Right-click → Open (first time only)
5. Grant Accessibility permission when prompted

### Manual Build
```bash
git clone https://github.com/joy-arz/Blur-AutoClicker-MacOS.git
cd Blur-AutoClicker-MacOS
npm install
npm run tauri build
```

## Usage

### Default Hotkey
`Ctrl+Y` (maps to Cmd+Y on Mac keyboards)

### Quick Start
1. Launch app → Simple tab
2. Set CPS (e.g., 25 clicks per second)
3. Select mouse button (Left/Right/Middle)
4. Choose mode: Toggle (start/stop) or Hold (click while held)
5. Press `Ctrl+Y` to start → Press again to stop

### Position Mode
- **Current Position**: Clicks wherever cursor is located
- **Fixed Position**: Clicks at specific X/Y coordinates
  - Enter coordinates manually, or
  - Click "Pick" to capture cursor position with 3-second countdown

### Advanced Failsafes
- **Edge Stop**: App stops if cursor within N pixels of screen edge
- **Corner Stop**: App stops if cursor enters NxN corner zone
- Both can be configured independently per edge/corner

## Architecture

### Technology Stack
| Layer | Technology |
|-------|------------|
| Frontend | React + TypeScript + Vite |
| Desktop | Tauri v2 |
| Backend | Rust |
| Mouse Events | Core Graphics |
| Hotkeys | tauri-plugin-global-shortcut |
| Storage | tauri-plugin-store |

### Key Files
```
src-tauri/src/
├── engine/
│   ├── worker.rs      # Main click loop
│   ├── mouse.rs        # Mouse event generation (CGEvent)
│   ├── failsafe.rs     # Edge/corner detection
│   ├── stats.rs        # Statistics tracking
│   └── mod.rs          # Config types
├── hotkeys.rs          # Global shortcut handling
├── ui_commands.rs      # Tauri IPC commands
├── settings/
│   └── mod.rs          # ClickerSettings struct
└── dev_logger.rs       # Debug file logging

src/
├── store.ts            # Frontend settings state
├── App.tsx             # Main React component
└── components/panels/  # UI panels (Simple, Advanced)
```

## Recent Changes

### v3.2.1 (Latest)
- Fixed position mode selection UI (Current vs Fixed buttons)
- Added OS catch-up delays (10ms) after mouse events for reliability
- Added dev_logger for file-based debugging
- Implemented proper position_mode field in backend
- Cached display height to prevent coordinate mismatches

### v3.2.0
- Initial macOS release with Tauri v2
- React frontend with original UI design
- Rust backend with Core Graphics mouse events

## Known Issues

1. **UI Position Mode Selection** - Buttons added but styling may need adjustment
2. **Multi-monitor** - Coordinate system may not handle multiple displays correctly
3. **Accessibility Prompt** - May require restart after granting permissions

## Roadmap

- [ ] Fix and test position mode selection UI
- [ ] Multi-monitor support with proper coordinate handling
- [ ] Human-like mode (smooth mouse movement between clicks)
- [ ] Profile system (save/load different configurations)
- [ ] Visual overlay for edge/corner stop zones
- [ ] Better error handling and user feedback

## License

GPL-3.0 - Same as original Blur Auto Clicker

## Credits

- **Original Project**: [Blur009/Blur-AutoClicker](https://github.com/Blur009/Blur-AutoClicker) (Windows)
- **macOS Port**: [joy-arz/Blur-AutoClicker-MacOS](https://github.com/joy-arz/Blur-AutoClicker-MacOS)
- **Built with**: [Tauri v2](https://tauri.app/)

## References

- [inket/Autoclick](https://github.com/inket/Autoclick) - Reference for macOS native approach
- [othyn/macos-auto-clicker](https://github.com/othyn/macos-auto-clicker) - Reference for SwiftUI/macOS integration
- [MrTanoshii/rusty-autoclicker](https://github.com/MrTanoshii/rusty-autoclicker) - Rust cross-platform reference
- [opautoclicker.com](https://www.opautoclicker.com/) - Feature reference

---

*Note: This is an unofficial macOS port of the original Blur Auto Clicker for Windows.*