use wgpu::{Device, Queue};

use crate::render::{
    bindable::Bindable, img_texture::ImageTexture, msdf::sprite::sprite_sheet::SpriteSheet,
};

use super::Manifest;

pub struct MsdfFont {
    pub manifest: Manifest,
    pub sprite_sheet: SpriteSheet,
}

impl MsdfFont {
    pub fn texture(&self) -> &ImageTexture {
        &self.sprite_sheet.texture
    }

    pub fn create(device: &Device, queue: &Queue, manifest: &str, image: &[u8]) -> Self {
        let manifest = serde_json::from_str::<Manifest>(manifest).unwrap();
        let sprite_sheet = SpriteSheet::create(device, queue, &manifest.clone().into(), image);

        Self {
            manifest,
            sprite_sheet,
        }
    }
}

impl Bindable for MsdfFont {
    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.sprite_sheet.bind_group_layout
    }

    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.sprite_sheet.bind_group
    }
}
