use bytemuck::{Pod, Zeroable};

use super::vertex::Vertex;

type Triangle = [Vertex; 3];

#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct BasicMesh<const C: usize> {
    pub(crate) tris: [Triangle; C],
}

impl<const C: usize> BasicMesh<C> {
    pub const fn vert_count() -> usize {
        C * 3
    }
}
