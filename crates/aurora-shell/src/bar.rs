use aurora_config::AuroraConfig;
use aurora_theme::M3ColorScheme;
use chrono::Local;
use std::fmt::Write;

pub struct TopBarView {
    pub config: AuroraConfig,
    pub colors: M3ColorScheme,
}

impl TopBarView {
    pub fn new(config: AuroraConfig, colors: M3ColorScheme) -> Self {
        Self { config, colors }
    }

    /// Render an M3 macOS-like top bar to an SVG string or UI representation.
    /// Features:
    /// - Left: Apple/Aurora logo pill, Active app name, Workspace pills (1-9) with active indicator
    /// - Center: Window title or status
    /// - Right: Battery / Memory indicator, Current Clock (%a %b %e  %H:%M), Control Center pill button
    pub fn render_svg(&self, width: u32, active_workspace: u32, active_app_title: &str) -> String {
        let height = self.config.top_bar.height;
        let c = &self.colors;
        let now = Local::now();
        let time_str = now.format(&self.config.top_bar.clock_format).to_string();

        let mut svg = String::new();
        let _ = write!(
            svg,
            r#"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
<rect width="{width}" height="{height}" fill="{bg}" fill-opacity="0.95"/>
<line x1="0" y1="{height}" x2="{width}" y2="{height}" stroke="{outline}" stroke-width="1"/>
"#,
            width = width,
            height = height,
            bg = c.surface_container,
            outline = c.outline_variant
        );

        // macOS styled Aurora Logo & Workspace pills (Left)
        let _ = write!(
            svg,
            r#"<g transform="translate(12, 6)">
  <!-- Logo pill -->
  <rect x="0" y="0" width="22" height="20" rx="6" fill="{primary_container}"/>
  <text x="11" y="14" fill="{on_primary_container}" font-family="{font}" font-size="11" font-weight="bold" text-anchor="middle">A</text>

  <!-- Active App name -->
  <text x="32" y="14" fill="{on_surface}" font-family="{font}" font-size="12" font-weight="600">{active_app}</text>
"#,
            primary_container = c.primary_container,
            on_primary_container = c.on_primary_container,
            font = self.config.theme.font_family,
            on_surface = c.on_surface,
            active_app = if active_app_title.is_empty() { "Finder" } else { active_app_title }
        );

        // Workspace indicators
        let mut ws_x = 110;
        for ws in 1..=6 {
            let is_active = ws == active_workspace;
            let ws_bg = if is_active { &c.primary } else { &c.surface_container_high };
            let ws_fg = if is_active { &c.on_primary } else { &c.on_surface_variant };
            let _ = write!(
                svg,
                r#"  <rect x="{x}" y="1" width="18" height="18" rx="5" fill="{bg}"/>
  <text x="{tx}" y="14" fill="{fg}" font-family="{font}" font-size="10" font-weight="{weight}" text-anchor="middle">{ws}</text>
"#,
                x = ws_x,
                tx = ws_x + 9,
                bg = ws_bg,
                fg = ws_fg,
                font = self.config.theme.font_family,
                weight = if is_active { "bold" } else { "normal" },
                ws = ws
            );
            ws_x += 22;
        }
        let _ = write!(svg, "</g>\n");

        // Right side: Battery, Clock, Control Center toggle
        let right_x = width - 180;
        let _ = write!(
            svg,
            r#"<g transform="translate({rx}, 6)">
  <!-- System tray indicators -->
  <text x="0" y="14" fill="{on_surface_variant}" font-family="{font}" font-size="11">RAM 24%</text>
  <text x="60" y="14" fill="{on_surface}" font-family="{font}" font-size="11" font-weight="500">{time}</text>
  <!-- Control Center Icon Pill -->
  <rect x="150" y="1" width="22" height="18" rx="6" fill="{sec_container}"/>
  <circle cx="157" cy="7" r="2" fill="{on_sec_container}"/>
  <circle cx="165" cy="13" r="2" fill="{on_sec_container}"/>
  <line x1="154" y1="7" x2="168" y2="7" stroke="{on_sec_container}" stroke-width="1.5" stroke-linecap="round"/>
  <line x1="154" y1="13" x2="168" y2="13" stroke="{on_sec_container}" stroke-width="1.5" stroke-linecap="round"/>
</g>
</svg>
"#,
            rx = right_x,
            on_surface = c.on_surface,
            on_surface_variant = c.on_surface_variant,
            font = self.config.theme.font_family,
            time = time_str,
            sec_container = c.secondary_container,
            on_sec_container = c.on_secondary_container
        );

        svg
    }
}
