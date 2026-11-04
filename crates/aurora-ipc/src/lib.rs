use aurora_config::LayoutMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub const DEFAULT_IPC_SOCKET: &str = "/tmp/aurora-wm.sock";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum IpcRequest {
    /// Ping / health check
    Ping,
    /// Get current state (workspaces, layout mode, active window, etc.)
    GetState,
    /// Set window layout mode: Dwindle or Floating
    SetLayoutMode(LayoutMode),
    /// Toggle between dwindle and floating mode
    ToggleLayoutMode,
    /// Switch active workspace (1-9)
    SwitchWorkspace(u32),
    /// Move active window to workspace (1-9)
    MoveToWorkspace(u32),
    /// Close currently focused window
    CloseActiveWindow,
    /// Toggle fullscreen on active window
    ToggleFullscreen,
    /// Change gaps and border settings live
    UpdateWmConfig {
        inner_gaps: Option<u32>,
        outer_gaps: Option<u32>,
        border_width: Option<u32>,
        split_ratio: Option<f32>,
        spiral: Option<bool>,
    },
    /// Reload configuration file
    ReloadConfig,
    /// Focus direction: "left", "right", "up", "down"
    FocusDirection(String),
    /// Swap window in direction: "left", "right", "up", "down"
    SwapDirection(String),
    /// Trigger launcher popup
    ToggleLauncher,
    /// Trigger control center popup
    ToggleControlCenter,
    /// Trigger terminal
    LaunchTerminal,
    /// Trigger browser
    LaunchBrowser,
    /// Trigger settings app
    LaunchSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", content = "data")]
pub enum IpcResponse {
    Ok(String),
    State(WmStateSnapshot),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WmStateSnapshot {
    pub current_workspace: u32,
    pub total_workspaces: u32,
    pub layout_mode: LayoutMode,
    pub active_window_title: Option<String>,
    pub window_count: usize,
    pub inner_gaps: u32,
    pub outer_gaps: u32,
    pub split_ratio: f32,
    pub spiral: bool,
}

pub struct IpcClient {
    socket_path: PathBuf,
}

impl IpcClient {
    pub fn new<P: Into<PathBuf>>(socket_path: P) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    pub fn default_client() -> Self {
        let sock = std::env::var("AURORA_SOCKET").unwrap_or_else(|_| DEFAULT_IPC_SOCKET.to_string());
        Self::new(sock)
    }

    pub async fn send_request(&self, req: IpcRequest) -> Result<IpcResponse, String> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| format!("Failed to connect to IPC socket at {:?}: {}", self.socket_path, e))?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        let json_line = serde_json::to_string(&req).map_err(|e| e.to_string())? + "\n";
        writer
            .write_all(json_line.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        writer.flush().await.map_err(|e| e.to_string())?;

        let mut resp_line = String::new();
        reader
            .read_line(&mut resp_line)
            .await
            .map_err(|e| e.to_string())?;

        let resp: IpcResponse =
            serde_json::from_str(&resp_line).map_err(|e| format!("Invalid JSON response: {}", e))?;
        Ok(resp)
    }
}
