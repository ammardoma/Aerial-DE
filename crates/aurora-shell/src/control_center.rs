use aurora_config::AuroraConfig;
use aurora_theme::M3ColorScheme;
use std::fmt::Write;

pub struct ControlCenterView {
    pub config: AuroraConfig,
    pub colors: M3ColorScheme,
}

impl ControlCenterView {
    pub fn new(config: AuroraConfig, colors: M3ColorScheme) -> Self {
        Self { config, colors }
    }

    /// Render M3 macOS-like Control Center (Quick Settings Panel)
    /// Features:
    /// - M3 elevated container cards (non-glassy, tonal elevation)
    /// - Wi-Fi, Bluetooth, DND, Night Light toggles (M3 toggle chips)
    /// - M3 Volume & Display sliders
    /// - Window Manager Quick Mode Switch (Dwindle vs Floating)
    pub fn render_svg(&self, dark_mode: bool, wifi_on: bool, bt_on: bool, dwindle_mode: bool) -> String {
        let width = 360;
        let height = 420;
        let c = &self.colors;
        let mut svg = String::new();

        let _ = write!(
            svg,
            r#"<svg width="{w}" height="{h}" viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg">
  <!-- Container Card: Surface Container High with 24px corner radius -->
  <rect width="{w}" height="{h}" rx="24" fill="{bg}" stroke="{outline}" stroke-width="1.5"/>

  <!-- Header Title -->
  <text x="20" y="32" fill="{on_surface}" font-family="{font}" font-size="16" font-weight="600">Control Center</text>

  <!-- Quick Toggles Grid (Wi-Fi & Bluetooth) -->
  <g transform="translate(20, 50)">
    <!-- Wi-Fi Toggle Pill -->
    <rect x="0" y="0" width="150" height="56" rx="16" fill="{wifi_bg}"/>
    <text x="16" y="26" fill="{wifi_fg}" font-family="{font}" font-size="13" font-weight="600">Wi-Fi</text>
    <text x="16" y="42" fill="{wifi_fg}" font-family="{font}" font-size="11" opacity="0.8">{wifi_sub}</text>

    <!-- Bluetooth Toggle Pill -->
    <rect x="170" y="0" width="150" height="56" rx="16" fill="{bt_bg}"/>
    <text x="186" y="26" fill="{bt_fg}" font-family="{font}" font-size="13" font-weight="600">Bluetooth</text>
    <text x="186" y="42" fill="{bt_fg}" font-family="{font}" font-size="11" opacity="0.8">{bt_sub}</text>
  </g>

  <!-- Window Manager Mode Quick Switch Card -->
  <g transform="translate(20, 120)">
    <rect x="0" y="0" width="320" height="64" rx="16" fill="{card_bg}" stroke="{card_outline}" stroke-width="1"/>
    <text x="16" y="28" fill="{on_surface}" font-family="{font}" font-size="13" font-weight="600">Window Manager Mode</text>
    <text x="16" y="46" fill="{on_surface_var}" font-family="{font}" font-size="11">Current: {wm_mode}</text>
    <rect x="220" y="16" width="86" height="32" rx="16" fill="{btn_bg}"/>
    <text x="263" y="36" fill="{btn_fg}" font-family="{font}" font-size="11" font-weight="600" text-anchor="middle">Toggle Mode</text>
  </g>

  <!-- Volume Slider Card -->
  <g transform="translate(20, 200)">
    <rect x="0" y="0" width="320" height="60" rx="16" fill="{card_bg}" stroke="{card_outline}" stroke-width="1"/>
    <text x="16" y="24" fill="{on_surface}" font-family="{font}" font-size="12" font-weight="500">Sound Volume</text>
    <text x="286" y="24" fill="{on_surface_var}" font-family="{font}" font-size="11">75%</text>
    <!-- M3 Slider Track & Thumb -->
    <rect x="16" y="36" width="288" height="10" rx="5" fill="{slider_track}"/>
    <rect x="16" y="36" width="216" height="10" rx="5" fill="{slider_fill}"/>
    <circle cx="232" cy="41" r="9" fill="{primary}"/>
  </g>

  <!-- Display Brightness Slider Card -->
  <g transform="translate(20, 274)">
    <rect x="0" y="0" width="320" height="60" rx="16" fill="{card_bg}" stroke="{card_outline}" stroke-width="1"/>
    <text x="16" y="24" fill="{on_surface}" font-family="{font}" font-size="12" font-weight="500">Display Brightness</text>
    <text x="286" y="24" fill="{on_surface_var}" font-family="{font}" font-size="11">85%</text>
    <rect x="16" y="36" width="288" height="10" rx="5" fill="{slider_track}"/>
    <rect x="16" y="36" width="244" height="10" rx="5" fill="{slider_fill}"/>
    <circle cx="260" cy="41" r="9" fill="{primary}"/>
  </g>

  <!-- Dark Mode & Settings Shortcut Pills -->
  <g transform="translate(20, 348)">
    <rect x="0" y="0" width="150" height="48" rx="16" fill="{theme_pill_bg}"/>
    <text x="75" y="29" fill="{theme_pill_fg}" font-family="{font}" font-size="12" font-weight="600" text-anchor="middle">{theme_name}</text>

    <rect x="170" y="0" width="150" height="48" rx="16" fill="{card_bg}" stroke="{card_outline}" stroke-width="1"/>
    <text x="245" y="29" fill="{on_surface}" font-family="{font}" font-size="12" font-weight="600" text-anchor="middle">Open Settings</text>
  </g>
</svg>
"#,
            w = width,
            h = height,
            bg = c.surface_container_high,
            outline = c.outline_variant,
            on_surface = c.on_surface,
            on_surface_var = c.on_surface_variant,
            font = self.config.theme.font_family,
            wifi_bg = if wifi_on { &c.primary } else { &c.surface_container_lowest },
            wifi_fg = if wifi_on { &c.on_primary } else { &c.on_surface },
            wifi_sub = if wifi_on { "Connected" } else { "Off" },
            bt_bg = if bt_on { &c.primary } else { &c.surface_container_lowest },
            bt_fg = if bt_on { &c.on_primary } else { &c.on_surface },
            bt_sub = if bt_on { "Active" } else { "Off" },
            card_bg = c.surface_container_low,
            card_outline = c.outline_variant,
            wm_mode = if dwindle_mode { "Dwindle Tiling" } else { "Floating macOS" },
            btn_bg = c.primary_container,
            btn_fg = c.on_primary_container,
            slider_track = c.surface_container_highest,
            slider_fill = c.primary,
            primary = c.primary,
            theme_pill_bg = if dark_mode { &c.secondary_container } else { &c.primary_container },
            theme_pill_fg = if dark_mode { &c.on_secondary_container } else { &c.on_primary_container },
            theme_name = if dark_mode { "Dark Theme" } else { "Light Theme" }
        );

        svg
    }
}
