use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::vertex_attr_array;

// A vertex with UV data
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct VertexUV(pub Vec2, pub Vec2);

impl VertexUV {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
    ];

    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self(Vec2::new(x, y), Vec2::new(u, v))
    }

    pub fn buffer_layout_descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
