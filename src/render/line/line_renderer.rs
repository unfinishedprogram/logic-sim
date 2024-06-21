use wgpu::{
    include_wgsl, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, ColorTargetState,
    Device, IndexFormat, PipelineLayout, RenderPass, RenderPipeline, ShaderModule,
};

use crate::render::{
    bindable::Bindable,
    camera::{Camera, CameraUniform},
    helpers,
    vertex::VertexUV,
    BaseRenderState,
};

pub struct LineRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    camera_binding: CameraUniform,
    vert_count: usize,
    index_count: usize,
}

impl LineRenderer {
    pub fn create(base: &BaseRenderState) -> Self {
        let shader_module = Self::shader_module(&base.device);
        let vertex_buffer = Self::vertex_buffer(&base.device);
        let index_buffer = Self::index_buffer(&base.device);

        let camera_binding = CameraUniform::create(&base.device);

        let render_pipeline = Self::create_render_pipeline(base, &shader_module, &camera_binding);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            camera_binding,
            vert_count: 0,
            index_count: 0,
        }
    }

    pub fn render<'pass, 'a: 'pass>(&'a self, rpass: &mut RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);

        rpass.set_bind_group(0, self.camera_binding.bind_group(), &[]);

        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }

    // Loads line instances to be rendered
    pub fn upload_geometry(&mut self, queue: &wgpu::Queue, indicies: &[u32], verts: &[VertexUV]) {
        self.vert_count = verts.len();
        self.index_count = indicies.len();
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(indicies));
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(verts));
    }

    fn vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Line Renderer Vertex Buffer"),
            size: 8192 * 8192,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn index_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Line Renderer Index Buffer"),
            size: 8192 * 8192,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn shader_module(device: &Device) -> ShaderModule {
        device.create_shader_module(include_wgsl!("shader.wgsl"))
    }

    fn pipeline_layout(device: &Device, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("LineRenderer Pipeline Layout"),
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
        let buffers = [VertexUV::buffer_layout_descriptor()];
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
}
