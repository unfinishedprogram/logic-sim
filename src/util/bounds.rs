use glam::{vec2, Vec2};

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
}

impl From<usvg::Rect> for Bounds {
    fn from(value: usvg::Rect) -> Self {
        Self::new(
            vec2(value.left(), value.top()),
            vec2(value.right(), value.bottom()),
        )
    }
}
