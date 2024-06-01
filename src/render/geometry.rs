use super::vertex::VertexUV;

#[repr(C)]
pub struct TexturedQuad {
    pub vertices: [VertexUV; 4],
    pub indices: [u32; 6],
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
            vertices: [bottom_left, top_right, bottom_right, top_left],
            indices: [0, 1, 2, 3, 1, 0],
        }
    }
}
