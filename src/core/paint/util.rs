
use crate::{Rect, Vec2};

pub(super) fn map_screen_space_rect_to_clip_space(rect: Rect, win_size: Vec2) -> Rect {
    rect.map(
        Rect::min_size(Vec2::ZERO, win_size),
        Rect::min_max(Vec2::NEG_ONE, Vec2::ONE) 
    )
}
