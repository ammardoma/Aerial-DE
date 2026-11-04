use clap::{Parser, Subcommand};
use aurora_ipc::{IpcClient, IpcRequest, IpcResponse};
use aurora_config::LayoutMode;

#[derive(Parser)]
#[command(name = "aurora-msg")]
#[command(about = "Command line control tool for Aurora Desktop Environment", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ping the compositor
    Ping,
    /// Query current window manager and workspace state
    GetState,
    /// Toggle window manager between Dwindle and Floating mode
    ToggleLayout,
    /// Switch window manager layout mode (dwindle or floating)
    SetLayout {
        mode: String,
    },
    /// Switch to workspace (1-9)
    Workspace {
        id: u32,
    },
    /// Move active window to workspace (1-9)
    MoveToWorkspace {
        id: u32,
    },
    /// Close the active window
    Close,
    /// Toggle fullscreen on active window
    Fullscreen,
    /// Set inner and outer window gaps
    Gaps {
        inner: Option<u32>,
        outer: Option<u32>,
    },
    /// Trigger Spotlight-style App Launcher
    Launcher,
    /// Trigger Control Center
    ControlCenter,
    /// Reload configuration from ~/.config/aurora/aurora.toml
    Reload,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = IpcClient::default_client();

    let req = match cli.command {
        Commands::Ping => IpcRequest::Ping,
        Commands::GetState => IpcRequest::GetState,
        Commands::ToggleLayout => IpcRequest::ToggleLayoutMode,
        Commands::SetLayout { mode } => {
            let m = if mode.to_lowercase() == "floating" {
                LayoutMode::Floating
            } else {
                LayoutMode::Dwindle
            };
            IpcRequest::SetLayoutMode(m)
        }
        Commands::Workspace { id } => IpcRequest::SwitchWorkspace(id),
        Commands::MoveToWorkspace { id } => IpcRequest::MoveToWorkspace(id),
        Commands::Close => IpcRequest::CloseActiveWindow,
        Commands::Fullscreen => IpcRequest::ToggleFullscreen,
        Commands::Gaps { inner, outer } => IpcRequest::UpdateWmConfig {
            inner_gaps: inner,
            outer_gaps: outer,
            border_width: None,
            split_ratio: None,
            spiral: None,
        },
        Commands::Launcher => IpcRequest::ToggleLauncher,
        Commands::ControlCenter => IpcRequest::ToggleControlCenter,
        Commands::Reload => IpcRequest::ReloadConfig,
    };

    match client.send_request(req).await {
        Ok(resp) => {
            match resp {
                IpcResponse::Ok(msg) => println!("{}", msg),
                IpcResponse::State(state) => {
                    println!("=== Aurora Window Manager State ===");
                    println!("Layout Mode:        {:?}", state.layout_mode);
                    println!("Current Workspace:  {}", state.current_workspace);
                    println!("Total Workspaces:   {}", state.total_workspaces);
                    println!("Windows on Desktop: {}", state.window_count);
                    println!("Active Window:      {}", state.active_window_title.as_deref().unwrap_or("<None>"));
                    println!("Inner Gaps:         {}px", state.inner_gaps);
                    println!("Outer Gaps:         {}px", state.outer_gaps);
                    println!("Split Ratio:        {:.2}", state.split_ratio);
                    println!("Spiral Dwindle:     {}", state.spiral);
                }
                IpcResponse::Error(err) => eprintln!("Error from compositor: {}", err),
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Aurora session: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
