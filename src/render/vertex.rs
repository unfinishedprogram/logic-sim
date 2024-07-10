use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use wgpu::vertex_attr_array;

// A vertex with UV data
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
pub struct VertexUV {
    pub position: Vec2,
    pub uv: Vec2,
    pub color: Vec4,
}

impl VertexUV {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32x4
    ];

    pub fn new(x: f32, y: f32, u: f32, v: f32, c: Vec4) -> Self {
        Self {
            position: Vec2::new(x, y),
            uv: Vec2::new(u, v),
            color: c,
        }
    }

    pub fn buffer_layout_descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
