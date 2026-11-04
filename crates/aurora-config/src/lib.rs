use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AuroraConfig {
    pub window_manager: WindowManagerConfig,
    pub theme: ThemeConfig,
    pub dock: DockConfig,
    pub top_bar: TopBarConfig,
    pub launcher: LauncherConfig,
    pub keybindings: KeybindingsConfig,
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct WindowManagerConfig {
    /// Window management mode: "dwindle" (tiling) or "floating"
    pub layout_mode: LayoutMode,
    /// Inner gaps between windows in pixels
    pub inner_gaps: u32,
    /// Outer gaps from screen edge in pixels
    pub outer_gaps: u32,
    /// Border width in pixels
    pub border_width: u32,
    /// Active border color hex
    pub active_border_color: String,
    /// Inactive border color hex
    pub inactive_border_color: String,
    /// Corner radius in pixels for M3 window curves
    pub corner_radius: u32,
    /// Dwindle split ratio (e.g. 0.5 for equal split)
    pub split_ratio: f32,
    /// Spiral dwindle mode (true = clockwise spiral, false = alternate horizontal/vertical)
    pub spiral: bool,
    /// Focus follows mouse cursor
    pub focus_follows_mouse: bool,
    /// macOS style traffic light buttons on window frames
    pub show_traffic_lights: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LayoutMode {
    Dwindle,
    Floating,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ThemeConfig {
    /// "dark" or "light"
    pub mode: String,
    /// M3 Accent tone seed or preset name ("aurora_blue", "emerald", "sakura", "amber", "purple")
    pub palette_preset: String,
    /// Custom seed hex color for M3 dynamic tonal palette generation (optional)
    pub primary_seed: String,
    /// Wallpaper path or color
    pub wallpaper: String,
    /// Font family for the desktop environment
    pub font_family: String,
    pub font_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DockConfig {
    pub enabled: bool,
    pub position: String, // "bottom", "left", "right"
    pub icon_size: u32,
    pub autohide: bool,
    pub magnification: bool,
    pub pinned_apps: Vec<PinnedApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PinnedApp {
    pub name: String,
    pub exec: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct TopBarConfig {
    pub enabled: bool,
    pub height: u32,
    pub show_workspaces: bool,
    pub show_clock: bool,
    pub clock_format: String,
    pub show_quick_settings: bool,
    pub show_battery: bool,
    pub show_system_stats: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LauncherConfig {
    pub width: u32,
    pub height: u32,
    pub search_placeholder: String,
    pub max_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct KeybindingsConfig {
    /// Map of key combination to action string
    pub bindings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GeneralConfig {
    pub session_name: String,
    pub terminal_emulator: String,
    pub file_manager: String,
    pub browser: String,
    pub low_ram_mode: bool,
}

impl Default for WindowManagerConfig {
    fn default() -> Self {
        Self {
            layout_mode: LayoutMode::Dwindle,
            inner_gaps: 8,
            outer_gaps: 12,
            border_width: 2,
            active_border_color: "#A8C7FA".to_string(), // M3 Primary container / accent
            inactive_border_color: "#444746".to_string(), // M3 Outline variant
            corner_radius: 12, // Material 3 Rounded corner
            split_ratio: 0.5,
            spiral: true,
            focus_follows_mouse: true,
            show_traffic_lights: true,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: "dark".to_string(),
            palette_preset: "aurora_blue".to_string(),
            primary_seed: "#A8C7FA".to_string(),
            wallpaper: "default_aurora_m3".to_string(),
            font_family: "Roboto, Cantarell, Sans-Serif".to_string(),
            font_size: 13,
        }
    }
}

impl Default for DockConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            position: "bottom".to_string(),
            icon_size: 48,
            autohide: false,
            magnification: true,
            pinned_apps: vec![
                PinnedApp {
                    name: "Terminal".to_string(),
                    exec: "alacritty || foot || kitty || xterm".to_string(),
                    icon: "utilities-terminal".to_string(),
                },
                PinnedApp {
                    name: "Web Browser".to_string(),
                    exec: "firefox || chromium || google-chrome".to_string(),
                    icon: "web-browser".to_string(),
                },
                PinnedApp {
                    name: "Files".to_string(),
                    exec: "nautilus || thunar || pcmanfm".to_string(),
                    icon: "system-file-manager".to_string(),
                },
                PinnedApp {
                    name: "Settings".to_string(),
                    exec: "aurora-settings".to_string(),
                    icon: "preferences-system".to_string(),
                },
            ],
        }
    }
}

impl Default for TopBarConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            height: 32,
            show_workspaces: true,
            show_clock: true,
            clock_format: "%a %b %e  %H:%M".to_string(), // macOS style "Wed Oct 24  09:41"
            show_quick_settings: true,
            show_battery: true,
            show_system_stats: true,
        }
    }
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            width: 620,
            height: 380,
            search_placeholder: "Search applications, files, and commands...".to_string(),
            max_results: 8,
        }
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        // Application Launching
        bindings.insert("Super+Return".to_string(), "terminal".to_string());
        bindings.insert("Super+Space".to_string(), "launcher".to_string());
        bindings.insert("Super+E".to_string(), "files".to_string());
        bindings.insert("Super+B".to_string(), "browser".to_string());
        bindings.insert("Super+I".to_string(), "settings".to_string());

        // Window Control (macOS style + dwindle window management)
        bindings.insert("Super+Q".to_string(), "close_window".to_string());
        bindings.insert("Super+F".to_string(), "toggle_fullscreen".to_string());
        bindings.insert("Super+T".to_string(), "toggle_tiling_mode".to_string());
        bindings.insert("Super+M".to_string(), "minimize_window".to_string());

        // Focus navigation
        bindings.insert("Super+Left".to_string(), "focus_left".to_string());
        bindings.insert("Super+Right".to_string(), "focus_right".to_string());
        bindings.insert("Super+Up".to_string(), "focus_up".to_string());
        bindings.insert("Super+Down".to_string(), "focus_down".to_string());
        bindings.insert("Super+h".to_string(), "focus_left".to_string());
        bindings.insert("Super+l".to_string(), "focus_right".to_string());
        bindings.insert("Super+k".to_string(), "focus_up".to_string());
        bindings.insert("Super+j".to_string(), "focus_down".to_string());

        // Window Swap / Move
        bindings.insert("Super+Shift+Left".to_string(), "swap_left".to_string());
        bindings.insert("Super+Shift+Right".to_string(), "swap_right".to_string());
        bindings.insert("Super+Shift+Up".to_string(), "swap_up".to_string());
        bindings.insert("Super+Shift+Down".to_string(), "swap_down".to_string());

        // Workspaces 1-9
        for i in 1..=9 {
            bindings.insert(format!("Super+{}", i), format!("workspace_{}", i));
            bindings.insert(format!("Super+Shift+{}", i), format!("move_to_workspace_{}", i));
        }

        // Quick Toggles & Control Center
        bindings.insert("Super+C".to_string(), "toggle_control_center".to_string());
        bindings.insert("Super+L".to_string(), "lock_screen".to_string());
        bindings.insert("Super+Escape".to_string(), "session_menu".to_string());

        Self { bindings }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            session_name: "Aurora M3 Desktop".to_string(),
            terminal_emulator: "alacritty".to_string(),
            file_manager: "nautilus".to_string(),
            browser: "firefox".to_string(),
            low_ram_mode: true,
        }
    }
}

