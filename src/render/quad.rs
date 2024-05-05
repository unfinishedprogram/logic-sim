
use super::vertex::Vertex;

pub struct Quad {
    p1: Vertex,
    p2: Vertex,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            p1: Vertex::new(-0.5, -0.5),
            p2: Vertex::new(0.5, 0.5),
        }
    }
}

impl Quad {
    pub fn new(p1: impl Into<Vertex>, p2: impl Into<Vertex>) -> Self {
        Self {
            p1: p1.into(),
            p2: p2.into(),
        }
    }

    pub fn triangles(&self) -> Vec<[Vertex;3]> {
        let top_left = self.p1;
        let bottom_right = self.p2;
        let top_right = Vertex::new(top_left.0.x, bottom_right.0.y); // Swap positions
        let bottom_left = Vertex::new(bottom_right.0.x, top_left.0.y); // Swap positions

        let t1 = [bottom_left, top_right, bottom_right];
        let t2 = [top_left, top_right, bottom_left];
        vec![t1, t2]
    }
}
