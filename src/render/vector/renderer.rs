use std::collections::HashMap;

use glam::Vec2;
use wgpu::{
    include_wgsl, vertex_attr_array, BindGroupLayout, Buffer, ColorTargetState, Device,
    IndexFormat, PipelineLayout, RenderPass, RenderPipeline, ShaderModule,
};

use crate::{
    render::{
        bindable::Bindable,
        camera::{Camera, CameraUniform},
        helpers, BaseRenderState,
    },
    util::{bounds::Bounds, handle::Handle},
};

use super::{
    instance::{RawInstance, VectorInstance},
    svg_geometry::{Error, SVGGeometry},
};

#[derive(Default, Clone, Debug)]
struct VectorObjectMeta {
    pub vertex_range: std::ops::Range<u32>,
    pub index_range: std::ops::Range<u32>,
    pub instance_range: std::ops::Range<u32>,
}

pub struct VectorRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    camera_binding: CameraUniform,

    vector_objects: Vec<(VectorObjectMeta, SVGGeometry)>,
    vector_lookup: HashMap<String, Handle<SVGGeometry>>,
    render_queue: Vec<Vec<VectorInstance>>,
}

fn vec2_buffer_descriptor() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vec2>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &vertex_attr_array![0 => Float32x2],
    }
}

impl VectorRenderer {
    pub fn create(base: &BaseRenderState) -> Self {
        let shader_module = base.create_shader_module(include_wgsl!("shader.wgsl"));

        let vertex_buffer = base.create_vertex_buffer::<Self>(8192 * 8192);
        let index_buffer = base.create_index_buffer::<Self>(8192 * 512);
        let instance_buffer = base.create_instance_buffer::<Self>(8192 * 512);

        let camera_binding = CameraUniform::create(&base.device);

        let render_pipeline = Self::create_render_pipeline(base, &shader_module, &camera_binding);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            camera_binding,
            vector_lookup: HashMap::new(),
            vector_objects: vec![],
            render_queue: vec![],
        }
    }

    pub fn render<'pass, 'a: 'pass>(&'a self, rpass: &mut RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);

        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);

        rpass.set_bind_group(0, self.camera_binding.bind_group(), &[]);

        for (meta, _) in self.vector_objects.iter() {
            rpass.draw_indexed(
                meta.index_range.clone(),
                meta.vertex_range.start as i32,
                meta.instance_range.clone(),
            )
        }
    }

    fn update_geometry(&mut self, queue: &wgpu::Queue) {
        let mut vertex_data: Vec<Vec2> = vec![];
        let mut index_data: Vec<u32> = vec![];

        for (_, instance) in self.vector_objects.iter() {
            vertex_data.extend(instance.vertex_buffers.vertices.iter());
            index_data.extend(instance.vertex_buffers.indices.iter());
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertex_data));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&index_data));
    }

    // Loads vector instances to be rendered
    pub fn upload_instances(&mut self, queue: &wgpu::Queue, instances: &[VectorInstance]) {
        self.update_geometry(queue);

        let mut sorted: Vec<Vec<RawInstance>> = vec![vec![]; self.vector_objects.len()];

        for instance in instances {
            sorted[instance.id.index].push((*instance).into());
        }

        let instance_data = {
            let mut instance_data: Vec<u8> = vec![];
            let mut offset = 0;
            for (i, instances) in sorted.iter().enumerate() {
                instance_data.extend(bytemuck::cast_slice(instances));
                self.vector_objects[i].0.instance_range = offset..offset + instances.len() as u32;
                offset += instances.len() as u32;
            }
            instance_data
        };

        queue.write_buffer(&self.instance_buffer, 0, &instance_data);
    }

    fn pipeline_layout(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Vector Renderer Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        })
    }

    fn create_render_pipeline(
        base: &BaseRenderState,
        shader_module: &ShaderModule,
        camera: &CameraUniform,
    ) -> RenderPipeline {
        let bind_group_layouts: Vec<&BindGroupLayout> = vec![camera.bind_group_layout()];

        let layout = Self::pipeline_layout(&base.device, &bind_group_layouts);

        let target = ColorTargetState {
            format: base.swapchain_format,
            blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        };

        let targets = [Some(target)];
        let buffers = [
            vec2_buffer_descriptor(),
            RawInstance::buffer_layout_descriptor(),
        ];
        let descriptor = &helpers::generic_pipeline_descriptor(
            &layout,
            shader_module,
            &targets,
            &buffers,
            base.msaa_config,
        );

        base.device.create_render_pipeline(descriptor)
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera_binding.update(queue, camera);
    }

    pub fn load_svg(
        &mut self,
        path: &str,
        name: Option<&str>,
    ) -> Result<Handle<SVGGeometry>, Error> {
        let obj = SVGGeometry::load_svg_form_path(path, name)?;
        Ok(self.add_vector_object(obj))
    }

    fn next_vector_object_meta(&self, obj: &SVGGeometry) -> VectorObjectMeta {
        let previous_meta = self
            .vector_objects
            .last()
            .map(|it| it.0.clone())
            .unwrap_or_default();

        let vertex_offset = obj.vertex_buffers.vertices.len() as u32;
        let index_offset = obj.vertex_buffers.indices.len() as u32;

        VectorObjectMeta {
            vertex_range: (previous_meta.vertex_range.end
                ..previous_meta.vertex_range.end + vertex_offset),
            index_range: (previous_meta.index_range.end
                ..previous_meta.index_range.end + index_offset),
            instance_range: 0..0,
        }
    }

    fn add_vector_object(&mut self, vector_object: SVGGeometry) -> Handle<SVGGeometry> {
        let handle = Handle::new(self.vector_objects.len());

        self.vector_lookup
            .insert(vector_object.name.clone(), handle);

        self.render_queue.push(vec![]);

        self.vector_objects
            .push((self.next_vector_object_meta(&vector_object), vector_object));

        handle
    }

    pub fn render_instance(&mut self, instance: VectorInstance) {
        self.render_queue[instance.id.index].push(instance);
    }

    pub fn reference(&self) -> VectorRendererReference {
        let mut hit_boxes = HashMap::new();
        for obj in self.vector_lookup.values() {
            hit_boxes.insert(*obj, self.vector_objects[obj.index].1.hit_box.into());
        }

        VectorRendererReference {
            vectors: self.vector_lookup.clone(),
            hit_boxes,
        }
    }
}

#[derive(Clone)]
pub struct VectorRendererReference {
    pub vectors: HashMap<String, Handle<SVGGeometry>>,
    pub hit_boxes: HashMap<Handle<SVGGeometry>, Bounds>,
}

impl VectorRendererReference {
    pub fn get_vector(&self, name: &str) -> Option<&Handle<SVGGeometry>> {
        self.vectors.get(name)
    }
}
