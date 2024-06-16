use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use wgpu::vertex_attr_array;

use crate::util::handle::Handle;

use super::vector_object::VectorObject;

#[derive(Clone, Copy)]
pub struct Instance {
    pub id: Handle<VectorObject>,
    pub transform: Vec2,
    pub color: Vec4,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct RawInstance {
    pub transform: Vec2,
    pub padding: u64,
    pub color: Vec4,
}

impl From<Instance> for RawInstance {
    fn from(value: Instance) -> Self {
        Self {
            transform: value.transform,
            padding: 0,
            color: value.color,
        }
    }
}

impl RawInstance {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![
        1 => Float32x4,
        2 => Float32x4
    ];

    pub fn buffer_layout_descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
