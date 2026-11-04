use aurora_config::ThemeConfig;
use serde::{Deserialize, Serialize};

/// Material 3 Color Tokens
/// Following Google's Material Design 3 Spec:
/// - Primary: Key component highlight, active borders, toggle highlights
/// - OnPrimary: Content on top of primary
/// - PrimaryContainer: Soft tonal fill for active pills / badges
/// - OnPrimaryContainer: Contrast text on primary container
/// - Secondary / SecondaryContainer: Less prominent components, chips
/// - Surface: Main background surface
/// - SurfaceDim / SurfaceBright: Elevation baseline
/// - SurfaceContainerLowest / Low / Medium / High / Highest: M3 tonal elevation without glassy blur
/// - OnSurface: High emphasis text
/// - OnSurfaceVariant: Medium emphasis text / subtitles
/// - Outline & OutlineVariant: Crisp subtle borders separating cards & panes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct M3ColorScheme {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub secondary: String,
    pub on_secondary: String,
    pub secondary_container: String,
    pub on_secondary_container: String,
    pub surface: String,
    pub on_surface: String,
    pub surface_dim: String,
    pub surface_bright: String,
    pub surface_container_lowest: String,
    pub surface_container_low: String,
    pub surface_container: String,
    pub surface_container_high: String,
    pub surface_container_highest: String,
    pub on_surface_variant: String,
    pub outline: String,
    pub outline_variant: String,
    pub error: String,
    pub on_error: String,
}

impl M3ColorScheme {
    pub fn dark_aurora_blue() -> Self {
        Self {
            primary: "#A8C7FA".to_string(),
            on_primary: "#062E6F".to_string(),
            primary_container: "#234185".to_string(),
            on_primary_container: "#D3E3FD".to_string(),
            secondary: "#7FCFFF".to_string(),
            on_secondary: "#00344F".to_string(),
            secondary_container: "#004B70".to_string(),
            on_secondary_container: "#C2E7FF".to_string(),
            surface: "#111318".to_string(),
            on_surface: "#E2E2E9".to_string(),
            surface_dim: "#111318".to_string(),
            surface_bright: "#37393E".to_string(),
            surface_container_lowest: "#0C0E13".to_string(),
            surface_container_low: "#191B20".to_string(),
            surface_container: "#1D1F24".to_string(),
            surface_container_high: "#282A2F".to_string(),
            surface_container_highest: "#33353A".to_string(),
            on_surface_variant: "#C4C6D0".to_string(),
            outline: "#8E9099".to_string(),
            outline_variant: "#44474E".to_string(),
            error: "#F2B8B5".to_string(),
            on_error: "#601410".to_string(),
        }
    }

    pub fn light_aurora_blue() -> Self {
        Self {
            primary: "#0B57D0".to_string(),
            on_primary: "#FFFFFF".to_string(),
            primary_container: "#D3E3FD".to_string(),
            on_primary_container: "#041E49".to_string(),
            secondary: "#00639B".to_string(),
            on_secondary: "#FFFFFF".to_string(),
            secondary_container: "#C2E7FF".to_string(),
            on_secondary_container: "#001D32".to_string(),
            surface: "#FDF8FD".to_string(),
            on_surface: "#1A1B20".to_string(),
            surface_dim: "#DCD9DE".to_string(),
            surface_bright: "#FDF8FD".to_string(),
            surface_container_lowest: "#FFFFFF".to_string(),
            surface_container_low: "#F7F2FA".to_string(),
            surface_container: "#F1EDF4".to_string(),
            surface_container_high: "#ECE6EE".to_string(),
            surface_container_highest: "#E6E0E9".to_string(),
            on_surface_variant: "#44474E".to_string(),
            outline: "#74777F".to_string(),
            outline_variant: "#C4C6D0".to_string(),
            error: "#B3261E".to_string(),
            on_error: "#FFFFFF".to_string(),
        }
    }

