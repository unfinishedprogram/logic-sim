use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use miniserde::Deserialize;
use wgpu::{util::DeviceExt, BindGroupLayoutDescriptor, Device, Queue};

use crate::render::{geometry::TexturedQuad, img_texture::ImageTexture, vertex::VertexUV};

use super::SpriteInstance;

pub struct SpriteSheet {
    pub name: String,
    pub bind_group: wgpu::BindGroup,
    pub sprites: HashMap<String, usize>,
    pub sprites_vec: Vec<Sprite>,
    _texture: ImageTexture,
}

pub struct Sprite {
    pub offsets: [Vec2; 2],
    pub uv: [Vec2; 2],
}

impl Sprite {
    pub fn as_textured_quad(&self, sprite_instance: &SpriteInstance) -> TexturedQuad {
        let [uv1, uv2] = self.uv;

        let p1 = VertexUV {
            position: sprite_instance.position + self.offsets[0] * sprite_instance.scale,
            uv: uv1,
            color: sprite_instance.color,
        };
        let p2 = VertexUV {
            position: sprite_instance.position + self.offsets[1] * sprite_instance.scale,
            uv: uv2,
            color: sprite_instance.color,
        };

        TexturedQuad::new(p1, p2)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MsdfSpriteSheetUniform {
    distance_range: f32,
}

impl SpriteSheet {
    pub fn layout_descriptor() -> &'static BindGroupLayoutDescriptor<'static> {
        &wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Sprite sheet layout descriptor"),
        }
    }

    pub fn create(device: &Device, queue: &Queue, manifest: &Manifest, image: &[u8]) -> Self {
        let mut sprites = HashMap::new();
        let mut sprites_vec = Vec::new();

        for (name, sprite) in manifest.sprites() {
            let idx = sprites_vec.len();
            sprites.insert(name.clone(), idx);
            sprites_vec.push(sprite);
        }

        let uniform = MsdfSpriteSheetUniform {
            distance_range: manifest.atlas.distance_range,
        };

        let bind_group_layout = device.create_bind_group_layout(Self::layout_descriptor());

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let texture = ImageTexture::create(device, queue, image);

        let texture_view = &texture.texture_view;
        let sampler = &texture.sampler;

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            label: None,
        });

        Self {
            bind_group,
            _texture: texture,
            sprites,
            sprites_vec,
            name: manifest.name.clone(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Atlas {
    #[serde(rename = "distanceRange")]
    pub distance_range: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Deserialize, Default)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Deserialize, Clone)]
pub struct Manifest {
    pub name: String,
    pub atlas: Atlas,
    pub sprites: Vec<SpriteDef>,
}

#[derive(Deserialize, Clone)]
pub struct SpriteDef {
    pub name: String,
    #[serde(rename = "planeBounds")]
    pub plane_bounds: Bounds,
    #[serde(rename = "atlasBounds")]
    pub atlas_bounds: Bounds,
}

impl Manifest {
    pub fn sprites(&self) -> HashMap<String, Sprite> {
        let mut sprites = HashMap::new();
        let atlas_size = Vec2::new(self.atlas.width, self.atlas.height);
        for sprite_def in &self.sprites {
            let offsets = [
                Vec2::new(sprite_def.plane_bounds.left, sprite_def.plane_bounds.top),
                Vec2::new(
                    sprite_def.plane_bounds.right,
                    sprite_def.plane_bounds.bottom,
                ),
            ];
            let uv = [
                Vec2::new(sprite_def.atlas_bounds.left, sprite_def.atlas_bounds.top) / atlas_size,
                Vec2::new(
                    sprite_def.atlas_bounds.right,
                    sprite_def.atlas_bounds.bottom,
                ) / atlas_size,
            ];
            sprites.insert(sprite_def.name.clone(), Sprite { offsets, uv });
        }
        sprites
    }
}
