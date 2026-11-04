# Aurora Desktop Environment

Aurora is a modern, ultra-low RAM desktop environment written in **pure Rust** for Wayland. It blends the clean ergonomics of **macOS** with **Google's Material Design 3 (M3)** tonal system and Hyprland-style **Fibonacci Dwindle Tiling**.

---

## Key Features

1. **Material Design 3 (M3) System**:
   - Dynamic M3 tonal palettes (Primary, Secondary, Surface, Surface Container Low/Medium/High/Highest).
   - Elevated pill cards, toggle chips, and sliders.
   - Non-glassy, crisp UI styling with rounded corners (16px / 24px / 28px).
   - Built-in presets: `aurora_blue`, `emerald`, `purple`, `sakura`, plus light/dark mode.

2. **macOS-like Ergonomics**:
   - Clean top menu bar with app title, workspace pagers (1-9), system tray metrics, and quick settings trigger.
   - Elevated bottom floating dock with running application dot indicators and launch actions.
   - Window decorations with macOS traffic lights (Red/Yellow/Green window action controls).
   - Spotlight-style fuzzy application launcher (`Super+Space`).

3. **Hybrid Window Manager (Dwindle & Floating)**:
   - Full **Fibonacci Dwindle Tiling** engine with spiral / alternating splits.
   - Dynamic gap controls (inner & outer gaps) and split ratio configuration.
   - Switchable to free-form **Floating** mode on the fly (`Super+T` or via Settings app).
   - Multi-workspace management (Workspaces 1 through 9).

4. **Complete M3 Settings Application**:
   - Graphical settings app to customize window manager rules, dwindle vs floating mode, gaps, and borders.
   - Keybindings management table.
   - M3 appearance and dynamic color scheme selector.
   - Dock size, autohide, and position customization.
   - System resource monitor.

5. **Ultra Low RAM Footprint**:
   - Entire core compositor and shell consume **under ~20MB of RAM**.
   - Zero-copy Rust architecture, blazingly fast startup times (<10ms).

6. **Rich Super / Mod4 Keybindings**:
   - `Super + Space`: Spotlight App Launcher
   - `Super + Return`: Terminal Emulator
   - `Super + Q`: Close focused window
   - `Super + T`: Toggle between Dwindle tiling and Floating mode
   - `Super + F`: Toggle fullscreen
   - `Super + C`: Open Control Center Quick Settings
   - `Super + [1-9]`: Switch to Workspace 1-9
   - `Super + Shift + [1-9]`: Move active window to workspace 1-9
   - `Super + Arrow Keys` or `Super + [h/j/k/l]`: Navigate window focus

7. **CLI Control (`aurora-msg`)**:
   - Script and control any DE feature via IPC:
     ```bash
     aurora-msg get-state
     aurora-msg toggle-layout
     aurora-msg workspace 2
     aurora-msg gaps 12 16
     ```

---

## Workspace Architecture

- `crates/aurora-config`: TOML configuration loader, schema definition, and persistent settings (`~/.config/aurora/aurora.toml`).
- `crates/aurora-theme`: Material 3 design tokens, tonal elevations, and color math.
- `crates/aurora-ipc`: Unix domain socket IPC protocol for hot-reloading and live WM control.
- `crates/aurora-wm`: Wayland Window Manager with Dwindle layout calculation and workspace tracking.
- `crates/aurora-shell`: Desktop components: macOS-like Top Bar, Bottom Dock, Spotlight Launcher, Control Center.
- `crates/aurora-settings`: M3 Graphical Settings Application.
- `crates/aurora-session`: Session orchestrator and `aurora-msg` command line controller.

---

## Building and Running

### Build all components:
```bash
cargo build --release
```

### Launch Aurora Desktop:
```bash
./target/release/aurora-session
```

### Launch Settings:
```bash
./target/release/aurora-settings
```
