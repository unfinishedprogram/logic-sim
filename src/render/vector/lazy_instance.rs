use glam::{Vec2, Vec4};

use super::{tessellator::GLOBAL_TESSELLATOR, VectorInstance, VectorRenderer};

#[derive(Clone, Copy)]
pub struct LazyVectorInstance<'a> {
    pub source: &'a str,
    pub transform: Vec2,
    pub color: Vec4,
    pub scale: Vec2,
    pub z_index: u16,
}

impl VectorRenderer {
    pub fn convert_lazy_instance(&mut self, instance: &LazyVectorInstance) -> VectorInstance {
        let reference = self.reference();
        let handle = reference.vectors.get(instance.source).copied();

        let handle = handle.unwrap_or_else(|| {
            let geometry = GLOBAL_TESSELLATOR.get_geometry(instance.source);
            self.add_vector_object(geometry)
        });

        VectorInstance {
            id: handle,
            transform: instance.transform,
            color: instance.color,
            scale: instance.scale,
            z_index: instance.z_index,
        }
    }
}
