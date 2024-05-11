mod manifest;

use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, BindGroupLayoutDescriptor, Device, Queue};

use crate::render::img_texture::ImageTexture;

pub struct MsdfFont {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub texture: ImageTexture,
    pub uniform: MsdfFontUniform,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MsdfFontUniform {
    distance_range: f32,
}

impl MsdfFont {
    fn layout_descriptor() -> &'static BindGroupLayoutDescriptor<'static> {
        &wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        }
    }

    pub fn create(device: &Device, queue: &Queue, manifest: &str, image: &[u8]) -> Self {
        let manifest = serde_json::from_str::<manifest::Manifest>(manifest).unwrap();

        let uniform = MsdfFontUniform {
            distance_range: manifest.distance_field.distance_range as f32,
        };

        let bind_group_layout = device.create_bind_group_layout(&Self::layout_descriptor());

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let texture = ImageTexture::create(device, queue, image, None);

        Self {
            bind_group_layout,
            bind_group,
            texture,
            uniform,
        }
    }
}
