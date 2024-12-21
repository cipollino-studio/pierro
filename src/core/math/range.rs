
#[derive(Clone, Copy)]
pub struct Range {
    pub min: f32,
    pub max: f32
}

impl Range {

    pub const fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn min_size(min: f32, size: f32) -> Self {
        Self::new(min, min + size)
    }

    pub fn point(pt: f32) -> Self {
        Self::min_size(pt, 0.0)
    }

    pub fn size(&self) -> f32 {
        self.max - self.min 
    }

    pub fn contains(&self, x: f32) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn intersect(&self, other: Range) -> Self {
        Self::new(
            self.min.max(other.min),
            self.max.min(other.max)
        )
    }

    pub fn center(&self) -> f32 {
        (self.min + self.max) / 2.0
    }

}
