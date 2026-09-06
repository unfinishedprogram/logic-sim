use std::collections::HashMap;

use assets::SVGSource;
use wgpu::{
    BindGroupLayout, Buffer, ColorTargetState, Device, IndexFormat, PipelineLayout, RenderPass,
    RenderPipeline, ShaderModule, include_wgsl,
};

use crate::render::{
    BaseRenderState,
    bindable::Bindable,
    camera::{Camera, CameraUniform},
    helpers,
    vector::tessellator::Tessellator,
};

use common::{handle::Handle, profiler};

use super::{
    draw_call_ordering::{VectorRenderRequest, create_render_request},
    instance::{RawInstance, VectorInstance},
    lazy_instance::LazyVectorInstance,
    svg_geometry::SVGGeometry,
    vertex::SVGVertex,
};

#[derive(Default, Clone, Debug)]
struct VectorInstanceBufferRanges {
    pub vertex_range: std::ops::Range<u32>,
    pub index_range: std::ops::Range<u32>,
}

pub struct VectorRenderer {
    pub tessellator: Tessellator,

    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    camera_binding: CameraUniform,

    vector_objects: Vec<(VectorInstanceBufferRanges, SVGGeometry)>,
    vector_lookup: HashMap<SVGSourceId, Handle<SVGGeometry>>,

    render_request: VectorRenderRequest,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SVGSourceId(usize);

impl SVGSourceId {
    pub fn of(source: &'static SVGSource) -> Self {
        Self(std::ptr::from_ref(source) as usize)
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
            render_request: Default::default(),
            tessellator: Tessellator::default(),
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
            vertex_data.extend_from_slice(&instance.vertex_buffers.vertices);
            index_data.extend_from_slice(&instance.vertex_buffers.indices);
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertex_data));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&index_data));
    }

    // Loads vector instances to be rendered
    pub fn upload_instances(
        &mut self,
        queue: &wgpu::Queue,
        instances: &[VectorInstance],
        lazy_instances: &[LazyVectorInstance<'static>],
        profiler: &mut profiler::Profiler,
    ) {
        profiler.begin("convert");
        let mut converted = self.convert_lazy_instances(lazy_instances);
        converted.extend(instances);
        profiler.end("convert");

        profiler.begin("update geometry");
        self.update_geometry(queue);
        self.render_request = create_render_request(converted, profiler);
        profiler.end("update geometry");

        profiler.begin("upload buffers");
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.render_request.instances_buf),
        );
        profiler.end("upload buffers");
    }

    fn pipeline_layout(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Vector Renderer Pipeline Layout"),
            bind_group_layouts,
            immediate_size: 0,
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

        let vertex_start = previous_meta.vertex_range.end;
        let index_start = previous_meta.index_range.end;

        VectorInstanceBufferRanges {
            vertex_range: (vertex_start..vertex_start + vertex_offset),
            index_range: (index_start..index_start + index_offset),
        }
    }

    pub fn add_vector_object(
        &mut self,
        id: SVGSourceId,
        vector_object: SVGGeometry,
    ) -> Handle<SVGGeometry> {
        let handle = Handle::new(self.vector_objects.len());

        self.vector_lookup.insert(id, handle);

        self.vector_objects
            .push((self.next_vector_object_meta(&vector_object), vector_object));

        handle
    }

    pub fn get_vector(&self, id: SVGSourceId) -> Option<Handle<SVGGeometry>> {
        self.vector_lookup.get(&id).copied()
    }
}
