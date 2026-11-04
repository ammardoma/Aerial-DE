use crate::layout::Rect;
use aurora_config::LayoutMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub floating_geometry: Rect,
    pub tiled_geometry: Rect,
    pub is_floating: bool,
    pub is_fullscreen: bool,
    pub workspace_id: u32,
    pub pid: Option<u32>,
}

impl Window {
    pub fn new(id: WindowId, title: String, app_id: String, workspace_id: u32, default_rect: Rect) -> Self {
        Self {
            id,
            title,
            app_id,
            floating_geometry: default_rect,
            tiled_geometry: default_rect,
            is_floating: false,
            is_fullscreen: false,
            workspace_id,
            pid: None,
        }
    }

    pub fn current_geometry(&self, wm_mode: LayoutMode) -> Rect {
        if self.is_fullscreen {
            // Fullscreen uses entire display output
            Rect::new(0, 0, 1920, 1080)
        } else if self.is_floating || wm_mode == LayoutMode::Floating {
            self.floating_geometry
        } else {
            self.tiled_geometry
        }
    }
}

pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub windows: Vec<WindowId>,
    pub focused_window: Option<WindowId>,
}

impl Workspace {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: format!("{}", id),
            windows: Vec::new(),
            focused_window: None,
        }
    }

    pub fn add_window(&mut self, id: WindowId) {
        if !self.windows.contains(&id) {
            self.windows.push(id);
        }
        self.focused_window = Some(id);
    }

    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.retain(|&w| w != id);
        if self.focused_window == Some(id) {
            self.focused_window = self.windows.last().copied();
        }
    }
}
