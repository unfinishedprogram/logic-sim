mod handle;
mod instance;
pub mod sheet;

pub use handle::SpriteHandle;
pub use instance::SpriteInstance;
use sheet::Sprite;
pub use sheet::SpriteSheet;

use std::{
    collections::HashMap,
    ops::{Index, Range},
};

use wgpu::{
    include_wgsl, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, ColorTargetState,
    Device, PipelineLayout, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
};

use crate::render::{
    bindable::Bindable,
    camera::{Camera, CameraUniform},
    vertex::VertexUV,
    BaseRenderState,
};

pub struct SpriteRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    camera_binding: CameraUniform,
    sprite_sheets: Vec<SpriteSheet>,
    sprite_ranges: Vec<RenderRange>,
}

#[derive(Default, Clone)]
struct RenderRange {
    verts: Range<u32>,
    indices: Range<u32>,
}

impl Index<SpriteHandle> for SpriteRenderer {
    type Output = Sprite;

    fn index(&self, handle: SpriteHandle) -> &Self::Output {
        &self.sprite_sheets[handle.sheet.idx].sprites_vec[handle.sprite.idx]
    }
}

impl SpriteRenderer {
    pub fn create(base: &BaseRenderState, sprite_sheets: Vec<SpriteSheet>) -> Self {
        let shader_module = Self::shader_module(&base.device);
        let vertex_buffer = Self::vertex_buffer(&base.device);
        let index_buffer = Self::index_buffer(&base.device);
        let camera_binding = CameraUniform::create(&base.device);

        let render_pipeline = Self::create_render_pipeline(base, &shader_module, &camera_binding);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            sprite_sheets,
            camera_binding,
            sprite_ranges: vec![],
        }
    }

    #[inline(never)]
    pub fn render<'pass, 'a: 'pass>(&'a self, rpass: &mut RenderPass<'pass>) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        rpass.set_bind_group(0, self.camera_binding.bind_group(), &[]);

        for (sheet_idx, sheet) in self.sprite_sheets.iter().enumerate() {
            rpass.set_bind_group(1, &sheet.bind_group, &[]);

            let range = &self.sprite_ranges[sheet_idx];

            rpass.draw_indexed(range.indices.clone(), range.verts.start as i32, 0..1);
        }
    }

    pub fn reference(&self) -> SpriteRendererReference {
        let sheets = self
            .sprite_sheets
            .iter()
            .enumerate()
            .map(|(sheet_idx, sheet)| {
                let sprites = sheet
                    .sprites
                    .iter()
                    .map(|(name, sprite_idx)| {
                        (name.clone(), SpriteHandle::new(sheet_idx, *sprite_idx))
                    })
                    .collect();
                (sheet.name.to_owned(), sprites)
            })
            .collect();

        SpriteRendererReference { sheets }
    }

    fn index_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Sprite Renderer Index Buffer"),
            size: 8192 * 512,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Sprite Renderer Vertex Buffer"),
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
        let mut instances_by_sheet = vec![vec![]; self.sprite_sheets.len()];

        for sprite in sprites {
            instances_by_sheet[sprite.sprite_handle.sheet.idx].push(*sprite);
        }

        let verts_by_sheet: Vec<_> = instances_by_sheet
            .into_iter()
            .map(|instances| {
                let mut verts = vec![];
                let mut indices = vec![];

                for instance in instances {
                    let quad = self[instance.sprite_handle].as_textured_quad(&instance);

                    let start = verts.len() as u32;
                    verts.extend(quad.vertices.into_iter());
                    indices.extend(quad.indices.iter().map(|i| i + start));
                }

                (verts, indices)
            })
            .collect();

        // Add vertex index ranges
        let mut ranges: Vec<RenderRange> = vec![];
        let mut verts: Vec<VertexUV> = vec![];
        let mut indices: Vec<u32> = vec![];

        for (sheet_verts, sheet_indices) in verts_by_sheet.into_iter() {
            let v_start = verts.len() as u32;
            let i_start = indices.len() as u32;

            verts.extend(sheet_verts);
            indices.extend(sheet_indices);

            let v_end = verts.len() as u32;
            let i_end = indices.len() as u32;

            ranges.push(RenderRange {
                verts: (v_start..v_end),
                indices: (i_start..i_end),
            });
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
        multisample: wgpu::MultisampleState,
    ) -> RenderPipelineDescriptor<'a> {
        wgpu::RenderPipelineDescriptor {
            label: Some("SpriteRenderer Pipeline"),
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
        camera: &CameraUniform,
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
        let descriptor = &Self::pipeline_descriptor(
            &layout,
            shader_module,
            &targets,
            &buffers,
            base.msaa_config,
        );

        base.device.create_render_pipeline(descriptor)
    }
}

#[derive(Clone)]
pub struct SpriteRendererReference {
    pub sheets: HashMap<String, HashMap<String, SpriteHandle>>,
}

impl SpriteRendererReference {
    pub fn get_sprite(&self, sheet: &str, sprite: &str) -> Option<&SpriteHandle> {
        self.sheets.get(sheet).and_then(|sheet| sheet.get(sprite))
    }
}
