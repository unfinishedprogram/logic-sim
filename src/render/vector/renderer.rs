use std::collections::HashMap;

use glam::{Vec2, Vec4};
use wgpu::{
    include_wgsl, vertex_attr_array, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages,
    ColorTargetState, Device, IndexFormat, PipelineLayout, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, ShaderModule,
};

use crate::{
    render::{
        bindable::Bindable,
        camera::{Camera, CameraBinding},
        vertex::VertexUV,
        BaseRenderState,
    },
    util::handle::Handle,
};

use super::{
    instance::{Instance, RawInstance},
    vector_object::{Error, VectorObject},
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
    camera_binding: CameraBinding,

    vector_objects: Vec<(VectorObjectMeta, VectorObject)>,
    vector_lookup: HashMap<String, Handle<VectorObject>>,
    render_queue: Vec<Vec<Instance>>,
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
        let shader_module = Self::shader_module(&base.device);
        let vertex_buffer = Self::vertex_buffer(&base.device);
        let index_buffer = Self::index_buffer(&base.device);
        let instance_buffer = Self::instance_buffer(&base.device);

        let camera_binding = CameraBinding::create(&base.device);

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
    pub fn upload_instances(&mut self, queue: &wgpu::Queue, instances: &[Instance]) {
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

    fn vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Vector Renderer Vertex Buffer"),
            size: 8192 * 8192,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn index_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Vector Renderer Index Buffer"),
            size: 8192 * 512,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn instance_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Vector Renderer Instance Buffer"),
            size: 8192 * 8192,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn shader_module(device: &Device) -> ShaderModule {
        device.create_shader_module(include_wgsl!("shader.wgsl"))
    }

    fn pipeline_layout(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Vector Renderer Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        })
    }

    fn pipeline_descriptor<'a>(
        layout: &'a PipelineLayout,
        shader: &'a ShaderModule,
        targets: &'a [Option<ColorTargetState>],
        buffers: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> RenderPipelineDescriptor<'a> {
        let multisample = wgpu::MultisampleState {
            count: 4,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets,
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample,
            multiview: None,
        }
    }

    fn create_render_pipeline(
        base: &BaseRenderState,
        shader_module: &ShaderModule,
        camera: &CameraBinding,
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
        let descriptor = &Self::pipeline_descriptor(&layout, shader_module, &targets, &buffers);

        base.device.create_render_pipeline(descriptor)
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera_binding.update(queue, camera);
    }

    pub fn load_svg(
        &mut self,
        path: &str,
        name: Option<&str>,
    ) -> Result<Handle<VectorObject>, Error> {
        let obj = VectorObject::load_svg_form_path(path, name)?;
        Ok(self.add_vector_object(obj))
    }

    fn next_vector_object_meta(&self, obj: &VectorObject) -> VectorObjectMeta {
        let previous_meta = self
            .vector_objects
            .last()
            .map(|it| it.0.clone())
            .unwrap_or_default();

        let vertex_offset = obj.vertex_buffers.vertices.len() as u32;
        let index_offset = obj.vertex_buffers.indices.len() as u32;

        VectorObjectMeta {
            vertex_range: (previous_meta.vertex_range.end
                ..previous_meta.vertex_range.end + vertex_offset as u32),
            index_range: (previous_meta.index_range.end
                ..previous_meta.index_range.end + index_offset as u32),
            instance_range: 0..0,
        }
    }

    fn add_vector_object(&mut self, vector_object: VectorObject) -> Handle<VectorObject> {
        let handle = Handle::new(self.vector_objects.len());

        self.vector_lookup
            .insert(vector_object.name.clone(), handle);

        self.render_queue.push(vec![]);

        self.vector_objects
            .push((self.next_vector_object_meta(&vector_object), vector_object));

        handle
    }

    pub fn render_instance(&mut self, instance: Instance) {
        self.render_queue[instance.id.index].push(instance);
    }

    pub fn reference(&self) -> VectorRendererReference {
        VectorRendererReference {
            vectors: self.vector_lookup.clone(),
        }
    }
}

#[derive(Clone)]
pub struct VectorRendererReference {
    pub vectors: HashMap<String, Handle<VectorObject>>,
}

impl VectorRendererReference {
    pub fn get_vector(&self, name: &str) -> Option<&Handle<VectorObject>> {
        self.vectors.get(name)
    }
}