use crate::layout::{DwindleEngine, Rect};
use crate::window::{Window, WindowId, Workspace};
use aurora_config::{AuroraConfig, LayoutMode};
use aurora_ipc::WmStateSnapshot;
use std::collections::HashMap;

pub struct WmState {
    pub config: AuroraConfig,
    pub screen: Rect,
    pub workspaces: HashMap<u32, Workspace>,
    pub windows: HashMap<WindowId, Window>,
    pub current_workspace: u32,
    pub next_window_id: u64,
}

impl WmState {
    pub fn new(config: AuroraConfig, screen: Rect) -> Self {
        let mut workspaces = HashMap::new();
        for i in 1..=9 {
            workspaces.insert(i, Workspace::new(i));
        }

        Self {
            config,
            screen,
            workspaces,
            windows: HashMap::new(),
            current_workspace: 1,
            next_window_id: 1,
        }
    }

    pub fn recalculate_layout(&mut self) {
        let current_ws_id = self.current_workspace;
        let ws = match self.workspaces.get(&current_ws_id) {
            Some(w) => w,
            None => return,
        };

        let tiled_ids: Vec<WindowId> = ws
            .windows
            .iter()
            .copied()
            .filter(|id| {
                if let Some(win) = self.windows.get(id) {
                    !win.is_floating && !win.is_fullscreen
                } else {
                    false
                }
            })
            .collect();

        // Calculate top bar and dock offsets if present
        let top_offset = if self.config.top_bar.enabled {
            self.config.top_bar.height as i32
        } else {
            0
        };

        let dock_offset = if self.config.dock.enabled && self.config.dock.position == "bottom" {
            // Leave space for floating dock at the bottom
            (self.config.dock.icon_size + 24) as i32
        } else {
            0
        };

        let usable_screen = Rect::new(
            self.screen.x,
            self.screen.y + top_offset,
            self.screen.width,
            self.screen
                .height
                .saturating_sub((top_offset + dock_offset) as u32),
        );

        let wm = &self.config.window_manager;
        let engine = DwindleEngine::new(
            wm.inner_gaps,
            wm.outer_gaps,
            wm.split_ratio,
            wm.spiral,
        );

        let geometries = engine.calculate_layout(usable_screen, tiled_ids.len());

        for (id, geom) in tiled_ids.into_iter().zip(geometries) {
            if let Some(win) = self.windows.get_mut(&id) {
                win.tiled_geometry = geom;
            }
        }
    }

    pub fn spawn_window(&mut self, title: String, app_id: String) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;

        // macOS styled initial floating geometry centered
        let width = 800.min(self.screen.width - 40);
        let height = 550.min(self.screen.height - 80);
        let x = self.screen.x + ((self.screen.width.saturating_sub(width)) / 2) as i32;
        let y = self.screen.y + 40 + ((self.screen.height.saturating_sub(height)) / 4) as i32;

        let win = Window::new(id, title, app_id, self.current_workspace, Rect::new(x, y, width, height));
        self.windows.insert(id, win);

        if let Some(ws) = self.workspaces.get_mut(&self.current_workspace) {
            ws.add_window(id);
        }

