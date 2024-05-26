use glam::Vec2;

pub struct Bounds {
    a: Vec2,
    b: Vec2,
}

impl Bounds {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.a.x && point.x <= self.b.x && point.y >= self.a.y && point.y <= self.b.y
    }

    pub fn translate(&self, offset: Vec2) -> Self {
        Self {
            a: self.a + offset,
            b: self.b + offset,
        }
    }
}
