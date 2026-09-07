use std::collections::HashMap;

use super::{
    VectorInstance,
    instance::{RawInstance, ZIndex},
    svg_geometry::SVGGeometry,
};

use common::{handle::Handle, profiler};

pub struct DrawCall {
    pub id: Handle<SVGGeometry>,
    pub range: std::ops::Range<u32>,
}

#[derive(Default)]
pub struct VectorRenderRequest {
    pub instances_buf: Vec<RawInstance>,
    pub draw_calls: Vec<DrawCall>,
}

// Instances bucketed by z-index, then by the geometry they draw
pub type GroupedInstances = HashMap<ZIndex, HashMap<Handle<SVGGeometry>, Vec<VectorInstance>>>;

// Orders draw calls to support z-indexing
//
// Instances arrive already grouped by geometry, so each inner entry is exactly
// one draw call. Both levels are sorted before emitting: z-index ascending so
// lower layers draw first, and geometry handle so that ordering within a layer
// is stable between frames - `HashMap` iteration order is not.
pub fn create_render_request(
    instances: GroupedInstances,
    profiler: &mut profiler::Profiler,
) -> VectorRenderRequest {
    profiler.begin("group by z-index");
    let mut z_index_groups: Vec<_> = instances.into_iter().collect();
    z_index_groups.sort_unstable_by_key(|(z_index, _)| *z_index);
    profiler.end("group by z-index");

    profiler.begin("create draw calls");
    let mut draw_calls: Vec<DrawCall> = Vec::new();
    let mut instances_buf: Vec<RawInstance> = Vec::new();

    for (_, by_geometry) in z_index_groups {
        let mut by_geometry: Vec<_> = by_geometry.into_iter().collect();
        by_geometry.sort_unstable_by_key(|(id, _)| id.index);

        for (id, instances) in by_geometry {
            let start = instances_buf.len() as u32;
            instances_buf.extend(instances.into_iter().map(RawInstance::from));

            draw_calls.push(DrawCall {
                id,
                range: start..instances_buf.len() as u32,
            });
        }
    }
    profiler.end("create draw calls");

    VectorRenderRequest {
        instances_buf,
        draw_calls,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::Vec2;

    // `transform.x` is used as a tag so instances can be identified in the
    // flattened output buffer.
    fn instance(id: Handle<SVGGeometry>, z_index: ZIndex, tag: f32) -> VectorInstance {
        let mut instance = VectorInstance::new(id).with_transform(Vec2::splat(tag));
        instance.z_index = z_index;
        instance
    }

    fn request_of(map: GroupedInstances) -> VectorRenderRequest {
        create_render_request(map, &mut profiler::Profiler::default())
    }

    #[test]
    fn every_geometry_in_a_layer_gets_a_draw_call() {
        let mut map = GroupedInstances::new();
        let layer = map.entry(0).or_default();
        for index in 0..32 {
            let id = Handle::new(index);
            layer.insert(id, vec![instance(id, 0, index as f32)]);
        }

        let request = request_of(map);

        assert_eq!(request.draw_calls.len(), 32);
        assert_eq!(request.instances_buf.len(), 32);
        assert_eq!(
            request
                .draw_calls
                .iter()
                .map(|call| call.id.index)
                .collect::<Vec<_>>(),
            (0..32).collect::<Vec<_>>()
        );
    }

    #[test]
    fn draw_calls_are_ordered_by_z_index_then_geometry() {
        let mut map = GroupedInstances::new();
        for (z_index, handle) in [(2, 5), (0, 9), (0, 4), (1, 8)] {
            let id = Handle::new(handle);
            map.entry(z_index)
                .or_default()
                .insert(id, vec![instance(id, z_index, 0.0)]);
        }

        let ids: Vec<usize> = request_of(map)
            .draw_calls
            .iter()
            .map(|call| call.id.index)
            .collect();

        // z 0 first (handles ascending), then z 1, then z 2
        assert_eq!(ids, [4, 9, 8, 5]);
    }

    #[test]
    fn ranges_tile_the_buffer_and_keep_every_instance() {
        let mut map = GroupedInstances::new();
        for (z_index, handle, tags) in [
            (1u8, 7usize, vec![1.0f32, 2.0]),
            (0, 3, vec![3.0]),
            (0, 9, vec![4.0, 5.0, 6.0]),
        ] {
            let id = Handle::new(handle);
            let instances = tags
                .iter()
                .map(|tag| instance(id, z_index, *tag))
                .collect::<Vec<_>>();
            map.entry(z_index).or_default().insert(id, instances);
        }

        let request = request_of(map);

        let mut next = 0;
        for call in request.draw_calls.iter() {
            assert_eq!(
                call.range.start, next,
                "draw call ranges must be contiguous"
            );
            next = call.range.end;
        }
        assert_eq!(next as usize, request.instances_buf.len());

        let mut tags: Vec<f32> = request
            .instances_buf
            .iter()
            .map(|raw| raw.transform.x)
            .collect();
        tags.sort_by(f32::total_cmp);
        assert_eq!(tags, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }
}
