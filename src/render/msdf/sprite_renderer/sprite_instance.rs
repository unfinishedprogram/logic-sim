use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SpriteInstance {
    pub sprite_index: u32,
    pub position: Vec2,
    pub scale: f32,
    pub color: Vec4,
}

impl SpriteInstance {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 4] = vertex_attr_array![
        0 => Uint32,
        1 => Float32x2,
        2 => Float32,
        3 => Float32x4
    ];

    pub fn buffer_layout_descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
