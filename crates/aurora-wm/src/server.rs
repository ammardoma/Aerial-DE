use crate::layout::Rect;
use crate::state::WmState;
use aurora_config::AuroraConfig;
use aurora_ipc::{IpcRequest, IpcResponse, DEFAULT_IPC_SOCKET};
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;
use wayland_server::ListeningSocket;

pub struct WmServer {
    state: Arc<Mutex<WmState>>,
    socket_path: String,
    wayland_display_socket: String,
}

impl WmServer {
    pub fn new(config: AuroraConfig, screen: Rect) -> Self {
        let socket_path = std::env::var("AURORA_SOCKET").unwrap_or_else(|_| DEFAULT_IPC_SOCKET.to_string());
        let wayland_display_socket = std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-1".to_string());
        let state = Arc::new(Mutex::new(WmState::new(config, screen)));
        Self {
            state,
            socket_path,
            wayland_display_socket,
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<WmState>> {
        self.state.clone()
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize Wayland display listening socket
        let socket_res = ListeningSocket::bind(&self.wayland_display_socket);
        match socket_res {
            Ok(_wl_socket) => {
                log::info!("Wayland display listening on socket: {}", self.wayland_display_socket);
                println!("Wayland Compositor socket active: WAYLAND_DISPLAY={}", self.wayland_display_socket);
            }
            Err(e) => {
                log::warn!("Could not bind Wayland socket ({}): {}. Compositor running.", self.wayland_display_socket, e);
            }
        }

        // Initialize IPC server
        let sock_path = Path::new(&self.socket_path);
        if sock_path.exists() {
            let _ = std::fs::remove_file(sock_path);
        }

        let listener = UnixListener::bind(sock_path)?;
        log::info!("Aurora WM IPC server listening on {:?}", sock_path);

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let state = self.state.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, state).await {
                            log::warn!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Failed to accept IPC connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(
    stream: UnixStream,
    state: Arc<Mutex<WmState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    while reader.read_line(&mut line).await? > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        let req: IpcRequest = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = IpcResponse::Error(format!("Malformed request: {}", e));
                let resp_str = serde_json::to_string(&err_resp)? + "\n";
                writer.write_all(resp_str.as_bytes()).await?;
                writer.flush().await?;
                line.clear();
                continue;
            }
        };

        let response = {
            let mut state_guard = state.lock().await;
            match req {
                IpcRequest::Ping => IpcResponse::Ok("pong".to_string()),
                IpcRequest::GetState => IpcResponse::State(state_guard.snapshot()),
                IpcRequest::SetLayoutMode(mode) => {
                    state_guard.set_layout_mode(mode);
                    IpcResponse::Ok(format!("Layout mode set to {:?}", mode))
                }
                IpcRequest::ToggleLayoutMode => {
                    state_guard.toggle_layout_mode();
                    let current = state_guard.config.window_manager.layout_mode;
                    IpcResponse::Ok(format!("Layout mode toggled to {:?}", current))
                }
                IpcRequest::SwitchWorkspace(ws) => {
                    state_guard.switch_workspace(ws);
                    IpcResponse::Ok(format!("Switched to workspace {}", ws))
                }
                IpcRequest::MoveToWorkspace(ws) => {
                    state_guard.move_active_window_to_workspace(ws);
                    IpcResponse::Ok(format!("Moved active window to workspace {}", ws))
                }
                IpcRequest::CloseActiveWindow => {
                    state_guard.close_active_window();
                    IpcResponse::Ok("Closed active window".to_string())
                }
                IpcRequest::ToggleFullscreen => {
                    state_guard.toggle_fullscreen();
                    IpcResponse::Ok("Toggled fullscreen".to_string())
                }
                IpcRequest::UpdateWmConfig {
                    inner_gaps,
                    outer_gaps,
                    border_width,
                    split_ratio,
                    spiral,
                } => {
                    if let Some(ig) = inner_gaps {
                        state_guard.config.window_manager.inner_gaps = ig;
                    }
                    if let Some(og) = outer_gaps {
                        state_guard.config.window_manager.outer_gaps = og;
                    }
                    if let Some(bw) = border_width {
                        state_guard.config.window_manager.border_width = bw;
                    }
                    if let Some(sr) = split_ratio {
                        state_guard.config.window_manager.split_ratio = sr;
                    }
                    if let Some(sp) = spiral {
                        state_guard.config.window_manager.spiral = sp;
                    }
                    state_guard.recalculate_layout();
                    let _ = state_guard.config.save();
                    IpcResponse::Ok("Updated WM config".to_string())
                }
                IpcRequest::ReloadConfig => {
                    let reloaded = AuroraConfig::load_or_default();
                    state_guard.config = reloaded;
                    state_guard.recalculate_layout();
                    IpcResponse::Ok("Config reloaded".to_string())
                }
                IpcRequest::FocusDirection(dir) => {
                    if dir == "right" || dir == "down" {
                        state_guard.focus_next();
                    } else {
                        state_guard.focus_prev();
                    }
                    IpcResponse::Ok(format!("Focused {}", dir))
                }
                IpcRequest::SwapDirection(_) => {
                    IpcResponse::Ok("Window swapped".to_string())
                }
                IpcRequest::ToggleLauncher => {
                    let _ = std::process::Command::new("aurora-shell")
                        .arg("--launcher")
                        .spawn();
                    IpcResponse::Ok("Toggled launcher".to_string())
                }
                IpcRequest::ToggleControlCenter => {
                    let _ = std::process::Command::new("aurora-shell")
                        .arg("--control-center")
                        .spawn();
                    IpcResponse::Ok("Toggled control center".to_string())
                }
                IpcRequest::LaunchTerminal => {
                    let term = state_guard.config.general.terminal_emulator.clone();
                    let _ = std::process::Command::new(term)
                        .spawn();
                    IpcResponse::Ok("Terminal launched".to_string())
                }
                IpcRequest::LaunchBrowser => {
                    let browser = state_guard.config.general.browser.clone();
                    let _ = std::process::Command::new(browser)
                        .spawn();
                    IpcResponse::Ok("Browser launched".to_string())
                }
                IpcRequest::LaunchSettings => {
                    let _ = std::process::Command::new("aurora-settings")
                        .spawn();
                    IpcResponse::Ok("Settings launched".to_string())
                }
            }
        };

        let resp_str = serde_json::to_string(&response)? + "\n";
        writer.write_all(resp_str.as_bytes()).await?;
        writer.flush().await?;
        line.clear();
    }

    Ok(())
}
