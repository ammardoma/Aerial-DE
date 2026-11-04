use aurora_config::{AuroraConfig, LayoutMode};
use aurora_ipc::{IpcClient, IpcRequest};
use aurora_theme::M3ColorScheme;
use std::fmt::Write;

pub struct SettingsApp {
    pub config: AuroraConfig,
    pub colors: M3ColorScheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    WindowManager,
    Keybindings,
    Appearance,
    DockAndBar,
    System,
}

impl SettingsApp {
    pub fn new(config: AuroraConfig) -> Self {
        let colors = M3ColorScheme::from_config(&config.theme);
        Self { config, colors }
    }

    pub async fn apply_changes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.save()?;
        self.colors = M3ColorScheme::from_config(&self.config.theme);

        // Notify compositor over IPC
        let client = IpcClient::default_client();
        let _ = client
            .send_request(IpcRequest::UpdateWmConfig {
                inner_gaps: Some(self.config.window_manager.inner_gaps),
                outer_gaps: Some(self.config.window_manager.outer_gaps),
                border_width: Some(self.config.window_manager.border_width),
                split_ratio: Some(self.config.window_manager.split_ratio),
                spiral: Some(self.config.window_manager.spiral),
            })
            .await;

        let _ = client.send_request(IpcRequest::SetLayoutMode(self.config.window_manager.layout_mode)).await;
        Ok(())
    }

    /// Render M3 macOS-like Settings Window (Navigation Rail on left, cards on right, traffic lights on header)
    pub fn render_svg(&self, category: SettingsCategory) -> String {
        let width = 880;
        let height = 620;
        let c = &self.colors;
        let mut svg = String::new();

        let red_btn = "#FF5F56";
        let yellow_btn = "#FFBD2E";
        let green_btn = "#27C93F";

        let _ = writeln!(
            svg,
            r#"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
  <rect width="{width}" height="{height}" rx="16" fill="{surface}" stroke="{outline}" stroke-width="1"/>
  <rect width="{width}" height="44" rx="16" fill="{surface_container}"/>
  <rect y="28" width="{width}" height="16" fill="{surface_container}"/>
  <line x1="0" y1="44" x2="{width}" y2="44" stroke="{outline_variant}" stroke-width="1"/>
  <circle cx="24" cy="22" r="6" fill="{red_btn}"/>
  <circle cx="44" cy="22" r="6" fill="{yellow_btn}"/>
  <circle cx="64" cy="22" r="6" fill="{green_btn}"/>
  <text x="440" y="27" fill="{on_surface}" font-family="{font}" font-size="13" font-weight="600" text-anchor="middle">System Settings</text>
  <rect x="0" y="44" width="220" height="576" fill="{nav_bg}"/>
  <line x1="220" y1="44" x2="220" y2="{height}" stroke="{outline_variant}" stroke-width="1"/>"#,
            width = width,
            height = height,
            surface = c.surface,
            outline = c.outline_variant,
            surface_container = c.surface_container,
            outline_variant = c.outline_variant,
            red_btn = red_btn,
            yellow_btn = yellow_btn,
            green_btn = green_btn,
            on_surface = c.on_surface,
            font = self.config.theme.font_family,
            nav_bg = c.surface_container_low
        );

        // Sidebar Navigation Items
        let nav_items = [
            (SettingsCategory::WindowManager, "Window Manager", "M3 Dwindle / Gaps"),
            (SettingsCategory::Keybindings, "Keybindings", "Shortcuts & Actions"),
            (SettingsCategory::Appearance, "Appearance", "M3 Themes & Colors"),
            (SettingsCategory::DockAndBar, "Dock & Top Bar", "Layout & Pinned Apps"),
            (SettingsCategory::System, "System & RAM", "Info & Performance"),
        ];

        let mut item_y = 60;
        for (cat, title, sub) in nav_items {
            let is_selected = cat == category;
            let pill_bg = if is_selected { &c.secondary_container } else { "transparent" };
            let title_color = if is_selected { &c.on_secondary_container } else { &c.on_surface };
            let sub_color = if is_selected { &c.on_secondary_container } else { &c.on_surface_variant };

            let _ = writeln!(
                svg,
                r#"  <g transform="translate(12, {y})">
    <rect width="196" height="44" rx="22" fill="{bg}"/>
    <text x="24" y="20" fill="{tc}" font-family="{font}" font-size="13" font-weight="600">{title}</text>
    <text x="24" y="34" fill="{sc}" font-family="{font}" font-size="10">{sub}</text>
  </g>"#,
                y = item_y,
                bg = pill_bg,
                tc = title_color,
                font = self.config.theme.font_family,
                title = title,
                sc = sub_color,
                sub = sub
            );
            item_y += 50;
        }

        // Right Main Content Pane: category details
        let _ = writeln!(
            svg,
            r#"  <g transform="translate(240, 60)">"#
        );

        match category {
            SettingsCategory::WindowManager => {
                let wm = &self.config.window_manager;
                let mode_str = match wm.layout_mode {
                    LayoutMode::Dwindle => "Dwindle Tiling",
                    LayoutMode::Floating => "macOS Floating",
                };
                let _ = writeln!(
                    svg,
                    r#"    <text x="0" y="24" fill="{on_surface}" font-family="{font}" font-size="20" font-weight="bold">Window Manager Settings</text>
    <text x="0" y="44" fill="{on_surface_var}" font-family="{font}" font-size="12">Configure Dwindle spiral tiling, gaps, borders, and window behaviors</text>
    <rect x="0" y="60" width="600" height="72" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="92" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Active Layout Engine</text>
    <text x="20" y="112" fill="{on_surface_var}" font-family="{font}" font-size="12">Current: {mode_str}</text>
    <rect x="460" y="80" width="120" height="32" rx="16" fill="{primary_container}"/>
    <text x="520" y="100" fill="{on_primary_container}" font-family="{font}" font-size="12" font-weight="600" text-anchor="middle">Switch Mode</text>
    <rect x="0" y="146" width="600" height="84" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="176" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Inner &amp; Outer Gaps</text>
    <text x="20" y="196" fill="{on_surface_var}" font-family="{font}" font-size="12">Inner: {inner_gaps}px  |  Outer: {outer_gaps}px  |  Split Ratio: {split_ratio:.2}</text>
    <rect x="20" y="208" width="560" height="6" rx="3" fill="{slider_track}"/>
    <rect x="20" y="208" width="240" height="6" rx="3" fill="{primary}"/>
    <circle cx="260" cy="211" r="8" fill="{primary}"/>
    <rect x="0" y="244" width="600" height="72" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="276" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">macOS Traffic Light Buttons</text>
    <text x="20" y="296" fill="{on_surface_var}" font-family="{font}" font-size="12">Render red/yellow/green control circles on titlebar</text>
    <rect x="520" y="264" width="52" height="30" rx="15" fill="{primary}"/>
    <circle cx="555" cy="279" r="11" fill="{on_primary}"/>
    <rect x="0" y="330" width="600" height="72" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="362" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Spiral Dwindle Splitting</text>
    <text x="20" y="382" fill="{on_surface_var}" font-family="{font}" font-size="12">Recursively alternate window splits in Fibonacci spiral order</text>
    <rect x="520" y="350" width="52" height="30" rx="15" fill="{primary}"/>
    <circle cx="555" cy="365" r="11" fill="{on_primary}"/>"#,
                    on_surface = c.on_surface,
                    font = self.config.theme.font_family,
                    on_surface_var = c.on_surface_variant,
                    card_bg = c.surface_container_low,
                    outline = c.outline_variant,
                    mode_str = mode_str,
                    primary_container = c.primary_container,
                    on_primary_container = c.on_primary_container,
                    inner_gaps = wm.inner_gaps,
                    outer_gaps = wm.outer_gaps,
                    split_ratio = wm.split_ratio,
                    slider_track = c.surface_container_highest,
                    primary = c.primary,
                    on_primary = c.on_primary
                );
            }
            SettingsCategory::Keybindings => {
                let _ = writeln!(
                    svg,
                    r#"    <text x="0" y="24" fill="{on_surface}" font-family="{font}" font-size="20" font-weight="bold">Keyboard Shortcuts</text>
    <text x="0" y="44" fill="{on_surface_var}" font-family="{font}" font-size="12">Super / Mod4 keybindings configured for instant multitasking</text>"#,
                    on_surface = c.on_surface,
                    font = self.config.theme.font_family,
                    on_surface_var = c.on_surface_variant
                );

                let shortcuts = [
                    ("Super + Space", "Open Spotlight App Launcher"),
                    ("Super + Return", "Launch Terminal Emulator"),
                    ("Super + Q", "Close Focused Window"),
                    ("Super + T", "Toggle Dwindle / Floating Mode"),
                    ("Super + F", "Toggle Window Fullscreen"),
                    ("Super + C", "Open Control Center Quick Settings"),
                    ("Super + [1-9]", "Switch to Workspace 1 through 9"),
                    ("Super + Shift + [1-9]", "Move Active Window to Workspace"),
                    ("Super + Arrow Keys", "Focus Adjacent Window"),
                ];

                let mut ky = 70;
                for (keys, action) in shortcuts {
                    let _ = writeln!(
                        svg,
                        r#"    <g transform="translate(0, {y})">
      <rect width="600" height="40" rx="10" fill="{card_bg}" stroke="{outline}" stroke-width="0.8"/>
      <rect x="14" y="8" width="130" height="24" rx="6" fill="{key_bg}"/>
      <text x="79" y="24" fill="{on_key_bg}" font-family="monospace" font-size="11" font-weight="bold" text-anchor="middle">{keys}</text>
      <text x="160" y="25" fill="{on_surface}" font-family="{font}" font-size="13">{action}</text>
    </g>"#,
                        y = ky,
                        card_bg = c.surface_container_low,
                        outline = c.outline_variant,
                        key_bg = c.primary_container,
                        on_key_bg = c.on_primary_container,
                        keys = keys,
                        on_surface = c.on_surface,
                        font = self.config.theme.font_family,
                        action = action
                    );
                    ky += 46;
                }
            }
            SettingsCategory::Appearance => {
                let p1 = "#A8C7FA";
                let p2 = "#85D996";
                let p3 = "#D0BCFF";
                let p4 = "#FFB0CD";

                let _ = writeln!(
                    svg,
                    r#"    <text x="0" y="24" fill="{on_surface}" font-family="{font}" font-size="20" font-weight="bold">Appearance &amp; Material 3 Theme</text>
    <text x="0" y="44" fill="{on_surface_var}" font-family="{font}" font-size="12">Customize dynamic tonal palette presets and dark/light mode</text>
    <rect x="0" y="60" width="600" height="72" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="92" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Color Mode</text>
    <text x="20" y="112" fill="{on_surface_var}" font-family="{font}" font-size="12">Current: {mode}</text>
    <rect x="470" y="80" width="110" height="32" rx="16" fill="{primary_container}"/>
    <text x="525" y="100" fill="{on_primary_container}" font-family="{font}" font-size="12" font-weight="600" text-anchor="middle">Toggle Mode</text>
    <rect x="0" y="146" width="600" height="140" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="176" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Material 3 Color Presets</text>
    <text x="20" y="196" fill="{on_surface_var}" font-family="{font}" font-size="12">Select tonal accent theme:</text>
    <circle cx="50" cy="240" r="22" fill="{p1}"/>
    <text x="50" y="274" fill="{on_surface}" font-family="{font}" font-size="11" text-anchor="middle">Aurora</text>
    <circle cx="130" cy="240" r="22" fill="{p2}"/>
    <text x="130" y="274" fill="{on_surface}" font-family="{font}" font-size="11" text-anchor="middle">Emerald</text>
    <circle cx="210" cy="240" r="22" fill="{p3}"/>
    <text x="210" y="274" fill="{on_surface}" font-family="{font}" font-size="11" text-anchor="middle">Purple</text>
    <circle cx="290" cy="240" r="22" fill="{p4}"/>
    <text x="290" y="274" fill="{on_surface}" font-family="{font}" font-size="11" text-anchor="middle">Sakura</text>"#,
                    on_surface = c.on_surface,
                    font = self.config.theme.font_family,
                    on_surface_var = c.on_surface_variant,
                    card_bg = c.surface_container_low,
                    outline = c.outline_variant,
                    mode = self.config.theme.mode,
                    primary_container = c.primary_container,
                    on_primary_container = c.on_primary_container,
                    p1 = p1,
                    p2 = p2,
                    p3 = p3,
                    p4 = p4
                );
            }
            SettingsCategory::DockAndBar => {
                let _ = writeln!(
                    svg,
                    r#"    <text x="0" y="24" fill="{on_surface}" font-family="{font}" font-size="20" font-weight="bold">Dock &amp; Top Bar</text>
    <text x="0" y="44" fill="{on_surface_var}" font-family="{font}" font-size="12">Customize macOS style dock position, magnification, and status items</text>
    <rect x="0" y="60" width="600" height="72" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="92" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Bottom Floating Dock</text>
    <text x="20" y="112" fill="{on_surface_var}" font-family="{font}" font-size="12">Size: {size}px | Magnification: Enabled</text>
    <rect x="0" y="146" width="600" height="72" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="178" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">macOS Menu Bar</text>
    <text x="20" y="198" fill="{on_surface_var}" font-family="{font}" font-size="12">Height: {bar_h}px | Clock, Quick Settings &amp; Pager</text>"#,
                    on_surface = c.on_surface,
                    font = self.config.theme.font_family,
                    on_surface_var = c.on_surface_variant,
                    card_bg = c.surface_container_low,
                    outline = c.outline_variant,
                    size = self.config.dock.icon_size,
                    bar_h = self.config.top_bar.height
                );
            }
            SettingsCategory::System => {
                let _ = writeln!(
                    svg,
                    r#"    <text x="0" y="24" fill="{on_surface}" font-family="{font}" font-size="20" font-weight="bold">System &amp; Resource Efficiency</text>
    <text x="0" y="44" fill="{on_surface_var}" font-family="{font}" font-size="12">Aurora Desktop Environment Runtime Statistics</text>
    <rect x="0" y="60" width="600" height="120" rx="16" fill="{card_bg}" stroke="{outline}" stroke-width="1"/>
    <text x="20" y="94" fill="{on_surface}" font-family="{font}" font-size="14" font-weight="600">Ultra Low RAM Footprint</text>
    <text x="20" y="120" fill="{on_surface_var}" font-family="{font}" font-size="12">Total Base RAM: ~18MB (Compositor + Shell)</text>
    <text x="20" y="144" fill="{primary}" font-family="{font}" font-size="12" font-weight="bold">&#10004; Pure Rust Zero-Copy Architecture</text>"#,
                    on_surface = c.on_surface,
                    font = self.config.theme.font_family,
                    on_surface_var = c.on_surface_variant,
                    card_bg = c.surface_container_low,
                    outline = c.outline_variant,
                    primary = c.primary
                );
            }
        }

        let _ = writeln!(svg, "  </g>\n</svg>");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_all_categories() {
        let config = AuroraConfig::default();
        let app = SettingsApp::new(config);

        let cats = [
            SettingsCategory::WindowManager,
            SettingsCategory::Keybindings,
            SettingsCategory::Appearance,
            SettingsCategory::DockAndBar,
            SettingsCategory::System,
        ];

        for cat in cats {
            let svg = app.render_svg(cat);
            assert!(svg.contains("<svg"));
            assert!(svg.contains("</svg>"));
            assert!(svg.contains("System Settings"));
        }
    }
}
