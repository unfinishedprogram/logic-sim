use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use serde::Deserialize;
use wgpu::{util::DeviceExt, BindGroupLayoutDescriptor, Device, Queue};

use crate::render::{geometry::TexturedQuad, img_texture::ImageTexture, vertex::VertexUV};

pub struct SpriteSheet {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub texture: ImageTexture,
    pub sprites: HashMap<String, Sprite>,
}

impl SpriteSheet {
    pub fn get_sprite(&self, name: &str) -> Option<&Sprite> {
        println!("Getting sprite: {}", name);
        dbg!(self.sprites.get(name))
    }
}

#[derive(Clone, Copy)]
pub struct SpriteInstance {
    pub sprite: Sprite,
    pub position: Vec2,
    pub scale: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub offsets: [Vec2; 2],
    pub uv: [Vec2; 2],
}

impl Sprite {
    pub fn instantiate(self, position: Vec2, scale: f32) -> SpriteInstance {
        SpriteInstance {
            sprite: self,
            position,
            scale,
        }
    }
}

impl From<SpriteInstance> for TexturedQuad {
    fn from(val: SpriteInstance) -> Self {
        let [uv1, uv2] = val.sprite.uv;

        let p1 = VertexUV(val.position + val.sprite.offsets[0] * val.scale, uv1);
        let p2 = VertexUV(val.position + val.sprite.offsets[1] * val.scale, uv2);

        TexturedQuad::new(p1, p2)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MsdfSpriteSheetUniform {
    distance_range: f32,
}

impl SpriteSheet {
    pub fn build_sprite_lookup(manifest: &Manifest) -> HashMap<String, Sprite> {
        let mut sprites = HashMap::new();
        let atlas_size = Vec2::new(manifest.atlas.width, manifest.atlas.height);
        for sprite_def in &manifest.sprites {
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

    pub fn create(device: &Device, queue: &Queue, manifest: &Manifest, image: &[u8]) -> Self {
        let sprites = Self::build_sprite_lookup(manifest);

        let uniform = MsdfSpriteSheetUniform {
            distance_range: manifest.atlas.distance_range,
        };

        let bind_group_layout = device.create_bind_group_layout(Self::layout_descriptor());

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
            sprites,
        }
    }
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Manifest {
    pub atlas: Atlas,
    pub sprites: Vec<SpriteDef>,
}

#[derive(Deserialize)]
pub struct SpriteDef {
    pub name: String,
    #[serde(rename = "planeBounds")]
    pub plane_bounds: Bounds,
    #[serde(rename = "atlasBounds")]
    pub atlas_bounds: Bounds,
}
