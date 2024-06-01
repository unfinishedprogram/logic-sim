mod sprite_instance;

use std::{collections::HashMap, ops::Range};

use wgpu::{
    include_wgsl, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, ColorTargetState,
    Device, PipelineLayout, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
};

use crate::render::{
    bindable::Bindable,
    camera::{Camera, CameraBinding},
    geometry::TexturedQuad,
    vertex::VertexUV,
    BaseRenderState,
};

use super::sprite::sprite_sheet::{Sprite, SpriteInstance, SpriteSheet};

pub struct SpriteRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    instance_buffer: Buffer,
    index_buffer: Buffer,
    camera_binding: CameraBinding,
    sprite_sheets: HashMap<String, SpriteSheet>,
    sprite_ranges: HashMap<String, RenderRange>,
}

#[derive(Default, Clone)]
struct RenderRange {
    verts: Range<u32>,
    indices: Range<u32>,
}

impl RenderRange {
    const ZERO: Self = Self {
        verts: 0..0,
        indices: 0..0,
    };
}

impl SpriteRenderer {
    pub fn create(base: &BaseRenderState, sheets: Vec<SpriteSheet>) -> Self {
        let shader_module = Self::shader_module(&base.device);
        let vertex_buffer = Self::vertex_buffer(&base.device);
        let instance_buffer = Self::instance_buffer(&base.device);
        let index_buffer = Self::index_buffer(&base.device);
        let camera_binding = CameraBinding::create(&base.device);

        let render_pipeline = Self::create_render_pipeline(base, &shader_module, &camera_binding);

        let sprite_sheets = sheets
            .into_iter()
            .map(|sheet| (sheet.name.to_string(), sheet))
            .collect();

        Self {
            render_pipeline,
            vertex_buffer,
            instance_buffer,
            index_buffer,
            sprite_sheets,
            camera_binding,
            sprite_ranges: HashMap::new(),
        }
    }

    #[inline(never)]
    pub fn render<'pass, 'a: 'pass>(&'a self, rpass: &mut RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        rpass.set_bind_group(0, self.camera_binding.bind_group(), &[]);

        for (name, sheet) in self.sprite_sheets.iter() {
            rpass.set_bind_group(1, &sheet.bind_group, &[]);

            let range = self.sprite_ranges.get(name).unwrap_or(&RenderRange::ZERO);

            rpass.draw_indexed(range.indices.clone(), range.verts.start as i32, 0..1);
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

    fn index_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Sprite Renderer Index Buffer"),
            size: 8096 * 512,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Sprite Renderer Vertex Buffer"),
            size: 8096 * 8096,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn instance_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Sprite Renderer Instance Buffer"),
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

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera_binding.update(queue, camera);
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
                .get_mut(sprite.sprite.sheet_name)
                .expect("Sprite sheet not found on this renderer")
                .push(sprite);
        }

        let verts_by_sheet: HashMap<String, (Vec<VertexUV>, Vec<u32>)> = instances_by_sheet
            .into_iter()
            .map(|(name, instances)| {
                let quads = instances
                    .into_iter()
                    .map(|instance| TexturedQuad::from(*instance))
                    .collect::<Vec<TexturedQuad>>();

                let mut verts = vec![];
                let mut indices = vec![];

                for quad in quads.iter() {
                    let start = verts.len() as u32;
                    verts.extend(quad.vertices.into_iter());
                    indices.extend(quad.indices.iter().map(|i| i + start));
                }

                (name.to_string(), (verts, indices))
            })
            .collect();

        // Add vertex index ranges
        let mut ranges: HashMap<String, RenderRange> = HashMap::new();
        let mut verts: Vec<VertexUV> = vec![];
        let mut indices: Vec<u32> = vec![];

        for (name, (sheet_verts, sheet_indices)) in verts_by_sheet.iter() {
            let v_start = verts.len() as u32;
            let i_start = indices.len() as u32;

            verts.extend(sheet_verts);
            indices.extend(sheet_indices);

            let v_end = verts.len() as u32;
            let i_end = indices.len() as u32;

            ranges.insert(
                name.clone(),
                RenderRange {
                    verts: (v_start..v_end),
                    indices: (i_start..i_end),
                },
            );
        }

        self.sprite_ranges = ranges;

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&verts));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
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
        let layout = &base
            .device
            .create_bind_group_layout(SpriteSheet::layout_descriptor());

        let bind_group_layouts: Vec<&BindGroupLayout> = vec![camera.bind_group_layout(), layout];

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
}

pub struct SpriteRendererReference {
    pub sheets: HashMap<String, HashMap<String, Sprite>>,
}

impl SpriteRendererReference {
    pub fn get_sprite(&self, sheet: &str, sprite: &str) -> Option<&Sprite> {
        self.sheets.get(sheet).and_then(|sheet| sheet.get(sprite))
    }
}
