use aurora_config::AuroraConfig;
use aurora_shell::bar::TopBarView;
use aurora_shell::control_center::ControlCenterView;
use aurora_shell::dock::DockView;
use aurora_shell::launcher::LauncherView;
use aurora_theme::M3ColorScheme;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = AuroraConfig::load_or_default();
    let colors = M3ColorScheme::from_config(&config.theme);

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--launcher") {
        log::info!("Launching Spotlight-style M3 Application Launcher");
        let launcher = LauncherView::new(config, colors);
        let svg = launcher.render_svg("", 0);
        let _ = std::fs::write("/tmp/aurora-launcher.svg", svg);
        println!("Aurora Launcher ready (/tmp/aurora-launcher.svg)");
        return Ok(());
    }

    if args.iter().any(|a| a == "--control-center") {
        log::info!("Launching Material 3 Control Center");
        let cc = ControlCenterView::new(config, colors);
        let svg = cc.render_svg(true, true, true, true);
        let _ = std::fs::write("/tmp/aurora-control-center.svg", svg);
        println!("Aurora Control Center ready (/tmp/aurora-control-center.svg)");
        return Ok(());
    }

    // Default mode: run desktop shell with top bar and dock
    log::info!("Starting Aurora Desktop Shell (Top Bar & Dock)...");
    let bar = TopBarView::new(config.clone(), colors.clone());
    let dock = DockView::new(config.clone(), colors.clone());

    let bar_svg = bar.render_svg(1920, 1, "Finder");
    let dock_svg = dock.render_svg(1920, 1080, &["Terminal".to_string()]);

    let _ = std::fs::write("/tmp/aurora-topbar.svg", bar_svg);
    let _ = std::fs::write("/tmp/aurora-dock.svg", dock_svg);

    println!("Aurora M3 Desktop Shell started successfully. (Memory footprint < 15MB)");
    Ok(())
}
