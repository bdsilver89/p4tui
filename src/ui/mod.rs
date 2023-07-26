mod scrollbar;
pub mod style;

pub use scrollbar::{draw_scrollbar, Orientation};

use ratatui::layout::Rect;

pub fn centered_rect_absolute(width: u16, height: u16, r: Rect) -> Rect {
    Rect::new(
        (r.width.saturating_sub(width)) / 2,
        (r.height.saturating_sub(height)) / 2,
        width.min(r.width),
        height.min(r.height),
    )
}
