use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};

use crate::render::vertex::VertexUV;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LineGeometry {
    pub vertices: [VertexUV; 6],
}

#[derive(Clone, Copy)]
pub struct LineDescriptor {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
}

impl LineGeometry {
    pub fn from_descriptor(desc: LineDescriptor) -> Self {
        let half_width = desc.width / 2.0;
        let dir = (desc.end - desc.start).normalize();
        let perp = Vec2::new(-dir.y, dir.x) * half_width;

        let vertices = [
            VertexUV {
                position: desc.start + perp,
                uv: Vec2::new(0.0, 0.0),
                color: Vec4::splat(1.0),
            },
            VertexUV {
                position: desc.start - perp,
                uv: Vec2::new(0.0, 1.0),
                color: Vec4::splat(1.0),
            },
            VertexUV {
                position: desc.end + perp,
                uv: Vec2::new(1.0, 0.0),
                color: Vec4::splat(1.0),
            },
            VertexUV {
                position: desc.start - perp,
                uv: Vec2::new(0.0, 1.0),
                color: Vec4::splat(1.0),
            },
            VertexUV {
                position: desc.end - perp,
                uv: Vec2::new(1.0, 1.0),
                color: Vec4::splat(1.0),
            },
            VertexUV {
                position: desc.end + perp,
                uv: Vec2::new(1.0, 0.0),
                color: Vec4::splat(1.0),
            },
        ];

        Self { vertices }
    }
}
