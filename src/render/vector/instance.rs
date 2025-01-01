use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use wgpu::vertex_attr_array;

use super::svg_geometry::SVGGeometry;
use common::handle::Handle;

pub type ZIndex = u16;

#[derive(Clone, Copy)]
pub struct VectorInstance {
    pub id: Handle<SVGGeometry>,
    pub transform: Vec2,
    pub scale: Vec2,
    pub color: Vec4,
    pub z_index: ZIndex,
}

impl VectorInstance {
    #[inline(always)]
    pub fn new(id: Handle<SVGGeometry>) -> Self {
        Self {
            id,
            transform: Vec2::ZERO,
            scale: Vec2::splat(1.0),
            color: Vec4::ONE,
            z_index: 0,
        }
    }

    #[inline(always)]
    pub fn with_transform(mut self, transform: Vec2) -> Self {
        self.transform = transform;
        self
    }

    #[inline(always)]
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    #[inline(always)]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct RawInstance {
    pub transform: Vec2,
    pub scale: Vec2,
    pub color: Vec4,
}

impl From<VectorInstance> for RawInstance {
    fn from(value: VectorInstance) -> Self {
        Self {
            transform: value.transform,
            scale: value.scale,
            color: value.color,
        }
    }
}

impl RawInstance {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] = vertex_attr_array![
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32x4
    ];

    pub fn buffer_layout_descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
