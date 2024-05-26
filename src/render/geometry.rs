use bytemuck::{Pod, Zeroable};

use super::vertex::VertexUV;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TexturedQuad {
    pub vertices: [VertexUV; 6],
}

impl TexturedQuad {
    pub fn new(p1: VertexUV, p2: VertexUV) -> Self {
        let top_left = p1;
        let bottom_right = p2;

        let top_right = VertexUV::new(
            top_left.position.x,
            bottom_right.position.y,
            top_left.uv.x,
            bottom_right.uv.y,
            top_left.color,
        );
        let bottom_left = VertexUV::new(
            bottom_right.position.x,
            top_left.position.y,
            bottom_right.uv.x,
            top_left.uv.y,
            top_left.color,
        );
        Self {
            vertices: [
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
