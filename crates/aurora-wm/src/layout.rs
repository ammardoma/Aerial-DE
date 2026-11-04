use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn center(&self) -> (i32, i32) {
        (
            self.x + (self.width as i32 / 2),
            self.y + (self.height as i32 / 2),
        )
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x
            && px < self.x + self.width as i32
            && py >= self.y
            && py < self.y + self.height as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitType {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Right,
    Down,
    Left,
    Up,
}

/// Dwindle Layout Engine:
/// Computes tiling geometries recursively using binary space partitioning with spiral or alternating cuts.
/// Windows are organized cleanly with inner and outer gaps.
pub struct DwindleEngine {
    pub inner_gaps: u32,
    pub outer_gaps: u32,
    pub split_ratio: f32,
    pub spiral: bool,
}

impl DwindleEngine {
    pub fn new(inner_gaps: u32, outer_gaps: u32, split_ratio: f32, spiral: bool) -> Self {
        Self {
            inner_gaps,
            outer_gaps,
            split_ratio,
            spiral,
        }
    }

    /// Calculate window rectangles for `count` windows given a total available screen area `screen`.
    pub fn calculate_layout(&self, screen: Rect, count: usize) -> Vec<Rect> {
        if count == 0 {
            return Vec::new();
        }

        // Apply outer gaps
        let x = screen.x + self.outer_gaps as i32;
        let y = screen.y + self.outer_gaps as i32;
        let width = screen
            .width
            .saturating_sub(self.outer_gaps * 2)
            .max(100);
        let height = screen
            .height
            .saturating_sub(self.outer_gaps * 2)
            .max(100);

        let initial_rect = Rect::new(x, y, width, height);

        if count == 1 {
            return vec![initial_rect];
        }

        let mut rects = Vec::with_capacity(count);
        let mut current_rect = initial_rect;

        for i in 0..count {
            let is_last = i == count - 1;
            if is_last {
                rects.push(current_rect);
                break;
            }

            // Decide split orientation:
            // If spiral is true, cycle: 0 -> Vertical (left/right), 1 -> Horizontal (top/bottom), 2 -> Vertical, etc.
            // Or adaptively split the longest side.
            let split_vertical = if self.spiral {
                i % 2 == 0
            } else {
                current_rect.width >= current_rect.height
            };

            let inner_gap = self.inner_gaps;

            if split_vertical {
                // Split horizontally: left and right parts
                let usable_w = current_rect.width.saturating_sub(inner_gap);
                let first_w = ((usable_w as f32) * self.split_ratio).round() as u32;
                let second_w = usable_w.saturating_sub(first_w);

                let first = Rect::new(current_rect.x, current_rect.y, first_w, current_rect.height);
                let second = Rect::new(
                    current_rect.x + (first_w + inner_gap) as i32,
                    current_rect.y,
                    second_w,
                    current_rect.height,
                );

                rects.push(first);
                current_rect = second;
            } else {
                // Split vertically: top and bottom parts
                let usable_h = current_rect.height.saturating_sub(inner_gap);
                let first_h = ((usable_h as f32) * self.split_ratio).round() as u32;
                let second_h = usable_h.saturating_sub(first_h);

                let first = Rect::new(current_rect.x, current_rect.y, current_rect.width, first_h);
                let second = Rect::new(
                    current_rect.x,
                    current_rect.y + (first_h + inner_gap) as i32,
                    current_rect.width,
                    second_h,
                );

                rects.push(first);
                current_rect = second;
            }
        }

        rects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwindle_single_window() {
        let engine = DwindleEngine::new(8, 12, 0.5, true);
        let screen = Rect::new(0, 32, 1920, 1048);
        let rects = engine.calculate_layout(screen, 1);
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].x, 12);
        assert_eq!(rects[0].y, 44);
        assert_eq!(rects[0].width, 1920 - 24);
        assert_eq!(rects[0].height, 1048 - 24);
    }

    #[test]
    fn test_dwindle_multiple_windows() {
        let engine = DwindleEngine::new(10, 10, 0.5, true);
        let screen = Rect::new(0, 0, 1000, 1000);
        let rects = engine.calculate_layout(screen, 3);
        assert_eq!(rects.len(), 3);
        // All windows must stay inside usable area
        for r in rects {
            assert!(r.x >= 10);
            assert!(r.y >= 10);
            assert!((r.x + r.width as i32) <= 990);
            assert!((r.y + r.height as i32) <= 990);
        }
    }
}