    pub fn dark_emerald() -> Self {
        Self {
            primary: "#85D996".to_string(),
            on_primary: "#003919".to_string(),
            primary_container: "#005227".to_string(),
            on_primary_container: "#A1F6B1".to_string(),
            secondary: "#B7CCB8".to_string(),
            on_secondary: "#233427".to_string(),
            secondary_container: "#394B3C".to_string(),
            on_secondary_container: "#D3E8D3".to_string(),
            surface: "#111411".to_string(),
            on_surface: "#E1E3DE".to_string(),
            surface_dim: "#111411".to_string(),
            surface_bright: "#373A36".to_string(),
            surface_container_lowest: "#0C0F0C".to_string(),
            surface_container_low: "#191C19".to_string(),
            surface_container: "#1D201D".to_string(),
            surface_container_high: "#272B27".to_string(),
            surface_container_highest: "#323632".to_string(),
            on_surface_variant: "#C1C9BF".to_string(),
            outline: "#8B938A".to_string(),
            outline_variant: "#414942".to_string(),
            error: "#F2B8B5".to_string(),
            on_error: "#601410".to_string(),
        }
    }

    pub fn dark_purple() -> Self {
        Self {
            primary: "#D0BCFF".to_string(),
            on_primary: "#381E72".to_string(),
            primary_container: "#4F378B".to_string(),
            on_primary_container: "#EADDFF".to_string(),
            secondary: "#CCC2DC".to_string(),
            on_secondary: "#332D41".to_string(),
            secondary_container: "#4A4458".to_string(),
            on_secondary_container: "#E8DEF8".to_string(),
            surface: "#141218".to_string(),
            on_surface: "#E6E0E9".to_string(),
            surface_dim: "#141218".to_string(),
            surface_bright: "#3B383E".to_string(),
            surface_container_lowest: "#0F0D13".to_string(),
            surface_container_low: "#1D1B20".to_string(),
            surface_container: "#211F26".to_string(),
            surface_container_high: "#2B2930".to_string(),
            surface_container_highest: "#36343B".to_string(),
            on_surface_variant: "#CAC4D0".to_string(),
            outline: "#938F99".to_string(),
            outline_variant: "#49454F".to_string(),
            error: "#F2B8B5".to_string(),
            on_error: "#601410".to_string(),
        }
    }

    pub fn from_config(config: &ThemeConfig) -> Self {
        let is_dark = config.mode != "light";
        match config.palette_preset.as_str() {
            "emerald" => {
                if is_dark {
                    Self::dark_emerald()
                } else {
                    let mut s = Self::dark_emerald();
                    s.surface = "#F7FAF5".to_string();
                    s.on_surface = "#181C19".to_string();
                    s
                }
            }
            "purple" => {
                if is_dark {
                    Self::dark_purple()
                } else {
                    let mut s = Self::dark_purple();
                    s.surface = "#FEF7FF".to_string();
                    s.on_surface = "#1D1B20".to_string();
                    s
                }
            }
            _ => {
                if is_dark {
                    Self::dark_aurora_blue()
                } else {
                    Self::light_aurora_blue()
                }
            }
        }
    }
}

/// Helper struct for converting hex colors to u32 RGB or RGBA tuples for software or GPU rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Rgba(r, g, b, 255)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Rgba(r, g, b, a)
        } else {
            Rgba(0, 0, 0, 255)
        }
    }

    pub fn to_u32_argb(&self) -> u32 {
        ((self.3 as u32) << 24) | ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }

    pub fn to_u32_rgba(&self) -> u32 {
        ((self.0 as u32) << 24) | ((self.1 as u32) << 16) | ((self.2 as u32) << 8) | (self.3 as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let color = Rgba::from_hex("#A8C7FA");
        assert_eq!(color, Rgba(168, 199, 250, 255));
        assert_eq!(color.to_u32_argb(), 0xFFA8C7FA);
    }

    #[test]
    fn test_theme_presets() {
        let mut cfg = ThemeConfig::default();
        cfg.palette_preset = "purple".to_string();
        let scheme = M3ColorScheme::from_config(&cfg);
        assert_eq!(scheme.primary, "#D0BCFF");
    }
}
