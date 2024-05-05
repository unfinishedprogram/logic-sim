use super::basic_mesh::BasicMesh;

pub struct Scene {
    pub meshes: Vec<BasicMesh<2>>,
}

impl Scene {
    pub fn as_vertex_buffer(&self) -> &[u8] {
        bytemuck::cast_slice(&self.meshes)
    }
}
