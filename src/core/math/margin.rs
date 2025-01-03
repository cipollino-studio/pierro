use super::{vec2, Axis, Range, Rect, Vec2};


#[derive(Clone, Copy)]
pub struct Margin {
    /// The margin at the minimum corner of the rectange (left, top)
    pub min: Vec2,
    /// The margin at the maximum corner of the rectangle (right, bottom)
    pub max: Vec2 
}

impl Margin {

    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            min: vec2(left, top),
            max: vec2(right, bottom)
        }
    }

    pub const fn same(margin: f32) -> Self {
        Self::new(margin, margin, margin, margin)
    }

    pub const fn horizontal(margin: f32) -> Self {
        Self::new(margin, 0.0, margin, 0.0)
    }

    pub const fn vertical(margin: f32) -> Self {
        Self::new(0.0, margin, 0.0, margin)
    }

    pub const ZERO: Self = Self::same(0.0);

    pub fn total(&self) -> Vec2 {
        self.min + self.max
    }

    pub fn on_axis(&self, axis: Axis) -> (f32, f32) {
        (self.min.on_axis(axis), self.max.on_axis(axis))
    }

    pub fn apply_on_axis(&self, space: Range, axis: Axis) -> Range {
        let (margin_min, margin_max) = self.on_axis(axis);
        if space.size() > margin_min + margin_max {
            Range::new(space.min + margin_min, space.max - margin_max)
        } else {
            Range::point(space.center())
        }
    }

    pub fn apply(&self, rect: Rect) -> Rect {
        Rect::from_ranges(
            self.apply_on_axis(rect.x_range(), Axis::X),
            self.apply_on_axis(rect.y_range(), Axis::Y) 
        )
    }

}
