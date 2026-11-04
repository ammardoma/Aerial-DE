use aurora_config::AuroraConfig;
use aurora_theme::M3ColorScheme;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub description: String,
}

pub struct LauncherView {
    pub config: AuroraConfig,
    pub colors: M3ColorScheme,
}

impl LauncherView {
    pub fn new(config: AuroraConfig, colors: M3ColorScheme) -> Self {
        Self { config, colors }
    }

    /// Discover system apps or provide standard built-ins
    pub fn list_applications() -> Vec<DesktopEntry> {
        vec![
            DesktopEntry {
                name: "Terminal".to_string(),
                exec: "alacritty".to_string(),
                description: "Command line shell & terminal emulator".to_string(),
            },
            DesktopEntry {
                name: "Settings".to_string(),
                exec: "aurora-settings".to_string(),
                description: "Aurora M3 Desktop Configuration & Personalization".to_string(),
            },
            DesktopEntry {
                name: "Files".to_string(),
                exec: "nautilus".to_string(),
                description: "File Manager and document browser".to_string(),
            },
            DesktopEntry {
                name: "Web Browser".to_string(),
                exec: "firefox".to_string(),
                description: "Browse the World Wide Web".to_string(),
            },
            DesktopEntry {
                name: "Text Editor".to_string(),
                exec: "gedit".to_string(),
                description: "View and edit text files".to_string(),
            },
            DesktopEntry {
                name: "System Monitor".to_string(),
                exec: "htop".to_string(),
                description: "View processes and system performance".to_string(),
            },
            DesktopEntry {
                name: "Calculator".to_string(),
                exec: "gnome-calculator".to_string(),
                description: "Perform arithmetic and scientific calculations".to_string(),
            },
        ]
    }

    pub fn filter_applications<'a>(query: &str, apps: &'a [DesktopEntry]) -> Vec<&'a DesktopEntry> {
        if query.trim().is_empty() {
            return apps.iter().collect();
        }
        let q = query.to_lowercase();
        apps.iter()
            .filter(|a| a.name.to_lowercase().contains(&q) || a.description.to_lowercase().contains(&q))
            .collect()
    }

    /// Render M3 macOS-like Spotlight Search Launcher
    pub fn render_svg(&self, query: &str, selected_index: usize) -> String {
        let width = self.config.launcher.width;
        let height = self.config.launcher.height;
        let c = &self.colors;
        let apps = Self::list_applications();
        let matches = Self::filter_applications(query, &apps);

        let mut svg = String::new();
        let _ = write!(
            svg,
            r#"<svg width="{w}" height="{h}" viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg">
  <!-- Outer card: Material 3 Surface Container High with 28px rounded corners -->
  <rect width="{w}" height="{h}" rx="28" fill="{bg}" stroke="{outline}" stroke-width="1.5"/>

  <!-- Search input bar: M3 Pill -->
  <g transform="translate(20, 20)">
    <rect width="{input_w}" height="48" rx="24" fill="{input_bg}"/>
    <circle cx="24" cy="24" r="7" stroke="{on_surface_var}" stroke-width="2" fill="none"/>
    <line x1="29" y1="29" x2="35" y2="35" stroke="{on_surface_var}" stroke-width="2" stroke-linecap="round"/>
    <text x="46" y="30" fill="{text_color}" font-family="{font}" font-size="16">{display_text}</text>
  </g>
  <line x1="20" y1="84" x2="{line_x2}" y2="84" stroke="{outline_var}" stroke-width="1"/>
"#,
            w = width,
            h = height,
            bg = c.surface_container_high,
            outline = c.outline_variant,
            input_w = width - 40,
            input_bg = c.surface_container_lowest,
            on_surface_var = c.on_surface_variant,
            text_color = if query.is_empty() { &c.on_surface_variant } else { &c.on_surface },
            font = self.config.theme.font_family,
            display_text = if query.is_empty() { &self.config.launcher.search_placeholder } else { query },
            line_x2 = width - 20,
            outline_var = c.outline_variant
        );

        let mut list_y = 100;
        let max_items = self.config.launcher.max_results.min(matches.len());

        for (i, app) in matches.iter().take(max_items).enumerate() {
            let is_sel = i == selected_index;
            let item_bg = if is_sel { &c.secondary_container } else { "transparent" };
            let title_color = if is_sel { &c.on_secondary_container } else { &c.on_surface };
            let desc_color = if is_sel { &c.on_secondary_container } else { &c.on_surface_variant };

            let _ = write!(
                svg,
                r#"  <g transform="translate(20, {y})">
    <rect width="{item_w}" height="42" rx="16" fill="{item_bg}"/>
    <circle cx="26" cy="21" r="14" fill="{icon_bg}"/>
    <text x="26" y="26" fill="{icon_fg}" font-family="{font}" font-size="12" font-weight="bold" text-anchor="middle">{initial}</text>
    <text x="50" y="20" fill="{t_color}" font-family="{font}" font-size="14" font-weight="500">{name}</text>
    <text x="50" y="34" fill="{d_color}" font-family="{font}" font-size="11">{desc}</text>
  </g>
"#,
                y = list_y,
                item_w = width - 40,
                item_bg = item_bg,
                icon_bg = c.primary_container,
                icon_fg = c.on_primary_container,
                font = self.config.theme.font_family,
                initial = app.name.chars().next().unwrap_or('A'),
                t_color = title_color,
                name = app.name,
                d_color = desc_color,
                desc = app.description
            );

            list_y += 48;
        }

        let _ = write!(svg, "</svg>\n");
        svg
    }
}
