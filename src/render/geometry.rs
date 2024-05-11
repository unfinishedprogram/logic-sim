use bytemuck::{Pod, Zeroable};

use super::vertex::VertexUV;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TexturedQuad {
    pub verticies: [VertexUV; 6],
}

impl TexturedQuad {
    pub fn new(p1: VertexUV, p2: VertexUV) -> Self {
        let top_left = p1;
        let bottom_right = p2;

        let top_right = VertexUV::new(
            top_left.0.x,
            bottom_right.0.y,
            top_left.1.x,
            bottom_right.1.y,
        );
        let bottom_left = VertexUV::new(
            bottom_right.0.x,
            top_left.0.y,
            bottom_right.1.x,
            top_left.1.y,
        );
        Self {
            verticies: [
                bottom_left,
                top_right,
                bottom_right,
                top_left,
                top_right,
                bottom_left,
            ],
        }
    }
}
