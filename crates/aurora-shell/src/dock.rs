use aurora_config::AuroraConfig;
use aurora_theme::M3ColorScheme;
use std::fmt::Write;

pub struct DockView {
    pub config: AuroraConfig,
    pub colors: M3ColorScheme,
}

impl DockView {
    pub fn new(config: AuroraConfig, colors: M3ColorScheme) -> Self {
        Self { config, colors }
    }

    /// Render M3 macOS-like bottom dock:
    /// - Material 3 tonal elevated container (surface_container_high, rounded corners, outline border)
    /// - App icons with active dot indicators below
    /// - Magnification and crisp hover highlights
    pub fn render_svg(&self, screen_width: u32, screen_height: u32, active_apps: &[String]) -> String {
        let apps = &self.config.dock.pinned_apps;
        let icon_size = self.config.dock.icon_size;
        let pad = 12;
        let dock_w = (apps.len() as u32 * (icon_size + 14)) + (pad * 2);
        let dock_h = icon_size + 20;

        let dock_x = (screen_width.saturating_sub(dock_w)) / 2;
        let dock_y = screen_height.saturating_sub(dock_h + 10);

        let c = &self.colors;
        let mut svg = String::new();

        let _ = write!(
            svg,
            r#"<svg width="{sw}" height="{sh}" viewBox="0 0 {sw} {sh}" xmlns="http://www.w3.org/2000/svg">
  <!-- Dock container: M3 Elevated Pill -->
  <rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{radius}" fill="{bg}" stroke="{outline}" stroke-width="1"/>
"#,
            sw = screen_width,
            sh = screen_height,
            x = dock_x,
            y = dock_y,
            w = dock_w,
            h = dock_h,
            radius = (dock_h / 2).min(20),
            bg = c.surface_container_high,
            outline = c.outline_variant
        );

        let mut app_x = dock_x + pad;
        for app in apps {
            let is_running = active_apps.iter().any(|a| a.to_lowercase().contains(&app.name.to_lowercase()));
            let item_y = dock_y + 8;

            // Render app icon pill
            let _ = write!(
                svg,
                r#"  <g transform="translate({ax}, {ay})">
    <rect x="0" y="0" width="{size}" height="{size}" rx="12" fill="{primary_container}"/>
    <text x="{center}" y="{text_y}" fill="{on_primary_container}" font-family="{font}" font-size="16" font-weight="bold" text-anchor="middle">{initial}</text>
"#,
                ax = app_x,
                ay = item_y,
                size = icon_size,
                center = icon_size / 2,
                text_y = (icon_size / 2) + 6,
                primary_container = c.primary_container,
                on_primary_container = c.on_primary_container,
                font = self.config.theme.font_family,
                initial = app.name.chars().next().unwrap_or('A')
            );

            // Active indicator dot below icon (macOS style running indicator)
            if is_running {
                let _ = write!(
                    svg,
                    r#"    <circle cx="{cx}" cy="{cy}" r="2.5" fill="{primary}"/>
"#,
                    cx = icon_size / 2,
                    cy = icon_size + 7,
                    primary = c.primary
                );
            }

            let _ = write!(svg, "  </g>\n");
            app_x += icon_size + 14;
        }

        let _ = write!(svg, "</svg>\n");
        svg
    }
}
