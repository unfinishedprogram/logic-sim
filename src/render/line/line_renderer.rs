use wgpu::{
    include_wgsl, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, ColorTargetState,
    Device, PipelineLayout, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
};

use crate::render::{
    bindable::Bindable,
    camera::{Camera, CameraBinding},
    vertex::VertexUV,
    BaseRenderState,
};

use super::geometry::{LineDescriptor, LineGeometry};

pub struct LineRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    camera_binding: CameraBinding,
    quad_count: usize,
}

impl LineRenderer {
    pub fn create(base: &BaseRenderState) -> Self {
        let shader_module = Self::shader_module(&base.device);
        let vertex_buffer = Self::vertex_buffer(&base.device);
        let camera_binding = CameraBinding::create(&base.device);

        let render_pipeline = Self::create_render_pipeline(base, &shader_module, &camera_binding);

        Self {
            render_pipeline,
            vertex_buffer,
            camera_binding,
            quad_count: 0,
        }
    }

    pub fn render<'pass, 'a: 'pass>(&'a self, mut rpass: RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_bind_group(0, self.camera_binding.bind_group(), &[]);
        rpass.draw(0..(self.quad_count * 6) as u32, 0..1);
    }

    // Loads sprite instances to be rendered
    pub fn upload_lines(&mut self, queue: &wgpu::Queue, lines: &[LineDescriptor]) {
        // TODO: Remove this unnecessary double allocation
        let geometries: Vec<LineGeometry> = lines
            .iter()
            .copied()
            .map(LineGeometry::from_descriptor)
            .collect();

        let vertices = geometries
            .iter()
            .flat_map(|quad| quad.vertices)
            .collect::<Vec<VertexUV>>();

        self.quad_count = geometries.len();
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
    }

    fn vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Sprite Renderer Vertex Buffer"),
            size: 8096 * 8096,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
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

    fn pipeline_descriptor<'a>(
        layout: &'a PipelineLayout,
        shader: &'a ShaderModule,
        targets: &'a [Option<ColorTargetState>],
        buffers: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> RenderPipelineDescriptor<'a> {
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
            multisample: wgpu::MultisampleState::default(),
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
        let buffers = [VertexUV::buffer_layout_descriptor()];
        let descriptor = &Self::pipeline_descriptor(&layout, shader_module, &targets, &buffers);

        base.device.create_render_pipeline(descriptor)
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera_binding.update(queue, camera);
    }
}
