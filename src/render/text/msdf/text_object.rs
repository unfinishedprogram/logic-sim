use bytemuck::{Pod, Zeroable};
use glam::Vec2;

use crate::render::vertex::VertexUV;

// Defines some text to render
pub struct TextObject {
    content: String,
    position: Vec2,
    scale: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct RenderableChar {
    pub verticies: [VertexUV; 4],
}
