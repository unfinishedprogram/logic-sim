use std::collections::HashMap;

use wgpu::{
    include_wgsl, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, ColorTargetState,
    Device, PipelineLayout, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
};

use crate::render::{
    bindable::Bindable, camera::Camera, geometry::TexturedQuad, vertex::VertexUV, BaseRenderState,
};

use super::sprite::sprite_sheet::{Sprite, SpriteInstance, SpriteSheet};

pub struct SpriteRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    sprite_sheets: HashMap<String, SpriteSheet>,
    sprite_ranges: HashMap<String, (usize, usize)>,
    vertex_count: usize,
}

impl SpriteRenderer {
    pub fn create(base: &BaseRenderState, sheets: Vec<SpriteSheet>, camera: &Camera) -> Self {
        let shader_module = Self::shader_module(&base.device);
        let render_pipeline = Self::create_render_pipeline(base, &shader_module, camera);
        let vertex_buffer = Self::vertex_buffer(&base.device);

        let sprite_sheets = sheets
            .into_iter()
            .map(|sheet| (sheet.name.to_string(), sheet))
            .collect();

        Self {
            render_pipeline,
            vertex_buffer,
            sprite_sheets,
            sprite_ranges: HashMap::new(),
            vertex_count: 0,
        }
    }

    pub fn reference(&self) -> SpriteRendererReference {
        let sheets = self
            .sprite_sheets
            .iter()
            .map(|(name, sheet)| {
                let sprites = sheet.sprites.clone();
                (name.to_string(), sprites)
            })
            .collect();

        SpriteRendererReference { sheets }
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

    // Loads sprite instances to be rendered
    pub fn upload_sprites(&mut self, queue: &wgpu::Queue, sprites: &[SpriteInstance]) {
        let mut instances_by_sheet: HashMap<String, Vec<&SpriteInstance>> = self
            .sprite_sheets
            .keys()
            .map(|name| (name.to_owned(), vec![]))
            .collect();

        for sprite in sprites {
            instances_by_sheet
                .get_mut(sprite.sprite.name)
                .expect("Sprite sheet not found on this renderer")
                .push(sprite);
        }

        let verts_by_sheet = instances_by_sheet
            .into_iter()
            .map(|(name, instances)| {
                let quads = instances
                    .into_iter()
                    .map(|instance| TexturedQuad::from(*instance))
                    .collect::<Vec<TexturedQuad>>();

                let verts = quads
                    .iter()
                    .flat_map(|quad| quad.vertices)
                    .collect::<Vec<VertexUV>>();

                (name.to_string(), verts)
            })
            .collect::<HashMap<String, Vec<VertexUV>>>();

        // Add vertex index ranges
        let mut ranges: HashMap<String, (usize, usize)> = HashMap::new();
        let mut verts: Vec<VertexUV> = vec![];

        for (name, vertices) in verts_by_sheet.iter() {
            let start = verts.len();
            verts.extend(vertices);
            let end = verts.len();
            ranges.insert(name.clone(), (start, end));
        }
        self.sprite_ranges = ranges;

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&verts));
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
        camera: &Camera,
    ) -> RenderPipeline {
        let layout = &base
            .device
            .create_bind_group_layout(SpriteSheet::layout_descriptor());

        let bind_group_layouts: Vec<&BindGroupLayout> = vec![camera.bind_group_layout(), layout];

        let layout = Self::pipeline_layout(&base.device, bind_group_layouts.as_slice());
        let targets = [Some(base.swapchain_format.into())];
        let buffers = [VertexUV::buffer_layout_descriptor()];
        let descriptor = &Self::pipeline_descriptor(&layout, shader_module, &targets, &buffers);

        base.device.create_render_pipeline(descriptor)
    }

    pub fn render<'pass, 'a: 'pass>(&'a self, mut rpass: RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);

        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for (name, sheet) in self.sprite_sheets.iter() {
            rpass.set_bind_group(1, &sheet.bind_group, &[]);

            let range = self.sprite_ranges.get(name).unwrap_or(&(0, 0));

            rpass.draw(range.0 as u32..range.1 as u32, 0..1);
        }
    }

    pub fn get_sprite(&self, sheet: &str, sprite: &str) -> Option<&Sprite> {
        self.sprite_sheets
            .get(sheet)
            .and_then(|sheet| sheet.get_sprite(sprite))
    }
}

pub struct SpriteRendererReference {
    pub sheets: HashMap<String, HashMap<String, Sprite>>,
}

impl SpriteRendererReference {
    pub fn get_sprite(&self, sheet: &str, sprite: &str) -> Option<&Sprite> {
        self.sheets.get(sheet).and_then(|sheet| sheet.get(sprite))
    }
}
