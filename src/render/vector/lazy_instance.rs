use std::collections::HashMap;

use assets::SVGSource;
use glam::{Vec2, Vec4};

use crate::render::vector::{draw_call_ordering::GroupedInstances, instance::ZIndex};

use super::{VectorInstance, VectorRenderer, renderer::SVGSourceId};

#[derive(Clone, Copy)]
pub struct LazyVectorInstance<'a> {
    pub source: &'a SVGSource,
    pub transform: Vec2,
    pub scale: Vec2,
    pub color: Vec4,
    pub z_index: ZIndex,
}

impl VectorRenderer {
    pub fn convert_lazy_instances(
        &mut self,
        instances: &[LazyVectorInstance<'static>],
    ) -> GroupedInstances {
        let mut res = HashMap::new();

        for instance in instances.iter() {
            let id = SVGSourceId::of(instance.source);
            let handle = self.get_vector(id);
            let handle = handle.unwrap_or_else(|| {
                let geometry = self.tessellator.tesselate(instance.source);
                self.add_vector_object(id, geometry)
            });

            res.entry(instance.z_index)
                .or_insert_with(HashMap::new)
                .entry(handle)
                .or_insert_with(Vec::new)
                .push(VectorInstance {
                    id: handle,
                    transform: instance.transform,
                    color: instance.color,
                    scale: instance.scale,
                    z_index: instance.z_index,
                });
        }

        res
    }
}
