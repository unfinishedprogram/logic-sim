use std::collections::HashMap;

use wgpu::{
    include_wgsl, BindGroupLayout, Buffer, ColorTargetState, Device, IndexFormat, PipelineLayout,
    RenderPass, RenderPipeline, ShaderModule,
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
    draw_call_ordering::{create_render_request, VectorRenderRequest},
    instance::{RawInstance, VectorInstance},
    svg_geometry::SVGGeometry,
    vertex::SVGVertex,
};

#[derive(Default, Clone, Debug)]
struct VectorInstanceBufferRanges {
    pub vertex_range: std::ops::Range<u32>,
    pub index_range: std::ops::Range<u32>,
}

pub struct VectorRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    camera_binding: CameraUniform,

    vector_objects: Vec<(VectorInstanceBufferRanges, SVGGeometry)>,
    vector_lookup: HashMap<String, Handle<SVGGeometry>>,

    render_request: VectorRenderRequest,
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
            render_request: Default::default(),
        }
    }

    pub fn render<'pass, 'a: 'pass>(&'a self, rpass: &mut RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);

        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);

        rpass.set_bind_group(0, self.camera_binding.bind_group(), &[]);

        for call in self.render_request.draw_calls.iter() {
            let meta = &self.vector_objects[call.id.index].0;
            rpass.draw_indexed(
                meta.index_range.clone(),
                meta.vertex_range.start as i32,
                call.range.clone(),
            );
        }
    }

    fn update_geometry(&mut self, queue: &wgpu::Queue) {
        let mut vertex_data: Vec<SVGVertex> = vec![];
        let mut index_data: Vec<u32> = vec![];

        for (_, instance) in self.vector_objects.iter() {
            vertex_data.extend(instance.vertex_buffers.vertices.iter());
            index_data.extend(instance.vertex_buffers.indices.iter());
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertex_data));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&index_data));
    }

    // Loads vector instances to be rendered
    pub fn upload_instances(&mut self, queue: &wgpu::Queue, instances: Vec<VectorInstance>) {
        self.update_geometry(queue);
        self.render_request = create_render_request(instances);

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.render_request.instances_buf),
        );
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
            SVGVertex::buffer_layout(),
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

    fn next_vector_object_meta(&self, obj: &SVGGeometry) -> VectorInstanceBufferRanges {
        let previous_meta = self
            .vector_objects
            .last()
            .map(|it| it.0.clone())
            .unwrap_or_default();

        let vertex_offset = obj.vertex_buffers.vertices.len() as u32;
        let index_offset = obj.vertex_buffers.indices.len() as u32;

        VectorInstanceBufferRanges {
            vertex_range: (previous_meta.vertex_range.end
                ..previous_meta.vertex_range.end + vertex_offset),
            index_range: (previous_meta.index_range.end
                ..previous_meta.index_range.end + index_offset),
        }
    }

    pub fn add_vector_object(&mut self, vector_object: SVGGeometry) -> Handle<SVGGeometry> {
        let handle = Handle::new(self.vector_objects.len());

        self.vector_lookup
            .insert(vector_object.source.clone(), handle);

        self.vector_objects
            .push((self.next_vector_object_meta(&vector_object), vector_object));

        handle
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
    pub fn get_vector(&self, name: &str) -> Option<Handle<SVGGeometry>> {
        self.vectors.get(name).copied()
    }
}
