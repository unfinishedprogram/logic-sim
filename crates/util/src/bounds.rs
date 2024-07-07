use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl Bounds {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            top_left: a,
            bottom_right: b,
        }
    }

    pub fn from_points(a: Vec2, b: Vec2) -> Self {
        Self::new(a.min(b), a.max(b))
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y <= self.bottom_right.y
    }

    pub fn translate(&self, offset: Vec2) -> Self {
        Self {
            top_left: self.top_left + offset,
            bottom_right: self.bottom_right + offset,
        }
    }

    pub fn from_center_and_size(center: Vec2, size: Vec2) -> Self {
        let half_size = size / 2.0;
        Self {
            top_left: center - half_size,
            bottom_right: center + half_size,
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.top_left + self.bottom_right) / 2.0
    }

    pub fn scale(&self, scale: Vec2) -> Self {
        let center = self.center();
        let half_size = (self.bottom_right - self.top_left) / 2.0;
        let new_half_size = half_size * scale;
        Self {
            top_left: center - new_half_size,
            bottom_right: center + new_half_size,
        }
    }

    pub fn pad(&self, padding: f32) -> Self {
        let padding = Vec2::splat(padding);
        Self {
            top_left: self.top_left - padding,
            bottom_right: self.bottom_right + padding,
        }
    }

    pub fn overlaps(&self, other: &Bounds) -> bool {
        self.top_left.x <= other.bottom_right.x
            && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y
            && self.bottom_right.y >= other.top_left.y
    }
}
