use crate::geometry::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutKind {
    Bsp,
    Columns,
    Monocle,
    Floating,
}

impl Default for LayoutKind {
    fn default() -> Self {
        Self::Bsp
    }
}

pub fn calculate_tiled_rects(bounds: Rect, count: usize, gap: i32) -> Vec<Rect> {
    if count == 0 {
        return Vec::new();
    }
    if count == 1 {
        return vec![bounds.apply_gaps(gap)];
    }

    let mut rects = Vec::with_capacity(count);
    let mut current_bounds = bounds;

    for i in 0..count {
        if i == count - 1 {
            rects.push(current_bounds.apply_gaps(gap));
            break;
        }

        let split_horizontal = current_bounds.width > current_bounds.height;
        if split_horizontal {
            let half_w = current_bounds.width / 2;
            let left = Rect::new(current_bounds.x, current_bounds.y, half_w, current_bounds.height);
            rects.push(left.apply_gaps(gap));
            current_bounds = Rect::new(current_bounds.x + half_w, current_bounds.y, current_bounds.width - half_w, current_bounds.height);
        } else {
            let half_h = current_bounds.height / 2;
            let top = Rect::new(current_bounds.x, current_bounds.y, current_bounds.width, half_h);
            rects.push(top.apply_gaps(gap));
            current_bounds = Rect::new(current_bounds.x, current_bounds.y + half_h, current_bounds.width, current_bounds.height - half_h);
        }
    }

    rects
}
