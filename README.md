# Blur Auto Clicker for macOS

<div align="center">
    <img src="https://github.com/Blur009/Blur-AutoClicker/blob/main/public/V3.0.0_UI.png" width="600"/>
</div>
<p align="center"><em>An accuracy and performance focused auto clicker</em></p>

## ⚠️ Fork Notice

This is an **unofficial macOS fork** of the original [Blur Auto Clicker](https://github.com/Blur009/Blur-AutoClicker) project. This version ports the application to macOS while maintaining feature parity with the Windows version.

**Original Project:** https://github.com/Blur009/Blur-AutoClicker
**This Repository:** https://github.com/joy-arz/Blur-AutoClicker-MacOS

## Why macOS Fork?

The original Blur Auto Clicker was designed exclusively for Windows. This fork brings the same accuracy, performance, and features to macOS users. The application is rebuilt from the ground up using macOS-compatible APIs while preserving the original UI design and functionality.

## Features

<div align="center">
    <img src="https://github.com/Blur009/Blur-AutoClicker/blob/main/public/30s_500cps_Speed_Test.png" width="600"/>
</div>
<p align="center"><em>Blur Auto Clicker reaching 500 CPS steadily</em></p>

### Simple Mode
- On / Off Indicator (blur logo turns green when active)
- Individual mouse button settings (left, right, middle)
- Hold / Toggle activation modes
- Customizable hotkeys

### Advanced Mode (includes all simple mode features plus)
- Adjustable click timing (duty cycle)
- Speed Range Mode (randomizes CPS within a range)
- Corner and edge stopping (turns off when mouse is in corners or near edges)
- Click and Time limits (stop after certain amount of clicks or time)
- Double clicks
- Position Clicking (pick a position where the mouse will move to and click)
- CPS can be adjusted to per Second, Minute, Hour, or Day

### Other Features
- Click stats (total clicks, clicks per second, etc)
- Multiple mode tabs (Simple, Advanced, Macro, Settings)

## System Requirements

- **Operating System:** macOS 10.15 (Catalina) or later
- **Architecture:** Apple Silicon (M1/M2/M3) or Intel

## Installation

### DMG Installer (Recommended)
1. Download the latest `.dmg` file from the [Releases](https://github.com/joy-arz/Blur-AutoClicker-MacOS/releases) page
2. Open the `.dmg` file
3. Drag `BlurAutoClicker.app` to your Applications folder
4. Launch from Applications (you may need to allow it in System Preferences > Privacy & Security)

### Manual Build
```bash
# Clone the repository
git clone https://github.com/joy-arz/Blur-AutoClicker-MacOS.git

# Navigate to project directory
cd Blur-AutoClicker-MacOS

# Install dependencies
npm install

# Build for macOS
npm run tauri build
```

The built application will be at:
```
src-tauri/target/release/bundle/macos/BlurAutoClicker.app
```

## Usage

### Default Hotkey
The default hotkey is `Ctrl+Y` (or `Cmd+Y` on macOS keyboards - the key maps to the same physical key).

### Basic Setup
1. Launch BlurAutoClicker
2. Select your preferred mode (Simple or Advanced)
3. Set your desired clicks per second (CPS)
4. Choose your mouse button (Left, Middle, or Right)
5. Choose activation mode:
   - **Toggle**: Press hotkey once to start, press again to stop
   - **Hold**: Hold hotkey to click, release to stop
6. Press the hotkey to start clicking!

### Advanced Options
- **Duty Cycle**: How long the mouse button is held down (0-100%)
- **Speed Variation**: Randomizes CPS within a range for natural clicking
- **Edge Stop**: Stops clicking when cursor approaches screen edges
- **Corner Stop**: Stops clicking when cursor is in screen corners
- **Click Limit**: Stop after a certain number of clicks
- **Time Limit**: Stop after a certain duration
- **Double Click**: Enable double-click mode
- **Position Mode**: Click at a fixed screen position

## Architecture

### Technology Stack
- **Frontend:** React + TypeScript + Vite
- **Backend:** Rust + Tauri v2
- **Mouse Events:** Core Graphics framework (macOS native)
- **Global Shortcuts:** tauri-plugin-global-shortcut

### Key Differences from Windows Version
| Feature | Windows | macOS |
|---------|---------|-------|
| Mouse Events | SendInput API | Core Graphics |
| Global Hotkeys | Windows API | tauri-plugin-global-shortcut |
| Telemetry | Supabase backend | Disabled |
| Auto-Updates | tauri-plugin-updater | Not yet implemented |

### Project Structure
```
Blur-AutoClicker-MacOS/
├── src/                    # React frontend
│   ├── components/          # UI components
│   │   └── panels/         # Mode panels (Simple, Advanced, etc.)
│   ├── App.tsx             # Main app component
│   ├── store.ts            # Settings state management
│   └── hotkeys.ts         # Hotkey utilities
├── src-tauri/              # Rust backend
│   └── src/
│       ├── engine/         # Click engine core
│       │   ├── worker.rs   # Main click loop
│       │   ├── mouse.rs    # Mouse event generation
│       │   ├── failsafe.rs # Edge/corner detection
│       │   ├── stats.rs    # Statistics tracking
│       │   └── rng.rs      # Random number generation
│       ├── settings/       # Settings management
│       ├── hotkeys.rs      # Hotkey handling
│       ├── telemetry.rs    # Telemetry (disabled)
│       ├── ui_commands.rs  # Tauri commands
│       └── updates/       # Update checker (disabled)
└── README.md
```

## Performance

Like the Windows version, this macOS fork maintains high accuracy at high CPS rates:
- Sustained 500+ CPS on modern hardware
- Low CPU usage (~1% average during use)
- ~50MB RAM usage

## Known Limitations

1. **Accessibility Permissions Required:** The app needs Accessibility permissions to control the mouse. macOS will prompt you to grant this on first launch.

2. **Auto-Updates:** Not yet implemented on macOS. You'll need to download new versions manually.

3. **Telemetry:** Telemetry collection is disabled on macOS (no backend configured).

4. **Screen Overlay:** The visual overlay showing edge/corner stop zones (from the Windows version) is not yet implemented.

## Syncing with Upstream

To pull updates from the original Windows repository:
```bash
git fetch upstream
git merge upstream/main  # or appropriate branch
```

## License

This project is licensed under [GPL v3](https://www.gnu.org/licenses/gpl-3.0.en.html#license-text) - Same as the original project.

## Contributing

Contributions are welcome! This is an unofficial port, so please keep in mind:

1. Maintain compatibility with the original Windows version's feature set
2. Use native macOS APIs where possible
3. Test thoroughly on both Apple Silicon and Intel Macs

## Acknowledgments

- Original project by [Blur009](https://github.com/Blur009/Blur-AutoClicker)
- Built with [Tauri v2](https://tauri.app/)
- Inspired by the need for accurate auto-clickers at high CPS rates

## Support

For issues specific to this macOS fork, please open an issue on [this repository](https://github.com/joy-arz/Blur-AutoClicker-MacOS/issues).

For the original Windows version and general auto-clicker questions, visit the [main repository](https://github.com/Blur009/Blur-AutoClicker).

---

*Note: This is not affiliated with or endorsed by the original Blur Auto Clicker author.*