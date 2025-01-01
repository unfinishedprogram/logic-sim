use std::collections::HashMap;

use super::{
    instance::{RawInstance, ZIndex},
    svg_geometry::SVGGeometry,
    VectorInstance,
};

use common::handle::Handle;

pub struct DrawCall {
    pub id: Handle<SVGGeometry>,
    pub range: std::ops::Range<u32>,
}

#[derive(Default)]
pub struct VectorRenderRequest {
    pub instances_buf: Vec<RawInstance>,
    pub draw_calls: Vec<DrawCall>,
}

// Returns a sorted vector of instances, grouped by z-index
// Each group is then sorted by instanceID
fn group_by_z_index(instances: Vec<VectorInstance>) -> Vec<Vec<VectorInstance>> {
    let mut unique_z_indexes: HashMap<ZIndex, Vec<VectorInstance>> = HashMap::new();

    // First priority is the z-index
    for instance in instances {
        if let Some(z_group) = unique_z_indexes.get_mut(&instance.z_index) {
            z_group.push(instance);
        } else {
            unique_z_indexes.insert(instance.z_index, vec![instance]);
        }
    }

    let mut z_indexes: Vec<(u16, Vec<VectorInstance>)> = unique_z_indexes.into_iter().collect();

    z_indexes.sort_by_key(|(z_index, _)| *z_index);
    let mut z_indexes = z_indexes
        .into_iter()
        .map(|(_, instances)| instances)
        .collect::<Vec<_>>();

    // Second priority is the instance ID
    // This is mostly just so we can render as many instances as possible within the same draw call
    for z_index_group in z_indexes.iter_mut() {
        z_index_group.sort_by_key(|instance| instance.id.index);
    }

    z_indexes
}

// Orders draw calls to support z-indexing
pub fn create_render_request(instances: Vec<VectorInstance>) -> VectorRenderRequest {
    let z_index_groups = group_by_z_index(instances);

    let mut draw_calls: Vec<DrawCall> = Vec::new();
    let mut instances_buf: Vec<RawInstance> = Vec::new();

    for group in z_index_groups {
        let mut start = 0;
        let mut current_id;

        while start < group.len() {
            current_id = group[start].id;
            let end = group.partition_point(|it| it.id.index <= current_id.index);
            let instances = &group[start..end];
            instances_buf.extend(instances.iter().map(|it| RawInstance::from(*it)));

            draw_calls.push(DrawCall {
                id: current_id,
                range: (instances_buf.len() - instances.len()) as u32..instances_buf.len() as u32,
            });
            start = end
        }
    }

    VectorRenderRequest {
        instances_buf,
        draw_calls,
    }
}