        self.recalculate_layout();
        id
    }

    pub fn close_window(&mut self, id: WindowId) {
        if let Some(win) = self.windows.remove(&id) {
            if let Some(ws) = self.workspaces.get_mut(&win.workspace_id) {
                ws.remove_window(id);
            }
            self.recalculate_layout();
        }
    }

    pub fn close_active_window(&mut self) {
        if let Some(ws) = self.workspaces.get(&self.current_workspace) {
            if let Some(focused) = ws.focused_window {
                self.close_window(focused);
            }
        }
    }

    pub fn switch_workspace(&mut self, ws_id: u32) {
        if (1..=9).contains(&ws_id) {
            self.current_workspace = ws_id;
            self.recalculate_layout();
        }
    }

    pub fn move_active_window_to_workspace(&mut self, target_ws_id: u32) {
        if !(1..=9).contains(&target_ws_id) || target_ws_id == self.current_workspace {
            return;
        }

        let focused = self
            .workspaces
            .get(&self.current_workspace)
            .and_then(|ws| ws.focused_window);

        if let Some(win_id) = focused {
            if let Some(current_ws) = self.workspaces.get_mut(&self.current_workspace) {
                current_ws.remove_window(win_id);
            }
            if let Some(target_ws) = self.workspaces.get_mut(&target_ws_id) {
                target_ws.add_window(win_id);
            }
            if let Some(win) = self.windows.get_mut(&win_id) {
                win.workspace_id = target_ws_id;
            }
            self.recalculate_layout();
        }
    }

    pub fn toggle_layout_mode(&mut self) {
        self.config.window_manager.layout_mode = match self.config.window_manager.layout_mode {
            LayoutMode::Dwindle => LayoutMode::Floating,
            LayoutMode::Floating => LayoutMode::Dwindle,
        };
        self.recalculate_layout();
    }

    pub fn set_layout_mode(&mut self, mode: LayoutMode) {
        self.config.window_manager.layout_mode = mode;
        self.recalculate_layout();
    }

    pub fn toggle_fullscreen(&mut self) {
        let focused = self
            .workspaces
            .get(&self.current_workspace)
            .and_then(|ws| ws.focused_window);

        if let Some(win_id) = focused {
            if let Some(win) = self.windows.get_mut(&win_id) {
                win.is_fullscreen = !win.is_fullscreen;
            }
            self.recalculate_layout();
        }
    }

    pub fn focus_next(&mut self) {
        if let Some(ws) = self.workspaces.get_mut(&self.current_workspace) {
            if ws.windows.is_empty() {
                return;
            }
            let next_idx = match ws.focused_window {
                Some(current) => {
                    let idx = ws.windows.iter().position(|&w| w == current).unwrap_or(0);
                    (idx + 1) % ws.windows.len()
                }
                None => 0,
            };
            ws.focused_window = Some(ws.windows[next_idx]);
        }
    }

    pub fn focus_prev(&mut self) {
        if let Some(ws) = self.workspaces.get_mut(&self.current_workspace) {
            if ws.windows.is_empty() {
                return;
            }
            let prev_idx = match ws.focused_window {
                Some(current) => {
                    let idx = ws.windows.iter().position(|&w| w == current).unwrap_or(0);
                    if idx == 0 {
                        ws.windows.len() - 1
                    } else {
                        idx - 1
                    }
                }
                None => 0,
            };
            ws.focused_window = Some(ws.windows[prev_idx]);
        }
    }

    pub fn snapshot(&self) -> WmStateSnapshot {
        let focused_title = self
            .workspaces
            .get(&self.current_workspace)
            .and_then(|ws| ws.focused_window)
            .and_then(|id| self.windows.get(&id))
            .map(|w| w.title.clone());

        let window_count = self
            .workspaces
            .get(&self.current_workspace)
            .map(|ws| ws.windows.len())
            .unwrap_or(0);

        let wm = &self.config.window_manager;
        WmStateSnapshot {
            current_workspace: self.current_workspace,
            total_workspaces: 9,
            layout_mode: wm.layout_mode,
            active_window_title: focused_title,
            window_count,
            inner_gaps: wm.inner_gaps,
            outer_gaps: wm.outer_gaps,
            split_ratio: wm.split_ratio,
            spiral: wm.spiral,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wm_state_lifecycle() {
        let config = AuroraConfig::default();
        let screen = Rect::new(0, 0, 1920, 1080);
        let mut state = WmState::new(config, screen);

        assert_eq!(state.current_workspace, 1);
        let w1 = state.spawn_window("Alacritty".into(), "alacritty".into());
        let w2 = state.spawn_window("Firefox".into(), "firefox".into());

        assert_eq!(state.windows.len(), 2);
        let ws1 = state.workspaces.get(&1).unwrap();
        assert_eq!(ws1.windows.len(), 2);
        assert_eq!(ws1.focused_window, Some(w2));

        // Test dwindle layout was calculated
        let win1 = state.windows.get(&w1).unwrap();
        let win2 = state.windows.get(&w2).unwrap();
        assert_ne!(win1.tiled_geometry.width, 0);
        assert_ne!(win2.tiled_geometry.width, 0);

        // Move active window to workspace 2
        state.move_active_window_to_workspace(2);
        assert_eq!(state.workspaces.get(&1).unwrap().windows.len(), 1);
        assert_eq!(state.workspaces.get(&2).unwrap().windows.len(), 1);

        // Switch to workspace 2
        state.switch_workspace(2);
        assert_eq!(state.current_workspace, 2);

        // Toggle layout mode
        state.toggle_layout_mode();
        assert_eq!(state.config.window_manager.layout_mode, LayoutMode::Floating);
    }
}