impl Default for AuroraConfig {
    fn default() -> Self {
        Self {
            window_manager: WindowManagerConfig::default(),
            theme: ThemeConfig::default(),
            dock: DockConfig::default(),
            top_bar: TopBarConfig::default(),
            launcher: LauncherConfig::default(),
            keybindings: KeybindingsConfig::default(),
            general: GeneralConfig::default(),
        }
    }
}

impl AuroraConfig {
    pub fn config_path() -> PathBuf {
        if let Some(mut path) = dirs::config_dir() {
            path.push("aurora");
            path.push("aurora.toml");
            path
        } else {
            PathBuf::from("aurora.toml")
        }
    }

    pub fn load_or_default() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(cfg) => return cfg,
                    Err(e) => {
                        eprintln!("Failed to parse config at {:?}: {}, using defaults", path, e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read config at {:?}: {}, using defaults", path, e);
                }
            }
        }
        let default_cfg = Self::default();
        let _ = default_cfg.save_to_file(&path);
        default_cfg
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = toml::to_string_pretty(self)?;
        std::fs::write(path, text)?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to_file(&Self::config_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_roundtrip() {
        let default_cfg = AuroraConfig::default();
        let serialized = toml::to_string_pretty(&default_cfg).expect("serialize");
        let parsed: AuroraConfig = toml::from_str(&serialized).expect("deserialize");
        assert_eq!(default_cfg, parsed);
        assert_eq!(parsed.window_manager.layout_mode, LayoutMode::Dwindle);
        assert!(parsed.keybindings.bindings.contains_key("Super+Space"));
        assert!(parsed.keybindings.bindings.contains_key("Super+Q"));
    }
}
