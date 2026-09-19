use crate::geometry::Rect;
use crate::layout::{calculate_tiled_rects, LayoutKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedWindow {
    pub hwnd: isize,
    pub title: String,
    pub class_name: String,
    pub is_floating: bool,
    pub rect: Rect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub layout: LayoutKind,
    pub windows: Vec<ManagedWindow>,
    pub focused_window_index: Option<usize>,
}

impl Workspace {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            layout: LayoutKind::Bsp,
            windows: Vec::new(),
            focused_window_index: None,
        }
    }

    pub fn add_window(&mut self, hwnd: isize, title: String, class_name: String) {
        if self.windows.iter().any(|w| w.hwnd == hwnd) {
            return;
        }
        self.windows.push(ManagedWindow {
            hwnd,
            title,
            class_name,
            is_floating: false,
            rect: Rect::default(),
        });
        self.focused_window_index = Some(self.windows.len() - 1);
    }

    pub fn remove_window(&mut self, hwnd: isize) -> bool {
        if let Some(pos) = self.windows.iter().position(|w| w.hwnd == hwnd) {
            self.windows.remove(pos);
            if self.windows.is_empty() {
                self.focused_window_index = None;
            } else if let Some(focused) = self.focused_window_index {
                if focused >= self.windows.len() {
                    self.focused_window_index = Some(self.windows.len() - 1);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn recompute_layout(&mut self, screen_bounds: Rect, gap: i32) -> Vec<(isize, Rect)> {
        let tiled_windows: Vec<usize> = self.windows
            .iter()
            .enumerate()
            .filter(|(_, w)| !w.is_floating)
            .map(|(i, _)| i)
            .collect();

        let rects = calculate_tiled_rects(screen_bounds, tiled_windows.len(), gap);
        let mut updates = Vec::new();

        for (idx, &win_idx) in tiled_windows.iter().enumerate() {
            if let Some(rect) = rects.get(idx) {
                self.windows[win_idx].rect = *rect;
                updates.push((self.windows[win_idx].hwnd, *rect));
            }
        }

        updates
    }
}
