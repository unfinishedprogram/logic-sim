use std::ops::Range;

use super::basic_mesh::BasicMesh;

pub struct Scene {
    pub meshes: Vec<BasicMesh<2>>,
}

impl Scene {
    pub fn vert_draw_range(&self) -> Range<u32> {
        0..BasicMesh::<2>::vert_count() as u32
    }

    pub fn instnace_draw_range(&self) -> Range<u32> {
        0..self.meshes.len() as u32
    }

    pub fn as_vertex_buffer(&self) -> &[u8] {
        bytemuck::cast_slice(&self.meshes)
    }
}
