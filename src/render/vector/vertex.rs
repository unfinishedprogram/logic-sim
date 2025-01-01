use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use wgpu::{vertex_attr_array, VertexAttribute};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SVGVertex {
    pub color: Vec4,
    pub position: Vec2,
    _padding: Vec2,
}

impl SVGVertex {
    const VERTEX_ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x4,
        1 => Float32x2,
    ];

    pub fn new(position: Vec2, color: Vec4) -> Self {
        Self {
            position,
            color,
            _padding: Vec2::ZERO,
        }
    }

    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
