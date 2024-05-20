use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, ColorTargetState, Device,
    PipelineLayout, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
};

use crate::render::{
    bindable::{BindList, Bindable},
    camera::Camera,
    geometry::TexturedQuad,
    vertex::VertexUV,
    BaseRenderState,
};

use super::sprite::sprite_sheet::{SpriteInstance, SpriteSheet};

pub struct SpriteRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    shader_module: ShaderModule,
    sprite_sheets: Vec<SpriteSheet>,
    vertex_count: usize,
}

impl SpriteRenderer {
    pub fn create(base: &BaseRenderState, sheets: Vec<SpriteSheet>, camera: &Camera) -> Self {
        let shader_module = Self::shader_module(&base.device);
        let render_pipeline = Self::create_render_pipeline(base, &shader_module, &sheets, camera);
        let vertex_buffer = Self::vertex_buffer(&base.device);

        Self {
            render_pipeline,
            vertex_buffer,
            shader_module,
            sprite_sheets: sheets,
            vertex_count: 0,
        }
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
            label: Some("SpriteRenderer Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        })
    }

    pub fn upload_sprites(&mut self, queue: &wgpu::Queue, sprites: &[SpriteInstance]) {
        let textured_quads = sprites.iter().copied().map(|s| s.into());
        let vertices: Vec<VertexUV> = textured_quads
            .flat_map(|q: TexturedQuad| q.verticies.into_iter())
            .collect();

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        self.vertex_count = vertices.len();
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
        sheets: &[SpriteSheet],
        camera: &Camera,
    ) -> RenderPipeline {
        let mut bind_group_layouts = vec![camera.bind_group_layout()];

        for sheet in sheets {
            bind_group_layouts.push(sheet.bind_group_layout());
        }

        let layout = Self::pipeline_layout(&base.device, bind_group_layouts.as_slice());
        let targets = [Some(base.swapchain_format.into())];
        let buffers = [VertexUV::buffer_layout_descriptor()];
        let descriptor = &Self::pipeline_descriptor(&layout, &shader_module, &targets, &buffers);

        base.device.create_render_pipeline(descriptor)
    }

    pub fn render<'pass, 'a: 'pass>(&'a self, mut rpass: RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);

        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for (i, sheet) in self.sprite_sheets.iter().enumerate() {
            rpass.set_bind_group(i as u32 + 1, &sheet.bind_group, &[]);
        }

        // rpass.draw(0..self.text_object.content.len() as u32 * 6, 0..1);
        rpass.draw(0..self.vertex_count as u32, 0..1);
        drop(rpass)
    }
}
