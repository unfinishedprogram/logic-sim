use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::vertex_attr_array;

#[repr(transparent)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex(pub Vec2);

impl From<Vec2> for Vertex {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn buffer_layout_descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float32x2],
        }
    }
}
