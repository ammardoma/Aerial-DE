use aurora_config::AuroraConfig;
use aurora_wm::layout::Rect;
use aurora_wm::server::WmServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting Aurora Wayland Window Manager...");

    let config = AuroraConfig::load_or_default();
    let screen = Rect::new(0, 0, 1920, 1080);
    let server = WmServer::new(config, screen);

    log::info!("Aurora Window Manager initialized successfully in {:?} mode", server.get_state().lock().await.config.window_manager.layout_mode);

    server.run().await?;
    Ok(())
}
