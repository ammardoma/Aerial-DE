use aurora_config::AuroraConfig;
use aurora_settings::{SettingsApp, SettingsCategory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("Starting Aurora M3 Settings Application...");

    let config = AuroraConfig::load_or_default();
    let app = SettingsApp::new(config);

    // Render tabs to files for verification and display
    let wm_svg = app.render_svg(SettingsCategory::WindowManager);
    let keybinds_svg = app.render_svg(SettingsCategory::Keybindings);
    let appearance_svg = app.render_svg(SettingsCategory::Appearance);
    let dock_svg = app.render_svg(SettingsCategory::DockAndBar);
    let sys_svg = app.render_svg(SettingsCategory::System);

    let _ = std::fs::write("/tmp/aurora-settings-wm.svg", wm_svg);
    let _ = std::fs::write("/tmp/aurora-settings-keybinds.svg", keybinds_svg);
    let _ = std::fs::write("/tmp/aurora-settings-appearance.svg", appearance_svg);
    let _ = std::fs::write("/tmp/aurora-settings-dock.svg", dock_svg);
    let _ = std::fs::write("/tmp/aurora-settings-system.svg", sys_svg);

    println!("Aurora Settings App launched successfully! Rendered panels in /tmp/aurora-settings-*.svg");
    Ok(())
}
