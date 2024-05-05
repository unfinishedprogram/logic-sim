
use super::{basic_mesh::BasicMesh, vertex::Vertex};

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

impl From<Quad> for BasicMesh<2> {
    fn from(value: Quad) -> Self {
        let top_left = value.p1;
        let bottom_right = value.p2;
        let top_right = Vertex::new(top_left.0.x, bottom_right.0.y); // Swap positions
        let bottom_left = Vertex::new(bottom_right.0.x, top_left.0.y); // Swap positions

        let t1 = [bottom_left, top_right, bottom_right];
        let t2 = [top_left, top_right, bottom_left];

        Self { tris: [t1, t2] }
    }
}
